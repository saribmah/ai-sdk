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
//! use ai_sdk_togetherai::TogetherAIClient;
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
//! use ai_sdk_togetherai::{TogetherAIProvider, TogetherAIProviderSettings};
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
//! use ai_sdk_togetherai::TogetherAIClient;
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
//! use ai_sdk_togetherai::TogetherAIClient;
//!
//! // API key will be read from TOGETHER_AI_API_KEY environment variable
//! let provider = TogetherAIClient::new()
//!     .load_api_key_from_env()
//!     .build();
//!
//! let model = provider.chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");
//! ```
//!
//! ## Text Generation
//!
//! ```no_run
//! use ai_sdk_togetherai::TogetherAIClient;
//! use ai_sdk_core::{GenerateText, prompt::Prompt};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = TogetherAIClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");
//!
//! let result = GenerateText::new(
//!     model,
//!     Prompt::text("What is Rust?")
//! )
//! .temperature(0.7)
//! .execute()
//! .await?;
//!
//! println!("Response: {}", result.text);
//! # Ok(())
//! # }
//! ```
//!
//! ## Streaming
//!
//! ```no_run
//! use ai_sdk_togetherai::TogetherAIClient;
//! use ai_sdk_core::{StreamText, prompt::Prompt};
//! use futures_util::StreamExt;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = TogetherAIClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");
//!
//! let result = StreamText::new(
//!     model,
//!     Prompt::text("Tell me a story")
//! )
//! .execute()
//! .await?;
//!
//! // Stream text deltas
//! let mut text_stream = result.text_stream();
//! while let Some(delta) = text_stream.next().await {
//!     print!("{}", delta);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Embedding Models
//!
//! ```no_run
//! use ai_sdk_togetherai::TogetherAIClient;
//! use ai_sdk_core::Embed;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = TogetherAIClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.text_embedding_model("WhereIsAI/UAE-Large-V1");
//!
//! let result = Embed::new(model, "Hello world".to_string())
//!     .execute()
//!     .await?;
//!
//! println!("Embedding dimension: {}", result.embedding.len());
//! # Ok(())
//! # }
//! ```
//!
//! ## Image Generation
//!
//! ```no_run
//! use ai_sdk_togetherai::TogetherAIClient;
//! use ai_sdk_core::GenerateImage;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = TogetherAIClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.image_model("black-forest-labs/FLUX.1-schnell");
//!
//! let result = GenerateImage::new(model, "A serene landscape".to_string())
//!     .size("1024x1024")
//!     .execute()
//!     .await?;
//!
//! // Images are returned as base64 encoded strings
//! for image in result.images {
//!     // Save or process image
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Document Reranking
//!
//! ```no_run
//! use ai_sdk_togetherai::TogetherAIClient;
//! use ai_sdk_core::Rerank;
//! use ai_sdk_provider::reranking_model::call_options::RerankingDocuments;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = TogetherAIClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
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
//!
//! for ranked_doc in result.ranking {
//!     println!("Index: {}, Score: {}", ranked_doc.index, ranked_doc.relevance_score);
//! }
//! # Ok(())
//! # }
//! ```

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
