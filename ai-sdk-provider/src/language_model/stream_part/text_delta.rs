use crate::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelStreamTextDelta {
    #[serde(rename = "type")]
    pub content_type: TextDeltaType,

    /// ID of the text block
    pub id: String,

    /// The text delta/chunk
    pub delta: String,

    /// Provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "text-delta")]
pub(crate) struct TextDeltaType;

impl LanguageModelStreamTextDelta {
    pub fn new(id: impl Into<String>, delta: impl Into<String>) -> Self {
        Self {
            content_type: TextDeltaType,
            id: id.into(),
            delta: delta.into(),
            provider_metadata: None,
        }
    }

    pub fn with_metadata(
        id: impl Into<String>,
        delta: impl Into<String>,
        provider_metadata: Option<SharedProviderMetadata>,
    ) -> Self {
        Self {
            content_type: TextDeltaType,
            id: id.into(),
            delta: delta.into(),
            provider_metadata,
        }
    }
}
