use llm_kit_provider::embedding_model::EmbeddingModelUsage;
use llm_kit_provider::embedding_model::embedding::EmbeddingModelEmbedding;
use llm_kit_provider::shared::headers::SharedHeaders;
use llm_kit_provider::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The result of an `embed` call.
/// It contains the embedding, the value, and additional information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbedResult<V> {
    /// The value that was embedded.
    pub value: V,

    /// The embedding of the value.
    pub embedding: EmbeddingModelEmbedding,

    /// The embedding token usage.
    pub usage: EmbeddingModelUsage,

    /// Optional provider-specific metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,

    /// Optional response data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<EmbedResultResponseData>,
}

/// Response data for an embed call.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbedResultResponseData {
    /// Response headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<SharedHeaders>,

    /// The response body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Value>,
}

impl<V> EmbedResult<V> {
    /// Create a new embed result.
    ///
    /// # Arguments
    ///
    /// * `value` - The value that was embedded
    /// * `embedding` - The embedding vector
    /// * `usage` - Token usage information
    ///
    /// # Returns
    ///
    /// A new `EmbedResult` instance
    pub fn new(value: V, embedding: EmbeddingModelEmbedding, usage: EmbeddingModelUsage) -> Self {
        Self {
            value,
            embedding,
            usage,
            provider_metadata: None,
            response: None,
        }
    }

    /// Add provider metadata to the result.
    pub fn with_provider_metadata(mut self, metadata: SharedProviderMetadata) -> Self {
        self.provider_metadata = Some(metadata);
        self
    }

    /// Add response data to the result.
    pub fn with_response(mut self, response: EmbedResultResponseData) -> Self {
        self.response = Some(response);
        self
    }
}

impl EmbedResultResponseData {
    /// Create new response data.
    pub fn new() -> Self {
        Self {
            headers: None,
            body: None,
        }
    }

    /// Add headers to the response data.
    pub fn with_headers(mut self, headers: SharedHeaders) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Add body to the response data.
    pub fn with_body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }
}

impl Default for EmbedResultResponseData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_embed_result_new() {
        let value = "hello world".to_string();
        let embedding = vec![0.1, 0.2, 0.3];
        let usage = EmbeddingModelUsage::new(5);

        let result = EmbedResult::new(value.clone(), embedding.clone(), usage);

