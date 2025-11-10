//! Error types for filesystem storage operations.

use ai_sdk_storage::StorageError;
use thiserror::Error;

/// Filesystem-specific errors.
#[derive(Debug, Error)]
pub enum FilesystemError {
    /// I/O error occurred during filesystem operation.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Failed to serialize or deserialize JSON data.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// The provided path is invalid.
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    /// The requested session was not found.
    #[error("Session not found: {0}")]
    SessionNotFound(String),

    /// The requested message was not found.
    #[error("Message not found: {0}")]
    MessageNotFound(String),

    /// Index file is corrupted or invalid.
    #[error("Index corrupted: {0}")]
    IndexCorrupted(String),

    /// Failed to acquire lock for concurrent access.
    #[error("Lock acquisition failed: {0}")]
    LockFailed(String),

    /// Generic storage error.
    #[error("Storage error: {0}")]
    Storage(String),
}

impl From<FilesystemError> for StorageError {
    fn from(err: FilesystemError) -> Self {
        match err {
            FilesystemError::Io(e) => StorageError::IoError(e.to_string()),
            FilesystemError::Serialization(e) => StorageError::SerializationError(e.to_string()),
            FilesystemError::InvalidPath(msg) => StorageError::InvalidInput(msg),
            FilesystemError::SessionNotFound(msg) => StorageError::NotFound(msg),
            FilesystemError::MessageNotFound(msg) => StorageError::NotFound(msg),
            FilesystemError::IndexCorrupted(msg) => StorageError::DatabaseError(msg),
            FilesystemError::LockFailed(msg) => StorageError::DatabaseError(msg),
            FilesystemError::Storage(msg) => StorageError::Unknown(msg),
        }
    }
}
