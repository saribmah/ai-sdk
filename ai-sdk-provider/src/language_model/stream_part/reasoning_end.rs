use crate::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelStreamReasoningEnd {
    #[serde(rename = "type")]
    pub content_type: ReasoningEndType,

    /// ID of the reasoning block
    pub id: String,

    /// Provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "reasoning-end")]
pub struct ReasoningEndType;

impl LanguageModelStreamReasoningEnd {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            content_type: ReasoningEndType,
            id: id.into(),
            provider_metadata: None,
        }
    }

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
