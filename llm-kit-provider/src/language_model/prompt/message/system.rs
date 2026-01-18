use crate::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};

/// System message in a prompt
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelSystemMessage {
    /// Role identifier for system messages
    pub role: SystemRole,

    /// The system message content
    pub content: String,

    /// Additional provider-specific options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<SharedProviderOptions>,
}

/// Role identifier for system messages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "system")]
pub struct SystemRole;

impl LanguageModelSystemMessage {
    /// Create a new system message
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            role: SystemRole,
            content: content.into(),
            provider_options: None,
        }
    }

    /// Create a new system message with provider options
    pub fn with_options(
        content: impl Into<String>,
        provider_options: Option<SharedProviderOptions>,
    ) -> Self {
        Self {
            role: SystemRole,
            content: content.into(),
            provider_options,
        }
    }
}
