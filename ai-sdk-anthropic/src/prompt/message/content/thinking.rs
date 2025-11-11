use serde::{Deserialize, Serialize};

/// Anthropic thinking content block.
///
/// Represents a thinking content block in an Anthropic message. These blocks contain
/// the model's reasoning process and are used with extended thinking models.
///
/// **Note:** Thinking blocks cannot be directly cached with cache_control.
/// They are cached implicitly when appearing in previous assistant turns.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::prompt::message::content::thinking::AnthropicThinkingContent;
///
/// // Create a thinking content block
/// let thinking_content = AnthropicThinkingContent::new(
///     "Let me analyze this step by step...",
///     "sig_abc123"
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicThinkingContent {
    /// The type of content (always "thinking")
    #[serde(rename = "type")]
    pub content_type: String,

    /// The thinking/reasoning content
    pub thinking: String,

    /// A signature identifying this thinking block
    pub signature: String,
}

impl AnthropicThinkingContent {
    /// Creates a new thinking content block.
    ///
    /// # Arguments
    ///
    /// * `thinking` - The thinking/reasoning content
    /// * `signature` - A signature identifying this thinking block
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::thinking::AnthropicThinkingContent;
    ///
    /// let thinking = AnthropicThinkingContent::new(
    ///     "First, I need to understand the problem...",
    ///     "sig_xyz789"
    /// );
    /// assert_eq!(thinking.thinking, "First, I need to understand the problem...");
    /// assert_eq!(thinking.signature, "sig_xyz789");
    /// ```
    pub fn new(thinking: impl Into<String>, signature: impl Into<String>) -> Self {
        Self {
            content_type: "thinking".to_string(),
            thinking: thinking.into(),
            signature: signature.into(),
        }
    }

    /// Updates the thinking content.
    ///
    /// # Arguments
    ///
    /// * `thinking` - The new thinking/reasoning content
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::thinking::AnthropicThinkingContent;
    ///
    /// let thinking = AnthropicThinkingContent::new("Initial thought", "sig_1")
    ///     .with_thinking("Updated thought process");
    /// assert_eq!(thinking.thinking, "Updated thought process");
    /// ```
    pub fn with_thinking(mut self, thinking: impl Into<String>) -> Self {
        self.thinking = thinking.into();
        self
    }

