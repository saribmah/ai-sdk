use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Image data storage format.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "format", rename_all = "lowercase")]
pub enum ImageData {
    /// Base64-encoded image data
    Base64 {
        /// Base64-encoded string
        data: String,
    },
    /// Raw binary image data
    Binary {
        /// Binary data
        data: Vec<u8>,
    },
    /// URL pointing to the image
    Url {
        /// Image URL
        url: String,
    },
}

/// Image dimensions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImageDimensions {
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
}

/// An image content part within a message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImagePart {
    /// Unique identifier for this part
    pub id: String,

    /// Media type (e.g., "image/png", "image/jpeg")
    pub media_type: String,

    /// Image data
    pub data: ImageData,

    /// Optional dimensions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<ImageDimensions>,

    /// When this part was created
    pub created_at: DateTime<Utc>,

    /// Optional provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<serde_json::Value>,
}

impl ImagePart {
    /// Create a new image part with the given parameters.
    pub fn new(id: String, media_type: String, data: ImageData) -> Self {
        Self {
            id,
            media_type,
            data,
            dimensions: None,
            created_at: Utc::now(),
            provider_metadata: None,
        }
    }

    /// Set the dimensions for this image.
    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self {
        self.dimensions = Some(ImageDimensions { width, height });
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_part_creation_with_url() {
        let data = ImageData::Url {
            url: "https://example.com/image.png".to_string(),
        };
        let part = ImagePart::new("img-id".to_string(), "image/png".to_string(), data);
        assert_eq!(part.id, "img-id");
        assert_eq!(part.media_type, "image/png");
        assert!(part.dimensions.is_none());
    }

    #[test]
    fn test_image_part_with_dimensions() {
        let data = ImageData::Url {
            url: "https://example.com/image.png".to_string(),
        };
        let part = ImagePart::new("img-id".to_string(), "image/png".to_string(), data)
            .with_dimensions(1024, 768);
        assert_eq!(
            part.dimensions,
            Some(ImageDimensions {
                width: 1024,
                height: 768
            })
        );
    }

    #[test]
    fn test_image_part_serialization() {
        let data = ImageData::Base64 {
            data: "base64data".to_string(),
        };
        let part = ImagePart::new("img-id".to_string(), "image/png".to_string(), data);
        let json = serde_json::to_string(&part).unwrap();
        let deserialized: ImagePart = serde_json::from_str(&json).unwrap();
        assert_eq!(part.id, deserialized.id);
    }

    #[test]
    fn test_image_data_variants() {
        // Test Base64
        let base64 = ImageData::Base64 {
            data: "test".to_string(),
        };
        let json = serde_json::to_string(&base64).unwrap();
        assert!(json.contains(r#""format":"base64""#));

        // Test URL
        let url = ImageData::Url {
            url: "https://example.com".to_string(),
        };
        let json = serde_json::to_string(&url).unwrap();
        assert!(json.contains(r#""format":"url""#));

        // Test Binary
        let binary = ImageData::Binary {
            data: vec![1, 2, 3],
        };
        let json = serde_json::to_string(&binary).unwrap();
        assert!(json.contains(r#""format":"binary""#));
    }
}
