use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    fn test_tool_message() {
        let msg = OpenAICompatibleToolMessage::new("Result".to_string(), "call_123".to_string());
        assert_eq!(msg.content, "Result");
        assert_eq!(msg.tool_call_id, "call_123");
    }
}
