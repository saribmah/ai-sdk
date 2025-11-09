/// Result type for image generation operations.
pub mod result;

pub use result::{GenerateImageResult, ImageModelResponseMetadata};

use crate::error::AISDKError;
use crate::generate_text::{GeneratedFile, prepare_retries};
use ai_sdk_provider::image_model::call_options::{AspectRatio, ImageModelCallOptions, ImageSize};
use ai_sdk_provider::image_model::call_warning::ImageModelCallWarning;
use ai_sdk_provider::image_model::{ImageData, ImageModel, ImageModelProviderMetadata};
use ai_sdk_provider::shared::headers::SharedHeaders;
use ai_sdk_provider::shared::provider_options::SharedProviderOptions;
use result::image_data_to_generated_file;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Builder for generating images using an image model.
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::GenerateImage;
///
/// let result = GenerateImage::new(model, "A beautiful sunset over mountains".to_string())
///     .n(2)
///     .size("1024x1024")
///     .max_retries(3)
///     .execute()
///     .await?;
///
/// println!("Generated {} images", result.images.len());
/// ```
pub struct GenerateImage {
    model: Arc<dyn ImageModel>,
    prompt: String,
    n: Option<u32>,
    max_images_per_call: Option<u32>,
    size: Option<String>,
    aspect_ratio: Option<String>,
    seed: Option<u32>,
    provider_options: Option<SharedProviderOptions>,
    max_retries: Option<u32>,
    abort_signal: Option<CancellationToken>,
    headers: Option<SharedHeaders>,
}

impl GenerateImage {
    /// Create a new GenerateImage builder.
    ///
    /// # Arguments
    ///
    /// * `model` - The image model to use
    /// * `prompt` - The prompt that should be used to generate the image
    pub fn new(model: Arc<dyn ImageModel>, prompt: String) -> Self {
        Self {
            model,
            prompt,
            n: None,
            max_images_per_call: None,
            size: None,
            aspect_ratio: None,
            seed: None,
            provider_options: None,
            max_retries: None,
            abort_signal: None,
            headers: None,
        }
    }

    /// Set the number of images to generate.
    ///
    /// # Arguments
    ///
    /// * `n` - Number of images to generate. Default: 1.
    pub fn n(mut self, n: u32) -> Self {
        self.n = Some(n);
        self
    }

    /// Set the maximum number of images per API call.
    ///
    /// # Arguments
    ///
    /// * `max_images_per_call` - Maximum number of images per call
    pub fn max_images_per_call(mut self, max_images_per_call: u32) -> Self {
        self.max_images_per_call = Some(max_images_per_call);
        self
    }

