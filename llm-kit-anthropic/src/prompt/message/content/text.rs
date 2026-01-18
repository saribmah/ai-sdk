use serde::{Deserialize, Serialize};

use crate::prompt::message::cache_control::AnthropicCacheControl;

/// Anthropic text content block.
///
/// Represents a text content block in an Anthropic message. This is the most common
/// content type used for regular text responses.
///
/// # Example
///
/// ```
/// use llm_kit_anthropic::prompt::message::content::text::AnthropicTextContent;
///
/// // Create a simple text content block
/// let text_content = AnthropicTextContent::new("Hello, world!");
///
/// // Create a text content block with cache control
/// use llm_kit_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
///
/// let text_with_cache = AnthropicTextContent::new("This is cached content")
///     .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicTextContent {
    /// The type of content (always "text")
    #[serde(rename = "type")]
    pub content_type: String,

    /// The text content
    pub text: String,

    /// Optional cache control configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}

impl AnthropicTextContent {
    /// Creates a new text content block.
    ///
    /// # Arguments
    ///
    /// * `text` - The text content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text::AnthropicTextContent;
    ///
    /// let text_content = AnthropicTextContent::new("Hello, world!");
    /// assert_eq!(text_content.text, "Hello, world!");
    /// ```
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            content_type: "text".to_string(),
            text: text.into(),
            cache_control: None,
        }
    }

    /// Sets the cache control for this text content block.
    ///
    /// # Arguments
    ///
    /// * `cache_control` - The cache control configuration
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text::AnthropicTextContent;
    /// use llm_kit_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    ///
    /// let text_content = AnthropicTextContent::new("Cached content")
    ///     .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes));
    /// ```
    pub fn with_cache_control(mut self, cache_control: AnthropicCacheControl) -> Self {
        self.cache_control = Some(cache_control);
        self
    }

    /// Sets the cache control to None, removing any existing cache configuration.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text::AnthropicTextContent;
    /// use llm_kit_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    ///
    /// let text_content = AnthropicTextContent::new("Content")
    ///     .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour))
    ///     .without_cache_control();
    ///
    /// assert!(text_content.cache_control.is_none());
    /// ```
    pub fn without_cache_control(mut self) -> Self {
        self.cache_control = None;
        self
    }
}

impl From<String> for AnthropicTextContent {
    /// Converts a String into an AnthropicTextContent.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text::AnthropicTextContent;
    ///
    /// let text_content: AnthropicTextContent = "Hello".to_string().into();
    /// assert_eq!(text_content.text, "Hello");
    /// ```
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

impl From<&str> for AnthropicTextContent {
    /// Converts a string slice into an AnthropicTextContent.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::text::AnthropicTextContent;
    ///
    /// let text_content: AnthropicTextContent = "Hello".into();
    /// assert_eq!(text_content.text, "Hello");
    /// ```
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    use serde_json::json;

    #[test]
    fn test_new() {
        let text_content = AnthropicTextContent::new("Hello, world!");

        assert_eq!(text_content.content_type, "text");
        assert_eq!(text_content.text, "Hello, world!");
        assert!(text_content.cache_control.is_none());
    }

    #[test]
    fn test_new_with_string() {
        let text_content = AnthropicTextContent::new("Test".to_string());

        assert_eq!(text_content.text, "Test");
    }

    #[test]
    fn test_with_cache_control() {
        let cache_control = AnthropicCacheControl::new(CacheTTL::FiveMinutes);
        let text_content =
            AnthropicTextContent::new("Cached text").with_cache_control(cache_control.clone());

        assert_eq!(text_content.cache_control, Some(cache_control));
    }

    #[test]
    fn test_without_cache_control() {
        let text_content = AnthropicTextContent::new("Text")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour))
            .without_cache_control();

