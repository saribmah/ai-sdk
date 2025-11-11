use crate::embed::many_result::{EmbedManyResult, EmbedManyResultResponseData};
use crate::error::AISDKError;
use crate::generate_text::{RetryConfig, prepare_retries};
use ai_sdk_provider::embedding_model::call_options::EmbeddingModelCallOptions;
use ai_sdk_provider::embedding_model::embedding::EmbeddingModelEmbedding;
use ai_sdk_provider::embedding_model::{EmbeddingModel, EmbeddingModelUsage};
use ai_sdk_provider::shared::headers::SharedHeaders;
use ai_sdk_provider::shared::provider_metadata::SharedProviderMetadata;
use ai_sdk_provider::shared::provider_options::SharedProviderOptions;
use std::collections::HashMap;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Builder for embedding multiple values using an embedding model.
///
/// `EmbedMany` automatically splits large requests into smaller chunks if the model
/// has a limit on how many embeddings can be generated in a single call.
///
/// # Example
///
/// ```no_run
/// use ai_sdk_core::EmbedMany;
/// # use std::sync::Arc;
/// # use ai_sdk_provider::embedding_model::EmbeddingModel;
/// # async fn example(model: Arc<dyn EmbeddingModel<String>>) -> Result<(), Box<dyn std::error::Error>> {
///
/// let result = EmbedMany::new(
///     model,
///     vec!["Hello, world!".to_string(), "How are you?".to_string()],
/// )
///     .max_retries(3)
///     .max_parallel_calls(5)
///     .execute()
///     .await?;
///
/// println!("Embeddings: {:?}", result.embeddings);
/// println!("Usage: {:?}", result.usage);
/// # Ok(())
/// # }
/// ```
pub struct EmbedMany<V>
where
    V: Clone + Send + Sync + 'static,
{
    model: Arc<dyn EmbeddingModel<V>>,
    values: Vec<V>,
    max_retries: Option<u32>,
    abort_signal: Option<CancellationToken>,
    headers: Option<SharedHeaders>,
    provider_options: Option<SharedProviderOptions>,
    max_parallel_calls: Option<usize>,
}

