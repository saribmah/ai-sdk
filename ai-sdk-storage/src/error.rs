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

impl StorageError {
    /// Returns `true` if this error is potentially transient and retryable.
    ///
    /// Transient errors are typically I/O related and may succeed on retry.
    /// Permanent errors like validation failures or not-found errors will not benefit from retries.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_storage::StorageError;
    ///
    /// let io_error = StorageError::IoError("Connection timeout".to_string());
    /// assert!(io_error.is_transient());
    ///
    /// let not_found = StorageError::NotFound("session_123".to_string());
    /// assert!(!not_found.is_transient());
    /// ```
    pub fn is_transient(&self) -> bool {
        matches!(
            self,
            StorageError::IoError(_) | StorageError::ProviderError(_)
        )
    }
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
