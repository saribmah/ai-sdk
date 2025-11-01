use super::data_content::DataContent;
use super::tool::ToolResultOutput;
use crate::shared::provider_options::ProviderOptions;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Assistant message with various content types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssistantMessage {
    /// Array of content parts (text, files, reasoning, tool calls, tool results)
    pub content: Vec<AssistantMessagePart>,

    /// Additional provider-specific options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<ProviderOptions>,
}

/// Content parts that can appear in an assistant message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum AssistantMessagePart {
    /// Text content
    #[serde(rename_all = "camelCase")]
    Text {
        /// The text content
        text: String,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// File content
    #[serde(rename_all = "camelCase")]
    File {
        /// Optional filename of the file
        #[serde(skip_serializing_if = "Option::is_none")]
        filename: Option<String>,

        /// File data
        data: DataContent,

        /// IANA media type of the file
        media_type: String,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// Reasoning content
    #[serde(rename_all = "camelCase")]
    Reasoning {
        /// The reasoning text
        text: String,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// Tool call
    #[serde(rename = "tool-call", rename_all = "camelCase")]
    ToolCall {
        /// ID of the tool call
        tool_call_id: String,

        /// Name of the tool being called
        tool_name: String,

        /// Arguments of the tool call (JSON-serializable object)
        input: Value,

        /// Whether the tool call will be executed by the provider
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_executed: Option<bool>,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// Tool result
    #[serde(rename = "tool-result", rename_all = "camelCase")]
    ToolResult {
        /// ID of the tool call this result is associated with
        tool_call_id: String,

        /// Name of the tool that generated this result
        tool_name: String,

        /// Result of the tool call
        output: ToolResultOutput,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
}

impl AssistantMessage {
    /// Create an assistant message with text
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            content: vec![AssistantMessagePart::Text {
                text: text.into(),
                provider_options: None,
            }],
            provider_options: None,
        }
    }
}
