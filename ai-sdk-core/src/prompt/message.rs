pub mod assistant;
pub mod content_parts;
pub mod data_content;
pub mod system;
pub mod tool;
pub mod user;

pub use assistant::{AssistantContent, AssistantContentPart, AssistantModelMessage};
pub use content_parts::{
    FileId, FilePart, FileSource, ImagePart, ImageSource, ReasoningPart, TextPart, ToolCallPart,
    ToolResultContentPart, ToolResultOutput, ToolResultPart,
};
pub use data_content::DataContent;
pub use system::SystemModelMessage;
pub use tool::{
    execute_tool, Tool, ToolApprovalRequest, ToolApprovalResponse, ToolCall, ToolCallOptions,
    ToolContent, ToolContentPart, ToolExecuteFunction, ToolExecutionEvent, ToolExecutionOutput,
    ToolModelMessage, ToolNeedsApprovalFunction, ToolResult, ToolType,
};
pub use user::{UserContent, UserContentPart, UserModelMessage};

use serde::{Deserialize, Deserializer, Serialize};

/// A message that can be used in the `messages` field of a prompt.
/// It can be a user message, an assistant message, a system message, or a tool message.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ModelMessage {
    /// System message containing instructions or context.
    System(SystemModelMessage),

    /// User message containing the user's input.
    User(UserModelMessage),

    /// Assistant message containing the AI's response.
    Assistant(AssistantModelMessage),

    /// Tool message containing tool execution results.
    Tool(ToolModelMessage),
}

impl<'de> Deserialize<'de> for ModelMessage {
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
                .map(ModelMessage::System)
                .map_err(D::Error::custom),
            "user" => serde_json::from_value(value)
                .map(ModelMessage::User)
                .map_err(D::Error::custom),
            "assistant" => serde_json::from_value(value)
                .map(ModelMessage::Assistant)
                .map_err(D::Error::custom),
            "tool" => serde_json::from_value(value)
                .map(ModelMessage::Tool)
                .map_err(D::Error::custom),
            _ => Err(D::Error::unknown_variant(
                role,
                &["system", "user", "assistant", "tool"],
            )),
        }
    }
}

impl ModelMessage {
    /// Returns the role of this message.
    pub fn role(&self) -> &str {
        match self {
            ModelMessage::System(_) => "system",
            ModelMessage::User(_) => "user",
            ModelMessage::Assistant(_) => "assistant",
            ModelMessage::Tool(_) => "tool",
        }
    }

    /// Returns true if this is a system message.
    pub fn is_system(&self) -> bool {
        matches!(self, ModelMessage::System(_))
    }

    /// Returns true if this is a user message.
    pub fn is_user(&self) -> bool {
        matches!(self, ModelMessage::User(_))
    }

    /// Returns true if this is an assistant message.
    pub fn is_assistant(&self) -> bool {
        matches!(self, ModelMessage::Assistant(_))
    }

    /// Returns true if this is a tool message.
    pub fn is_tool(&self) -> bool {
        matches!(self, ModelMessage::Tool(_))
    }
}

impl From<SystemModelMessage> for ModelMessage {
    fn from(message: SystemModelMessage) -> Self {
        ModelMessage::System(message)
    }
}

impl From<UserModelMessage> for ModelMessage {
    fn from(message: UserModelMessage) -> Self {
        ModelMessage::User(message)
    }
}

impl From<AssistantModelMessage> for ModelMessage {
    fn from(message: AssistantModelMessage) -> Self {
        ModelMessage::Assistant(message)
    }
}

impl From<ToolModelMessage> for ModelMessage {
    fn from(message: ToolModelMessage) -> Self {
        ModelMessage::Tool(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_message_system() {
        let system_msg = SystemModelMessage::new("You are a helpful assistant.");
        let message = ModelMessage::from(system_msg);

        assert_eq!(message.role(), "system");
        assert!(message.is_system());
        assert!(!message.is_user());
        assert!(!message.is_assistant());
        assert!(!message.is_tool());
    }

    #[test]
    fn test_model_message_user() {
        let user_msg = UserModelMessage::new("Hello!");
        let message = ModelMessage::from(user_msg);

        assert_eq!(message.role(), "user");
        assert!(!message.is_system());
        assert!(message.is_user());
        assert!(!message.is_assistant());
        assert!(!message.is_tool());
    }

    #[test]
    fn test_model_message_assistant() {
        let assistant_msg = AssistantModelMessage::new("Hello, how can I help you?");
        let message = ModelMessage::from(assistant_msg);

        assert_eq!(message.role(), "assistant");
        assert!(!message.is_system());
        assert!(!message.is_user());
        assert!(message.is_assistant());
        assert!(!message.is_tool());
    }

    #[test]
    fn test_model_message_tool() {
        let tool_msg = ToolModelMessage::new(vec![ToolContentPart::ToolResult(
            ToolResultPart::new("call_123", "tool_name", ToolResultOutput::text("Success")),
        )]);
        let message = ModelMessage::from(tool_msg);

        assert_eq!(message.role(), "tool");
        assert!(!message.is_system());
        assert!(!message.is_user());
        assert!(!message.is_assistant());
        assert!(message.is_tool());
    }

    #[test]
    fn test_model_message_serialization_system() {
        let system_msg = SystemModelMessage::new("You are helpful.");
        let message = ModelMessage::from(system_msg);

        let serialized = serde_json::to_value(&message).unwrap();

        assert_eq!(serialized["role"], "system");
        assert_eq!(serialized["content"], "You are helpful.");
    }

    #[test]
    fn test_model_message_serialization_user() {
        let user_msg = UserModelMessage::new("Hello!");
        let message = ModelMessage::from(user_msg);

        let serialized = serde_json::to_value(&message).unwrap();

        assert_eq!(serialized["role"], "user");
        assert_eq!(serialized["content"], "Hello!");
    }

    #[test]
    fn test_model_message_serialization_assistant() {
        let assistant_msg = AssistantModelMessage::new("Hi there!");
        let message = ModelMessage::from(assistant_msg);

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

        let message: ModelMessage = serde_json::from_value(json).unwrap();

        assert_eq!(message.role(), "system");
        assert!(message.is_system());
    }

    #[test]
    fn test_model_message_deserialization_user() {
        let json = serde_json::json!({
            "role": "user",
            "content": "Hello!"
        });

        let message: ModelMessage = serde_json::from_value(json).unwrap();

        assert_eq!(message.role(), "user");
        assert!(message.is_user());
    }

    #[test]
    fn test_model_message_deserialization_assistant() {
        let json = serde_json::json!({
            "role": "assistant",
            "content": "Hi there!"
        });

        let message: ModelMessage = serde_json::from_value(json).unwrap();

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

        let message: ModelMessage = serde_json::from_value(json).unwrap();

        assert_eq!(message.role(), "tool");
        assert!(message.is_tool());
    }

    #[test]
    fn test_model_message_clone() {
        let system_msg = SystemModelMessage::new("You are helpful.");
        let message = ModelMessage::from(system_msg);
        let cloned = message.clone();

        assert_eq!(message, cloned);
    }

    #[test]
    fn test_model_message_conversation() {
        let messages = vec![
            ModelMessage::from(SystemModelMessage::new("You are helpful.")),
            ModelMessage::from(UserModelMessage::new("Hello!")),
            ModelMessage::from(AssistantModelMessage::new("Hi! How can I help?")),
        ];

        assert_eq!(messages.len(), 3);
        assert!(messages[0].is_system());
        assert!(messages[1].is_user());
        assert!(messages[2].is_assistant());
    }
}
