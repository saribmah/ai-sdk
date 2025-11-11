use ai_sdk_provider::embedding_model::call_options::EmbeddingModelCallOptions;
use ai_sdk_provider::embedding_model::embedding::EmbeddingModelEmbedding;
use ai_sdk_provider::embedding_model::{
    EmbeddingModel, EmbeddingModelResponse, EmbeddingModelResponseMetadata, EmbeddingModelUsage,
};
use ai_sdk_provider::error::ProviderError;
use async_trait::async_trait;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;

use crate::embedding::OpenAICompatibleEmbeddingModelId;

/// Type alias for URL generation function
pub type UrlGeneratorFn = Box<dyn Fn(&str, &str) -> String + Send + Sync>;

/// Configuration for an OpenAI-compatible embedding model
pub struct OpenAICompatibleEmbeddingConfig {
    /// Provider name (e.g., "openai", "azure", "custom")
    pub provider: String,

    /// Function to generate headers for API requests
    pub headers: Box<dyn Fn() -> HashMap<String, String> + Send + Sync>,

    /// Function to generate the URL for API requests
    pub url: UrlGeneratorFn,

    /// Override the maximum number of embeddings per call
    pub max_embeddings_per_call: Option<usize>,

    /// Override the parallelism of embedding calls
    pub supports_parallel_calls: Option<bool>,
}

impl Default for OpenAICompatibleEmbeddingConfig {
    fn default() -> Self {
        Self {
            provider: "openai-compatible".to_string(),
            headers: Box::new(HashMap::new),
            url: Box::new(|_model_id, path| format!("https://api.openai.com/v1{}", path)),
            max_embeddings_per_call: None,
            supports_parallel_calls: None,
        }
    }
}

/// OpenAI-compatible embedding model implementation
pub struct OpenAICompatibleEmbeddingModel {
    /// The model identifier
    model_id: OpenAICompatibleEmbeddingModelId,

    /// Configuration for the model
    config: OpenAICompatibleEmbeddingConfig,
}

impl OpenAICompatibleEmbeddingModel {
    /// Create a new OpenAI-compatible embedding model
    ///
    /// # Arguments
    ///
    /// * `model_id` - The model identifier (e.g., "text-embedding-3-small", "text-embedding-ada-002")
    /// * `config` - Configuration for the model
    ///
    /// # Returns
    ///
    /// A new `OpenAICompatibleEmbeddingModel` instance
    pub fn new(
        model_id: OpenAICompatibleEmbeddingModelId,
        config: OpenAICompatibleEmbeddingConfig,
    ) -> Self {
        Self { model_id, config }
    }

    /// Get the provider options name (first part of provider string before '.')
    fn provider_options_name(&self) -> &str {
        self.config
            .provider
            .split('.')
            .next()
            .unwrap_or(&self.config.provider)
    }
}

#[async_trait]
impl EmbeddingModel<String> for OpenAICompatibleEmbeddingModel {
    fn provider(&self) -> &str {
        &self.config.provider
    }

    fn model_id(&self) -> &str {
        &self.model_id
    }

    async fn max_embeddings_per_call(&self) -> Option<usize> {
        Some(self.config.max_embeddings_per_call.unwrap_or(2048))
    }

    async fn supports_parallel_calls(&self) -> bool {
        self.config.supports_parallel_calls.unwrap_or(true)
    }

    async fn do_embed(
        &self,
        options: EmbeddingModelCallOptions<String>,
    ) -> Result<EmbeddingModelResponse, Box<dyn std::error::Error>> {
        let values = options.values;

        // Check if we exceed the maximum embeddings per call
        if let Some(max) = self.config.max_embeddings_per_call
            && values.len() > max
        {
            return Err(Box::new(ProviderError::too_many_embedding_values_for_call(
                self.config.provider.clone(),
                self.model_id.clone(),
                max,
                values.len(),
            )));
        }

        // Build the request body
        let mut body = json!({
            "model": self.model_id,
            "input": values,
            "encoding_format": "float",
        });

        // Add provider options if present
        if let Some(provider_options) = options.provider_options {
            // Check for "openai-compatible" provider options
            if let Some(compat_options) = provider_options.get("openai-compatible") {
                if let Some(dimensions) = compat_options.get("dimensions") {
                    body["dimensions"] = dimensions.clone();
                }
                if let Some(user) = compat_options.get("user") {
                    body["user"] = user.clone();
                }
            }

            // Also check for provider-specific options
            if let Some(provider_specific_options) =
                provider_options.get(self.provider_options_name())
            {
                if let Some(dimensions) = provider_specific_options.get("dimensions") {
                    body["dimensions"] = dimensions.clone();
                }
                if let Some(user) = provider_specific_options.get("user") {
                    body["user"] = user.clone();
                }
            }
        }

        // Get headers
        let headers = (self.config.headers)();

        // Build the URL
        let url = (self.config.url)(&self.model_id, "/embeddings");

        // Create HTTP client
        let client = reqwest::Client::new();

        // Build request
        let mut request = client.post(&url).json(&body);

        // Add headers
        for (key, value) in headers {
            request = request.header(&key, value);
        }

        // Send request
        let response = request.send().await?;

        // Get response headers
        let response_headers: HashMap<String, String> = response
            .headers()
            .iter()
            .filter_map(|(k, v)| {
                v.to_str()
                    .ok()
                    .map(|s| (k.as_str().to_string(), s.to_string()))
            })
            .collect();

        // Check for errors
        if !response.status().is_success() {
            let status = response.status();
            let error_body: Value = response.json().await?;
            let error_body_str = serde_json::to_string(&error_body).unwrap_or_default();

            return Err(Box::new(ProviderError::api_call_error_with_details(
                format!("API request failed with status {}", status.as_u16()),
                url.clone(),
                serde_json::to_string(&body).unwrap_or_default(),
                Some(status.as_u16()),
                Some(response_headers),
                Some(error_body_str),
                None, // Auto-determine retryability based on status code
                None,
                None,
            )));
        }

        // Parse response
        let response_body: Value = response.json().await?;
        let response_data: OpenAIEmbeddingResponse = serde_json::from_value(response_body.clone())?;

        // Extract embeddings (EmbeddingModelEmbedding is just Vec<f64>)
        let embeddings: Vec<EmbeddingModelEmbedding> = response_data
            .data
            .into_iter()
            .map(|item| item.embedding)
            .collect();

        // Build response
        let mut embedding_response = EmbeddingModelResponse::new(embeddings);

        // Add usage if present
        if let Some(usage) = response_data.usage {
            embedding_response =
                embedding_response.with_usage(EmbeddingModelUsage::new(usage.prompt_tokens));
        }

        // Add response metadata (SharedHeaders is just HashMap<String, String>)
        let response_metadata = EmbeddingModelResponseMetadata::new()
            .with_headers(response_headers)
            .with_body(response_body);

        embedding_response = embedding_response.with_response_metadata(response_metadata);

        Ok(embedding_response)
    }
}

