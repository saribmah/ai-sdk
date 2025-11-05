//! OpenAI-compatible provider implementation for the AI SDK.
//!
//! This crate provides a provider implementation for OpenAI-compatible APIs,
//! including OpenAI, Azure OpenAI, and other compatible services.
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```ignore
//! use openai_compatible::{create_openai_compatible, OpenAICompatibleProviderSettings};
//!
//! // Create a provider and get a chat model
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
//! use openai_compatible::{create_openai_compatible, OpenAICompatibleProviderSettings};
//!
//! let model = create_openai_compatible(
//!     OpenAICompatibleProviderSettings::new(
//!         "https://api.example.com/v1",
//!         "example"
//!     )
//!     .with_api_key("your-api-key")
//! )
//! .chat_model("meta-llama/Llama-3-70b-chat-hf");
//! ```
//!
//! ## Custom Headers and Query Parameters
//!
//! ```ignore
//! use openai_compatible::{create_openai_compatible, OpenAICompatibleProviderSettings};
//!
//! let provider = create_openai_compatible(
//!     OpenAICompatibleProviderSettings::new(
//!         "https://api.example.com/v1",
//!         "custom"
//!     )
//!     .with_api_key("your-api-key")
//!     .with_header("X-Custom-Header", "value")
//!     .with_query_param("version", "2024-01")
//! );
//!
//! let model = provider.language_model("gpt-4");
//! ```

pub mod chat;
pub mod completion;
pub mod error;
mod provider;
pub mod settings;
mod utils;

pub use chat::*;
pub use completion::*;
pub use error::*;
pub use provider::{OpenAICompatibleProvider, create_openai_compatible};
pub use settings::OpenAICompatibleProviderSettings;
