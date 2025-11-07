use ai_sdk_provider::image_model::call_options::ImageModelCallOptions;
use ai_sdk_provider::image_model::call_warning::ImageModelCallWarning;
use ai_sdk_provider::image_model::{
    ImageData, ImageModel, ImageModelResponse, ImageModelResponseMetadata,
};
use async_trait::async_trait;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::time::SystemTime;

use crate::error::OpenAICompatibleError;
use crate::image::OpenAICompatibleImageModelId;

/// Type alias for URL generation function
pub type UrlGeneratorFn = Box<dyn Fn(&str, &str) -> String + Send + Sync>;

/// Configuration for an OpenAI-compatible image model
pub struct OpenAICompatibleImageModelConfig {
    /// Provider name (e.g., "openai", "azure", "custom")
    pub provider: String,

    /// Function to generate headers for API requests
    pub headers: Box<dyn Fn() -> HashMap<String, String> + Send + Sync>,

    /// Function to generate the URL for API requests
    pub url: UrlGeneratorFn,

    /// Optional custom fetch function
    pub fetch: Option<fn()>, // TODO: proper fetch function type
}

impl Default for OpenAICompatibleImageModelConfig {
    fn default() -> Self {
        Self {
            provider: "openai-compatible".to_string(),
            headers: Box::new(HashMap::new),
            url: Box::new(|_model_id, path| format!("https://api.openai.com/v1{}", path)),
            fetch: None,
        }
    }
}

/// OpenAI-compatible image model implementation
pub struct OpenAICompatibleImageModel {
    /// The model identifier
    model_id: OpenAICompatibleImageModelId,

    /// Configuration for the model
    config: OpenAICompatibleImageModelConfig,
}

impl OpenAICompatibleImageModel {
    /// Create a new OpenAI-compatible image model
    ///
    /// # Arguments
    ///
    /// * `model_id` - The model identifier (e.g., "dall-e-3", "dall-e-2")
    /// * `config` - Configuration for the model
    ///
    /// # Returns
    ///
    /// A new `OpenAICompatibleImageModel` instance
    pub fn new(
        model_id: OpenAICompatibleImageModelId,
        config: OpenAICompatibleImageModelConfig,
    ) -> Self {
        Self { model_id, config }
    }
}

#[async_trait]
impl ImageModel for OpenAICompatibleImageModel {
    fn specification_version(&self) -> &str {
        "v3"
    }

    fn provider(&self) -> &str {
        &self.config.provider
    }

    fn model_id(&self) -> &str {
        &self.model_id
    }

    async fn max_images_per_call(&self, _model_id: &str) -> Option<usize> {
        Some(10)
    }

    async fn do_generate(
        &self,
        options: ImageModelCallOptions,
    ) -> Result<ImageModelResponse, Box<dyn std::error::Error>> {
        let mut warnings: Vec<ImageModelCallWarning> = Vec::new();

        // Check for unsupported settings
        if options.aspect_ratio.is_some() {
            warnings.push(ImageModelCallWarning::UnsupportedSetting {
                setting: "aspectRatio".to_string(),
                details: Some(
                    "This model does not support aspect ratio. Use `size` instead.".to_string(),
                ),
            });
        }

        if options.seed.is_some() {
            warnings.push(ImageModelCallWarning::UnsupportedSetting {
                setting: "seed".to_string(),
                details: None,
            });
        }

        // Build the request body
        let mut body = json!({
            "model": self.model_id,
            "prompt": options.prompt,
            "response_format": "b64_json",
        });

        // Add optional parameters
        body["n"] = json!(options.n);

        if let Some(size) = options.size {
            body["size"] = json!(size.to_string());
        }

        // Add provider options if present
        if let Some(provider_options) = options.provider_options {
            // Check for "openai" provider options
            if let Some(openai_options) = provider_options.get("openai") {
                for (key, value) in openai_options {
                    body[key] = value.clone();
                }
            }
        }

        // Get headers
        let headers = (self.config.headers)();

        // Build the URL
        let url = (self.config.url)(&self.model_id, "/images/generations");

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

            return Err(Box::new(OpenAICompatibleError::ApiError {
                status: status.as_u16(),
                body: error_body,
            }));
        }

        // Parse response
        let response_body: OpenAIImageResponse = response.json().await?;

        // Extract images (convert base64 strings to ImageData::Base64)
        let images: Vec<ImageData> = response_body
            .data
            .into_iter()
            .map(|item| ImageData::Base64(item.b64_json))
            .collect();

        // Get current timestamp
        let timestamp = SystemTime::now();

        // Build response metadata
        let response_metadata =
            ImageModelResponseMetadata::with_timestamp(self.model_id.clone(), timestamp)
                .with_headers(response_headers);

        // Build response
        let mut image_response = ImageModelResponse::new(images, response_metadata);

        // Add warnings if any
        if !warnings.is_empty() {
            image_response = image_response.with_warnings(warnings);
        }

        Ok(image_response)
    }
}

/// OpenAI image API response structure
#[derive(Debug, Clone, Deserialize, Serialize)]
struct OpenAIImageResponse {
    data: Vec<OpenAIImageData>,
}

/// OpenAI image data structure
#[derive(Debug, Clone, Deserialize, Serialize)]
struct OpenAIImageData {
    b64_json: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_model() {
        let config = OpenAICompatibleImageModelConfig::default();
        let model = OpenAICompatibleImageModel::new("dall-e-3".to_string(), config);

        assert_eq!(model.model_id(), "dall-e-3");
        assert_eq!(model.provider(), "openai-compatible");
        assert_eq!(model.specification_version(), "v3");
    }

    #[test]
    fn test_config_default() {
        let config = OpenAICompatibleImageModelConfig::default();

        assert_eq!(config.provider, "openai-compatible");
    }

    #[test]
    fn test_config_custom_provider() {
        let config = OpenAICompatibleImageModelConfig {
            provider: "custom-provider".to_string(),
            ..Default::default()
        };

        assert_eq!(config.provider, "custom-provider");
    }

    #[tokio::test]
    async fn test_max_images_per_call() {
        let config = OpenAICompatibleImageModelConfig::default();
        let model = OpenAICompatibleImageModel::new("dall-e-3".to_string(), config);

        assert_eq!(model.max_images_per_call("dall-e-3").await, Some(10));
    }

    #[test]
    fn test_openai_image_response_deserialization() {
        let json = r#"{
            "data": [
                {"b64_json": "base64string1"},
                {"b64_json": "base64string2"}
            ]
        }"#;

        let response: OpenAIImageResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.data.len(), 2);
        assert_eq!(response.data[0].b64_json, "base64string1");
        assert_eq!(response.data[1].b64_json, "base64string2");
    }

    #[test]
    fn test_openai_image_response_single_image() {
        let json = r#"{
            "data": [
                {"b64_json": "singleimage"}
            ]
        }"#;

        let response: OpenAIImageResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.data.len(), 1);
        assert_eq!(response.data[0].b64_json, "singleimage");
    }
}
