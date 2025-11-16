//! Baseten provider implementation for the AI SDK.
//!
//! This crate provides a provider implementation for Baseten, supporting both
//! Model APIs (hosted models) and custom model deployments.
//!
//! # Examples
//!
//! ## Recommended: Builder Pattern
//!
//! ```no_run
//! use ai_sdk_baseten::BasetenClient;
//!
//! // Create a provider using Model APIs
//! let provider = BasetenClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.chat_model(Some("deepseek-ai/DeepSeek-V3-0324"));
//! ```
//!
//! ## Alternative: Direct Instantiation
//!
//! ```no_run
//! use ai_sdk_baseten::{BasetenProvider, BasetenProviderSettings};
//!
//! let provider = BasetenProvider::new(
//!     BasetenProviderSettings::new()
//!         .with_api_key("your-api-key")
//! );
//!
//! let model = provider.chat_model(Some("deepseek-ai/DeepSeek-V3-0324"));
//! ```
//!
//! ## Custom Model URL
//!
//! ```no_run
//! use ai_sdk_baseten::BasetenClient;
//!
//! // Create a provider with custom model URL
//! let provider = BasetenClient::new()
//!     .api_key("your-api-key")
//!     .model_url("https://model-abc123.api.baseten.co/environments/production/sync/v1")
//!     .build();
//!
//! let model = provider.chat_model(None);
//! ```
//!
//! ## Text Embeddings
//!
//! ```no_run
//! use ai_sdk_baseten::BasetenClient;
//! use ai_sdk_core::EmbedMany;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = BasetenClient::new()
//!     .api_key("your-api-key")
//!     .model_url("https://model-xyz789.api.baseten.co/environments/production/sync")
//!     .build();
//!
//! let model = provider.text_embedding_model(None);
//!
//! let texts = vec![
//!     "The capital of France is Paris.".to_string(),
//!     "The capital of Germany is Berlin.".to_string(),
//! ];
//! let result = EmbedMany::new(model, texts).execute().await?;
//! println!("Embeddings: {:?}", result.embeddings);
//! # Ok(())
//! # }
//! ```

/// Chat model types and configuration.
pub mod chat;
/// Client builder for creating Baseten providers.
pub mod client;
/// Embedding model types and configuration.
pub mod embedding;
/// Error types for Baseten provider operations.
pub mod error;
/// Provider implementation and creation functions.
pub mod provider;
/// Settings and configuration for Baseten providers.
pub mod settings;

pub use chat::BasetenChatModelId;
pub use client::BasetenClient;
pub use embedding::BasetenEmbeddingModelId;
pub use error::BasetenError;
pub use provider::BasetenProvider;
pub use settings::BasetenProviderSettings;
