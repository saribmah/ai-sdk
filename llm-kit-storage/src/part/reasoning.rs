use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A reasoning/thinking content part within a message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReasoningPart {
    /// Unique identifier for this part
    pub id: String,

    /// The reasoning/thinking content
    pub content: String,

    /// When this part was created
    pub created_at: DateTime<Utc>,

    /// Optional provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<serde_json::Value>,
}

impl ReasoningPart {
    /// Create a new reasoning part with the given ID and content.
    pub fn new(id: String, content: String) -> Self {
        Self {
            id,
            content,
            created_at: Utc::now(),
            provider_metadata: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reasoning_part_creation() {
        let part = ReasoningPart::new("test-id".to_string(), "Thinking...".to_string());
        assert_eq!(part.id, "test-id");
        assert_eq!(part.content, "Thinking...");
        assert!(part.provider_metadata.is_none());
    }

    #[test]
    fn test_reasoning_part_serialization() {
        let part = ReasoningPart::new("test-id".to_string(), "Thinking...".to_string());
        let json = serde_json::to_string(&part).unwrap();
        let deserialized: ReasoningPart = serde_json::from_str(&json).unwrap();
        assert_eq!(part, deserialized);
    }
}
