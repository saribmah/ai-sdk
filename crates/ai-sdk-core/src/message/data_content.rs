use base64::Engine;
use serde::{Deserialize, Serialize};

/// Data content. Can either be a base64-encoded string or raw bytes.
///
/// This type represents binary data that can be included in messages,
/// such as images, audio, or other file attachments.
///
/// # Examples
///
/// ```
/// use ai_sdk_core::message::DataContent;
///
/// // From a base64-encoded string
/// let data = DataContent::from("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==");
///
/// // From raw bytes
/// let bytes = vec![0x89, 0x50, 0x4E, 0x47];
/// let data = DataContent::from(bytes);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DataContent {
    /// Base64-encoded string
    Base64(String),

    /// Raw bytes (equivalent to Uint8Array, ArrayBuffer, or Buffer in TypeScript/Node.js)
    #[serde(with = "serde_bytes")]
    Bytes(Vec<u8>),
}

impl DataContent {
    /// Creates a new `DataContent` from a base64-encoded string.
    pub fn base64(s: impl Into<String>) -> Self {
        Self::Base64(s.into())
    }

    /// Creates a new `DataContent` from raw bytes.
    pub fn bytes(b: Vec<u8>) -> Self {
        Self::Bytes(b)
    }

    /// Returns `true` if this is a base64-encoded string.
    pub fn is_base64(&self) -> bool {
        matches!(self, Self::Base64(_))
    }

    /// Returns `true` if this is raw bytes.
    pub fn is_bytes(&self) -> bool {
        matches!(self, Self::Bytes(_))
    }

    /// Converts the data content to raw bytes.
    ///
    /// If the content is already bytes, returns a clone.
    /// If the content is base64, decodes it and returns the bytes.
    ///
    /// # Errors
    /// Returns an error if the base64 string cannot be decoded.
    pub fn to_bytes(&self) -> Result<Vec<u8>, base64::DecodeError> {
        match self {
            Self::Bytes(b) => Ok(b.clone()),
            Self::Base64(s) => base64::prelude::BASE64_STANDARD.decode(s),
        }
    }

    /// Converts the data content to a base64-encoded string.
    ///
    /// If the content is already base64, returns a clone of the string.
    /// If the content is bytes, encodes it to base64.
    pub fn to_base64(&self) -> String {
        match self {
            Self::Base64(s) => s.clone(),
            Self::Bytes(b) => base64::prelude::BASE64_STANDARD.encode(b),
        }
    }
}

// Conversion traits for ergonomic usage

impl From<String> for DataContent {
    fn from(s: String) -> Self {
        Self::Base64(s)
    }
}

impl From<&str> for DataContent {
    fn from(s: &str) -> Self {
        Self::Base64(s.to_string())
    }
}

impl From<Vec<u8>> for DataContent {
    fn from(b: Vec<u8>) -> Self {
        Self::Bytes(b)
    }
}

impl From<&[u8]> for DataContent {
    fn from(b: &[u8]) -> Self {
        Self::Bytes(b.to_vec())
    }
}

// Allow converting from fixed-size byte arrays
impl<const N: usize> From<[u8; N]> for DataContent {
    fn from(b: [u8; N]) -> Self {
        Self::Bytes(b.to_vec())
    }
}

impl<const N: usize> From<&[u8; N]> for DataContent {
    fn from(b: &[u8; N]) -> Self {
        Self::Bytes(b.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_content_base64() {
        let data = DataContent::base64("SGVsbG8gV29ybGQ=");
        assert!(data.is_base64());
        assert!(!data.is_bytes());
    }

    #[test]
    fn test_data_content_bytes() {
        let data = DataContent::bytes(vec![1, 2, 3, 4]);
        assert!(data.is_bytes());
        assert!(!data.is_base64());
    }

    #[test]
    fn test_from_string() {
        let data = DataContent::from("SGVsbG8gV29ybGQ=");
        assert_eq!(data, DataContent::Base64("SGVsbG8gV29ybGQ=".to_string()));
    }

    #[test]
    fn test_from_vec_u8() {
        let data = DataContent::from(vec![1, 2, 3, 4]);
        assert_eq!(data, DataContent::Bytes(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_from_slice() {
        let bytes: &[u8] = &[1, 2, 3, 4];
        let data = DataContent::from(bytes);
        assert_eq!(data, DataContent::Bytes(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_from_array() {
        let data = DataContent::from([1u8, 2, 3, 4]);
        assert_eq!(data, DataContent::Bytes(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_to_bytes_from_bytes() {
        let data = DataContent::bytes(vec![1, 2, 3, 4]);
        let bytes = data.to_bytes().unwrap();
        assert_eq!(bytes, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_to_bytes_from_base64() {
        // "Hello World" in base64
        let data = DataContent::base64("SGVsbG8gV29ybGQ=");
        let bytes = data.to_bytes().unwrap();
        assert_eq!(bytes, b"Hello World");
    }

    #[test]
    fn test_to_base64_from_base64() {
        let data = DataContent::base64("SGVsbG8gV29ybGQ=");
        let base64 = data.to_base64();
        assert_eq!(base64, "SGVsbG8gV29ybGQ=");
    }

    #[test]
    fn test_to_base64_from_bytes() {
        let data = DataContent::bytes(b"Hello World".to_vec());
        let base64 = data.to_base64();
        assert_eq!(base64, "SGVsbG8gV29ybGQ=");
    }

    #[test]
    fn test_to_bytes_invalid_base64() {
        let data = DataContent::base64("invalid!@#$%");
        let result = data.to_bytes();
        assert!(result.is_err());
    }

    #[test]
    fn test_round_trip_bytes() {
        let original = vec![0x89, 0x50, 0x4E, 0x47];
        let data = DataContent::from(original.clone());
        let base64 = data.to_base64();
        let data2 = DataContent::base64(base64);
        let bytes = data2.to_bytes().unwrap();
        assert_eq!(bytes, original);
    }
}
