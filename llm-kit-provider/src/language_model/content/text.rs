use crate::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};

/// Plain text content in a language model response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageModelText {
    /// Type discriminator for text content
    #[serde(rename = "type")]
    pub content_type: TextType,

    /// The text content
    pub text: String,

    /// Additional provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,
}

/// Type discriminator for text content
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "text")]
pub struct TextType;

impl LanguageModelText {
    /// Create a new text content
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            content_type: TextType,
            text: text.into(),
            provider_metadata: None,
        }
    }

    /// Create a new text content with provider metadata
    pub fn with_metadata(
        text: impl Into<String>,
        provider_metadata: SharedProviderMetadata,
    ) -> Self {
        Self {
            content_type: TextType,
            text: text.into(),
            provider_metadata: Some(provider_metadata),
        }
    }
}
