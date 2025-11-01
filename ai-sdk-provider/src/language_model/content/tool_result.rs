use crate::shared::provider_metadata::ProviderMetadata;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] // ← Add this!
pub struct ToolResult {
    #[serde(rename = "type")]
    pub content_type: ToolResultType,

    pub tool_call_id: String,

    pub tool_name: String,

    pub result: Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<ProviderMetadata>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "tool-result")]
struct ToolResultType;

impl ToolResult {
    pub fn new(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        result: Value,
    ) -> Self {
        Self {
            content_type: ToolResultType,
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            result,
            is_error: None,
            provider_executed: None,
            provider_metadata: None,
        }
    }

    /// Create a tool result with all fields
    pub fn with_options(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        result: Value,
        is_error: Option<bool>,
        provider_executed: Option<bool>,
        provider_metadata: Option<ProviderMetadata>,
    ) -> Self {
        Self {
            content_type: ToolResultType,
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            result,
            is_error,
            provider_executed,
            provider_metadata,
        }
    }
}
