use super::parts::{FilePart, ReasoningPart, TextPart, ToolCallPart, ToolResultPart};
use crate::shared::provider_options::ProviderOptions;
use serde::{Deserialize, Serialize};

/// Content parts that can appear in an assistant message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantMessagePart {
    /// Text content
    Text(TextPart),

    /// File content
    File(FilePart),

    /// Reasoning content
    Reasoning(ReasoningPart),

    /// Tool call
    ToolCall(ToolCallPart),

    /// Tool result
    ToolResult(ToolResultPart),
}

/// Assistant message struct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assistant {
    #[serde(rename = "role")]
    pub message_role: AssistantRole,

    /// Array of content parts (text, files, reasoning, tool calls, tool results)
    pub content: Vec<AssistantMessagePart>,

    /// Additional provider-specific options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<ProviderOptions>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "assistant")]
pub(crate) struct AssistantRole;

impl Assistant {
    pub fn new(content: Vec<AssistantMessagePart>) -> Self {
        Self {
            message_role: AssistantRole,
            content,
            provider_options: None,
        }
    }

    pub fn with_options(
        content: Vec<AssistantMessagePart>,
        provider_options: Option<ProviderOptions>,
    ) -> Self {
        Self {
            message_role: AssistantRole,
            content,
            provider_options,
        }
    }

    /// Create an assistant message with text
    pub fn text(text: impl Into<String>) -> Self {
        Self::new(vec![AssistantMessagePart::Text(TextPart::new(text))])
    }
}
