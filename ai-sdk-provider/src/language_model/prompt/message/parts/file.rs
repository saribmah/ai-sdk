use crate::language_model::prompt::message::data_content::DataContent;
use crate::shared::provider_options::ProviderOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilePart {
    #[serde(rename = "type")]
    pub content_type: FilePartType,

    /// Optional filename of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// File data
    pub data: DataContent,

    /// IANA media type of the file
    pub media_type: String,

    /// Additional provider-specific options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<ProviderOptions>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "file")]
pub(crate) struct FilePartType;

impl FilePart {
    pub fn new(data: DataContent, media_type: impl Into<String>) -> Self {
        Self {
            content_type: FilePartType,
            filename: None,
            data,
            media_type: media_type.into(),
            provider_options: None,
        }
    }

    pub fn with_options(
        filename: Option<String>,
        data: DataContent,
        media_type: impl Into<String>,
        provider_options: Option<ProviderOptions>,
    ) -> Self {
        Self {
            content_type: FilePartType,
            filename,
            data,
            media_type: media_type.into(),
            provider_options,
        }
    }
}
