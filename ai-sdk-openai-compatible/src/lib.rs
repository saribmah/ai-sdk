//! OpenAI-compatible provider implementation for the AI SDK.
//!
//! This crate provides a provider implementation for OpenAI-compatible APIs,
//! including OpenAI, Azure OpenAI, and other compatible services.
//!
//! # Examples
//!
//! ## Basic Usage with Client Builder (Recommended)
//!
//! ```ignore
//! use ai_sdk_openai_compatible::OpenAICompatibleClient;
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
//! ## Alternative: Using Settings (Legacy)
//!
//! ```ignore
//! use ai_sdk_openai_compatible::{create_openai_compatible, OpenAICompatibleProviderSettings};
//!
//! // Create a provider using settings
//! let provider = create_openai_compatible(
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
//! ## Chained Usage (like Vercel AI SDK)
//!
//! ```ignore
//! use ai_sdk_openai_compatible::OpenAICompatibleClient;
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
//! ```ignore
//! use ai_sdk_openai_compatible::OpenAICompatibleClient;
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
//! ```ignore
//! use ai_sdk_openai_compatible::OpenAICompatibleClient;
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

pub mod chat;
pub mod client;
pub mod completion;
pub mod embedding;
pub mod error;
pub mod image;
pub mod provider;
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
pub use provider::{OpenAICompatibleProvider, create_openai_compatible};
pub use settings::OpenAICompatibleProviderSettings;
