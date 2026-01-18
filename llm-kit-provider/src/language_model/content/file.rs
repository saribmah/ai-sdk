use serde::{Deserialize, Serialize};

/// File content in a language model response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageModelFile {
    /// Type discriminator for file content
    #[serde(rename = "type")]
    pub file_type: FileType,

    /// IANA media type of the file (e.g., "image/png", "application/pdf")
    #[serde(rename = "mediaType")]
    pub media_type: String,

    /// The file data, either as base64 string or binary
    pub data: FileData,
}

/// Type discriminator for file content
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    /// Standard file type
    #[default]
    File,
}

/// File data encoding formats
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FileData {
    /// Base64-encoded file data
    Base64(String),
    /// Raw binary file data
    Binary(Vec<u8>),
}

impl LanguageModelFile {
    /// Create a file from base64-encoded data
    pub fn from_base64(media_type: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            file_type: FileType::File,
            media_type: media_type.into(),
            data: FileData::Base64(data.into()),
        }
    }

    /// Create a file from raw binary data
    pub fn from_binary(media_type: impl Into<String>, data: Vec<u8>) -> Self {
        Self {
            file_type: FileType::File,
            media_type: media_type.into(),
            data: FileData::Binary(data),
        }
    }
}
