//! # Anthropic Provider for AI SDK Rust
//!
//! This crate provides a comprehensive Rust implementation of the Anthropic API provider
//! for the AI SDK. It enables building AI-powered applications using Claude models with
//! full support for text generation, streaming, tool calling, and advanced features like
//! extended thinking, citations, and prompt caching.
//!
//! ## Features
//!
//! - **Text Generation**: Generate text using Claude models with `GenerateText` builder
//! - **Streaming**: Stream responses in real-time with `StreamText`
//! - **Tool Calling**: Support for both custom tools and Anthropic provider-defined tools
//! - **Extended Thinking**: Enable Claude's reasoning process with thinking blocks
//! - **Citations**: Enable source citations for generated content
//! - **Prompt Caching**: Reduce costs with automatic prompt caching
//! - **Vision**: Support for image inputs in prompts
//! - **Multiple Models**: Support for all Claude models (Opus, Sonnet, Haiku)
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use ai_sdk_anthropic::{create_anthropic, AnthropicProviderSettings};
//! use ai_sdk_core::{GenerateText, Prompt};
//! use ai_sdk_provider::provider::Provider;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create provider (uses ANTHROPIC_API_KEY env var)
//!     let provider = create_anthropic(AnthropicProviderSettings::default());
//!     
//!     // Create a language model
//!     let model = provider.language_model("claude-3-5-sonnet-20241022".to_string());
//!     
//!     // Generate text
//!     let result = GenerateText::new(std::sync::Arc::new(model), Prompt::text("Hello, Claude!"))
//!         .temperature(0.7)
//!         .max_output_tokens(100)
//!         .execute()
//!         .await?;
//!     
//!     println!("{}", result.text);
//!     Ok(())
//! }
//! ```
//!
//! ## Tool Calling
//!
//! The Anthropic provider supports both custom tools and provider-defined tools:
//!
//! ```rust,no_run
//! use ai_sdk_anthropic::{create_anthropic, anthropic_tools};
//! use ai_sdk_core::{GenerateText, Prompt, ToolSet};
//! use ai_sdk_provider::provider::Provider;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = create_anthropic(Default::default());
//! let model = provider.language_model("claude-3-5-sonnet-20241022".to_string());
//!
//! // Use provider-defined tools
//! let tools = ToolSet::from_vec(vec![
//!     anthropic_tools::bash_20250124(None),
//!     anthropic_tools::web_search_20250305()
//!         .max_uses(5)
//!         .build(),
//! ]);
//!
//! let result = GenerateText::new(std::sync::Arc::new(model), Prompt::text("Search for Rust news"))
//!     .tools(tools)
//!     .execute()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Provider-Defined Tools
//!
//! Anthropic provides several powerful provider-defined tools:
//!
//! - **Bash**: Execute shell commands (`bash_20241022`, `bash_20250124`)
//! - **Computer Use**: Control desktop environments (`computer_20241022`, `computer_20250124`)
//! - **Code Execution**: Run Python/Bash code (`code_execution_20250522`, `code_execution_20250825`)
//! - **Text Editor**: Edit files (`text_editor_20241022`, `text_editor_20250124`, `text_editor_20250728`)
//! - **Web Search**: Search the web with citations (`web_search_20250305`)
//! - **Web Fetch**: Fetch web content (`web_fetch_20250910`)
//! - **Memory**: Persistent memory across conversations (`memory_20250818`)
//!
//! ## Advanced Features
//!
//! ### Extended Thinking
//!
//! Enable Claude's extended reasoning process:
//!
//! ```rust,no_run
//! use ai_sdk_anthropic::create_anthropic;
//! use ai_sdk_core::{GenerateText, Prompt};
//! use ai_sdk_provider::provider::Provider;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = create_anthropic(Default::default());
//! let model = provider.language_model("claude-3-7-sonnet-20250219".to_string());
//!
//! let result = GenerateText::new(std::sync::Arc::new(model),
//!     Prompt::text("Solve this complex problem"))
//!     .thinking_enabled(true)
//!     .thinking_budget(10000) // Optional token budget
//!     .execute()
//!     .await?;
//!
//! // Access reasoning
//! for output in result.experimental_output.iter() {
//!     if let ai_sdk_provider::language_model::Output::Reasoning(reasoning) = output {
//!         println!("Reasoning: {}", reasoning.text);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ### Streaming
//!
//! Stream responses for real-time output:
//!
//! ```rust,no_run
//! use ai_sdk_anthropic::create_anthropic;
//! use ai_sdk_core::{StreamText, Prompt};
//! use ai_sdk_provider::provider::Provider;
//! use futures_util::StreamExt;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = create_anthropic(Default::default());
//! let model = provider.language_model("claude-3-5-sonnet-20241022".to_string());
//!
//! let mut stream = StreamText::new(std::sync::Arc::new(model),
//!     Prompt::text("Write a story"))
//!     .temperature(0.8)
//!     .execute()
//!     .await?;
//!
//! while let Some(part) = stream.next().await {
//!     match part {
//!         Ok(ai_sdk_provider::language_model::TextStreamPart::TextDelta { delta, .. }) => {
//!             print!("{}", delta);
//!         }
//!         _ => {}
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Configuration
//!
//! Configure the provider with custom settings:
//!
//! ```rust
//! use ai_sdk_anthropic::AnthropicProviderSettings;
//! use std::collections::HashMap;
//!
//! let settings = AnthropicProviderSettings::new()
//!     .with_api_key("your-api-key")
//!     .with_base_url("https://api.anthropic.com/v1")
//!     .add_header("Custom-Header", "value")
//!     .with_name("my-anthropic-provider");
//! ```
//!
//! ## Environment Variables
//!
//! - `ANTHROPIC_API_KEY`: API key for authentication (required)
//! - `ANTHROPIC_BASE_URL`: Custom base URL (optional, defaults to `https://api.anthropic.com/v1`)
//!
//! ## Error Handling
//!
//! The provider uses the `AnthropicError` type for error handling:
//!
//! ```rust,no_run
//! use ai_sdk_anthropic::{create_anthropic, AnthropicError};
//! use ai_sdk_core::{GenerateText, Prompt};
//! use ai_sdk_provider::provider::Provider;
//!
//! # async fn example() {
//! let provider = create_anthropic(Default::default());
//! let model = provider.language_model("claude-3-5-sonnet-20241022".to_string());
//!
//! match GenerateText::new(std::sync::Arc::new(model), Prompt::text("Hello"))
//!     .execute()
//!     .await
//! {
//!     Ok(result) => println!("{}", result.text),
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! # }
//! ```

/// Anthropic provider-defined tools namespace.
pub mod anthropic_tools;
/// Internal module for converting prompts to Anthropic message format.
mod convert_to_message_prompt;
/// Error types for Anthropic provider.
pub mod error;
/// Utilities for cache control breakpoint management.
pub mod get_cache_control;
/// Language model implementation for Anthropic.
pub mod language_model;
/// Utilities for mapping stop reasons.
pub mod map_stop_reason;
/// Options and settings for Anthropic models.
pub mod options;
/// Tool preparation utilities.
pub mod prepare_tools;
/// Prompt and message types.
pub mod prompt;
/// Provider implementation.
pub mod provider;
/// Provider metadata utilities.
pub mod provider_metadata_utils;
/// Provider-defined tool factory functions.
pub mod provider_tool;

// Re-export main types for convenience
pub use error::{AnthropicError, AnthropicErrorData, AnthropicErrorDetails, parse_anthropic_error};
pub use language_model::{
    response_schema::{AnthropicMessagesResponse, ContentBlock, Usage},
    stream_schema::{AnthropicChunk, ContentBlockDelta, ContentBlockStart},
};
pub use provider::{AnthropicProvider, AnthropicProviderSettings, anthropic, create_anthropic};
