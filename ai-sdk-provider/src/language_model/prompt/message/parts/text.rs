use crate::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelTextPart {
    #[serde(rename = "type")]
    pub content_type: TextPartType,

    /// The text content
    pub text: String,

    /// Additional provider-specific options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<SharedProviderOptions>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "text")]
pub struct TextPartType;

impl LanguageModelTextPart {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            content_type: TextPartType,
            text: text.into(),
            provider_options: None,
        }
    }

    pub fn with_options(
        text: impl Into<String>,
        provider_options: Option<SharedProviderOptions>,
    ) -> Self {
        Self {
            content_type: TextPartType,
            text: text.into(),
            provider_options,
        }
    }
}
