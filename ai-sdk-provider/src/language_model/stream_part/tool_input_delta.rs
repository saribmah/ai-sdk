use crate::shared::provider_metadata::ProviderMetadata;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolInputDelta {
    #[serde(rename = "type")]
    pub content_type: ToolInputDeltaType,

    /// ID of the tool call
    pub id: String,

    /// The input delta/chunk (JSON string)
    pub delta: String,

    /// Provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<ProviderMetadata>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "tool-input-delta")]
pub(crate) struct ToolInputDeltaType;

impl ToolInputDelta {
    pub fn new(id: impl Into<String>, delta: impl Into<String>) -> Self {
        Self {
            content_type: ToolInputDeltaType,
            id: id.into(),
            delta: delta.into(),
            provider_metadata: None,
        }
    }

    pub fn with_metadata(
        id: impl Into<String>,
        delta: impl Into<String>,
        provider_metadata: Option<ProviderMetadata>,
    ) -> Self {
        Self {
            content_type: ToolInputDeltaType,
            id: id.into(),
            delta: delta.into(),
            provider_metadata,
        }
    }
}
