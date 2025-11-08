use ai_sdk_provider::embedding_model::EmbeddingModelUsage;
use ai_sdk_provider::embedding_model::embedding::EmbeddingModelEmbedding;
use ai_sdk_provider::shared::headers::SharedHeaders;
use ai_sdk_provider::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The result of an `embed_many` call.
/// It contains the embeddings, the values, and additional information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbedManyResult<V> {
    /// The values that were embedded.
    pub values: Vec<V>,

    /// The embeddings. They are in the same order as the values.
    pub embeddings: Vec<EmbeddingModelEmbedding>,

    /// The embedding token usage.
    pub usage: EmbeddingModelUsage,

    /// Optional provider-specific metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,

    /// Optional raw response data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<Option<EmbedManyResultResponseData>>>,
}

/// Response data for a single embedding call within embed_many.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmbedManyResultResponseData {
    /// Response headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<SharedHeaders>,

    /// The response body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Value>,
}

impl<V> EmbedManyResult<V> {
    /// Create a new embed many result.
    ///
    /// # Arguments
    ///
    /// * `values` - The values that were embedded
    /// * `embeddings` - The embeddings (in same order as values)
    /// * `usage` - Token usage information
    ///
    /// # Returns
    ///
    /// A new `EmbedManyResult` instance
    pub fn new(
        values: Vec<V>,
        embeddings: Vec<EmbeddingModelEmbedding>,
        usage: EmbeddingModelUsage,
    ) -> Self {
        Self {
            values,
            embeddings,
            usage,
            provider_metadata: None,
            responses: None,
        }
    }

    /// Add provider metadata to the result.
    pub fn with_provider_metadata(mut self, metadata: SharedProviderMetadata) -> Self {
        self.provider_metadata = Some(metadata);
        self
    }

    /// Add response data to the result.
    pub fn with_responses(mut self, responses: Vec<Option<EmbedManyResultResponseData>>) -> Self {
        self.responses = Some(responses);
        self
    }

    /// Add a single response to the result.
    pub fn with_response(mut self, response: EmbedManyResultResponseData) -> Self {
        if let Some(ref mut responses) = self.responses {
            responses.push(Some(response));
        } else {
            self.responses = Some(vec![Some(response)]);
        }
        self
    }
}

impl EmbedManyResultResponseData {
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

impl Default for EmbedManyResultResponseData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_embed_many_result_new() {
        let values = vec!["hello".to_string(), "world".to_string()];
        let embeddings = vec![vec![0.1, 0.2, 0.3], vec![0.4, 0.5, 0.6]];
        let usage = EmbeddingModelUsage::new(10);

        let result = EmbedManyResult::new(values.clone(), embeddings.clone(), usage);

        assert_eq!(result.values, values);
        assert_eq!(result.embeddings, embeddings);
        assert_eq!(result.usage.tokens, 10);
        assert!(result.provider_metadata.is_none());
        assert!(result.responses.is_none());
    }

    #[test]
    fn test_embed_many_result_with_provider_metadata() {
        let values = vec!["test".to_string()];
        let embeddings = vec![vec![0.1, 0.2]];
        let usage = EmbeddingModelUsage::new(5);

        let mut metadata = HashMap::new();
        let mut provider_data = HashMap::new();
        provider_data.insert("key".to_string(), serde_json::json!("value"));
        metadata.insert("openai".to_string(), provider_data);

        let result = EmbedManyResult::new(values, embeddings, usage)
            .with_provider_metadata(metadata.clone());

        assert!(result.provider_metadata.is_some());
        assert_eq!(result.provider_metadata.unwrap(), metadata);
    }

    #[test]
    fn test_embed_many_result_with_responses() {
        let values = vec!["test".to_string()];
        let embeddings = vec![vec![0.1, 0.2]];
        let usage = EmbeddingModelUsage::new(5);

        let response_data = EmbedManyResultResponseData::new().with_headers(HashMap::from([(
            "x-request-id".to_string(),
            "123".to_string(),
        )]));

        let result = EmbedManyResult::new(values, embeddings, usage)
            .with_responses(vec![Some(response_data)]);

        assert!(result.responses.is_some());
        assert_eq!(result.responses.unwrap().len(), 1);
    }

