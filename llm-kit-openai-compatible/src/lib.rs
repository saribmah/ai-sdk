//! OpenAI-compatible provider implementation for the LLM Kit.
//!
//! This crate provides a provider implementation for OpenAI-compatible APIs,
//! including OpenAI, Azure OpenAI, and other compatible services.
//!
//! # Examples
//!
//! ## Basic Usage with Client Builder (Recommended)
//!
//! ```no_run
//! use llm_kit_openai_compatible::OpenAICompatibleClient;
//!
//! // Create a provider using the client builder
//! let provider = OpenAICompatibleClient::new()
//!     .base_url("https://api.openai.com/v1")
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.chat_model("gpt-4");
//! ```
//!
//! ## Alternative: Using Settings Directly
//!
//! ```no_run
//! use llm_kit_openai_compatible::{OpenAICompatibleProvider, OpenAICompatibleProviderSettings};
//!
//! // Create a provider using settings
//! let provider = OpenAICompatibleProvider::new(
//!     OpenAICompatibleProviderSettings::new(
//!         "https://api.openai.com/v1",
//!         "openai"
//!     )
//!     .with_api_key("your-api-key")
//! );
//!
//! let model = provider.chat_model("gpt-4");
//! ```
//!
//! ## Chained Usage
//!
//! ```no_run
//! use llm_kit_openai_compatible::OpenAICompatibleClient;
//!
//! let model = OpenAICompatibleClient::new()
//!     .base_url("https://api.example.com/v1")
//!     .name("example")
//!     .api_key("your-api-key")
//!     .build()
//!     .chat_model("meta-llama/Llama-3-70b-chat-hf");
//! ```
//!
//! ## Custom Headers and Query Parameters
//!
//! ```no_run
//! use llm_kit_openai_compatible::OpenAICompatibleClient;
//!
//! let provider = OpenAICompatibleClient::new()
//!     .base_url("https://api.example.com/v1")
//!     .name("custom")
//!     .api_key("your-api-key")
//!     .header("X-Custom-Header", "value")
//!     .query_param("version", "2024-01")
//!     .build();
//!
//! let model = provider.chat_model("gpt-4");
//! ```
//!
//! ## Azure OpenAI Example
//!
//! ```no_run
//! use llm_kit_openai_compatible::OpenAICompatibleClient;
//!
//! let provider = OpenAICompatibleClient::new()
//!     .base_url("https://my-resource.openai.azure.com/openai")
//!     .name("azure-openai")
//!     .api_key("your-api-key")
//!     .query_param("api-version", "2024-02-15-preview")
//!     .build();
//!
//! let model = provider.chat_model("gpt-4");
//! ```
//!
//! ## Text Embeddings
//!
//! ```no_run
//! use llm_kit_openai_compatible::OpenAICompatibleClient;
//! use llm_kit_provider::EmbeddingModel;
//! use llm_kit_provider::embedding_model::call_options::EmbeddingModelCallOptions;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = OpenAICompatibleClient::new()
//!     .base_url("https://api.openai.com/v1")
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.text_embedding_model("text-embedding-3-small");
//!
//! let options = EmbeddingModelCallOptions {
//!     values: vec!["The capital of France is Paris.".to_string()],
//!     headers: None,
//!     provider_options: None,
//!     abort_signal: None,
//! };
//!
//! let result = model.do_embed(options).await?;
//! println!("Embeddings: {:?}", result.embeddings);
//! # Ok(())
//! # }
//! ```
//!
//! ## Image Generation
//!
//! ```no_run
//! use llm_kit_openai_compatible::OpenAICompatibleClient;
//! use llm_kit_provider::ImageModel;
//! use llm_kit_provider::image_model::call_options::ImageModelCallOptions;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = OpenAICompatibleClient::new()
//!     .base_url("https://api.openai.com/v1")
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.image_model("dall-e-3");
//!
//! let options = ImageModelCallOptions {
//!     prompt: "A beautiful sunset over the ocean".to_string(),
//!     n: 1,
//!     size: None,
//!     aspect_ratio: None,
//!     seed: None,
//!     headers: None,
//!     provider_options: None,
//!     abort_signal: None,
//! };
//!
//! let result = model.do_generate(options).await?;
//!
//! println!("Generated {} image(s)", result.images.len());
//! # Ok(())
//! # }
//! ```

/// Chat completion implementation for OpenAI-compatible APIs.
pub mod chat;
/// Client builder for creating OpenAI-compatible providers.
pub mod client;
/// Text completion implementation for OpenAI-compatible APIs.
pub mod completion;
/// Embedding model implementation for OpenAI-compatible APIs.
pub mod embedding;
/// Error types for OpenAI-compatible provider operations.
pub mod error;
/// Image generation implementation for OpenAI-compatible APIs.
pub mod image;
/// Provider implementation and creation functions.
pub mod provider;
/// Settings and configuration for OpenAI-compatible providers.
pub mod settings;
mod utils;

// Re-export main types from chat
pub use chat::{
    MetadataExtractor, OpenAICompatibleChatConfig, OpenAICompatibleChatLanguageModel,
    OpenAICompatibleChatModelId, OpenAICompatibleProviderOptions as ChatProviderOptions,
    convert_to_openai_compatible_chat_messages, prepare_tools,
};

// Re-export main types from completion
pub use completion::{
    OpenAICompatibleCompletionConfig, OpenAICompatibleCompletionLanguageModel,
    OpenAICompatibleCompletionModelId, OpenAICompatibleCompletionPrompt,
    OpenAICompatibleCompletionProviderOptions as CompletionProviderOptions,
    convert_to_openai_compatible_completion_prompt,
};

// Re-export main types from embedding
pub use embedding::{
    OpenAICompatibleEmbeddingConfig, OpenAICompatibleEmbeddingModel,
    OpenAICompatibleEmbeddingModelId,
    OpenAICompatibleEmbeddingProviderOptions as EmbeddingProviderOptions,
};

// Re-export main types from image
pub use image::{
    OpenAICompatibleImageModel, OpenAICompatibleImageModelConfig, OpenAICompatibleImageModelId,
};

pub use client::OpenAICompatibleClient;
pub use error::*;
pub use provider::OpenAICompatibleProvider;
pub use settings::OpenAICompatibleProviderSettings;
