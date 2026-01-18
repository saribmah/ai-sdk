use async_trait::async_trait;
use llm_kit_provider::image_model::{
    ImageData, ImageModel, ImageModelResponse, ImageModelResponseMetadata,
    call_options::ImageModelCallOptions,
};
use llm_kit_provider::shared::headers::SharedHeaders;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

use super::settings::TogetherAIImageModelId;

/// Configuration for Together AI image model.
pub struct TogetherAIImageModelConfig {
    /// Provider name
    pub provider: String,
    /// Base URL for API requests
    pub base_url: String,
    /// Function to generate headers
    pub headers: Box<dyn Fn() -> HashMap<String, String> + Send + Sync>,
}

/// Together AI image generation model implementation.
///
/// Implements the `ImageModel` trait for Together AI's image generation API.
pub struct TogetherAIImageModel {
    /// Model identifier
    model_id: TogetherAIImageModelId,
    /// Model configuration
    config: TogetherAIImageModelConfig,
}

impl TogetherAIImageModel {
    /// Creates a new Together AI image model.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The model identifier (e.g., "black-forest-labs/FLUX.1-schnell")
    /// * `config` - Configuration for the model
    pub fn new(model_id: TogetherAIImageModelId, config: TogetherAIImageModelConfig) -> Self {
        Self { model_id, config }
    }
}

/// Together AI image generation request.
#[derive(Debug, Serialize)]
struct TogetherAIImageRequest {
    model: String,
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<u64>,
    response_format: String,
}

/// Together AI image generation response.
#[derive(Debug, Deserialize)]
struct TogetherAIImageResponse {
    data: Vec<TogetherAIImageData>,
}

#[derive(Debug, Deserialize)]
struct TogetherAIImageData {
    b64_json: String,
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
impl ImageModel for TogetherAIImageModel {
    fn provider(&self) -> &str {
        &self.config.provider
    }

    fn model_id(&self) -> &str {
        &self.model_id
    }

    async fn max_images_per_call(&self, _model_id: &str) -> Option<usize> {
        Some(1)
    }

    async fn do_generate(
        &self,
        options: ImageModelCallOptions,
    ) -> Result<ImageModelResponse, Box<dyn std::error::Error>> {
        // Check if already cancelled before starting
        if let Some(signal) = &options.abort_signal
            && signal.is_cancelled()
        {
            return Err("Operation cancelled".into());
        }

        // Extract width and height from size if provided
        let (width, height) = if let Some(size) = &options.size {
            (Some(size.width), Some(size.height))
        } else {
            (None, None)
        };

        // Build request body
        let request_body = TogetherAIImageRequest {
            model: self.model_id.clone(),
            prompt: options.prompt.clone(),
            width,
            height,
            n: Some(options.n),
            seed: options.seed.map(|s| s as u64),
            response_format: "base64".to_string(),
        };

        let body_string = serde_json::to_string(&request_body)?;

        // Build URL
        let url = format!("{}/images/generations", self.config.base_url);

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
        let api_response: TogetherAIImageResponse = serde_json::from_str(&response_body)?;

        // Extract base64 images
        let images: Vec<ImageData> = api_response
            .data
            .into_iter()
            .map(|data| ImageData::from_base64(data.b64_json))
            .collect();

        // Build response headers map
        let mut headers_map: SharedHeaders = HashMap::new();
        for (key, value) in response_headers.iter() {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(key.as_str().to_string(), value_str.to_string());
            }
        }

        Ok(ImageModelResponse {
            images,
            response: ImageModelResponseMetadata {
                timestamp: SystemTime::now(),
                model_id: self.model_id.clone(),
                headers: Some(headers_map),
            },
            warnings: Vec::new(),
            provider_metadata: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_model() {
        let config = TogetherAIImageModelConfig {
            provider: "togetherai.image".to_string(),
            base_url: "https://api.together.xyz/v1".to_string(),
            headers: Box::new(HashMap::new),
        };

        let model =
            TogetherAIImageModel::new("black-forest-labs/FLUX.1-schnell".to_string(), config);

        assert_eq!(model.model_id(), "black-forest-labs/FLUX.1-schnell");
        assert_eq!(model.provider(), "togetherai.image");
        assert_eq!(
            model
                .max_images_per_call("black-forest-labs/FLUX.1-schnell")
                .await,
            Some(1)
        );
    }
}
