use crate::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct System {
    #[serde(rename = "role")]
    pub message_role: SystemRole,

    /// The system message content
    pub content: String,

    /// Additional provider-specific options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<SharedProviderOptions>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "system")]
pub(crate) struct SystemRole;

impl System {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            message_role: SystemRole,
            content: content.into(),
            provider_options: None,
        }
    }

    pub fn with_options(
        content: impl Into<String>,
        provider_options: Option<SharedProviderOptions>,
    ) -> Self {
        Self {
            message_role: SystemRole,
            content: content.into(),
            provider_options,
        }
    }
}
