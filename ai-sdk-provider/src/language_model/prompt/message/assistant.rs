use super::parts::{
    LanguageModelFilePart, LanguageModelReasoningPart, LanguageModelTextPart,
    LanguageModelToolCallPart, LanguageModelToolResultPart,
};
use crate::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};

/// Content parts that can appear in an assistant message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LanguageModelAssistantMessagePart {
    /// Text content
    Text(LanguageModelTextPart),

    /// File content
    File(LanguageModelFilePart),

    /// Reasoning content
    Reasoning(LanguageModelReasoningPart),

    /// Tool call
    ToolCall(LanguageModelToolCallPart),

    /// Tool result
    ToolResult(LanguageModelToolResultPart),
}

/// Assistant message struct
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelAssistantMessage {
    pub role: AssistantRole,

    /// Array of content parts (text, files, reasoning, tool calls, tool results)
    pub content: Vec<LanguageModelAssistantMessagePart>,

    /// Additional provider-specific options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<SharedProviderOptions>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "assistant")]
pub struct AssistantRole;

impl LanguageModelAssistantMessage {
    pub fn new(content: Vec<LanguageModelAssistantMessagePart>) -> Self {
        Self {
            role: AssistantRole,
            content,
            provider_options: None,
        }
    }

    pub fn with_options(
        content: Vec<LanguageModelAssistantMessagePart>,
        provider_options: Option<SharedProviderOptions>,
    ) -> Self {
        Self {
            role: AssistantRole,
            content,
            provider_options,
        }
    }

    /// Create an assistant message with text
    pub fn text(text: impl Into<String>) -> Self {
        Self::new(vec![LanguageModelAssistantMessagePart::Text(
            LanguageModelTextPart::new(text),
        )])
    }
}
