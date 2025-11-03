use crate::language_model::call_warning::LanguageModelCallWarning;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelStreamStart {
    #[serde(rename = "type")]
    pub content_type: StreamStartType,

    /// Warnings for the call (e.g., unsupported settings)
    pub warnings: Vec<LanguageModelCallWarning>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "stream-start")]
pub(crate) struct StreamStartType;

impl LanguageModelStreamStart {
    pub fn new(warnings: Vec<LanguageModelCallWarning>) -> Self {
        Self {
            content_type: StreamStartType,
            warnings,
        }
    }
}
