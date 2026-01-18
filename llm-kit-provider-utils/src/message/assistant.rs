use super::content_parts::{FilePart, ReasoningPart, TextPart, ToolCallPart, ToolResultPart};
use crate::tool::ToolApprovalRequest;
use llm_kit_provider::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};

/// Content of an assistant message.
/// It can be a string or an array of text, file, reasoning, and tool call parts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantContent {
    /// Simple text content as a string.
    Text(String),
    /// Array of content parts (text, files, reasoning, tool calls, tool results, or tool approval requests).
    Parts(Vec<AssistantContentPart>),
}

/// A part of assistant content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantContentPart {
    /// Text content part.
    Text(TextPart),
    /// File attachment part.
    File(FilePart),
    /// Reasoning or thought process part.
    Reasoning(ReasoningPart),
    /// Tool call part (function to invoke).
    ToolCall(ToolCallPart),
    /// Tool result part (result of a tool execution).
    ToolResult(ToolResultPart),
    /// Tool approval request part (request to approve a tool call).
    ToolApprovalRequest(ToolApprovalRequest),
}

/// An assistant message. It can contain text, tool calls, or a combination of text and tool calls.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssistantMessage {
    /// Role is always 'assistant' for assistant messages.
    pub role: String,

    /// Content of the assistant message.
    pub content: AssistantContent,

    /// Additional provider-specific metadata. They are passed through
    /// to the provider from the LLM Kit and enable provider-specific
    /// functionality that can be fully encapsulated in the provider.
    #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<SharedProviderOptions>,
}

impl AssistantMessage {
    /// Creates a new assistant model message with text content.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: AssistantContent::Text(content.into()),
            provider_options: None,
        }
    }

    /// Creates a new assistant model message with multi-part content.
    pub fn with_parts(parts: Vec<AssistantContentPart>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: AssistantContent::Parts(parts),
            provider_options: None,
        }
    }

    /// Sets the provider options for this message.
    pub fn with_provider_options(mut self, options: SharedProviderOptions) -> Self {
        self.provider_options = Some(options);
        self
    }
}

impl From<String> for AssistantContent {
    fn from(s: String) -> Self {
        AssistantContent::Text(s)
    }
}

impl From<&str> for AssistantContent {
    fn from(s: &str) -> Self {
        AssistantContent::Text(s.to_string())
    }
}

impl From<Vec<AssistantContentPart>> for AssistantContent {
    fn from(parts: Vec<AssistantContentPart>) -> Self {
        AssistantContent::Parts(parts)
    }
}

impl From<TextPart> for AssistantContentPart {
    fn from(part: TextPart) -> Self {
        AssistantContentPart::Text(part)
    }
}

impl From<FilePart> for AssistantContentPart {
    fn from(part: FilePart) -> Self {
        AssistantContentPart::File(part)
    }
}

impl From<ReasoningPart> for AssistantContentPart {
    fn from(part: ReasoningPart) -> Self {
        AssistantContentPart::Reasoning(part)
    }
}

impl From<ToolCallPart> for AssistantContentPart {
    fn from(part: ToolCallPart) -> Self {
        AssistantContentPart::ToolCall(part)
    }
}

impl From<ToolResultPart> for AssistantContentPart {
    fn from(part: ToolResultPart) -> Self {
        AssistantContentPart::ToolResult(part)
    }
}

