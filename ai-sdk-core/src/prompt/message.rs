pub mod assistant;
pub mod content_parts;
pub mod data_content;
pub mod system;
pub mod tool;
pub mod user;

pub use assistant::{AssistantContent, AssistantContentPart, AssistantMessage};
pub use content_parts::{
    FileId, FilePart, FileSource, ImagePart, ImageSource, ReasoningPart, TextPart, ToolCallPart,
    ToolResultContentPart, ToolResultOutput, ToolResultPart,
};
pub use data_content::DataContent;
pub use system::SystemMessage;
pub use tool::{ToolContent, ToolContentPart, ToolMessage};
pub use user::{UserContent, UserContentPart, UserMessage};

use serde::{Deserialize, Deserializer, Serialize};

/// A message that can be used in the `messages` field of a prompt.
/// It can be a user message, an assistant message, a system message, or a tool message.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Message {
    /// System message containing instructions or context.
    System(SystemMessage),

    /// User message containing the user's input.
    User(UserMessage),

    /// Assistant message containing the AI's response.
    Assistant(AssistantMessage),

    /// Tool message containing tool execution results.
    Tool(ToolMessage),
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        use serde_json::Value;

        let value = Value::deserialize(deserializer)?;

        // Extract the role field to determine which variant to deserialize into
        let role = value
            .get("role")
            .and_then(|v| v.as_str())
            .ok_or_else(|| D::Error::missing_field("role"))?;

        match role {
            "system" => serde_json::from_value(value)
                .map(Message::System)
                .map_err(D::Error::custom),
            "user" => serde_json::from_value(value)
                .map(Message::User)
                .map_err(D::Error::custom),
            "assistant" => serde_json::from_value(value)
                .map(Message::Assistant)
                .map_err(D::Error::custom),
            "tool" => serde_json::from_value(value)
                .map(Message::Tool)
                .map_err(D::Error::custom),
            _ => Err(D::Error::unknown_variant(
                role,
                &["system", "user", "assistant", "tool"],
            )),
        }
    }
}

impl Message {
    /// Returns the role of this message.
    pub fn role(&self) -> &str {
        match self {
            Message::System(_) => "system",
            Message::User(_) => "user",
            Message::Assistant(_) => "assistant",
            Message::Tool(_) => "tool",
        }
    }

    /// Returns true if this is a system message.
    pub fn is_system(&self) -> bool {
        matches!(self, Message::System(_))
    }

    /// Returns true if this is a user message.
    pub fn is_user(&self) -> bool {
        matches!(self, Message::User(_))
    }

    /// Returns true if this is an assistant message.
    pub fn is_assistant(&self) -> bool {
        matches!(self, Message::Assistant(_))
    }

    /// Returns true if this is a tool message.
    pub fn is_tool(&self) -> bool {
        matches!(self, Message::Tool(_))
    }
}

impl From<SystemMessage> for Message {
    fn from(message: SystemMessage) -> Self {
        Message::System(message)
    }
}

impl From<UserMessage> for Message {
    fn from(message: UserMessage) -> Self {
        Message::User(message)
    }
}

impl From<AssistantMessage> for Message {
    fn from(message: AssistantMessage) -> Self {
        Message::Assistant(message)
    }
}

