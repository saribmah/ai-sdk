//! Utility functions for filesystem operations.

use std::path::Path;
use tokio::fs;

use crate::error::FilesystemError;

/// Creates a directory and all parent directories if they don't exist.
///
/// # Arguments
///
/// * `path` - The directory path to create
///
/// # Errors
///
/// Returns an error if the directory cannot be created.
pub async fn ensure_dir(path: &Path) -> Result<(), FilesystemError> {
    if !path.exists() {
        fs::create_dir_all(path).await?;
    }
    Ok(())
}

/// Performs an atomic write by writing to a temporary file and then renaming.
///
/// This ensures that concurrent readers never see partial writes.
///
/// # Arguments
///
/// * `path` - The target file path
/// * `content` - The content to write
///
/// # Errors
///
/// Returns an error if the write or rename operation fails.
pub async fn atomic_write(_path: &Path, _content: &[u8]) -> Result<(), FilesystemError> {
    todo!("atomic_write")
}

/// Reads a JSON file and deserializes it.
///
/// # Arguments
///
/// * `path` - The file path to read
///
/// # Errors
///
/// Returns an error if the file cannot be read or deserialized.
pub async fn read_json<T>(_path: &Path) -> Result<T, FilesystemError>
where
    T: serde::de::DeserializeOwned,
{
    todo!("read_json")
}

/// Serializes data to JSON and writes it to a file atomically.
///
/// # Arguments
///
/// * `path` - The target file path
/// * `data` - The data to serialize and write
///
/// # Errors
///
/// Returns an error if serialization or write fails.
pub async fn write_json<T>(_path: &Path, _data: &T) -> Result<(), FilesystemError>
where
    T: serde::Serialize,
{
    todo!("write_json")
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_ensure_dir() {
        // This test will be implemented when the function is complete
    }
}
