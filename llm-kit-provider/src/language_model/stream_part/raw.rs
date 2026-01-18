use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Raw chunk from the provider
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelStreamRaw {
    /// Type discriminator for raw events
    #[serde(rename = "type")]
    pub content_type: RawType,

    /// The raw value from the provider
    #[serde(rename = "rawValue")]
    pub raw_value: Value,
}

/// Type discriminator for raw events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "raw")]
pub struct RawType;

impl LanguageModelStreamRaw {
    /// Create a new raw chunk event
    pub fn new(raw_value: Value) -> Self {
        Self {
            content_type: RawType,
            raw_value,
        }
    }
}
