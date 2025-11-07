use ai_sdk_provider::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};

/// A system message. It can contain system information.
///
/// Note: using the "system" part of the prompt is strongly preferred
/// to increase the resilience against prompt injection attacks,
/// and because not all providers support several system messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemMessage {
    /// Role is always 'system' for system messages.
    pub role: String,

    /// Content of the system message.
    pub content: String,

    /// Additional provider-specific metadata. They are passed through
    /// to the provider from the AI SDK and enable provider-specific
    /// functionality that can be fully encapsulated in the provider.
    #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<SharedProviderOptions>,
}

impl SystemMessage {
    /// Creates a new system model message with the given content.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
            provider_options: None,
        }
    }

    /// Sets the provider options for this message.
    pub fn with_provider_options(mut self, options: SharedProviderOptions) -> Self {
        self.provider_options = Some(options);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_model_message_new() {
        let message = SystemMessage::new("You are a helpful assistant.");

        assert_eq!(message.role, "system");
        assert_eq!(message.content, "You are a helpful assistant.");
        assert!(message.provider_options.is_none());
    }

    #[test]
    fn test_system_model_message_with_provider_options() {
        let provider_options = SharedProviderOptions::new();

        let message = SystemMessage::new("You are a helpful assistant.")
            .with_provider_options(provider_options.clone());

        assert_eq!(message.provider_options, Some(provider_options));
    }

    #[test]
    fn test_system_model_message_from_string() {
        let content = "System instructions".to_string();
        let message = SystemMessage::new(content.clone());

        assert_eq!(message.content, content);
    }

    #[test]
    fn test_system_model_message_from_str() {
        let message = SystemMessage::new("Instructions");

        assert_eq!(message.content, "Instructions");
    }

    #[test]
    fn test_system_model_message_serialization() {
        let message = SystemMessage::new("You are helpful.");

        let serialized = serde_json::to_value(&message).unwrap();

        assert_eq!(serialized["role"], "system");
        assert_eq!(serialized["content"], "You are helpful.");
        assert!(serialized.get("providerOptions").is_none());
    }

    #[test]
    fn test_system_model_message_serialization_with_provider_options() {
        let provider_options = SharedProviderOptions::new();

        let message =
            SystemMessage::new("You are helpful.").with_provider_options(provider_options);

        let serialized = serde_json::to_value(&message).unwrap();

        assert_eq!(serialized["role"], "system");
        assert!(serialized.get("providerOptions").is_some());
    }

    #[test]
    fn test_system_model_message_deserialization() {
        let json = serde_json::json!({
            "role": "system",
            "content": "You are a helpful assistant."
        });

        let message: SystemMessage = serde_json::from_value(json).unwrap();

        assert_eq!(message.role, "system");
        assert_eq!(message.content, "You are a helpful assistant.");
        assert!(message.provider_options.is_none());
    }

    #[test]
    fn test_system_model_message_deserialization_with_provider_options() {
        let json = serde_json::json!({
            "role": "system",
            "content": "You are a helpful assistant.",
            "providerOptions": {}
        });

        let message: SystemMessage = serde_json::from_value(json).unwrap();

        assert_eq!(message.role, "system");
        assert_eq!(message.content, "You are a helpful assistant.");
        assert!(message.provider_options.is_some());
    }

    #[test]
    fn test_system_model_message_clone() {
        let message = SystemMessage::new("You are helpful.");
        let cloned = message.clone();

        assert_eq!(message, cloned);
    }
}
