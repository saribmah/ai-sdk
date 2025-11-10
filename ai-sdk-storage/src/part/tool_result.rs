use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Result data from a tool execution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum ToolResultData {
    /// Successful tool execution
    Success {
        /// Output from the tool
        output: serde_json::Value,
    },
    /// Failed tool execution
    Error {
        /// Error message
        error: String,
    },
}

/// A tool result part within a message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolResultPart {
    /// Unique identifier for this part
    pub id: String,

    /// Tool call identifier this result corresponds to
    pub tool_call_id: String,

    /// Name of the tool that was called
    pub tool_name: String,

    /// Result data
    pub result: ToolResultData,

    /// When this part was created
    pub created_at: DateTime<Utc>,

    /// Optional provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<serde_json::Value>,
}

impl ToolResultPart {
    /// Create a new tool result part with a successful result.
    pub fn new_success(
        id: String,
        tool_call_id: String,
        tool_name: String,
        output: serde_json::Value,
    ) -> Self {
        Self {
            id,
            tool_call_id,
            tool_name,
            result: ToolResultData::Success { output },
            created_at: Utc::now(),
            provider_metadata: None,
        }
    }

    /// Create a new tool result part with an error result.
    pub fn new_error(id: String, tool_call_id: String, tool_name: String, error: String) -> Self {
        Self {
            id,
            tool_call_id,
            tool_name,
            result: ToolResultData::Error { error },
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
    fn test_tool_result_part_success() {
        let output = json!({"results": ["a", "b"]});
        let part = ToolResultPart::new_success(
            "part-id".to_string(),
            "call-id".to_string(),
            "search".to_string(),
            output.clone(),
        );
        assert_eq!(part.id, "part-id");
        assert_eq!(part.tool_call_id, "call-id");
        assert_eq!(part.tool_name, "search");
        match &part.result {
            ToolResultData::Success { output: o } => assert_eq!(o, &output),
            _ => panic!("Expected Success variant"),
        }
    }

    #[test]
    fn test_tool_result_part_error() {
        let part = ToolResultPart::new_error(
            "part-id".to_string(),
            "call-id".to_string(),
            "search".to_string(),
            "Connection failed".to_string(),
        );
        match &part.result {
            ToolResultData::Error { error } => assert_eq!(error, "Connection failed"),
            _ => panic!("Expected Error variant"),
        }
    }

    #[test]
    fn test_tool_result_part_serialization() {
        let output = json!({"results": ["a", "b"]});
        let part = ToolResultPart::new_success(
            "part-id".to_string(),
            "call-id".to_string(),
            "search".to_string(),
            output,
        );
        let json = serde_json::to_string(&part).unwrap();
        let deserialized: ToolResultPart = serde_json::from_str(&json).unwrap();
        assert_eq!(part, deserialized);
    }
}
