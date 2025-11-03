use crate::prompt::message::{AssistantMessage, Message, ToolModelMessage};

/// A message that was generated during the generation process.
///
/// It can be either an assistant message or a tool message.
///
/// # Example
///
/// ```
/// use ai_sdk_core::ResponseMessage;
/// use ai_sdk_core::message::AssistantModelMessage;
///
/// // Create an assistant response message
/// let assistant_msg = AssistantModelMessage::new("Hello, how can I help you?");
/// let response_msg = ResponseMessage::Assistant(assistant_msg);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ResponseMessage {
    /// An assistant model message.
    Assistant(AssistantMessage),

    /// A tool model message.
    Tool(ToolModelMessage),
}

impl ResponseMessage {
    /// Creates a new `ResponseMessage` from an `AssistantModelMessage`.
    ///
    /// # Arguments
    ///
    /// * `message` - The assistant model message
    ///
    /// # Example
    ///
    /// ```ignore
    /// use ai_sdk_core::{ResponseMessage, AssistantModelMessage};
    ///
    /// let assistant_msg = AssistantModelMessage {
    ///     role: "assistant".to_string(),
    ///     content: vec![],
    /// };
    /// let response = ResponseMessage::from_assistant(assistant_msg);
    /// ```
    pub fn from_assistant(message: AssistantMessage) -> Self {
        ResponseMessage::Assistant(message)
    }

    /// Creates a new `ResponseMessage` from a `ToolModelMessage`.
    ///
    /// # Arguments
    ///
    /// * `message` - The tool model message
    ///
    /// # Example
    ///
    /// ```ignore
    /// use ai_sdk_core::{ResponseMessage, ToolModelMessage};
    ///
    /// let tool_msg = ToolModelMessage {
    ///     role: "tool".to_string(),
    ///     content: vec![],
    /// };
    /// let response = ResponseMessage::from_tool(tool_msg);
    /// ```
    pub fn from_tool(message: ToolModelMessage) -> Self {
        ResponseMessage::Tool(message)
    }

    /// Returns `true` if this is an assistant message.
    pub fn is_assistant(&self) -> bool {
        matches!(self, ResponseMessage::Assistant(_))
    }

    /// Returns `true` if this is a tool message.
    pub fn is_tool(&self) -> bool {
        matches!(self, ResponseMessage::Tool(_))
    }

    /// Returns a reference to the assistant message if this is an assistant message.
    pub fn as_assistant(&self) -> Option<&AssistantMessage> {
        match self {
            ResponseMessage::Assistant(msg) => Some(msg),
            _ => None,
        }
    }

    /// Returns a reference to the tool message if this is a tool message.
    pub fn as_tool(&self) -> Option<&ToolModelMessage> {
        match self {
            ResponseMessage::Tool(msg) => Some(msg),
            _ => None,
        }
    }
}

impl From<Message> for ResponseMessage {
    fn from(message: Message) -> Self {
        match message {
            Message::Assistant(msg) => ResponseMessage::Assistant(msg),
            Message::Tool(msg) => ResponseMessage::Tool(msg),
            Message::System(_) | Message::User(_) => {
                panic!("Cannot convert System or User messages to ResponseMessage")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_message_from_assistant() {
        let assistant_msg = AssistantMessage::new("Hello");
        let response = ResponseMessage::from_assistant(assistant_msg.clone());

        assert!(response.is_assistant());
        assert!(!response.is_tool());
        assert_eq!(response.as_assistant(), Some(&assistant_msg));
        assert_eq!(response.as_tool(), None);
    }

    #[test]
    fn test_response_message_from_tool() {
        use crate::prompt::message::content_parts::{ToolResultOutput, ToolResultPart};
        use crate::prompt::message::tool::ToolContentPart;

        let tool_msg =
            ToolModelMessage::new(vec![ToolContentPart::ToolResult(ToolResultPart::new(
                "call_123",
                "tool_name",
                ToolResultOutput::text("Tool result"),
            ))]);
        let response = ResponseMessage::from_tool(tool_msg.clone());

        assert!(response.is_tool());
        assert!(!response.is_assistant());
        assert_eq!(response.as_tool(), Some(&tool_msg));
        assert_eq!(response.as_assistant(), None);
    }

    #[test]
    fn test_response_message_assistant_variant() {
        let assistant_msg = AssistantMessage::new("Test message");
        let response = ResponseMessage::Assistant(assistant_msg.clone());

        match response {
            ResponseMessage::Assistant(msg) => assert_eq!(msg, assistant_msg),
            _ => panic!("Expected Assistant variant"),
        }
    }

    #[test]
    fn test_response_message_tool_variant() {
        let tool_msg = ToolModelMessage::new(vec![]);
        let response = ResponseMessage::Tool(tool_msg.clone());

        match response {
            ResponseMessage::Tool(msg) => assert_eq!(msg, tool_msg),
            _ => panic!("Expected Tool variant"),
        }
    }

    #[test]
    fn test_response_message_clone() {
        let assistant_msg = AssistantMessage::new("Test");
        let response = ResponseMessage::from_assistant(assistant_msg);
        let cloned = response.clone();

        assert_eq!(response, cloned);
    }

    #[test]
    fn test_from_model_message_assistant() {
        let assistant_msg = AssistantMessage::new("Hello");
        let model_msg = Message::Assistant(assistant_msg.clone());
        let response = ResponseMessage::from(model_msg);

        assert!(response.is_assistant());
        assert_eq!(response.as_assistant(), Some(&assistant_msg));
    }

    #[test]
    fn test_from_model_message_tool() {
        let tool_msg = ToolModelMessage::new(vec![]);
        let model_msg = Message::Tool(tool_msg.clone());
        let response = ResponseMessage::from(model_msg);

        assert!(response.is_tool());
        assert_eq!(response.as_tool(), Some(&tool_msg));
    }
}