    /// Updates the signature.
    ///
    /// # Arguments
    ///
    /// * `signature` - The new signature
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::thinking::AnthropicThinkingContent;
    ///
    /// let thinking = AnthropicThinkingContent::new("Thinking...", "sig_old")
    ///     .with_signature("sig_new");
    /// assert_eq!(thinking.signature, "sig_new");
    /// ```
    pub fn with_signature(mut self, signature: impl Into<String>) -> Self {
        self.signature = signature.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new() {
        let thinking = AnthropicThinkingContent::new("Let me think about this...", "sig_123");

        assert_eq!(thinking.content_type, "thinking");
        assert_eq!(thinking.thinking, "Let me think about this...");
        assert_eq!(thinking.signature, "sig_123");
    }

    #[test]
    fn test_new_with_string() {
        let thinking =
            AnthropicThinkingContent::new("Thinking...".to_string(), "sig_456".to_string());

        assert_eq!(thinking.thinking, "Thinking...");
        assert_eq!(thinking.signature, "sig_456");
    }

    #[test]
    fn test_with_thinking() {
        let thinking =
            AnthropicThinkingContent::new("Original", "sig_1").with_thinking("Updated thinking");

        assert_eq!(thinking.thinking, "Updated thinking");
        assert_eq!(thinking.signature, "sig_1"); // Signature unchanged
    }

    #[test]
    fn test_with_signature() {
        let thinking =
            AnthropicThinkingContent::new("Thinking...", "sig_old").with_signature("sig_new");

        assert_eq!(thinking.thinking, "Thinking..."); // Thinking unchanged
        assert_eq!(thinking.signature, "sig_new");
    }

    #[test]
    fn test_builder_chaining() {
        let thinking = AnthropicThinkingContent::new("Initial", "sig_1")
            .with_thinking("Second")
            .with_signature("sig_2")
            .with_thinking("Final");

        assert_eq!(thinking.thinking, "Final");
        assert_eq!(thinking.signature, "sig_2");
    }

    #[test]
    fn test_serialize() {
        let thinking = AnthropicThinkingContent::new("Step-by-step reasoning", "sig_abc123");
        let json = serde_json::to_value(&thinking).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "thinking",
                "thinking": "Step-by-step reasoning",
                "signature": "sig_abc123"
            })
        );
    }

    #[test]
    fn test_serialize_no_cache_control() {
        // Verify that cache_control is never serialized (as per TypeScript never type)
        let thinking = AnthropicThinkingContent::new("Thinking...", "sig_1");
        let json = serde_json::to_value(&thinking).unwrap();

        // Should only have type, thinking, and signature fields
        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 3);
        assert!(obj.contains_key("type"));
        assert!(obj.contains_key("thinking"));
        assert!(obj.contains_key("signature"));
        assert!(!obj.contains_key("cache_control"));
    }

    #[test]
    fn test_deserialize() {
        let json = json!({
            "type": "thinking",
            "thinking": "Analytical reasoning here",
            "signature": "sig_xyz789"
        });

        let thinking: AnthropicThinkingContent = serde_json::from_value(json).unwrap();

        assert_eq!(thinking.content_type, "thinking");
        assert_eq!(thinking.thinking, "Analytical reasoning here");
        assert_eq!(thinking.signature, "sig_xyz789");
    }

    #[test]
    fn test_deserialize_ignores_extra_fields() {
        // Should ignore cache_control if present in JSON
        let json = json!({
            "type": "thinking",
            "thinking": "Reasoning",
            "signature": "sig_1",
            "cache_control": {
                "type": "ephemeral",
                "ttl": "5m"
            }
        });

        let thinking: AnthropicThinkingContent = serde_json::from_value(json).unwrap();

        assert_eq!(thinking.thinking, "Reasoning");
        assert_eq!(thinking.signature, "sig_1");
    }

    #[test]
    fn test_equality() {
        let thinking1 = AnthropicThinkingContent::new("Same thinking", "sig_1");
        let thinking2 = AnthropicThinkingContent::new("Same thinking", "sig_1");
        let thinking3 = AnthropicThinkingContent::new("Different", "sig_1");
        let thinking4 = AnthropicThinkingContent::new("Same thinking", "sig_2");

        assert_eq!(thinking1, thinking2);
        assert_ne!(thinking1, thinking3);
        assert_ne!(thinking1, thinking4);
    }

    #[test]
    fn test_clone() {
        let thinking1 = AnthropicThinkingContent::new("Clone me", "sig_abc");
        let thinking2 = thinking1.clone();

        assert_eq!(thinking1, thinking2);
    }

    #[test]
    fn test_empty_thinking() {
        let thinking = AnthropicThinkingContent::new("", "sig_empty");

        assert_eq!(thinking.thinking, "");
        assert_eq!(thinking.content_type, "thinking");
    }

    #[test]
    fn test_empty_signature() {
        let thinking = AnthropicThinkingContent::new("Thinking...", "");

        assert_eq!(thinking.signature, "");
    }

    #[test]
    fn test_multiline_thinking() {
        let multiline =
            "Step 1: Analyze the problem\nStep 2: Consider options\nStep 3: Choose solution";
        let thinking = AnthropicThinkingContent::new(multiline, "sig_multi");

        assert_eq!(thinking.thinking, multiline);
    }

    #[test]
    fn test_unicode_thinking() {
        let unicode_thinking = "思考过程: Let me think 🤔";
        let thinking = AnthropicThinkingContent::new(unicode_thinking, "sig_unicode");

        assert_eq!(thinking.thinking, unicode_thinking);
    }

    #[test]
    fn test_long_signature() {
        let long_sig = "sig_".to_string() + &"a".repeat(100);
        let thinking = AnthropicThinkingContent::new("Thinking", &long_sig);

        assert_eq!(thinking.signature, long_sig);
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original = AnthropicThinkingContent::new(
            "Complex reasoning with special chars: \n\t\"quotes\"",
            "sig_roundtrip",
        );

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicThinkingContent = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }
}
