use crate::image_model::call_options::ImageModelCallOptions;
use crate::image_model::call_warning::ImageModelCallWarning;
use crate::shared::headers::SharedHeaders;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::SystemTime;

/// Call options for image generation requests.
pub mod call_options;
/// Warning types for image generation.
pub mod call_warning;

/// Provider-specific metadata for image generation.
/// The outer map is keyed by the provider name, and the inner
/// map contains provider-specific metadata including an `images` key
/// with image-specific metadata.
///
/// Example:
/// ```json
/// {
///   "openai": {
///     "images": [{"revisedPrompt": "Revised prompt here."}]
///   }
/// }
/// ```
pub type ImageModelProviderMetadata = HashMap<String, HashMap<String, Value>>;

/// Image generation model trait.
///
/// This trait defines the interface for AI image generation models. Implementations
/// handle converting text prompts into images using provider-specific APIs.
///
/// # Specification Version
///
/// This implements version 3 of the image model interface, which supports:
/// - Text-to-image generation
/// - Multiple images per request
/// - Base64 and binary image formats
/// - Provider-specific metadata
///
/// # Examples
///
/// ```no_run
/// use llm_kit_provider::ImageModel;
/// use llm_kit_provider::image_model::call_options::ImageModelCallOptions;
/// use llm_kit_provider::image_model::ImageModelResponse;
/// use async_trait::async_trait;
///
/// struct MyImageModel {
///     model_id: String,
///     api_key: String,
/// }
///
/// #[async_trait]
/// impl ImageModel for MyImageModel {
///     fn provider(&self) -> &str {
///         "my-provider"
///     }
///
///     fn model_id(&self) -> &str {
///         &self.model_id
///     }
///
///     async fn max_images_per_call(&self, model_id: &str) -> Option<usize> {
///         Some(10) // Limit to 10 images per call
///     }
///
///     async fn do_generate(
///         &self,
///         options: ImageModelCallOptions,
///     ) -> Result<ImageModelResponse, Box<dyn std::error::Error>> {
///         // Implementation
///         todo!()
///     }
/// }
/// ```
#[async_trait]
pub trait ImageModel: Send + Sync {
    /// Returns the specification version this model implements.
    ///
    /// Defaults to "v3" for the current SDK version. This allows us to evolve
    /// the image model interface and retain backwards compatibility.
    fn specification_version(&self) -> &str {
        "v3"
    }

    /// Name of the provider for logging purposes.
    ///
    /// Examples: "openai", "stability-ai", "midjourney"
    fn provider(&self) -> &str;

    /// Provider-specific model ID for logging purposes.
    ///
    /// Examples: "dall-e-3", "stable-diffusion-xl", "midjourney-v6"
    fn model_id(&self) -> &str;

    /// Returns the maximum number of images that can be generated in a single API call.
    ///
    /// This limit may vary by model. Returns `None` for no limit, or `Some(n)` for
    /// a specific limit.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The model ID to check the limit for
    ///
    /// # Returns
    ///
    /// - `None` if there's no limit
    /// - `Some(n)` if the model is limited to n images per call
    async fn max_images_per_call(&self, model_id: &str) -> Option<usize>;

    /// Generates images from a text prompt.
    ///
    /// # Arguments
    ///
    /// * `options` - Call options including prompt, image count, size, quality, etc.
    ///
    /// # Returns
    ///
    /// An [`ImageModelResponse`] containing:
    /// - Generated images (as base64 or binary data)
    /// - Provider-specific metadata
    /// - Warnings about unsupported settings
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The API request fails
    /// - The prompt is invalid or violates content policy
    /// - The requested image count exceeds limits
    /// - Image generation fails
    async fn do_generate(
        &self,
        options: ImageModelCallOptions,
    ) -> Result<ImageModelResponse, Box<dyn std::error::Error>>;
}

/// Response from an image generation model.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageModelResponse {
    /// Generated images as base64 encoded strings or binary data.
    /// The images should be returned without any unnecessary conversion.
    /// If the API returns base64 encoded strings, the images should be returned
    /// as base64 encoded strings. If the API returns binary data, the images should
    /// be returned as binary data.
    pub images: Vec<ImageData>,

    /// Warnings for the call, e.g. unsupported settings.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub warnings: Vec<ImageModelCallWarning>,

    /// Additional provider-specific metadata. They are passed through
    /// from the provider to the LLM Kit and enable provider-specific
    /// results that can be fully encapsulated in the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<ImageModelProviderMetadata>,

    /// Response information for telemetry and debugging purposes.
    pub response: ImageModelResponseMetadata,
}

/// Image data can be either base64 encoded strings or binary data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageData {
    /// Base64 encoded image string
    Base64(String),
    /// Binary image data (serialized as array of numbers)
    Binary(Vec<u8>),
}

impl ImageData {
    /// Create image data from a base64 string
    pub fn from_base64(data: impl Into<String>) -> Self {
        Self::Base64(data.into())
    }

    /// Create image data from binary data
    pub fn from_binary(data: Vec<u8>) -> Self {
        Self::Binary(data)
    }

    /// Check if this is base64 data
    pub fn is_base64(&self) -> bool {
        matches!(self, Self::Base64(_))
    }

    /// Check if this is binary data
    pub fn is_binary(&self) -> bool {
        matches!(self, Self::Binary(_))
    }

    /// Get the base64 string if this is base64 data
    pub fn as_base64(&self) -> Option<&str> {
        match self {
            Self::Base64(s) => Some(s),
            Self::Binary(_) => None,
        }
    }

    /// Get the binary data if this is binary data
    pub fn as_binary(&self) -> Option<&[u8]> {
        match self {
            Self::Base64(_) => None,
            Self::Binary(b) => Some(b),
        }
    }
}

/// Response metadata for telemetry and debugging purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageModelResponseMetadata {
    /// Timestamp for the start of the generated response.
    #[serde(with = "system_time_as_timestamp")]
    pub timestamp: SystemTime,

    /// The ID of the response model that was used to generate the response.
    pub model_id: String,

    /// Response headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<SharedHeaders>,
}

impl ImageModelResponse {
    /// Create a new response with images.
    pub fn new(images: Vec<ImageData>, response: ImageModelResponseMetadata) -> Self {
        Self {
            images,
            warnings: Vec::new(),
            provider_metadata: None,
            response,
        }
    }

    /// Add warnings to the response.
    pub fn with_warnings(mut self, warnings: Vec<ImageModelCallWarning>) -> Self {
        self.warnings = warnings;
        self
    }

    /// Add a single warning to the response.
    pub fn with_warning(mut self, warning: ImageModelCallWarning) -> Self {
        self.warnings.push(warning);
        self
    }

    /// Add provider metadata to the response.
    pub fn with_provider_metadata(mut self, metadata: ImageModelProviderMetadata) -> Self {
        self.provider_metadata = Some(metadata);
        self
    }
}

impl ImageModelResponseMetadata {
    /// Create new response metadata.
    pub fn new(model_id: impl Into<String>) -> Self {
        Self {
            timestamp: SystemTime::now(),
            model_id: model_id.into(),
            headers: None,
        }
    }

    /// Create response metadata with a specific timestamp.
    pub fn with_timestamp(model_id: impl Into<String>, timestamp: SystemTime) -> Self {
        Self {
            timestamp,
            model_id: model_id.into(),
            headers: None,
        }
    }

    /// Add headers to the response metadata.
    pub fn with_headers(mut self, headers: SharedHeaders) -> Self {
        self.headers = Some(headers);
        self
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
