//! # LLM Kit Storage - Filesystem Provider
//!
//! Filesystem-based storage implementation for LLM Kit Storage.
//!
//! This crate provides a production-ready filesystem storage provider that implements
//! the `Storage` trait from `llm-kit-storage`. It stores sessions, messages, and parts
//! as JSON files in a hierarchical directory structure.
//!
//! ## Features
//!
//! - **Hierarchical Storage**: Session → Message → Part structure
//! - **Sortable IDs**: Timestamp-based IDs with lexicographic ordering
//! - **Atomic Operations**: Write-rename pattern prevents corruption
//! - **Type Safety**: Strongly-typed message parts with serde serialization
//!
//! ## Directory Structure
//!
//! ```text
//! base_path/
//! ├── session/{sessionID}.json
//! ├── message/{sessionID}/{messageID}.json
//! └── part/{messageID}/{partID}.json
//! ```
//!
//! ## Usage
//!
//! ```rust,no_run
//! use llm_kit_storage::Storage;
//! use llm_kit_storage_filesystem::FilesystemStorage;
//! use std::sync::Arc;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create and initialize storage
//! let storage = Arc::new(FilesystemStorage::new("./ai-storage")?);
//! storage.initialize().await?;
//!
//! // Generate a new session
//! let session_id = storage.generate_session_id();
//!
//! // Use with GenerateText (future integration)
//! // let result = GenerateText::new(model, prompt)
//! //     .with_storage(storage.clone())
//! //     .with_session_id(session_id)
//! //     .execute()
//! //     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## ID Format
//!
//! The filesystem provider generates sortable IDs:
//!
//! - **Session IDs**: `ses_{inverted_time}{random}` (descending order)
//! - **Message IDs**: `msg_{time}{random}` (ascending order)
//! - **Part IDs**: `prt_{time}{random}` (ascending order)
//!
//! Total length: 30 characters (4 prefix + 12 time + 14 random)

mod id_generator;

use async_trait::async_trait;
use llm_kit_storage::{
    AssistantMessage, MessageMetadata, MessagePart, MessageRole, Session, Storage, StorageError,
    UserMessage,
};
use std::path::PathBuf;
use tokio::fs;

/// Filesystem-based storage provider
pub struct FilesystemStorage {
    base_path: PathBuf,
}

impl FilesystemStorage {
    /// Create a new filesystem storage instance
    ///
    /// # Arguments
    ///
    /// * `base_path` - The root directory for storage
    ///
    /// # Errors
    ///
    /// Returns an error if the path is invalid
    pub fn new(base_path: impl Into<PathBuf>) -> Result<Self, StorageError> {
        Ok(Self {
            base_path: base_path.into(),
        })
    }

    /// Initialize storage directories (must be called before use)
    ///
    /// Creates the base directory structure:
    /// - `base_path/session/`
    /// - `base_path/message/`
    /// - `base_path/part/`
    ///
    /// # Errors
    ///
    /// Returns an error if directory creation fails
    pub async fn initialize(&self) -> Result<(), StorageError> {
        fs::create_dir_all(&self.base_path)
            .await
            .map_err(|e| StorageError::IoError(e.to_string()))?;
        fs::create_dir_all(self.base_path.join("session"))
            .await
            .map_err(|e| StorageError::IoError(e.to_string()))?;
        fs::create_dir_all(self.base_path.join("message"))
            .await
            .map_err(|e| StorageError::IoError(e.to_string()))?;
        fs::create_dir_all(self.base_path.join("part"))
            .await
            .map_err(|e| StorageError::IoError(e.to_string()))?;
        Ok(())
    }

    /// Build path for session file
    fn session_path(&self, session_id: &str) -> PathBuf {
        self.base_path
            .join("session")
            .join(format!("{}.json", session_id))
    }

    /// Build path for message file
    fn message_path(&self, session_id: &str, message_id: &str) -> PathBuf {
        self.base_path
            .join("message")
            .join(session_id)
            .join(format!("{}.json", message_id))
    }

    /// Build path for part file
    fn part_path(&self, message_id: &str, part_id: &str) -> PathBuf {
        self.base_path
            .join("part")
            .join(message_id)
            .join(format!("{}.json", part_id))
    }

