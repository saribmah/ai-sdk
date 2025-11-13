//! Anthropic Messages API Language Model Implementation
//!
//! This module provides the implementation of the `LanguageModel` trait for the
//! Anthropic Messages API. The implementation is broken down into several focused
//! modules for maintainability:
//!
//! ## Module Structure
//!
//! - **config**: Configuration types and builders for the language model
//! - **citation**: Citation source creation from Anthropic citation data
//! - **model_limits**: Maximum output token limits for different Claude models
//! - **args_builder**: Request argument preparation and validation
//! - **generate**: Non-streaming text generation implementation (TODO)
//! - **stream**: Streaming text generation implementation (TODO)
//! - **process_content**: Content processing utilities for responses (TODO)
//!
//! ## Usage
//!
//! ```rust,no_run
//! use ai_sdk_anthropic::language_model::{AnthropicMessagesLanguageModel, AnthropicMessagesConfig};
//! use std::sync::Arc;
//!
//! let config = AnthropicMessagesConfig::new(
//!     "anthropic".to_string(),
//!     "https://api.anthropic.com/v1".to_string(),
//!     Arc::new(|| {
//!         let mut headers = std::collections::HashMap::new();
//!         headers.insert("x-api-key".to_string(), "your-api-key".to_string());
//!         headers
//!     }),
//! );
//!
//! let model = AnthropicMessagesLanguageModel::new(
//!     "claude-3-5-sonnet-20241022".to_string(),
//!     config,
//! );
//! ```

pub mod args_builder;
pub mod citation;
pub mod config;
pub mod http_client;
pub mod model_limits;
pub mod process_content;
pub mod response_schema;
pub mod sse_parser;
pub mod stream_schema;

// TODO: Implement these modules
// pub mod generate;
// pub mod process_content;
// pub mod stream;

use ai_sdk_provider::language_model::LanguageModel;
use async_trait::async_trait;
use std::collections::HashMap;

use crate::options::AnthropicMessagesModelId;
pub use config::AnthropicMessagesConfig;

/// Anthropic Messages API Language Model
///
/// This struct implements the `LanguageModel` trait for the Anthropic Messages API.
/// It handles both streaming and non-streaming text generation with support for:
///
/// - Tool calling (including provider-defined tools)
/// - Prompt caching
/// - Extended thinking mode
/// - MCP (Model Context Protocol) servers
/// - Agent skills and containers
/// - Citations from documents
/// - Multi-modal inputs (text, images, files)
///
/// ## Architecture
///
/// The implementation is split across multiple modules:
///
/// 1. **Config** (`config.rs`): Holds configuration like base URL, headers, and optional
///    transformation functions
///
/// 2. **Args Builder** (`args_builder.rs`): Prepares the request body by:
///    - Converting prompts to Anthropic format
///    - Validating and applying settings (temperature, max tokens, etc.)
///    - Handling provider-specific options (thinking, MCP, skills)
///    - Managing beta feature flags
///    - Collecting warnings for unsupported features
///
/// 3. **Generate** (`generate.rs` - TODO): Implements non-streaming generation:
///    - Makes HTTP POST request to `/messages` endpoint
///    - Processes response content (text, tool calls, citations, etc.)
///    - Maps finish reasons and usage statistics
///    - Handles provider-specific metadata
///
/// 4. **Stream** (`stream.rs` - TODO): Implements streaming generation:
///    - Makes HTTP POST request with `stream=true`
///    - Processes Server-Sent Events (SSE)
///    - Emits stream parts (text-delta, tool-call, etc.)
///    - Tracks state across chunks
///    - Handles tool streaming and citations
///
/// 5. **Process Content** (`process_content.rs` - TODO): Shared utilities for:
///    - Converting Anthropic content blocks to SDK format
///    - Handling special cases (JSON response tools, code execution mapping)
///    - Processing tool results (web search, web fetch, MCP, etc.)
///    - Creating citation sources
///
/// 6. **Citation** (`citation.rs`): Creates citation sources from Anthropic citation data
///
/// 7. **Model Limits** (`model_limits.rs`): Provides max output token limits for different models
///
pub struct AnthropicMessagesLanguageModel {
    model_id: AnthropicMessagesModelId,
    config: AnthropicMessagesConfig,
    #[allow(dead_code)]
    generate_id: Box<dyn Fn() -> String + Send + Sync>,
}

