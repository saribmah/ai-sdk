/// File content part.
pub mod file;
/// Image content part.
pub mod image;
/// Reasoning content part.
pub mod reasoning;
/// Source reference part.
pub mod source;
/// Text content part.
pub mod text;
/// Tool call part.
pub mod tool_call;
/// Tool result part.
pub mod tool_result;

pub use file::{FileData, FilePart};
pub use image::{ImageData, ImageDimensions, ImagePart};
pub use reasoning::ReasoningPart;
pub use source::SourcePart;
pub use text::TextPart;
pub use tool_call::ToolCallPart;
pub use tool_result::{ToolResultData, ToolResultPart};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Unified part type for storage with tagged union serialization.
///
/// This enum represents all possible content parts that can appear in a message.
/// Each variant wraps a specific part type with its own fields and behavior.
///
/// # Serialization Format
///
/// Parts are serialized using a tagged union format with `"type"` as the discriminator:
///
/// ```json
/// {
///   "type": "text",
///   "id": "part-123",
///   "text": "Hello world",
///   "created_at": "2024-01-01T00:00:00Z"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum MessagePart {
    /// Text content
    Text(TextPart),
    /// Image content
    Image(ImagePart),
    /// File attachment
    File(FilePart),
    /// Reasoning/thinking content
    Reasoning(ReasoningPart),
    /// Tool call invocation
    ToolCall(ToolCallPart),
    /// Tool execution result
    ToolResult(ToolResultPart),
    /// Source reference
    Source(SourcePart),
}

impl MessagePart {
    /// Get the ID of this part regardless of variant.
    pub fn id(&self) -> &str {
        match self {
            MessagePart::Text(p) => &p.id,
            MessagePart::Image(p) => &p.id,
            MessagePart::File(p) => &p.id,
            MessagePart::Reasoning(p) => &p.id,
            MessagePart::ToolCall(p) => &p.id,
            MessagePart::ToolResult(p) => &p.id,
            MessagePart::Source(p) => &p.id,
        }
    }

    /// Get the creation timestamp of this part regardless of variant.
    pub fn created_at(&self) -> DateTime<Utc> {
        match self {
            MessagePart::Text(p) => p.created_at,
            MessagePart::Image(p) => p.created_at,
            MessagePart::File(p) => p.created_at,
            MessagePart::Reasoning(p) => p.created_at,
            MessagePart::ToolCall(p) => p.created_at,
            MessagePart::ToolResult(p) => p.created_at,
            MessagePart::Source(p) => p.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_part_id() {
        let part = MessagePart::Text(TextPart::new("test-id".to_string(), "hello".to_string()));
        assert_eq!(part.id(), "test-id");
    }

    #[test]
    fn test_message_part_created_at() {
        let part = MessagePart::Text(TextPart::new("test-id".to_string(), "hello".to_string()));
        let created = part.created_at();
        // Just verify it's a valid timestamp
        assert!(created.timestamp() > 0);
    }

    #[test]
    fn test_message_part_serialization_text() {
        let part = MessagePart::Text(TextPart::new("test-id".to_string(), "hello".to_string()));
        let json = serde_json::to_string(&part).unwrap();
        assert!(json.contains(r#""type":"text""#));

        let deserialized: MessagePart = serde_json::from_str(&json).unwrap();
        assert_eq!(part.id(), deserialized.id());
    }

    #[test]
    fn test_message_part_serialization_reasoning() {
        let part = MessagePart::Reasoning(ReasoningPart::new(
            "test-id".to_string(),
            "thinking...".to_string(),
        ));
        let json = serde_json::to_string(&part).unwrap();
        assert!(json.contains(r#""type":"reasoning""#));
    }

    #[test]
    fn test_message_part_serialization_tool_call() {
        use serde_json::json;
        let part = MessagePart::ToolCall(ToolCallPart::new(
            "test-id".to_string(),
            "call-id".to_string(),
            "search".to_string(),
            json!({"query": "test"}),
        ));
        let json = serde_json::to_string(&part).unwrap();
        assert!(json.contains(r#""type":"tool-call""#));
    }
}
