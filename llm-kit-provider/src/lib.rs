//! AI SDK Provider - Provider interface and traits
//!
//! This crate defines the provider interface and traits that all AI providers must implement.
//! It provides a standardized abstraction layer for language models, embedding models,
//! image generation, speech synthesis, transcription, and reranking across different AI providers.
//!
//! # Overview
//!
//! The provider layer serves as the contract between the high-level SDK APIs (in `llm-kit-core`)
//! and concrete provider implementations (like `llm-kit-openai-compatible`). By implementing
//! these traits, new providers can be seamlessly integrated into the SDK.
//!
//! # Core Traits
//!
//! - [`Provider`]: Top-level trait for creating model instances
//! - [`LanguageModel`]: Text generation and streaming
//! - [`EmbeddingModel`]: Text embeddings
//! - [`ImageModel`]: Image generation
//! - [`SpeechModel`]: Speech synthesis
//! - [`TranscriptionModel`]: Audio transcription
//! - [`RerankingModel`]: Document reranking
//!
//! # Architecture
//!
//! Provider implementations handle:
//! - Authentication and API credentials
//! - HTTP client management
//! - Request/response format conversion
//! - Error handling and retry logic
//! - Provider-specific metadata and options
//!
//! # Implementing a Provider
//!
//! To create a new provider:
//!
//! 1. Implement the [`Provider`] trait to create model instances
//! 2. Implement relevant model traits (e.g., [`LanguageModel`])
//! 3. Convert between provider format and SDK types
//! 4. Handle provider-specific configuration
//!
//! ## Example
//!
//! ```no_run
//! use llm_kit_provider::{Provider, LanguageModel, ProviderError, EmbeddingModel, ImageModel, TranscriptionModel, SpeechModel, RerankingModel};
//! use llm_kit_provider::language_model::call_options::LanguageModelCallOptions;
//! use llm_kit_provider::language_model::{LanguageModelGenerateResponse, LanguageModelStreamResponse};
//! use std::sync::Arc;
//! use std::collections::HashMap;
//! use regex::Regex;
//!
//! struct MyProvider {
//!     api_key: String,
//!     base_url: String,
//! }
//!
//! impl Provider for MyProvider {
//!     fn language_model(&self, model_id: &str) -> Result<Arc<dyn LanguageModel>, ProviderError> {
//!         Ok(Arc::new(MyLanguageModel {
//!             model_id: model_id.to_string(),
//!             api_key: self.api_key.clone(),
//!             base_url: self.base_url.clone(),
//!         }))
//!     }
//!     
//!     fn text_embedding_model(&self, model_id: &str) -> Result<Arc<dyn EmbeddingModel<String>>, ProviderError> {
//!         Err(ProviderError::no_such_model(model_id, "my-provider"))
//!     }
//!     
//!     fn image_model(&self, model_id: &str) -> Result<Arc<dyn ImageModel>, ProviderError> {
//!         Err(ProviderError::no_such_model(model_id, "my-provider"))
//!     }
//!     
//!     fn transcription_model(&self, model_id: &str) -> Result<Arc<dyn TranscriptionModel>, ProviderError> {
//!         Err(ProviderError::no_such_model(model_id, "my-provider"))
//!     }
//!     
//!     fn speech_model(&self, model_id: &str) -> Result<Arc<dyn SpeechModel>, ProviderError> {
//!         Err(ProviderError::no_such_model(model_id, "my-provider"))
//!     }
//!     
//!     fn reranking_model(&self, model_id: &str) -> Result<Arc<dyn RerankingModel>, ProviderError> {
//!         Err(ProviderError::no_such_model(model_id, "my-provider"))
//!     }
//! }
//!
//! struct MyLanguageModel {
//!     model_id: String,
//!     api_key: String,
//!     base_url: String,
//! }
//!
//! #[async_trait::async_trait]
//! impl LanguageModel for MyLanguageModel {
//!     fn provider(&self) -> &str { "my-provider" }
//!     fn model_id(&self) -> &str { &self.model_id }
//!     async fn supported_urls(&self) -> HashMap<String, Vec<Regex>> { HashMap::new() }
//!     
//!     async fn do_generate(
//!         &self,
//!         options: LanguageModelCallOptions,
//!     ) -> Result<LanguageModelGenerateResponse, Box<dyn std::error::Error>> {
//!         // Implement generation logic
//!         todo!()
//!     }
//!
//!     async fn do_stream(
//!         &self,
//!         options: LanguageModelCallOptions,
//!     ) -> Result<LanguageModelStreamResponse, Box<dyn std::error::Error>> {
//!         // Implement streaming logic
//!         todo!()
//!     }
//! }
//! ```
//!
//! # Type System
//!
//! The provider layer defines standardized types for:
//!
//! - **Content**: Text, images, files, reasoning (see [`language_model::content`])
//! - **Messages**: User, assistant, system, tool messages (see [`language_model::prompt`])
//! - **Tools**: Tool definitions and choices (see [`language_model::tool`])
//! - **Responses**: Generation results with metadata and usage (see [`LanguageModelGenerateResponse`])
//! - **Streaming**: Stream parts and events (see [`language_model::stream_part`])
//! - **Errors**: Standardized error types (see [`ProviderError`])
//!
//! # Error Handling
//!
//! All provider operations return [`ProviderError`], which includes:
//!
//! - API call errors (network, authentication)
//! - Invalid prompts or arguments
//! - Response parsing errors
//! - Rate limiting and retries
//! - Provider-specific errors
//!
//! # Metadata and Options
//!
//! Providers can include:
//!
//! - **Call Options**: Per-request configuration (temperature, max tokens, etc.)
//! - **Response Metadata**: Request IDs, timestamps, model information
//! - **Usage Information**: Token counts, costs, latency
//! - **Warnings**: Non-fatal issues during processing
//! - **Provider Options**: Custom provider-specific settings
//!
//! # See Also
//!
//! - `llm-kit-openai-compatible`: Reference implementation for OpenAI-compatible APIs
//! - `llm-kit-core`: High-level builder APIs that use these traits

#![warn(missing_docs)]

/// Embedding model types and traits for generating vector embeddings.
pub mod embedding_model;
/// Error types for provider operations.
pub mod error;
/// Image generation model types and traits.
pub mod image_model;
/// Language model types and traits for text generation and streaming.
pub mod language_model;
/// Provider trait for creating model instances.
pub mod provider;
/// Reranking model types and traits for document reranking.
pub mod reranking_model;
/// Shared types used across multiple model types.
pub mod shared;
/// Speech synthesis model types and traits.
pub mod speech_model;
/// Audio transcription model types and traits.
pub mod transcription_model;

// Re-export commonly used types
pub use embedding_model::{
    EmbeddingModel, EmbeddingModelResponse, EmbeddingModelResponseMetadata, EmbeddingModelUsage,
};
pub use error::ProviderError;
pub use image_model::{ImageData, ImageModel, ImageModelResponse, ImageModelResponseMetadata};
pub use language_model::{
    LanguageModel, LanguageModelGenerateResponse, LanguageModelRequestMetadata,
    LanguageModelStreamResponse,
};
pub use provider::Provider;
pub use reranking_model::{
    RankedDocument, RerankingModel, RerankingModelResponse, RerankingModelResponseMetadata,
};
pub use shared::warning::SharedWarning;
pub use speech_model::{AudioData, SpeechModel, SpeechModelResponse, SpeechModelResponseMetadata};
pub use transcription_model::{
    TranscriptSegment, TranscriptionModel, TranscriptionModelResponse,
    TranscriptionModelResponseMetadata,
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