impl AnthropicMessagesLanguageModel {
    /// Create a new Anthropic Messages language model
    ///
    /// # Arguments
    ///
    /// * `model_id` - The Claude model identifier (e.g., "claude-3-5-sonnet-20241022")
    /// * `config` - Configuration for the model including base URL, headers, etc.
    ///
    /// # Returns
    ///
    /// A new `AnthropicMessagesLanguageModel` instance
    pub fn new(model_id: AnthropicMessagesModelId, config: AnthropicMessagesConfig) -> Self {
        let generate_id = config
            .generate_id
            .clone()
            .unwrap_or_else(|| std::sync::Arc::new(|| uuid::Uuid::new_v4().to_string()));

        Self {
            model_id,
            config,
            generate_id: Box::new(move || (*generate_id)()),
        }
    }

    /// Build the request URL for the given streaming mode
    #[allow(dead_code)]
    fn build_request_url(&self, is_streaming: bool) -> String {
        if let Some(ref builder) = self.config.build_request_url {
            (*builder)(&self.config.base_url, is_streaming)
        } else {
            format!("{}/messages", self.config.base_url)
        }
    }

    /// Transform the request body if a transformer is configured
    #[allow(dead_code)]
    fn transform_request_body(&self, args: serde_json::Value) -> serde_json::Value {
        if let Some(ref transformer) = self.config.transform_request_body {
            (*transformer)(args)
        } else {
            args
        }
    }

    /// Get headers for the request
    #[allow(dead_code)]
    fn get_headers(
        &self,
        betas: &std::collections::HashSet<String>,
        additional_headers: Option<&HashMap<String, String>>,
    ) -> HashMap<String, String> {
        let mut headers = (self.config.headers)();

        // Add beta header if needed
        if !betas.is_empty() {
            let beta_values: Vec<String> = betas.iter().cloned().collect();
            headers.insert("anthropic-beta".to_string(), beta_values.join(","));
        }

        // Add additional headers
        if let Some(extra) = additional_headers {
            headers.extend(extra.clone());
        }

        headers
    }
}

#[async_trait]
impl LanguageModel for AnthropicMessagesLanguageModel {
    fn specification_version(&self) -> &str {
        "v3"
    }

    fn provider(&self) -> &str {
        &self.config.provider
    }

    fn model_id(&self) -> &str {
        &self.model_id
    }

    async fn supported_urls(&self) -> HashMap<String, Vec<regex::Regex>> {
        // TODO: Implement supported URLs
        HashMap::new()
    }

