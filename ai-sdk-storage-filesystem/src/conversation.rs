//! Filesystem-based conversation storage implementation.

use ai_sdk_storage::{ConversationSession, ConversationStorage, StorageError, StoredMessage};
use async_trait::async_trait;
use std::path::PathBuf;

/// Filesystem implementation of conversation storage.
///
/// Stores conversations in a directory structure with JSON files for each message
/// and session metadata.
pub struct FilesystemConversationStorage {
    #[allow(dead_code)]
    base_path: PathBuf,
}

impl FilesystemConversationStorage {
    /// Creates a new filesystem conversation storage.
    ///
    /// # Arguments
    ///
    /// * `base_path` - The root directory for conversation storage
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    // Helper methods for path construction
    #[allow(dead_code)]
    fn session_dir(&self, session_id: &str) -> PathBuf {
        self.base_path.join(session_id)
    }

    #[allow(dead_code)]
    fn messages_dir(&self, session_id: &str) -> PathBuf {
        self.session_dir(session_id).join("messages")
    }

    #[allow(dead_code)]
    fn session_file(&self, session_id: &str) -> PathBuf {
        self.session_dir(session_id).join("session.json")
    }

    #[allow(dead_code)]
    fn message_file(&self, session_id: &str, message_id: &str) -> PathBuf {
        self.messages_dir(session_id)
            .join(format!("{}.json", message_id))
    }

    #[allow(dead_code)]
    fn session_index_file(&self, session_id: &str) -> PathBuf {
        self.session_dir(session_id).join("index.json")
    }

    #[allow(dead_code)]
    fn global_sessions_index_file(&self) -> PathBuf {
        self.base_path
            .parent()
            .expect("base_path should have parent")
            .join("sessions_index.json")
    }
}

#[async_trait]
impl ConversationStorage for FilesystemConversationStorage {
    async fn store_message(&self, _message: StoredMessage) -> Result<String, StorageError> {
        todo!("FilesystemConversationStorage::store_message")
    }

    async fn get_messages(
        &self,
        _session_id: &str,
        _limit: Option<usize>,
    ) -> Result<Vec<StoredMessage>, StorageError> {
        todo!("FilesystemConversationStorage::get_messages")
    }

    async fn create_session(&self, _session: ConversationSession) -> Result<String, StorageError> {
        todo!("FilesystemConversationStorage::create_session")
    }

    async fn list_sessions(
        &self,
        _limit: Option<usize>,
    ) -> Result<Vec<ConversationSession>, StorageError> {
        todo!("FilesystemConversationStorage::list_sessions")
    }

    async fn delete_session(&self, _session_id: &str) -> Result<(), StorageError> {
        todo!("FilesystemConversationStorage::delete_session")
    }
}
