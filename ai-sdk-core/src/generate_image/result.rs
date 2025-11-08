use crate::generate_text::GeneratedFile;
use ai_sdk_provider::image_model::call_warning::ImageModelCallWarning;
use ai_sdk_provider::image_model::{ImageData, ImageModelProviderMetadata};
use ai_sdk_provider::shared::headers::SharedHeaders;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Response metadata from the provider for an image generation call.
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

/// The result of a `generate_image` call.
/// It contains the images and additional information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateImageResult {
    /// The first image that was generated.
    pub image: GeneratedFile,

    /// The images that were generated.
    pub images: Vec<GeneratedFile>,

    /// Warnings for the call, e.g. unsupported settings.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub warnings: Vec<ImageModelCallWarning>,

    /// Response metadata from the provider. There may be multiple responses
    /// if we made multiple calls to the model.
    pub responses: Vec<ImageModelResponseMetadata>,

    /// Provider-specific metadata. They are passed through from the provider
    /// to the AI SDK and enable provider-specific results that can be fully
    /// encapsulated in the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<ImageModelProviderMetadata>,
}

impl GenerateImageResult {
    /// Create a new generate image result.
    ///
    /// # Arguments
    ///
    /// * `images` - The generated images
    /// * `responses` - Response metadata from provider calls
    ///
    /// # Panics
    ///
    /// Panics if `images` is empty.
    ///
    /// # Returns
    ///
    /// A new `GenerateImageResult` instance
    pub fn new(images: Vec<GeneratedFile>, responses: Vec<ImageModelResponseMetadata>) -> Self {
        assert!(!images.is_empty(), "images cannot be empty");

        let first_image = images[0].clone();

        Self {
            image: first_image,
            images,
            warnings: Vec::new(),
            responses,
            provider_metadata: None,
        }
    }

    /// Add warnings to the result.
    pub fn with_warnings(mut self, warnings: Vec<ImageModelCallWarning>) -> Self {
        self.warnings = warnings;
        self
    }

    /// Add a single warning to the result.
    pub fn with_warning(mut self, warning: ImageModelCallWarning) -> Self {
        self.warnings.push(warning);
        self
    }

    /// Add provider metadata to the result.
    pub fn with_provider_metadata(mut self, metadata: ImageModelProviderMetadata) -> Self {
        self.provider_metadata = Some(metadata);
        self
    }
}

