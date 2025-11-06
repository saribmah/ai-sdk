use crate::ToolApprovalResponse;
use crate::prompt::message::ToolResultPart;
use ai_sdk_provider::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};

/// Content of a tool message. It is an array of tool result parts.
pub type ToolContent = Vec<ToolContentPart>;

/// A part of tool content, either a tool result or a tool approval response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolContentPart {
    ToolResult(ToolResultPart),
    ApprovalResponse(ToolApprovalResponse),
}

/// A tool message. It contains the result of one or more tool calls.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolMessage {
    /// Role is always 'tool' for tool messages.
    pub role: String,

    /// Content of the tool message - array of tool result parts or approval responses.
    pub content: ToolContent,

    /// Optional provider-specific options.
    #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<SharedProviderOptions>,
}

impl ToolMessage {
    /// Creates a new tool model message with the given content.
    pub fn new(content: ToolContent) -> Self {
        Self {
            role: "tool".to_string(),
            content,
            provider_options: None,
        }
    }

    /// Sets the provider options for this message.
    pub fn with_provider_options(mut self, options: SharedProviderOptions) -> Self {
        self.provider_options = Some(options);
        self
    }

    /// Adds a tool result part to the content.
    pub fn add_result_part(mut self, part: ToolResultPart) -> Self {
        self.content.push(ToolContentPart::ToolResult(part));
        self
    }

    /// Adds a tool approval response to the content.
    pub fn add_approval_response(mut self, response: ToolApprovalResponse) -> Self {
        self.content
            .push(ToolContentPart::ApprovalResponse(response));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::content_parts::ToolResultOutput;
    use serde_json::json;

    #[test]
    fn test_tool_model_message_new() {
        let content = vec![ToolContentPart::ToolResult(ToolResultPart::new(
            "call_123",
            "tool_name",
            ToolResultOutput::text("Success"),
        ))];

        let message = ToolMessage::new(content.clone());

        assert_eq!(message.role, "tool");
        assert_eq!(message.content.len(), 1);
        assert!(message.provider_options.is_none());
    }

    #[test]
    fn test_tool_model_message_with_provider_options() {
        let content = vec![ToolContentPart::ToolResult(ToolResultPart::new(
            "call_123",
            "tool_name",
            ToolResultOutput::text("Success"),
        ))];

        let provider_options = SharedProviderOptions::new();

        let message = ToolMessage::new(content).with_provider_options(provider_options.clone());

        assert_eq!(message.provider_options, Some(provider_options));
    }

    #[test]
    fn test_tool_model_message_add_result_part() {
        let message = ToolMessage::new(vec![]).add_result_part(ToolResultPart::new(
            "call_123",
            "tool_name",
            ToolResultOutput::text("Success"),
        ));

        assert_eq!(message.content.len(), 1);
        match &message.content[0] {
            ToolContentPart::ToolResult(part) => {
                assert_eq!(part.tool_call_id, "call_123");
            }
            _ => panic!("Expected ToolResult variant"),
        }
    }

    #[test]
    fn test_tool_model_message_add_approval_response() {
        let response = ToolApprovalResponse::granted("approval_123");
        let message = ToolMessage::new(vec![]).add_approval_response(response.clone());

        assert_eq!(message.content.len(), 1);
        match &message.content[0] {
            ToolContentPart::ApprovalResponse(resp) => {
                assert_eq!(resp.approval_id, "approval_123");
                assert_eq!(resp.approved, true);
            }
            _ => panic!("Expected ApprovalResponse variant"),
        }
    }

    #[test]
    fn test_tool_model_message_mixed_content() {
        let message = ToolMessage::new(vec![])
            .add_result_part(ToolResultPart::new(
                "call_123",
                "tool_name",
                ToolResultOutput::text("Success"),
            ))
            .add_approval_response(ToolApprovalResponse::granted("approval_456"));

        assert_eq!(message.content.len(), 2);
        assert!(matches!(message.content[0], ToolContentPart::ToolResult(_)));
        assert!(matches!(
            message.content[1],
            ToolContentPart::ApprovalResponse(_)
        ));
    }

    #[test]
    fn test_tool_model_message_serialization() {
        let content = vec![ToolContentPart::ToolResult(ToolResultPart::new(
            "call_123",
            "tool_name",
            ToolResultOutput::text("Success"),
        ))];

        let message = ToolMessage::new(content);

        let serialized = serde_json::to_value(&message).unwrap();

        assert_eq!(serialized["role"], "tool");
        assert!(serialized["content"].is_array());
        assert!(serialized.get("providerOptions").is_none());
    }

    #[test]
    fn test_tool_model_message_serialization_with_provider_options() {
        let content = vec![ToolContentPart::ToolResult(ToolResultPart::new(
            "call_123",
            "tool_name",
            ToolResultOutput::text("Success"),
        ))];

        let provider_options = SharedProviderOptions::new();

        let message = ToolMessage::new(content).with_provider_options(provider_options);

        let serialized = serde_json::to_value(&message).unwrap();

        assert_eq!(serialized["role"], "tool");
        assert!(serialized.get("providerOptions").is_some());
    }

    #[test]
    fn test_tool_model_message_deserialization() {
        let json = json!({
            "role": "tool",
            "content": [
                {
                    "type": "text",
                    "toolCallId": "call_123",
                    "toolName": "tool_name",
                    "output": {
                        "type": "text",
                        "value": "Success"
                    }
                }
            ]
        });

        let message: ToolMessage = serde_json::from_value(json).unwrap();

        assert_eq!(message.role, "tool");
        assert_eq!(message.content.len(), 1);
        assert!(message.provider_options.is_none());
    }

    #[test]
    fn test_tool_model_message_clone() {
        let content = vec![ToolContentPart::ToolResult(ToolResultPart::new(
            "call_123",
            "tool_name",
            ToolResultOutput::text("Success"),
        ))];

        let message = ToolMessage::new(content);
        let cloned = message.clone();

        assert_eq!(message, cloned);
    }
}
