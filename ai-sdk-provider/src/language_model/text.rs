use crate::shared::provider_metadata::ProviderMetadata;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Text {
    #[serde(rename = "type")]
    pub content_type: TextType,

    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<ProviderMetadata>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "text")]
struct TextType;

impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            content_type: TextType,
            text: text.into(),
            provider_metadata: None,
        }
    }

    pub fn with_metadata(text: impl Into<String>, provider_metadata: ProviderMetadata) -> Self {
        Self {
            content_type: TextType,
            text: text.into(),
            provider_metadata: Some(provider_metadata),
        }
    }
}
