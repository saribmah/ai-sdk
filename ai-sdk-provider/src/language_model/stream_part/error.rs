use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    #[serde(rename = "type")]
    pub content_type: ErrorType,

    /// The error value
    pub error: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "error")]
pub(crate) struct ErrorType;

impl Error {
    pub fn new(error: Value) -> Self {
        Self {
            content_type: ErrorType,
            error,
        }
    }
}
