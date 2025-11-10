/// Error types for storage operations.
#[derive(Debug, Clone)]
pub enum StorageError {
    /// Resource doesn't exist
    NotFound(String),
    /// Resource already exists
    AlreadyExists(String),
    /// I/O operation failed
    IoError(String),
    /// JSON serialization failed
    SerializationError(String),
    /// Data validation failed
    InvalidData(String),
    /// Provider-specific error
    ProviderError(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::NotFound(msg) => write!(f, "Not found: {}", msg),
            StorageError::AlreadyExists(msg) => write!(f, "Already exists: {}", msg),
            StorageError::IoError(msg) => write!(f, "I/O error: {}", msg),
            StorageError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            StorageError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            StorageError::ProviderError(msg) => write!(f, "Provider error: {}", msg),
        }
    }
}

impl std::error::Error for StorageError {}
