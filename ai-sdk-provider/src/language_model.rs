use crate::language_model::call_options::LanguageModelCallOptions;
use crate::language_model::call_warning::LanguageModelCallWarning;
use crate::language_model::content::LanguageModelContent;
use crate::language_model::finish_reason::LanguageModelFinishReason;
use crate::language_model::response_metadata::LanguageModelResponseMetadata;
use crate::language_model::stream_part::LanguageModelStreamPart;
use crate::language_model::usage::LanguageModelUsage;
use crate::shared::headers::SharedHeaders;
use crate::shared::provider_metadata::SharedProviderMetadata;
use async_trait::async_trait;
use futures::Stream;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

/// Call options for language model requests.
pub mod call_options;
/// Warning types for language model responses.
pub mod call_warning;
/// Content types for language model inputs and outputs.
pub mod content;
mod data_content;
/// Finish reason types for language model responses.
pub mod finish_reason;
/// Prompt types for language model requests.
pub mod prompt;
/// Response metadata types.
pub mod response_metadata;
/// Stream part types for streaming responses.
pub mod stream_part;
/// Tool calling types and utilities.
pub mod tool;
/// Tool choice strategy types.
pub mod tool_choice;
/// Token usage tracking types.
pub mod usage;

/// Language model trait for text generation and streaming.
///
/// This trait defines the interface that all language model implementations must provide.
/// It supports both synchronous generation (`do_generate`) and streaming (`do_stream`)
/// with comprehensive options for prompt, tools, temperature, token limits, and more.
///
/// # Implementation Requirements
///
/// Implementors must:
/// - Handle text generation with optional tool calling
/// - Support streaming responses with proper error handling
/// - Convert between provider-specific formats and SDK types
/// - Manage authentication and API communication
/// - Track token usage and metadata
///
/// # Examples
///
/// ```no_run
/// use ai_sdk_provider::LanguageModel;
/// use ai_sdk_provider::language_model::call_options::LanguageModelCallOptions;
/// use ai_sdk_provider::language_model::{LanguageModelGenerateResponse, LanguageModelStreamResponse};
/// use async_trait::async_trait;
/// use std::collections::HashMap;
/// use regex::Regex;
///
/// struct MyLanguageModel {
///     model_id: String,
///     api_key: String,
/// }
///
/// #[async_trait]
/// impl LanguageModel for MyLanguageModel {
///     fn provider(&self) -> &str {
///         "my-provider"
///     }
///
///     fn model_id(&self) -> &str {
///         &self.model_id
///     }
///
///     async fn supported_urls(&self) -> HashMap<String, Vec<Regex>> {
///         HashMap::new()
///     }
///
///     async fn do_generate(
///         &self,
///         options: LanguageModelCallOptions,
///     ) -> Result<LanguageModelGenerateResponse, Box<dyn std::error::Error>> {
///         // Implementation
///         todo!()
///     }
///
///     async fn do_stream(
///         &self,
///         options: LanguageModelCallOptions,
///     ) -> Result<LanguageModelStreamResponse, Box<dyn std::error::Error>> {
///         // Implementation
///         todo!()
///     }
/// }
/// ```
#[async_trait]
pub trait LanguageModel: Send + Sync {
    /// Returns the specification version this model implements.
    ///
    /// Defaults to "v2" for the current SDK version.
    fn specification_version(&self) -> &str {
        "v2"
    }

    /// Name of the provider for logging purposes.
    ///
    /// Examples: "openai", "anthropic", "azure-openai"
    fn provider(&self) -> &str;

    /// Provider-specific model ID for logging purposes.
    ///
    /// Examples: "gpt-4", "claude-3-opus", "llama-3-70b"
    fn model_id(&self) -> &str;

    /// Returns supported URL patterns for this model.
    ///
    /// Used for validating and routing requests to appropriate models.
    /// The key is the URL type (e.g., "api", "stream") and the value is
    /// a list of regex patterns that match valid URLs for that type.
    async fn supported_urls(&self) -> HashMap<String, Vec<Regex>>;

    /// Generates text synchronously.
    ///
    /// This method performs a complete text generation request and returns
    /// the full response once generation is complete.
    ///
    /// # Arguments
    ///
    /// * `options` - Call options including prompt, tools, temperature, max tokens, etc.
    ///
    /// # Returns
    ///
    /// A [`LanguageModelGenerateResponse`] containing:
    /// - Generated content (text, tool calls, reasoning, etc.)
    /// - Finish reason (stop, length, tool calls, etc.)
    /// - Token usage statistics
    /// - Provider metadata and warnings
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The API request fails (network, authentication, etc.)
    /// - The response cannot be parsed
    /// - The model returns an error
    /// - Rate limits are exceeded
    async fn do_generate(
        &self,
        options: LanguageModelCallOptions,
    ) -> Result<LanguageModelGenerateResponse, Box<dyn std::error::Error>>;

    /// Generates text as a stream.
    ///
    /// This method initiates streaming text generation and returns a stream
    /// of parts that can include text deltas, tool calls, and metadata.
    ///
    /// # Arguments
    ///
    /// * `options` - Call options including prompt, tools, temperature, max tokens, etc.
    ///
    /// # Returns
    ///
    /// A [`LanguageModelStreamResponse`] containing:
    /// - A stream of [`LanguageModelStreamPart`] items
    /// - Request metadata (body, headers)
    /// - Response metadata (headers)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The stream cannot be initiated
    /// - Authentication fails
    /// - The model doesn't support streaming
    ///
    /// # Note
    ///
    /// Individual stream items may also contain errors which should be
    /// handled by the stream consumer.
    async fn do_stream(
        &self,
        options: LanguageModelCallOptions,
    ) -> Result<LanguageModelStreamResponse, Box<dyn std::error::Error>>;
}

/// Response from a synchronous language model generation.
///
/// Contains the complete generated content along with metadata about the generation.
#[derive(Debug)]
pub struct LanguageModelGenerateResponse {
    /// Generated content parts (text, tool calls, reasoning, etc.)
    pub content: Vec<LanguageModelContent>,

    /// Reason why generation finished (stop, length, tool_calls, etc.)
    pub finish_reason: LanguageModelFinishReason,

    /// Token usage statistics
    pub usage: LanguageModelUsage,

    /// Provider-specific metadata
    pub provider_metadata: Option<SharedProviderMetadata>,

    /// Request metadata (body sent to provider)
    pub request: Option<LanguageModelRequestMetadata>,

    /// Response metadata (headers, timing, etc.)
    pub response: Option<LanguageModelResponseMetadata>,

    /// Non-fatal warnings from the generation
    pub warnings: Vec<LanguageModelCallWarning>,
}

/// Response from a streaming language model generation.
///
/// Contains a stream of parts that are emitted as the model generates content.
pub struct LanguageModelStreamResponse {
    /// Stream of content parts (text deltas, tool calls, etc.)
    pub stream: Box<dyn Stream<Item = LanguageModelStreamPart> + Unpin + Send>,

    /// Request metadata (body sent to provider)
    pub request: Option<LanguageModelRequestMetadata>,

    /// Response metadata (headers)
    pub response: Option<StreamResponseMetadata>,
}

/// Metadata about the request sent to the language model.
#[derive(Debug)]
pub struct LanguageModelRequestMetadata {
    /// The raw request body sent to the provider API
    pub body: Option<Value>,
}

/// Metadata about the streaming response.
pub struct StreamResponseMetadata {
    /// HTTP headers from the streaming response
    pub headers: Option<SharedHeaders>,
}
