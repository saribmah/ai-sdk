use ai_sdk_provider::language_model::stream_part::LanguageModelStreamPart;
use ai_sdk_provider::language_model::{
    LanguageModel, LanguageModelGenerateResponse, LanguageModelStreamResponse,
    call_options::LanguageModelCallOptions, call_warning::LanguageModelCallWarning,
    content::LanguageModelContent, content::text::LanguageModelText, usage::LanguageModelUsage,
};
use async_trait::async_trait;
use futures_util::{Stream, StreamExt};
use regex::Regex;
use reqwest;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;

use crate::completion::{
    OpenAICompatibleCompletionModelId, convert_to_openai_compatible_completion_prompt,
};
use crate::utils::get_response_metadata::get_response_metadata;
use crate::utils::map_finish_reason::map_openai_compatible_finish_reason;

/// Configuration for an OpenAI-compatible completion language model
pub struct OpenAICompatibleCompletionConfig {
    /// Provider name (e.g., "openai", "azure", "custom")
    pub provider: String,

    /// Function to generate headers for API requests
    pub headers: Box<dyn Fn() -> HashMap<String, String> + Send + Sync>,

    /// Function to generate the URL for API requests
    pub url: Box<dyn Fn(&str, &str) -> String + Send + Sync>,

    /// Optional custom fetch function
    pub fetch: Option<fn()>, // TODO: proper fetch function type

    /// Whether to include usage information in streaming responses
    pub include_usage: bool,
}

impl Default for OpenAICompatibleCompletionConfig {
    fn default() -> Self {
        Self {
            provider: "openai-compatible".to_string(),
            headers: Box::new(|| HashMap::new()),
            url: Box::new(|_model_id, path| format!("https://api.openai.com/v1{}", path)),
            fetch: None,
            include_usage: false,
        }
    }
}

/// OpenAI-compatible completion language model implementation
pub struct OpenAICompatibleCompletionLanguageModel {
    /// The model identifier
    model_id: OpenAICompatibleCompletionModelId,

    /// Configuration for the model
    config: OpenAICompatibleCompletionConfig,
}

impl OpenAICompatibleCompletionLanguageModel {
    /// Create a new OpenAI-compatible completion language model
    ///
    /// # Arguments
    ///
    /// * `model_id` - The model identifier (e.g., "gpt-3.5-turbo-instruct")
    /// * `config` - Configuration for the model
    ///
    /// # Returns
    ///
    /// A new `OpenAICompatibleCompletionLanguageModel` instance
    pub fn new(
        model_id: OpenAICompatibleCompletionModelId,
        config: OpenAICompatibleCompletionConfig,
    ) -> Self {
        Self { model_id, config }
    }

    /// Get the provider options name (first part of provider string before '.')
    fn provider_options_name(&self) -> &str {
        self.config
            .provider
            .split('.')
            .next()
            .unwrap_or(&self.config.provider)
    }

    /// Prepare arguments for API request
    fn prepare_request_body(
        &self,
        options: &LanguageModelCallOptions,
    ) -> Result<(Value, Vec<LanguageModelCallWarning>), Box<dyn std::error::Error>> {
        let mut warnings = Vec::new();

        // Check for unsupported settings
        if options.top_k.is_some() {
            warnings.push(LanguageModelCallWarning::UnsupportedSetting {
                setting: "topK".to_string(),
                details: None,
            });
        }

        if options.tools.is_some() {
            warnings.push(LanguageModelCallWarning::UnsupportedSetting {
                setting: "tools".to_string(),
                details: None,
            });
        }

        if options.tool_choice.is_some() {
            warnings.push(LanguageModelCallWarning::UnsupportedSetting {
                setting: "toolChoice".to_string(),
                details: None,
            });
        }

        // Convert prompt to completion format
        let completion_result =
            convert_to_openai_compatible_completion_prompt(options.prompt.clone(), None, None)?;

        // Combine stop sequences
        let mut stop_sequences = completion_result.stop_sequences.unwrap_or_default();
        if let Some(user_stop) = &options.stop_sequences {
            stop_sequences.extend(user_stop.clone());
        }

        // Build request body
        let mut body = json!({
            "model": self.model_id,
            "prompt": completion_result.prompt,
        });

        // Add optional parameters
        if let Some(max_tokens) = options.max_output_tokens {
            body["max_tokens"] = json!(max_tokens);
        }
        if let Some(temperature) = options.temperature {
            body["temperature"] = json!(temperature);
        }
        if let Some(top_p) = options.top_p {
            body["top_p"] = json!(top_p);
        }
        if let Some(frequency_penalty) = options.frequency_penalty {
            body["frequency_penalty"] = json!(frequency_penalty);
        }
        if let Some(presence_penalty) = options.presence_penalty {
            body["presence_penalty"] = json!(presence_penalty);
        }
        if let Some(seed) = options.seed {
            body["seed"] = json!(seed);
        }

        // Add stop sequences if present
        if !stop_sequences.is_empty() {
            body["stop"] = json!(stop_sequences);
        }

        Ok((body, warnings))
    }
}

