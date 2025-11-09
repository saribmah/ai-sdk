/// Assistant message types
pub mod assistant;
/// Data content types for messages
pub mod data_content;
/// Message part types (text, file, tool calls, etc.)
pub mod parts;
/// System message types
pub mod system;
/// Tool message types
pub mod tool;
/// User message types
pub mod user;

pub use assistant::{LanguageModelAssistantMessage, LanguageModelAssistantMessagePart};
pub use data_content::LanguageModelDataContent;
pub use parts::{
    LanguageModelFilePart, LanguageModelReasoningPart, LanguageModelTextPart,
    LanguageModelToolCallPart, LanguageModelToolResultContentItem, LanguageModelToolResultOutput,
    LanguageModelToolResultPart,
};
pub use system::LanguageModelSystemMessage;
pub use tool::LanguageModelToolMessage;
pub use user::{LanguageModelUserMessage, LanguageModelUserMessagePart};

use serde::{Deserialize, Serialize};

/// A message in a prompt with role-specific content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LanguageModelMessage {
    /// System message with text content
    System(LanguageModelSystemMessage),

    /// User message with text and/or file parts
    User(LanguageModelUserMessage),

    /// Assistant message with various content types
    Assistant(LanguageModelAssistantMessage),

    /// Tool message with tool results
    Tool(LanguageModelToolMessage),
}

// Helper implementations for Message
impl LanguageModelMessage {
    /// Create a system message
    pub fn system(content: impl Into<String>) -> Self {
        Self::System(LanguageModelSystemMessage::new(content))
    }

    /// Create a user message with text
    pub fn user_text(text: impl Into<String>) -> Self {
        Self::User(LanguageModelUserMessage::text(text))
    }

    /// Create an assistant message with text
    pub fn assistant_text(text: impl Into<String>) -> Self {
        Self::Assistant(LanguageModelAssistantMessage::text(text))
    }

    /// Get the role of this message
    pub fn role(&self) -> &str {
        match self {
            Self::System(_) => "system",
            Self::User(_) => "user",
            Self::Assistant(_) => "assistant",
            Self::Tool(_) => "tool",
        }
    }
}