    /// Set the size of the images to generate.
    ///
    /// # Arguments
    ///
    /// * `size` - Size in format `{width}x{height}` (e.g., "1024x1024")
    pub fn size(mut self, size: impl Into<String>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Set the aspect ratio of the images to generate.
    ///
    /// # Arguments
    ///
    /// * `aspect_ratio` - Aspect ratio in format `{width}:{height}` (e.g., "16:9")
    pub fn aspect_ratio(mut self, aspect_ratio: impl Into<String>) -> Self {
        self.aspect_ratio = Some(aspect_ratio.into());
        self
    }

    /// Set the seed for the image generation.
    ///
    /// # Arguments
    ///
    /// * `seed` - Seed for deterministic generation
    pub fn seed(mut self, seed: u32) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set additional provider-specific options.
    ///
    /// # Arguments
    ///
    /// * `provider_options` - Additional provider-specific options
    pub fn provider_options(mut self, provider_options: SharedProviderOptions) -> Self {
        self.provider_options = Some(provider_options);
        self
    }

    /// Set the maximum number of retries.
    ///
    /// # Arguments
    ///
    /// * `max_retries` - Maximum number of retries. Set to 0 to disable retries. Default: 2.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    /// Set an abort signal that can be used to cancel the call.
    ///
    /// # Arguments
    ///
    /// * `abort_signal` - An optional abort signal
    pub fn abort_signal(mut self, abort_signal: CancellationToken) -> Self {
        self.abort_signal = Some(abort_signal);
        self
    }

    /// Set additional HTTP headers to be sent with the request.
    ///
    /// # Arguments
    ///
    /// * `headers` - Additional HTTP headers
    pub fn headers(mut self, headers: SharedHeaders) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Execute the image generation operation.
    ///
    /// # Returns
    ///
    /// A result object that contains the generated images.
    pub async fn execute(self) -> Result<GenerateImageResult, AISDKError> {
        // Check specification version
        if self.model.specification_version() != "v3" {
            return Err(AISDKError::model_error(format!(
                "Unsupported model version: {}. Provider: {}, Model ID: {}",
                self.model.specification_version(),
                self.model.provider(),
                self.model.model_id()
            )));
        }

        let n = self.n.unwrap_or(1);

        // Parse size and aspect ratio if provided
        let parsed_size = if let Some(ref size_str) = self.size {
            Some(
                ImageSize::from_str(size_str)
                    .map_err(|e| AISDKError::invalid_argument("size", size_str, e))?,
            )
        } else {
            None
        };

        let parsed_aspect_ratio =
            if let Some(ref aspect_ratio_str) = self.aspect_ratio {
                Some(AspectRatio::from_str(aspect_ratio_str).map_err(|e| {
                    AISDKError::invalid_argument("aspect_ratio", aspect_ratio_str, e)
                })?)
            } else {
                None
            };

        // Add user agent to headers
        let headers_with_user_agent =
            add_user_agent_suffix(self.headers, format!("ai/{}", VERSION));

        // Prepare retry configuration
        let retry_config = prepare_retries(self.max_retries, self.abort_signal.clone())?;

        // Get max images per call from model or use provided value
        let max_images_per_call_with_default = if let Some(max) = self.max_images_per_call {
            max
        } else {
            self.model
                .max_images_per_call(self.model.model_id())
                .await
                .unwrap_or(1) as u32
        };

        // Calculate how many calls we need to make
        let call_count = n.div_ceil(max_images_per_call_with_default);

        // Calculate how many images per call
        let mut call_image_counts = Vec::new();
        for i in 0..call_count {
            if i < call_count - 1 {
                call_image_counts.push(max_images_per_call_with_default);
            } else {
                let remainder = n % max_images_per_call_with_default;
                call_image_counts.push(if remainder == 0 {
                    max_images_per_call_with_default
                } else {
                    remainder
                });
            }
        }

        // Make parallel calls to the model
        let mut futures_vec = Vec::new();
        let seed = self.seed;

        for call_image_count in call_image_counts {
            let model = self.model.clone();
            let prompt = self.prompt.clone();
            let headers = headers_with_user_agent.clone();
            let provider_options = self.provider_options.clone();
            let max_retries = retry_config.max_retries;
            let abort_signal_clone = self.abort_signal.clone();

            let future = async move {
                let retry_config = crate::generate_text::RetryConfig {
                    max_retries,
                    abort_signal: abort_signal_clone,
                };

                retry_config
                    .execute_with_boxed_error(move || {
                        let model = model.clone();
                        let prompt = prompt.clone();
                        let headers = headers.clone();
                        let provider_options = provider_options.clone();
                        async move {
                            let options = ImageModelCallOptions {
                                prompt,
                                n: call_image_count,
                                abort_signal: None,
                                headers,
                                size: parsed_size,
                                aspect_ratio: parsed_aspect_ratio,
                                seed,
                                provider_options,
                            };
                            model.do_generate(options).await
                        }
                    })
                    .await
            };

            futures_vec.push(future);
        }

        // Wait for all futures to complete
        let results = futures::future::try_join_all(futures_vec).await?;

        // Collect result images, warnings, and response metadata
        let mut all_images: Vec<GeneratedFile> = Vec::new();
        let mut all_warnings: Vec<ImageModelCallWarning> = Vec::new();
        let mut all_responses: Vec<ImageModelResponseMetadata> = Vec::new();
        let mut combined_provider_metadata: Option<ImageModelProviderMetadata> = None;

        for result in results {
            // Convert ImageData to GeneratedFile
            for image_data in result.images {
                let media_type = detect_image_media_type(&image_data);
                all_images.push(image_data_to_generated_file(image_data, media_type));
            }

            // Collect warnings
            all_warnings.extend(result.warnings);

            // Collect responses
            all_responses.push(
                ImageModelResponseMetadata::new(result.response.model_id.clone())
                    .with_headers(result.response.headers.clone().unwrap_or_default()),
            );

            // Merge provider metadata
            if let Some(metadata) = result.provider_metadata {
                if let Some(ref mut combined) = combined_provider_metadata {
                    // Merge metadata from different provider calls
                    for (provider_name, provider_data) in metadata {
                        combined
                            .entry(provider_name.clone())
                            .or_insert_with(HashMap::new)
                            .extend(provider_data);
                    }
                } else {
                    combined_provider_metadata = Some(metadata);
                }
            }
        }

        // Check if we got any images
        if all_images.is_empty() {
            return Err(AISDKError::no_image_generated_with_responses(
                "No images were generated by the model",
                all_responses,
            ));
        }

        // Build the result
        let mut result = GenerateImageResult::new(all_images, all_responses);

        if !all_warnings.is_empty() {
            result = result.with_warnings(all_warnings);
        }

        if let Some(metadata) = combined_provider_metadata {
            result = result.with_provider_metadata(metadata);
        }

        Ok(result)
    }
}

/// Detect media type from image data.
///
/// Detects the media type based on the magic bytes at the start of the data.
fn detect_image_media_type(image_data: &ImageData) -> String {
    match image_data {
        ImageData::Base64(base64_str) => {
            // Decode base64 and check magic bytes
            use base64::Engine;
            if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(base64_str) {
                detect_media_type_from_bytes(&bytes)
            } else {
                "image/png".to_string() // Default fallback
            }
        }
        ImageData::Binary(bytes) => detect_media_type_from_bytes(bytes),
    }
}

/// Detect media type from raw bytes based on magic bytes.
fn detect_media_type_from_bytes(bytes: &[u8]) -> String {
    if bytes.len() < 4 {
        return "image/png".to_string();
    }

    // Check PNG signature
    if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
        return "image/png".to_string();
    }

