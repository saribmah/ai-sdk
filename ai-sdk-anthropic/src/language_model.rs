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
pub mod model_limits;

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
        _options: ai_sdk_provider::language_model::call_options::LanguageModelCallOptions,
    ) -> Result<
        ai_sdk_provider::language_model::LanguageModelGenerateResponse,
        Box<dyn std::error::Error>,
    > {
        // TODO: Implement generate
        // This should:
        // 1. Build request args using args_builder::build_request_args
        // 2. Make HTTP POST request to build_request_url(false)
        // 3. Process response using process_content utilities
        // 4. Return LanguageModelGenerateResponse with content, usage, warnings, etc.
        Err("Not yet implemented".into())
    }

    async fn do_stream(
        &self,
        _options: ai_sdk_provider::language_model::call_options::LanguageModelCallOptions,
    ) -> Result<
        ai_sdk_provider::language_model::LanguageModelStreamResponse,
        Box<dyn std::error::Error>,
    > {
        // TODO: Implement stream
        // This should:
        // 1. Build request args using args_builder::build_request_args with stream=true
        // 2. Make HTTP POST request to build_request_url(true)
        // 3. Process SSE stream and emit LanguageModelStreamPart events
        // 4. Track state across chunks (contentBlocks, mcpToolCalls, etc.)
        // 5. Return LanguageModelStreamResponse with stream and metadata
        Err("Not yet implemented".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
