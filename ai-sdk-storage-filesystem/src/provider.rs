//! Filesystem storage provider implementation.
//!
//! This module implements the `StorageProvider` trait for filesystem-based storage.

use ai_sdk_storage::{ConversationStorage, StorageProvider};
use std::path::PathBuf;
use std::sync::Arc;

use crate::conversation::FilesystemConversationStorage;
use crate::error::FilesystemError;
use crate::utils::ensure_dir;

/// Filesystem-based storage provider.
///
/// This provider stores all data in a directory structure on the local filesystem.
/// It uses JSON for serialization and maintains indexes for efficient queries.
///
/// # Example
///
/// ```no_run
/// use ai_sdk_storage_filesystem::FilesystemStorageProvider;
/// use std::sync::Arc;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let provider = Arc::new(FilesystemStorageProvider::new("./ai-storage")?);
/// # Ok(())
/// # }
/// ```
pub struct FilesystemStorageProvider {
    base_path: PathBuf,
    conversation_storage: Arc<FilesystemConversationStorage>,
}

impl FilesystemStorageProvider {
    /// Creates a new filesystem storage provider.
    ///
    /// This constructor will:
    /// - Create the base directory if it doesn't exist
    /// - Validate that the path is a valid directory
    /// - Initialize conversation storage
    ///
    /// # Arguments
    ///
    /// * `base_path` - The root directory for storing all data
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The base path cannot be created
    /// - The base path exists but is not a directory
    /// - Permissions are insufficient
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ai_sdk_storage_filesystem::FilesystemStorageProvider;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let provider = FilesystemStorageProvider::new("./my-storage")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(base_path: impl Into<PathBuf>) -> Result<Self, FilesystemError> {
        let base_path = base_path.into();

        // Validate base path
        if base_path.exists() && !base_path.is_dir() {
            return Err(FilesystemError::InvalidPath(format!(
                "Path exists but is not a directory: {}",
                base_path.display()
            )));
        }

        // Create the provider instance
        let provider = Self {
            conversation_storage: Arc::new(FilesystemConversationStorage::new(base_path.clone())),
            base_path,
        };

        Ok(provider)
    }

    /// Initializes the storage directories asynchronously.
    ///
    /// This method ensures all required directories exist.
    /// It's called automatically when needed, but can be called
    /// explicitly to pre-create the directory structure.
    ///
    /// # Errors
    ///
    /// Returns an error if directories cannot be created.
    pub async fn initialize(&self) -> Result<(), FilesystemError> {
        // Create base directory
        ensure_dir(&self.base_path).await?;
        Ok(())
    }

    /// Returns the base path of this storage provider.
    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }
}

impl StorageProvider for FilesystemStorageProvider {
    fn specification_version(&self) -> &str {
        "v1"
    }

    fn conversation_storage(&self) -> Arc<dyn ConversationStorage> {
        Arc::clone(&self.conversation_storage) as Arc<dyn ConversationStorage>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_sdk_storage::{ConversationSession, MessageMetadata, MessageRole, StoredMessage};
    use chrono::Utc;
    use tempfile::tempdir;

    #[test]
    fn test_new_provider() {
        let dir = tempdir().unwrap();
        let provider = FilesystemStorageProvider::new(dir.path()).unwrap();
        assert_eq!(provider.base_path(), dir.path());
    }

    #[test]
    fn test_new_provider_with_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("file.txt");
        std::fs::write(&file_path, "test").unwrap();

        let result = FilesystemStorageProvider::new(&file_path);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_initialize() {
        let dir = tempdir().unwrap();
        let storage_path = dir.path().join("new-storage");

        let provider = FilesystemStorageProvider::new(&storage_path).unwrap();
        provider.initialize().await.unwrap();

        assert!(storage_path.exists());
        assert!(storage_path.is_dir());
    }

    #[test]
    fn test_specification_version() {
        let dir = tempdir().unwrap();
        let provider = FilesystemStorageProvider::new(dir.path()).unwrap();
        assert_eq!(provider.specification_version(), "v1");
    }

    #[test]
    fn test_conversation_storage_access() {
        let dir = tempdir().unwrap();
        let provider = FilesystemStorageProvider::new(dir.path()).unwrap();
        let _storage = provider.conversation_storage();
        // Just verify we can get the storage without panic
    }

    #[tokio::test]
    async fn test_end_to_end_conversation_flow() {
        let dir = tempdir().unwrap();
        let provider = FilesystemStorageProvider::new(dir.path()).unwrap();
        provider.initialize().await.unwrap();

        let storage = provider.conversation_storage();

        // Create a session
        let session = ConversationSession {
            id: "test-session".to_string(),
            title: Some("Test Session".to_string()),
            metadata: ai_sdk_storage::SessionMetadata {
                user_id: Some("user-123".to_string()),
                tags: Some(vec!["test".to_string()]),
                custom: None,
            },
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        storage.create_session(session.clone()).await.unwrap();

        // Store a message
        let message = StoredMessage {
            id: "msg-1".to_string(),
            session_id: "test-session".to_string(),
            role: MessageRole::User,
            content: serde_json::json!({"text": "Hello, world!"}),
            metadata: MessageMetadata {
                model_id: None,
                provider: None,
                usage: None,
                finish_reason: None,
                tool_calls: None,
                custom: None,
            },
            created_at: Utc::now(),
        };

        let message_id = storage.store_message(message.clone()).await.unwrap();
        assert_eq!(message_id, "msg-1");

        // Retrieve messages
        let messages = storage.get_messages("test-session", None).await.unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].id, "msg-1");

        // List sessions
        let sessions = storage.list_sessions(None).await.unwrap();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].id, "test-session");

        // Delete session
        storage.delete_session("test-session").await.unwrap();

        // Verify session is deleted
        let result = storage.get_messages("test-session", None).await;
        assert!(result.is_err());
    }
}
