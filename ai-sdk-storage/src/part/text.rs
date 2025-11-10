use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A text content part within a message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextPart {
    /// Unique identifier for this part
    pub id: String,

    /// The text content
    pub text: String,

    /// When this part was created
    pub created_at: DateTime<Utc>,

    /// Optional provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<serde_json::Value>,
}

impl TextPart {
    /// Create a new text part with the given ID and text content.
    pub fn new(id: String, text: String) -> Self {
        Self {
            id,
            text,
            created_at: Utc::now(),
            provider_metadata: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_part_creation() {
        let part = TextPart::new("test-id".to_string(), "Hello".to_string());
        assert_eq!(part.id, "test-id");
        assert_eq!(part.text, "Hello");
        assert!(part.provider_metadata.is_none());
    }

    #[test]
    fn test_text_part_serialization() {
        let part = TextPart::new("test-id".to_string(), "Hello".to_string());
        let json = serde_json::to_string(&part).unwrap();
        let deserialized: TextPart = serde_json::from_str(&json).unwrap();
        assert_eq!(part.id, deserialized.id);
        assert_eq!(part.text, deserialized.text);
    }
}
