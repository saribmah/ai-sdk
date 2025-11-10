//! Filesystem storage provider implementation.
//!
//! This module implements the `StorageProvider` trait for filesystem-based storage.

use ai_sdk_storage::{ConversationStorage, StorageProvider};
use std::path::PathBuf;
use std::sync::Arc;

use crate::conversation::FilesystemConversationStorage;
use crate::error::FilesystemError;

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
    #[allow(dead_code)]
    base_path: PathBuf,
    conversation_storage: Arc<FilesystemConversationStorage>,
}

impl FilesystemStorageProvider {
    /// Creates a new filesystem storage provider.
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
    pub fn new(base_path: impl Into<PathBuf>) -> Result<Self, FilesystemError> {
        let _base_path = base_path.into();

        // TODO: Validate and create directory structure
        // TODO: Initialize conversation storage

        todo!("FilesystemStorageProvider::new")
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
