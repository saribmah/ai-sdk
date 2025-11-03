use crate::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageModelReasoning {
    #[serde(rename = "type")]
    pub content_type: ReasoningType,

    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "reasoning")]
struct ReasoningType;

impl LanguageModelReasoning {
    pub fn init(text: impl Into<String>) -> Self {
        Self {
            content_type: ReasoningType,
            text: text.into(),
            provider_metadata: None,
        }
    }

    pub fn with_metadata(
        text: impl Into<String>,
        provider_metadata: SharedProviderMetadata,
    ) -> Self {
        Self {
            content_type: ReasoningType,
            text: text.into(),
            provider_metadata: Some(provider_metadata),
        }
    }
}
