use crate::shared::provider_options::ProviderOptions;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolCallPart {
    #[serde(rename = "type")]
    pub content_type: ToolCallPartType,

    /// ID of the tool call
    pub tool_call_id: String,

    /// Name of the tool being called
    pub tool_name: String,

    /// Arguments of the tool call (JSON-serializable object)
    pub input: Value,

    /// Whether the tool call will be executed by the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,

    /// Additional provider-specific options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<ProviderOptions>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "tool-call")]
pub(crate) struct ToolCallPartType;

impl ToolCallPart {
    pub fn new(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        input: Value,
    ) -> Self {
        Self {
            content_type: ToolCallPartType,
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            input,
            provider_executed: None,
            provider_options: None,
        }
    }

    pub fn with_options(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        input: Value,
        provider_executed: Option<bool>,
        provider_options: Option<ProviderOptions>,
    ) -> Self {
        Self {
            content_type: ToolCallPartType,
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            input,
            provider_executed,
            provider_options,
        }
    }
}
