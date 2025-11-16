//! DeepSeek provider implementation for the AI SDK.
//!
//! This crate provides a provider implementation for DeepSeek's chat and reasoning models,
//! supporting standard chat completions and advanced reasoning capabilities.
//!
//! # Features
//!
//! - Chat completions with `deepseek-chat`
//! - Advanced reasoning with `deepseek-reasoner` (R1)
//! - Streaming support
//! - Tool calling
//! - DeepSeek-specific metadata (prompt cache hit/miss tokens)
//!
//! # Examples
//!
//! ## Basic Usage with Client Builder (Recommended)
//!
//! ```no_run
//! use ai_sdk_deepseek::DeepSeekClient;
//!
//! // Create a provider using the client builder
//! let provider = DeepSeekClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.chat_model("deepseek-chat");
//! ```
//!
//! ## Alternative: Direct Instantiation
//!
//! ```no_run
//! use ai_sdk_deepseek::{DeepSeekProvider, DeepSeekProviderSettings};
//!
//! // Create a provider using settings
//! let provider = DeepSeekProvider::new(
//!     DeepSeekProviderSettings::new()
//!         .with_api_key("your-api-key")
//! );
//!
//! let model = provider.chat_model("deepseek-chat");
//! ```
//!
//! ## Chained Usage
//!
//! ```no_run
//! use ai_sdk_deepseek::DeepSeekClient;
//!
//! let model = DeepSeekClient::new()
//!     .api_key("your-api-key")
//!     .build()
//!     .chat_model("deepseek-reasoner");
//! ```
//!
//! ## Environment Variable
//!
//! ```no_run
//! use ai_sdk_deepseek::DeepSeekClient;
//!
//! // API key will be read from DEEPSEEK_API_KEY environment variable
//! let provider = DeepSeekClient::new()
//!     .load_api_key_from_env()
//!     .build();
//!
//! let model = provider.chat_model("deepseek-chat");
//! ```
//!
//! ## Text Generation
//!
//! ```no_run
//! use ai_sdk_deepseek::DeepSeekClient;
//! use ai_sdk_core::{GenerateText, prompt::Prompt};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = DeepSeekClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.chat_model("deepseek-chat");
//!
//! let result = GenerateText::new(
//!     model,
//!     Prompt::text("Write a function to calculate factorial")
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
//! ## Reasoning Model
//!
//! ```no_run
//! use ai_sdk_deepseek::DeepSeekClient;
//! use ai_sdk_core::{GenerateText, prompt::Prompt};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = DeepSeekClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! // Use the DeepSeek Reasoner (R1) model for advanced reasoning
//! let model = provider.chat_model("deepseek-reasoner");
//!
//! let result = GenerateText::new(
//!     model,
//!     Prompt::text("Solve this logic puzzle: ...")
//! )
//! .execute()
//! .await?;
//!
//! // Access reasoning content if available
//! if !result.reasoning.is_empty() {
//!     println!("Reasoning:");
//!     for reasoning in &result.reasoning {
//!         println!("{}", reasoning.text);
//!     }
//! }
//! println!("Answer: {}", result.text);
//! # Ok(())
//! # }
//! ```
//!
//! ## Streaming
//!
//! ```no_run
//! use ai_sdk_deepseek::DeepSeekClient;
//! use ai_sdk_core::{StreamText, prompt::Prompt};
//! use futures_util::StreamExt;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = DeepSeekClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.chat_model("deepseek-chat");
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
//! ## DeepSeek-Specific Metadata
//!
//! DeepSeek provides prompt cache statistics in the metadata:
//!
//! ```no_run
//! use ai_sdk_deepseek::DeepSeekClient;
//! use ai_sdk_core::{GenerateText, prompt::Prompt};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = DeepSeekClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.chat_model("deepseek-chat");
//!
//! let result = GenerateText::new(model, Prompt::text("Hello"))
//!     .execute()
//!     .await?;
//!
//! // Access DeepSeek-specific metadata
//! if let Some(provider_metadata) = &result.provider_metadata {
//!     if let Some(deepseek) = provider_metadata.get("deepseek") {
//!         println!("Prompt cache hit tokens: {:?}",
//!             deepseek.get("promptCacheHitTokens"));
//!         println!("Prompt cache miss tokens: {:?}",
//!             deepseek.get("promptCacheMissTokens"));
//!     }
//! }
//! # Ok(())
//! # }
//! ```

/// Chat completion implementation for DeepSeek models.
pub mod chat;

/// Client builder for creating DeepSeek providers.
pub mod client;

/// Error types for DeepSeek provider operations.
pub mod error;

/// Provider implementation and creation functions.
pub mod provider;

/// Settings and configuration for DeepSeek providers.
pub mod settings;

// Re-export main types from chat
pub use chat::{
    DeepSeekChatLanguageModel, DeepSeekChatModelId, DeepSeekMetadataExtractor,
    DeepSeekProviderOptions, DeepSeekUsage,
};

pub use client::DeepSeekClient;
pub use error::DeepSeekError;
pub use provider::DeepSeekProvider;
pub use settings::DeepSeekProviderSettings;

// Re-export error data type
pub use error::DeepSeekErrorData;
