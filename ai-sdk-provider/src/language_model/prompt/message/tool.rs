use crate::shared::provider_options::ProviderOptions;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Tool message with tool results
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolMessage {
    /// Array of tool result parts
    pub content: Vec<ToolResultPart>,

    /// Additional provider-specific options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<ProviderOptions>,
}

/// Tool result part (used in tool messages and assistant messages)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolResultPart {
    /// Type discriminator (always "tool-result")
    #[serde(rename = "type")]
    part_type: ToolResultPartType,

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
struct ToolResultPartType;

/// Tool result output types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ToolResultOutput {
    /// Plain text output
    Text {
        /// The text value
        value: String,
    },

    /// JSON output
    Json {
        /// The JSON value
        value: Value,
    },

    /// Error as text
    #[serde(rename = "error-text")]
    ErrorText {
        /// The error message
        value: String,
    },

    /// Error as JSON
    #[serde(rename = "error-json")]
    ErrorJson {
        /// The error value as JSON
        value: Value,
    },

    /// Content with multiple parts (text and/or media)
    Content {
        /// Array of content items
        value: Vec<ToolResultContentItem>,
    },
}

/// Content items within a tool result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ToolResultContentItem {
    /// Text content
    Text {
        /// Text content
        text: String,
    },

    /// Media content (image, audio, etc.)
    Media {
        /// Base64-encoded media data
        data: String,

        /// IANA media type
        #[serde(rename = "mediaType")]
        media_type: String,
    },
}

impl ToolResultPart {
    /// Create a new tool result part
    pub fn new(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        output: ToolResultOutput,
    ) -> Self {
        Self {
            part_type: ToolResultPartType,
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            output,
            provider_options: None,
        }
    }
}

impl ToolResultOutput {
    /// Create a text output
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text {
            value: value.into(),
        }
    }

    /// Create a JSON output
    pub fn json(value: Value) -> Self {
        Self::Json { value }
    }

    /// Create an error text output
    pub fn error_text(value: impl Into<String>) -> Self {
        Self::ErrorText {
            value: value.into(),
        }
    }

    /// Create an error JSON output
    pub fn error_json(value: Value) -> Self {
        Self::ErrorJson { value }
    }

    /// Create a content output with items
    pub fn content(value: Vec<ToolResultContentItem>) -> Self {
        Self::Content { value }
    }
}
