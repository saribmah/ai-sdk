use llm_kit_provider_utils::message::data_content::DataContent;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// A generated file.
///
/// This represents a file with its content and metadata. The file content
/// can be provided as either base64-encoded string or raw bytes, and will
/// be lazily converted between formats as needed.
#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedFile {
    /// Internal storage for the file data (either base64 or bytes).
    data: DataContent,

    /// Cached base64 representation (lazily computed).
    #[serde(skip)]
    cached_base64: OnceLock<String>,

    /// Cached bytes representation (lazily computed).
    #[serde(skip)]
    cached_bytes: OnceLock<Vec<u8>>,

    /// The IANA media type of the file.
    ///
    /// See <https://www.iana.org/assignments/media-types/media-types.xhtml>
    pub media_type: String,

    /// Optional filename.
    pub name: Option<String>,
}

impl GeneratedFile {
    /// Creates a new `GeneratedFile` from base64-encoded data.
    ///
    /// # Arguments
    ///
    /// * `base64` - The file content as a base64-encoded string
    /// * `media_type` - The IANA media type of the file
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_core::generate_text::GeneratedFile;
    ///
    /// let file = GeneratedFile::from_base64("SGVsbG8gV29ybGQh", "text/plain");
    /// assert_eq!(file.media_type, "text/plain");
    /// ```
    pub fn from_base64(base64: impl Into<String>, media_type: impl Into<String>) -> Self {
        let base64_str = base64.into();
        let cached_base64 = OnceLock::new();
        let _ = cached_base64.set(base64_str.clone());
        Self {
            data: DataContent::base64(&base64_str),
            cached_base64,
            cached_bytes: OnceLock::new(),
            media_type: media_type.into(),
            name: None,
        }
    }

    /// Creates a new `GeneratedFile` from raw bytes.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The file content as raw bytes
    /// * `media_type` - The IANA media type of the file
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_core::generate_text::GeneratedFile;
    ///
    /// let file = GeneratedFile::from_bytes(b"Hello World!", "text/plain");
    /// assert_eq!(file.media_type, "text/plain");
    /// ```
    pub fn from_bytes(bytes: impl Into<Vec<u8>>, media_type: impl Into<String>) -> Self {
        let bytes_vec = bytes.into();
        let cached_bytes = OnceLock::new();
        let _ = cached_bytes.set(bytes_vec.clone());
        Self {
            data: DataContent::bytes(bytes_vec),
            cached_base64: OnceLock::new(),
            cached_bytes,
            media_type: media_type.into(),
            name: None,
        }
    }

    /// Sets the filename for this file.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_core::generate_text::GeneratedFile;
    ///
    /// let file = GeneratedFile::from_base64("SGVsbG8gV29ybGQh", "text/plain")
    ///     .with_name("hello.txt");
    /// assert_eq!(file.name, Some("hello.txt".to_string()));
    /// ```
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Gets the file content as a base64-encoded string.
    ///
    /// The conversion is lazy and cached, so subsequent calls will return
    /// the same string without re-computing the conversion.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_core::generate_text::GeneratedFile;
    ///
    /// let file = GeneratedFile::from_bytes(b"Hello World!", "text/plain");
    /// let base64 = file.base64();
    /// assert_eq!(base64, "SGVsbG8gV29ybGQh");
    /// ```
    pub fn base64(&self) -> &str {
        self.cached_base64.get_or_init(|| self.data.to_base64())
    }

    /// Gets the file content as raw bytes.
    ///
    /// The conversion is lazy and cached, so subsequent calls will return
    /// the same bytes without re-computing the conversion.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_core::generate_text::GeneratedFile;
    ///
    /// let file = GeneratedFile::from_base64("SGVsbG8gV29ybGQh", "text/plain");
    /// let bytes = file.bytes();
    /// assert_eq!(bytes, b"Hello World!");
    /// ```
    pub fn bytes(&self) -> &[u8] {
        self.cached_bytes
            .get_or_init(|| self.data.to_bytes().unwrap_or_default())
    }

    /// Gets the file content as a `Vec<u8>`.
    ///
    /// This is a convenience method that clones the bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_core::generate_text::GeneratedFile;
    ///
    /// let file = GeneratedFile::from_base64("SGVsbG8gV29ybGQh", "text/plain");
    /// let bytes = file.to_vec();
    /// assert_eq!(bytes, b"Hello World!");
    /// ```
    pub fn to_vec(&self) -> Vec<u8> {
        self.bytes().to_vec()
    }
}

impl Clone for GeneratedFile {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            cached_base64: OnceLock::new(),
            cached_bytes: OnceLock::new(),
            media_type: self.media_type.clone(),
            name: self.name.clone(),
        }
    }
}

impl PartialEq for GeneratedFile {
    fn eq(&self, other: &Self) -> bool {
        // Compare by bytes to ensure equality regardless of internal representation
        self.bytes() == other.bytes()
            && self.media_type == other.media_type
            && self.name == other.name
    }
}

impl Eq for GeneratedFile {}

