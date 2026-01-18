use llm_kit_provider::reranking_model::RerankingModelResponseMetadata;
use llm_kit_provider::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};

/// A ranked document with its relevance score.
///
/// This is the core SDK's representation which includes the document value
/// along with its ranking information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RankedDocumentWithValue<V> {
    /// The index of the document in the original list of documents before reranking.
    pub original_index: usize,

    /// The relevance score of the document after reranking.
    pub score: f64,

    /// The document value.
    pub document: V,
}

impl<V> RankedDocumentWithValue<V> {
    /// Create a new ranked document with value.
    pub fn new(original_index: usize, score: f64, document: V) -> Self {
        Self {
            original_index,
            score,
            document,
        }
    }
}

/// The result of a `rerank` call.
/// It contains the original documents, the reranked documents, and additional information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RerankResult<V> {
    /// The original documents that were reranked.
    pub original_documents: Vec<V>,

    /// Reranked documents.
    ///
    /// Sorted by relevance score in descending order.
    ///
    /// Can be less than the original documents if there was a topK limit.
    pub reranked_documents: Vec<V>,

    /// The ranking is a list of objects with the original index,
    /// relevance score, and the reranked document.
    ///
    /// Sorted by relevance score in descending order.
    ///
    /// Can be less than the original documents if there was a topK limit.
    pub ranking: Vec<RankedDocumentWithValue<V>>,

    /// Optional provider-specific metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,

    /// Optional raw response data.
    pub response: RerankResponseMetadata,
}

/// Response metadata for a rerank call.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RerankResponseMetadata {
    /// ID for the generated response if the provider sends one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Timestamp of the generated response.
    #[serde(with = "system_time_as_timestamp")]
    pub timestamp: std::time::SystemTime,

    /// The ID of the model that was used to generate the response.
    pub model_id: String,

    /// Response headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,

    /// The response body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
}

impl<V: Clone> RerankResult<V> {
    /// Create a new rerank result.
    ///
    /// # Arguments
    ///
    /// * `original_documents` - The original documents before reranking
    /// * `ranking` - The ranking information from the provider
    /// * `response` - Response metadata
    ///
    /// # Returns
    ///
    /// A new `RerankResult` instance with reranked documents extracted from the ranking
    pub fn new(
        original_documents: Vec<V>,
        ranking: Vec<RankedDocumentWithValue<V>>,
        response: RerankResponseMetadata,
    ) -> Self {
        // Extract the reranked documents from the ranking (already sorted by score)
        let reranked_documents: Vec<V> = ranking.iter().map(|r| r.document.clone()).collect();

        Self {
            original_documents,
            reranked_documents,
            ranking,
            provider_metadata: None,
            response,
        }
    }

    /// Add provider metadata to the result.
    pub fn with_provider_metadata(mut self, metadata: SharedProviderMetadata) -> Self {
        self.provider_metadata = Some(metadata);
        self
    }

    /// Get the top N reranked documents.
    pub fn top_n(&self, n: usize) -> &[V] {
        let end = n.min(self.reranked_documents.len());
        &self.reranked_documents[..end]
    }

    /// Get the top N ranking entries.
    pub fn top_n_ranking(&self, n: usize) -> &[RankedDocumentWithValue<V>] {
        let end = n.min(self.ranking.len());
        &self.ranking[..end]
    }
}

impl RerankResponseMetadata {
    /// Create new response metadata.
    pub fn new(model_id: impl Into<String>) -> Self {
        Self {
            id: None,
            timestamp: std::time::SystemTime::now(),
            model_id: model_id.into(),
            headers: None,
            body: None,
        }
    }

    /// Create response metadata with a specific timestamp.
    pub fn with_timestamp(model_id: impl Into<String>, timestamp: std::time::SystemTime) -> Self {
        Self {
            id: None,
            timestamp,
            model_id: model_id.into(),
            headers: None,
            body: None,
        }
    }