/// Helper function to convert ImageData to GeneratedFile.
#[allow(dead_code)]
pub(crate) fn image_data_to_generated_file(
    image_data: ImageData,
    media_type: impl Into<String>,
) -> GeneratedFile {
    match image_data {
        ImageData::Base64(base64) => GeneratedFile::from_base64(base64, media_type),
        ImageData::Binary(bytes) => GeneratedFile::from_bytes(bytes, media_type),
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
    use ai_sdk_provider::image_model::ImageData;
    use std::collections::HashMap;

    #[test]
    fn test_image_model_response_metadata_new() {
        let metadata = ImageModelResponseMetadata::new("dall-e-3");

        assert_eq!(metadata.model_id, "dall-e-3");
        assert!(metadata.headers.is_none());
    }

    #[test]
    fn test_image_model_response_metadata_with_timestamp() {
        let timestamp = SystemTime::now();
        let metadata = ImageModelResponseMetadata::with_timestamp("dall-e-3", timestamp);

        assert_eq!(metadata.model_id, "dall-e-3");
        assert_eq!(metadata.timestamp, timestamp);
    }

    #[test]
    fn test_image_model_response_metadata_with_headers() {
        let headers = HashMap::from([
            ("x-request-id".to_string(), "123".to_string()),
            ("content-type".to_string(), "application/json".to_string()),
        ]);

        let metadata = ImageModelResponseMetadata::new("dall-e-3").with_headers(headers.clone());

        assert!(metadata.headers.is_some());
        assert_eq!(metadata.headers.unwrap(), headers);
    }

    #[test]
    fn test_generate_image_result_new() {
        let images = vec![
            GeneratedFile::from_base64("base64data1", "image/png"),
            GeneratedFile::from_base64("base64data2", "image/png"),
        ];
        let responses = vec![ImageModelResponseMetadata::new("dall-e-3")];

        let result = GenerateImageResult::new(images.clone(), responses);

        assert_eq!(result.images.len(), 2);
        assert_eq!(result.image, images[0]);
        assert!(result.warnings.is_empty());
        assert_eq!(result.responses.len(), 1);
        assert!(result.provider_metadata.is_none());
    }

    #[test]
    #[should_panic(expected = "images cannot be empty")]
    fn test_generate_image_result_new_panics_on_empty() {
        let images = vec![];
        let responses = vec![ImageModelResponseMetadata::new("dall-e-3")];

        GenerateImageResult::new(images, responses);
    }

    #[test]
    fn test_generate_image_result_with_warnings() {
        let images = vec![GeneratedFile::from_base64("base64data", "image/png")];
        let responses = vec![ImageModelResponseMetadata::new("dall-e-3")];

        let warnings = vec![
            ImageModelCallWarning::unsupported_setting("aspectRatio"),
            ImageModelCallWarning::unsupported_setting("seed"),
        ];

        let result = GenerateImageResult::new(images, responses).with_warnings(warnings.clone());

        assert_eq!(result.warnings.len(), 2);
    }

    #[test]
    fn test_generate_image_result_with_warning() {
        let images = vec![GeneratedFile::from_base64("base64data", "image/png")];
        let responses = vec![ImageModelResponseMetadata::new("dall-e-3")];

        let warning = ImageModelCallWarning::unsupported_setting("aspectRatio");

        let result = GenerateImageResult::new(images, responses).with_warning(warning);

        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_generate_image_result_with_provider_metadata() {
        let images = vec![GeneratedFile::from_base64("base64data", "image/png")];
        let responses = vec![ImageModelResponseMetadata::new("dall-e-3")];

        let mut metadata = HashMap::new();
        let mut openai_data = HashMap::new();
        openai_data.insert(
            "revised_prompt".to_string(),
            serde_json::json!("A revised prompt"),
        );
        metadata.insert("openai".to_string(), openai_data);

        let result =
            GenerateImageResult::new(images, responses).with_provider_metadata(metadata.clone());

        assert!(result.provider_metadata.is_some());
        assert_eq!(result.provider_metadata.unwrap(), metadata);
    }

    #[test]
    fn test_generate_image_result_builder_pattern() {
        let images = vec![
            GeneratedFile::from_base64("base64data1", "image/png"),
            GeneratedFile::from_base64("base64data2", "image/png"),
        ];
        let responses = vec![ImageModelResponseMetadata::new("dall-e-3")];

        let warnings = vec![ImageModelCallWarning::unsupported_setting("seed")];

        let mut metadata = HashMap::new();
        let mut openai_data = HashMap::new();
        openai_data.insert("test".to_string(), serde_json::json!("value"));
        metadata.insert("openai".to_string(), openai_data);

        let result = GenerateImageResult::new(images.clone(), responses)
            .with_warnings(warnings)
            .with_provider_metadata(metadata.clone());

        assert_eq!(result.images.len(), 2);
        assert_eq!(result.image, images[0]);
        assert_eq!(result.warnings.len(), 1);
        assert!(result.provider_metadata.is_some());
    }

    #[test]
    fn test_image_data_to_generated_file_base64() {
        let image_data = ImageData::Base64("SGVsbG8gV29ybGQh".to_string());
        let file = image_data_to_generated_file(image_data, "image/png");

        assert_eq!(file.media_type, "image/png");
        assert_eq!(file.base64(), "SGVsbG8gV29ybGQh");
    }

    #[test]
    fn test_image_data_to_generated_file_binary() {
        let image_data = ImageData::Binary(b"Hello World!".to_vec());
        let file = image_data_to_generated_file(image_data, "image/jpeg");

        assert_eq!(file.media_type, "image/jpeg");
        assert_eq!(file.bytes(), b"Hello World!");
    }

    #[test]
    fn test_generate_image_result_serialization() {
        let images = vec![GeneratedFile::from_base64("base64data", "image/png")];
        let responses = vec![ImageModelResponseMetadata::new("dall-e-3")];

        let result = GenerateImageResult::new(images, responses);
        let json = serde_json::to_value(&result).unwrap();

        assert!(json.get("image").is_some());
        assert!(json.get("images").is_some());
        assert!(json.get("responses").is_some());
        // warnings is empty so it should be omitted
        assert!(json.get("warnings").is_none());
        // providerMetadata is None so it should be omitted
        assert!(json.get("providerMetadata").is_none());
    }

    #[test]
    fn test_generate_image_result_serialization_with_warnings() {
        let images = vec![GeneratedFile::from_base64("base64data", "image/png")];
        let responses = vec![ImageModelResponseMetadata::new("dall-e-3")];
        let warnings = vec![ImageModelCallWarning::unsupported_setting("seed")];

        let result = GenerateImageResult::new(images, responses).with_warnings(warnings);
        let json = serde_json::to_value(&result).unwrap();

        assert!(json.get("warnings").is_some());
    }

    #[test]
    fn test_generate_image_result_round_trip() {
        // Test serialization round-trip
        let images = vec![GeneratedFile::from_base64("base64data", "image/png")];
        let responses = vec![ImageModelResponseMetadata::new("dall-e-3")];

        let result = GenerateImageResult::new(images, responses);

        // Serialize and deserialize
        let json = serde_json::to_value(&result).unwrap();
        let deserialized: GenerateImageResult = serde_json::from_value(json).unwrap();

        assert_eq!(deserialized.images.len(), 1);
        assert_eq!(deserialized.responses.len(), 1);
        assert_eq!(deserialized.responses[0].model_id, "dall-e-3");
        assert!(deserialized.warnings.is_empty());
    }

    #[test]
    fn test_response_metadata_serialization() {
        let metadata = ImageModelResponseMetadata::new("dall-e-3");
        let json = serde_json::to_value(&metadata).unwrap();

        assert!(json.get("timestamp").is_some());
        assert!(json.get("modelId").is_some());
        assert_eq!(json.get("modelId").unwrap(), "dall-e-3");
    }

    #[test]
    fn test_multiple_warnings() {
        let images = vec![GeneratedFile::from_base64("base64data", "image/png")];
        let responses = vec![ImageModelResponseMetadata::new("dall-e-3")];

        let result = GenerateImageResult::new(images, responses)
            .with_warning(ImageModelCallWarning::unsupported_setting("aspectRatio"))
            .with_warning(ImageModelCallWarning::unsupported_setting("seed"))
            .with_warning(ImageModelCallWarning::other("Custom warning"));

        assert_eq!(result.warnings.len(), 3);
    }

    #[test]
    fn test_multiple_responses() {
        let images = vec![
            GeneratedFile::from_base64("base64data1", "image/png"),
            GeneratedFile::from_base64("base64data2", "image/png"),
        ];
        let responses = vec![
            ImageModelResponseMetadata::new("dall-e-3"),
            ImageModelResponseMetadata::new("dall-e-3"),
        ];

        let result = GenerateImageResult::new(images, responses);

        assert_eq!(result.responses.len(), 2);
    }

    #[test]
    fn test_first_image_reference() {
        let image1 = GeneratedFile::from_base64("base64data1", "image/png");
        let image2 = GeneratedFile::from_base64("base64data2", "image/png");
        let images = vec![image1.clone(), image2];
        let responses = vec![ImageModelResponseMetadata::new("dall-e-3")];

        let result = GenerateImageResult::new(images, responses);

        // The image field should reference the first image
        assert_eq!(result.image, image1);
    }
}
