use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// File data storage format.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "format", rename_all = "lowercase")]
pub enum FileData {
    /// Base64-encoded file data
    Base64 {
        /// Base64-encoded string
        data: String,
    },
    /// Raw binary file data
    Binary {
        /// Binary data
        data: Vec<u8>,
    },
    /// URL pointing to the file
    Url {
        /// File URL
        url: String,
    },
}

/// A file content part within a message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FilePart {
    /// Unique identifier for this part
    pub id: String,

    /// Media type (MIME type)
    pub media_type: String,

    /// File data
    pub data: FileData,

    /// Optional filename
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// When this part was created
    pub created_at: DateTime<Utc>,

    /// Optional provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<serde_json::Value>,
}

impl FilePart {
    /// Create a new file part with the given parameters.
    pub fn new(id: String, media_type: String, data: FileData) -> Self {
        Self {
            id,
            media_type,
            data,
            filename: None,
            created_at: Utc::now(),
            provider_metadata: None,
        }
    }

    /// Set the filename for this file.
    pub fn with_filename(mut self, filename: String) -> Self {
        self.filename = Some(filename);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_part_creation() {
        let data = FileData::Url {
            url: "https://example.com/file.pdf".to_string(),
        };
        let part = FilePart::new("file-id".to_string(), "application/pdf".to_string(), data);
        assert_eq!(part.id, "file-id");
        assert_eq!(part.media_type, "application/pdf");
        assert!(part.filename.is_none());
    }

    #[test]
    fn test_file_part_with_filename() {
        let data = FileData::Url {
            url: "https://example.com/file.pdf".to_string(),
        };
        let part = FilePart::new("file-id".to_string(), "application/pdf".to_string(), data)
            .with_filename("document.pdf".to_string());
        assert_eq!(part.filename, Some("document.pdf".to_string()));
    }

    #[test]
    fn test_file_part_serialization() {
        let data = FileData::Base64 {
            data: "base64filedata".to_string(),
        };
        let part = FilePart::new("file-id".to_string(), "application/pdf".to_string(), data);
        let json = serde_json::to_string(&part).unwrap();
        let deserialized: FilePart = serde_json::from_str(&json).unwrap();
        assert_eq!(part, deserialized);
    }

    #[test]
    fn test_file_data_variants() {
        // Test Base64
        let base64 = FileData::Base64 {
            data: "test".to_string(),
        };
        let json = serde_json::to_string(&base64).unwrap();
        assert!(json.contains(r#""format":"base64""#));

        // Test URL
        let url = FileData::Url {
            url: "https://example.com".to_string(),
        };
        let json = serde_json::to_string(&url).unwrap();
        assert!(json.contains(r#""format":"url""#));

        // Test Binary
        let binary = FileData::Binary {
            data: vec![1, 2, 3],
        };
        let json = serde_json::to_string(&binary).unwrap();
        assert!(json.contains(r#""format":"binary""#));
    }
}
