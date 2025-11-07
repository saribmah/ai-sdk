use crate::embedding_model::call_options::EmbeddingModelCallOptions;
use crate::embedding_model::embedding::EmbeddingModelEmbedding;
use crate::shared::headers::SharedHeaders;
use crate::shared::provider_metadata::SharedProviderMetadata;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod call_options;
pub mod embedding;

/// Specification for an embedding model that implements the embedding model
/// interface version 3.
///
/// VALUE is the type of the values that the model can embed.
/// This will allow us to go beyond text embeddings in the future,
/// e.g. to support image embeddings.
#[async_trait]
pub trait EmbeddingModel<V>: Send + Sync
where
    V: Send + Sync,
{
    /// The embedding model must specify which embedding model interface
    /// version it implements. This will allow us to evolve the embedding
    /// model interface and retain backwards compatibility. The different
    /// implementation versions can be handled as a discriminated union
    /// on our side.
    fn specification_version(&self) -> &str {
        "v3"
    }

    /// Name of the provider for logging purposes.
    fn provider(&self) -> &str;

    /// Provider-specific model ID for logging purposes.
    fn model_id(&self) -> &str;

    /// Limit of how many embeddings can be generated in a single API call.
    ///
    /// Use None for models that do not have a limit.
    async fn max_embeddings_per_call(&self) -> Option<usize>;

    /// True if the model can handle multiple embedding calls in parallel.
    async fn supports_parallel_calls(&self) -> bool;

    /// Generates a list of embeddings for the given input values.
    ///
    /// Naming: "do" prefix to prevent accidental direct usage of the method
    /// by the user.
    async fn do_embed(
        &self,
        options: EmbeddingModelCallOptions<V>,
    ) -> Result<EmbeddingModelResponse, Box<dyn std::error::Error>>;
}

/// Response from an embedding model.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddingModelResponse {
    /// Generated embeddings. They are in the same order as the input values.
    pub embeddings: Vec<EmbeddingModelEmbedding>,

    /// Token usage. We only have input tokens for embeddings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<EmbeddingModelUsage>,

    /// Additional provider-specific metadata. They are passed through
    /// from the provider to the AI SDK and enable provider-specific
    /// results that can be fully encapsulated in the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,

    /// Optional response information for debugging purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<EmbeddingModelResponseMetadata>,
}

/// Token usage information for embeddings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddingModelUsage {
    /// Number of tokens used.
    pub tokens: u32,
}

impl EmbeddingModelUsage {
    /// Create a new usage struct with the given token count.
    pub fn new(tokens: u32) -> Self {
        Self { tokens }
    }
}

/// Response metadata for debugging purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddingModelResponseMetadata {
    /// Response headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<SharedHeaders>,

    /// The response body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Value>,
}

impl EmbeddingModelResponse {
    /// Create a new response with embeddings.
    pub fn new(embeddings: Vec<EmbeddingModelEmbedding>) -> Self {
        Self {
            embeddings,
            usage: None,
            provider_metadata: None,
            response: None,
        }
    }

    /// Add usage information to the response.
    pub fn with_usage(mut self, usage: EmbeddingModelUsage) -> Self {
        self.usage = Some(usage);
        self
    }

    /// Add provider metadata to the response.
    pub fn with_provider_metadata(mut self, metadata: SharedProviderMetadata) -> Self {
        self.provider_metadata = Some(metadata);
        self
    }

    /// Add response metadata to the response.
    pub fn with_response_metadata(mut self, metadata: EmbeddingModelResponseMetadata) -> Self {
        self.response = Some(metadata);
        self
    }
}

impl EmbeddingModelResponseMetadata {
    /// Create new response metadata.
    pub fn new() -> Self {
        Self {
            headers: None,
            body: None,
        }
    }

    /// Add headers to the response metadata.
    pub fn with_headers(mut self, headers: SharedHeaders) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Add body to the response metadata.
    pub fn with_body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }
}

impl Default for EmbeddingModelResponseMetadata {
    fn default() -> Self {
        Self::new()
    }
}
