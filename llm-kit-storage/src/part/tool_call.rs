use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A tool call part within a message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolCallPart {
    /// Unique identifier for this part
    pub id: String,

    /// Tool call identifier (for matching with results)
    pub tool_call_id: String,

    /// Name of the tool being called
    pub tool_name: String,

    /// Tool arguments (strongly typed JSON)
    pub arguments: serde_json::Value,

    /// When this part was created
    pub created_at: DateTime<Utc>,

    /// Optional provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<serde_json::Value>,
}

impl ToolCallPart {
    /// Create a new tool call part with the given parameters.
    pub fn new(
        id: String,
        tool_call_id: String,
        tool_name: String,
        arguments: serde_json::Value,
    ) -> Self {
        Self {
            id,
            tool_call_id,
            tool_name,
            arguments,
            created_at: Utc::now(),
            provider_metadata: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_tool_call_part_creation() {
        let args = json!({"query": "test"});
        let part = ToolCallPart::new(
            "part-id".to_string(),
            "call-id".to_string(),
            "search".to_string(),
            args.clone(),
        );
        assert_eq!(part.id, "part-id");
        assert_eq!(part.tool_call_id, "call-id");
        assert_eq!(part.tool_name, "search");
        assert_eq!(part.arguments, args);
    }

    #[test]
    fn test_tool_call_part_serialization() {
        let args = json!({"query": "test"});
        let part = ToolCallPart::new(
            "part-id".to_string(),
            "call-id".to_string(),
            "search".to_string(),
            args,
        );
        let json = serde_json::to_string(&part).unwrap();
        let deserialized: ToolCallPart = serde_json::from_str(&json).unwrap();
        assert_eq!(part, deserialized);
    }
}
