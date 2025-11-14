use serde::{Deserialize, Serialize};

/// Anthropic redacted thinking content block.
///
/// Represents a redacted thinking content block in an Anthropic message. These blocks
/// contain redacted versions of the model's reasoning process, where sensitive or
/// unnecessary thinking has been removed or obscured.
///
/// **Note:** Redacted thinking blocks cannot be directly cached with cache_control.
/// They are cached implicitly when appearing in previous assistant turns.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::prompt::message::content::redacted_thinking::AnthropicRedactedThinkingContent;
///
/// // Create a redacted thinking content block
/// let redacted = AnthropicRedactedThinkingContent::new("[REDACTED]");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicRedactedThinkingContent {
    /// The type of content (always "redacted_thinking")
    #[serde(rename = "type")]
    pub content_type: String,

    /// The redacted data/content
    pub data: String,
}

impl AnthropicRedactedThinkingContent {
    /// Creates a new redacted thinking content block.
    ///
    /// # Arguments
    ///
    /// * `data` - The redacted data/content
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::redacted_thinking::AnthropicRedactedThinkingContent;
    ///
    /// let redacted = AnthropicRedactedThinkingContent::new("[REDACTED]");
    /// assert_eq!(redacted.data, "[REDACTED]");
    /// ```
    pub fn new(data: impl Into<String>) -> Self {
        Self {
            content_type: "redacted_thinking".to_string(),
            data: data.into(),
        }
    }

    /// Updates the redacted data.
    ///
    /// # Arguments
    ///
    /// * `data` - The new redacted data/content
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::redacted_thinking::AnthropicRedactedThinkingContent;
    ///
    /// let redacted = AnthropicRedactedThinkingContent::new("[INITIAL]")
    ///     .with_data("[UPDATED]");
    /// assert_eq!(redacted.data, "[UPDATED]");
    /// ```
    pub fn with_data(mut self, data: impl Into<String>) -> Self {
        self.data = data.into();
        self
    }
}

impl From<String> for AnthropicRedactedThinkingContent {
    /// Converts a String into an AnthropicRedactedThinkingContent.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::redacted_thinking::AnthropicRedactedThinkingContent;
    ///
    /// let redacted: AnthropicRedactedThinkingContent = "[REDACTED]".to_string().into();
    /// assert_eq!(redacted.data, "[REDACTED]");
    /// ```
    fn from(data: String) -> Self {
        Self::new(data)
    }
}

