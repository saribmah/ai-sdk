use super::{MessageMetadata, MessageRole};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A user message in a conversation session.
///
/// User messages represent input from the user and can contain multiple parts
/// such as text, images, and files.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserMessage {
    /// Unique message identifier
    pub id: String,

    /// Session this message belongs to
    pub session_id: String,

    /// Role (always User)
    pub role: MessageRole,

    /// IDs of parts that comprise this message
    pub part_ids: Vec<String>,

    /// Message metadata
    pub metadata: MessageMetadata,

    /// When this message was created
    pub created_at: DateTime<Utc>,
}

impl UserMessage {
    /// Create a new user message.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the message
    /// * `session_id` - ID of the session this message belongs to
    /// * `part_ids` - IDs of the message parts
    pub fn new(id: String, session_id: String, part_ids: Vec<String>) -> Self {
        Self {
            id,
            session_id,
            role: MessageRole::User,
            part_ids,
            metadata: MessageMetadata::default(),
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_message_creation() {
        let message = UserMessage::new(
            "msg-123".to_string(),
            "session-456".to_string(),
            vec!["part-1".to_string(), "part-2".to_string()],
        );

        assert_eq!(message.id, "msg-123");
        assert_eq!(message.session_id, "session-456");
        assert_eq!(message.role, MessageRole::User);
        assert_eq!(message.part_ids.len(), 2);
    }

    #[test]
    fn test_user_message_serialization() {
        let message = UserMessage::new(
            "msg-123".to_string(),
            "session-456".to_string(),
            vec!["part-1".to_string()],
        );

        let json = serde_json::to_string(&message).unwrap();
        let deserialized: UserMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(message.id, deserialized.id);
        assert_eq!(message.session_id, deserialized.session_id);
        assert_eq!(message.role, deserialized.role);
    }
}
