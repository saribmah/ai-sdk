use serde::{Deserialize, Serialize};
use serde_json::Value;

/// System message with text content
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAICompatibleSystemMessage {
    /// The system message content
    pub content: String,

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_message() {
        let msg = OpenAICompatibleSystemMessage::new("You are helpful".to_string());
        assert_eq!(msg.content, "You are helpful");
    }
}
