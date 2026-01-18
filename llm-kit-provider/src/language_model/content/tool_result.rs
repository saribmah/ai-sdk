use crate::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Tool call result in a language model response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] // ← Add this!
pub struct LanguageModelToolResult {
    /// Type discriminator for tool result content
    #[serde(rename = "type")]
    pub content_type: ToolResultType,

    /// ID of the tool call this result is for
    pub tool_call_id: String,

    /// Name of the tool that was called
    pub tool_name: String,

    /// Result value from the tool execution
    pub result: Value,

    /// Whether this result represents an error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,

    /// Whether this tool was executed by the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,

    /// Additional provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,
}

/// Type discriminator for tool result content
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "tool-result")]
pub struct ToolResultType;

impl LanguageModelToolResult {
    /// Create a new tool result
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
        provider_metadata: Option<SharedProviderMetadata>,
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
