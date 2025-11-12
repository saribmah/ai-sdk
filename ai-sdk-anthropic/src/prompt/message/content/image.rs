use serde::{Deserialize, Serialize};

use crate::prompt::message::cache_control::AnthropicCacheControl;
use crate::prompt::message::content::source_type::AnthropicContentSource;

/// Anthropic image content block.
///
/// Represents an image content block in an Anthropic message. Images can be provided
/// as base64-encoded data or as URLs.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::prompt::message::content::image::AnthropicImageContent;
/// use ai_sdk_anthropic::prompt::message::content::source_type::AnthropicContentSource;
///
/// // Create an image from a URL
/// let image = AnthropicImageContent::new(
///     AnthropicContentSource::url("https://example.com/image.png")
/// );
///
/// // Create an image from base64 data
/// let base64_image = AnthropicImageContent::new(
///     AnthropicContentSource::base64("image/jpeg", "base64data...")
/// );
///
/// // Create an image with cache control
/// use ai_sdk_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
///
/// let cached_image = AnthropicImageContent::new(
///     AnthropicContentSource::url("https://example.com/large-image.png")
/// ).with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicImageContent {
    /// The type of content (always "image")
    #[serde(rename = "type")]
    pub content_type: String,

    /// The image source (base64, URL, or text)
    pub source: AnthropicContentSource,

    /// Optional cache control configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}

impl AnthropicImageContent {
    /// Creates a new image content block.
    ///
    /// # Arguments
    ///
    /// * `source` - The image source (base64, URL, or text)
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::image::AnthropicImageContent;
    /// use ai_sdk_anthropic::prompt::message::content::source_type::AnthropicContentSource;
    ///
    /// let image = AnthropicImageContent::new(
    ///     AnthropicContentSource::url("https://example.com/photo.jpg")
    /// );
    /// ```
    pub fn new(source: AnthropicContentSource) -> Self {
        Self {
            content_type: "image".to_string(),
            source,
            cache_control: None,
        }
    }

    /// Creates a new image content block from a URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The image URL
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::image::AnthropicImageContent;
    ///
    /// let image = AnthropicImageContent::from_url("https://example.com/image.png");
    /// ```
    pub fn from_url(url: impl Into<String>) -> Self {
        Self::new(AnthropicContentSource::url(url))
    }

    /// Creates a new image content block from base64-encoded data.
    ///
    /// # Arguments
    ///
    /// * `media_type` - The MIME type (e.g., "image/png", "image/jpeg")
    /// * `data` - The base64-encoded image data
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::image::AnthropicImageContent;
    ///
    /// let image = AnthropicImageContent::from_base64("image/png", "iVBORw0KG...");
    /// ```
    pub fn from_base64(media_type: impl Into<String>, data: impl Into<String>) -> Self {
        Self::new(AnthropicContentSource::base64(media_type, data))
    }

    /// Sets the cache control for this image content block.
    ///
    /// # Arguments
    ///
    /// * `cache_control` - The cache control configuration
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::image::AnthropicImageContent;
    /// use ai_sdk_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    ///
    /// let image = AnthropicImageContent::from_url("https://example.com/image.png")
    ///     .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes));
    /// ```
    pub fn with_cache_control(mut self, cache_control: AnthropicCacheControl) -> Self {
        self.cache_control = Some(cache_control);
        self
    }

    /// Removes cache control from this image content block.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::image::AnthropicImageContent;
    /// use ai_sdk_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    ///
    /// let image = AnthropicImageContent::from_url("https://example.com/image.png")
    ///     .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour))
    ///     .without_cache_control();
    ///
    /// assert!(image.cache_control.is_none());
    /// ```
    pub fn without_cache_control(mut self) -> Self {
        self.cache_control = None;
        self
    }

    /// Updates the source of this image content block.
    ///
    /// # Arguments
    ///
    /// * `source` - The new image source
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::image::AnthropicImageContent;
    /// use ai_sdk_anthropic::prompt::message::content::source_type::AnthropicContentSource;
    ///
    /// let image = AnthropicImageContent::from_url("https://example.com/old.png")
    ///     .with_source(AnthropicContentSource::url("https://example.com/new.png"));
    /// ```
    pub fn with_source(mut self, source: AnthropicContentSource) -> Self {
        self.source = source;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    use serde_json::json;

    #[test]
    fn test_new_with_url_source() {
        let source = AnthropicContentSource::url("https://example.com/image.png");
        let image = AnthropicImageContent::new(source);

        assert_eq!(image.content_type, "image");
        assert!(matches!(image.source, AnthropicContentSource::Url { .. }));
        assert!(image.cache_control.is_none());
    }

    #[test]
    fn test_new_with_base64_source() {
        let source = AnthropicContentSource::base64("image/jpeg", "base64data");
        let image = AnthropicImageContent::new(source);

        assert_eq!(image.content_type, "image");
        assert!(matches!(
            image.source,
            AnthropicContentSource::Base64 { .. }
        ));
    }

    #[test]
    fn test_from_url() {
        let image = AnthropicImageContent::from_url("https://example.com/photo.jpg");

        assert_eq!(image.content_type, "image");
        match image.source {
            AnthropicContentSource::Url { url } => {
                assert_eq!(url, "https://example.com/photo.jpg");
            }
            _ => panic!("Expected Url source"),
        }
    }

    #[test]
    fn test_from_base64() {
        let image = AnthropicImageContent::from_base64("image/png", "pngdata123");

        assert_eq!(image.content_type, "image");
        match image.source {
            AnthropicContentSource::Base64 { media_type, data } => {
                assert_eq!(media_type, "image/png");
                assert_eq!(data, "pngdata123");
            }
            _ => panic!("Expected Base64 source"),
        }
    }

    #[test]
    fn test_with_cache_control() {
        let cache_control = AnthropicCacheControl::new(CacheTTL::FiveMinutes);
        let image = AnthropicImageContent::from_url("https://example.com/image.png")
            .with_cache_control(cache_control.clone());

        assert_eq!(image.cache_control, Some(cache_control));
    }

    #[test]
    fn test_without_cache_control() {
        let image = AnthropicImageContent::from_url("https://example.com/image.png")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour))
            .without_cache_control();

