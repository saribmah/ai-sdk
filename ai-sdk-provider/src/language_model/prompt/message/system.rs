use crate::shared::provider_options::ProviderOptions;
use serde::{Deserialize, Serialize};

/// System message with text content
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemMessage {
    /// The system message content
    pub content: String,

    /// Additional provider-specific options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<ProviderOptions>,
}

impl SystemMessage {
    /// Create a system message
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            provider_options: None,
        }
    }
}
