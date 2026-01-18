use crate::language_model::prompt::message::data_content::LanguageModelDataContent;
use crate::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};

/// File content part in a message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelFilePart {
    /// Type discriminator for file parts
    #[serde(rename = "type")]
    pub content_type: FilePartType,

    /// Optional filename of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// File data
    pub data: LanguageModelDataContent,

    /// IANA media type of the file
    pub media_type: String,

    /// Additional provider-specific options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<SharedProviderOptions>,
}

/// Type discriminator for file parts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "file")]
pub struct FilePartType;

impl LanguageModelFilePart {
    /// Create a new file part
    pub fn new(data: LanguageModelDataContent, media_type: impl Into<String>) -> Self {
        Self {
            content_type: FilePartType,
            filename: None,
            data,
            media_type: media_type.into(),
            provider_options: None,
        }
    }

    /// Create a new file part with all options
    pub fn with_options(
        filename: Option<String>,
        data: LanguageModelDataContent,
        media_type: impl Into<String>,
        provider_options: Option<SharedProviderOptions>,
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
