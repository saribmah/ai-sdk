use crate::shared::provider_metadata::ProviderMetadata;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] // ← Add this!
pub struct LanguageModelToolCall {
    #[serde(rename = "type")]
    pub content_type: ToolCallType,

    pub tool_call_id: String,

    pub tool_name: String,

    pub input: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<ProviderMetadata>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "tool-call")]
struct ToolCallType;

impl LanguageModelToolCall {
    pub fn new(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        input: impl Into<String>,
    ) -> Self {
        Self {
            content_type: ToolCallType,
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            input: input.into(),
            provider_executed: None,
            provider_metadata: None,
        }
    }

    /// Create a tool call with all fields
    pub fn with_options(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        input: impl Into<String>,
        provider_executed: Option<bool>,
        provider_metadata: Option<ProviderMetadata>,
    ) -> Self {
        Self {
            content_type: ToolCallType,
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            input: input.into(),
            provider_executed,
            provider_metadata,
        }
    }
}