/// A generated file with a type field.
///
/// This is an extension of `GeneratedFile` that includes a `type` field
/// set to "file", useful for serialization and type identification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedFileWithType {
    /// The underlying file data.
    #[allow(dead_code)]
    file: GeneratedFile,

    /// The type field, always set to "file".
    pub r#type: String,
}

impl GeneratedFileWithType {
    /// Creates a new `GeneratedFileWithType` from a `GeneratedFile`.
    pub fn new(file: GeneratedFile) -> Self {
        Self {
            file,
            r#type: "file".to_string(),
        }
    }

    /// Creates a new `GeneratedFileWithType` from base64-encoded data.
    pub fn from_base64(base64: impl Into<String>, media_type: impl Into<String>) -> Self {
        Self::new(GeneratedFile::from_base64(base64, media_type))
    }

    /// Creates a new `GeneratedFileWithType` from raw bytes.
    pub fn from_bytes(bytes: impl Into<Vec<u8>>, media_type: impl Into<String>) -> Self {
        Self::new(GeneratedFile::from_bytes(bytes, media_type))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_base64() {
        let file = GeneratedFile::from_base64("SGVsbG8gV29ybGQh", "text/plain");
        assert_eq!(file.media_type, "text/plain");
        assert_eq!(file.base64(), "SGVsbG8gV29ybGQh");
        assert_eq!(file.bytes(), b"Hello World!");
    }

    #[test]
    fn test_from_bytes() {
        let file = GeneratedFile::from_bytes(b"Hello World!".to_vec(), "text/plain");
        assert_eq!(file.media_type, "text/plain");
        assert_eq!(file.bytes(), b"Hello World!");
        assert_eq!(file.base64(), "SGVsbG8gV29ybGQh");
    }

    #[test]
    fn test_with_name() {
        let file =
            GeneratedFile::from_base64("SGVsbG8gV29ybGQh", "text/plain").with_name("hello.txt");
        assert_eq!(file.name, Some("hello.txt".to_string()));
    }

    #[test]
    fn test_lazy_conversion_base64_to_bytes() {
        let file = GeneratedFile::from_base64("SGVsbG8gV29ybGQh", "text/plain");
        // First access converts and caches
        let bytes1 = file.bytes();
        // Second access should return cached value
        let bytes2 = file.bytes();
        assert_eq!(bytes1, bytes2);
        assert_eq!(bytes1, b"Hello World!");
    }

    #[test]
    fn test_lazy_conversion_bytes_to_base64() {
        let file = GeneratedFile::from_bytes(b"Hello World!".to_vec(), "text/plain");
        // First access converts and caches
        let base64_1 = file.base64();
        // Second access should return cached value
        let base64_2 = file.base64();
        assert_eq!(base64_1, base64_2);
        assert_eq!(base64_1, "SGVsbG8gV29ybGQh");
    }

    #[test]
    fn test_equality_same_representation() {
        let file1 = GeneratedFile::from_base64("SGVsbG8gV29ybGQh", "text/plain");
        let file2 = GeneratedFile::from_base64("SGVsbG8gV29ybGQh", "text/plain");
        assert_eq!(file1, file2);
    }

    #[test]
    fn test_equality_different_representation() {
        let file1 = GeneratedFile::from_base64("SGVsbG8gV29ybGQh", "text/plain");
        let file2 = GeneratedFile::from_bytes(b"Hello World!".to_vec(), "text/plain");
        assert_eq!(file1, file2);
    }

    #[test]
    fn test_inequality_different_content() {
        let file1 = GeneratedFile::from_base64("SGVsbG8gV29ybGQh", "text/plain");
        let file2 = GeneratedFile::from_base64("R29vZGJ5ZQ==", "text/plain");
        assert_ne!(file1, file2);
    }

    #[test]
    fn test_inequality_different_media_type() {
        let file1 = GeneratedFile::from_base64("SGVsbG8gV29ybGQh", "text/plain");
        let file2 = GeneratedFile::from_base64("SGVsbG8gV29ybGQh", "text/html");
        assert_ne!(file1, file2);
    }

    #[test]
    fn test_to_vec() {
        let file = GeneratedFile::from_base64("SGVsbG8gV29ybGQh", "text/plain");
        let vec = file.to_vec();
        assert_eq!(vec, b"Hello World!");
    }

    #[test]
    fn test_generated_file_with_type() {
        let file = GeneratedFileWithType::from_base64("SGVsbG8gV29ybGQh", "text/plain");
        assert_eq!(file.r#type, "file");
    }

    #[test]
    fn test_generated_file_with_type_from_bytes() {
        let file = GeneratedFileWithType::from_bytes(b"Hello World!".to_vec(), "text/plain");
        assert_eq!(file.r#type, "file");
    }

    #[test]
    fn test_clone() {
        let file1 =
            GeneratedFile::from_base64("SGVsbG8gV29ybGQh", "text/plain").with_name("test.txt");
        let file2 = file1.clone();
        assert_eq!(file1, file2);
        assert_eq!(file1.name, file2.name);
    }
}