impl From<&str> for AnthropicRedactedThinkingContent {
    /// Converts a string slice into an AnthropicRedactedThinkingContent.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::redacted_thinking::AnthropicRedactedThinkingContent;
    ///
    /// let redacted: AnthropicRedactedThinkingContent = "[REDACTED]".into();
    /// assert_eq!(redacted.data, "[REDACTED]");
    /// ```
    fn from(data: &str) -> Self {
        Self::new(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new() {
        let redacted = AnthropicRedactedThinkingContent::new("[REDACTED]");

        assert_eq!(redacted.content_type, "redacted_thinking");
        assert_eq!(redacted.data, "[REDACTED]");
    }

    #[test]
    fn test_new_with_string() {
        let redacted = AnthropicRedactedThinkingContent::new("[REDACTED]".to_string());

        assert_eq!(redacted.data, "[REDACTED]");
    }

    #[test]
    fn test_with_data() {
        let redacted = AnthropicRedactedThinkingContent::new("[INITIAL]").with_data("[UPDATED]");

        assert_eq!(redacted.data, "[UPDATED]");
    }

    #[test]
    fn test_builder_chaining() {
        let redacted = AnthropicRedactedThinkingContent::new("[FIRST]")
            .with_data("[SECOND]")
            .with_data("[FINAL]");

        assert_eq!(redacted.data, "[FINAL]");
    }

    #[test]
    fn test_from_string() {
        let redacted: AnthropicRedactedThinkingContent = "[REDACTED]".to_string().into();

        assert_eq!(redacted.content_type, "redacted_thinking");
        assert_eq!(redacted.data, "[REDACTED]");
    }

    #[test]
    fn test_from_str() {
        let redacted: AnthropicRedactedThinkingContent = "[REDACTED]".into();

        assert_eq!(redacted.content_type, "redacted_thinking");
        assert_eq!(redacted.data, "[REDACTED]");
    }

    #[test]
    fn test_serialize() {
        let redacted = AnthropicRedactedThinkingContent::new("[REDACTED]");
        let json = serde_json::to_value(&redacted).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "redacted_thinking",
                "data": "[REDACTED]"
            })
        );
    }

    #[test]
    fn test_serialize_no_cache_control() {
        let redacted = AnthropicRedactedThinkingContent::new("[REDACTED]");
        let json = serde_json::to_value(&redacted).unwrap();

        // Should only have type and data fields
        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 2);
        assert!(obj.contains_key("type"));
        assert!(obj.contains_key("data"));
        assert!(!obj.contains_key("cache_control"));
    }

    #[test]
    fn test_deserialize() {
        let json = json!({
            "type": "redacted_thinking",
            "data": "[SENSITIVE CONTENT REMOVED]"
        });

        let redacted: AnthropicRedactedThinkingContent = serde_json::from_value(json).unwrap();

        assert_eq!(redacted.content_type, "redacted_thinking");
        assert_eq!(redacted.data, "[SENSITIVE CONTENT REMOVED]");
    }

    #[test]
    fn test_deserialize_ignores_extra_fields() {
        // Should ignore cache_control if present in JSON
        let json = json!({
            "type": "redacted_thinking",
            "data": "[REDACTED]",
            "cache_control": {
                "type": "ephemeral",
                "ttl": "5m"
            }
        });

        let redacted: AnthropicRedactedThinkingContent = serde_json::from_value(json).unwrap();

        assert_eq!(redacted.data, "[REDACTED]");
    }

    #[test]
    fn test_equality() {
        let redacted1 = AnthropicRedactedThinkingContent::new("[SAME]");
        let redacted2 = AnthropicRedactedThinkingContent::new("[SAME]");
        let redacted3 = AnthropicRedactedThinkingContent::new("[DIFFERENT]");

        assert_eq!(redacted1, redacted2);
        assert_ne!(redacted1, redacted3);
    }

    #[test]
    fn test_clone() {
        let redacted1 = AnthropicRedactedThinkingContent::new("[CLONE ME]");
        let redacted2 = redacted1.clone();

        assert_eq!(redacted1, redacted2);
    }

    #[test]
    fn test_empty_data() {
        let redacted = AnthropicRedactedThinkingContent::new("");

        assert_eq!(redacted.data, "");
        assert_eq!(redacted.content_type, "redacted_thinking");
    }

    #[test]
    fn test_multiline_data() {
        let multiline = "[REDACTED LINE 1]\n[REDACTED LINE 2]\n[REDACTED LINE 3]";
        let redacted = AnthropicRedactedThinkingContent::new(multiline);

        assert_eq!(redacted.data, multiline);
    }

    #[test]
    fn test_unicode_data() {
        let unicode_data = "[已编辑] Redacted 🔒";
        let redacted = AnthropicRedactedThinkingContent::new(unicode_data);

        assert_eq!(redacted.data, unicode_data);
    }

    #[test]
    fn test_special_characters() {
        let special = r#"[REDACTED: "quotes", \backslashes\, and\nnewlines]"#;
        let redacted = AnthropicRedactedThinkingContent::new(special);

        assert_eq!(redacted.data, special);
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original = AnthropicRedactedThinkingContent::new(
            "[REDACTED]\nSensitive content removed\n\t\"with quotes\"",
        );

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicRedactedThinkingContent = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_long_data() {
        let long_data = "[REDACTED]".repeat(1000);
        let redacted = AnthropicRedactedThinkingContent::new(&long_data);

        assert_eq!(redacted.data, long_data);
    }

    #[test]
    fn test_json_structure() {
        let redacted = AnthropicRedactedThinkingContent::new("[CONFIDENTIAL]");
        let json_str = serde_json::to_string(&redacted).unwrap();

        // Verify JSON structure
        assert!(json_str.contains(r#""type":"redacted_thinking""#));
        assert!(json_str.contains(r#""data":"[CONFIDENTIAL]""#));
        assert!(!json_str.contains("cache_control"));
    }
}
