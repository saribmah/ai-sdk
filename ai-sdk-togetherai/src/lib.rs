//! Together AI Provider for AI SDK
//!
//! This crate provides a Together AI provider implementation for the AI SDK,
//! enabling access to Together AI's extensive collection of open-source models.
//!
//! # Features
//!
//! - **Chat Models**: Access to Llama, Mistral, Qwen, DeepSeek, and more
//! - **Completion Models**: Traditional text completion interface
//! - **Embedding Models**: Text embedding models for semantic search
//! - **Image Generation**: FLUX and Stable Diffusion models
//! - **Reranking**: Document reranking for improved search results
//!
//! # Quick Start
//!
//! ```ignore
//! use ai_sdk_togetherai::{create_togetherai, TogetherAIProviderSettings};
//! use ai_sdk_core::{GenerateText, prompt::Prompt};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create provider with API key
//!     let provider = create_togetherai(
//!         TogetherAIProviderSettings::new()
//!             .with_api_key("your-api-key")
//!     );
//!
//!     // Create a chat model
//!     let model = provider.chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");
//!
//!     // Generate text
//!     let result = GenerateText::new(model, Prompt::text("What is Rust?"))
//!         .execute()
//!         .await?;
//!
//!     println!("{}", result.text);
//!     Ok(())
//! }
//! ```
//!
//! # API Key Setup
//!
//! You can provide your Together AI API key in two ways:
//!
//! 1. **Directly in code**:
//! ```
//! use ai_sdk_togetherai::{create_togetherai, TogetherAIProviderSettings};
//!
//! let provider = create_togetherai(
//!     TogetherAIProviderSettings::new()
//!         .with_api_key("your-api-key")
//! );
//! ```
//!
//! 2. **Environment variable**: Set `TOGETHER_AI_API_KEY`
//! ```
//! use ai_sdk_togetherai::togetherai;
//!
//! let provider = togetherai(); // Reads from TOGETHER_AI_API_KEY
//! ```
//!
//! # Model Types
//!
//! ## Chat Models
//!
//! ```
//! use ai_sdk_togetherai::togetherai;
//!
//! let provider = togetherai();
//! let model = provider.chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");
//! ```
//!
//! ## Embedding Models
//!
//! ```ignore
//! use ai_sdk_togetherai::togetherai;
//! use ai_sdk_core::Embed;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = togetherai();
//! let model = provider.text_embedding_model("WhereIsAI/UAE-Large-V1");
//!
//! let result = Embed::new(model, "Hello world".to_string())
//!     .execute()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Image Generation
//!
//! ```ignore
//! use ai_sdk_togetherai::togetherai;
//! use ai_sdk_core::GenerateImage;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = togetherai();
//! let model = provider.image_model("black-forest-labs/FLUX.1-schnell");
//!
//! let result = GenerateImage::new(model, "A serene landscape".to_string())
//!     .size("1024x1024")
//!     .execute()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Reranking
//!
//! ```ignore
//! use ai_sdk_togetherai::togetherai;
//! use ai_sdk_core::Rerank;
//! use ai_sdk_provider::reranking_model::call_options::RerankingDocuments;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = togetherai();
//! let model = provider.reranking_model("Salesforce/Llama-Rank-v1");
//!
//! let documents = RerankingDocuments::from_strings(vec![
//!     "Document 1".to_string(),
//!     "Document 2".to_string(),
//! ]);
//!
//! let result = Rerank::new(model, documents, "search query".to_string())
//!     .top_n(5)
//!     .execute()
//!     .await?;
//! # Ok(())
//! # }
//! ```

/// Chat model options and model IDs.
pub mod chat_options;
/// Completion model options and model IDs.
pub mod completion_options;
/// Embedding model options and model IDs.
pub mod embedding_options;
/// Image generation models.
pub mod image;
/// Provider implementation.
pub mod provider;
/// Reranking models.
pub mod reranking;
/// Provider settings and configuration.
pub mod settings;

// Re-exports for convenience
pub use provider::{TogetherAIProvider, create_togetherai, togetherai};
pub use settings::TogetherAIProviderSettings;

pub use chat_options::TogetherAIChatModelId;
pub use completion_options::TogetherAICompletionModelId;
pub use embedding_options::TogetherAIEmbeddingModelId;
pub use image::settings::TogetherAIImageModelId;
pub use reranking::options::{TogetherAIRerankingModelId, TogetherAIRerankingOptions};
