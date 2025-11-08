use crate::reranking_model::call_options::RerankingModelCallOptions;
use crate::shared::headers::SharedHeaders;
use crate::shared::provider_metadata::SharedProviderMetadata;
use crate::shared::warning::SharedWarning;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::SystemTime;

pub mod call_options;

/// Specification for a reranking model that implements the reranking model
/// interface version 3.
#[async_trait]
pub trait RerankingModel: Send + Sync {
    /// The reranking model must specify which reranking model interface
    /// version it implements. This will allow us to evolve the reranking
    /// model interface and retain backwards compatibility. The different
    /// implementation versions can be handled as a discriminated union
    /// on our side.
    fn specification_version(&self) -> &str {
        "v3"
    }

    /// Provider ID.
    fn provider(&self) -> &str;

    /// Provider-specific model ID.
    fn model_id(&self) -> &str;

    /// Reranking a list of documents using the query.
    ///
    /// Naming: "do" prefix to prevent accidental direct usage of the method
    /// by the user.
    async fn do_rerank(
        &self,
        options: RerankingModelCallOptions,
    ) -> Result<RerankingModelResponse, Box<dyn std::error::Error>>;
}

/// Response from a reranking model.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RerankingModelResponse {
    /// Ordered list of reranked documents (via index before reranking).
    /// The documents are sorted by the descending order of relevance scores.
    pub ranking: Vec<RankedDocument>,

    /// Additional provider-specific metadata. They are passed through
    /// from the provider to the AI SDK and enable provider-specific
    /// functionality that can be fully encapsulated in the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,

    /// Warnings for the call, e.g. unsupported settings.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub warnings: Vec<SharedWarning>,

    /// Optional response information for debugging purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<RerankingModelResponseMetadata>,
}

/// A ranked document with its relevance score.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RankedDocument {
    /// The index of the document in the original list of documents before reranking.
    pub index: usize,

    /// The relevance score of the document after reranking.
    pub relevance_score: f64,
}

impl RankedDocument {
    /// Create a new ranked document.
    pub fn new(index: usize, relevance_score: f64) -> Self {
        Self {
            index,
            relevance_score,
        }
    }
}

impl PartialOrd for RankedDocument {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Sort by relevance score in descending order
        other.relevance_score.partial_cmp(&self.relevance_score)
    }
}

/// Response metadata for debugging purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RerankingModelResponseMetadata {
    /// ID for the generated response, if the provider sends one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Timestamp for the start of the generated response, if the provider sends one.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "optional_system_time_as_timestamp")]
    pub timestamp: Option<SystemTime>,

    /// The ID of the response model that was used to generate the response,
    /// if the provider sends one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,

    /// Response headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<SharedHeaders>,

    /// Response body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Value>,
}

impl RerankingModelResponse {
    /// Create a new response with ranking.
    pub fn new(ranking: Vec<RankedDocument>) -> Self {
        Self {
            ranking,
            provider_metadata: None,
            warnings: Vec::new(),
            response: None,
        }
    }

    /// Create a new response and sort the ranking by relevance score.
    pub fn new_sorted(mut ranking: Vec<RankedDocument>) -> Self {
        ranking.sort_by(|a, b| {
            b.relevance_score
                .partial_cmp(&a.relevance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Self::new(ranking)
    }

    /// Add provider metadata to the response.
    pub fn with_provider_metadata(mut self, metadata: SharedProviderMetadata) -> Self {
        self.provider_metadata = Some(metadata);
        self
    }

    /// Add warnings to the response.
    pub fn with_warnings(mut self, warnings: Vec<SharedWarning>) -> Self {
        self.warnings = warnings;
        self
    }

    /// Add a single warning to the response.
    pub fn with_warning(mut self, warning: SharedWarning) -> Self {
        self.warnings.push(warning);
        self
    }

    /// Add response metadata to the response.
    pub fn with_response_metadata(mut self, metadata: RerankingModelResponseMetadata) -> Self {
        self.response = Some(metadata);
        self
    }

    /// Get the top N documents from the ranking.
    pub fn top_n(&self, n: usize) -> &[RankedDocument] {
        let end = n.min(self.ranking.len());
        &self.ranking[..end]
    }
}

impl RerankingModelResponseMetadata {
    /// Create new response metadata.
    pub fn new() -> Self {
        Self {
            id: None,
            timestamp: None,
            model_id: None,
            headers: None,
            body: None,
        }
    }

    /// Add an ID to the response metadata.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Add a timestamp to the response metadata.
    pub fn with_timestamp(mut self, timestamp: SystemTime) -> Self {
        self.timestamp = Some(timestamp);
        self
    }

    /// Add a model ID to the response metadata.
    pub fn with_model_id(mut self, model_id: impl Into<String>) -> Self {
        self.model_id = Some(model_id.into());
        self
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

impl Default for RerankingModelResponseMetadata {
    fn default() -> Self {
        Self::new()
    }
}

// Serde module for optional SystemTime serialization
mod optional_system_time_as_timestamp {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn serialize<S>(time: &Option<SystemTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match time {
            Some(t) => {
                let duration = t
                    .duration_since(UNIX_EPOCH)
                    .map_err(serde::ser::Error::custom)?;
                serializer.serialize_some(&(duration.as_millis() as u64))
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<SystemTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis: Option<u64> = Option::deserialize(deserializer)?;
        Ok(millis.map(|m| UNIX_EPOCH + std::time::Duration::from_millis(m)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ranked_document_ordering() {
        let doc1 = RankedDocument::new(0, 0.9);
        let doc2 = RankedDocument::new(1, 0.7);
        let doc3 = RankedDocument::new(2, 0.95);

        let mut ranking = [doc1, doc2, doc3];
        ranking.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert_eq!(ranking[0].index, 2); // 0.95
        assert_eq!(ranking[1].index, 0); // 0.9
        assert_eq!(ranking[2].index, 1); // 0.7
    }

    #[test]
    fn test_new_sorted() {
        let doc1 = RankedDocument::new(0, 0.7);
        let doc2 = RankedDocument::new(1, 0.95);
        let doc3 = RankedDocument::new(2, 0.85);

        let response = RerankingModelResponse::new_sorted(vec![doc1, doc2, doc3]);

        assert_eq!(response.ranking[0].index, 1); // 0.95
        assert_eq!(response.ranking[1].index, 2); // 0.85
        assert_eq!(response.ranking[2].index, 0); // 0.7
    }

    #[test]
    fn test_top_n() {
        let doc1 = RankedDocument::new(0, 0.9);
        let doc2 = RankedDocument::new(1, 0.7);
        let doc3 = RankedDocument::new(2, 0.95);

        let response = RerankingModelResponse::new_sorted(vec![doc1, doc2, doc3]);

        let top_2 = response.top_n(2);
        assert_eq!(top_2.len(), 2);
        assert_eq!(top_2[0].index, 2); // 0.95
        assert_eq!(top_2[1].index, 0); // 0.9
    }

    #[test]
    fn test_top_n_exceeds_length() {
        let doc1 = RankedDocument::new(0, 0.9);
        let doc2 = RankedDocument::new(1, 0.7);

        let response = RerankingModelResponse::new_sorted(vec![doc1, doc2]);

        let top_5 = response.top_n(5);
        assert_eq!(top_5.len(), 2); // Only 2 documents available
    }
}