        assert_eq!(result.value, value);
        assert_eq!(result.embedding, embedding);
        assert_eq!(result.usage.tokens, 5);
        assert!(result.provider_metadata.is_none());
        assert!(result.response.is_none());
    }

    #[test]
    fn test_embed_result_with_provider_metadata() {
        let value = "test".to_string();
        let embedding = vec![0.1, 0.2];
        let usage = EmbeddingModelUsage::new(3);

        let mut metadata = HashMap::new();
        let mut provider_data = HashMap::new();
        provider_data.insert(
            "model".to_string(),
            serde_json::json!("text-embedding-3-small"),
        );
        metadata.insert("openai".to_string(), provider_data);

        let result =
            EmbedResult::new(value, embedding, usage).with_provider_metadata(metadata.clone());

        assert!(result.provider_metadata.is_some());
        assert_eq!(result.provider_metadata.unwrap(), metadata);
    }

    #[test]
    fn test_embed_result_with_response() {
        let value = "test".to_string();
        let embedding = vec![0.1, 0.2];
        let usage = EmbeddingModelUsage::new(3);

        let response_data = EmbedResultResponseData::new().with_headers(HashMap::from([(
            "x-request-id".to_string(),
            "123".to_string(),
        )]));

        let result = EmbedResult::new(value, embedding, usage).with_response(response_data);

        assert!(result.response.is_some());
        let response = result.response.unwrap();
        assert!(response.headers.is_some());
    }

    #[test]
    fn test_embed_result_response_data_new() {
        let data = EmbedResultResponseData::new();

        assert!(data.headers.is_none());
        assert!(data.body.is_none());
    }

    #[test]
    fn test_embed_result_response_data_default() {
        let data = EmbedResultResponseData::default();

        assert!(data.headers.is_none());
        assert!(data.body.is_none());
    }

    #[test]
    fn test_embed_result_response_data_with_headers() {
        let headers = HashMap::from([
            ("content-type".to_string(), "application/json".to_string()),
            ("x-request-id".to_string(), "abc123".to_string()),
        ]);

        let data = EmbedResultResponseData::new().with_headers(headers.clone());

        assert!(data.headers.is_some());
        assert_eq!(data.headers.unwrap(), headers);
    }

    #[test]
    fn test_embed_result_response_data_with_body() {
        let body = serde_json::json!({
            "model": "text-embedding-3-small",
            "usage": {"tokens": 5}
        });

        let data = EmbedResultResponseData::new().with_body(body.clone());

        assert!(data.body.is_some());
        assert_eq!(data.body.unwrap(), body);
    }

    #[test]
    fn test_embed_result_response_data_builder_pattern() {
        let headers = HashMap::from([("x-test".to_string(), "value".to_string())]);
        let body = serde_json::json!({"test": "data"});

        let data = EmbedResultResponseData::new()
            .with_headers(headers.clone())
            .with_body(body.clone());

        assert!(data.headers.is_some());
        assert_eq!(data.headers.unwrap(), headers);
        assert!(data.body.is_some());
        assert_eq!(data.body.unwrap(), body);
    }

    #[test]
    fn test_embed_result_builder_pattern() {
        let value = "hello".to_string();
        let embedding = vec![0.1, 0.2, 0.3];
        let usage = EmbeddingModelUsage::new(5);

        let mut metadata = HashMap::new();
        let mut provider_data = HashMap::new();
        provider_data.insert("key".to_string(), serde_json::json!("value"));
        metadata.insert("provider".to_string(), provider_data);

        let response_data = EmbedResultResponseData::new()
            .with_headers(HashMap::from([("x-id".to_string(), "123".to_string())]));

        let result = EmbedResult::new(value.clone(), embedding.clone(), usage)
            .with_provider_metadata(metadata.clone())
            .with_response(response_data);

        assert_eq!(result.value, value);
        assert_eq!(result.embedding, embedding);
        assert_eq!(result.usage.tokens, 5);
        assert!(result.provider_metadata.is_some());
        assert!(result.response.is_some());
    }

    #[test]
    fn test_embed_result_serialization() {
        let value = "hello".to_string();
        let embedding = vec![0.1, 0.2, 0.3];
        let usage = EmbeddingModelUsage::new(5);

        let result = EmbedResult::new(value, embedding, usage);
        let json = serde_json::to_string(&result).unwrap();

        // Check that camelCase is used
        assert!(json.contains("\"value\""));
        assert!(json.contains("\"embedding\""));
        assert!(json.contains("\"usage\""));
        // Optional fields should be omitted
        assert!(!json.contains("\"providerMetadata\""));
        assert!(!json.contains("\"response\""));
    }

    #[test]
    fn test_embed_result_deserialization() {
        let json = r#"{
            "value": "test",
            "embedding": [0.1, 0.2],
            "usage": {"tokens": 3}
        }"#;

        let result: EmbedResult<String> = serde_json::from_str(json).unwrap();

        assert_eq!(result.value, "test");
        assert_eq!(result.embedding, vec![0.1, 0.2]);
        assert_eq!(result.usage.tokens, 3);
        assert!(result.provider_metadata.is_none());
        assert!(result.response.is_none());
    }

    #[test]
    fn test_embed_result_with_all_fields_serialization() {
        let value = "test".to_string();
        let embedding = vec![0.1];
        let usage = EmbeddingModelUsage::new(2);

        let mut metadata = HashMap::new();
        let mut provider_data = HashMap::new();
        provider_data.insert("model".to_string(), serde_json::json!("test-model"));
        metadata.insert("openai".to_string(), provider_data);

        let response_data =
            EmbedResultResponseData::new().with_body(serde_json::json!({"status": "ok"}));

        let result = EmbedResult::new(value, embedding, usage)
            .with_provider_metadata(metadata)
            .with_response(response_data);

        let json = serde_json::to_value(&result).unwrap();

        assert!(json.get("value").is_some());
        assert!(json.get("embedding").is_some());
        assert!(json.get("usage").is_some());
        assert!(json.get("providerMetadata").is_some());
        assert!(json.get("response").is_some());
    }

    #[test]
    fn test_embed_result_generic_type() {
        // Test with different value types
        let int_result = EmbedResult::new(42, vec![0.1, 0.2], EmbeddingModelUsage::new(1));
        assert_eq!(int_result.value, 42);

        let string_result = EmbedResult::new(
            "test".to_string(),
            vec![0.3, 0.4],
            EmbeddingModelUsage::new(2),
        );
        assert_eq!(string_result.value, "test");

        let vec_result =
            EmbedResult::new(vec![1, 2, 3], vec![0.5, 0.6], EmbeddingModelUsage::new(3));
        assert_eq!(vec_result.value, vec![1, 2, 3]);
    }
}
