pub mod assistant;
pub mod cache_control;
pub mod content;
pub mod metadata;
pub mod user;

use serde::{Deserialize, Deserializer, Serialize};

use self::assistant::AnthropicAssistantMessage;
use self::user::AnthropicUserMessage;

/// Anthropic message type.
///
/// Represents a message in a conversation, which can be either from the user or the assistant.
/// Messages are discriminated by the `role` field in the underlying message struct.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::prompt::message::{AnthropicMessage, user::AnthropicUserMessage};
/// use ai_sdk_anthropic::prompt::message::content::text::AnthropicTextContent;
///
/// // Create a user message
/// let user_msg = AnthropicMessage::User(
///     AnthropicUserMessage::from_text("Hello, Claude!")
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum AnthropicMessage {
    /// User message
    User(AnthropicUserMessage),

    /// Assistant message
    Assistant(AnthropicAssistantMessage),
}

impl<'de> Deserialize<'de> for AnthropicMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        use serde_json::Value;

        let value = Value::deserialize(deserializer)?;

        // Check the role field
        let role = value
            .get("role")
            .and_then(|r| r.as_str())
            .ok_or_else(|| D::Error::missing_field("role"))?;

        match role {
            "user" => {
                let msg = serde_json::from_value(value).map_err(|e| {
                    D::Error::custom(format!("failed to deserialize user message: {}", e))
                })?;
                Ok(AnthropicMessage::User(msg))
            }
            "assistant" => {
                let msg = serde_json::from_value(value).map_err(|e| {
                    D::Error::custom(format!("failed to deserialize assistant message: {}", e))
                })?;
                Ok(AnthropicMessage::Assistant(msg))
            }
            _ => Err(D::Error::custom(format!("unknown role: {}", role))),
        }
    }
}

impl From<AnthropicUserMessage> for AnthropicMessage {
    fn from(message: AnthropicUserMessage) -> Self {
        Self::User(message)
    }
}

impl From<AnthropicAssistantMessage> for AnthropicMessage {
    fn from(message: AnthropicAssistantMessage) -> Self {
        Self::Assistant(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_message_from_user() {
        let user_msg = AnthropicUserMessage::from_text("Hello");
        let message: AnthropicMessage = user_msg.into();

        match message {
            AnthropicMessage::User(_) => (),
            _ => panic!("Expected User variant"),
        }
    }

    #[test]
    fn test_message_from_assistant() {
        let assistant_msg = AnthropicAssistantMessage::text("Hi there");
        let message: AnthropicMessage = assistant_msg.into();

        match message {
            AnthropicMessage::Assistant(_) => (),
            _ => panic!("Expected Assistant variant"),
        }
    }

    #[test]
    fn test_serialize_user_message() {
        let message = AnthropicMessage::User(AnthropicUserMessage::from_text("Hello, Claude!"));
        let json = serde_json::to_value(&message).unwrap();

        assert_eq!(
            json,
            json!({
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Hello, Claude!"
                    }
                ]
            })
        );
    }

    #[test]
    fn test_serialize_assistant_message() {
        let message = AnthropicMessage::Assistant(AnthropicAssistantMessage::text("I can help!"));
        let json = serde_json::to_value(&message).unwrap();

        assert_eq!(
            json,
            json!({
                "role": "assistant",
                "content": [
                    {
                        "type": "text",
                        "text": "I can help!"
                    }
                ]
            })
        );
    }

    #[test]
    fn test_deserialize_user_message() {
        let json = json!({
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Hello, Claude!"
                }
            ]
        });

        let message: AnthropicMessage = serde_json::from_value(json).unwrap();

        match message {
            AnthropicMessage::User(msg) => {
                assert_eq!(msg.role, "user");
                assert_eq!(msg.content.len(), 1);
            }
            _ => panic!("Expected User variant"),
        }
    }

    #[test]
    fn test_deserialize_assistant_message() {
        let json = json!({
            "role": "assistant",
            "content": [
                {
                        "type": "text",
                        "text": "I can help!"
                }
            ]
        });

        let message: AnthropicMessage = serde_json::from_value(json).unwrap();

        match message {
            AnthropicMessage::Assistant(msg) => {
                assert_eq!(msg.role, "assistant");
                assert_eq!(msg.content.len(), 1);
            }
            _ => panic!("Expected Assistant variant"),
        }
    }

    #[test]
    fn test_roundtrip_user_message() {
        let original = AnthropicMessage::User(AnthropicUserMessage::from_text("Test message"));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicMessage = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_roundtrip_assistant_message() {
        let original =
            AnthropicMessage::Assistant(AnthropicAssistantMessage::text("Test response"));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicMessage = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_equality() {
        let msg1 = AnthropicMessage::User(AnthropicUserMessage::from_text("Hello"));
        let msg2 = AnthropicMessage::User(AnthropicUserMessage::from_text("Hello"));
        let msg3 = AnthropicMessage::Assistant(AnthropicAssistantMessage::text("Hello"));

        assert_eq!(msg1, msg2);
        assert_ne!(msg1, msg3);
    }

    #[test]
    fn test_clone() {
        let msg1 = AnthropicMessage::User(AnthropicUserMessage::from_text("Hello"));
        let msg2 = msg1.clone();

        assert_eq!(msg1, msg2);
    }
}