    /// Atomic write using temp file + rename
    async fn write_json<T: serde::Serialize>(
        &self,
        path: &PathBuf,
        data: &T,
    ) -> Result<(), StorageError> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| StorageError::IoError(e.to_string()))?;
        }

        // Write to temp file
        let temp_path = path.with_extension("tmp");
        let json = serde_json::to_vec_pretty(data)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;
        fs::write(&temp_path, json)
            .await
            .map_err(|e| StorageError::IoError(e.to_string()))?;

        // Atomic rename
        fs::rename(&temp_path, path)
            .await
            .map_err(|e| StorageError::IoError(e.to_string()))?;

        Ok(())
    }

    /// Read and deserialize JSON
    async fn read_json<T: serde::de::DeserializeOwned>(
        &self,
        path: &PathBuf,
    ) -> Result<T, StorageError> {
        let data = fs::read(path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                StorageError::NotFound(format!("File not found: {}", path.display()))
            } else {
                StorageError::IoError(e.to_string())
            }
        })?;

        serde_json::from_slice(&data).map_err(|e| StorageError::SerializationError(e.to_string()))
    }

    /// Extract part ID from MessagePart enum
    fn extract_part_id(part: &MessagePart) -> &str {
        match part {
            MessagePart::Text(p) => &p.id,
            MessagePart::Image(p) => &p.id,
            MessagePart::File(p) => &p.id,
            MessagePart::Reasoning(p) => &p.id,
            MessagePart::ToolCall(p) => &p.id,
            MessagePart::ToolResult(p) => &p.id,
            MessagePart::Source(p) => &p.id,
        }
    }
}

#[async_trait]
impl Storage for FilesystemStorage {
    // ID Generation

    fn generate_session_id(&self) -> String {
        id_generator::IdGenerator::generate_session_id()
    }

    fn generate_message_id(&self) -> String {
        id_generator::IdGenerator::generate_message_id()
    }

    fn generate_part_id(&self) -> String {
        id_generator::IdGenerator::generate_part_id()
    }

    // Session Operations

    async fn store_session(&self, session: &Session) -> Result<(), StorageError> {
        let path = self.session_path(&session.id);
        self.write_json(&path, session).await
    }

    async fn get_session(&self, session_id: &str) -> Result<Session, StorageError> {
        let path = self.session_path(session_id);
        self.read_json(&path).await
    }

