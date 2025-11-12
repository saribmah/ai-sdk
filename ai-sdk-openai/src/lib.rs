//! # AI SDK OpenAI
//!
//! OpenAI provider implementation for the AI SDK.
//!
//! This crate provides integration with OpenAI's chat completion API,
//! following the AI SDK provider pattern for Rust.
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
//! ## Example
//!
//! ```no_run
//! use ai_sdk_openai::{openai, OpenAIProviderSettings};
//! use ai_sdk_provider::language_model::LanguageModel;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create an OpenAI provider (reads OPENAI_API_KEY from environment)
//!     let provider = openai();
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
//! ## Architecture
//!
//! The implementation follows the AI SDK provider pattern:
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
//! use ai_sdk_openai::chat::OpenAIChatLanguageModelOptions;
//! use serde_json::json;
//!
//! let options = OpenAIChatLanguageModelOptions {
//!     reasoning_effort: Some(ai_sdk_openai::chat::openai_chat_options::ReasoningEffort::High),
//!     logprobs: Some(ai_sdk_openai::chat::openai_chat_options::LogprobsOption::Number(5)),
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
/// OpenAI provider implementation
pub mod openai_provider;

// Re-export main types for convenience
pub use chat::{OpenAIChatLanguageModel, OpenAIChatLanguageModelOptions, OpenAIChatModelId};
pub use openai_provider::{OpenAIProvider, OpenAIProviderSettings, create_openai, openai};
