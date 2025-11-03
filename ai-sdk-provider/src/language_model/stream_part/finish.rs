use crate::language_model::finish_reason::LanguageModelFinishReason;
use crate::language_model::usage::LanguageModelUsage;
use crate::shared::provider_metadata::ProviderMetadata;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Finish {
    #[serde(rename = "type")]
    pub content_type: FinishType,

    /// Usage information
    pub usage: LanguageModelUsage,

    /// Reason for finishing
    pub finish_reason: LanguageModelFinishReason,

    /// Provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<ProviderMetadata>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "finish")]
pub(crate) struct FinishType;

impl Finish {
    pub fn new(usage: LanguageModelUsage, finish_reason: LanguageModelFinishReason) -> Self {
        Self {
            content_type: FinishType,
            usage,
            finish_reason,
            provider_metadata: None,
        }
    }

    pub fn with_metadata(
        usage: LanguageModelUsage,
        finish_reason: LanguageModelFinishReason,
        provider_metadata: Option<ProviderMetadata>,
    ) -> Self {
        Self {
            content_type: FinishType,
            usage,
            finish_reason,
            provider_metadata,
        }
    }
}
