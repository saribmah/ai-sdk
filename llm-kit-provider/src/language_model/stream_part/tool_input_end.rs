use crate::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};

/// End of tool input in a streaming response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelStreamToolInputEnd {
    /// Type discriminator for tool input end events
    #[serde(rename = "type")]
    pub content_type: ToolInputEndType,

    /// ID of the tool call
    pub id: String,

    /// Provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,
}

/// Type discriminator for tool input end events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "tool-input-end")]
pub struct ToolInputEndType;

impl LanguageModelStreamToolInputEnd {
    /// Create a new tool input end event
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            content_type: ToolInputEndType,
            id: id.into(),
            provider_metadata: None,
        }
    }

    /// Create a new tool input end event with provider metadata
    pub fn with_metadata(
        id: impl Into<String>,
        provider_metadata: Option<SharedProviderMetadata>,
    ) -> Self {
        Self {
            content_type: ToolInputEndType,
            id: id.into(),
            provider_metadata,
        }
    }
}
