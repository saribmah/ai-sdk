use llm_kit_provider::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};

/// Text content part of a prompt. It contains a string of text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextPart {
    /// The text content.
    pub text: String,

    /// Additional provider-specific metadata. They are passed through
    /// to the provider from the LLM Kit and enable provider-specific
    /// functionality that can be fully encapsulated in the provider.
    #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<SharedProviderOptions>,
}

impl TextPart {
    /// Creates a new text part with the given text.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            provider_options: None,
        }
    }

    /// Sets the provider options.
    pub fn with_provider_options(mut self, options: SharedProviderOptions) -> Self {
        self.provider_options = Some(options);
        self
    }
}

impl From<String> for TextPart {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

impl From<&str> for TextPart {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_part_new() {
        let part = TextPart::new("Hello, world!");
        assert_eq!(part.text, "Hello, world!");
        assert!(part.provider_options.is_none());
    }

    #[test]
    fn test_text_part_from_string() {
        let part = TextPart::from("Hello".to_string());
        assert_eq!(part.text, "Hello");
    }

    #[test]
    fn test_text_part_from_str() {
        let part = TextPart::from("Hello");
        assert_eq!(part.text, "Hello");
    }
}
