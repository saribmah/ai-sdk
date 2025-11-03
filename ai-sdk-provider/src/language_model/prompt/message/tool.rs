use super::parts::LanguageModelToolResultPart;
use crate::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};

/// Tool message struct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelToolMessage {
    #[serde(rename = "role")]
    pub message_role: ToolRole,

    /// Array of tool result parts
    pub content: Vec<LanguageModelToolResultPart>,

    /// Additional provider-specific options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<SharedProviderOptions>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "tool")]
pub(crate) struct ToolRole;

impl LanguageModelToolMessage {
    pub fn new(content: Vec<LanguageModelToolResultPart>) -> Self {
        Self {
            message_role: ToolRole,
            content,
            provider_options: None,
        }
    }

    pub fn with_options(
        content: Vec<LanguageModelToolResultPart>,
        provider_options: Option<SharedProviderOptions>,
    ) -> Self {
        Self {
            message_role: ToolRole,
            content,
            provider_options,
        }
    }
}
