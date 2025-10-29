use serde::{Deserialize, Serialize};
use serde_json::Value;

/// OpenAI-compatible chat prompt (array of messages)
pub type OpenAICompatibleChatPrompt = Vec<OpenAICompatibleMessage>;

/// OpenAI-compatible message types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum OpenAICompatibleMessage {
    /// System message
    System(OpenAICompatibleSystemMessage),

    /// User message
    User(OpenAICompatibleUserMessage),

    /// Assistant message
    Assistant(OpenAICompatibleAssistantMessage),

    /// Tool message
    Tool(OpenAICompatibleToolMessage),
}

/// System message with text content
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAICompatibleSystemMessage {
    /// The system message content
    pub content: String,

    /// Additional properties for extensibility (flattened into the struct)
    #[serde(flatten)]
    pub additional_properties: Option<Value>,
}

/// User message with text or multimodal content
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAICompatibleUserMessage {
    /// The user message content (can be a string or array of content parts)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<UserMessageContent>,

    /// Additional properties for extensibility (flattened into the struct)
    #[serde(flatten)]
    pub additional_properties: Option<Value>,
}

/// User message content can be either a simple string or an array of content parts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserMessageContent {
    /// Simple text content
    Text(String),

    /// Array of content parts (text, images, etc.)
    Parts(Vec<OpenAICompatibleContentPart>),
}

/// Content part in a user message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OpenAICompatibleContentPart {
    /// Text content part
    Text(OpenAICompatibleContentPartText),

    /// Image URL content part
    ImageUrl(OpenAICompatibleContentPartImage),
}

/// Text content part
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAICompatibleContentPartText {
    /// The text content
    pub text: String,

    /// Additional properties for extensibility (flattened into the struct)
    #[serde(flatten)]
    pub additional_properties: Option<Value>,
}

/// Image URL content part
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAICompatibleContentPartImage {
    /// Image URL information
    pub image_url: ImageUrl,

    /// Additional properties for extensibility (flattened into the struct)
    #[serde(flatten)]
    pub additional_properties: Option<Value>,
}

/// Image URL information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageUrl {
    /// The URL of the image
    pub url: String,
}

/// Assistant message with optional content and tool calls
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAICompatibleAssistantMessage {
    /// The assistant message content (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Tool calls made by the assistant (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<OpenAICompatibleMessageToolCall>>,

    /// Additional properties for extensibility (flattened into the struct)
    #[serde(flatten)]
    pub additional_properties: Option<Value>,
}

/// Tool call in an assistant message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAICompatibleMessageToolCall {
    /// The type of tool call (always "function" for OpenAI)
    #[serde(rename = "type")]
    pub tool_type: String,

    /// The ID of the tool call
    pub id: String,

    /// Function call details
    pub function: FunctionCall,

    /// Additional properties for extensibility (flattened into the struct)
    #[serde(flatten)]
    pub additional_properties: Option<Value>,
}

/// Function call details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionCall {
    /// The name of the function being called
    pub name: String,

    /// The arguments for the function (as a JSON string)
    pub arguments: String,
}

/// Tool message with the result of a tool call
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAICompatibleToolMessage {
    /// The content of the tool response
    pub content: String,

    /// The ID of the tool call this is responding to
    pub tool_call_id: String,

    /// Additional properties for extensibility (flattened into the struct)
    #[serde(flatten)]
    pub additional_properties: Option<Value>,
}

impl OpenAICompatibleSystemMessage {
    /// Creates a new system message
    pub fn new(content: String) -> Self {
        Self {
            content,
            additional_properties: None,
        }
    }
}

impl OpenAICompatibleUserMessage {
    /// Creates a new user message with text content
    pub fn new_text(content: String) -> Self {
        Self {
            content: Some(UserMessageContent::Text(content)),
            additional_properties: None,
        }
    }

    /// Creates a new user message with content parts
    pub fn new_parts(parts: Vec<OpenAICompatibleContentPart>) -> Self {
        Self {
            content: Some(UserMessageContent::Parts(parts)),
            additional_properties: None,
        }
    }
}