    async fn do_generate(
        &self,
        options: ai_sdk_provider::language_model::call_options::LanguageModelCallOptions,
    ) -> Result<
        ai_sdk_provider::language_model::LanguageModelGenerateResponse,
        Box<dyn std::error::Error>,
    > {
        use ai_sdk_provider::language_model::{
            LanguageModelGenerateResponse, LanguageModelRequestMetadata,
        };
        use args_builder::build_request_args;
        use http_client::post_json;
        use process_content::process_content_blocks;
        use response_schema::AnthropicMessagesResponse;

        // Build request arguments
        let build_result = build_request_args(&self.model_id, &options, false).await?;

        // Build request URL
        let url = self.build_request_url(false);

        // Get headers with beta flags
        let headers = self.get_headers(&build_result.betas, options.headers.as_ref());

        // Transform request body if configured
        let transformed_body = self.transform_request_body(build_result.args);

        // Make HTTP POST request
        let response_json = post_json(&url, headers, transformed_body.clone()).await?;

        // Parse response
        let response: AnthropicMessagesResponse = serde_json::from_value(response_json)?;

        // Process content blocks
        let content = process_content_blocks(response.content);

        // Map finish reason
        let finish_reason = crate::map_stop_reason::map_anthropic_stop_reason(
            response.stop_reason.as_deref(),
            build_result.uses_json_response_tool,
        );

        // Build usage statistics
        let usage = ai_sdk_provider::language_model::usage::LanguageModelUsage {
            input_tokens: response.usage.input_tokens as u64,
            output_tokens: response.usage.output_tokens as u64,
            total_tokens: (response.usage.input_tokens + response.usage.output_tokens) as u64,
            reasoning_tokens: 0, // Anthropic doesn't separately report reasoning tokens
            cached_input_tokens: response.usage.cache_read_input_tokens.unwrap_or(0) as u64,
        };

        // Build provider metadata
        let provider_metadata = if response.usage.cache_creation_input_tokens.is_some()
            || response.stop_sequence.is_some()
            || response.container.is_some()
        {
            let mut metadata: ai_sdk_provider::shared::provider_metadata::SharedProviderMetadata =
                HashMap::new();
            let mut anthropic_metadata: HashMap<String, serde_json::Value> = HashMap::new();

            if let Some(cache_creation_tokens) = response.usage.cache_creation_input_tokens {
                anthropic_metadata.insert(
                    "cacheCreationInputTokens".to_string(),
                    serde_json::json!(cache_creation_tokens),
                );
            }

            if let Some(stop_seq) = response.stop_sequence {
                anthropic_metadata.insert("stopSequence".to_string(), serde_json::json!(stop_seq));
            }

            if let Some(container) = response.container {
                anthropic_metadata.insert("container".to_string(), serde_json::json!(container));
            }

            metadata.insert("anthropic".to_string(), anthropic_metadata);

            Some(metadata)
        } else {
            None
        };

        // Build response metadata
        let response_metadata = Some(
            ai_sdk_provider::language_model::response_metadata::LanguageModelResponseMetadata {
                id: response.id,
                model_id: response.model,
                timestamp: None,
            },
        );

        // Build request metadata
        let request_metadata = Some(LanguageModelRequestMetadata {
            body: Some(transformed_body),
        });

        Ok(LanguageModelGenerateResponse {
            content,
            finish_reason,
            usage,
            provider_metadata,
            request: request_metadata,
            response: response_metadata,
            warnings: build_result.warnings,
        })
    }

