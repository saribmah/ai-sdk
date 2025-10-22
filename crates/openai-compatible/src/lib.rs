//! OpenAI-compatible provider implementation for the AI SDK.
//!
//! This crate provides a provider implementation for OpenAI-compatible APIs,
//! including OpenAI, Azure OpenAI, and other compatible services.
//!
//! # Examples
//!
//! ```ignore
//! use openai_compatible::{create_openai_compatible, OpenAICompatibleConfig};
//! use ai_sdk_provider::provider::Provider;
//!
//! // Create an OpenAI provider
//! let config = OpenAICompatibleConfig::default()
//!     .with_api_key("your-api-key");
//! let provider = create_openai_compatible(config);
//!
//! // Get a language model
//! let model = provider.language_model("gpt-4").unwrap();
//! ```

mod provider;
mod language_model;

pub use provider::{create_openai_compatible, OpenAICompatibleConfig, OpenAICompatibleProvider};
pub use language_model::{
    create_language_model, OpenAICompatibleLanguageModel, OpenAICompatibleLanguageModelConfig,
};
