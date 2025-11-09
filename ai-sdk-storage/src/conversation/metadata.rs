//! Metadata types for conversation messages.

use serde::{Deserialize, Serialize};

/// Token usage statistics for a message.
///
/// Tracks token consumption for both input (prompt) and output (completion).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageStats {
    /// Number of tokens in the prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens: Option<u32>,

    /// Number of tokens in the completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<u32>,

    /// Total number of tokens (prompt + completion)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<u32>,
}

/// Record of a tool call made during message generation.
///
/// This captures information about tools that were called by the assistant,
/// including arguments, results, and any errors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallRecord {
    /// Name of the tool that was called
    pub tool_name: String,

    /// Arguments passed to the tool (as JSON)
    pub tool_arguments: serde_json::Value,

    /// Result returned by the tool (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_result: Option<String>,

    /// Error message (if tool execution failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Metadata associated with a message.
///
/// This type captures additional information about how a message was generated,
/// including model details, token usage, finish reason, and tool calls.
///
/// # Example
///
/// ```rust
/// use ai_sdk_storage::{MessageMetadata, UsageStats};
///
/// let metadata = MessageMetadata {
///     model_id: Some("gpt-4".to_string()),
///     provider: Some("openai".to_string()),
///     usage: Some(UsageStats {
///         prompt_tokens: Some(10),
///         completion_tokens: Some(20),
///         total_tokens: Some(30),
///     }),
///     finish_reason: Some("stop".to_string()),
///     tool_calls: None,
///     custom: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MessageMetadata {
    /// ID of the model that generated this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,

    /// Provider that was used (e.g., "openai", "anthropic")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    /// Token usage statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<UsageStats>,

    /// Reason why generation finished (e.g., "stop", "length", "tool_calls")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,

    /// Records of tool calls made during generation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallRecord>>,

    /// Custom metadata (provider-specific or application-specific)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_stats_default() {
        let stats = UsageStats::default();
        assert!(stats.prompt_tokens.is_none());
        assert!(stats.completion_tokens.is_none());
        assert!(stats.total_tokens.is_none());
    }

    #[test]
    fn test_usage_stats_serialization() {
        let stats = UsageStats {
            prompt_tokens: Some(100),
            completion_tokens: Some(50),
            total_tokens: Some(150),
        };

        let json = serde_json::to_string(&stats).unwrap();
        let deserialized: UsageStats = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.prompt_tokens, stats.prompt_tokens);
        assert_eq!(deserialized.completion_tokens, stats.completion_tokens);
        assert_eq!(deserialized.total_tokens, stats.total_tokens);
    }

    #[test]
    fn test_tool_call_record_serialization() {
        let record = ToolCallRecord {
            tool_name: "get_weather".to_string(),
            tool_arguments: serde_json::json!({"city": "San Francisco"}),
            tool_result: Some("Sunny, 72°F".to_string()),
            error: None,
        };

        let json = serde_json::to_string(&record).unwrap();
        let deserialized: ToolCallRecord = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.tool_name, record.tool_name);
        assert_eq!(deserialized.tool_result, record.tool_result);
    }

    #[test]
    fn test_message_metadata_default() {
        let metadata = MessageMetadata::default();
        assert!(metadata.model_id.is_none());
        assert!(metadata.provider.is_none());
        assert!(metadata.usage.is_none());
        assert!(metadata.finish_reason.is_none());
        assert!(metadata.tool_calls.is_none());
        assert!(metadata.custom.is_none());
    }

    #[test]
    fn test_message_metadata_serialization() {
        let metadata = MessageMetadata {
            model_id: Some("gpt-4".to_string()),
            provider: Some("openai".to_string()),
            usage: Some(UsageStats {
                prompt_tokens: Some(10),
                completion_tokens: Some(20),
                total_tokens: Some(30),
            }),
            finish_reason: Some("stop".to_string()),
            tool_calls: Some(vec![ToolCallRecord {
                tool_name: "test_tool".to_string(),
                tool_arguments: serde_json::json!({}),
                tool_result: Some("success".to_string()),
                error: None,
            }]),
            custom: None,
        };

        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: MessageMetadata = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.model_id, metadata.model_id);
        assert_eq!(deserialized.provider, metadata.provider);
        assert_eq!(deserialized.finish_reason, metadata.finish_reason);
    }

    #[test]
    fn test_metadata_omits_none_fields() {
        let metadata = MessageMetadata {
            model_id: Some("gpt-4".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&metadata).unwrap();
        // Verify that None fields are omitted
        assert!(!json.contains("\"provider\""));
        assert!(!json.contains("\"usage\""));
        assert!(json.contains("\"model_id\""));
    }
}
