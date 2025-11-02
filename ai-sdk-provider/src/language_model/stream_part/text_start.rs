use crate::shared::provider_metadata::ProviderMetadata;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextStart {
    #[serde(rename = "type")]
    pub content_type: TextStartType,

    /// Unique identifier for this text block
    pub id: String,

    /// Provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<ProviderMetadata>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "text-start")]
pub(crate) struct TextStartType;

impl TextStart {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            content_type: TextStartType,
            id: id.into(),
            provider_metadata: None,
        }
    }

    pub fn with_metadata(
        id: impl Into<String>,
        provider_metadata: Option<ProviderMetadata>,
    ) -> Self {
        Self {
            content_type: TextStartType,
            id: id.into(),
            provider_metadata,
        }
    }
}
