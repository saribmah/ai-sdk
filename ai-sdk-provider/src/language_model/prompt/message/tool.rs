use super::parts::ToolResultPart;
use crate::shared::provider_options::ProviderOptions;
use serde::{Deserialize, Serialize};

/// Tool message struct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    #[serde(rename = "role")]
    pub message_role: ToolRole,

    /// Array of tool result parts
    pub content: Vec<ToolResultPart>,

    /// Additional provider-specific options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<ProviderOptions>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "tool")]
pub(crate) struct ToolRole;

impl Tool {
    pub fn new(content: Vec<ToolResultPart>) -> Self {
        Self {
            message_role: ToolRole,
            content,
            provider_options: None,
        }
    }

    pub fn with_options(
        content: Vec<ToolResultPart>,
        provider_options: Option<ProviderOptions>,
    ) -> Self {
        Self {
            message_role: ToolRole,
            content,
            provider_options,
        }
    }
}
