use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "type")]
    pub file_type: FileType,

    #[serde(rename = "mediaType")]
    pub media_type: String,

    pub data: FileData,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    File,
}

impl Default for FileType {
    fn default() -> Self {
        Self::File
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FileData {
    Base64(String),
    Binary(Vec<u8>),
}

impl File {
    pub fn from_base64(media_type: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            file_type: FileType::File,
            media_type: media_type.into(),
            data: FileData::Base64(data.into()),
        }
    }

    pub fn from_binary(media_type: impl Into<String>, data: Vec<u8>) -> Self {
        Self {
            file_type: FileType::File,
            media_type: media_type.into(),
            data: FileData::Binary(data.into()),
        }
    }
}
