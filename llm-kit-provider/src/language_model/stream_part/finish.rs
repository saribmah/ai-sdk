use crate::language_model::finish_reason::LanguageModelFinishReason;
use crate::language_model::usage::LanguageModelUsage;
use crate::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};

/// Stream finish event with usage and finish reason
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelStreamFinish {
    /// Type discriminator for finish events
    #[serde(rename = "type")]
    pub content_type: FinishType,

    /// Usage information
    pub usage: LanguageModelUsage,

    /// Reason for finishing
    pub finish_reason: LanguageModelFinishReason,

    /// Provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,
}

/// Type discriminator for finish events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "finish")]
pub struct FinishType;

impl LanguageModelStreamFinish {
    /// Create a new finish event
    pub fn new(usage: LanguageModelUsage, finish_reason: LanguageModelFinishReason) -> Self {
        Self {
            content_type: FinishType,
            usage,
            finish_reason,
            provider_metadata: None,
        }
    }

    /// Create a new finish event with provider metadata
    pub fn with_metadata(
        usage: LanguageModelUsage,
        finish_reason: LanguageModelFinishReason,
        provider_metadata: Option<SharedProviderMetadata>,
    ) -> Self {
        Self {
            content_type: FinishType,
            usage,
            finish_reason,
            provider_metadata,
        }
    }
}
