/// Baseten embedding model identifier.
///
/// For embedding models, you typically pass the model URL directly rather than using a model ID.
/// The model ID can be any string and is primarily used for identification purposes.
///
/// # Note
///
/// Embeddings require a custom model URL and are not available via Model APIs.
/// You must provide a `modelURL` when creating an embedding model.
///
/// # Example URLs
///
/// - `https://model-{id}.api.baseten.co/environments/production/sync`
/// - `https://model-{id}.api.baseten.co/environments/production/sync/v1`
pub type BasetenEmbeddingModelId = String;

/// Creates an embedding model ID from a string.
///
/// # Examples
///
/// ```
/// use ai_sdk_baseten::embedding::embedding_model_id;
///
/// let model_id = embedding_model_id("embeddings");
/// ```
pub fn embedding_model_id(id: impl Into<String>) -> BasetenEmbeddingModelId {
    id.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_model_id() {
        let id = embedding_model_id("embeddings");
        assert_eq!(id, "embeddings");

        let id = embedding_model_id("custom-embedding-model");
        assert_eq!(id, "custom-embedding-model");
    }
}
