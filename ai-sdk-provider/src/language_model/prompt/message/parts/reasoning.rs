use crate::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelReasoningPart {
    #[serde(rename = "type")]
    pub content_type: ReasoningPartType,

    /// The reasoning text
    pub text: String,

    /// Additional provider-specific options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<SharedProviderOptions>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "reasoning")]
pub struct ReasoningPartType;

impl LanguageModelReasoningPart {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            content_type: ReasoningPartType,
            text: text.into(),
            provider_options: None,
        }
    }

    pub fn with_options(
        text: impl Into<String>,
        provider_options: Option<SharedProviderOptions>,
    ) -> Self {
        Self {
            content_type: ReasoningPartType,
            text: text.into(),
            provider_options,
        }
    }
}
