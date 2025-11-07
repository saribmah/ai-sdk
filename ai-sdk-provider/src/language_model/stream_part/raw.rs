use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelStreamRaw {
    #[serde(rename = "type")]
    pub content_type: RawType,

    /// The raw value from the provider
    #[serde(rename = "rawValue")]
    pub raw_value: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "raw")]
pub struct RawType;

impl LanguageModelStreamRaw {
    pub fn new(raw_value: Value) -> Self {
        Self {
            content_type: RawType,
            raw_value,
        }
    }
}
