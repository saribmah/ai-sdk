use serde::{Deserialize, Serialize};

/// Metadata about a language model response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelResponseMetadata {
    /// Unique identifier for this response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Unix timestamp when the response was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,

    /// Identifier of the model used for this response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
}
