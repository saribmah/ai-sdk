/// Prompt conversion utilities for text completions.
pub mod convert_prompt;
/// Language model implementation for text completions.
pub mod language_model;
/// Provider options for text completions.
pub mod options;
/// Prompt types for text completions.
pub mod prompt;

pub use convert_prompt::convert_to_openai_compatible_completion_prompt;
pub use language_model::{
    OpenAICompatibleCompletionConfig, OpenAICompatibleCompletionLanguageModel,
};
pub use options::{OpenAICompatibleCompletionModelId, OpenAICompatibleCompletionProviderOptions};
pub use prompt::OpenAICompatibleCompletionPrompt;