    async fn list_sessions(&self, limit: Option<usize>) -> Result<Vec<Session>, StorageError> {
        let sessions_dir = self.base_path.join("session");

        if !sessions_dir.exists() {
            return Ok(Vec::new());
        }

        let mut entries = fs::read_dir(&sessions_dir)
            .await
            .map_err(|e| StorageError::IoError(e.to_string()))?;

        let mut sessions = Vec::new();
        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| StorageError::IoError(e.to_string()))?
        {
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                let session: Session = self.read_json(&entry.path()).await?;
                sessions.push(session);
            }
        }

        // Sort by updated_at (most recent first)
        sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        // Apply limit
        if let Some(limit) = limit {
            sessions.truncate(limit);
        }

        Ok(sessions)
    }

    async fn delete_session(&self, session_id: &str) -> Result<(), StorageError> {
        // Delete session file
        let session_path = self.session_path(session_id);
        if session_path.exists() {
            fs::remove_file(&session_path)
                .await
                .map_err(|e| StorageError::IoError(e.to_string()))?;
        }

        // Delete all messages and their parts
        let messages_dir = self.base_path.join("message").join(session_id);
        if messages_dir.exists() {
            let message_ids = self.list_messages(session_id, None).await?;

            for message_id in message_ids {
                // Delete parts
                let parts_dir = self.base_path.join("part").join(&message_id);
                if parts_dir.exists() {
                    fs::remove_dir_all(&parts_dir)
                        .await
                        .map_err(|e| StorageError::IoError(e.to_string()))?;
                }
            }

            // Delete messages directory
            fs::remove_dir_all(&messages_dir)
                .await
                .map_err(|e| StorageError::IoError(e.to_string()))?;
        }

        Ok(())
    }

    // Message Operations

    async fn store_user_message(
        &self,
        message: &UserMessage,
        parts: &[MessagePart],
    ) -> Result<(), StorageError> {
        // Store message metadata
        let message_path = self.message_path(&message.session_id, &message.id);
        self.write_json(&message_path, message).await?;

        // Store all parts
        for part in parts {
            let part_id = Self::extract_part_id(part);
            let part_path = self.part_path(&message.id, part_id);
            self.write_json(&part_path, part).await?;
        }

        Ok(())
    }

    async fn store_assistant_message(
        &self,
        message: &AssistantMessage,
        parts: &[MessagePart],
    ) -> Result<(), StorageError> {
        // Store message metadata
        let message_path = self.message_path(&message.session_id, &message.id);
        self.write_json(&message_path, message).await?;

        // Store all parts
        for part in parts {
            let part_id = Self::extract_part_id(part);
            let part_path = self.part_path(&message.id, part_id);
            self.write_json(&part_path, part).await?;
        }

        Ok(())
    }

    async fn get_message(
        &self,
        session_id: &str,
        message_id: &str,
    ) -> Result<(MessageRole, Vec<MessagePart>), StorageError> {
        // Read message to determine role
        let message_path = self.message_path(session_id, message_id);
        let message_data = fs::read(&message_path)
            .await
            .map_err(|_| StorageError::NotFound(format!("Message not found: {}", message_id)))?;

        // Deserialize to determine role - check the "role" field in the JSON
        let json_value: serde_json::Value = serde_json::from_slice(&message_data).map_err(|e| {
            StorageError::SerializationError(format!("Invalid message JSON: {}", e))
        })?;

        let role = json_value
            .get("role")
            .and_then(|v| v.as_str())
            .and_then(|s| match s {
                "user" => Some(MessageRole::User),
                "assistant" => Some(MessageRole::Assistant),
                _ => None,
            })
            .ok_or_else(|| {
                StorageError::SerializationError("Missing or invalid role field".to_string())
            })?;

        // List and load all parts
        let parts_dir = self.base_path.join("part").join(message_id);
        let mut parts = Vec::new();

        if parts_dir.exists() {
            let mut entries = fs::read_dir(&parts_dir)
                .await
                .map_err(|e| StorageError::IoError(e.to_string()))?;

            let mut part_files = Vec::new();
            while let Some(entry) = entries
                .next_entry()
                .await
                .map_err(|e| StorageError::IoError(e.to_string()))?
            {
                if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                    part_files.push(entry.path());
                }
            }

            // Sort by filename (which are part IDs)
            part_files.sort();

            for part_file in part_files {
                let part: MessagePart = self.read_json(&part_file).await?;
                parts.push(part);
            }
        }

        Ok((role, parts))
    }

    async fn get_message_with_metadata(
        &self,
        session_id: &str,
        message_id: &str,
    ) -> Result<(MessageRole, Vec<MessagePart>, Option<MessageMetadata>), StorageError> {
        // Read message to determine role and metadata
        let message_path = self.message_path(session_id, message_id);
        let message_data = fs::read(&message_path)
            .await
            .map_err(|_| StorageError::NotFound(format!("Message not found: {}", message_id)))?;

        // Deserialize to determine role - check the "role" field in the JSON
        let json_value: serde_json::Value = serde_json::from_slice(&message_data).map_err(|e| {
            StorageError::SerializationError(format!("Invalid message JSON: {}", e))
        })?;

        let role = json_value
            .get("role")
            .and_then(|v| v.as_str())
            .and_then(|s| match s {
                "user" => Some(MessageRole::User),
                "assistant" => Some(MessageRole::Assistant),
                _ => None,
            })
            .ok_or_else(|| {
                StorageError::SerializationError("Missing or invalid role field".to_string())
            })?;

        // Extract metadata for assistant messages
        let metadata = if role == MessageRole::Assistant {
            serde_json::from_value::<AssistantMessage>(json_value)
                .ok()
                .map(|msg| msg.metadata)
        } else {
            None
        };

        // List and load all parts
        let parts_dir = self.base_path.join("part").join(message_id);
        let mut parts = Vec::new();

        if parts_dir.exists() {
            let mut entries = fs::read_dir(&parts_dir)
                .await
                .map_err(|e| StorageError::IoError(e.to_string()))?;

            let mut part_files = Vec::new();
            while let Some(entry) = entries
                .next_entry()
                .await
                .map_err(|e| StorageError::IoError(e.to_string()))?
            {
                if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                    part_files.push(entry.path());
                }
            }

            // Sort by filename (which are part IDs)
            part_files.sort();

            for part_file in part_files {
                let part: MessagePart = self.read_json(&part_file).await?;
                parts.push(part);
            }
        }

        Ok((role, parts, metadata))
    }

    async fn list_messages(
        &self,
        session_id: &str,
        limit: Option<usize>,
    ) -> Result<Vec<String>, StorageError> {
        let messages_dir = self.base_path.join("message").join(session_id);

        if !messages_dir.exists() {
            return Ok(Vec::new());
        }

        let mut entries = fs::read_dir(&messages_dir)
            .await
            .map_err(|e| StorageError::IoError(e.to_string()))?;

        let mut message_ids = Vec::new();
        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| StorageError::IoError(e.to_string()))?
        {
            if let Some(filename) = entry.file_name().to_str()
                && let Some(id) = filename.strip_suffix(".json")
            {
                message_ids.push(id.to_string());
            }
        }

        // Sort by ID (chronological due to timestamp-based IDs)
        message_ids.sort();

        // Apply limit
        if let Some(limit) = limit {
            message_ids.truncate(limit);
        }

        Ok(message_ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use llm_kit_storage::{MessageMetadata, Session, TextPart, UsageStats};
    use tempfile::TempDir;

    async fn setup_storage() -> (FilesystemStorage, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let storage = FilesystemStorage::new(temp_dir.path()).unwrap();
        storage.initialize().await.unwrap();
        (storage, temp_dir)
    }

    #[tokio::test]
    async fn test_store_and_get_session() {
        let (storage, _dir) = setup_storage().await;

        let session_id = storage.generate_session_id();
        let session = Session::new(session_id.clone()).with_title("Test Session".to_string());

        storage.store_session(&session).await.unwrap();
        let retrieved = storage.get_session(&session_id).await.unwrap();

        assert_eq!(session.id, retrieved.id);
        assert_eq!(session.title, retrieved.title);
    }

    #[tokio::test]
    async fn test_store_and_get_user_message() {
        let (storage, _dir) = setup_storage().await;

        let session_id = storage.generate_session_id();
        let message_id = storage.generate_message_id();
        let part_id = storage.generate_part_id();

        let text_part = TextPart::new(part_id.clone(), "Hello".to_string());
        let parts = vec![MessagePart::Text(text_part)];
        let message = UserMessage::new(message_id.clone(), session_id.clone(), vec![part_id]);

        storage.store_user_message(&message, &parts).await.unwrap();
        let (role, retrieved_parts) = storage.get_message(&session_id, &message_id).await.unwrap();

        assert_eq!(role, MessageRole::User);
        assert_eq!(parts.len(), retrieved_parts.len());
    }

    #[tokio::test]
    async fn test_store_and_get_assistant_message() {
        let (storage, _dir) = setup_storage().await;

        let session_id = storage.generate_session_id();
        let message_id = storage.generate_message_id();
        let part_id = storage.generate_part_id();

        let text_part = TextPart::new(part_id.clone(), "Hello from AI".to_string());
        let parts = vec![MessagePart::Text(text_part)];
        let message = AssistantMessage::new(message_id.clone(), session_id.clone(), vec![part_id])
            .with_metadata(MessageMetadata {
                model_id: Some("gpt-4".to_string()),
                provider: Some("openai".to_string()),
                usage: Some(UsageStats::new(10, 20)),
                finish_reason: Some("stop".to_string()),
                custom: None,
            });

        storage
            .store_assistant_message(&message, &parts)
            .await
            .unwrap();
        let (role, retrieved_parts) = storage.get_message(&session_id, &message_id).await.unwrap();

        assert_eq!(role, MessageRole::Assistant);
        assert_eq!(parts.len(), retrieved_parts.len());
    }

    #[tokio::test]
    async fn test_list_sessions_sorted() {
        let (storage, _dir) = setup_storage().await;

        // Create multiple sessions
        let session1_id = storage.generate_session_id();
        let session1 = Session::new(session1_id).with_title("Session 1".to_string());
        storage.store_session(&session1).await.unwrap();

        // Sleep to ensure different timestamps
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let session2_id = storage.generate_session_id();
        let session2 = Session::new(session2_id).with_title("Session 2".to_string());
        storage.store_session(&session2).await.unwrap();

        let sessions = storage.list_sessions(None).await.unwrap();
        assert_eq!(sessions.len(), 2);
        // Most recent first (session2)
        assert_eq!(sessions[0].title, Some("Session 2".to_string()));
        assert_eq!(sessions[1].title, Some("Session 1".to_string()));
    }

    #[tokio::test]
    async fn test_list_messages_sorted() {
        let (storage, _dir) = setup_storage().await;

        let session_id = storage.generate_session_id();

        // Create multiple messages
        for i in 0..3 {
            let message_id = storage.generate_message_id();
            let part_id = storage.generate_part_id();
            let text_part = TextPart::new(part_id.clone(), format!("Message {}", i));
            let parts = vec![MessagePart::Text(text_part)];
            let message = UserMessage::new(message_id, session_id.clone(), vec![part_id]);
            storage.store_user_message(&message, &parts).await.unwrap();
        }

        let message_ids = storage.list_messages(&session_id, None).await.unwrap();
        assert_eq!(message_ids.len(), 3);
        // Should be in chronological order (ascending)
        assert!(message_ids[0] < message_ids[1]);
        assert!(message_ids[1] < message_ids[2]);
    }

    #[tokio::test]
    async fn test_delete_session_cascades() {
        let (storage, _dir) = setup_storage().await;

        let session_id = storage.generate_session_id();
        let session = Session::new(session_id.clone());
        storage.store_session(&session).await.unwrap();

        // Add a message
        let message_id = storage.generate_message_id();
        let part_id = storage.generate_part_id();
        let text_part = TextPart::new(part_id.clone(), "Test".to_string());
        let parts = vec![MessagePart::Text(text_part)];
        let message = UserMessage::new(message_id.clone(), session_id.clone(), vec![part_id]);
        storage.store_user_message(&message, &parts).await.unwrap();

        // Delete session
        storage.delete_session(&session_id).await.unwrap();

        // Verify session is gone
        assert!(storage.get_session(&session_id).await.is_err());

        // Verify messages are gone
        let messages = storage.list_messages(&session_id, None).await.unwrap();
        assert_eq!(messages.len(), 0);
    }

    #[tokio::test]
    async fn test_get_nonexistent_session() {
        let (storage, _dir) = setup_storage().await;
        let result = storage.get_session("nonexistent").await;
        assert!(matches!(result, Err(StorageError::NotFound(_))));
    }

    #[tokio::test]
    async fn test_get_nonexistent_message() {
        let (storage, _dir) = setup_storage().await;
        let result = storage.get_message("session-123", "message-456").await;
        assert!(matches!(result, Err(StorageError::NotFound(_))));
    }

    #[tokio::test]
    async fn test_list_sessions_with_limit() {
        let (storage, _dir) = setup_storage().await;

        // Create 5 sessions
        for i in 0..5 {
            let session_id = storage.generate_session_id();
            let session = Session::new(session_id).with_title(format!("Session {}", i));
            storage.store_session(&session).await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
        }

        let sessions = storage.list_sessions(Some(3)).await.unwrap();
        assert_eq!(sessions.len(), 3);
    }

    #[tokio::test]
    async fn test_list_messages_with_limit() {
        let (storage, _dir) = setup_storage().await;

        let session_id = storage.generate_session_id();

        // Create 5 messages
        for i in 0..5 {
            let message_id = storage.generate_message_id();
            let part_id = storage.generate_part_id();
            let text_part = TextPart::new(part_id.clone(), format!("Message {}", i));
            let parts = vec![MessagePart::Text(text_part)];
            let message = UserMessage::new(message_id, session_id.clone(), vec![part_id]);
            storage.store_user_message(&message, &parts).await.unwrap();
        }

        let message_ids = storage.list_messages(&session_id, Some(3)).await.unwrap();
        assert_eq!(message_ids.len(), 3);
    }
}