        assert!(text_content.cache_control.is_none());
    }

    #[test]
    fn test_from_string() {
        let text_content: AnthropicTextContent = "Hello".to_string().into();

        assert_eq!(text_content.content_type, "text");
        assert_eq!(text_content.text, "Hello");
        assert!(text_content.cache_control.is_none());
    }

    #[test]
    fn test_from_str() {
        let text_content: AnthropicTextContent = "Hello".into();

        assert_eq!(text_content.content_type, "text");
        assert_eq!(text_content.text, "Hello");
        assert!(text_content.cache_control.is_none());
    }

    #[test]
    fn test_serialize_without_cache() {
        let text_content = AnthropicTextContent::new("Hello");
        let json = serde_json::to_value(&text_content).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "text",
                "text": "Hello"
            })
        );
    }

    #[test]
    fn test_serialize_with_cache() {
        let text_content = AnthropicTextContent::new("Cached")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes));
        let json = serde_json::to_value(&text_content).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "text",
                "text": "Cached",
                "cache_control": {
                    "type": "ephemeral",
                    "ttl": "5m"
                }
            })
        );
    }

    #[test]
    fn test_serialize_with_one_hour_cache() {
        let text_content = AnthropicTextContent::new("Cached for an hour")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let json = serde_json::to_value(&text_content).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "text",
                "text": "Cached for an hour",
                "cache_control": {
                    "type": "ephemeral",
                    "ttl": "1h"
                }
            })
        );
    }

    #[test]
    fn test_deserialize_without_cache() {
        let json = json!({
            "type": "text",
            "text": "Hello"
        });

        let text_content: AnthropicTextContent = serde_json::from_value(json).unwrap();

        assert_eq!(text_content.content_type, "text");
        assert_eq!(text_content.text, "Hello");
        assert!(text_content.cache_control.is_none());
    }

    #[test]
    fn test_deserialize_with_cache() {
        let json = json!({
            "type": "text",
            "text": "Cached",
            "cache_control": {
                "type": "ephemeral",
                "ttl": "5m"
            }
        });

        let text_content: AnthropicTextContent = serde_json::from_value(json).unwrap();

        assert_eq!(text_content.content_type, "text");
        assert_eq!(text_content.text, "Cached");
        assert_eq!(
            text_content.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
        );
    }

    #[test]
    fn test_deserialize_with_one_hour_cache() {
        let json = json!({
            "type": "text",
            "text": "Long cached",
            "cache_control": {
                "type": "ephemeral",
                "ttl": "1h"
            }
        });

        let text_content: AnthropicTextContent = serde_json::from_value(json).unwrap();

        assert_eq!(text_content.text, "Long cached");
        assert_eq!(
            text_content.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::OneHour))
        );
    }

    #[test]
    fn test_equality() {
        let text1 = AnthropicTextContent::new("Same");
        let text2 = AnthropicTextContent::new("Same");
        let text3 = AnthropicTextContent::new("Different");

        assert_eq!(text1, text2);
        assert_ne!(text1, text3);
    }

    #[test]
    fn test_equality_with_cache() {
        let text1 = AnthropicTextContent::new("Text")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes));
        let text2 = AnthropicTextContent::new("Text")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes));
        let text3 = AnthropicTextContent::new("Text")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        assert_eq!(text1, text2);
        assert_ne!(text1, text3);
    }

    #[test]
    fn test_clone() {
        let text1 = AnthropicTextContent::new("Clone me")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let text2 = text1.clone();

        assert_eq!(text1, text2);
    }

    #[test]
    fn test_builder_pattern_chaining() {
        let text_content = AnthropicTextContent::new("Test")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
            .without_cache_control()
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        assert_eq!(
            text_content.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::OneHour))
        );
    }

    #[test]
    fn test_empty_text() {
        let text_content = AnthropicTextContent::new("");

        assert_eq!(text_content.text, "");
        assert_eq!(text_content.content_type, "text");
    }

    #[test]
    fn test_multiline_text() {
        let multiline = "Line 1\nLine 2\nLine 3";
        let text_content = AnthropicTextContent::new(multiline);

        assert_eq!(text_content.text, multiline);
    }

    #[test]
    fn test_unicode_text() {
        let unicode_text = "Hello 👋 World 🌍";
        let text_content = AnthropicTextContent::new(unicode_text);

        assert_eq!(text_content.text, unicode_text);
    }
}
