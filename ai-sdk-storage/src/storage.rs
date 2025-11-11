use crate::{
    AssistantMessage, MessageMetadata, MessagePart, MessageRole, Session, StorageError, UserMessage,
};
use async_trait::async_trait;

/// Storage trait for AI SDK conversation and message persistence.
///
/// This trait defines the interface that all storage implementations must provide.
/// It supports session management, message storage, and conversation history retrieval.
///
/// # Implementation Requirements
///
/// Implementors must:
/// - Handle session creation, retrieval, and deletion
/// - Store and retrieve messages with their constituent parts
/// - Generate unique IDs for sessions, messages, and parts
/// - Maintain message ordering within sessions
/// - Support efficient listing and querying operations
///
/// # Architecture
///
/// The storage system follows a hierarchical structure:
/// ```text
/// Session
///   └── Message (User/Assistant)
///       └── Part (Text/Tool/File/etc.)
/// ```
///
/// Each storage provider is responsible for:
/// - ID generation strategy (timestamp-based, UUIDs, auto-increment, etc.)
/// - Physical storage mechanism (filesystem, database, cloud storage)
/// - Serialization format (JSON, binary, protocol buffers)
/// - Indexing and query optimization
///
/// # Examples
///
/// ```no_run
/// use ai_sdk_storage::{Storage, Session, UserMessage, AssistantMessage, MessagePart, StorageError, MessageRole};
/// use async_trait::async_trait;
///
/// struct MyStorage {
///     // Implementation-specific fields
/// }
///
/// #[async_trait]
/// impl Storage for MyStorage {
///     fn generate_session_id(&self) -> String {
///         // Provider-specific ID generation
///         todo!()
///     }
///     
///     fn generate_message_id(&self) -> String {
///         todo!()
///     }
///     
///     fn generate_part_id(&self) -> String {
///         todo!()
///     }
///     
///     async fn store_session(&self, session: &Session) -> Result<(), StorageError> {
///         // Provider-specific storage logic
///         todo!()
///     }
///     
///     // ... implement remaining methods
/// #   async fn get_session(&self, session_id: &str) -> Result<Session, StorageError> { todo!() }
/// #   async fn store_user_message(&self, message: &UserMessage, parts: &[MessagePart]) -> Result<(), StorageError> { todo!() }
/// #   async fn store_assistant_message(&self, message: &AssistantMessage, parts: &[MessagePart]) -> Result<(), StorageError> { todo!() }
/// #   async fn get_message(&self, session_id: &str, message_id: &str) -> Result<(MessageRole, Vec<MessagePart>), StorageError> { todo!() }
/// #   async fn list_messages(&self, session_id: &str, limit: Option<usize>) -> Result<Vec<String>, StorageError> { todo!() }
/// #   async fn list_sessions(&self, limit: Option<usize>) -> Result<Vec<Session>, StorageError> { todo!() }
/// #   async fn delete_session(&self, session_id: &str) -> Result<(), StorageError> { todo!() }
/// }
/// ```
#[async_trait]
pub trait Storage: Send + Sync {
    // ID Generation (provider-specific)

    /// Generate a new session ID.
    ///
    /// The format and uniqueness guarantees are provider-specific:
    /// - Filesystem: Sortable timestamp-based IDs
    /// - MongoDB: ObjectId
    /// - PostgreSQL: UUIDs or auto-increment
    fn generate_session_id(&self) -> String;

    /// Generate a new message ID.
    ///
    /// Message IDs should maintain chronological ordering within a session
    /// when using lexicographic sorting (for filesystem-based providers).
    fn generate_message_id(&self) -> String;

    /// Generate a new part ID.
    ///
    /// Part IDs should maintain ordering within a message.
    fn generate_part_id(&self) -> String;

    // Storage Operations

    /// Store a session.
    ///
    /// # Arguments
    ///
    /// * `session` - The session to store
    ///
    /// # Errors
    ///
    /// Returns an error if the session already exists or storage fails.
    async fn store_session(&self, session: &Session) -> Result<(), StorageError>;