impl From<ToolMessage> for Message {
    fn from(message: ToolMessage) -> Self {
        Message::Tool(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_message_system() {
        let system_msg = SystemMessage::new("You are a helpful assistant.");
        let message = Message::from(system_msg);

        assert_eq!(message.role(), "system");
        assert!(message.is_system());
        assert!(!message.is_user());
        assert!(!message.is_assistant());
        assert!(!message.is_tool());
    }

    #[test]
    fn test_model_message_user() {
        let user_msg = UserMessage::new("Hello!");
        let message = Message::from(user_msg);

        assert_eq!(message.role(), "user");
        assert!(!message.is_system());
        assert!(message.is_user());
        assert!(!message.is_assistant());
        assert!(!message.is_tool());
    }

    #[test]
    fn test_model_message_assistant() {
        let assistant_msg = AssistantMessage::new("Hello, how can I help you?");
        let message = Message::from(assistant_msg);

        assert_eq!(message.role(), "assistant");
        assert!(!message.is_system());
        assert!(!message.is_user());
        assert!(message.is_assistant());
        assert!(!message.is_tool());
    }

    #[test]
    fn test_model_message_tool() {
        let tool_msg = ToolMessage::new(vec![ToolContentPart::ToolResult(ToolResultPart::new(
            "call_123",
            "tool_name",
            ToolResultOutput::text("Success"),
        ))]);
        let message = Message::from(tool_msg);

        assert_eq!(message.role(), "tool");
        assert!(!message.is_system());
        assert!(!message.is_user());
        assert!(!message.is_assistant());
        assert!(message.is_tool());
    }

    #[test]
    fn test_model_message_serialization_system() {
        let system_msg = SystemMessage::new("You are helpful.");
        let message = Message::from(system_msg);

        let serialized = serde_json::to_value(&message).unwrap();

        assert_eq!(serialized["role"], "system");
        assert_eq!(serialized["content"], "You are helpful.");
    }

    #[test]
    fn test_model_message_serialization_user() {
        let user_msg = UserMessage::new("Hello!");
        let message = Message::from(user_msg);

        let serialized = serde_json::to_value(&message).unwrap();

        assert_eq!(serialized["role"], "user");
        assert_eq!(serialized["content"], "Hello!");
    }

    #[test]
    fn test_model_message_serialization_assistant() {
        let assistant_msg = AssistantMessage::new("Hi there!");
        let message = Message::from(assistant_msg);

        let serialized = serde_json::to_value(&message).unwrap();

        assert_eq!(serialized["role"], "assistant");
        assert_eq!(serialized["content"], "Hi there!");
    }

    #[test]
    fn test_model_message_deserialization_system() {
        let json = serde_json::json!({
            "role": "system",
            "content": "You are helpful."
        });

        let message: Message = serde_json::from_value(json).unwrap();

        assert_eq!(message.role(), "system");
        assert!(message.is_system());
    }

    #[test]
    fn test_model_message_deserialization_user() {
        let json = serde_json::json!({
            "role": "user",
            "content": "Hello!"
        });

        let message: Message = serde_json::from_value(json).unwrap();

        assert_eq!(message.role(), "user");
        assert!(message.is_user());
    }

    #[test]
    fn test_model_message_deserialization_assistant() {
        let json = serde_json::json!({
            "role": "assistant",
            "content": "Hi there!"
        });

        let message: Message = serde_json::from_value(json).unwrap();

        assert_eq!(message.role(), "assistant");
        assert!(message.is_assistant());
    }

    #[test]
    fn test_model_message_deserialization_tool() {
        let json = serde_json::json!({
            "role": "tool",
            "content": [
                {
                    "type": "text",
                    "toolCallId": "call_123",
                    "toolName": "tool_name",
                    "output": {
                        "type": "text",
                        "value": "Success"
                    }
                }
            ]
        });

        let message: Message = serde_json::from_value(json).unwrap();

        assert_eq!(message.role(), "tool");
        assert!(message.is_tool());
    }

    #[test]
    fn test_model_message_clone() {
        let system_msg = SystemMessage::new("You are helpful.");
        let message = Message::from(system_msg);
        let cloned = message.clone();

        assert_eq!(message, cloned);
    }

    #[test]
    fn test_model_message_conversation() {
        let messages = vec![
            Message::from(SystemMessage::new("You are helpful.")),
            Message::from(UserMessage::new("Hello!")),
            Message::from(AssistantMessage::new("Hi! How can I help?")),
        ];

        assert_eq!(messages.len(), 3);
        assert!(messages[0].is_system());
        assert!(messages[1].is_user());
        assert!(messages[2].is_assistant());
    }
}
