//! Together AI provider implementation for the AI SDK.
//!
//! This crate provides a provider implementation for Together AI's extensive collection of
//! open-source models, supporting chat, embeddings, image generation, and reranking.
//!
//! # Features
//!
//! - Chat completions with Llama, Mistral, Qwen, DeepSeek, and more
//! - Traditional text completion interface
//! - Text embeddings for semantic search
//! - Image generation with FLUX and Stable Diffusion
//! - Document reranking for improved search results
//!
//! # Examples
//!
//! ## Basic Usage with Client Builder (Recommended)
//!
//! ```no_run
//! use llm_kit_togetherai::TogetherAIClient;
//!
//! // Create a provider using the client builder
//! let provider = TogetherAIClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");
//! ```
//!
//! ## Alternative: Direct Instantiation
//!
//! ```no_run
//! use llm_kit_togetherai::{TogetherAIProvider, TogetherAIProviderSettings};
//!
//! // Create a provider using settings
//! let provider = TogetherAIProvider::new(
//!     TogetherAIProviderSettings::new()
//!         .with_api_key("your-api-key")
//! );
//!
//! let model = provider.chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");
//! ```
//!
//! ## Chained Usage
//!
//! ```no_run
//! use llm_kit_togetherai::TogetherAIClient;
//!
//! let model = TogetherAIClient::new()
//!     .api_key("your-api-key")
//!     .build()
//!     .chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");
//! ```
//!
//! ## Environment Variable
//!
//! ```no_run
//! use llm_kit_togetherai::TogetherAIClient;
//!
//! // API key will be read from TOGETHER_AI_API_KEY environment variable
//! let provider = TogetherAIClient::new()
//!     .load_api_key_from_env()
//!     .build();
//!
//! let model = provider.chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");
//! ```
//!
//! ## Using the Provider
//!
//! This provider implements the `llm-kit-provider` traits. See the examples directory
//! for detailed usage with `LanguageModel`, `EmbeddingModel`, `ImageModel`, and
//! `RerankingModel` traits.

/// Chat model options and model IDs.
pub mod chat_options;

/// Client builder for creating Together AI providers.
pub mod client;

/// Completion model options and model IDs.
pub mod completion_options;

/// Embedding model options and model IDs.
pub mod embedding_options;

/// Image generation models.
pub mod image;

/// Provider implementation and creation functions.
pub mod provider;

/// Reranking models.
pub mod reranking;

/// Settings and configuration for Together AI providers.
pub mod settings;

// Re-export main types
pub use client::TogetherAIClient;
pub use provider::TogetherAIProvider;
pub use settings::TogetherAIProviderSettings;

// Re-export model IDs
pub use chat_options::TogetherAIChatModelId;
pub use completion_options::TogetherAICompletionModelId;
pub use embedding_options::TogetherAIEmbeddingModelId;
pub use image::settings::TogetherAIImageModelId;
pub use reranking::options::{TogetherAIRerankingModelId, TogetherAIRerankingOptions};