    /// Get a session by ID.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The session identifier
    ///
    /// # Returns
    ///
    /// The session if found.
    ///
    /// # Errors
    ///
    /// Returns `StorageError::NotFound` if the session doesn't exist.
    async fn get_session(&self, session_id: &str) -> Result<Session, StorageError>;

    /// Store a user message with all its parts.
    ///
    /// This operation should be atomic - either all parts are stored or none are.
    ///
    /// # Arguments
    ///
    /// * `message` - The user message metadata
    /// * `parts` - The message parts (text, images, files, etc.)
    ///
    /// # Errors
    ///
    /// Returns an error if storage fails or the message already exists.
    async fn store_user_message(
        &self,
        message: &UserMessage,
        parts: &[MessagePart],
    ) -> Result<(), StorageError>;

    /// Store an assistant message with all its parts.
    ///
    /// This operation should be atomic - either all parts are stored or none are.
    ///
    /// # Arguments
    ///
    /// * `message` - The assistant message metadata
    /// * `parts` - The message parts (text, reasoning, tool calls, etc.)
    ///
    /// # Errors
    ///
    /// Returns an error if storage fails or the message already exists.
    async fn store_assistant_message(
        &self,
        message: &AssistantMessage,
        parts: &[MessagePart],
    ) -> Result<(), StorageError>;

    /// Get a message with all its parts.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The session containing the message
    /// * `message_id` - The message identifier
    ///
    /// # Returns
    ///
    /// A tuple of (message role, parts) where parts are in chronological order.
    ///
    /// # Errors
    ///
    /// Returns `StorageError::NotFound` if the message doesn't exist.
    async fn get_message(
        &self,
        session_id: &str,
        message_id: &str,
    ) -> Result<(MessageRole, Vec<MessagePart>), StorageError>;

    /// Get a message with all its parts and metadata (for assistant messages).
    ///
    /// # Arguments
    ///
    /// * `session_id` - The session containing the message
    /// * `message_id` - The message identifier
    ///
    /// # Returns
    ///
    /// A tuple of (message role, parts, optional metadata) where parts are in chronological order.
    /// Metadata is only provided for assistant messages.
    ///
    /// # Errors
    ///
    /// Returns `StorageError::NotFound` if the message doesn't exist.
    async fn get_message_with_metadata(
        &self,
        session_id: &str,
        message_id: &str,
    ) -> Result<(MessageRole, Vec<MessagePart>, Option<MessageMetadata>), StorageError>;

    /// List all message IDs in a session.
    ///
    /// # Arguments
    ///
    /// * `session_id` - The session identifier
    /// * `limit` - Optional maximum number of messages to return
    ///
    /// # Returns
    ///
    /// Message IDs in chronological order (oldest first).
    ///
    /// # Errors
    ///
    /// Returns `StorageError::NotFound` if the session doesn't exist.
    async fn list_messages(
        &self,
        session_id: &str,
        limit: Option<usize>,
    ) -> Result<Vec<String>, StorageError>;

    /// List all sessions.
    ///
    /// # Arguments
    ///
    /// * `limit` - Optional maximum number of sessions to return
    ///
    /// # Returns
    ///
    /// Sessions sorted by most recent first (descending order by updated_at).
    async fn list_sessions(&self, limit: Option<usize>) -> Result<Vec<Session>, StorageError>;

    /// Delete a session and all its data.
    ///
    /// This operation should recursively delete:
    /// - The session metadata
    /// - All messages in the session
    /// - All parts for each message
    ///
    /// # Arguments
    ///
    /// * `session_id` - The session identifier
    ///
    /// # Errors
    ///
    /// Returns `StorageError::NotFound` if the session doesn't exist.
    async fn delete_session(&self, session_id: &str) -> Result<(), StorageError>;
}
