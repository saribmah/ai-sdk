use crate::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};

/// Reasoning or thought process content in a language model response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageModelReasoning {
    /// Type discriminator for reasoning content
    #[serde(rename = "type")]
    pub content_type: ReasoningType,

    /// The reasoning text
    pub text: String,

    /// Additional provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,
}

/// Type discriminator for reasoning content
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "reasoning")]
pub struct ReasoningType;

impl LanguageModelReasoning {
    /// Create a new reasoning content
    pub fn init(text: impl Into<String>) -> Self {
        Self {
            content_type: ReasoningType,
            text: text.into(),
            provider_metadata: None,
        }
    }

    /// Create a new reasoning content with provider metadata
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
