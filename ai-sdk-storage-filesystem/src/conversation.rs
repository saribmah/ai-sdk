//! Filesystem-based conversation storage implementation.

use ai_sdk_storage::{ConversationSession, ConversationStorage, StorageError, StoredMessage};
use async_trait::async_trait;
use chrono::Utc;
use std::path::PathBuf;
use tokio::fs;

use crate::index::{MessageIndex, MessageIndexEntry, SessionIndex, SessionIndexEntry};
use crate::utils::{ensure_dir, read_json, write_json};

/// Filesystem implementation of conversation storage.
///
/// Stores conversations in a directory structure with JSON files for each message
/// and session metadata.
pub struct FilesystemConversationStorage {
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
    fn session_dir(&self, session_id: &str) -> PathBuf {
        self.base_path.join(session_id)
    }

    fn messages_dir(&self, session_id: &str) -> PathBuf {
        self.session_dir(session_id).join("messages")
    }

    fn session_file(&self, session_id: &str) -> PathBuf {
        self.session_dir(session_id).join("session.json")
    }

    fn message_file(&self, session_id: &str, message_id: &str) -> PathBuf {
        self.messages_dir(session_id)
            .join(format!("{}.json", message_id))
    }

    fn session_index_file(&self, session_id: &str) -> PathBuf {
        self.session_dir(session_id).join("index.json")
    }

    fn global_sessions_index_file(&self) -> PathBuf {
        self.base_path.join("sessions_index.json")
    }
}

#[async_trait]
impl ConversationStorage for FilesystemConversationStorage {
    async fn store_message(&self, message: StoredMessage) -> Result<String, StorageError> {
        let session_id = &message.session_id;
        let message_id = &message.id;

        // Ensure session directory structure exists
        let messages_dir = self.messages_dir(session_id);
        ensure_dir(&messages_dir)
            .await
            .map_err(StorageError::from)?;

        // Check if session exists, if not create a basic one
        let session_file = self.session_file(session_id);
        if !session_file.exists() {
            let session = ConversationSession {
                id: session_id.clone(),
                title: None,
                metadata: ai_sdk_storage::SessionMetadata {
                    user_id: None,
                    tags: None,
                    custom: None,
                },
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            write_json(&session_file, &session)
                .await
                .map_err(StorageError::from)?;
        }

        // Write message to file
        let message_file = self.message_file(session_id, message_id);
        write_json(&message_file, &message)
            .await
            .map_err(StorageError::from)?;

        // Update message index
        let index_file = self.session_index_file(session_id);
        let mut index = MessageIndex::load_or_new(&index_file, session_id.clone())
            .await
            .map_err(StorageError::from)?;

        index.add_message(MessageIndexEntry {
            message_id: message_id.clone(),
            created_at: message.created_at,
            role: message.role,
        });

        index.save(&index_file).await.map_err(StorageError::from)?;

        // Update global session index
        let global_index_file = self.global_sessions_index_file();
        let mut global_index = SessionIndex::load_or_new(&global_index_file)
            .await
            .map_err(StorageError::from)?;

        global_index.update_session(session_id, Utc::now(), index.messages.len());

        // If session doesn't exist in global index, add it
        if !global_index
            .sessions
            .iter()
            .any(|s| s.session_id == *session_id)
        {
            let session: ConversationSession =
                read_json(&session_file).await.map_err(StorageError::from)?;
            global_index.add_session(SessionIndexEntry {
                session_id: session_id.clone(),
                title: session.title.clone(),
                created_at: session.created_at,
                updated_at: Utc::now(),
                message_count: index.messages.len(),
            });
        }

        global_index
            .save(&global_index_file)
            .await
            .map_err(StorageError::from)?;

        Ok(message_id.clone())
    }

    async fn get_messages(
        &self,
        session_id: &str,
        limit: Option<usize>,
    ) -> Result<Vec<StoredMessage>, StorageError> {
        let index_file = self.session_index_file(session_id);

        // Check if session exists
        if !index_file.exists() {
            return Err(StorageError::NotFound(format!(
                "Session not found: {}",
                session_id
            )));
        }

        // Load message index
        let index = MessageIndex::load(&index_file)
            .await
            .map_err(StorageError::from)?;

        // Get message IDs with limit
        let message_ids = index.get_message_ids(limit);

        // Load each message
        let mut messages = Vec::new();
        for message_id in message_ids {
            let message_file = self.message_file(session_id, &message_id);
            let message: StoredMessage =
                read_json(&message_file).await.map_err(StorageError::from)?;
            messages.push(message);
        }

        Ok(messages)
    }

    async fn create_session(&self, session: ConversationSession) -> Result<String, StorageError> {
        let session_id = &session.id;
        let session_dir = self.session_dir(session_id);

        // Check if session already exists
        if session_dir.exists() {
            return Err(StorageError::InvalidInput(format!(
                "Session already exists: {}",
                session_id
            )));
        }

        // Create session directory structure
        let messages_dir = self.messages_dir(session_id);
        ensure_dir(&messages_dir)
            .await
            .map_err(StorageError::from)?;

        // Write session metadata
        let session_file = self.session_file(session_id);
        write_json(&session_file, &session)
            .await
            .map_err(StorageError::from)?;

        // Create empty message index
        let index_file = self.session_index_file(session_id);
        let index = MessageIndex::new(session_id.clone());
        index.save(&index_file).await.map_err(StorageError::from)?;

        // Update global session index
        let global_index_file = self.global_sessions_index_file();
        let mut global_index = SessionIndex::load_or_new(&global_index_file)
            .await
            .map_err(StorageError::from)?;

        global_index.add_session(SessionIndexEntry {
            session_id: session_id.clone(),
            title: session.title.clone(),
            created_at: session.created_at,
            updated_at: session.updated_at,
            message_count: 0,
        });

        global_index
            .save(&global_index_file)
            .await
            .map_err(StorageError::from)?;

        Ok(session_id.clone())
    }

    async fn list_sessions(
        &self,
        limit: Option<usize>,
    ) -> Result<Vec<ConversationSession>, StorageError> {
        let global_index_file = self.global_sessions_index_file();

        // Load global index (or return empty if doesn't exist)
        let index = SessionIndex::load_or_new(&global_index_file)
            .await
            .map_err(StorageError::from)?;

        // Get session entries with limit
        let session_entries = index.get_sessions(limit);

        // Load full session metadata for each entry
        let mut sessions = Vec::new();
        for entry in session_entries {
            let session_file = self.session_file(&entry.session_id);
            if let Ok(session) = read_json::<ConversationSession>(&session_file).await {
                sessions.push(session);
            }
            // Skip sessions that can't be loaded (may have been deleted)
        }

        Ok(sessions)
    }

    async fn delete_session(&self, session_id: &str) -> Result<(), StorageError> {
        let session_dir = self.session_dir(session_id);

        // Check if session exists
        if !session_dir.exists() {
            return Err(StorageError::NotFound(format!(
                "Session not found: {}",
                session_id
            )));
        }

        // Delete session directory recursively
        fs::remove_dir_all(&session_dir)
            .await
            .map_err(|e| StorageError::IoError(e.to_string()))?;

        // Update global session index
        let global_index_file = self.global_sessions_index_file();
        let mut global_index = SessionIndex::load_or_new(&global_index_file)
            .await
            .map_err(StorageError::from)?;

        global_index.remove_session(session_id);

        global_index
            .save(&global_index_file)
            .await
            .map_err(StorageError::from)?;

        Ok(())
    }
}