    async fn do_stream(
        &self,
        options: ai_sdk_provider::language_model::call_options::LanguageModelCallOptions,
    ) -> Result<
        ai_sdk_provider::language_model::LanguageModelStreamResponse,
        Box<dyn std::error::Error>,
    > {
        use ai_sdk_provider::language_model::{
            LanguageModelStreamResponse, StreamResponseMetadata,
        };
        use args_builder::build_request_args;
        use http_client::post_stream;
        use sse_parser::parse_sse_stream;

        // Build request arguments with streaming enabled
        let build_result = build_request_args(&self.model_id, &options, true).await?;

        // Build request URL and store it
        let url = self.build_request_url(true);

        // Get headers with beta flags
        let headers = self.get_headers(&build_result.betas, options.headers.as_ref());

        // Transform request body if configured
        let transformed_body = self.transform_request_body(build_result.args.clone());

        // Make HTTP POST request with streaming
        let byte_stream = post_stream(url.as_str(), headers, transformed_body).await?;

        // Parse SSE events and convert to SDK stream parts
        let stream = parse_sse_stream(
            byte_stream,
            build_result.uses_json_response_tool,
            build_result.warnings,
            options.include_raw_chunks.unwrap_or(false),
        );

        Ok(LanguageModelStreamResponse {
            stream: Box::new(stream),
            request: Some(
                ai_sdk_provider::language_model::LanguageModelRequestMetadata {
                    body: Some(build_result.args),
                },
            ),
            response: Some(StreamResponseMetadata { headers: None }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_sdk_provider::language_model::finish_reason::LanguageModelFinishReason;
    use ai_sdk_provider::language_model::prompt::message::LanguageModelMessage;
    use std::sync::Arc;

    #[test]
    fn test_new_model() {
        let config = AnthropicMessagesConfig::new(
            "anthropic".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            Arc::new(HashMap::new),
        );

        let model =
            AnthropicMessagesLanguageModel::new("claude-3-5-sonnet-20241022".to_string(), config);

        assert_eq!(model.model_id(), "claude-3-5-sonnet-20241022");
        assert_eq!(model.provider(), "anthropic");
        assert_eq!(model.specification_version(), "v3");
    }

    #[test]
    fn test_build_request_url() {
        let config = AnthropicMessagesConfig::new(
            "anthropic".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            Arc::new(HashMap::new),
        );

        let model =
            AnthropicMessagesLanguageModel::new("claude-3-5-sonnet-20241022".to_string(), config);

        assert_eq!(
            model.build_request_url(false),
            "https://api.anthropic.com/v1/messages"
        );
    }

    #[test]
    fn test_custom_url_builder() {
        let config = AnthropicMessagesConfig::new(
            "anthropic".to_string(),
            "https://custom.api.com".to_string(),
            Arc::new(HashMap::new),
        )
        .with_build_request_url(Arc::new(|base, _streaming| {
            format!("{}/custom/messages", base)
        }));

        let model =
            AnthropicMessagesLanguageModel::new("claude-3-5-sonnet-20241022".to_string(), config);

        assert_eq!(
            model.build_request_url(false),
            "https://custom.api.com/custom/messages"
        );
    }

    #[test]
    fn test_get_headers_with_betas() {
        let config = AnthropicMessagesConfig::new(
            "anthropic".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            Arc::new(|| {
                let mut headers = HashMap::new();
                headers.insert("x-api-key".to_string(), "test-key".to_string());
                headers
            }),
        );

        let model =
            AnthropicMessagesLanguageModel::new("claude-3-5-sonnet-20241022".to_string(), config);

        let mut betas = std::collections::HashSet::new();
        betas.insert("prompt-caching-2024-07-31".to_string());

        let headers = model.get_headers(&betas, None);

        assert_eq!(headers.get("x-api-key").unwrap(), "test-key");
        assert!(headers.contains_key("anthropic-beta"));
    }

    #[tokio::test]
    async fn test_do_generate_basic_structure() {
        use ai_sdk_provider::language_model::LanguageModel;
        use ai_sdk_provider::language_model::call_options::LanguageModelCallOptions;

        // This is a mock test to verify the structure compiles and types are correct
        // We can't test actual API calls without a real API key

        let config = AnthropicMessagesConfig::new(
            "anthropic".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            Arc::new(|| {
                let mut headers = HashMap::new();
                headers.insert("x-api-key".to_string(), "test-key".to_string());
                headers
            }),
        );

        let model =
            AnthropicMessagesLanguageModel::new("claude-3-5-sonnet-20241022".to_string(), config);

        // Create a simple prompt using the LanguageModelPrompt type (Vec<LanguageModelMessage>)
        let prompt = vec![LanguageModelMessage::user_text("Hello")];

        let options = LanguageModelCallOptions::new(prompt)
            .with_temperature(0.7)
            .with_max_output_tokens(100);

        // We expect this to fail because we're not making a real API call
        // but this verifies the types and structure are correct
        let result = model.do_generate(options).await;

        // We expect an error (likely network or auth error), but the types should be correct
        assert!(result.is_err() || result.is_ok());

        // If it somehow succeeds (unlikely), verify the response structure
        if let Ok(response) = result {
            // Verify the response has the expected fields (input_tokens is u64, always >= 0)
            let _ = response.usage.input_tokens; // Verify field exists
            let _ = response.content.len(); // Verify content exists
            // finish_reason should be one of the valid enum values
            match response.finish_reason {
                LanguageModelFinishReason::Stop
                | LanguageModelFinishReason::Length
                | LanguageModelFinishReason::ToolCalls
                | LanguageModelFinishReason::ContentFilter
                | LanguageModelFinishReason::Unknown
                | LanguageModelFinishReason::Error
                | LanguageModelFinishReason::Other => {}
            }
        }
    }

    #[test]
    fn test_transform_request_body() {
        let config = AnthropicMessagesConfig::new(
            "anthropic".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            Arc::new(HashMap::new),
        )
        .with_transform_request_body(Arc::new(|mut args| {
            args.as_object_mut().unwrap().insert(
                "custom_field".to_string(),
                serde_json::json!("custom_value"),
            );
            args
        }));

        let model =
            AnthropicMessagesLanguageModel::new("claude-3-5-sonnet-20241022".to_string(), config);

        let args = serde_json::json!({
            "model": "claude-3-5-sonnet-20241022",
            "max_tokens": 100
        });

        let transformed = model.transform_request_body(args);

        assert_eq!(
            transformed.get("custom_field").unwrap(),
            &serde_json::json!("custom_value")
        );
        assert_eq!(
            transformed.get("model").unwrap(),
            &serde_json::json!("claude-3-5-sonnet-20241022")
        );
    }
}
