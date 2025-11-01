pub mod assistant;
pub mod data_content;
pub mod system;
pub mod tool;
pub mod user;

pub use assistant::{AssistantMessage, AssistantMessagePart};
pub use data_content::DataContent;
pub use system::SystemMessage;
pub use tool::{ToolMessage, ToolResultContentItem, ToolResultOutput, ToolResultPart};
pub use user::{UserMessage, UserMessagePart};

use crate::shared::provider_options::ProviderOptions;
use serde::{Deserialize, Serialize};

/// A message in a prompt with role-specific content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum Message {
    /// System message with text content
    #[serde(rename_all = "camelCase")]
    System {
        /// The system message content
        content: String,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// User message with text and/or file parts
    #[serde(rename_all = "camelCase")]
    User {
        /// Array of text or file content parts
        content: Vec<UserMessagePart>,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// Assistant message with various content types
    #[serde(rename_all = "camelCase")]
    Assistant {
        /// Array of content parts (text, files, reasoning, tool calls, tool results)
        content: Vec<AssistantMessagePart>,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },

    /// Tool message with tool results
    #[serde(rename_all = "camelCase")]
    Tool {
        /// Array of tool result parts
        content: Vec<ToolResultPart>,

        /// Additional provider-specific options
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_options: Option<ProviderOptions>,
    },
}

// Helper implementations for Message
impl Message {
    /// Create a system message
    pub fn system(content: impl Into<String>) -> Self {
        Self::System {
            content: content.into(),
            provider_options: None,
        }
    }

    /// Create a user message with text
    pub fn user_text(text: impl Into<String>) -> Self {
        Self::User {
            content: vec![UserMessagePart::Text {
                text: text.into(),
                provider_options: None,
            }],
            provider_options: None,
        }
    }

    /// Create an assistant message with text
    pub fn assistant_text(text: impl Into<String>) -> Self {
        Self::Assistant {
            content: vec![AssistantMessagePart::Text {
                text: text.into(),
                provider_options: None,
            }],
            provider_options: None,
        }
    }

    /// Get the role of this message
    pub fn role(&self) -> &str {
        match self {
            Self::System { .. } => "system",
            Self::User { .. } => "user",
            Self::Assistant { .. } => "assistant",
            Self::Tool { .. } => "tool",
        }
    }
}
