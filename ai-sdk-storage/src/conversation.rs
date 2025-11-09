//! Conversation storage module.
//!
//! This module provides types and traits for storing conversation messages,
//! sessions, and related metadata.

pub mod message;
pub mod metadata;
pub mod session;

// Re-export types from submodules
pub use message::{MessageRole, StoredMessage};
pub use metadata::{MessageMetadata, ToolCallRecord, UsageStats};
pub use session::{ConversationSession, SessionMetadata};

use crate::error::StorageError;
use async_trait::async_trait;

/// Trait for conversation storage operations.
///
/// This trait defines the interface for storing and retrieving conversation
/// messages and sessions. Implementations can use different backends (filesystem,
/// databases, etc.) while maintaining a consistent API.
///
/// # Example
///
/// ```rust,no_run
/// use ai_sdk_storage::{ConversationStorage, StoredMessage, MessageRole, ConversationSession};
/// use std::sync::Arc;
///
/// # async fn example(storage: Arc<dyn ConversationStorage>) -> Result<(), Box<dyn std::error::Error>> {
/// // Create a session
/// let session = ConversationSession {
///     id: "session-123".to_string(),
///     title: Some("My Chat".to_string()),
///     metadata: Default::default(),
///     created_at: chrono::Utc::now(),
///     updated_at: chrono::Utc::now(),
/// };
/// storage.create_session(session).await?;
///
/// // Store a message
/// let message = StoredMessage {
///     id: "msg-1".to_string(),
///     session_id: "session-123".to_string(),
///     role: MessageRole::User,
///     content: serde_json::json!({"text": "Hello!"}),
///     metadata: Default::default(),
///     created_at: chrono::Utc::now(),
/// };
/// storage.store_message(message).await?;
///
/// // Retrieve messages
/// let messages = storage.get_messages("session-123", Some(50)).await?;
/// # Ok(())
/// # }
/// ```
#[async_trait]
pub trait ConversationStorage: Send + Sync {
    /// Store a message from generate_text or stream_text.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to store
    ///
    /// # Returns
    ///
    /// The ID of the stored message (may be the same as `message.id` or provider-generated)
    async fn store_message(&self, message: StoredMessage) -> Result<String, StorageError>;

    /// Retrieve messages for a session.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The session identifier
    /// * `limit` - Optional maximum number of messages to return (most recent first)
    ///
    /// # Returns
    ///
    /// A vector of messages ordered by creation time (oldest first)
    async fn get_messages(
        &self,
        session_id: &str,
        limit: Option<usize>,
    ) -> Result<Vec<StoredMessage>, StorageError>;

    /// Create a new conversation session.
    ///
    /// # Arguments
    ///
    /// * `session` - The session to create
    ///
    /// # Returns
    ///
    /// The ID of the created session (may be the same as `session.id` or provider-generated)
    async fn create_session(&self, session: ConversationSession) -> Result<String, StorageError>;

    /// List conversation sessions.
    ///
    /// # Arguments
    ///
    /// * `limit` - Optional maximum number of sessions to return (most recent first)
    ///
    /// # Returns
    ///
    /// A vector of sessions ordered by update time (most recent first)
    async fn list_sessions(
        &self,
        limit: Option<usize>,
    ) -> Result<Vec<ConversationSession>, StorageError>;

    /// Delete a conversation session and its messages.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The session identifier to delete
    ///
    /// # Errors
    ///
    /// Returns an error if the session doesn't exist or deletion fails.
    async fn delete_session(&self, session_id: &str) -> Result<(), StorageError>;
}