impl<V> EmbedMany<V>
where
    V: Clone + Send + Sync + 'static,
{
    /// Create a new EmbedMany builder.
    ///
    /// # Arguments
    ///
    /// * `model` - The embedding model to use
    /// * `values` - The values that should be embedded
    pub fn new(model: Arc<dyn EmbeddingModel<V>>, values: Vec<V>) -> Self {
        Self {
            model,
            values,
            max_retries: None,
            abort_signal: None,
            headers: None,
            provider_options: None,
            max_parallel_calls: None,
        }
    }

    /// Set the maximum number of retries.
    ///
    /// # Arguments
    ///
    /// * `max_retries` - Maximum number of retries. Set to 0 to disable retries. Default: 2.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    /// Set an abort signal that can be used to cancel the call.
    ///
    /// # Arguments
    ///
    /// * `abort_signal` - An optional abort signal
    pub fn abort_signal(mut self, abort_signal: CancellationToken) -> Self {
        self.abort_signal = Some(abort_signal);
        self
    }

    /// Set additional HTTP headers to be sent with the request.
    ///
    /// # Arguments
    ///
    /// * `headers` - Additional HTTP headers. Only applicable for HTTP-based providers.
    pub fn headers(mut self, headers: SharedHeaders) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Set additional provider-specific options.
    ///
    /// # Arguments
    ///
    /// * `provider_options` - Additional provider-specific options
    pub fn provider_options(mut self, provider_options: SharedProviderOptions) -> Self {
        self.provider_options = Some(provider_options);
        self
    }

    /// Set the maximum number of parallel calls.
    ///
    /// # Arguments
    ///
    /// * `max_parallel_calls` - Maximum number of concurrent requests. Default: unlimited.
    pub fn max_parallel_calls(mut self, max_parallel_calls: usize) -> Self {
        self.max_parallel_calls = Some(max_parallel_calls);
        self
    }

    /// Execute the embedding operation.
    ///
    /// # Returns
    ///
    /// A result object that contains the embeddings, the values, and additional information.
    pub async fn execute(self) -> Result<EmbedManyResult<V>, AISDKError> {
        // Prepare retry configuration
        let retry_config = prepare_retries(self.max_retries, self.abort_signal.clone())?;

        // Add user agent to headers
        let headers_with_user_agent =
            add_user_agent_suffix(self.headers, format!("ai/{}", VERSION));

        // Get model limits
        let max_embeddings_per_call = self.model.max_embeddings_per_call().await;
        let supports_parallel_calls = self.model.supports_parallel_calls().await;

        // If the model has no limit on embeddings per call, process all values at once
        if max_embeddings_per_call.is_none() || max_embeddings_per_call == Some(usize::MAX) {
            let result = retry_config
                .execute_with_boxed_error(|| {
                    let model = self.model.clone();
                    let values = self.values.clone();
                    let headers = headers_with_user_agent.clone();
                    let provider_options = self.provider_options.clone();
                    let abort_signal = self.abort_signal.clone();
                    async move {
                        let options = EmbeddingModelCallOptions {
                            values,
                            abort_signal,
                            headers,
                            provider_options,
                        };
                        model.do_embed(options).await
                    }
                })
                .await?;

            let embeddings = result.embeddings;
            let usage = result.usage.unwrap_or_else(|| EmbeddingModelUsage::new(0));
            let provider_metadata = result.provider_metadata;
            let response = result.response.map(|r| {
                EmbedManyResultResponseData::new()
                    .with_headers(r.headers.unwrap_or_default())
                    .with_body(r.body.unwrap_or_default())
            });

            return Ok(EmbedManyResult::new(self.values, embeddings, usage)
                .with_provider_metadata(provider_metadata.unwrap_or_default())
                .with_responses(vec![response]));
        }

        // Split values into chunks based on model limits
        let max_embeddings = max_embeddings_per_call.unwrap();
        let value_chunks = split_array(self.values.clone(), max_embeddings);

        // Determine how many parallel calls we can make
        let max_parallel = self.max_parallel_calls.unwrap_or(usize::MAX);
        let parallel_limit = if supports_parallel_calls {
            max_parallel
        } else {
            1
        };

        // Split chunks into parallel batches
        let parallel_chunks = split_array(value_chunks, parallel_limit);

        // Process chunks in parallel batches
        let mut all_embeddings: Vec<EmbeddingModelEmbedding> = Vec::new();
        let mut all_responses: Vec<Option<EmbedManyResultResponseData>> = Vec::new();
        let mut total_tokens = 0u32;
        let mut combined_provider_metadata: Option<SharedProviderMetadata> = None;

        for parallel_chunk in parallel_chunks {
            // Process chunks in this parallel batch concurrently
            let mut futures_vec = Vec::new();

            for chunk in parallel_chunk {
                let model = self.model.clone();
                let max_retries = retry_config.max_retries;
                let abort_signal_clone = self.abort_signal.clone();
                let headers = headers_with_user_agent.clone();
                let provider_options = self.provider_options.clone();

                let future = async move {
                    let retry_config = RetryConfig {
                        max_retries,
                        abort_signal: abort_signal_clone.clone(),
                    };

                    retry_config
                        .execute_with_boxed_error(move || {
                            let model = model.clone();
                            let chunk = chunk.clone();
                            let headers = headers.clone();
                            let provider_options = provider_options.clone();
                            let abort_signal = abort_signal_clone.clone();
                            async move {
                                let options = EmbeddingModelCallOptions {
                                    values: chunk,
                                    abort_signal,
                                    headers,
                                    provider_options,
                                };
                                model.do_embed(options).await
                            }
                        })
                        .await
                };

                futures_vec.push(future);
            }

            // Wait for all futures in this parallel batch to complete
            let results = futures::future::try_join_all(futures_vec).await?;

            // Collect results
            for result in results {
                all_embeddings.extend(result.embeddings);

                // Add response metadata
                let response = result.response.map(|r| {
                    EmbedManyResultResponseData::new()
                        .with_headers(r.headers.unwrap_or_default())
                        .with_body(r.body.unwrap_or_default())
                });
                all_responses.push(response);

                // Accumulate usage
                if let Some(usage) = result.usage {
                    total_tokens += usage.tokens;
                }

                // Merge provider metadata
                if let Some(metadata) = result.provider_metadata {
                    if let Some(ref mut combined) = combined_provider_metadata {
                        // Merge metadata from different provider calls
                        for (provider_name, provider_data) in metadata {
                            combined
                                .entry(provider_name.clone())
                                .or_insert_with(HashMap::new)
                                .extend(provider_data);
                        }
                    } else {
                        combined_provider_metadata = Some(metadata);
                    }
                }
            }
        }

        Ok(EmbedManyResult::new(
            self.values,
            all_embeddings,
            EmbeddingModelUsage::new(total_tokens),
        )
        .with_provider_metadata(combined_provider_metadata.unwrap_or_default())
        .with_responses(all_responses))
    }
}

