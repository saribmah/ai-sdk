use crate::language_model::call_warning::LanguageModelCallWarning;
use serde::{Deserialize, Serialize};

/// Stream start event with warnings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelStreamStart {
    /// Type discriminator for stream start events
    #[serde(rename = "type")]
    pub content_type: StreamStartType,

    /// Warnings for the call (e.g., unsupported settings)
    pub warnings: Vec<LanguageModelCallWarning>,
}

/// Type discriminator for stream start events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "stream-start")]
pub struct StreamStartType;

impl LanguageModelStreamStart {
    /// Create a new stream start event
    pub fn new(warnings: Vec<LanguageModelCallWarning>) -> Self {
        Self {
            content_type: StreamStartType,
            warnings,
        }
    }
}
