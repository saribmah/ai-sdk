//! # LLM Kit OpenAI
//!
//! OpenAI provider implementation for the LLM Kit.
//!
//! This crate provides integration with OpenAI's chat completion API,
//! following the LLM Kit provider pattern for Rust.
//!
//! ## Features
//!
//! - Chat completions with streaming support
//! - Tool calling (function calling)
//! - Multi-modal inputs (text, images, audio, PDFs)
//! - Reasoning models support (o1, o3, etc.)
//! - Provider-specific options (logprobs, reasoning effort, service tiers, etc.)
//! - Type-safe configuration
//!
//! ## Quick Start
//!
//! ### Using the Client Builder (Recommended)
//!
//! ```no_run
//! use llm_kit_openai::OpenAIClient;
//! use llm_kit_provider::language_model::LanguageModel;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create provider using the client builder
//!     let provider = OpenAIClient::new()
//!         .api_key("your-api-key")  // Or use OPENAI_API_KEY env var
//!         .build();
//!
//!     // Get a language model
//!     let model = provider.chat("gpt-4o");
//!
//!     println!("Model: {}", model.model_id());
//!     println!("Provider: {}", model.provider());
//!     Ok(())
//! }
//! ```
//!
//! ### Using Settings Directly (Alternative)
//!
//! ```no_run
//! use llm_kit_openai::{OpenAIProvider, OpenAIProviderSettings};
//! use llm_kit_provider::language_model::LanguageModel;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create provider (uses OPENAI_API_KEY from environment)
//!     let provider = OpenAIProvider::new(OpenAIProviderSettings::default());
//!
//!     // Get a language model
//!     let model = provider.chat("gpt-4o");
//!
//!     println!("Model: {}", model.model_id());
//!     println!("Provider: {}", model.provider());
//!     Ok(())
//! }
//! ```
//!
//! ## Configuration
//!
//! ### Using the Client Builder
//!
//! ```rust
//! use llm_kit_openai::OpenAIClient;
//!
//! let provider = OpenAIClient::new()
//!     .api_key("your-api-key")
//!     .base_url("https://api.openai.com/v1")
//!     .organization("org-123")
//!     .project("proj-456")
//!     .header("Custom-Header", "value")
//!     .name("my-openai-provider")
//!     .build();
//! ```
//!
//! ### Using Settings Directly
//!
//! ```rust
//! use llm_kit_openai::{OpenAIProvider, OpenAIProviderSettings};
//!
//! let settings = OpenAIProviderSettings::new()
//!     .with_api_key("your-api-key")
//!     .with_base_url("https://api.openai.com/v1")
//!     .with_organization("org-123")
//!     .with_project("proj-456")
//!     .add_header("Custom-Header", "value")
//!     .with_name("my-openai-provider");
//!
//! let provider = OpenAIProvider::new(settings);
//! ```
//!
//! ## Architecture
//!
//! The implementation follows the LLM Kit provider pattern:
//!
//! - **Provider** (`OpenAIProvider`): Creates model instances
//! - **Language Model** (`OpenAIChatLanguageModel`): Implements text generation and streaming
//! - **Message Conversion**: Converts SDK messages to OpenAI format
//! - **Tool Preparation**: Converts SDK tools to OpenAI function format
//! - **API Types**: Request and response types for the OpenAI API
//!
//! ## Provider-Specific Options
//!
//! OpenAI-specific options can be passed through `provider_options`:
//!
//! ```no_run
//! use llm_kit_openai::chat::OpenAIChatLanguageModelOptions;
//! use serde_json::json;
//!
//! let options = OpenAIChatLanguageModelOptions {
//!     reasoning_effort: Some(llm_kit_openai::chat::openai_chat_options::ReasoningEffort::High),
//!     logprobs: Some(llm_kit_openai::chat::openai_chat_options::LogprobsOption::Number(5)),
//!     ..Default::default()
//! };
//! ```
//!
//! ## Supported Models
//!
//! All OpenAI chat models are supported, including:
//!
//! - GPT-4 family: `gpt-4`, `gpt-4-turbo`, `gpt-4o`, etc.
//! - GPT-3.5: `gpt-3.5-turbo`
//! - Reasoning models: `o1`, `o3-mini`, etc.
//! - GPT-5 family (when available)
//!
//! ## Reasoning Models
//!
//! Reasoning models (o1, o3, etc.) have special handling:
//!
//! - System messages use "developer" role instead of "system"
//! - Unsupported settings (temperature, top_p, etc.) are automatically removed
//! - Uses `max_completion_tokens` instead of `max_tokens`
//!

#![warn(missing_docs)]

/// Chat completion API implementation
pub mod chat;
/// Client builder for creating OpenAI providers
pub mod client;
/// OpenAI provider implementation
pub mod provider;
/// Settings and configuration for OpenAI providers
pub mod settings;

// Re-export main types for convenience
pub use chat::{OpenAIChatLanguageModel, OpenAIChatLanguageModelOptions, OpenAIChatModelId};
pub use client::OpenAIClient;
pub use provider::OpenAIProvider;
pub use settings::OpenAIProviderSettings;
