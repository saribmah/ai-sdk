use crate::embedding_model::call_options::EmbeddingModelCallOptions;
use crate::embedding_model::embedding::EmbeddingModelEmbedding;
use crate::shared::headers::SharedHeaders;
use crate::shared::provider_metadata::SharedProviderMetadata;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Call options for embedding model requests.
pub mod call_options;
/// Embedding response types.
pub mod embedding;

/// Embedding model trait for generating vector embeddings.
///
/// This trait defines the interface for embedding models that convert input values
/// (typically text) into high-dimensional vector representations. These embeddings
/// can be used for semantic search, similarity comparison, clustering, and more.
///
/// # Type Parameter
///
/// * `V` - The type of values that can be embedded. Currently this is typically `String`
///   for text embeddings, but the generic design allows for future support of other
///   types like images or multimodal inputs.
///
/// # Specification Version
///
/// This implements version 3 of the embedding model interface, which supports:
/// - Text-to-embedding conversion
/// - Batch processing of multiple inputs
/// - Parallel API calls for large batches
/// - Token usage tracking
///
/// # Use Cases
///
/// - **Semantic Search**: Convert documents and queries to embeddings for similarity search
/// - **RAG Systems**: Embed documents for retrieval-augmented generation
/// - **Clustering**: Group similar items based on embedding distance
/// - **Recommendation**: Find similar content based on vector similarity
///
/// # Examples
///
/// ```no_run
/// use ai_sdk_provider::EmbeddingModel;
/// use ai_sdk_provider::embedding_model::call_options::EmbeddingModelCallOptions;
/// use ai_sdk_provider::embedding_model::EmbeddingModelResponse;
/// use async_trait::async_trait;
///
/// struct MyEmbeddingModel {
///     model_id: String,
///     api_key: String,
/// }
///
/// #[async_trait]
/// impl EmbeddingModel<String> for MyEmbeddingModel {
///     fn provider(&self) -> &str {
///         "my-provider"
///     }
///
///     fn model_id(&self) -> &str {
///         &self.model_id
///     }
///
///     async fn max_embeddings_per_call(&self) -> Option<usize> {
///         Some(100) // Limit to 100 embeddings per call
///     }
///
///     async fn supports_parallel_calls(&self) -> bool {
///         true // Support parallel batching
///     }
///
///     async fn do_embed(
///         &self,
///         options: EmbeddingModelCallOptions<String>,
///     ) -> Result<EmbeddingModelResponse, Box<dyn std::error::Error>> {
///         // Implementation
///         todo!()
///     }
/// }
/// ```
#[async_trait]
pub trait EmbeddingModel<V>: Send + Sync
where
    V: Send + Sync,
{
    /// Returns the specification version this model implements.
    ///
    /// Defaults to "v3" for the current SDK version. This allows us to evolve
    /// the embedding model interface and retain backwards compatibility.
    fn specification_version(&self) -> &str {
        "v3"
    }

    /// Name of the provider for logging purposes.
    ///
    /// Examples: "openai", "cohere", "voyage"
    fn provider(&self) -> &str;

    /// Provider-specific model ID for logging purposes.
    ///
    /// Examples: "text-embedding-3-small", "embed-english-v3.0"
    fn model_id(&self) -> &str;

    /// Returns the maximum number of embeddings that can be generated in a single API call.
    ///
    /// This limit varies by provider and model. The SDK automatically splits large
    /// batches into multiple calls if needed.
    ///
    /// # Returns
    ///
    /// - `None` if there's no limit
    /// - `Some(n)` if the model is limited to n embeddings per call
    async fn max_embeddings_per_call(&self) -> Option<usize>;

    /// Returns whether the model supports parallel API calls.
    ///
    /// When `true`, the SDK can make multiple embedding API calls in parallel
    /// to process large batches more efficiently. When `false`, calls are made
    /// sequentially.
    ///
    /// # Returns
    ///
    /// - `true` if parallel calls are supported and safe
    /// - `false` if calls must be made sequentially
    async fn supports_parallel_calls(&self) -> bool;

    /// Generates embeddings for the given input values.
    ///
    /// This method processes one or more input values and returns their vector
    /// embeddings in the same order as the inputs.
    ///
    /// # Arguments
    ///
    /// * `options` - Call options including values to embed and provider-specific settings
    ///
    /// # Returns
    ///
    /// An [`EmbeddingModelResponse`] containing:
    /// - Vector embeddings in the same order as inputs
    /// - Token usage statistics
    /// - Provider-specific metadata
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The API request fails
    /// - The input is invalid or too long
    /// - The number of inputs exceeds the limit
    /// - Embedding generation fails
    ///
    /// # Note
    ///
    /// The "do_" prefix prevents accidental direct usage of this method.
    /// Use the high-level `Embed` or `EmbedMany` builders from `ai-sdk-core` instead.
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
