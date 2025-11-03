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

/// User message struct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "role")]
    pub message_role: UserRole,

    /// Array of text or file content parts
    pub content: Vec<UserMessagePart>,

    /// Additional provider-specific options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<ProviderOptions>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "user")]
pub(crate) struct UserRole;

impl User {
    pub fn new(content: Vec<UserMessagePart>) -> Self {
        Self {
            message_role: UserRole,
            content,
            provider_options: None,
        }
    }

    pub fn with_options(
        content: Vec<UserMessagePart>,
        provider_options: Option<ProviderOptions>,
    ) -> Self {
        Self {
            message_role: UserRole,
            content,
            provider_options,
        }
    }

    /// Create a user message with text
    pub fn text(text: impl Into<String>) -> Self {
        Self::new(vec![UserMessagePart::Text {
            text: text.into(),
            provider_options: None,
        }])
    }
}
