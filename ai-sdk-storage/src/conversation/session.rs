//! Session types for conversation storage.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Metadata for a conversation session.
///
/// This type allows for extensible session-level metadata including
/// user identification, tags, and custom provider-specific data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionMetadata {
    /// Optional user identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Optional tags for categorizing sessions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    /// Custom metadata (provider-specific or application-specific)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<serde_json::Value>,
}

/// A conversation session.
///
/// A session represents a coherent conversation thread that may contain
/// multiple messages exchanged between the user and assistant.
///
/// # Example
///
/// ```rust
/// use ai_sdk_storage::{ConversationSession, SessionMetadata};
/// use chrono::Utc;
///
/// let session = ConversationSession {
///     id: "session-123".to_string(),
///     title: Some("Technical Discussion".to_string()),
///     metadata: SessionMetadata {
///         user_id: Some("user-456".to_string()),
///         tags: Some(vec!["rust".to_string(), "programming".to_string()]),
///         custom: None,
///     },
///     created_at: Utc::now(),
///     updated_at: Utc::now(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSession {
    /// Unique identifier for this session
    pub id: String,

    /// Optional human-readable title for the session
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Metadata associated with this session
    pub metadata: SessionMetadata,

    /// When this session was created
    pub created_at: DateTime<Utc>,

    /// When this session was last updated
    pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_metadata_default() {
        let metadata = SessionMetadata::default();
        assert!(metadata.user_id.is_none());
        assert!(metadata.tags.is_none());
        assert!(metadata.custom.is_none());
    }

    #[test]
    fn test_session_metadata_serialization() {
        let metadata = SessionMetadata {
            user_id: Some("user-123".to_string()),
            tags: Some(vec!["ai".to_string(), "chat".to_string()]),
            custom: Some(serde_json::json!({"key": "value"})),
        };

        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: SessionMetadata = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.user_id, metadata.user_id);
        assert_eq!(deserialized.tags, metadata.tags);
    }

    #[test]
    fn test_conversation_session_serialization() {
        let now = Utc::now();
        let session = ConversationSession {
            id: "session-1".to_string(),
            title: Some("Test Chat".to_string()),
            metadata: SessionMetadata::default(),
            created_at: now,
            updated_at: now,
        };

        let json = serde_json::to_string(&session).unwrap();
        let deserialized: ConversationSession = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, session.id);
        assert_eq!(deserialized.title, session.title);
    }

    #[test]
    fn test_session_without_title() {
        let session = ConversationSession {
            id: "session-1".to_string(),
            title: None,
            metadata: SessionMetadata::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&session).unwrap();
        // Verify that title is omitted when None
        assert!(!json.contains("\"title\""));
    }
}