        assert!(image.cache_control.is_none());
    }

    #[test]
    fn test_with_source() {
        let image = AnthropicImageContent::from_url("https://example.com/old.png")
            .with_source(AnthropicContentSource::url("https://example.com/new.png"));

        match image.source {
            AnthropicContentSource::Url { url } => {
                assert_eq!(url, "https://example.com/new.png");
            }
            _ => panic!("Expected Url source"),
        }
    }

    #[test]
    fn test_serialize_url_without_cache() {
        let image = AnthropicImageContent::from_url("https://example.com/image.png");
        let json = serde_json::to_value(&image).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "image",
                "source": {
                    "type": "url",
                    "url": "https://example.com/image.png"
                }
            })
        );
    }

    #[test]
    fn test_serialize_base64_without_cache() {
        let image = AnthropicImageContent::from_base64("image/jpeg", "jpegdata");
        let json = serde_json::to_value(&image).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "image",
                "source": {
                    "type": "base64",
                    "media_type": "image/jpeg",
                    "data": "jpegdata"
                }
            })
        );
    }

    #[test]
    fn test_serialize_with_cache() {
        let image = AnthropicImageContent::from_url("https://example.com/cached.png")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let json = serde_json::to_value(&image).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "image",
                "source": {
                    "type": "url",
                    "url": "https://example.com/cached.png"
                },
                "cache_control": {
                    "type": "ephemeral",
                    "ttl": "1h"
                }
            })
        );
    }

    #[test]
    fn test_deserialize_url() {
        let json = json!({
            "type": "image",
            "source": {
                "type": "url",
                "url": "https://example.com/image.png"
            }
        });

        let image: AnthropicImageContent = serde_json::from_value(json).unwrap();

        assert_eq!(image.content_type, "image");
        match image.source {
            AnthropicContentSource::Url { url } => {
                assert_eq!(url, "https://example.com/image.png");
            }
            _ => panic!("Expected Url source"),
        }
        assert!(image.cache_control.is_none());
    }

    #[test]
    fn test_deserialize_base64() {
        let json = json!({
            "type": "image",
            "source": {
                "type": "base64",
                "media_type": "image/webp",
                "data": "webpdata"
            }
        });

        let image: AnthropicImageContent = serde_json::from_value(json).unwrap();

        match image.source {
            AnthropicContentSource::Base64 { media_type, data } => {
                assert_eq!(media_type, "image/webp");
                assert_eq!(data, "webpdata");
            }
            _ => panic!("Expected Base64 source"),
        }
    }

    #[test]
    fn test_deserialize_with_cache() {
        let json = json!({
            "type": "image",
            "source": {
                "type": "url",
                "url": "https://example.com/image.png"
            },
            "cache_control": {
                "type": "ephemeral",
                "ttl": "5m"
            }
        });

        let image: AnthropicImageContent = serde_json::from_value(json).unwrap();

        assert_eq!(
            image.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
        );
    }

    #[test]
    fn test_equality() {
        let image1 = AnthropicImageContent::from_url("https://example.com/same.png");
        let image2 = AnthropicImageContent::from_url("https://example.com/same.png");
        let image3 = AnthropicImageContent::from_url("https://example.com/different.png");

        assert_eq!(image1, image2);
        assert_ne!(image1, image3);
    }

    #[test]
    fn test_clone() {
        let image1 = AnthropicImageContent::from_url("https://example.com/image.png")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let image2 = image1.clone();

        assert_eq!(image1, image2);
    }

    #[test]
    fn test_builder_chaining() {
        let image = AnthropicImageContent::from_url("https://example.com/1.png")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
            .without_cache_control()
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour))
            .with_source(AnthropicContentSource::url("https://example.com/2.png"));

        match image.source {
            AnthropicContentSource::Url { url } => {
                assert_eq!(url, "https://example.com/2.png");
            }
            _ => panic!("Expected Url source"),
        }
        assert_eq!(
            image.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::OneHour))
        );
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original = AnthropicImageContent::from_base64("image/png", "base64data")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicImageContent = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }
}
