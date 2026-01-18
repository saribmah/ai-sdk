use crate::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};

/// Tool call request in a language model response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] // ← Add this!
pub struct LanguageModelToolCall {
    /// Type discriminator for tool call content
    #[serde(rename = "type")]
    pub content_type: ToolCallType,

    /// Unique identifier for this tool call
    pub tool_call_id: String,

    /// Name of the tool to call
    pub tool_name: String,

    /// JSON-serialized input arguments for the tool
    pub input: String,

    /// Whether this tool call will be executed by the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,

    /// Additional provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,
}

/// Type discriminator for tool call content
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "tool-call")]
pub struct ToolCallType;

impl LanguageModelToolCall {
    /// Create a new tool call
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
        provider_metadata: Option<SharedProviderMetadata>,
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
