use super::data_content::DataContent;
use crate::shared::provider_options::ProviderOptions;
use serde::{Deserialize, Serialize};

/// Content parts that can appear in a user message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum UserMessagePart {
    /// Text content
    #[serde(rename_all = "camelCase")]
    Text {
        /// The text content
        text: String,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// File content
    #[serde(rename_all = "camelCase")]
    File {
        /// Optional filename of the file
        #[serde(skip_serializing_if = "Option::is_none")]
        filename: Option<String>,

        /// File data. Can be binary, base64, or URL
        data: DataContent,

        /// IANA media type of the file
        media_type: String,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
}
