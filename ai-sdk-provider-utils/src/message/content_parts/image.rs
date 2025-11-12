use crate::message::DataContent;
use ai_sdk_provider::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};
use url::Url;

/// Image content part of a prompt. It contains an image.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImagePart {
    /// Image data. Can either be data (base64 or bytes) or a URL.
    pub image: ImageSource,

    /// Optional IANA media type of the image.
    ///
    /// @see <https://www.iana.org/assignments/media-types/media-types.xhtml>
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,

    /// Additional provider-specific metadata.
    #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<SharedProviderOptions>,
}

/// Source of image data - either raw data or a URL.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageSource {
    /// Data content (base64 or raw bytes)
    Data(DataContent),
    /// URL pointing to the image
    Url(Url),
}

impl ImagePart {
    /// Creates a new image part from data content.
    pub fn from_data(image: impl Into<DataContent>) -> Self {
        Self {
            image: ImageSource::Data(image.into()),
            media_type: None,
            provider_options: None,
        }
    }

    /// Creates a new image part from a URL.
    pub fn from_url(url: Url) -> Self {
        Self {
            image: ImageSource::Url(url),
            media_type: None,
            provider_options: None,
        }
    }

    /// Sets the media type.
    pub fn with_media_type(mut self, media_type: impl Into<String>) -> Self {
        self.media_type = Some(media_type.into());
        self
    }

    /// Sets the provider options.
    pub fn with_provider_options(mut self, options: SharedProviderOptions) -> Self {
        self.provider_options = Some(options);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_part_from_data() {
        let data = DataContent::base64("iVBORw0KGgo=");
        let part = ImagePart::from_data(data.clone());

        match part.image {
            ImageSource::Data(d) => assert_eq!(d, data),
            _ => panic!("Expected Data variant"),
        }
        assert!(part.media_type.is_none());
    }

    #[test]
    fn test_image_part_from_url() {
        let url = Url::parse("https://example.com/image.png").unwrap();
        let part = ImagePart::from_url(url.clone());

        match part.image {
            ImageSource::Url(u) => assert_eq!(u, url),
            _ => panic!("Expected Url variant"),
        }
    }

    #[test]
    fn test_image_part_with_media_type() {
        let data = DataContent::base64("data");
        let part = ImagePart::from_data(data).with_media_type("image/png");

        assert_eq!(part.media_type, Some("image/png".to_string()));
    }
}
