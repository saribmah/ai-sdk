use ai_sdk_provider::language_model::prompt::LanguageModelToolResultOutput;
use ai_sdk_provider::language_model::prompt::message::LanguageModelToolMessage;
use ai_sdk_provider::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Tool message with the result of a tool call
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAICompatibleToolMessage {
    /// The content of the tool response
    pub content: String,

    /// The ID of the tool call this is responding to
    pub tool_call_id: String,

    /// Additional properties for extensibility (flattened into the struct)
    #[serde(flatten)]
    pub additional_properties: Option<Value>,
}

impl OpenAICompatibleToolMessage {
    /// Creates a new tool message
    pub fn new(content: String, tool_call_id: String) -> Self {
        Self {
            content,
            tool_call_id,
            additional_properties: None,
        }
    }

    /// Creates tool messages from a provider tool message
    /// Returns a vector because a single LanguageModelToolMessage can contain multiple tool results
    pub fn from_provider(msg: LanguageModelToolMessage) -> Vec<Self> {
        let mut tool_messages = Vec::new();

        for tool_response in msg.content {
            let content_value = match &tool_response.output {
                LanguageModelToolResultOutput::Text { value } => value.clone(),
                LanguageModelToolResultOutput::ErrorText { value } => value.clone(),
                LanguageModelToolResultOutput::Json { value } => {
                    serde_json::to_string(&value).unwrap_or_else(|_| "{}".to_string())
                }
                LanguageModelToolResultOutput::ErrorJson { value } => {
                    serde_json::to_string(&value).unwrap_or_else(|_| "{}".to_string())
                }
                LanguageModelToolResultOutput::Content { value } => {
                    serde_json::to_string(&value).unwrap_or_else(|_| "[]".to_string())
                }
            };

            let mut tool_message = Self::new(content_value, tool_response.tool_call_id.clone());
            if let Some(metadata) = get_openai_metadata(&tool_response.provider_options) {
                tool_message.additional_properties = Some(metadata);
            }
            tool_messages.push(tool_message);
        }

        tool_messages
    }
}

/// Extracts OpenAI-compatible metadata from provider options
fn get_openai_metadata(provider_options: &Option<SharedProviderOptions>) -> Option<Value> {
    provider_options
        .as_ref()
        .and_then(|opts| opts.get("openaiCompatible"))
        .and_then(|metadata| serde_json::to_value(metadata).ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_message() {
        let msg = OpenAICompatibleToolMessage::new("Result".to_string(), "call_123".to_string());
        assert_eq!(msg.content, "Result");
        assert_eq!(msg.tool_call_id, "call_123");
    }
}
