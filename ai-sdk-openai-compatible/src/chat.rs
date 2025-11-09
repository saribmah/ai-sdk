/// Prompt conversion utilities for chat completions.
pub mod convert_prompt;
/// Language model implementation for chat completions.
pub mod language_model;
/// Metadata extraction from chat responses.
pub mod metadata_extractor;
/// Provider options for chat completions.
pub mod options;
/// Tool preparation for chat completions.
pub mod prepare_tools;
/// Prompt types for chat completions.
pub mod prompt;

pub use convert_prompt::convert_to_openai_compatible_chat_messages;
pub use language_model::{OpenAICompatibleChatConfig, OpenAICompatibleChatLanguageModel};
pub use metadata_extractor::MetadataExtractor;
pub use options::{OpenAICompatibleChatModelId, OpenAICompatibleProviderOptions};
pub use prepare_tools::prepare_tools;
pub use prompt::message::*;
