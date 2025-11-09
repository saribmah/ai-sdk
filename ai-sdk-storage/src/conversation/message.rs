//! Message types for conversation storage.

use super::metadata::MessageMetadata;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The role of the message sender.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    /// Message from the user
    User,
    /// Message from the AI assistant
    Assistant,
    /// System message (instructions, context)
    System,
    /// Tool result message
    Tool,
}

/// A stored conversation message.
///
/// This type represents a single message in a conversation, including its content,
/// metadata, and timing information.
///
/// # Example
///
/// ```rust
/// use ai_sdk_storage::{StoredMessage, MessageRole, MessageMetadata};
/// use chrono::Utc;
///
/// let message = StoredMessage {
///     id: "msg-123".to_string(),
///     session_id: "session-456".to_string(),
///     role: MessageRole::User,
///     content: serde_json::json!({"text": "Hello, AI!"}),
///     metadata: MessageMetadata::default(),
///     created_at: Utc::now(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredMessage {
    /// Unique identifier for this message
    pub id: String,

    /// ID of the session this message belongs to
    pub session_id: String,

    /// The role of the message sender
    pub role: MessageRole,

    /// The message content (flexible JSON structure)
    ///
    /// This can contain text, images, tool calls, or any other content
    /// supported by the AI model. The structure is intentionally flexible
    /// to accommodate different content types.
    pub content: serde_json::Value,

    /// Metadata associated with this message
    pub metadata: MessageMetadata,

    /// When this message was created
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_role_serialization() {
        let role = MessageRole::User;
        let json = serde_json::to_string(&role).unwrap();
        assert_eq!(json, r#""user""#);

        let role = MessageRole::Assistant;
        let json = serde_json::to_string(&role).unwrap();
        assert_eq!(json, r#""assistant""#);
    }

    #[test]
    fn test_message_role_deserialization() {
        let role: MessageRole = serde_json::from_str(r#""user""#).unwrap();
        assert_eq!(role, MessageRole::User);

        let role: MessageRole = serde_json::from_str(r#""assistant""#).unwrap();
        assert_eq!(role, MessageRole::Assistant);
    }

    #[test]
    fn test_stored_message_serialization() {
        let message = StoredMessage {
            id: "msg-1".to_string(),
            session_id: "session-1".to_string(),
            role: MessageRole::User,
            content: serde_json::json!({"text": "Hello"}),
            metadata: MessageMetadata::default(),
            created_at: Utc::now(),
        };

        let json = serde_json::to_string(&message).unwrap();
        let deserialized: StoredMessage = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, message.id);
        assert_eq!(deserialized.session_id, message.session_id);
        assert_eq!(deserialized.role, message.role);
    }
}
