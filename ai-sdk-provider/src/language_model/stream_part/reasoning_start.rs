use crate::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};

/// Start of a reasoning block in a streaming response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelStreamReasoningStart {
    /// Type discriminator for reasoning start events
    #[serde(rename = "type")]
    pub content_type: ReasoningStartType,

    /// Unique identifier for this reasoning block
    pub id: String,

    /// Provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,
}

/// Type discriminator for reasoning start events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "reasoning-start")]
pub struct ReasoningStartType;

impl LanguageModelStreamReasoningStart {
    /// Create a new reasoning start event
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            content_type: ReasoningStartType,
            id: id.into(),
            provider_metadata: None,
        }
    }

    /// Create a new reasoning start event with provider metadata
    pub fn with_metadata(
        id: impl Into<String>,
        provider_metadata: Option<SharedProviderMetadata>,
    ) -> Self {
        Self {
            content_type: ReasoningStartType,
            id: id.into(),
            provider_metadata,
        }
    }
}
