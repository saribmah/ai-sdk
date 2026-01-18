use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A conversation session.
///
/// A session represents a conversation context that groups related messages together.
/// It can have metadata like user ID, tags, and custom fields.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Session {
    /// Unique session identifier (provider-generated)
    pub id: String,

    /// Human-readable title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Session metadata
    pub metadata: SessionMetadata,

    /// When this session was created
    pub created_at: DateTime<Utc>,

    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl Session {
    /// Create a new session with the given ID.
    pub fn new(id: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            title: None,
            metadata: SessionMetadata::default(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Set the title for this session (builder pattern).
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    /// Set the metadata for this session (builder pattern).
    pub fn with_metadata(mut self, metadata: SessionMetadata) -> Self {
        self.metadata = metadata;
        self
    }
}

/// Session metadata for categorization and filtering.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SessionMetadata {
    /// User identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    /// Tags for categorization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    /// Custom metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<serde_json::Value>,
}

impl SessionMetadata {
    /// Create new session metadata with a user ID.
    pub fn with_user_id(user_id: String) -> Self {
        Self {
            user_id: Some(user_id),
            tags: None,
            custom: None,
        }
    }

    /// Add tags to the metadata (builder pattern).
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    /// Add custom metadata (builder pattern).
    pub fn with_custom(mut self, custom: serde_json::Value) -> Self {
        self.custom = Some(custom);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_session_creation() {
        let session = Session::new("session-123".to_string());
        assert_eq!(session.id, "session-123");
        assert!(session.title.is_none());
        assert_eq!(session.metadata, SessionMetadata::default());
    }

    #[test]
    fn test_session_with_title() {
        let session =
            Session::new("session-123".to_string()).with_title("My Conversation".to_string());
        assert_eq!(session.title, Some("My Conversation".to_string()));
    }

    #[test]
    fn test_session_with_metadata() {
        let metadata = SessionMetadata::with_user_id("user-456".to_string())
            .with_tags(vec!["work".to_string(), "project-x".to_string()]);

        let session = Session::new("session-123".to_string()).with_metadata(metadata.clone());

        assert_eq!(session.metadata, metadata);
    }

    #[test]
    fn test_session_metadata_default() {
        let metadata = SessionMetadata::default();
        assert!(metadata.user_id.is_none());
        assert!(metadata.tags.is_none());
        assert!(metadata.custom.is_none());
    }

    #[test]
    fn test_session_metadata_with_custom() {
        let custom = json!({"priority": "high", "project": "alpha"});
        let metadata =
            SessionMetadata::with_user_id("user-123".to_string()).with_custom(custom.clone());

        assert_eq!(metadata.custom, Some(custom));
    }

    #[test]
    fn test_session_serialization() {
        let session =
            Session::new("session-123".to_string()).with_title("Test Session".to_string());

        let json = serde_json::to_string(&session).unwrap();
        let deserialized: Session = serde_json::from_str(&json).unwrap();
        assert_eq!(session.id, deserialized.id);
        assert_eq!(session.title, deserialized.title);
    }

    #[test]
    fn test_session_metadata_serialization() {
        let metadata = SessionMetadata::with_user_id("user-123".to_string())
            .with_tags(vec!["tag1".to_string(), "tag2".to_string()]);

        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: SessionMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(metadata, deserialized);
    }
}
