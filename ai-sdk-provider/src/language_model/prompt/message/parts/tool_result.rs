use crate::language_model::prompt::message::tool::ToolResultOutput;
use crate::shared::provider_options::ProviderOptions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolResultPart {
    #[serde(rename = "type")]
    pub content_type: ToolResultPartType,

    /// ID of the tool call this result is associated with
    pub tool_call_id: String,

    /// Name of the tool that generated this result
    pub tool_name: String,

    /// Result of the tool call
    pub output: ToolResultOutput,

    /// Additional provider-specific options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<ProviderOptions>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "tool-result")]
pub(crate) struct ToolResultPartType;

impl ToolResultPart {
    pub fn new(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        output: ToolResultOutput,
    ) -> Self {
        Self {
            content_type: ToolResultPartType,
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            output,
            provider_options: None,
        }
    }

    pub fn with_options(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        output: ToolResultOutput,
        provider_options: Option<ProviderOptions>,
    ) -> Self {
        Self {
            content_type: ToolResultPartType,
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            output,
            provider_options,
        }
    }
}
