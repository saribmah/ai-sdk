use llm_kit_provider::language_model::prompt::message::LanguageModelSystemMessage;
use llm_kit_provider::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// System message with text content
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAICompatibleSystemMessage {
    /// The system message content
    pub content: String,

    /// Additional properties for extensibility (flattened into the struct)
    #[serde(flatten)]
    pub additional_properties: Option<Value>,
}

impl OpenAICompatibleSystemMessage {
    /// Creates a new system message
    pub fn new(content: String) -> Self {
        Self {
            content,
            additional_properties: None,
        }
    }

    /// Creates a system message from a provider system message
    pub fn from_provider(msg: LanguageModelSystemMessage) -> Self {
        let mut system_msg = Self::new(msg.content);
        if let Some(metadata) = get_openai_metadata(&msg.provider_options) {
            system_msg.additional_properties = Some(metadata);
        }
        system_msg
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
    fn test_system_message() {
        let msg = OpenAICompatibleSystemMessage::new("You are helpful".to_string());
        assert_eq!(msg.content, "You are helpful");
    }
}
