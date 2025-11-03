pub mod assistant;
pub mod data_content;
pub mod system;
pub mod tool;
pub mod user;

pub use assistant::{Assistant, AssistantMessagePart};
pub use data_content::DataContent;
pub use system::System;
pub use tool::{Tool, ToolResultContentItem, ToolResultOutput, ToolResultPart};
pub use user::{User, UserMessagePart};

use serde::{Deserialize, Serialize};

/// A message in a prompt with role-specific content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    /// System message with text content
    System(System),

    /// User message with text and/or file parts
    User(User),

    /// Assistant message with various content types
    Assistant(Assistant),

    /// Tool message with tool results
    Tool(Tool),
}

// Helper implementations for Message
impl Message {
    /// Create a system message
    pub fn system(content: impl Into<String>) -> Self {
        Self::System(System::new(content))
    }

    /// Create a user message with text
    pub fn user_text(text: impl Into<String>) -> Self {
        Self::User(User::text(text))
    }

    /// Create an assistant message with text
    pub fn assistant_text(text: impl Into<String>) -> Self {
        Self::Assistant(Assistant::text(text))
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