/// OpenAI embedding API response structure
#[derive(Debug, Clone, Deserialize, Serialize)]
struct OpenAIEmbeddingResponse {
    data: Vec<OpenAIEmbeddingData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage: Option<OpenAIEmbeddingUsage>,
}

/// OpenAI embedding data structure
#[derive(Debug, Clone, Deserialize, Serialize)]
struct OpenAIEmbeddingData {
    embedding: Vec<f64>,
}

/// OpenAI embedding usage structure
#[derive(Debug, Clone, Deserialize, Serialize)]
struct OpenAIEmbeddingUsage {
    prompt_tokens: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_model() {
        let config = OpenAICompatibleEmbeddingConfig::default();
        let model =
            OpenAICompatibleEmbeddingModel::new("text-embedding-3-small".to_string(), config);

        assert_eq!(model.model_id(), "text-embedding-3-small");
        assert_eq!(model.provider(), "openai-compatible");
    }

    #[test]
    fn test_config_default() {
        let config = OpenAICompatibleEmbeddingConfig::default();

        assert_eq!(config.provider, "openai-compatible");
        assert_eq!(config.max_embeddings_per_call, None);
        assert_eq!(config.supports_parallel_calls, None);
    }

    #[test]
    fn test_config_custom_provider() {
        let config = OpenAICompatibleEmbeddingConfig {
            provider: "custom-provider".to_string(),
            ..Default::default()
        };

        assert_eq!(config.provider, "custom-provider");
    }

    #[tokio::test]
    async fn test_max_embeddings_per_call() {
        let config = OpenAICompatibleEmbeddingConfig {
            max_embeddings_per_call: Some(100),
            ..Default::default()
        };
        let model = OpenAICompatibleEmbeddingModel::new("test-model".to_string(), config);

        assert_eq!(model.max_embeddings_per_call().await, Some(100));
    }

    #[tokio::test]
    async fn test_max_embeddings_per_call_default() {
        let config = OpenAICompatibleEmbeddingConfig::default();
        let model = OpenAICompatibleEmbeddingModel::new("test-model".to_string(), config);

        assert_eq!(model.max_embeddings_per_call().await, Some(2048));
    }

    #[tokio::test]
    async fn test_supports_parallel_calls() {
        let config = OpenAICompatibleEmbeddingConfig {
            supports_parallel_calls: Some(false),
            ..Default::default()
        };
        let model = OpenAICompatibleEmbeddingModel::new("test-model".to_string(), config);

        assert!(!model.supports_parallel_calls().await);
    }

    #[tokio::test]
    async fn test_supports_parallel_calls_default() {
        let config = OpenAICompatibleEmbeddingConfig::default();
        let model = OpenAICompatibleEmbeddingModel::new("test-model".to_string(), config);

        assert!(model.supports_parallel_calls().await);
    }

    #[test]
    fn test_provider_options_name() {
        let config = OpenAICompatibleEmbeddingConfig {
            provider: "openai.chat".to_string(),
            ..Default::default()
        };
        let model = OpenAICompatibleEmbeddingModel::new("test-model".to_string(), config);

        assert_eq!(model.provider_options_name(), "openai");
    }

    #[test]
    fn test_provider_options_name_no_dot() {
        let config = OpenAICompatibleEmbeddingConfig {
            provider: "openai".to_string(),
            ..Default::default()
        };
        let model = OpenAICompatibleEmbeddingModel::new("test-model".to_string(), config);

        assert_eq!(model.provider_options_name(), "openai");
    }

    #[test]
    fn test_openai_embedding_response_deserialization() {
        let json = r#"{
            "data": [
                {"embedding": [0.1, 0.2, 0.3]},
                {"embedding": [0.4, 0.5, 0.6]}
            ],
            "usage": {
                "prompt_tokens": 10
            }
        }"#;

        let response: OpenAIEmbeddingResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].embedding, vec![0.1, 0.2, 0.3]);
        assert_eq!(response.data[1].embedding, vec![0.4, 0.5, 0.6]);
        assert_eq!(response.usage.unwrap().prompt_tokens, 10);
    }

    #[test]
    fn test_openai_embedding_response_deserialization_no_usage() {
        let json = r#"{
            "data": [
                {"embedding": [0.1, 0.2, 0.3]}
            ]
        }"#;

        let response: OpenAIEmbeddingResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].embedding, vec![0.1, 0.2, 0.3]);
        assert!(response.usage.is_none());
    }
}