/// OpenAI completion API response structure
#[derive(Debug, Deserialize)]
struct OpenAICompletionResponse {
    id: Option<String>,
    created: Option<i64>,
    model: Option<String>,
    choices: Vec<OpenAICompletionChoice>,
    usage: Option<OpenAICompletionUsage>,
}

#[derive(Debug, Deserialize)]
struct OpenAICompletionChoice {
    text: String,
    finish_reason: String,
}

#[derive(Debug, Deserialize)]
struct OpenAICompletionUsage {
    prompt_tokens: Option<u64>,
    completion_tokens: Option<u64>,
    total_tokens: Option<u64>,
}

/// OpenAI streaming completion chunk
#[derive(Debug, Deserialize)]
struct OpenAICompletionChunk {
    id: Option<String>,
    created: Option<i64>,
    model: Option<String>,
    choices: Vec<OpenAICompletionStreamChoice>,
    usage: Option<OpenAICompletionUsage>,
}

#[derive(Debug, Deserialize)]
struct OpenAICompletionStreamChoice {
    text: String,
    finish_reason: Option<String>,
    index: u64,
}

#[async_trait]
impl LanguageModel for OpenAICompatibleCompletionLanguageModel {
    fn specification_version(&self) -> &str {
        "v3"
    }

    fn provider(&self) -> &str {
        &self.config.provider
    }

    fn model_id(&self) -> &str {
        &self.model_id
    }

    async fn supported_urls(&self) -> HashMap<String, Vec<Regex>> {
        HashMap::new()
    }

