use crate::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};

/// End of a reasoning block in a streaming response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelStreamReasoningEnd {
    /// Type discriminator for reasoning end events
    #[serde(rename = "type")]
    pub content_type: ReasoningEndType,

    /// ID of the reasoning block
    pub id: String,

    /// Provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,
}

/// Type discriminator for reasoning end events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "reasoning-end")]
pub struct ReasoningEndType;

impl LanguageModelStreamReasoningEnd {
    /// Create a new reasoning end event
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            content_type: ReasoningEndType,
            id: id.into(),
            provider_metadata: None,
        }
    }

    /// Create a new reasoning end event with provider metadata
    pub fn with_metadata(
        id: impl Into<String>,
        provider_metadata: Option<SharedProviderMetadata>,
    ) -> Self {
        Self {
            content_type: ReasoningEndType,
            id: id.into(),
            provider_metadata,
        }
    }
}
