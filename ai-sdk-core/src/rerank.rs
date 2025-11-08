pub mod result;

pub use result::{RankedDocumentWithValue, RerankResponseMetadata, RerankResult};

use crate::error::AISDKError;
use crate::generate_text::prepare_retries;
use ai_sdk_provider::reranking_model::RerankingModel;
use ai_sdk_provider::reranking_model::call_options::{
    RerankingDocuments, RerankingModelCallOptions,
};
use ai_sdk_provider::shared::headers::SharedHeaders;
use ai_sdk_provider::shared::provider_options::SharedProviderOptions;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio_util::sync::CancellationToken;

/// Rerank documents using a reranking model.
///
/// The documents can be either text strings or JSON objects. The function automatically
/// detects the type from the first document.
///
/// # Arguments
///
/// * `model` - The reranking model to use.
/// * `documents` - The documents that should be reranked. Can be either text strings or JSON objects.
/// * `query` - The query string to rerank the documents against.
/// * `top_n` - Optional limit to return only the top N reranked documents.
/// * `provider_options` - Additional provider-specific options that are passed through to the provider.
/// * `max_retries` - Maximum number of retries. Set to 0 to disable retries. Default: 2.
/// * `abort_signal` - An optional abort signal that can be used to cancel the call.
/// * `headers` - Additional HTTP headers to be sent with the request. Only applicable for HTTP-based providers.
///
/// # Returns
///
/// A result object containing the original documents, reranked documents (sorted by relevance),
/// ranking information with scores, and optional provider metadata.
///
/// # Type Parameters
///
/// * `V` - The value type of documents. Must be either `String` or implement `Clone + serde::Serialize + serde::Deserialize`.
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::rerank;
///
/// // With text documents
/// let documents = vec![
///     "The sky is blue".to_string(),
///     "Grass is green".to_string(),
///     "Water is wet".to_string(),
/// ];
///
/// let result = rerank(
///     model,
///     documents,
///     "What color is the sky?".to_string(),
///     Some(2), // top 2 results
///     None,
///     None,
///     None,
///     None,
/// ).await?;
///
/// println!("Top result: {}", result.reranked_documents[0]);
/// println!("Score: {}", result.ranking[0].score);
/// ```
#[allow(clippy::too_many_arguments)]
pub async fn rerank<V>(
    model: Arc<dyn RerankingModel>,
    documents: Vec<V>,
    query: String,
    top_n: Option<usize>,
    provider_options: Option<SharedProviderOptions>,
    max_retries: Option<u32>,
    abort_signal: Option<CancellationToken>,
    headers: Option<SharedHeaders>,
) -> Result<RerankResult<V>, AISDKError>
where
    V: Clone + serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    // Check specification version
    if model.specification_version() != "v3" {
        return Err(AISDKError::model_error(format!(
            "Unsupported model version: {}. Provider: {}, Model ID: {}",
            model.specification_version(),
            model.provider(),
            model.model_id()
        )));
    }

    // Handle empty documents case - return early
    if documents.is_empty() {
        return Ok(RerankResult {
            original_documents: vec![],
            reranked_documents: vec![],
            ranking: vec![],
            provider_metadata: None,
            response: RerankResponseMetadata {
                id: None,
                timestamp: SystemTime::now(),
                model_id: model.model_id().to_string(),
                headers: None,
                body: None,
            },
        });
    }

    // Detect document type and convert to RerankingDocuments
    let documents_to_send = detect_document_type(&documents)?;

    // Prepare retry configuration
    let retry_config = prepare_retries(max_retries, abort_signal.clone())?;

    // Clone model info for logging before moving into closure
    let provider_name = model.provider().to_string();
    let model_id_str = model.model_id().to_string();

    // Execute the model call with retry logic
    let (ranking, response_metadata, provider_metadata, warnings) = retry_config
        .execute_with_boxed_error(move || {
            let model = model.clone();
            let query = query.clone();
            let documents_to_send = documents_to_send.clone();
            let provider_options = provider_options.clone();
            let abort_signal = abort_signal.clone();
            let headers = headers.clone();

            async move {
                // Call the model's do_rerank method
                let model_response = model
                    .do_rerank(RerankingModelCallOptions {
                        documents: documents_to_send,
                        query,
                        top_n,
                        provider_options,
                        headers,
                        abort_signal,
                    })
                    .await?;

                Ok((
                    model_response.ranking,
                    model_response.response,
                    model_response.provider_metadata,
                    model_response.warnings,
                ))
            }
        })
        .await?;

    // Log warnings if present
    if !warnings.is_empty() {
        eprintln!(
            "Warnings from {}/{}: {:?}",
            provider_name, model_id_str, warnings
        );
    }

    // Build the ranking with document values
    let ranking_with_values: Vec<RankedDocumentWithValue<V>> = ranking
        .iter()
        .map(|ranked| RankedDocumentWithValue {
            original_index: ranked.index,
            score: ranked.relevance_score,
            document: documents[ranked.index].clone(),
        })
        .collect();

    // Build response metadata
    let response = if let Some(resp) = response_metadata {
        RerankResponseMetadata {
            id: resp.id,
            timestamp: resp.timestamp.unwrap_or_else(SystemTime::now),
            model_id: resp.model_id.unwrap_or_else(|| model_id_str.clone()),
            headers: resp.headers,
            body: resp.body,
        }
    } else {
        RerankResponseMetadata {
            id: None,
            timestamp: SystemTime::now(),
            model_id: model_id_str.clone(),
            headers: None,
            body: None,
        }
    };

    // Create the result
    let mut result = RerankResult::new(documents, ranking_with_values, response);

    // Add provider metadata if present
    if let Some(metadata) = provider_metadata {
        result = result.with_provider_metadata(metadata);
    }

    Ok(result)
}