impl OpenAICompatibleContentPartText {
    /// Creates a new text content part
    pub fn new(text: String) -> Self {
        Self {
            text,
            additional_properties: None,
        }
    }
}

impl OpenAICompatibleContentPartImage {
    /// Creates a new image content part
    pub fn new(url: String) -> Self {
        Self {
            image_url: ImageUrl { url },
            additional_properties: None,
        }
    }
}

impl OpenAICompatibleAssistantMessage {
    /// Creates a new assistant message with text content
    pub fn new_text(content: String) -> Self {
        Self {
            content: Some(content),
            tool_calls: None,
            additional_properties: None,
        }
    }

    /// Creates a new assistant message with tool calls
    pub fn new_tool_calls(tool_calls: Vec<OpenAICompatibleMessageToolCall>) -> Self {
        Self {
            content: None,
            tool_calls: Some(tool_calls),
            additional_properties: None,
        }
    }

    /// Creates a new assistant message with both content and tool calls
    pub fn new(
        content: Option<String>,
        tool_calls: Option<Vec<OpenAICompatibleMessageToolCall>>,
    ) -> Self {
        Self {
            content,
            tool_calls,
            additional_properties: None,
        }
    }
}

impl OpenAICompatibleMessageToolCall {
    /// Creates a new function tool call
    pub fn new(id: String, name: String, arguments: String) -> Self {
        Self {
            tool_type: "function".to_string(),
            id,
            function: FunctionCall { name, arguments },
            additional_properties: None,
        }
    }
}

impl OpenAICompatibleToolMessage {
    /// Creates a new tool message
    pub fn new(content: String, tool_call_id: String) -> Self {
        Self {
            content,
            tool_call_id,
            additional_properties: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_message() {
        let msg = OpenAICompatibleSystemMessage::new("You are helpful".to_string());
        assert_eq!(msg.content, "You are helpful");
    }

    #[test]
    fn test_user_message_text() {
        let msg = OpenAICompatibleUserMessage::new_text("Hello".to_string());
        match msg.content {
            Some(UserMessageContent::Text(text)) => assert_eq!(text, "Hello"),
            _ => panic!("Expected text content"),
        }
    }

    #[test]
    fn test_user_message_parts() {
        let parts = vec![OpenAICompatibleContentPart::Text(
            OpenAICompatibleContentPartText::new("Hello".to_string()),
        )];
        let msg = OpenAICompatibleUserMessage::new_parts(parts);
        match msg.content {
            Some(UserMessageContent::Parts(parts)) => assert_eq!(parts.len(), 1),
            _ => panic!("Expected parts content"),
        }
    }

    #[test]
    fn test_assistant_message_text() {
        let msg = OpenAICompatibleAssistantMessage::new_text("Response".to_string());
        assert_eq!(msg.content, Some("Response".to_string()));
        assert!(msg.tool_calls.is_none());
    }

    #[test]
    fn test_assistant_message_tool_calls() {
        let tool_call = OpenAICompatibleMessageToolCall::new(
            "call_123".to_string(),
            "get_weather".to_string(),
            r#"{"city":"SF"}"#.to_string(),
        );
        let msg = OpenAICompatibleAssistantMessage::new_tool_calls(vec![tool_call]);
        assert!(msg.content.is_none());
        assert_eq!(msg.tool_calls.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_tool_message() {
        let msg = OpenAICompatibleToolMessage::new("Result".to_string(), "call_123".to_string());
        assert_eq!(msg.content, "Result");
        assert_eq!(msg.tool_call_id, "call_123");
    }

    #[test]
    fn test_message_serialization() {
        let msg =
            OpenAICompatibleMessage::System(OpenAICompatibleSystemMessage::new("Test".to_string()));
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains(r#""role":"system"#));
        assert!(json.contains(r#""content":"Test"#));
    }

    #[test]
    fn test_message_deserialization() {
        let json = r#"{"role":"system","content":"Test"}"#;
        let msg: OpenAICompatibleMessage = serde_json::from_str(json).unwrap();
        match msg {
            OpenAICompatibleMessage::System(sys) => assert_eq!(sys.content, "Test"),
            _ => panic!("Expected system message"),
        }
    }
}
