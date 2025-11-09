use crate::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};

/// End of a text block in a streaming response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelStreamTextEnd {
    /// Type discriminator for text end events
    #[serde(rename = "type")]
    pub content_type: TextEndType,

    /// ID of the text block
    pub id: String,

    /// Provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,
}

/// Type discriminator for text end events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "text-end")]
pub struct TextEndType;

impl LanguageModelStreamTextEnd {
    /// Create a new text end event
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            content_type: TextEndType,
            id: id.into(),
            provider_metadata: None,
        }
    }

    /// Create a new text end event with provider metadata
    pub fn with_metadata(
        id: impl Into<String>,
        provider_metadata: Option<SharedProviderMetadata>,
    ) -> Self {
        Self {
            content_type: TextEndType,
            id: id.into(),
            provider_metadata,
        }
    }
}