/// Detects the type of documents and converts them to RerankingDocuments.
///
/// This function serializes the documents to JSON and checks if they are strings or objects.
fn detect_document_type<V>(documents: &[V]) -> Result<RerankingDocuments, AISDKError>
where
    V: serde::Serialize,
{
    if documents.is_empty() {
        return Ok(RerankingDocuments::Text { values: vec![] });
    }

    // Serialize the first document to determine type
    let first_doc_json = serde_json::to_value(&documents[0]).map_err(|e| {
        AISDKError::invalid_argument("documents", "serialization failed", e.to_string())
    })?;

    match first_doc_json {
        Value::String(_) => {
            // All documents should be strings
            let values: Result<Vec<String>, AISDKError> = documents
                .iter()
                .enumerate()
                .map(|(i, doc)| {
                    let json = serde_json::to_value(doc).map_err(|e| {
                        AISDKError::invalid_argument(
                            format!("documents[{}]", i),
                            "serialization failed",
                            e.to_string(),
                        )
                    })?;
                    if let Value::String(s) = json {
                        Ok(s)
                    } else {
                        Err(AISDKError::invalid_argument(
                            format!("documents[{}]", i),
                            format!("{:?}", json),
                            "All documents must be of the same type (string or object)",
                        ))
                    }
                })
                .collect();

            Ok(RerankingDocuments::Text { values: values? })
        }
        Value::Object(_) => {
            // All documents should be objects
            let values: Result<Vec<HashMap<String, Value>>, AISDKError> = documents
                .iter()
                .enumerate()
                .map(|(i, doc)| {
                    let json = serde_json::to_value(doc).map_err(|e| {
                        AISDKError::invalid_argument(
                            format!("documents[{}]", i),
                            "serialization failed",
                            e.to_string(),
                        )
                    })?;
                    if let Value::Object(map) = json {
                        Ok(map.into_iter().collect())
                    } else {
                        Err(AISDKError::invalid_argument(
                            format!("documents[{}]", i),
                            format!("{:?}", json),
                            "All documents must be of the same type (string or object)",
                        ))
                    }
                })
                .collect();

            Ok(RerankingDocuments::Object { values: values? })
        }
        _ => Err(AISDKError::invalid_argument(
            "documents[0]",
            format!("{:?}", first_doc_json),
            "Documents must be either strings or JSON objects",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_sdk_provider::reranking_model::{
        RankedDocument, RerankingModelResponse, RerankingModelResponseMetadata,
    };
    use async_trait::async_trait;

    // Mock reranking model for testing
    struct MockRerankingModel {
        provider: String,
        model_id: String,
        ranking: Vec<RankedDocument>,
    }

    #[async_trait]
    impl RerankingModel for MockRerankingModel {
        fn specification_version(&self) -> &str {
            "v3"
        }

        fn provider(&self) -> &str {
            &self.provider
        }

        fn model_id(&self) -> &str {
            &self.model_id
        }

        async fn do_rerank(
            &self,
            _options: RerankingModelCallOptions,
        ) -> Result<RerankingModelResponse, Box<dyn std::error::Error>> {
            Ok(RerankingModelResponse {
                ranking: self.ranking.clone(),
                provider_metadata: None,
                warnings: vec![],
                response: Some(
                    RerankingModelResponseMetadata::default().with_model_id(self.model_id.clone()),
                ),
            })
        }
    }

    #[tokio::test]
    async fn test_rerank_with_text_documents() {
        let model = Arc::new(MockRerankingModel {
            provider: "test".to_string(),
            model_id: "rerank-1".to_string(),
            ranking: vec![
                RankedDocument::new(2, 0.95),
                RankedDocument::new(0, 0.85),
                RankedDocument::new(1, 0.75),
            ],
        }) as Arc<dyn RerankingModel>;

        let documents = vec![
            "First document".to_string(),
            "Second document".to_string(),
            "Third document".to_string(),
        ];

        let result = rerank(
            model,
            documents.clone(),
            "query".to_string(),
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        assert_eq!(result.original_documents, documents);
        assert_eq!(result.reranked_documents.len(), 3);
        assert_eq!(result.reranked_documents[0], "Third document");
        assert_eq!(result.reranked_documents[1], "First document");
        assert_eq!(result.reranked_documents[2], "Second document");

        assert_eq!(result.ranking[0].score, 0.95);
        assert_eq!(result.ranking[1].score, 0.85);
        assert_eq!(result.ranking[2].score, 0.75);
    }

    #[tokio::test]
    async fn test_rerank_with_empty_documents() {
        let model = Arc::new(MockRerankingModel {
            provider: "test".to_string(),
            model_id: "rerank-1".to_string(),
            ranking: vec![],
        }) as Arc<dyn RerankingModel>;

        let documents: Vec<String> = vec![];

        let result = rerank(
            model,
            documents,
            "query".to_string(),
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        assert_eq!(result.original_documents.len(), 0);
        assert_eq!(result.reranked_documents.len(), 0);
        assert_eq!(result.ranking.len(), 0);
    }

    #[tokio::test]
    async fn test_rerank_with_top_n() {
        let model = Arc::new(MockRerankingModel {
            provider: "test".to_string(),
            model_id: "rerank-1".to_string(),
            ranking: vec![RankedDocument::new(2, 0.95), RankedDocument::new(0, 0.85)],
        }) as Arc<dyn RerankingModel>;

        let documents = vec![
            "First document".to_string(),
            "Second document".to_string(),
            "Third document".to_string(),
        ];

        let result = rerank(
            model,
            documents,
            "query".to_string(),
            Some(2),
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        // With topN=2, should only get 2 results
        assert_eq!(result.reranked_documents.len(), 2);
        assert_eq!(result.ranking.len(), 2);
        assert_eq!(result.reranked_documents[0], "Third document");
        assert_eq!(result.reranked_documents[1], "First document");
    }

    #[tokio::test]
    async fn test_detect_document_type_text() {
        let documents = vec![
            "First".to_string(),
            "Second".to_string(),
            "Third".to_string(),
        ];

        let result = detect_document_type(&documents).unwrap();

        assert!(result.is_text());
        assert_eq!(result.as_text().unwrap().len(), 3);
        assert_eq!(result.as_text().unwrap()[0], "First");
    }

    #[tokio::test]
    async fn test_detect_document_type_objects() {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Clone)]
        struct Doc {
            title: String,
            content: String,
        }

        let documents = vec![
            Doc {
                title: "Doc 1".to_string(),
                content: "Content 1".to_string(),
            },
            Doc {
                title: "Doc 2".to_string(),
                content: "Content 2".to_string(),
            },
        ];

        let result = detect_document_type(&documents).unwrap();

        assert!(result.is_object());
        let objects = result.as_objects().unwrap();
        assert_eq!(objects.len(), 2);
        assert_eq!(objects[0].get("title").unwrap().as_str().unwrap(), "Doc 1");
    }

    #[tokio::test]
    async fn test_detect_document_type_empty() {
        let documents: Vec<String> = vec![];

        let result = detect_document_type(&documents).unwrap();

        assert!(result.is_text());
        assert_eq!(result.len(), 0);
    }
}
