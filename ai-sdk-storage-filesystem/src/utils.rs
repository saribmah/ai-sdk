//! Utility functions for filesystem operations.

use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;

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
pub async fn atomic_write(path: &Path, content: &[u8]) -> Result<(), FilesystemError> {
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        ensure_dir(parent).await?;
    }

    // Create a temporary file in the same directory
    let temp_path = path.with_extension("tmp");

    // Write to temporary file
    let mut file = fs::File::create(&temp_path).await?;
    file.write_all(content).await?;
    file.sync_all().await?; // Ensure data is written to disk

    // Atomically rename temp file to target
    fs::rename(&temp_path, path).await?;

    Ok(())
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
pub async fn read_json<T>(path: &Path) -> Result<T, FilesystemError>
where
    T: serde::de::DeserializeOwned,
{
    let content = fs::read(path).await?;
    let data = serde_json::from_slice(&content)?;
    Ok(data)
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
pub async fn write_json<T>(path: &Path, data: &T) -> Result<(), FilesystemError>
where
    T: serde::Serialize,
{
    let content = serde_json::to_vec_pretty(data)?;
    atomic_write(path, &content).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use tempfile::tempdir;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
    }

    #[tokio::test]
    async fn test_ensure_dir() {
        let dir = tempdir().unwrap();
        let nested_path = dir.path().join("a").join("b").join("c");

        ensure_dir(&nested_path).await.unwrap();
        assert!(nested_path.exists());
    }

    #[tokio::test]
    async fn test_atomic_write() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");

        atomic_write(&file_path, b"hello world").await.unwrap();

        let content = fs::read(&file_path).await.unwrap();
        assert_eq!(content, b"hello world");
    }

    #[tokio::test]
    async fn test_read_write_json() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.json");

        let original = TestData {
            name: "test".to_string(),
            value: 42,
        };

        // Write JSON
        write_json(&file_path, &original).await.unwrap();

        // Read JSON back
        let loaded: TestData = read_json(&file_path).await.unwrap();

        assert_eq!(loaded, original);
    }

    #[tokio::test]
    async fn test_read_json_missing_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("nonexistent.json");

        let result: Result<TestData, _> = read_json(&file_path).await;
        assert!(result.is_err());
    }
}
