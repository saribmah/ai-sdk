/// Assistant message type.
pub mod assistant;
/// User message type.
pub mod user;

pub use assistant::AssistantMessage;
pub use user::UserMessage;

use serde::{Deserialize, Serialize};

/// Role of a message sender.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    /// Message from the user
    User,
    /// Message from the assistant
    Assistant,
    /// System message
    System,
}

/// Common message metadata.
///
/// This struct holds metadata about a message, particularly useful for
/// assistant messages to track model information, token usage, and completion details.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MessageMetadata {
    /// Model ID that generated this message (for assistant messages)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,

    /// Provider name (e.g., "openai", "anthropic")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    /// Token usage statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<UsageStats>,

    /// Finish reason (for assistant messages)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,

    /// Custom metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<serde_json::Value>,
}

/// Token usage statistics for a message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UsageStats {
    /// Number of tokens in the prompt
    pub prompt_tokens: u32,
    /// Number of tokens in the completion
    pub completion_tokens: u32,
    /// Total number of tokens
    pub total_tokens: u32,
}

impl UsageStats {
    /// Create new usage statistics.
    pub fn new(prompt_tokens: u32, completion_tokens: u32) -> Self {
        Self {
            prompt_tokens,
            completion_tokens,
            total_tokens: prompt_tokens + completion_tokens,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_role_serialization() {
        assert_eq!(
            serde_json::to_string(&MessageRole::User).unwrap(),
            r#""user""#
        );
        assert_eq!(
            serde_json::to_string(&MessageRole::Assistant).unwrap(),
            r#""assistant""#
        );
        assert_eq!(
            serde_json::to_string(&MessageRole::System).unwrap(),
            r#""system""#
        );
    }

    #[test]
    fn test_usage_stats_creation() {
        let usage = UsageStats::new(100, 50);
        assert_eq!(usage.prompt_tokens, 100);
        assert_eq!(usage.completion_tokens, 50);
        assert_eq!(usage.total_tokens, 150);
    }

    #[test]
    fn test_message_metadata_default() {
        let metadata = MessageMetadata::default();
        assert!(metadata.model_id.is_none());
        assert!(metadata.provider.is_none());
        assert!(metadata.usage.is_none());
        assert!(metadata.finish_reason.is_none());
        assert!(metadata.custom.is_none());
    }

    #[test]
    fn test_message_metadata_serialization() {
        let metadata = MessageMetadata {
            model_id: Some("gpt-4".to_string()),
            provider: Some("openai".to_string()),
            usage: Some(UsageStats::new(100, 50)),
            finish_reason: Some("stop".to_string()),
            custom: None,
        };

        let json = serde_json::to_string(&metadata).unwrap();
        let deserialized: MessageMetadata = serde_json::from_str(&json).unwrap();
        assert_eq!(metadata, deserialized);
    }
}
