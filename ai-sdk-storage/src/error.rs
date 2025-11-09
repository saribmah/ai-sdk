//! Error types for storage operations.
//!
//! This module defines the error types that can occur during storage operations.

use thiserror::Error;

/// Errors that can occur during storage operations.
#[derive(Error, Debug)]
pub enum StorageError {
    /// The requested resource was not found.
    ///
    /// This error occurs when attempting to retrieve a session, message, or other
    /// resource that doesn't exist in storage.
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Invalid input parameters were provided.
    ///
    /// This error occurs when the input data is malformed, missing required fields,
    /// or violates storage constraints.
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Failed to serialize or deserialize data.
    ///
    /// This error occurs when converting between Rust types and storage formats
    /// (e.g., JSON, BSON) fails.
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// An I/O error occurred.
    ///
    /// This error occurs during file system operations or network communication
    /// with storage backends.
    #[error("I/O error: {0}")]
    IoError(String),

    /// A database-specific error occurred.
    ///
    /// This error wraps database-specific errors from backends like MongoDB,
    /// PostgreSQL, etc.
    #[error("Database error: {0}")]
    DatabaseError(String),

    /// A provider-specific error occurred.
    ///
    /// This error is used for provider-specific failures that don't fit
    /// other categories.
    #[error("Provider error: {0}")]
    ProviderError(String),

    /// An unknown or unexpected error occurred.
    ///
    /// This is a catch-all for errors that don't fit other categories.
    #[error("Unknown error: {0}")]
    Unknown(String),
}

// Implement conversions from common error types
impl From<std::io::Error> for StorageError {
    fn from(err: std::io::Error) -> Self {
        StorageError::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(err: serde_json::Error) -> Self {
        StorageError::SerializationError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = StorageError::NotFound("session-123".to_string());
        assert_eq!(error.to_string(), "Resource not found: session-123");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let storage_err: StorageError = io_err.into();
        assert!(matches!(storage_err, StorageError::IoError(_)));
    }

    #[test]
    fn test_json_error_conversion() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let storage_err: StorageError = json_err.into();
        assert!(matches!(storage_err, StorageError::SerializationError(_)));
    }
}