    /// Add an ID to the response metadata.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Add headers to the response metadata.
    pub fn with_headers(mut self, headers: std::collections::HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Add body to the response metadata.
    pub fn with_body(mut self, body: serde_json::Value) -> Self {
        self.body = Some(body);
        self
    }
}

/// Convert from provider's RerankingModelResponseMetadata to core SDK's RerankResponseMetadata
impl From<RerankingModelResponseMetadata> for RerankResponseMetadata {
    fn from(metadata: RerankingModelResponseMetadata) -> Self {
        Self {
            id: metadata.id,
            timestamp: metadata
                .timestamp
                .unwrap_or_else(std::time::SystemTime::now),
            model_id: metadata.model_id.unwrap_or_else(|| "unknown".to_string()),
            headers: metadata.headers,
            body: metadata.body,
        }
    }
}

// Serde module for SystemTime serialization
mod system_time_as_timestamp {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = time
            .duration_since(UNIX_EPOCH)
            .map_err(serde::ser::Error::custom)?;
        serializer.serialize_u64(duration.as_millis() as u64)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        Ok(UNIX_EPOCH + std::time::Duration::from_millis(millis))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_ranked_document_with_value_new() {
        let doc = RankedDocumentWithValue::new(0, 0.95, "test document");

        assert_eq!(doc.original_index, 0);
        assert_eq!(doc.score, 0.95);
        assert_eq!(doc.document, "test document");
    }

    #[test]
    fn test_rerank_result_new() {
        let original_docs = vec!["doc1", "doc2", "doc3"];
        let ranking = vec![
            RankedDocumentWithValue::new(2, 0.95, "doc3"),
            RankedDocumentWithValue::new(0, 0.85, "doc1"),
            RankedDocumentWithValue::new(1, 0.75, "doc2"),
        ];
        let response = RerankResponseMetadata::new("rerank-model");

        let result = RerankResult::new(original_docs.clone(), ranking.clone(), response);

        assert_eq!(result.original_documents, original_docs);
        assert_eq!(result.reranked_documents, vec!["doc3", "doc1", "doc2"]);
        assert_eq!(result.ranking.len(), 3);
        assert!(result.provider_metadata.is_none());
    }

    #[test]
    fn test_rerank_result_with_provider_metadata() {
        let original_docs = vec!["doc1", "doc2"];
        let ranking = vec![
            RankedDocumentWithValue::new(1, 0.9, "doc2"),
            RankedDocumentWithValue::new(0, 0.8, "doc1"),
        ];
        let response = RerankResponseMetadata::new("rerank-model");

        let mut metadata = HashMap::new();
        let mut provider_data = HashMap::new();
        provider_data.insert("test".to_string(), serde_json::json!("value"));
        metadata.insert("provider".to_string(), provider_data);

        let result = RerankResult::new(original_docs, ranking, response)
            .with_provider_metadata(metadata.clone());

        assert!(result.provider_metadata.is_some());
        assert_eq!(result.provider_metadata.unwrap(), metadata);
    }

    #[test]
    fn test_rerank_result_top_n() {
        let original_docs = vec!["doc1", "doc2", "doc3", "doc4"];
        let ranking = vec![
            RankedDocumentWithValue::new(2, 0.95, "doc3"),
            RankedDocumentWithValue::new(0, 0.85, "doc1"),
            RankedDocumentWithValue::new(3, 0.75, "doc4"),
            RankedDocumentWithValue::new(1, 0.65, "doc2"),
        ];
        let response = RerankResponseMetadata::new("rerank-model");

        let result = RerankResult::new(original_docs, ranking, response);

        let top_2 = result.top_n(2);
        assert_eq!(top_2.len(), 2);
        assert_eq!(top_2[0], "doc3");
        assert_eq!(top_2[1], "doc1");
    }