impl From<ToolApprovalRequest> for AssistantContentPart {
    fn from(part: ToolApprovalRequest) -> Self {
        AssistantContentPart::ToolApprovalRequest(part)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::content_parts::ToolResultOutput;
    use url::Url;

    #[test]
    fn test_assistant_model_message_new_text() {
        let message = AssistantMessage::new("Hello, how can I help you?");

        assert_eq!(message.role, "assistant");
        match message.content {
            AssistantContent::Text(text) => assert_eq!(text, "Hello, how can I help you?"),
            _ => panic!("Expected Text variant"),
        }
        assert!(message.provider_options.is_none());
    }

    #[test]
    fn test_assistant_model_message_with_parts() {
        let parts = vec![
            AssistantContentPart::Text(TextPart::new("Let me help you with that.")),
            AssistantContentPart::ToolCall(ToolCallPart::new(
                "call_123",
                "get_weather",
                serde_json::json!({"city": "San Francisco"}),
            )),
        ];

        let message = AssistantMessage::with_parts(parts);

        assert_eq!(message.role, "assistant");
        match message.content {
            AssistantContent::Parts(parts) => {
                assert_eq!(parts.len(), 2);
                assert!(matches!(parts[0], AssistantContentPart::Text(_)));
                assert!(matches!(parts[1], AssistantContentPart::ToolCall(_)));
            }
            _ => panic!("Expected Parts variant"),
        }
    }

    #[test]
    fn test_assistant_model_message_with_provider_options() {
        let provider_options = SharedProviderOptions::new();

        let message =
            AssistantMessage::new("Hello").with_provider_options(provider_options.clone());

        assert_eq!(message.provider_options, Some(provider_options));
    }

    #[test]
    fn test_assistant_content_from_string() {
        let content: AssistantContent = "Hello".to_string().into();

        match content {
            AssistantContent::Text(text) => assert_eq!(text, "Hello"),
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_assistant_content_from_str() {
        let content: AssistantContent = "Hello".into();

        match content {
            AssistantContent::Text(text) => assert_eq!(text, "Hello"),
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_assistant_content_from_parts() {
        let parts = vec![AssistantContentPart::Text(TextPart::new("Hello"))];
        let content: AssistantContent = parts.into();

        match content {
            AssistantContent::Parts(parts) => assert_eq!(parts.len(), 1),
            _ => panic!("Expected Parts variant"),
        }
    }

    #[test]
    fn test_assistant_content_part_from_text() {
        let part: AssistantContentPart = TextPart::new("Hello").into();

        assert!(matches!(part, AssistantContentPart::Text(_)));
    }

    #[test]
    fn test_assistant_content_part_from_file() {
        let part: AssistantContentPart = FilePart::from_url(
            Url::parse("https://example.com/file.pdf").unwrap(),
            "application/pdf",
        )
        .into();

        assert!(matches!(part, AssistantContentPart::File(_)));
    }

    #[test]
    fn test_assistant_content_part_from_reasoning() {
        let part: AssistantContentPart = ReasoningPart::new("Let me think about this...").into();

        assert!(matches!(part, AssistantContentPart::Reasoning(_)));
    }

    #[test]
    fn test_assistant_content_part_from_tool_call() {
        let part: AssistantContentPart =
            ToolCallPart::new("call_123", "get_weather", serde_json::json!({"city": "SF"})).into();

        assert!(matches!(part, AssistantContentPart::ToolCall(_)));
    }

    #[test]
    fn test_assistant_content_part_from_tool_result() {
        let part: AssistantContentPart =
            ToolResultPart::new("call_123", "get_weather", ToolResultOutput::text("Success"))
                .into();

        assert!(matches!(part, AssistantContentPart::ToolResult(_)));
    }

    #[test]
    fn test_assistant_content_part_from_tool_approval_request() {
        let part: AssistantContentPart =
            ToolApprovalRequest::new("approval_123", "call_123").into();

        assert!(matches!(part, AssistantContentPart::ToolApprovalRequest(_)));
    }

    #[test]
    fn test_assistant_model_message_serialization_text() {
        let message = AssistantMessage::new("Hello, world!");

        let serialized = serde_json::to_value(&message).unwrap();

        assert_eq!(serialized["role"], "assistant");
        assert_eq!(serialized["content"], "Hello, world!");
        assert!(serialized.get("providerOptions").is_none());
    }

    #[test]
    fn test_assistant_model_message_serialization_parts() {
        let parts = vec![AssistantContentPart::Text(TextPart::new("Response"))];

        let message = AssistantMessage::with_parts(parts);

        let serialized = serde_json::to_value(&message).unwrap();

        assert_eq!(serialized["role"], "assistant");
        assert!(serialized["content"].is_array());
        assert_eq!(serialized["content"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_assistant_model_message_serialization_with_provider_options() {
        let provider_options = SharedProviderOptions::new();

        let message = AssistantMessage::new("Hello").with_provider_options(provider_options);

        let serialized = serde_json::to_value(&message).unwrap();

        assert_eq!(serialized["role"], "assistant");
        assert!(serialized.get("providerOptions").is_some());
    }

    #[test]
    fn test_assistant_model_message_deserialization_text() {
        let json = serde_json::json!({
            "role": "assistant",
            "content": "Hello, world!"
        });

        let message: AssistantMessage = serde_json::from_value(json).unwrap();

        assert_eq!(message.role, "assistant");
        match message.content {
            AssistantContent::Text(text) => assert_eq!(text, "Hello, world!"),
            _ => panic!("Expected Text variant"),
        }
        assert!(message.provider_options.is_none());
    }

    #[test]
    fn test_assistant_model_message_deserialization_parts() {
        let json = serde_json::json!({
            "role": "assistant",
            "content": [
                {
                    "type": "text",
                    "text": "Hello"
                }
            ]
        });

        let message: AssistantMessage = serde_json::from_value(json).unwrap();

        assert_eq!(message.role, "assistant");
        match message.content {
            AssistantContent::Parts(parts) => {
                assert_eq!(parts.len(), 1);
                assert!(matches!(parts[0], AssistantContentPart::Text(_)));
            }
            _ => panic!("Expected Parts variant"),
        }
    }

    #[test]
    fn test_assistant_model_message_clone() {
        let message = AssistantMessage::new("Hello");
        let cloned = message.clone();

        assert_eq!(message, cloned);
    }

    #[test]
    fn test_assistant_model_message_mixed_content() {
        let parts = vec![
            AssistantContentPart::Text(TextPart::new("I'll check the weather for you.")),
            AssistantContentPart::Reasoning(ReasoningPart::new("User wants current weather info")),
            AssistantContentPart::ToolCall(ToolCallPart::new(
                "call_123",
                "get_weather",
                serde_json::json!({"city": "San Francisco"}),
            )),
        ];

        let message = AssistantMessage::with_parts(parts);

        match message.content {
            AssistantContent::Parts(parts) => {
                assert_eq!(parts.len(), 3);
                assert!(matches!(parts[0], AssistantContentPart::Text(_)));
                assert!(matches!(parts[1], AssistantContentPart::Reasoning(_)));
                assert!(matches!(parts[2], AssistantContentPart::ToolCall(_)));
            }
            _ => panic!("Expected Parts variant"),
        }
    }
}