    async fn do_generate(
        &self,
        options: LanguageModelCallOptions,
    ) -> Result<LanguageModelGenerateResponse, Box<dyn std::error::Error>> {
        // Prepare request body
        let (body, mut warnings) = self.prepare_request_body(&options)?;
        let body_string = serde_json::to_string(&body)?;

        // Build URL
        let url = (self.config.url)(&self.model_id, "/completions");

        // Build headers
        let mut headers = (self.config.headers)();
        if let Some(option_headers) = &options.headers {
            headers.extend(option_headers.clone());
        }

        // Make API request
        let client = reqwest::Client::new();
        let mut request = client.post(&url).header("Content-Type", "application/json");

        for (key, value) in headers {
            request = request.header(key, value);
        }

        let response = request.body(body_string.clone()).send().await?;
        let status = response.status();
        let response_headers = response.headers().clone();

        if !status.is_success() {
            let error_body = response.text().await?;

            // Check for Retry-After header to include in error message
            let retry_info = if let Some(retry_after) = response_headers.get("retry-after") {
                if let Ok(retry_str) = retry_after.to_str() {
                    // Try to parse as seconds (integer)
                    if let Ok(seconds) = retry_str.parse::<u64>() {
                        format!(" (Retry after {} seconds)", seconds)
                    } else {
                        // Could be HTTP-date format, but for now just include the raw value
                        format!(" (Retry-After: {})", retry_str)
                    }
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            return Err(format!(
                "API request failed with status {}: {}{}",
                status, error_body, retry_info
            )
            .into());
        }

        let response_body = response.text().await?;
        let api_response: OpenAICompletionResponse = serde_json::from_str(&response_body)?;

        // Extract content from response
        let choice = api_response
            .choices
            .first()
            .ok_or("No choices in response")?;

        let mut content = Vec::new();

        // Add text content
        if !choice.text.is_empty() {
            content.push(LanguageModelContent::Text(LanguageModelText::new(
                choice.text.clone(),
            )));
        }

        // Build usage information
        let usage = if let Some(api_usage) = &api_response.usage {
            LanguageModelUsage {
                input_tokens: api_usage.prompt_tokens.unwrap_or(0),
                output_tokens: api_usage.completion_tokens.unwrap_or(0),
                total_tokens: api_usage.total_tokens.unwrap_or(0),
                reasoning_tokens: 0,
                cached_input_tokens: 0,
            }
        } else {
            LanguageModelUsage::default()
        };

        // Build provider metadata with response headers
        let mut provider_metadata = HashMap::new();
        let mut provider_data = HashMap::new();

        // Add HTTP response headers to provider metadata for debugging
        // Headers are prefixed with "header." to distinguish them from other metadata
        // Common useful headers: x-ratelimit-*, x-request-id, retry-after, etc.
        for (key, value) in response_headers.iter() {
            if let Ok(value_str) = value.to_str() {
                provider_data.insert(format!("header.{}", key.as_str()), json!(value_str));
            }
        }

        provider_metadata.insert(self.provider_options_name().to_string(), provider_data);

        // Build response metadata
        let response_metadata = get_response_metadata(
            api_response.id.clone(),
            api_response.model.clone(),
            api_response.created,
        );

        // Map finish reason
        let finish_reason = map_openai_compatible_finish_reason(Some(&choice.finish_reason));

        Ok(LanguageModelGenerateResponse {
            content,
            finish_reason,
            usage,
            provider_metadata: Some(provider_metadata),
            request: Some(
                ai_sdk_provider::language_model::LanguageModelRequestMetadata { body: Some(body) },
            ),
            response: Some(response_metadata),
            warnings,
        })
    }

    async fn do_stream(
        &self,
        options: LanguageModelCallOptions,
    ) -> Result<LanguageModelStreamResponse, Box<dyn std::error::Error>> {
        // Prepare request body with streaming enabled
        let (mut body, warnings) = self.prepare_request_body(&options)?;
        body["stream"] = json!(true);

        // Add stream_options if include_usage is enabled
        if self.config.include_usage {
            body["stream_options"] = json!({
                "include_usage": true
            });
        }

        let body_string = serde_json::to_string(&body)?;

        // Build URL
        let url = (self.config.url)(&self.model_id, "/completions");

        // Build headers
        let mut headers = (self.config.headers)();
        if let Some(option_headers) = &options.headers {
            headers.extend(option_headers.clone());
        }

        // Make streaming API request
        let client = reqwest::Client::new();
        let mut request = client.post(&url).header("Content-Type", "application/json");

        for (key, value) in headers {
            request = request.header(key, value);
        }

        let response = request.body(body_string.clone()).send().await?;
        let status = response.status();
        let response_headers = response.headers().clone();

        if !status.is_success() {
            let error_body = response.text().await?;

            // Check for Retry-After header to include in error message
            let retry_info = if let Some(retry_after) = response_headers.get("retry-after") {
                if let Ok(retry_str) = retry_after.to_str() {
                    // Try to parse as seconds (integer)
                    if let Ok(seconds) = retry_str.parse::<u64>() {
                        format!(" (Retry after {} seconds)", seconds)
                    } else {
                        // Could be HTTP-date format, but for now just include the raw value
                        format!(" (Retry-After: {})", retry_str)
                    }
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            return Err(format!(
                "API request failed with status {}: {}{}",
                status, error_body, retry_info
            )
            .into());
        }

        // Build headers map from HTTP response headers
        let mut headers_map = HashMap::new();
        for (key, value) in response_headers.iter() {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(key.as_str().to_string(), value_str.to_string());
            }
        }

        // Create the stream processor
        let byte_stream = response.bytes_stream();

        // Process SSE events and convert to StreamPart
        let stream = Self::process_stream(byte_stream, warnings);

        Ok(LanguageModelStreamResponse {
            stream: Box::new(stream),
            request: Some(
                ai_sdk_provider::language_model::LanguageModelRequestMetadata { body: Some(body) },
            ),
            response: Some(ai_sdk_provider::language_model::StreamResponseMetadata {
                headers: Some(headers_map),
            }),
        })
    }
}

impl OpenAICompatibleCompletionLanguageModel {
    /// Process SSE byte stream and convert to StreamPart events
    fn process_stream(
        byte_stream: impl Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Send + 'static,
        warnings: Vec<LanguageModelCallWarning>,
    ) -> impl Stream<Item = LanguageModelStreamPart> + Unpin + Send {
        let mut buffer = String::new();
        let mut is_first_chunk = true;

        Box::pin(async_stream::stream! {
            // Emit stream start with warnings
            yield LanguageModelStreamPart::stream_start(warnings);

            let mut stream = Box::pin(byte_stream);
            let mut finish_reason = ai_sdk_provider::language_model::finish_reason::LanguageModelFinishReason::Unknown;
            let mut usage = LanguageModelUsage::default();

            while let Some(result) = stream.next().await {
                match result {
                    Ok(bytes) => {
                        // Append to buffer
                        if let Ok(text) = std::str::from_utf8(&bytes) {
                            buffer.push_str(text);

                            // Process complete lines
                            while let Some(pos) = buffer.find('\n') {
                                let line = buffer[..pos].trim().to_string();
                                buffer.drain(..=pos);

                                // Skip empty lines
                                if line.is_empty() {
                                    continue;
                                }

                                // Parse SSE format
                                if let Some(data) = line.strip_prefix("data: ") {
                                    // Check for [DONE] marker
                                    if data == "[DONE]" {
                                        break;
                                    }

                                    // Parse JSON chunk
                                    if let Ok(chunk) = serde_json::from_str::<OpenAICompletionChunk>(data) {
                                        if is_first_chunk {
                                            is_first_chunk = false;

                                            // Emit response metadata
                                            yield LanguageModelStreamPart::ResponseMetadata(
                                                get_response_metadata(chunk.id.clone(), chunk.model.clone(), chunk.created)
                                            );

                                            // Emit text start
                                            yield LanguageModelStreamPart::text_start("0");
                                        }

                                        // Update usage if present
                                        if let Some(api_usage) = &chunk.usage {
                                            usage.input_tokens = api_usage.prompt_tokens.unwrap_or(0);
                                            usage.output_tokens = api_usage.completion_tokens.unwrap_or(0);
                                            usage.total_tokens = api_usage.total_tokens.unwrap_or(0);
                                        }

                                        // Process choice
                                        if let Some(choice) = chunk.choices.first() {
                                            // Update finish reason if present
                                            if let Some(reason) = &choice.finish_reason {
                                                finish_reason = map_openai_compatible_finish_reason(Some(reason));
                                            }

                                            // Emit text delta
                                            if !choice.text.is_empty() {
                                                yield LanguageModelStreamPart::text_delta("0", &choice.text);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        yield LanguageModelStreamPart::error(json!({ "message": e.to_string() }));
                        break;
                    }
                }
            }

            // Emit text end if we started
            if !is_first_chunk {
                yield LanguageModelStreamPart::text_end("0");
            }

            // Emit finish event
            yield LanguageModelStreamPart::finish(usage, finish_reason);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_model() {
        let config = OpenAICompatibleCompletionConfig::default();
        let model = OpenAICompatibleCompletionLanguageModel::new(
            "gpt-3.5-turbo-instruct".to_string(),
            config,
        );

        assert_eq!(model.model_id(), "gpt-3.5-turbo-instruct");
        assert_eq!(model.provider(), "openai-compatible");
        assert_eq!(model.specification_version(), "v3");
    }

    #[test]
    fn test_provider_options_name() {
        let mut config = OpenAICompatibleCompletionConfig::default();
        config.provider = "openai.azure".to_string();
        let model = OpenAICompatibleCompletionLanguageModel::new(
            "gpt-3.5-turbo-instruct".to_string(),
            config,
        );

        assert_eq!(model.provider_options_name(), "openai");
    }

    #[test]
    fn test_provider_options_name_no_dot() {
        let mut config = OpenAICompatibleCompletionConfig::default();
        config.provider = "custom".to_string();
        let model =
            OpenAICompatibleCompletionLanguageModel::new("custom-model".to_string(), config);

        assert_eq!(model.provider_options_name(), "custom");
    }

    #[tokio::test]
    async fn test_supported_urls_empty() {
        let config = OpenAICompatibleCompletionConfig::default();
        let model = OpenAICompatibleCompletionLanguageModel::new(
            "gpt-3.5-turbo-instruct".to_string(),
            config,
        );

        let urls = model.supported_urls().await;
        assert_eq!(urls.len(), 0);
    }

    #[test]
    fn test_config_default() {
        let config = OpenAICompatibleCompletionConfig::default();

        assert_eq!(config.provider, "openai-compatible");
        assert!(!config.include_usage);
    }

    #[test]
    fn test_config_custom_provider() {
        let mut config = OpenAICompatibleCompletionConfig::default();
        config.provider = "azure".to_string();

        assert_eq!(config.provider, "azure");
    }
}
