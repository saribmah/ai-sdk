use ai_sdk_provider::language_model::prompt::LanguageModelAssistantMessagePart;
use ai_sdk_provider::language_model::prompt::message::LanguageModelAssistantMessage;
use ai_sdk_provider::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Assistant message with optional content and tool calls
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAICompatibleAssistantMessage {
    /// The assistant message content (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Tool calls made by the assistant (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<OpenAICompatibleMessageToolCall>>,

    /// Additional properties for extensibility (flattened into the struct)
    #[serde(flatten)]
    pub additional_properties: Option<Value>,
}

/// Tool call in an assistant message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAICompatibleMessageToolCall {
    /// The type of tool call (always "function" for OpenAI)
    #[serde(rename = "type")]
    pub tool_type: String,

    /// The ID of the tool call
    pub id: String,

    /// Function call details
    pub function: FunctionCall,

    /// Additional properties for extensibility (flattened into the struct)
    #[serde(flatten)]
    pub additional_properties: Option<Value>,
}

/// Function call details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionCall {
    /// The name of the function being called
    pub name: String,

    /// The arguments for the function (as a JSON string)
    pub arguments: String,
}

impl OpenAICompatibleAssistantMessage {
    /// Creates a new assistant message with text content
    pub fn new_text(content: String) -> Self {
        Self {
            content: Some(content),
            tool_calls: None,
            additional_properties: None,
        }
    }

    /// Creates a new assistant message with tool calls
    pub fn new_tool_calls(tool_calls: Vec<OpenAICompatibleMessageToolCall>) -> Self {
        Self {
            content: None,
            tool_calls: Some(tool_calls),
            additional_properties: None,
        }
    }

    /// Creates a new assistant message with both content and tool calls
    pub fn new(
        content: Option<String>,
        tool_calls: Option<Vec<OpenAICompatibleMessageToolCall>>,
    ) -> Self {
        Self {
            content,
            tool_calls,
            additional_properties: None,
        }
    }

    /// Creates an assistant message from a provider assistant message
    pub fn from_provider(msg: LanguageModelAssistantMessage) -> Self {
        let content = msg.content;
        let provider_options = msg.provider_options;
        let mut text = String::new();
        let mut tool_calls: Vec<OpenAICompatibleMessageToolCall> = Vec::new();

        for part in content {
            match part {
                LanguageModelAssistantMessagePart::Text(text_part) => {
                    text.push_str(&text_part.text);
                }
                LanguageModelAssistantMessagePart::ToolCall(tool_call_part) => {
                    let arguments = serde_json::to_string(&tool_call_part.input)
                        .unwrap_or_else(|_| "{}".to_string());
                    let mut tool_call = OpenAICompatibleMessageToolCall::new(
                        tool_call_part.tool_call_id,
                        tool_call_part.tool_name,
                        arguments,
                    );
                    if let Some(metadata) = get_openai_metadata(&tool_call_part.provider_options) {
                        tool_call.additional_properties = Some(metadata);
                    }
                    tool_calls.push(tool_call);
                }
                // Ignore other assistant content types (File, Reasoning, ToolResult)
                _ => {}
            }
        }

        let content_opt = if text.is_empty() { None } else { Some(text) };

        let tool_calls_opt = if tool_calls.is_empty() {
            None
        } else {
            Some(tool_calls)
        };

        let mut assistant_msg = Self::new(content_opt, tool_calls_opt);
        if let Some(metadata) = get_openai_metadata(&provider_options) {
            assistant_msg.additional_properties = Some(metadata);
        }
        assistant_msg
    }
}

impl OpenAICompatibleMessageToolCall {
    /// Creates a new function tool call
    pub fn new(id: String, name: String, arguments: String) -> Self {
        Self {
            tool_type: "function".to_string(),
            id,
            function: FunctionCall { name, arguments },
            additional_properties: None,
        }
    }
}

/// Extracts OpenAI-compatible metadata from provider options
fn get_openai_metadata(provider_options: &Option<SharedProviderOptions>) -> Option<Value> {
    provider_options
        .as_ref()
        .and_then(|opts| opts.get("openaiCompatible"))
        .map(|metadata| serde_json::to_value(metadata).ok())
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assistant_message_text() {
        let msg = OpenAICompatibleAssistantMessage::new_text("Response".to_string());
        assert_eq!(msg.content, Some("Response".to_string()));
        assert!(msg.tool_calls.is_none());
    }

    #[test]
    fn test_assistant_message_tool_calls() {
        let tool_call = OpenAICompatibleMessageToolCall::new(
            "call_123".to_string(),
            "get_weather".to_string(),
            r#"{"city":"SF"}"#.to_string(),
        );
        let msg = OpenAICompatibleAssistantMessage::new_tool_calls(vec![tool_call]);
        assert!(msg.content.is_none());
        assert_eq!(msg.tool_calls.as_ref().unwrap().len(), 1);
    }
}
