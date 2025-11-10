//! # AI SDK Filesystem Storage Provider
//!
//! This crate provides a filesystem-based storage implementation for the AI SDK.
//! It implements the storage traits defined in `ai-sdk-storage` using JSON files
//! and a file-based indexing system.
//!
//! ## Features
//!
//! - **Conversation Storage**: Store and retrieve conversation messages and sessions
//! - **File-based Indexing**: Fast lookups without reading all files
//! - **Atomic Writes**: Safe concurrent access using temporary files
//! - **Human-readable**: JSON format for easy debugging and inspection
//!
//! ## Directory Structure
//!
//! ```text
//! <base_path>/
//! ├── conversations/
//! │   ├── <session_id>/
//! │   │   ├── session.json              # ConversationSession metadata
//! │   │   ├── messages/
//! │   │   │   ├── <message_id>.json    # Individual messages
//! │   │   │   └── ...
//! │   │   └── index.json                # Message ordering & quick lookup
//! │   └── ...
//! └── sessions_index.json                # Global session index
//! ```
//!
//! ## Usage
//!
//! ```rust,no_run
//! use ai_sdk_storage_filesystem::FilesystemStorageProvider;
//! use std::sync::Arc;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a filesystem storage provider
//! let storage = Arc::new(FilesystemStorageProvider::new("./ai-storage")?);
//!
//! // Use with GenerateText
//! // let result = GenerateText::new(model, prompt)
//! //     .with_storage(storage.clone())
//! //     .with_session_id("my-session".to_string())
//! //     .execute()
//! //     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Performance Characteristics
//!
//! - **Message Storage**: O(1) per message + index update
//! - **Message Retrieval**: O(n) where n = limit (default: all messages)
//! - **Session Listing**: O(n) where n = number of sessions
//! - **Session Deletion**: O(m) where m = number of messages in session
//!
//! ## Concurrency
//!
//! The filesystem provider uses atomic writes (temp file + rename) to ensure
//! data consistency. Multiple readers can access files simultaneously, but
//! concurrent writes to the same session are serialized.
//!
//! ## Limitations
//!
//! - Not suitable for high-concurrency scenarios (use database provider instead)
//! - No built-in backup or replication
//! - File system performance limits apply

pub mod conversation;
pub mod error;
pub mod index;
pub mod provider;
pub mod utils;

// Re-export main types
pub use error::FilesystemError;
pub use provider::FilesystemStorageProvider;

// Re-export storage traits for convenience
pub use ai_sdk_storage::{
    ConversationSession, ConversationStorage, MessageRole, StorageError, StorageProvider,
    StoredMessage,
};
