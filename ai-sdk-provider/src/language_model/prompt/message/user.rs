use super::parts::{FilePart, TextPart};
use crate::shared::provider_options::ProviderOptions;
use serde::{Deserialize, Serialize};

/// Content parts that can appear in a user message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserMessagePart {
    /// Text content
    Text(TextPart),

    /// File content
    File(FilePart),
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
        Self::new(vec![UserMessagePart::Text(TextPart::new(text))])
    }
}