    // Check JPEG signature
    if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return "image/jpeg".to_string();
    }

    // Check WebP signature
    if bytes.len() >= 12
        && bytes[0..4] == [0x52, 0x49, 0x46, 0x46]
        && bytes[8..12] == [0x57, 0x45, 0x42, 0x50]
    {
        return "image/webp".to_string();
    }

    // Check GIF signature
    if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        return "image/gif".to_string();
    }

    // Default to PNG
    "image/png".to_string()
}

/// Add a user agent suffix to headers.
fn add_user_agent_suffix(headers: Option<SharedHeaders>, suffix: String) -> Option<SharedHeaders> {
    let mut headers = headers.unwrap_or_default();

    // Get existing user agent or use empty string
    let existing_user_agent = headers.get("user-agent").cloned().unwrap_or_default();

    // Add suffix
    let new_user_agent = if existing_user_agent.is_empty() {
        suffix
    } else {
        format!("{} {}", existing_user_agent, suffix)
    };

    headers.insert("user-agent".to_string(), new_user_agent);
    Some(headers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_media_type_png() {
        let png_signature = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
        assert_eq!(detect_media_type_from_bytes(&png_signature), "image/png");
    }

    #[test]
    fn test_detect_media_type_jpeg() {
        let jpeg_signature = vec![0xFF, 0xD8, 0xFF, 0xE0];
        assert_eq!(detect_media_type_from_bytes(&jpeg_signature), "image/jpeg");
    }

    #[test]
    fn test_detect_media_type_webp() {
        let webp_signature = vec![
            0x52, 0x49, 0x46, 0x46, // "RIFF"
            0x00, 0x00, 0x00, 0x00, // file size (placeholder)
            0x57, 0x45, 0x42, 0x50, // "WEBP"
        ];
        assert_eq!(detect_media_type_from_bytes(&webp_signature), "image/webp");
    }

    #[test]
    fn test_detect_media_type_gif() {
        let gif87_signature = b"GIF87a".to_vec();
        assert_eq!(detect_media_type_from_bytes(&gif87_signature), "image/gif");

        let gif89_signature = b"GIF89a".to_vec();
        assert_eq!(detect_media_type_from_bytes(&gif89_signature), "image/gif");
    }

    #[test]
    fn test_detect_media_type_unknown() {
        let unknown = vec![0x00, 0x01, 0x02, 0x03];
        assert_eq!(detect_media_type_from_bytes(&unknown), "image/png");
    }

    #[test]
    fn test_detect_media_type_short() {
        let short = vec![0x00];
        assert_eq!(detect_media_type_from_bytes(&short), "image/png");
    }

    #[test]
    fn test_add_user_agent_suffix_no_existing() {
        let headers = add_user_agent_suffix(None, "ai/1.0.0".to_string());
        assert!(headers.is_some());
        let headers = headers.unwrap();
        assert_eq!(headers.get("user-agent"), Some(&"ai/1.0.0".to_string()));
    }

    #[test]
    fn test_add_user_agent_suffix_with_existing() {
        let mut existing = HashMap::new();
        existing.insert("user-agent".to_string(), "custom/1.0".to_string());

        let headers = add_user_agent_suffix(Some(existing), "ai/1.0.0".to_string());
        assert!(headers.is_some());
        let headers = headers.unwrap();
        assert_eq!(
            headers.get("user-agent"),
            Some(&"custom/1.0 ai/1.0.0".to_string())
        );
    }
}