    #[test]
    fn test_embed_many_result_with_response() {
        let values = vec!["test".to_string()];
        let embeddings = vec![vec![0.1, 0.2]];
        let usage = EmbeddingModelUsage::new(5);

        let response_data =
            EmbedManyResultResponseData::new().with_body(serde_json::json!({"status": "success"}));

        let result = EmbedManyResult::new(values, embeddings, usage).with_response(response_data);

        assert!(result.responses.is_some());
        let responses = result.responses.unwrap();
        assert_eq!(responses.len(), 1);
        assert!(responses[0].is_some());
    }

    #[test]
    fn test_embed_many_response_data_new() {
        let data = EmbedManyResultResponseData::new();

        assert!(data.headers.is_none());
        assert!(data.body.is_none());
    }

    #[test]
    fn test_embed_many_response_data_default() {
        let data = EmbedManyResultResponseData::default();

        assert!(data.headers.is_none());
        assert!(data.body.is_none());
    }

    #[test]
    fn test_embed_many_response_data_with_headers() {
        let headers = HashMap::from([
            ("content-type".to_string(), "application/json".to_string()),
            ("x-request-id".to_string(), "abc123".to_string()),
        ]);

        let data = EmbedManyResultResponseData::new().with_headers(headers.clone());

        assert!(data.headers.is_some());
        assert_eq!(data.headers.unwrap(), headers);
    }

    #[test]
    fn test_embed_many_response_data_with_body() {
        let body = serde_json::json!({
            "model": "text-embedding-3-small",
            "usage": {"tokens": 10}
        });

        let data = EmbedManyResultResponseData::new().with_body(body.clone());

        assert!(data.body.is_some());
        assert_eq!(data.body.unwrap(), body);
    }

    #[test]
    fn test_embed_many_response_data_builder_pattern() {
        let headers = HashMap::from([("x-test".to_string(), "value".to_string())]);
        let body = serde_json::json!({"test": "data"});

        let data = EmbedManyResultResponseData::new()
            .with_headers(headers.clone())
            .with_body(body.clone());

        assert!(data.headers.is_some());
        assert_eq!(data.headers.unwrap(), headers);
        assert!(data.body.is_some());
        assert_eq!(data.body.unwrap(), body);
    }

    #[test]
    fn test_embed_many_result_serialization() {
        let values = vec!["hello".to_string()];
        let embeddings = vec![vec![0.1, 0.2, 0.3]];
        let usage = EmbeddingModelUsage::new(5);

        let result = EmbedManyResult::new(values, embeddings, usage);
        let json = serde_json::to_string(&result).unwrap();

        // Check that camelCase is used
        assert!(json.contains("\"values\""));
        assert!(json.contains("\"embeddings\""));
        assert!(json.contains("\"usage\""));
        // Optional fields should be omitted
        assert!(!json.contains("\"providerMetadata\""));
        assert!(!json.contains("\"responses\""));
    }

    #[test]
    fn test_embed_many_result_deserialization() {
        let json = r#"{
            "values": ["test"],
            "embeddings": [[0.1, 0.2]],
            "usage": {"tokens": 3}
        }"#;

        let result: EmbedManyResult<String> = serde_json::from_str(json).unwrap();

        assert_eq!(result.values, vec!["test"]);
        assert_eq!(result.embeddings, vec![vec![0.1, 0.2]]);
        assert_eq!(result.usage.tokens, 3);
    }

    #[test]
    fn test_embed_many_result_with_multiple_responses() {
        let values = vec!["a".to_string(), "b".to_string()];
        let embeddings = vec![vec![0.1], vec![0.2]];
        let usage = EmbeddingModelUsage::new(10);

        let response1 = EmbedManyResultResponseData::new().with_body(serde_json::json!({"id": 1}));
        let response2 = EmbedManyResultResponseData::new().with_body(serde_json::json!({"id": 2}));

        let result = EmbedManyResult::new(values, embeddings, usage)
            .with_response(response1)
            .with_response(response2);

        assert!(result.responses.is_some());
        let responses = result.responses.unwrap();
        assert_eq!(responses.len(), 2);
        assert!(responses[0].is_some());
        assert!(responses[1].is_some());
    }
}
