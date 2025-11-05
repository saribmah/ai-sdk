mod system;
mod user;
mod assistant;
mod tool;

pub use system::OpenAICompatibleSystemMessage;
pub use user::{
    ImageUrl, OpenAICompatibleContentPart, OpenAICompatibleContentPartImage,
    OpenAICompatibleContentPartText, OpenAICompatibleUserMessage, UserMessageContent,
};
pub use assistant::{
    FunctionCall, OpenAICompatibleAssistantMessage, OpenAICompatibleMessageToolCall,
};
pub use tool::OpenAICompatibleToolMessage;

use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use super::*;

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
