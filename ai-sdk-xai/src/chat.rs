//! xAI chat completion implementation.
//!
//! This module provides the chat completion functionality for xAI's Grok models,
//! including support for:
//! - Text and reasoning content
//! - Tool/function calling
//! - Streaming responses
//! - Provider-specific options (reasoning effort, search parameters)
//! - Citations extraction

/// xAI chat model options and configuration.
pub mod options;

/// Prompt conversion for xAI chat models.
pub mod prompt;

/// Tool preparation for xAI chat models.
pub mod prepare_tools;

/// xAI chat language model implementation.
pub mod language_model;

pub use language_model::XaiChatLanguageModel;
pub use options::{SearchParameters, SearchSource, XaiChatModelId, XaiProviderOptions};
pub use prompt::{XaiChatMessage, XaiUserContent, convert_to_xai_chat_messages};
