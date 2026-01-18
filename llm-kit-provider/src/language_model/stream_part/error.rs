use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Error event in a streaming response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelStreamError {
    /// Type discriminator for error events
    #[serde(rename = "type")]
    pub content_type: ErrorType,

    /// The error value
    pub error: Value,
}

/// Type discriminator for error events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "error")]
pub struct ErrorType;

impl LanguageModelStreamError {
    /// Create a new error event
    pub fn new(error: Value) -> Self {
        Self {
            content_type: ErrorType,
            error,
        }
    }
}
