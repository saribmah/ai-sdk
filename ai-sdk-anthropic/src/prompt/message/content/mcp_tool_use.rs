use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::prompt::message::cache_control::AnthropicCacheControl;

/// Anthropic MCP (Model Context Protocol) tool use content block.
///
/// Represents an MCP tool use content block in an Anthropic message.
/// MCP tools are external tools provided by MCP servers that the model can invoke.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::prompt::message::content::mcp_tool_use::AnthropicMcpToolUseContent;
/// use serde_json::json;
///
/// // Create an MCP tool use
/// let mcp_tool = AnthropicMcpToolUseContent::new(
///     "mcp_toolu_123",
///     "read_file",
///     "filesystem_server",
///     json!({"path": "/home/user/document.txt"})
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicMcpToolUseContent {
    /// The type of content (always "mcp_tool_use")
    #[serde(rename = "type")]
    pub content_type: String,

    /// Unique identifier for this MCP tool use
    pub id: String,

    /// Name of the MCP tool to call
    pub name: String,

    /// Name of the MCP server providing this tool
    pub server_name: String,

    /// Input/arguments for the MCP tool call
    pub input: Value,

    /// Optional cache control configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}

impl AnthropicMcpToolUseContent {
    /// Creates a new MCP tool use content block.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this MCP tool use
    /// * `name` - Name of the MCP tool to call
    /// * `server_name` - Name of the MCP server providing this tool
    /// * `input` - Input/arguments for the MCP tool call
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::mcp_tool_use::AnthropicMcpToolUseContent;
    /// use serde_json::json;
    ///
    /// let mcp_tool = AnthropicMcpToolUseContent::new(
    ///     "mcp_toolu_abc123",
    ///     "search_database",
    ///     "db_server",
    ///     json!({"query": "SELECT * FROM users", "limit": 10})
    /// );
    /// ```
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        server_name: impl Into<String>,
        input: Value,
    ) -> Self {
        Self {
            content_type: "mcp_tool_use".to_string(),
            id: id.into(),
            name: name.into(),
            server_name: server_name.into(),
            input,
            cache_control: None,
        }
    }

    /// Updates the MCP tool use ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The new MCP tool use ID
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::mcp_tool_use::AnthropicMcpToolUseContent;
    /// use serde_json::json;
    ///
    /// let mcp_tool = AnthropicMcpToolUseContent::new("old_id", "tool", "server", json!({}))
    ///     .with_id("new_id");
    /// assert_eq!(mcp_tool.id, "new_id");
    /// ```
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Updates the MCP tool name.
    ///
    /// # Arguments
    ///
    /// * `name` - The new MCP tool name
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::mcp_tool_use::AnthropicMcpToolUseContent;
    /// use serde_json::json;
    ///
    /// let mcp_tool = AnthropicMcpToolUseContent::new("id", "old_tool", "server", json!({}))
    ///     .with_name("new_tool");
    /// assert_eq!(mcp_tool.name, "new_tool");
    /// ```
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Updates the MCP server name.
    ///
    /// # Arguments
    ///
    /// * `server_name` - The new MCP server name
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::mcp_tool_use::AnthropicMcpToolUseContent;
    /// use serde_json::json;
    ///
    /// let mcp_tool = AnthropicMcpToolUseContent::new("id", "tool", "old_server", json!({}))
    ///     .with_server_name("new_server");
    /// assert_eq!(mcp_tool.server_name, "new_server");
    /// ```
    pub fn with_server_name(mut self, server_name: impl Into<String>) -> Self {
        self.server_name = server_name.into();
        self
    }

    /// Updates the MCP tool input.
    ///
    /// # Arguments
    ///
    /// * `input` - The new MCP tool input
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::mcp_tool_use::AnthropicMcpToolUseContent;
    /// use serde_json::json;
    ///
    /// let mcp_tool = AnthropicMcpToolUseContent::new("id", "tool", "server", json!({}))
    ///     .with_input(json!({"key": "value"}));
    /// ```
    pub fn with_input(mut self, input: Value) -> Self {
        self.input = input;
        self
    }

    /// Sets the cache control for this MCP tool use content block.
    ///
    /// # Arguments
    ///
    /// * `cache_control` - The cache control configuration
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::mcp_tool_use::AnthropicMcpToolUseContent;
    /// use ai_sdk_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    /// use serde_json::json;
    ///
    /// let mcp_tool = AnthropicMcpToolUseContent::new("id", "tool", "server", json!({}))
    ///     .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes));
    /// ```
    pub fn with_cache_control(mut self, cache_control: AnthropicCacheControl) -> Self {
        self.cache_control = Some(cache_control);
        self
    }

    /// Removes cache control from this MCP tool use content block.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::mcp_tool_use::AnthropicMcpToolUseContent;
    /// use ai_sdk_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    /// use serde_json::json;
    ///
    /// let mcp_tool = AnthropicMcpToolUseContent::new("id", "tool", "server", json!({}))
    ///     .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour))
    ///     .without_cache_control();
    ///
    /// assert!(mcp_tool.cache_control.is_none());
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
        let mcp_tool = AnthropicMcpToolUseContent::new(
            "mcp_toolu_123",
            "read_file",
            "filesystem_server",
            json!({"path": "/home/user/doc.txt"}),
        );

        assert_eq!(mcp_tool.content_type, "mcp_tool_use");
        assert_eq!(mcp_tool.id, "mcp_toolu_123");
        assert_eq!(mcp_tool.name, "read_file");
        assert_eq!(mcp_tool.server_name, "filesystem_server");
        assert_eq!(mcp_tool.input, json!({"path": "/home/user/doc.txt"}));
        assert!(mcp_tool.cache_control.is_none());
    }

    #[test]
    fn test_new_with_string() {
        let mcp_tool = AnthropicMcpToolUseContent::new(
            "id".to_string(),
            "tool".to_string(),
            "server".to_string(),
            json!({}),
        );

        assert_eq!(mcp_tool.id, "id");
        assert_eq!(mcp_tool.name, "tool");
        assert_eq!(mcp_tool.server_name, "server");
    }

    #[test]
    fn test_with_id() {
        let mcp_tool = AnthropicMcpToolUseContent::new("old_id", "tool", "server", json!({}))
            .with_id("new_id");

        assert_eq!(mcp_tool.id, "new_id");
    }

    #[test]
    fn test_with_name() {
        let mcp_tool = AnthropicMcpToolUseContent::new("id", "old_name", "server", json!({}))
            .with_name("new_name");

        assert_eq!(mcp_tool.name, "new_name");
    }

    #[test]
    fn test_with_server_name() {
        let mcp_tool = AnthropicMcpToolUseContent::new("id", "tool", "old_server", json!({}))
            .with_server_name("new_server");

        assert_eq!(mcp_tool.server_name, "new_server");
    }

    #[test]
    fn test_with_input() {
        let new_input = json!({"key": "value"});
        let mcp_tool = AnthropicMcpToolUseContent::new("id", "tool", "server", json!({}))
            .with_input(new_input.clone());

        assert_eq!(mcp_tool.input, new_input);
    }

    #[test]
    fn test_with_cache_control() {
        let cache_control = AnthropicCacheControl::new(CacheTTL::FiveMinutes);
        let mcp_tool = AnthropicMcpToolUseContent::new("id", "tool", "server", json!({}))
            .with_cache_control(cache_control.clone());

        assert_eq!(mcp_tool.cache_control, Some(cache_control));
    }

    #[test]
    fn test_without_cache_control() {
        let mcp_tool = AnthropicMcpToolUseContent::new("id", "tool", "server", json!({}))
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour))
            .without_cache_control();

        assert!(mcp_tool.cache_control.is_none());
    }

    #[test]
    fn test_serialize_without_cache() {
        let mcp_tool = AnthropicMcpToolUseContent::new(
            "mcp_toolu_abc",
            "query_db",
            "database_server",
            json!({"query": "SELECT * FROM users"}),
        );
        let json = serde_json::to_value(&mcp_tool).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "mcp_tool_use",
                "id": "mcp_toolu_abc",
                "name": "query_db",
                "server_name": "database_server",
                "input": {"query": "SELECT * FROM users"}
            })
        );
    }

    #[test]
    fn test_serialize_with_cache() {
        let mcp_tool = AnthropicMcpToolUseContent::new(
            "mcp_toolu_xyz",
            "search",
            "search_server",
            json!({"query": "test"}),
        )
        .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&mcp_tool).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "mcp_tool_use",
                "id": "mcp_toolu_xyz",
                "name": "search",
                "server_name": "search_server",
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
            "type": "mcp_tool_use",
            "id": "mcp_toolu_123",
            "name": "get_weather",
            "server_name": "weather_server",
            "input": {"city": "San Francisco"}
        });

        let mcp_tool: AnthropicMcpToolUseContent = serde_json::from_value(json).unwrap();

        assert_eq!(mcp_tool.content_type, "mcp_tool_use");
        assert_eq!(mcp_tool.id, "mcp_toolu_123");
        assert_eq!(mcp_tool.name, "get_weather");
        assert_eq!(mcp_tool.server_name, "weather_server");
        assert_eq!(mcp_tool.input, json!({"city": "San Francisco"}));
        assert!(mcp_tool.cache_control.is_none());
    }

    #[test]
    fn test_deserialize_with_cache() {
        let json = json!({
            "type": "mcp_tool_use",
            "id": "mcp_toolu_789",
            "name": "fetch_data",
            "server_name": "api_server",
            "input": {"endpoint": "/users"},
            "cache_control": {
                "type": "ephemeral",
                "ttl": "5m"
            }
        });

        let mcp_tool: AnthropicMcpToolUseContent = serde_json::from_value(json).unwrap();

        assert_eq!(
            mcp_tool.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
        );
    }

    #[test]
    fn test_input_types() {
        // Object
        let obj = AnthropicMcpToolUseContent::new("id", "tool", "server", json!({"key": "value"}));
        assert!(obj.input.is_object());

        // Array
        let arr = AnthropicMcpToolUseContent::new("id", "tool", "server", json!([1, 2, 3]));
        assert!(arr.input.is_array());

        // String
        let str = AnthropicMcpToolUseContent::new("id", "tool", "server", json!("string"));
        assert!(str.input.is_string());

        // Number
        let num = AnthropicMcpToolUseContent::new("id", "tool", "server", json!(42));
        assert!(num.input.is_number());

        // Boolean
        let bool = AnthropicMcpToolUseContent::new("id", "tool", "server", json!(true));
        assert!(bool.input.is_boolean());

        // Null
        let null = AnthropicMcpToolUseContent::new("id", "tool", "server", json!(null));
        assert!(null.input.is_null());
    }

    #[test]
    fn test_equality() {
        let tool1 = AnthropicMcpToolUseContent::new("id", "tool", "server", json!({}));
        let tool2 = AnthropicMcpToolUseContent::new("id", "tool", "server", json!({}));
        let tool3 = AnthropicMcpToolUseContent::new("id", "tool", "other_server", json!({}));

        assert_eq!(tool1, tool2);
        assert_ne!(tool1, tool3);
    }

    #[test]
    fn test_clone() {
        let tool1 = AnthropicMcpToolUseContent::new("id", "tool", "server", json!({"a": 1}))
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let tool2 = tool1.clone();

        assert_eq!(tool1, tool2);
    }

    #[test]
    fn test_builder_chaining() {
        let mcp_tool = AnthropicMcpToolUseContent::new("id1", "tool1", "server1", json!({}))
            .with_id("id2")
            .with_name("tool2")
            .with_server_name("server2")
            .with_input(json!({"key": "value"}))
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
            .with_id("id3")
            .without_cache_control();

        assert_eq!(mcp_tool.id, "id3");
        assert_eq!(mcp_tool.name, "tool2");
        assert_eq!(mcp_tool.server_name, "server2");
        assert_eq!(mcp_tool.input, json!({"key": "value"}));
        assert!(mcp_tool.cache_control.is_none());
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

        let mcp_tool =
            AnthropicMcpToolUseContent::new("id", "complex_tool", "server", complex_input.clone());

        assert_eq!(mcp_tool.input, complex_input);
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original = AnthropicMcpToolUseContent::new(
            "mcp_toolu_roundtrip",
            "test_tool",
            "test_server",
            json!({"param": "value", "number": 42}),
        )
        .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicMcpToolUseContent = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_empty_input() {
        let mcp_tool = AnthropicMcpToolUseContent::new("id", "tool", "server", json!({}));

        assert_eq!(mcp_tool.input, json!({}));
        assert!(mcp_tool.input.is_object());
    }

    #[test]
    fn test_different_server_names() {
        let mcp_tool1 =
            AnthropicMcpToolUseContent::new("id", "read_file", "filesystem_server", json!({}));
        let mcp_tool2 =
            AnthropicMcpToolUseContent::new("id", "query", "database_server", json!({}));
        let mcp_tool3 = AnthropicMcpToolUseContent::new("id", "search", "search_server", json!({}));

        assert_eq!(mcp_tool1.server_name, "filesystem_server");
        assert_eq!(mcp_tool2.server_name, "database_server");
        assert_eq!(mcp_tool3.server_name, "search_server");
    }

    #[test]
    fn test_server_name_with_special_characters() {
        let mcp_tool = AnthropicMcpToolUseContent::new("id", "tool", "my-server_v2.0", json!({}));

        assert_eq!(mcp_tool.server_name, "my-server_v2.0");
    }
}
