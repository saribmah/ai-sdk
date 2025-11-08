use crate::embed::result::{EmbedResult, EmbedResultResponseData};
use crate::error::AISDKError;
use crate::generate_text::prepare_retries;
use ai_sdk_provider::embedding_model::call_options::EmbeddingModelCallOptions;
use ai_sdk_provider::embedding_model::{EmbeddingModel, EmbeddingModelUsage};
use ai_sdk_provider::shared::headers::SharedHeaders;
use ai_sdk_provider::shared::provider_options::SharedProviderOptions;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Builder for embedding a single value using an embedding model.
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::Embed;
/// use std::sync::Arc;
///
/// let result = Embed::new(model, "Hello, world!".to_string())
///     .max_retries(3)
///     .execute()
///     .await?;
///
/// println!("Embedding: {:?}", result.embedding);
/// println!("Usage: {:?}", result.usage);
/// ```
pub struct Embed<V>
where
    V: Clone + Send + Sync + 'static,
{
    model: Arc<dyn EmbeddingModel<V>>,
    value: V,
    max_retries: Option<u32>,
    abort_signal: Option<CancellationToken>,
    headers: Option<SharedHeaders>,
    provider_options: Option<SharedProviderOptions>,
}

impl<V> Embed<V>
where
    V: Clone + Send + Sync + 'static,
{
    /// Create a new Embed builder.
    ///
    /// # Arguments
    ///
    /// * `model` - The embedding model to use
    /// * `value` - The value that should be embedded
    pub fn new(model: Arc<dyn EmbeddingModel<V>>, value: V) -> Self {
        Self {
            model,
            value,
            max_retries: None,
            abort_signal: None,
            headers: None,
            provider_options: None,
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

    /// Execute the embedding operation.
    ///
    /// # Returns
    ///
    /// A result object that contains the embedding, the value, and additional information.
    pub async fn execute(self) -> Result<EmbedResult<V>, AISDKError> {
        // Prepare retry configuration
        let retry_config = prepare_retries(self.max_retries, self.abort_signal.clone())?;

        // Add user agent to headers
        let headers_with_user_agent =
            add_user_agent_suffix(self.headers, format!("ai/{}", VERSION));

        // Execute the embedding call with retry logic
        let result = retry_config
            .execute_with_boxed_error(|| {
                let model = self.model.clone();
                let value = self.value.clone();
                let headers = headers_with_user_agent.clone();
                let provider_options = self.provider_options.clone();
                async move {
                    let options = EmbeddingModelCallOptions {
                        values: vec![value],
                        abort_signal: None,
                        headers,
                        provider_options,
                    };
                    model.do_embed(options).await
                }
            })
            .await?;

        // Extract the first embedding (since we only embedded one value)
        let embedding = result
            .embeddings
            .into_iter()
            .next()
            .ok_or_else(|| AISDKError::model_error("No embedding returned from model"))?;

        let usage = result.usage.unwrap_or_else(|| EmbeddingModelUsage::new(0));
        let provider_metadata = result.provider_metadata;
        let response = result.response.map(|r| {
            EmbedResultResponseData::new()
                .with_headers(r.headers.unwrap_or_default())
                .with_body(r.body.unwrap_or_default())
        });

        Ok(EmbedResult::new(self.value, embedding, usage)
            .with_provider_metadata(provider_metadata.unwrap_or_default())
            .with_response(response.unwrap_or_default()))
    }
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
    use ai_sdk_provider::embedding_model::EmbeddingModelResponse;
    use ai_sdk_provider::embedding_model::embedding::EmbeddingModelEmbedding;
    use async_trait::async_trait;
    use std::collections::HashMap;

    // Mock embedding model for testing
    struct MockEmbeddingModel {
        embeddings: Vec<EmbeddingModelEmbedding>,
        should_fail: bool,
    }

    #[async_trait]
    impl EmbeddingModel<String> for MockEmbeddingModel {
        fn specification_version(&self) -> &str {
            "v3"
        }

        fn provider(&self) -> &str {
            "mock"
        }

        fn model_id(&self) -> &str {
            "mock-model"
        }

        async fn max_embeddings_per_call(&self) -> Option<usize> {
            Some(10)
        }

        async fn supports_parallel_calls(&self) -> bool {
            true
        }

        async fn do_embed(
            &self,
            _options: EmbeddingModelCallOptions<String>,
        ) -> Result<EmbeddingModelResponse, Box<dyn std::error::Error>> {
            if self.should_fail {
                return Err("Mock error".into());
            }

            Ok(EmbeddingModelResponse::new(self.embeddings.clone())
                .with_usage(EmbeddingModelUsage::new(10)))
        }
    }

    #[tokio::test]
    async fn test_embed_basic() {
        let model = Arc::new(MockEmbeddingModel {
            embeddings: vec![vec![0.1, 0.2, 0.3]],
            should_fail: false,
        }) as Arc<dyn EmbeddingModel<String>>;

        let result = Embed::new(model, "Hello, world!".to_string())
            .max_retries(2)
            .execute()
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.value, "Hello, world!");
        assert_eq!(result.embedding, vec![0.1, 0.2, 0.3]);
        assert_eq!(result.usage.tokens, 10);
    }

    #[tokio::test]
    async fn test_embed_with_headers() {
        let model = Arc::new(MockEmbeddingModel {
            embeddings: vec![vec![0.5, 0.6]],
            should_fail: false,
        }) as Arc<dyn EmbeddingModel<String>>;

        let mut headers = HashMap::new();
        headers.insert("x-custom-header".to_string(), "custom-value".to_string());

        let result = Embed::new(model, "Test".to_string())
            .max_retries(2)
            .headers(headers)
            .execute()
            .await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.embedding, vec![0.5, 0.6]);
    }

    #[tokio::test]
    async fn test_embed_with_retry() {
        let model = Arc::new(MockEmbeddingModel {
            embeddings: vec![vec![0.1]],
            should_fail: false,
        }) as Arc<dyn EmbeddingModel<String>>;

        let result = Embed::new(model, "Retry test".to_string())
            .max_retries(3)
            .execute()
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_embed_no_retries() {
        let model = Arc::new(MockEmbeddingModel {
            embeddings: vec![vec![0.1, 0.2]],
            should_fail: false,
        }) as Arc<dyn EmbeddingModel<String>>;

        let result = Embed::new(model, "No retry".to_string())
            .max_retries(0)
            .execute()
            .await;

        assert!(result.is_ok());
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
