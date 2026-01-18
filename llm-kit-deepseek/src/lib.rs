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
//! use llm_kit_deepseek::DeepSeekClient;
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
//! use llm_kit_deepseek::{DeepSeekProvider, DeepSeekProviderSettings};
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
//! use llm_kit_deepseek::DeepSeekClient;
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
//! use llm_kit_deepseek::DeepSeekClient;
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
//! use llm_kit_deepseek::DeepSeekClient;
//! use llm_kit_provider::language_model::call_options::LanguageModelCallOptions;
//! use llm_kit_provider::language_model::prompt::LanguageModelMessage;
//! use llm_kit_provider::language_model::content::LanguageModelContent;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = DeepSeekClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.chat_model("deepseek-chat");
//!
//! let prompt = vec![LanguageModelMessage::user_text("Write a function to calculate factorial")];
//! let options = LanguageModelCallOptions::new(prompt).with_temperature(0.7);
//! let result = model.do_generate(options).await?;
//!
//! for content in &result.content {
//!     if let LanguageModelContent::Text(text) = content {
//!         println!("Response: {}", text.text);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Reasoning Model
//!
//! ```no_run
//! use llm_kit_deepseek::DeepSeekClient;
//! use llm_kit_provider::language_model::call_options::LanguageModelCallOptions;
//! use llm_kit_provider::language_model::prompt::LanguageModelMessage;
//! use llm_kit_provider::language_model::content::LanguageModelContent;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = DeepSeekClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! // Use the DeepSeek Reasoner (R1) model for advanced reasoning
//! let model = provider.chat_model("deepseek-reasoner");
//!
//! let prompt = vec![LanguageModelMessage::user_text("Solve this logic puzzle: ...")];
//! let options = LanguageModelCallOptions::new(prompt);
//! let result = model.do_generate(options).await?;
//!
//! // Access reasoning and text content
//! for content in &result.content {
//!     match content {
//!         LanguageModelContent::Reasoning(reasoning) => {
//!             println!("Reasoning: {}", reasoning.text);
//!         }
//!         LanguageModelContent::Text(text) => {
//!             println!("Answer: {}", text.text);
//!         }
//!         _ => {}
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Streaming
//!
//! ```no_run
//! use llm_kit_deepseek::DeepSeekClient;
//! use llm_kit_provider::language_model::call_options::LanguageModelCallOptions;
//! use llm_kit_provider::language_model::prompt::LanguageModelMessage;
//! use llm_kit_provider::language_model::stream_part::LanguageModelStreamPart;
//! use futures_util::StreamExt;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = DeepSeekClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.chat_model("deepseek-chat");
//!
//! let prompt = vec![LanguageModelMessage::user_text("Tell me a story")];
//! let options = LanguageModelCallOptions::new(prompt);
//! let mut result = model.do_stream(options).await?;
//!
//! // Stream text deltas
//! while let Some(part) = result.stream.next().await {
//!     if let LanguageModelStreamPart::TextDelta(delta) = part {
//!         print!("{}", delta.delta);
//!     }
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
//! use llm_kit_deepseek::DeepSeekClient;
//! use llm_kit_provider::language_model::call_options::LanguageModelCallOptions;
//! use llm_kit_provider::language_model::prompt::LanguageModelMessage;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = DeepSeekClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.chat_model("deepseek-chat");
//!
//! let prompt = vec![LanguageModelMessage::user_text("Hello")];
//! let options = LanguageModelCallOptions::new(prompt);
//! let result = model.do_generate(options).await?;
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
