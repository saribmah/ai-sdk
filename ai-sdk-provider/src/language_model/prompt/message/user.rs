use super::parts::{LanguageModelFilePart, LanguageModelTextPart};
use crate::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};

/// Content parts that can appear in a user message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LanguageModelUserMessagePart {
    /// Text content
    Text(LanguageModelTextPart),

    /// File content
    File(LanguageModelFilePart),
}

/// User message struct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelUserMessage {
    #[serde(rename = "role")]
    pub message_role: UserRole,

    /// Array of text or file content parts
    pub content: Vec<LanguageModelUserMessagePart>,

    /// Additional provider-specific options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<SharedProviderOptions>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "user")]
pub(crate) struct UserRole;

impl LanguageModelUserMessage {
    pub fn new(content: Vec<LanguageModelUserMessagePart>) -> Self {
        Self {
            message_role: UserRole,
            content,
            provider_options: None,
        }
    }

    pub fn with_options(
        content: Vec<LanguageModelUserMessagePart>,
        provider_options: Option<SharedProviderOptions>,
    ) -> Self {
        Self {
            message_role: UserRole,
            content,
            provider_options,
        }
    }

    /// Create a user message with text
    pub fn text(text: impl Into<String>) -> Self {
        Self::new(vec![LanguageModelUserMessagePart::Text(LanguageModelTextPart::new(text))])
    }
}
