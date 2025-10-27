use serde::{Deserialize, Serialize};

/// A unique identifier for an OpenAI-compatible chat model
pub type OpenAICompatibleChatModelId = String;

/// Provider-specific options for OpenAI-compatible chat models
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenAICompatibleProviderOptions {
    /// A unique identifier representing your end-user, which can help the provider to
    /// monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// Reasoning effort for reasoning models. Defaults to `medium`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,

    /// Controls the verbosity of the generated text. Defaults to `medium`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_verbosity: Option<String>,
}

impl OpenAICompatibleProviderOptions {
    /// Creates a new empty `OpenAICompatibleProviderOptions`
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the user identifier
    pub fn with_user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }

    /// Sets the reasoning effort
    pub fn with_reasoning_effort(mut self, reasoning_effort: String) -> Self {
        self.reasoning_effort = Some(reasoning_effort);
        self
    }

    /// Sets the text verbosity
    pub fn with_text_verbosity(mut self, text_verbosity: String) -> Self {
        self.text_verbosity = Some(text_verbosity);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let options = OpenAICompatibleProviderOptions::new();
        assert_eq!(options.user, None);
        assert_eq!(options.reasoning_effort, None);
        assert_eq!(options.text_verbosity, None);
    }

    #[test]
    fn test_default() {
        let options = OpenAICompatibleProviderOptions::default();
        assert_eq!(options.user, None);
        assert_eq!(options.reasoning_effort, None);
        assert_eq!(options.text_verbosity, None);
    }

    #[test]
    fn test_with_user() {
        let options = OpenAICompatibleProviderOptions::new()
            .with_user("user-123".to_string());

        assert_eq!(options.user, Some("user-123".to_string()));
        assert_eq!(options.reasoning_effort, None);
        assert_eq!(options.text_verbosity, None);
    }

    #[test]
    fn test_with_reasoning_effort() {
        let options = OpenAICompatibleProviderOptions::new()
            .with_reasoning_effort("high".to_string());

        assert_eq!(options.user, None);
        assert_eq!(options.reasoning_effort, Some("high".to_string()));
        assert_eq!(options.text_verbosity, None);
    }

    #[test]
    fn test_with_text_verbosity() {
        let options = OpenAICompatibleProviderOptions::new()
            .with_text_verbosity("low".to_string());

        assert_eq!(options.user, None);
        assert_eq!(options.reasoning_effort, None);
        assert_eq!(options.text_verbosity, Some("low".to_string()));
    }

    #[test]
    fn test_builder_chaining() {
        let options = OpenAICompatibleProviderOptions::new()
            .with_user("user-456".to_string())
            .with_reasoning_effort("medium".to_string())
            .with_text_verbosity("high".to_string());

        assert_eq!(options.user, Some("user-456".to_string()));
        assert_eq!(options.reasoning_effort, Some("medium".to_string()));
        assert_eq!(options.text_verbosity, Some("high".to_string()));
    }

    #[test]
    fn test_serialization() {
        let options = OpenAICompatibleProviderOptions::new()
            .with_user("user-789".to_string())
            .with_reasoning_effort("low".to_string());

        let json = serde_json::to_string(&options).unwrap();

        // Check that camelCase is used in JSON
        assert!(json.contains("\"user\""));
        assert!(json.contains("\"reasoningEffort\""));
        assert!(!json.contains("\"textVerbosity\"")); // Should be skipped
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{
            "user": "user-abc",
            "reasoningEffort": "high",
            "textVerbosity": "medium"
        }"#;

        let options: OpenAICompatibleProviderOptions = serde_json::from_str(json).unwrap();

        assert_eq!(options.user, Some("user-abc".to_string()));
        assert_eq!(options.reasoning_effort, Some("high".to_string()));
        assert_eq!(options.text_verbosity, Some("medium".to_string()));
    }

    #[test]
    fn test_deserialization_empty() {
        let json = "{}";

        let options: OpenAICompatibleProviderOptions = serde_json::from_str(json).unwrap();

        assert_eq!(options.user, None);
        assert_eq!(options.reasoning_effort, None);
        assert_eq!(options.text_verbosity, None);
    }

    #[test]
    fn test_clone() {
        let options1 = OpenAICompatibleProviderOptions::new()
            .with_user("user-clone".to_string());

        let options2 = options1.clone();

        assert_eq!(options1, options2);
    }
}
