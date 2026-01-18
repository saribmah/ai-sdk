use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::prompt::message::cache_control::AnthropicCacheControl;

/// Custom tool definition with JSON schema for input validation.
///
/// This is the standard tool type that allows you to define custom tools with
/// JSON Schema for input validation. The model will generate structured input
/// that conforms to the provided schema.
///
/// # Example
///
/// ```
/// use llm_kit_anthropic::prompt::tool::custom_tool::AnthropicCustomTool;
/// use serde_json::json;
///
/// let tool = AnthropicCustomTool::new(
///     "get_weather",
///     json!({
///         "type": "object",
///         "properties": {
///             "location": {
///                 "type": "string",
///                 "description": "The city name"
///             }
///         },
///         "required": ["location"]
///     })
/// )
/// .with_description("Get the current weather for a location");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicCustomTool {
    /// The name of the tool
    pub name: String,

    /// Optional description of what the tool does
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// JSON Schema (JSON Schema Draft 7) for the tool's input
    pub input_schema: JsonValue,

    /// Optional cache control configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}

impl AnthropicCustomTool {
    /// Creates a new custom tool.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the tool
    /// * `input_schema` - JSON Schema (Draft 7) for the tool's input
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::tool::custom_tool::AnthropicCustomTool;
    /// use serde_json::json;
    ///
    /// let tool = AnthropicCustomTool::new(
    ///     "search",
    ///     json!({
    ///         "type": "object",
    ///         "properties": {
    ///             "query": { "type": "string" }
    ///         }
    ///     })
    /// );
    /// ```
    pub fn new(name: impl Into<String>, input_schema: JsonValue) -> Self {
        Self {
            name: name.into(),
            description: None,
            input_schema,
            cache_control: None,
        }
    }

    /// Sets the description for this tool.
    ///
    /// # Arguments
    ///
    /// * `description` - Description of what the tool does
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::tool::custom_tool::AnthropicCustomTool;
    /// use serde_json::json;
    ///
    /// let tool = AnthropicCustomTool::new("tool", json!({}))
    ///     .with_description("This tool does something useful");
    /// ```
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Removes the description from this tool.
    pub fn without_description(mut self) -> Self {
        self.description = None;
        self
    }

    /// Sets the cache control for this tool.
    ///
    /// # Arguments
    ///
    /// * `cache_control` - The cache control configuration
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::tool::custom_tool::AnthropicCustomTool;
    /// use llm_kit_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    /// use serde_json::json;
    ///
    /// let tool = AnthropicCustomTool::new("tool", json!({}))
    ///     .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
    /// ```
    pub fn with_cache_control(mut self, cache_control: AnthropicCacheControl) -> Self {
        self.cache_control = Some(cache_control);
        self
    }

    /// Removes cache control from this tool.
    pub fn without_cache_control(mut self) -> Self {
        self.cache_control = None;
        self
    }

    /// Updates the name of this tool.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Updates the input schema of this tool.
    pub fn with_input_schema(mut self, input_schema: JsonValue) -> Self {
        self.input_schema = input_schema;
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
        let schema = json!({
            "type": "object",
            "properties": {
                "query": { "type": "string" }
            }
        });
        let tool = AnthropicCustomTool::new("search", schema.clone());

        assert_eq!(tool.name, "search");
        assert!(tool.description.is_none());
        assert_eq!(tool.input_schema, schema);
        assert!(tool.cache_control.is_none());
    }

    #[test]
    fn test_with_description() {
        let tool = AnthropicCustomTool::new("tool", json!({})).with_description("A helpful tool");

        assert_eq!(tool.description, Some("A helpful tool".to_string()));
    }

    #[test]
    fn test_without_description() {
        let tool = AnthropicCustomTool::new("tool", json!({}))
            .with_description("desc")
            .without_description();

        assert!(tool.description.is_none());
    }

    #[test]
    fn test_with_cache_control() {
        let cache = AnthropicCacheControl::new(CacheTTL::OneHour);
        let tool = AnthropicCustomTool::new("tool", json!({})).with_cache_control(cache.clone());

        assert_eq!(tool.cache_control, Some(cache));
    }

