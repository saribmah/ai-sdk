use crate::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};

/// Incremental reasoning content in a streaming response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelStreamReasoningDelta {
    /// Type discriminator for reasoning delta events
    #[serde(rename = "type")]
    pub content_type: ReasoningDeltaType,

    /// ID of the reasoning block
    pub id: String,

    /// The reasoning delta/chunk
    pub delta: String,

    /// Provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,
}

/// Type discriminator for reasoning delta events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "reasoning-delta")]
pub struct ReasoningDeltaType;

impl LanguageModelStreamReasoningDelta {
    /// Create a new reasoning delta event
    pub fn new(id: impl Into<String>, delta: impl Into<String>) -> Self {
        Self {
            content_type: ReasoningDeltaType,
            id: id.into(),
            delta: delta.into(),
            provider_metadata: None,
        }
    }

    /// Create a new reasoning delta event with provider metadata
    pub fn with_metadata(
        id: impl Into<String>,
        delta: impl Into<String>,
        provider_metadata: Option<SharedProviderMetadata>,
    ) -> Self {
        Self {
            content_type: ReasoningDeltaType,
            id: id.into(),
            delta: delta.into(),
            provider_metadata,
        }
    }
}