    #[test]
    fn test_rerank_result_top_n_exceeds_length() {
        let original_docs = vec!["doc1", "doc2"];
        let ranking = vec![
            RankedDocumentWithValue::new(1, 0.9, "doc2"),
            RankedDocumentWithValue::new(0, 0.8, "doc1"),
        ];
        let response = RerankResponseMetadata::new("rerank-model");

        let result = RerankResult::new(original_docs, ranking, response);

        let top_10 = result.top_n(10);
        assert_eq!(top_10.len(), 2);
    }

    #[test]
    fn test_rerank_result_top_n_ranking() {
        let original_docs = vec!["doc1", "doc2", "doc3"];
        let ranking = vec![
            RankedDocumentWithValue::new(2, 0.95, "doc3"),
            RankedDocumentWithValue::new(0, 0.85, "doc1"),
            RankedDocumentWithValue::new(1, 0.75, "doc2"),
        ];
        let response = RerankResponseMetadata::new("rerank-model");

        let result = RerankResult::new(original_docs, ranking, response);

        let top_2 = result.top_n_ranking(2);
        assert_eq!(top_2.len(), 2);
        assert_eq!(top_2[0].document, "doc3");
        assert_eq!(top_2[0].score, 0.95);
        assert_eq!(top_2[1].document, "doc1");
        assert_eq!(top_2[1].score, 0.85);
    }

    #[test]
    fn test_rerank_response_metadata_new() {
        let metadata = RerankResponseMetadata::new("rerank-model");

        assert_eq!(metadata.model_id, "rerank-model");
        assert!(metadata.id.is_none());
        assert!(metadata.headers.is_none());
        assert!(metadata.body.is_none());
    }

    #[test]
    fn test_rerank_response_metadata_with_timestamp() {
        let timestamp = std::time::SystemTime::now();
        let metadata = RerankResponseMetadata::with_timestamp("rerank-model", timestamp);

        assert_eq!(metadata.model_id, "rerank-model");
        assert_eq!(metadata.timestamp, timestamp);
    }

    #[test]
    fn test_rerank_response_metadata_with_id() {
        let metadata = RerankResponseMetadata::new("rerank-model").with_id("req-123");

        assert_eq!(metadata.id, Some("req-123".to_string()));
    }

    #[test]
    fn test_rerank_response_metadata_with_headers() {
        let headers = HashMap::from([
            ("x-request-id".to_string(), "123".to_string()),
            ("content-type".to_string(), "application/json".to_string()),
        ]);

        let metadata = RerankResponseMetadata::new("rerank-model").with_headers(headers.clone());

        assert!(metadata.headers.is_some());
        assert_eq!(metadata.headers.unwrap(), headers);
    }

    #[test]
    fn test_rerank_response_metadata_with_body() {
        let body = serde_json::json!({"test": "data"});

        let metadata = RerankResponseMetadata::new("rerank-model").with_body(body.clone());

        assert!(metadata.body.is_some());
        assert_eq!(metadata.body.unwrap(), body);
    }

    #[test]
    fn test_rerank_response_metadata_builder_pattern() {
        let headers = HashMap::from([("x-id".to_string(), "123".to_string())]);
        let body = serde_json::json!({"status": "ok"});

        let metadata = RerankResponseMetadata::new("rerank-model")
            .with_id("req-456")
            .with_headers(headers.clone())
            .with_body(body.clone());

        assert_eq!(metadata.id, Some("req-456".to_string()));
        assert!(metadata.headers.is_some());
        assert!(metadata.body.is_some());
    }

    #[test]
    fn test_rerank_result_serialization() {
        let original_docs = vec!["doc1", "doc2"];
        let ranking = vec![
            RankedDocumentWithValue::new(1, 0.9, "doc2"),
            RankedDocumentWithValue::new(0, 0.8, "doc1"),
        ];
        let response = RerankResponseMetadata::new("rerank-model");

        let result = RerankResult::new(original_docs, ranking, response);
        let json = serde_json::to_value(&result).unwrap();

        assert!(json.get("originalDocuments").is_some());
        assert!(json.get("rerankedDocuments").is_some());
        assert!(json.get("ranking").is_some());
        assert!(json.get("response").is_some());
        // providerMetadata is None so it should be omitted
        assert!(json.get("providerMetadata").is_none());
    }