    #[test]
    fn test_without_cache_control() {
        let tool = AnthropicCustomTool::new("tool", json!({}))
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
            .without_cache_control();

        assert!(tool.cache_control.is_none());
    }

    #[test]
    fn test_with_name() {
        let tool = AnthropicCustomTool::new("old_name", json!({})).with_name("new_name");

        assert_eq!(tool.name, "new_name");
    }

    #[test]
    fn test_with_input_schema() {
        let schema1 = json!({ "type": "string" });
        let schema2 = json!({ "type": "number" });
        let tool = AnthropicCustomTool::new("tool", schema1).with_input_schema(schema2.clone());

        assert_eq!(tool.input_schema, schema2);
    }

    #[test]
    fn test_serialize_minimal() {
        let tool = AnthropicCustomTool::new(
            "get_weather",
            json!({
                "type": "object",
                "properties": {
                    "location": { "type": "string" }
                }
            }),
        );

        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "name": "get_weather",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "location": { "type": "string" }
                    }
                }
            })
        );
    }

    #[test]
    fn test_serialize_with_description() {
        let tool = AnthropicCustomTool::new("tool", json!({})).with_description("A useful tool");

        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "name": "tool",
                "description": "A useful tool",
                "input_schema": {}
            })
        );
    }

    #[test]
    fn test_serialize_with_cache_control() {
        let tool = AnthropicCustomTool::new("tool", json!({}))
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "name": "tool",
                "input_schema": {},
                "cache_control": {
                    "type": "ephemeral",
                    "ttl": "1h"
                }
            })
        );
    }

    #[test]
    fn test_serialize_full() {
        let tool = AnthropicCustomTool::new(
            "search",
            json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string" }
                },
                "required": ["query"]
            }),
        )
        .with_description("Search for information")
        .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes));

        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "name": "search",
                "description": "Search for information",
                "input_schema": {
                    "type": "object",
                    "properties": {
                        "query": { "type": "string" }
                    },
                    "required": ["query"]
                },
                "cache_control": {
                    "type": "ephemeral",
                    "ttl": "5m"
                }
            })
        );
    }

    #[test]
    fn test_deserialize() {
        let json = json!({
            "name": "calculator",
            "description": "Perform calculations",
            "input_schema": {
                "type": "object",
                "properties": {
                    "expression": { "type": "string" }
                }
            },
            "cache_control": {
                "type": "ephemeral",
                "ttl": "1h"
            }
        });

        let tool: AnthropicCustomTool = serde_json::from_value(json).unwrap();

        assert_eq!(tool.name, "calculator");
        assert_eq!(tool.description, Some("Perform calculations".to_string()));
        assert_eq!(
            tool.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::OneHour))
        );
    }

    #[test]
    fn test_equality() {
        let tool1 = AnthropicCustomTool::new("tool", json!({}));
        let tool2 = AnthropicCustomTool::new("tool", json!({}));
        let tool3 = AnthropicCustomTool::new("different", json!({}));

        assert_eq!(tool1, tool2);
        assert_ne!(tool1, tool3);
    }

    #[test]
    fn test_clone() {
        let tool1 = AnthropicCustomTool::new("tool", json!({}))
            .with_description("desc")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let tool2 = tool1.clone();

        assert_eq!(tool1, tool2);
    }

    #[test]
    fn test_builder_chaining() {
        let tool = AnthropicCustomTool::new("tool1", json!({ "type": "string" }))
            .with_name("tool2")
            .with_description("desc1")
            .with_description("desc2")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
            .without_description()
            .with_input_schema(json!({ "type": "number" }))
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        assert_eq!(tool.name, "tool2");
        assert!(tool.description.is_none());
        assert_eq!(tool.input_schema, json!({ "type": "number" }));
        assert_eq!(
            tool.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::OneHour))
        );
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original = AnthropicCustomTool::new(
            "my_tool",
            json!({
                "type": "object",
                "properties": {
                    "param": { "type": "string" }
                }
            }),
        )
        .with_description("My tool description")
        .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicCustomTool = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }
}
