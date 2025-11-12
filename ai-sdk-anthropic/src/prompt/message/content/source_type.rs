use serde::{Deserialize, Serialize};

/// Anthropic content source types for images and documents.
///
/// Represents the different ways content can be provided to the Anthropic API:
/// - Base64-encoded data with media type
/// - URL reference
/// - Plain text data
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::prompt::message::content::source_type::AnthropicContentSource;
///
/// // Base64 image source
/// let base64_source = AnthropicContentSource::base64(
///     "image/png",
///     "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg=="
/// );
///
/// // URL source
/// let url_source = AnthropicContentSource::url("https://example.com/image.png");
///
/// // Text source
/// let text_source = AnthropicContentSource::text("This is plain text content");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AnthropicContentSource {
    /// Base64-encoded content with media type
    #[serde(rename = "base64")]
    Base64 {
        /// The media type (MIME type) of the content
        media_type: String,
        /// Base64-encoded data
        data: String,
    },

    /// URL reference to content
    #[serde(rename = "url")]
    Url {
        /// The URL of the content
        url: String,
    },

    /// Plain text content
    #[serde(rename = "text")]
    Text {
        /// The media type (always "text/plain")
        media_type: String,
        /// The text data
        data: String,
    },
}

impl AnthropicContentSource {
    /// Creates a new Base64 content source.
    ///
    /// # Arguments
    ///
    /// * `media_type` - The MIME type of the content (e.g., "image/png", "application/pdf")
    /// * `data` - The base64-encoded data
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::source_type::AnthropicContentSource;
    ///
    /// let source = AnthropicContentSource::base64(
    ///     "image/jpeg",
    ///     "base64encodeddata..."
    /// );
    /// ```
    pub fn base64(media_type: impl Into<String>, data: impl Into<String>) -> Self {
        Self::Base64 {
            media_type: media_type.into(),
            data: data.into(),
        }
    }

    /// Creates a new URL content source.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the content
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::source_type::AnthropicContentSource;
    ///
    /// let source = AnthropicContentSource::url("https://example.com/document.pdf");
    /// ```
    pub fn url(url: impl Into<String>) -> Self {
        Self::Url { url: url.into() }
    }

