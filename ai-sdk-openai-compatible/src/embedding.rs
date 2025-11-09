/// Embedding model implementation for OpenAI-compatible APIs.
pub mod embedding_model;
/// Provider options for embedding models.
pub mod options;

pub use embedding_model::{OpenAICompatibleEmbeddingConfig, OpenAICompatibleEmbeddingModel};
pub use options::{OpenAICompatibleEmbeddingModelId, OpenAICompatibleEmbeddingProviderOptions};
