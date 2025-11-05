use serde::{Deserialize, Serialize};
use serde_json::Value;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
