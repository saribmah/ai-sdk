use async_trait::async_trait;
use llm_kit_provider::reranking_model::{
    RankedDocument, RerankingModel, RerankingModelResponse, RerankingModelResponseMetadata,
    call_options::RerankingModelCallOptions,
};
use llm_kit_provider::shared::headers::SharedHeaders;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::options::{TogetherAIRerankingModelId, TogetherAIRerankingOptions};

/// Configuration for Together AI reranking model.
pub struct TogetherAIRerankingModelConfig {
    /// Provider name
    pub provider: String,
    /// Base URL for API requests
    pub base_url: String,
    /// Function to generate headers
    pub headers: Box<dyn Fn() -> HashMap<String, String> + Send + Sync>,
}

/// Together AI reranking model implementation.
///
/// Implements the `RerankingModel` trait for Together AI's reranking API.
pub struct TogetherAIRerankingModel {
    /// Model identifier
    model_id: TogetherAIRerankingModelId,
    /// Model configuration
    config: TogetherAIRerankingModelConfig,
}

impl TogetherAIRerankingModel {
    /// Creates a new Together AI reranking model.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The model identifier (e.g., "Salesforce/Llama-Rank-v1")
    /// * `config` - Configuration for the model
    pub fn new(
        model_id: TogetherAIRerankingModelId,
        config: TogetherAIRerankingModelConfig,
    ) -> Self {
        Self { model_id, config }
    }

    /// Extracts Together AI specific options from provider options.
    fn extract_togetherai_options(
        provider_options: &HashMap<String, HashMap<String, serde_json::Value>>,
    ) -> TogetherAIRerankingOptions {
        let mut options = TogetherAIRerankingOptions::default();

        if let Some(togetherai_opts) = provider_options.get("togetherai")
            && let Some(rank_fields) = togetherai_opts.get("rankFields")
            && let Some(fields) = rank_fields.as_array()
        {
            options.rank_fields = Some(
                fields
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect(),
            );
        }

        options
    }
}

/// Together AI reranking request.
#[derive(Debug, Serialize)]
struct TogetherAIRerankRequest {
    model: String,
    query: String,
    documents: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rank_fields: Option<Vec<String>>,
    return_documents: bool,
}

/// Together AI reranking response.
#[derive(Debug, Deserialize)]
struct TogetherAIRerankResponse {
    #[allow(dead_code)]
    id: Option<String>,
    #[allow(dead_code)]
    model: Option<String>,
    results: Vec<TogetherAIRerankResult>,
}

#[derive(Debug, Deserialize)]
struct TogetherAIRerankResult {
    index: usize,
    relevance_score: f64,
}

/// Together AI error response.
#[derive(Debug, Deserialize)]
struct TogetherAIErrorResponse {
    error: TogetherAIError,
}

#[derive(Debug, Deserialize)]
struct TogetherAIError {
    message: String,
}

#[async_trait]
impl RerankingModel for TogetherAIRerankingModel {
    fn provider(&self) -> &str {
        &self.config.provider
    }

    fn model_id(&self) -> &str {
        &self.model_id
    }

    async fn do_rerank(
        &self,
        options: RerankingModelCallOptions,
    ) -> Result<RerankingModelResponse, Box<dyn std::error::Error>> {
        // Check if already cancelled before starting
        if let Some(signal) = &options.abort_signal
            && signal.is_cancelled()
        {
            return Err("Operation cancelled".into());
        }

        // Extract Together AI specific options
        let togetherai_options =
            Self::extract_togetherai_options(&options.provider_options.unwrap_or_default());

        // Extract document values based on document type
        let document_values = match &options.documents {
            llm_kit_provider::reranking_model::call_options::RerankingDocuments::Text {
                values,
            } => values
                .iter()
                .map(|s| serde_json::Value::String(s.clone()))
                .collect(),
            llm_kit_provider::reranking_model::call_options::RerankingDocuments::Object {
                values,
            } => values
                .iter()
                .map(|obj| serde_json::to_value(obj).unwrap_or(serde_json::Value::Null))
                .collect(),
        };

        // Build request body
        let request_body = TogetherAIRerankRequest {
            model: self.model_id.clone(),
            query: options.query.clone(),
            documents: document_values,
            top_n: options.top_n.map(|n| n as u32),
            rank_fields: togetherai_options.rank_fields,
            return_documents: false, // Reduce response size
        };

        let body_string = serde_json::to_string(&request_body)?;

        // Build URL
        let url = format!("{}/rerank", self.config.base_url);

        // Build headers
        let mut headers = (self.config.headers)();
        if let Some(option_headers) = &options.headers {
            headers.extend(option_headers.clone());
        }

        // Make API request
        let client = reqwest::Client::new();
        let mut request = client.post(&url).header("Content-Type", "application/json");

        for (key, value) in headers {
            request = request.header(key, value);
        }

        // Send request with optional cancellation support
        let response = if let Some(signal) = &options.abort_signal {
            tokio::select! {
                result = request.body(body_string).send() => result?,
                _ = signal.cancelled() => {
                    return Err("Operation cancelled".into());
                }
            }
        } else {
            request.body(body_string).send().await?
        };

        let status = response.status();
        let response_headers = response.headers().clone();

        if !status.is_success() {
            let error_body = response.text().await?;

            // Try to parse as Together AI error format
            if let Ok(error_response) = serde_json::from_str::<TogetherAIErrorResponse>(&error_body)
            {
                return Err(format!(
                    "API request failed with status {}: {}",
                    status, error_response.error.message
                )
                .into());
            }

            return Err(
                format!("API request failed with status {}: {}", status, error_body).into(),
            );
        }

        let response_body = response.text().await?;
        let api_response: TogetherAIRerankResponse = serde_json::from_str(&response_body)?;

        // Convert results
        let ranking: Vec<RankedDocument> = api_response
            .results
            .into_iter()
            .map(|result| RankedDocument::new(result.index, result.relevance_score))
            .collect();

        // Build response headers map
        let mut headers_map: SharedHeaders = HashMap::new();
        for (key, value) in response_headers.iter() {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(key.as_str().to_string(), value_str.to_string());
            }
        }

        Ok(RerankingModelResponse {
            ranking,
            response: Some(RerankingModelResponseMetadata {
                id: api_response.id,
                model_id: api_response.model,
                timestamp: None,
                headers: Some(headers_map),
                body: Some(serde_json::from_str(&response_body)?),
            }),
            provider_metadata: None,
            warnings: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_model() {
        let config = TogetherAIRerankingModelConfig {
            provider: "togetherai.reranking".to_string(),
            base_url: "https://api.together.xyz/v1".to_string(),
            headers: Box::new(HashMap::new),
        };

        let model = TogetherAIRerankingModel::new("Salesforce/Llama-Rank-v1".to_string(), config);

        assert_eq!(model.model_id(), "Salesforce/Llama-Rank-v1");
        assert_eq!(model.provider(), "togetherai.reranking");
    }

    #[test]
    fn test_extract_togetherai_options() {
        let mut inner_map = HashMap::new();
        inner_map.insert(
            "rankFields".to_string(),
            serde_json::json!(["title", "text"]),
        );

        let mut provider_options = HashMap::new();
        provider_options.insert("togetherai".to_string(), inner_map);

        let options = TogetherAIRerankingModel::extract_togetherai_options(&provider_options);

        assert_eq!(
            options.rank_fields,
            Some(vec!["title".to_string(), "text".to_string()])
        );
    }
}