    #[test]
    fn test_rerank_result_round_trip() {
        let original_docs = vec!["doc1".to_string(), "doc2".to_string()];
        let ranking = vec![
            RankedDocumentWithValue::new(1, 0.9, "doc2".to_string()),
            RankedDocumentWithValue::new(0, 0.8, "doc1".to_string()),
        ];
        let response = RerankResponseMetadata::new("rerank-model");

        let result = RerankResult::new(original_docs.clone(), ranking, response);

        // Serialize and deserialize
        let json = serde_json::to_value(&result).unwrap();
        let deserialized: RerankResult<String> = serde_json::from_value(json).unwrap();

        assert_eq!(deserialized.original_documents, original_docs);
        assert_eq!(
            deserialized.reranked_documents,
            vec!["doc2".to_string(), "doc1".to_string()]
        );
        assert_eq!(deserialized.ranking.len(), 2);
        assert_eq!(deserialized.response.model_id, "rerank-model");
    }

    #[test]
    fn test_ranked_document_with_value_serialization() {
        let doc = RankedDocumentWithValue::new(0, 0.95, "test document");
        let json = serde_json::to_value(&doc).unwrap();

        assert!(json.get("originalIndex").is_some());
        assert!(json.get("score").is_some());
        assert!(json.get("document").is_some());

        assert_eq!(json.get("originalIndex").unwrap(), 0);
        assert_eq!(json.get("score").unwrap(), 0.95);
        assert_eq!(json.get("document").unwrap(), "test document");
    }

    #[test]
    fn test_rerank_result_with_different_value_types() {
        // Test with integer documents
        let original_docs = vec![1, 2, 3];
        let ranking = vec![
            RankedDocumentWithValue::new(2, 0.95, 3),
            RankedDocumentWithValue::new(0, 0.85, 1),
            RankedDocumentWithValue::new(1, 0.75, 2),
        ];
        let response = RerankResponseMetadata::new("rerank-model");

        let result = RerankResult::new(original_docs, ranking, response);

        assert_eq!(result.reranked_documents, vec![3, 1, 2]);
    }

    #[test]
    fn test_rerank_result_with_struct_documents() {
        #[derive(Debug, Clone, PartialEq)]
        struct Document {
            id: i32,
            content: String,
        }

        let doc1 = Document {
            id: 1,
            content: "First document".to_string(),
        };
        let doc2 = Document {
            id: 2,
            content: "Second document".to_string(),
        };

        let original_docs = vec![doc1.clone(), doc2.clone()];
        let ranking = vec![
            RankedDocumentWithValue::new(1, 0.9, doc2.clone()),
            RankedDocumentWithValue::new(0, 0.8, doc1.clone()),
        ];
        let response = RerankResponseMetadata::new("rerank-model");

        let result = RerankResult::new(original_docs, ranking, response);

        assert_eq!(result.reranked_documents[0].id, 2);
        assert_eq!(result.reranked_documents[1].id, 1);
    }

    #[test]
    fn test_from_reranking_model_response_metadata() {
        let provider_metadata = RerankingModelResponseMetadata::default()
            .with_id("req-123")
            .with_model_id("rerank-model")
            .with_timestamp(std::time::SystemTime::now());

        let core_metadata: RerankResponseMetadata = provider_metadata.into();

        assert_eq!(core_metadata.id, Some("req-123".to_string()));
        assert_eq!(core_metadata.model_id, "rerank-model");
    }

    #[test]
    fn test_from_reranking_model_response_metadata_with_defaults() {
        let provider_metadata = RerankingModelResponseMetadata::default();

        let core_metadata: RerankResponseMetadata = provider_metadata.into();

        // Should use defaults when optional fields are None
        assert!(core_metadata.id.is_none());
        assert_eq!(core_metadata.model_id, "unknown");
    }
}
