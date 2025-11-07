use crate::prompt::message::DataContent;
use ai_sdk_provider::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};
use url::Url;

/// File content part of a prompt. It contains a file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilePart {
    /// File data. Can either be data (base64 or bytes) or a URL.
    pub data: FileSource,

    /// Optional filename of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// IANA media type of the file.
    ///
    /// @see <https://www.iana.org/assignments/media-types/media-types.xhtml>
    #[serde(rename = "mediaType")]
    pub media_type: String,

    /// Additional provider-specific metadata.
    #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<SharedProviderOptions>,
}

/// Source of file data - either raw data or a URL.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FileSource {
    /// Data content (base64 or raw bytes)
    Data(DataContent),
    /// URL pointing to the file
    Url(Url),
}

impl FilePart {
    /// Creates a new file part from data content.
    pub fn from_data(data: impl Into<DataContent>, media_type: impl Into<String>) -> Self {
        Self {
            data: FileSource::Data(data.into()),
            filename: None,
            media_type: media_type.into(),
            provider_options: None,
        }
    }

    /// Creates a new file part from a URL.
    pub fn from_url(url: Url, media_type: impl Into<String>) -> Self {
        Self {
            data: FileSource::Url(url),
            filename: None,
            media_type: media_type.into(),
            provider_options: None,
        }
    }

    /// Sets the filename.
    pub fn with_filename(mut self, filename: impl Into<String>) -> Self {
        self.filename = Some(filename.into());
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
    fn test_file_part_from_data() {
        let data = DataContent::base64("SGVsbG8=");
        let part = FilePart::from_data(data.clone(), "application/pdf");

        match &part.data {
            FileSource::Data(d) => assert_eq!(d, &data),
            _ => panic!("Expected Data variant"),
        }
        assert_eq!(part.media_type, "application/pdf");
        assert!(part.filename.is_none());
    }

    #[test]
    fn test_file_part_from_url() {
        let url = Url::parse("https://example.com/document.pdf").unwrap();
        let part = FilePart::from_url(url.clone(), "application/pdf");

        match &part.data {
            FileSource::Url(u) => assert_eq!(u, &url),
            _ => panic!("Expected Url variant"),
        }
    }

    #[test]
    fn test_file_part_with_filename() {
        let data = DataContent::base64("data");
        let part = FilePart::from_data(data, "application/pdf").with_filename("document.pdf");

        assert_eq!(part.filename, Some("document.pdf".to_string()));
    }
}
