use ai_sdk_provider::shared::provider_options::ProviderOptions;
use serde::{Deserialize, Serialize};

/// Reasoning content part of a prompt. It contains reasoning text.
///
/// This is typically used for chain-of-thought or extended thinking from AI models.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReasoningPart {
    /// The reasoning text.
    pub text: String,

    /// Additional provider-specific metadata.
    #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<ProviderOptions>,
}

impl ReasoningPart {
    /// Creates a new reasoning part with the given text.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            provider_options: None,
        }
    }

    /// Sets the provider options.
    pub fn with_provider_options(mut self, options: ProviderOptions) -> Self {
        self.provider_options = Some(options);
        self
    }
}

impl From<String> for ReasoningPart {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

impl From<&str> for ReasoningPart {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reasoning_part_new() {
        let part = ReasoningPart::new("Let me think about this...");
        assert_eq!(part.text, "Let me think about this...");
        assert!(part.provider_options.is_none());
    }

    #[test]
    fn test_reasoning_part_from_string() {
        let part = ReasoningPart::from("Reasoning".to_string());
        assert_eq!(part.text, "Reasoning");
    }

    #[test]
    fn test_reasoning_part_from_str() {
        let part = ReasoningPart::from("Reasoning");
        assert_eq!(part.text, "Reasoning");
    }
}
