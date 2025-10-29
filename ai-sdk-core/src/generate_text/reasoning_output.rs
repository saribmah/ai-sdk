use ai_sdk_provider::shared::provider_metadata::ProviderMetadata;
use serde::{Deserialize, Serialize};

/// Reasoning output of a text generation. It contains reasoning text.
///
/// This represents the chain-of-thought or reasoning process that the model
/// went through before generating the final response.
///
/// # Example
///
/// ```
/// use ai_sdk_core::ReasoningOutput;
///
/// let reasoning = ReasoningOutput::new("First, I need to analyze the problem...");
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReasoningOutput {
    /// Type discriminator (always "reasoning").
    #[serde(rename = "type")]
    pub reasoning_type: String,

    /// The reasoning text.
    pub text: String,

    /// Additional provider-specific metadata.
    ///
    /// These are passed through to the provider from the AI SDK and enable
    /// provider-specific functionality that can be fully encapsulated in the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<ProviderMetadata>,
}

impl ReasoningOutput {
    /// Creates a new reasoning output.
    ///
    /// # Arguments
    ///
    /// * `text` - The reasoning text
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_core::ReasoningOutput;
    ///
    /// let reasoning = ReasoningOutput::new("Let me think through this step by step...");
    /// assert_eq!(reasoning.text, "Let me think through this step by step...");
    /// ```
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            reasoning_type: "reasoning".to_string(),
            text: text.into(),
            provider_metadata: None,
        }
    }

    /// Sets the provider metadata for this reasoning output.
    ///
    /// # Arguments
    ///
    /// * `metadata` - Provider-specific metadata
    ///
    /// # Example
    ///
    /// ```ignore
    /// use ai_sdk_core::ReasoningOutput;
    ///
    /// let reasoning = ReasoningOutput::new("Thinking...")
    ///     .with_provider_metadata(metadata);
    /// ```
    pub fn with_provider_metadata(mut self, metadata: ProviderMetadata) -> Self {
        self.provider_metadata = Some(metadata);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_reasoning_output_new() {
        let reasoning = ReasoningOutput::new("Step 1: Analyze the problem");

        assert_eq!(reasoning.reasoning_type, "reasoning");
        assert_eq!(reasoning.text, "Step 1: Analyze the problem");
        assert!(reasoning.provider_metadata.is_none());
    }

    #[test]
    fn test_reasoning_output_with_provider_metadata() {
        let mut metadata = ProviderMetadata::new();
        let mut inner = HashMap::new();
        inner.insert("key".to_string(), serde_json::json!("value"));
        metadata.insert("provider".to_string(), inner);

        let reasoning =
            ReasoningOutput::new("Thinking...").with_provider_metadata(metadata.clone());

        assert_eq!(reasoning.provider_metadata, Some(metadata));
    }

    #[test]
    fn test_reasoning_output_serialization() {
        let reasoning = ReasoningOutput::new("Let me reason about this");

        let serialized = serde_json::to_value(&reasoning).unwrap();

        assert_eq!(serialized["type"], "reasoning");
        assert_eq!(serialized["text"], "Let me reason about this");
        assert!(serialized.get("providerMetadata").is_none());
    }

    #[test]
    fn test_reasoning_output_deserialization() {
        let json = serde_json::json!({
            "type": "reasoning",
            "text": "Chain of thought..."
        });

        let reasoning: ReasoningOutput = serde_json::from_value(json).unwrap();

        assert_eq!(reasoning.reasoning_type, "reasoning");
        assert_eq!(reasoning.text, "Chain of thought...");
        assert!(reasoning.provider_metadata.is_none());
    }

    #[test]
    fn test_reasoning_output_with_metadata_serialization() {
        let mut metadata = ProviderMetadata::new();
        let mut inner = HashMap::new();
        inner.insert("tokens".to_string(), serde_json::json!(100));
        metadata.insert("openai".to_string(), inner);

        let reasoning = ReasoningOutput::new("Reasoning text").with_provider_metadata(metadata);

        let serialized = serde_json::to_value(&reasoning).unwrap();

        assert_eq!(serialized["type"], "reasoning");
        assert_eq!(serialized["text"], "Reasoning text");
        assert!(serialized["providerMetadata"].is_object());
        assert_eq!(serialized["providerMetadata"]["openai"]["tokens"], 100);
    }

    #[test]
    fn test_reasoning_output_clone() {
        let reasoning = ReasoningOutput::new("Test reasoning");
        let cloned = reasoning.clone();

        assert_eq!(reasoning, cloned);
    }

    #[test]
    fn test_reasoning_output_debug() {
        let reasoning = ReasoningOutput::new("Debug test");
        let debug_str = format!("{:?}", reasoning);

        assert!(debug_str.contains("ReasoningOutput"));
        assert!(debug_str.contains("Debug test"));
    }
}