/// Split an array into chunks of a specified size.
///
/// # Arguments
///
/// * `array` - The array to split
/// * `chunk_size` - The maximum size of each chunk
///
/// # Returns
///
/// A vector of chunks, where each chunk is a vector of elements.
///
/// # Example
///
/// ```
/// use ai_sdk_core::embed::many::split_array;
///
/// let values = vec![1, 2, 3, 4, 5];
/// let chunks = split_array(values, 2);
/// assert_eq!(chunks, vec![vec![1, 2], vec![3, 4], vec![5]]);
/// ```
pub fn split_array<T: Clone>(array: Vec<T>, chunk_size: usize) -> Vec<Vec<T>> {
    if chunk_size == 0 {
        return vec![array];
    }

    array
        .chunks(chunk_size)
        .map(|chunk| chunk.to_vec())
        .collect()
}

/// Add a user agent suffix to headers.
///
/// # Arguments
///
/// * `headers` - Optional existing headers
/// * `suffix` - The suffix to add to the user agent
///
/// # Returns
///
/// Headers with the user agent suffix added.
fn add_user_agent_suffix(headers: Option<SharedHeaders>, suffix: String) -> Option<SharedHeaders> {
    let mut headers = headers.unwrap_or_default();

    // Get existing user agent or use empty string
    let existing_user_agent = headers.get("user-agent").cloned().unwrap_or_default();

    // Add suffix
    let new_user_agent = if existing_user_agent.is_empty() {
        suffix
    } else {
        format!("{} {}", existing_user_agent, suffix)
    };

    headers.insert("user-agent".to_string(), new_user_agent);
    Some(headers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_array_basic() {
        let values = vec![1, 2, 3, 4, 5];
        let chunks = split_array(values, 2);
        assert_eq!(chunks, vec![vec![1, 2], vec![3, 4], vec![5]]);
    }

    #[test]
    fn test_split_array_exact_fit() {
        let values = vec![1, 2, 3, 4];
        let chunks = split_array(values, 2);
        assert_eq!(chunks, vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn test_split_array_single_chunk() {
        let values = vec![1, 2, 3];
        let chunks = split_array(values, 10);
        assert_eq!(chunks, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn test_split_array_single_element_chunks() {
        let values = vec![1, 2, 3];
        let chunks = split_array(values, 1);
        assert_eq!(chunks, vec![vec![1], vec![2], vec![3]]);
    }

    #[test]
    fn test_split_array_empty() {
        let values: Vec<i32> = vec![];
        let chunks = split_array(values, 2);
        assert_eq!(chunks, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_split_array_zero_chunk_size() {
        let values = vec![1, 2, 3];
        let chunks = split_array(values.clone(), 0);
        assert_eq!(chunks, vec![values]);
    }

    #[test]
    fn test_add_user_agent_suffix_no_existing_headers() {
        let headers = add_user_agent_suffix(None, "ai/1.0.0".to_string());
        assert!(headers.is_some());
        let headers = headers.unwrap();
        assert_eq!(headers.get("user-agent"), Some(&"ai/1.0.0".to_string()));
    }

    #[test]
    fn test_add_user_agent_suffix_with_existing_headers() {
        let mut existing = HashMap::new();
        existing.insert("content-type".to_string(), "application/json".to_string());

        let headers = add_user_agent_suffix(Some(existing), "ai/1.0.0".to_string());
        assert!(headers.is_some());
        let headers = headers.unwrap();
        assert_eq!(headers.get("user-agent"), Some(&"ai/1.0.0".to_string()));
        assert_eq!(
            headers.get("content-type"),
            Some(&"application/json".to_string())
        );
    }

    #[test]
    fn test_add_user_agent_suffix_with_existing_user_agent() {
        let mut existing = HashMap::new();
        existing.insert("user-agent".to_string(), "custom-agent/1.0".to_string());

        let headers = add_user_agent_suffix(Some(existing), "ai/1.0.0".to_string());
        assert!(headers.is_some());
        let headers = headers.unwrap();
        assert_eq!(
            headers.get("user-agent"),
            Some(&"custom-agent/1.0 ai/1.0.0".to_string())
        );
    }

    #[test]
    fn test_add_user_agent_suffix_empty_suffix() {
        let headers = add_user_agent_suffix(None, "".to_string());
        assert!(headers.is_some());
        let headers = headers.unwrap();
        assert_eq!(headers.get("user-agent"), Some(&"".to_string()));
    }
}
