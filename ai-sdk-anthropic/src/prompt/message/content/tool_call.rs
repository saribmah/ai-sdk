use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::prompt::message::cache_control::AnthropicCacheControl;

/// Anthropic tool call content block.
///
/// Represents a tool use (tool call) content block in an Anthropic message.
/// This is used when the model decides to call a tool/function.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::prompt::message::content::tool_call::AnthropicToolCallContent;
/// use serde_json::json;
///
/// // Create a tool call
/// let tool_call = AnthropicToolCallContent::new(
///     "toolu_123",
///     "get_weather",
///     json!({"city": "San Francisco"})
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicToolCallContent {
    /// The type of content (always "tool_use")
    #[serde(rename = "type")]
    pub content_type: String,

    /// Unique identifier for this tool call
    pub id: String,

    /// Name of the tool to call
    pub name: String,

    /// Input/arguments for the tool call
    pub input: Value,

    /// Optional cache control configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}

impl AnthropicToolCallContent {
    /// Creates a new tool call content block.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this tool call
    /// * `name` - Name of the tool to call
    /// * `input` - Input/arguments for the tool call
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::tool_call::AnthropicToolCallContent;
    /// use serde_json::json;
    ///
    /// let tool_call = AnthropicToolCallContent::new(
    ///     "toolu_abc123",
    ///     "calculate",
    ///     json!({"operation": "add", "a": 5, "b": 3})
    /// );
    /// ```
    pub fn new(id: impl Into<String>, name: impl Into<String>, input: Value) -> Self {
        Self {
            content_type: "tool_use".to_string(),
            id: id.into(),
            name: name.into(),
            input,
            cache_control: None,
        }
    }

    /// Updates the tool call ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The new tool call ID
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::tool_call::AnthropicToolCallContent;
    /// use serde_json::json;
    ///
    /// let tool_call = AnthropicToolCallContent::new("old_id", "tool", json!({}))
    ///     .with_id("new_id");
    /// assert_eq!(tool_call.id, "new_id");
    /// ```
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Updates the tool name.
    ///
    /// # Arguments
    ///
    /// * `name` - The new tool name
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::tool_call::AnthropicToolCallContent;
    /// use serde_json::json;
    ///
    /// let tool_call = AnthropicToolCallContent::new("id", "old_tool", json!({}))
    ///     .with_name("new_tool");
    /// assert_eq!(tool_call.name, "new_tool");
    /// ```
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Updates the tool input.
    ///
    /// # Arguments
    ///
    /// * `input` - The new tool input
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::tool_call::AnthropicToolCallContent;
    /// use serde_json::json;
    ///
    /// let tool_call = AnthropicToolCallContent::new("id", "tool", json!({}))
    ///     .with_input(json!({"key": "value"}));
    /// ```
    pub fn with_input(mut self, input: Value) -> Self {
        self.input = input;
        self
    }

    /// Sets the cache control for this tool call content block.
    ///
    /// # Arguments
    ///
    /// * `cache_control` - The cache control configuration
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::tool_call::AnthropicToolCallContent;
    /// use ai_sdk_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    /// use serde_json::json;
    ///
    /// let tool_call = AnthropicToolCallContent::new("id", "tool", json!({}))
    ///     .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes));
    /// ```
    pub fn with_cache_control(mut self, cache_control: AnthropicCacheControl) -> Self {
        self.cache_control = Some(cache_control);
        self
    }

    /// Removes cache control from this tool call content block.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::tool_call::AnthropicToolCallContent;
    /// use ai_sdk_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    /// use serde_json::json;
    ///
    /// let tool_call = AnthropicToolCallContent::new("id", "tool", json!({}))
    ///     .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour))
    ///     .without_cache_control();
    ///
    /// assert!(tool_call.cache_control.is_none());
    /// ```
    pub fn without_cache_control(mut self) -> Self {
        self.cache_control = None;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    use serde_json::json;

    #[test]
    fn test_new() {
        let tool_call = AnthropicToolCallContent::new(
            "toolu_123",
            "get_weather",
            json!({"city": "San Francisco"}),
        );

        assert_eq!(tool_call.content_type, "tool_use");
        assert_eq!(tool_call.id, "toolu_123");
        assert_eq!(tool_call.name, "get_weather");
        assert_eq!(tool_call.input, json!({"city": "San Francisco"}));
        assert!(tool_call.cache_control.is_none());
    }

    #[test]
    fn test_new_with_string() {
        let tool_call =
            AnthropicToolCallContent::new("id".to_string(), "tool".to_string(), json!({}));

        assert_eq!(tool_call.id, "id");
        assert_eq!(tool_call.name, "tool");
    }

    #[test]
    fn test_with_id() {
        let tool_call =
            AnthropicToolCallContent::new("old_id", "tool", json!({})).with_id("new_id");

        assert_eq!(tool_call.id, "new_id");
    }

    #[test]
    fn test_with_name() {
        let tool_call =
            AnthropicToolCallContent::new("id", "old_name", json!({})).with_name("new_name");

        assert_eq!(tool_call.name, "new_name");
    }

    #[test]
    fn test_with_input() {
        let new_input = json!({"key": "value"});
        let tool_call =
            AnthropicToolCallContent::new("id", "tool", json!({})).with_input(new_input.clone());

        assert_eq!(tool_call.input, new_input);
    }

    #[test]
    fn test_with_cache_control() {
        let cache_control = AnthropicCacheControl::new(CacheTTL::FiveMinutes);
        let tool_call = AnthropicToolCallContent::new("id", "tool", json!({}))
            .with_cache_control(cache_control.clone());

        assert_eq!(tool_call.cache_control, Some(cache_control));
    }

    #[test]
    fn test_without_cache_control() {
        let tool_call = AnthropicToolCallContent::new("id", "tool", json!({}))
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour))
            .without_cache_control();

        assert!(tool_call.cache_control.is_none());
    }

    #[test]
    fn test_serialize_without_cache() {
        let tool_call =
            AnthropicToolCallContent::new("toolu_abc", "calculate", json!({"a": 1, "b": 2}));
        let json = serde_json::to_value(&tool_call).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "tool_use",
                "id": "toolu_abc",
                "name": "calculate",
                "input": {"a": 1, "b": 2}
            })
        );
    }

    #[test]
    fn test_serialize_with_cache() {
        let tool_call =
            AnthropicToolCallContent::new("toolu_xyz", "search", json!({"query": "test"}))
                .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&tool_call).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "tool_use",
                "id": "toolu_xyz",
                "name": "search",
                "input": {"query": "test"},
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
            "type": "tool_use",
            "id": "toolu_123",
            "name": "get_user",
            "input": {"user_id": "456"}
        });

        let tool_call: AnthropicToolCallContent = serde_json::from_value(json).unwrap();

        assert_eq!(tool_call.content_type, "tool_use");
        assert_eq!(tool_call.id, "toolu_123");
        assert_eq!(tool_call.name, "get_user");
        assert_eq!(tool_call.input, json!({"user_id": "456"}));
        assert!(tool_call.cache_control.is_none());
    }

    #[test]
    fn test_deserialize_with_cache() {
        let json = json!({
            "type": "tool_use",
            "id": "toolu_789",
            "name": "fetch_data",
            "input": {"source": "api"},
            "cache_control": {
                "type": "ephemeral",
                "ttl": "5m"
            }
        });

        let tool_call: AnthropicToolCallContent = serde_json::from_value(json).unwrap();

        assert_eq!(
            tool_call.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
        );
    }

    #[test]
    fn test_input_types() {
        // Object
        let obj = AnthropicToolCallContent::new("id", "tool", json!({"key": "value"}));
        assert!(obj.input.is_object());

        // Array
        let arr = AnthropicToolCallContent::new("id", "tool", json!([1, 2, 3]));
        assert!(arr.input.is_array());

        // String
        let str = AnthropicToolCallContent::new("id", "tool", json!("string"));
        assert!(str.input.is_string());

        // Number
        let num = AnthropicToolCallContent::new("id", "tool", json!(42));
        assert!(num.input.is_number());

        // Boolean
        let bool = AnthropicToolCallContent::new("id", "tool", json!(true));
        assert!(bool.input.is_boolean());

        // Null
        let null = AnthropicToolCallContent::new("id", "tool", json!(null));
        assert!(null.input.is_null());
    }

    #[test]
    fn test_equality() {
        let tool1 = AnthropicToolCallContent::new("id", "tool", json!({}));
        let tool2 = AnthropicToolCallContent::new("id", "tool", json!({}));
        let tool3 = AnthropicToolCallContent::new("id", "tool", json!({"key": "value"}));

        assert_eq!(tool1, tool2);
        assert_ne!(tool1, tool3);
    }

    #[test]
    fn test_clone() {
        let tool1 = AnthropicToolCallContent::new("id", "tool", json!({"a": 1}))
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let tool2 = tool1.clone();

        assert_eq!(tool1, tool2);
    }

    #[test]
    fn test_builder_chaining() {
        let tool_call = AnthropicToolCallContent::new("id1", "tool1", json!({}))
            .with_id("id2")
            .with_name("tool2")
            .with_input(json!({"key": "value"}))
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
            .with_id("id3")
            .without_cache_control();

        assert_eq!(tool_call.id, "id3");
        assert_eq!(tool_call.name, "tool2");
        assert_eq!(tool_call.input, json!({"key": "value"}));
        assert!(tool_call.cache_control.is_none());
    }

    #[test]
    fn test_complex_input() {
        let complex_input = json!({
            "nested": {
                "array": [1, 2, 3],
                "object": {
                    "key": "value"
                }
            },
            "unicode": "Hello 世界 🌍"
        });

        let tool_call = AnthropicToolCallContent::new("id", "complex_tool", complex_input.clone());

        assert_eq!(tool_call.input, complex_input);
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original = AnthropicToolCallContent::new(
            "toolu_roundtrip",
            "test_tool",
            json!({"param": "value", "number": 42}),
        )
        .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicToolCallContent = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_empty_input() {
        let tool_call = AnthropicToolCallContent::new("id", "tool", json!({}));

        assert_eq!(tool_call.input, json!({}));
        assert!(tool_call.input.is_object());
    }
}
