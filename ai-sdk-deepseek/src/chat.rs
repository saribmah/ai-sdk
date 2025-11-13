/// DeepSeek chat language model implementation.
pub mod language_model;

/// Model IDs and provider options.
pub mod options;

/// DeepSeek-specific metadata extraction.
pub mod metadata_extractor;

// Re-export main types
pub use language_model::DeepSeekChatLanguageModel;
pub use metadata_extractor::{DeepSeekMetadataExtractor, DeepSeekUsage};
pub use options::{DeepSeekChatModelId, DeepSeekProviderOptions};
