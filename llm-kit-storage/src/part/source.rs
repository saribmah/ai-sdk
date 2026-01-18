use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A source reference part within a message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourcePart {
    /// Unique identifier for this part
    pub id: String,

    /// Source reference (URL, citation, etc.)
    pub reference: String,

    /// Optional title of the source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// When this part was created
    pub created_at: DateTime<Utc>,

    /// Optional provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<serde_json::Value>,
}

impl SourcePart {
    /// Create a new source part with the given ID and reference.
    pub fn new(id: String, reference: String) -> Self {
        Self {
            id,
            reference,
            title: None,
            created_at: Utc::now(),
            provider_metadata: None,
        }
    }

    /// Set the title for this source.
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_part_creation() {
        let part = SourcePart::new("test-id".to_string(), "https://example.com".to_string());
        assert_eq!(part.id, "test-id");
        assert_eq!(part.reference, "https://example.com");
        assert!(part.title.is_none());
    }

    #[test]
    fn test_source_part_with_title() {
        let part = SourcePart::new("test-id".to_string(), "https://example.com".to_string())
            .with_title("Example".to_string());
        assert_eq!(part.title, Some("Example".to_string()));
    }

    #[test]
    fn test_source_part_serialization() {
        let part = SourcePart::new("test-id".to_string(), "https://example.com".to_string());
        let json = serde_json::to_string(&part).unwrap();
        let deserialized: SourcePart = serde_json::from_str(&json).unwrap();
        assert_eq!(part, deserialized);
    }
}
