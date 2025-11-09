use crate::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};

/// Start of a text block in a streaming response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelStreamTextStart {
    /// Type discriminator for text start events
    #[serde(rename = "type")]
    pub content_type: TextStartType,

    /// Unique identifier for this text block
    pub id: String,

    /// Provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,
}

/// Type discriminator for text start events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "text-start")]
pub struct TextStartType;

impl LanguageModelStreamTextStart {
    /// Create a new text start event
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            content_type: TextStartType,
            id: id.into(),
            provider_metadata: None,
        }
    }

    /// Create a new text start event with provider metadata
    pub fn with_metadata(
        id: impl Into<String>,
        provider_metadata: Option<SharedProviderMetadata>,
    ) -> Self {
        Self {
            content_type: TextStartType,
            id: id.into(),
            provider_metadata,
        }
    }
}