    /// Creates a new text content source.
    ///
    /// The media type is automatically set to "text/plain".
    ///
    /// # Arguments
    ///
    /// * `data` - The text data
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::source_type::AnthropicContentSource;
    ///
    /// let source = AnthropicContentSource::text("This is plain text content");
    /// ```
    pub fn text(data: impl Into<String>) -> Self {
        Self::Text {
            media_type: "text/plain".to_string(),
            data: data.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_base64_constructor() {
        let source = AnthropicContentSource::base64("image/png", "base64data");

        match source {
            AnthropicContentSource::Base64 { media_type, data } => {
                assert_eq!(media_type, "image/png");
                assert_eq!(data, "base64data");
            }
            _ => panic!("Expected Base64 variant"),
        }
    }

    #[test]
    fn test_url_constructor() {
        let source = AnthropicContentSource::url("https://example.com/image.jpg");

        match source {
            AnthropicContentSource::Url { url } => {
                assert_eq!(url, "https://example.com/image.jpg");
            }
            _ => panic!("Expected Url variant"),
        }
    }

    #[test]
    fn test_text_constructor() {
        let source = AnthropicContentSource::text("Hello, world!");

        match source {
            AnthropicContentSource::Text { media_type, data } => {
                assert_eq!(media_type, "text/plain");
                assert_eq!(data, "Hello, world!");
            }
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_serialize_base64() {
        let source = AnthropicContentSource::base64("image/jpeg", "abc123");
        let json = serde_json::to_value(&source).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "base64",
                "media_type": "image/jpeg",
                "data": "abc123"
            })
        );
    }

    #[test]
    fn test_serialize_url() {
        let source = AnthropicContentSource::url("https://example.com/file.pdf");
        let json = serde_json::to_value(&source).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "url",
                "url": "https://example.com/file.pdf"
            })
        );
    }

    #[test]
    fn test_serialize_text() {
        let source = AnthropicContentSource::text("Plain text content");
        let json = serde_json::to_value(&source).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "text",
                "media_type": "text/plain",
                "data": "Plain text content"
            })
        );
    }

    #[test]
    fn test_deserialize_base64() {
        let json = json!({
            "type": "base64",
            "media_type": "application/pdf",
            "data": "pdfdata123"
        });

        let source: AnthropicContentSource = serde_json::from_value(json).unwrap();

        match source {
            AnthropicContentSource::Base64 { media_type, data } => {
                assert_eq!(media_type, "application/pdf");
                assert_eq!(data, "pdfdata123");
            }
            _ => panic!("Expected Base64 variant"),
        }
    }

    #[test]
    fn test_deserialize_url() {
        let json = json!({
            "type": "url",
            "url": "https://cdn.example.com/image.png"
        });

        let source: AnthropicContentSource = serde_json::from_value(json).unwrap();

        match source {
            AnthropicContentSource::Url { url } => {
                assert_eq!(url, "https://cdn.example.com/image.png");
            }
            _ => panic!("Expected Url variant"),
        }
    }

    #[test]
    fn test_deserialize_text() {
        let json = json!({
            "type": "text",
            "media_type": "text/plain",
            "data": "Text content here"
        });

        let source: AnthropicContentSource = serde_json::from_value(json).unwrap();

        match source {
            AnthropicContentSource::Text { media_type, data } => {
                assert_eq!(media_type, "text/plain");
                assert_eq!(data, "Text content here");
            }
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_equality() {
        let source1 = AnthropicContentSource::base64("image/png", "data");
        let source2 = AnthropicContentSource::base64("image/png", "data");
        let source3 = AnthropicContentSource::base64("image/jpeg", "data");

        assert_eq!(source1, source2);
        assert_ne!(source1, source3);
    }

    #[test]
    fn test_clone() {
        let source1 = AnthropicContentSource::url("https://example.com/file");
        let source2 = source1.clone();

        assert_eq!(source1, source2);
    }

    #[test]
    fn test_different_variants_not_equal() {
        let base64 = AnthropicContentSource::base64("text/plain", "data");
        let text = AnthropicContentSource::text("data");
        let url = AnthropicContentSource::url("data");

        assert_ne!(base64, text);
        assert_ne!(base64, url);
        assert_ne!(text, url);
    }

    #[test]
    fn test_roundtrip_base64() {
        let original = AnthropicContentSource::base64("image/webp", "webpdata123");
        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicContentSource = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_roundtrip_url() {
        let original = AnthropicContentSource::url("https://example.com/file.txt");
        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicContentSource = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_roundtrip_text() {
        let original = AnthropicContentSource::text("Sample text with unicode: 你好 🌍");
        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicContentSource = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_base64_with_various_media_types() {
        let media_types = vec![
            "image/png",
            "image/jpeg",
            "image/gif",
            "image/webp",
            "application/pdf",
            "video/mp4",
            "audio/mpeg",
        ];

        for media_type in media_types {
            let source = AnthropicContentSource::base64(media_type, "data");
            match source {
                AnthropicContentSource::Base64 {
                    media_type: mt,
                    data: _,
                } => {
                    assert_eq!(mt, media_type);
                }
                _ => panic!("Expected Base64 variant"),
            }
        }
    }

    #[test]
    fn test_empty_strings() {
        let base64 = AnthropicContentSource::base64("", "");
        let url = AnthropicContentSource::url("");
        let text = AnthropicContentSource::text("");

        // Should all be created successfully
        match base64 {
            AnthropicContentSource::Base64 { media_type, data } => {
                assert_eq!(media_type, "");
                assert_eq!(data, "");
            }
            _ => panic!("Expected Base64 variant"),
        }

        match url {
            AnthropicContentSource::Url { url } => {
                assert_eq!(url, "");
            }
            _ => panic!("Expected Url variant"),
        }

        match text {
            AnthropicContentSource::Text { media_type, data } => {
                assert_eq!(media_type, "text/plain");
                assert_eq!(data, "");
            }
            _ => panic!("Expected Text variant"),
        }
    }
}
