use serde::{Deserialize, Serialize};

use crate::prompt::message::cache_control::AnthropicCacheControl;

/// Nested text content for MCP tool results.
///
/// MCP tool results only support text content in array format.
/// This is a simpler version compared to regular tool results.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct McpNestedTextContent {
    /// The type of content (always "text")
    #[serde(rename = "type")]
    pub content_type: String,

    /// The text content
    pub text: String,
}

impl McpNestedTextContent {
    /// Creates a new MCP nested text content block.
    ///
    /// # Arguments
    ///
    /// * `text` - The text content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::mcp_tool_result::McpNestedTextContent;
    ///
    /// let text = McpNestedTextContent::new("MCP tool result text");
    /// ```
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            content_type: "text".to_string(),
            text: text.into(),
        }
    }
}

impl From<String> for McpNestedTextContent {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

impl From<&str> for McpNestedTextContent {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

/// MCP tool result content type.
///
/// Can be either a simple string or an array of text content blocks.
/// Unlike regular tool results, MCP tool results only support text content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum McpToolResultContentType {
    /// Simple string content
    String(String),

    /// Array of text content blocks
    Array(Vec<McpNestedTextContent>),
}

impl From<String> for McpToolResultContentType {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for McpToolResultContentType {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<Vec<McpNestedTextContent>> for McpToolResultContentType {
    fn from(v: Vec<McpNestedTextContent>) -> Self {
        Self::Array(v)
    }
}

/// Anthropic MCP tool result content block.
///
/// Represents the result of an MCP tool execution. The content can be either a simple
/// string or an array of text content blocks.
///
/// # Example
///
/// ```
/// use llm_kit_anthropic::prompt::message::content::mcp_tool_result::{
///     AnthropicMcpToolResultContent, McpToolResultContentType
/// };
///
/// // Simple string result
/// let result = AnthropicMcpToolResultContent::new(
///     "mcp_toolu_123",
///     false,
///     McpToolResultContentType::String("File content: Hello World".to_string())
/// );
///
/// // With error flag
/// let error_result = AnthropicMcpToolResultContent::new(
///     "mcp_toolu_456",
///     true,
///     McpToolResultContentType::String("File not found".to_string())
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicMcpToolResultContent {
    /// The type of content (always "mcp_tool_result")
    #[serde(rename = "type")]
    pub content_type: String,

    /// ID of the MCP tool use this is a result for
    pub tool_use_id: String,

    /// Whether this result represents an error
    pub is_error: bool,

    /// The result content (string or array of text content)
    pub content: McpToolResultContentType,

    /// Optional cache control configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}

impl AnthropicMcpToolResultContent {
    /// Creates a new MCP tool result content block.
    ///
    /// # Arguments
    ///
    /// * `tool_use_id` - ID of the MCP tool use this is a result for
    /// * `is_error` - Whether this result represents an error
    /// * `content` - The result content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::mcp_tool_result::{
    ///     AnthropicMcpToolResultContent, McpToolResultContentType
    /// };
    ///
    /// let result = AnthropicMcpToolResultContent::new(
    ///     "mcp_toolu_abc123",
    ///     false,
    ///     McpToolResultContentType::String("Success".to_string())
    /// );
    /// ```
    pub fn new(
        tool_use_id: impl Into<String>,
        is_error: bool,
        content: McpToolResultContentType,
    ) -> Self {
        Self {
            content_type: "mcp_tool_result".to_string(),
            tool_use_id: tool_use_id.into(),
            is_error,
            content,
            cache_control: None,
        }
    }

    /// Creates a new MCP tool result with string content.
    ///
    /// # Arguments
    ///
    /// * `tool_use_id` - ID of the MCP tool use this is a result for
    /// * `is_error` - Whether this result represents an error
    /// * `content` - The result content as a string
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::mcp_tool_result::AnthropicMcpToolResultContent;
    ///
    /// let result = AnthropicMcpToolResultContent::from_string(
    ///     "mcp_toolu_123",
    ///     false,
    ///     "File read successfully"
    /// );
    /// ```
    pub fn from_string(
        tool_use_id: impl Into<String>,
        is_error: bool,
        content: impl Into<String>,
    ) -> Self {
        Self::new(
            tool_use_id,
            is_error,
            McpToolResultContentType::String(content.into()),
        )
    }

    /// Creates a new MCP tool result with array content.
    ///
    /// # Arguments
    ///
    /// * `tool_use_id` - ID of the MCP tool use this is a result for
    /// * `is_error` - Whether this result represents an error
    /// * `content` - The result content as an array
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::mcp_tool_result::{
    ///     AnthropicMcpToolResultContent, McpNestedTextContent
    /// };
    ///
    /// let result = AnthropicMcpToolResultContent::from_array(
    ///     "mcp_toolu_123",
    ///     false,
    ///     vec![McpNestedTextContent::new("Line 1"), McpNestedTextContent::new("Line 2")]
    /// );
    /// ```
    pub fn from_array(
        tool_use_id: impl Into<String>,
        is_error: bool,
        content: Vec<McpNestedTextContent>,
    ) -> Self {
        Self::new(
            tool_use_id,
            is_error,
            McpToolResultContentType::Array(content),
        )
    }

    /// Creates a success result with string content.
    ///
    /// # Arguments
    ///
    /// * `tool_use_id` - ID of the MCP tool use this is a result for
    /// * `content` - The result content as a string
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::mcp_tool_result::AnthropicMcpToolResultContent;
    ///
    /// let result = AnthropicMcpToolResultContent::success(
    ///     "mcp_toolu_123",
    ///     "Operation completed successfully"
    /// );
    /// ```
    pub fn success(tool_use_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self::from_string(tool_use_id, false, content)
    }

    /// Creates an error result with string content.
    ///
    /// # Arguments
    ///
    /// * `tool_use_id` - ID of the MCP tool use this is a result for
    /// * `content` - The error message
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::mcp_tool_result::AnthropicMcpToolResultContent;
    ///
    /// let result = AnthropicMcpToolResultContent::error(
    ///     "mcp_toolu_123",
    ///     "File not found"
    /// );
    /// ```
    pub fn error(tool_use_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self::from_string(tool_use_id, true, content)
    }

    /// Updates the tool use ID.
    ///
    /// # Arguments
    ///
    /// * `tool_use_id` - The new tool use ID
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::mcp_tool_result::AnthropicMcpToolResultContent;
    ///
    /// let result = AnthropicMcpToolResultContent::success("old_id", "Success")
    ///     .with_tool_use_id("new_id");
    /// assert_eq!(result.tool_use_id, "new_id");
    /// ```
    pub fn with_tool_use_id(mut self, tool_use_id: impl Into<String>) -> Self {
        self.tool_use_id = tool_use_id.into();
        self
    }

    /// Updates the error flag.
    ///
    /// # Arguments
    ///
    /// * `is_error` - Whether this result represents an error
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::mcp_tool_result::AnthropicMcpToolResultContent;
    ///
    /// let result = AnthropicMcpToolResultContent::success("id", "Success")
    ///     .with_error(true);
    /// assert!(result.is_error);
    /// ```
    pub fn with_error(mut self, is_error: bool) -> Self {
        self.is_error = is_error;
        self
    }

    /// Updates the content.
    ///
    /// # Arguments
    ///
    /// * `content` - The new content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::mcp_tool_result::{
    ///     AnthropicMcpToolResultContent, McpToolResultContentType
    /// };
    ///
    /// let result = AnthropicMcpToolResultContent::success("id", "Old")
    ///     .with_content(McpToolResultContentType::String("New".to_string()));
    /// ```
    pub fn with_content(mut self, content: McpToolResultContentType) -> Self {
        self.content = content;
        self
    }

    /// Sets the cache control for this MCP tool result content block.
    ///
    /// # Arguments
    ///
    /// * `cache_control` - The cache control configuration
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::mcp_tool_result::AnthropicMcpToolResultContent;
    /// use llm_kit_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    ///
    /// let result = AnthropicMcpToolResultContent::success("id", "Success")
    ///     .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes));
    /// ```
    pub fn with_cache_control(mut self, cache_control: AnthropicCacheControl) -> Self {
        self.cache_control = Some(cache_control);
        self
    }

    /// Removes cache control from this MCP tool result content block.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::mcp_tool_result::AnthropicMcpToolResultContent;
    /// use llm_kit_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    ///
    /// let result = AnthropicMcpToolResultContent::success("id", "Success")
    ///     .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour))
    ///     .without_cache_control();
    ///
    /// assert!(result.cache_control.is_none());
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
    fn test_mcp_nested_text_content_new() {
        let text = McpNestedTextContent::new("Test text");

        assert_eq!(text.content_type, "text");
        assert_eq!(text.text, "Test text");
    }

    #[test]
    fn test_mcp_nested_text_content_from_string() {
        let text: McpNestedTextContent = "Test text".into();

        assert_eq!(text.text, "Test text");
    }

    #[test]
    fn test_mcp_nested_text_content_from_owned_string() {
        let text: McpNestedTextContent = "Test text".to_string().into();

        assert_eq!(text.text, "Test text");
    }

    #[test]
    fn test_mcp_tool_result_content_type_string() {
        let content = McpToolResultContentType::String("test".to_string());

        match content {
            McpToolResultContentType::String(s) => assert_eq!(s, "test"),
            _ => panic!("Expected String variant"),
        }
    }

    #[test]
    fn test_mcp_tool_result_content_type_array() {
        let content = McpToolResultContentType::Array(vec![McpNestedTextContent::new("test")]);

        match content {
            McpToolResultContentType::Array(arr) => {
                assert_eq!(arr.len(), 1);
                assert_eq!(arr[0].text, "test");
            }
            _ => panic!("Expected Array variant"),
        }
    }

    #[test]
    fn test_mcp_tool_result_content_type_from_string() {
        let content: McpToolResultContentType = "test".into();

        match content {
            McpToolResultContentType::String(s) => assert_eq!(s, "test"),
            _ => panic!("Expected String variant"),
        }
    }

    #[test]
    fn test_new() {
        let result = AnthropicMcpToolResultContent::new(
            "mcp_toolu_123",
            false,
            McpToolResultContentType::String("Success".to_string()),
        );

        assert_eq!(result.content_type, "mcp_tool_result");
        assert_eq!(result.tool_use_id, "mcp_toolu_123");
        assert!(!result.is_error);
        assert!(result.cache_control.is_none());
    }

    #[test]
    fn test_from_string() {
        let result =
            AnthropicMcpToolResultContent::from_string("mcp_toolu_123", false, "File content");

        match result.content {
            McpToolResultContentType::String(s) => assert_eq!(s, "File content"),
            _ => panic!("Expected String content"),
        }
    }

    #[test]
    fn test_from_array() {
        let result = AnthropicMcpToolResultContent::from_array(
            "mcp_toolu_123",
            false,
            vec![
                McpNestedTextContent::new("Line 1"),
                McpNestedTextContent::new("Line 2"),
            ],
        );

        match result.content {
            McpToolResultContentType::Array(arr) => {
                assert_eq!(arr.len(), 2);
                assert_eq!(arr[0].text, "Line 1");
                assert_eq!(arr[1].text, "Line 2");
            }
            _ => panic!("Expected Array content"),
        }
    }

    #[test]
    fn test_success() {
        let result =
            AnthropicMcpToolResultContent::success("mcp_toolu_123", "Operation successful");

        assert_eq!(result.tool_use_id, "mcp_toolu_123");
        assert!(!result.is_error);
        match result.content {
            McpToolResultContentType::String(s) => assert_eq!(s, "Operation successful"),
            _ => panic!("Expected String content"),
        }
    }

    #[test]
    fn test_error() {
        let result = AnthropicMcpToolResultContent::error("mcp_toolu_123", "File not found");

        assert_eq!(result.tool_use_id, "mcp_toolu_123");
        assert!(result.is_error);
        match result.content {
            McpToolResultContentType::String(s) => assert_eq!(s, "File not found"),
            _ => panic!("Expected String content"),
        }
    }

    #[test]
    fn test_with_tool_use_id() {
        let result =
            AnthropicMcpToolResultContent::success("old_id", "Success").with_tool_use_id("new_id");

        assert_eq!(result.tool_use_id, "new_id");
    }

    #[test]
    fn test_with_error() {
        let result = AnthropicMcpToolResultContent::success("id", "Success").with_error(true);

        assert!(result.is_error);
    }

    #[test]
    fn test_with_content() {
        let result = AnthropicMcpToolResultContent::success("id", "Old")
            .with_content(McpToolResultContentType::String("New".to_string()));

        match result.content {
            McpToolResultContentType::String(s) => assert_eq!(s, "New"),
            _ => panic!("Expected String content"),
        }
    }

    #[test]
    fn test_with_cache_control() {
        let cache_control = AnthropicCacheControl::new(CacheTTL::FiveMinutes);
        let result = AnthropicMcpToolResultContent::success("id", "Success")
            .with_cache_control(cache_control.clone());

        assert_eq!(result.cache_control, Some(cache_control));
    }

    #[test]
    fn test_without_cache_control() {
        let result = AnthropicMcpToolResultContent::success("id", "Success")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour))
            .without_cache_control();

        assert!(result.cache_control.is_none());
    }

    #[test]
    fn test_serialize_string_content_without_cache() {
        let result =
            AnthropicMcpToolResultContent::success("mcp_toolu_abc", "File read successfully");
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "mcp_tool_result",
                "tool_use_id": "mcp_toolu_abc",
                "is_error": false,
                "content": "File read successfully"
            })
        );
    }

    #[test]
    fn test_serialize_string_content_with_error() {
        let result = AnthropicMcpToolResultContent::error("mcp_toolu_xyz", "File not found");
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "mcp_tool_result",
                "tool_use_id": "mcp_toolu_xyz",
                "is_error": true,
                "content": "File not found"
            })
        );
    }

    #[test]
    fn test_serialize_array_content() {
        let result = AnthropicMcpToolResultContent::from_array(
            "mcp_toolu_123",
            false,
            vec![
                McpNestedTextContent::new("Line 1"),
                McpNestedTextContent::new("Line 2"),
            ],
        );
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "mcp_tool_result",
                "tool_use_id": "mcp_toolu_123",
                "is_error": false,
                "content": [
                    {"type": "text", "text": "Line 1"},
                    {"type": "text", "text": "Line 2"}
                ]
            })
        );
    }

    #[test]
    fn test_serialize_with_cache() {
        let result = AnthropicMcpToolResultContent::success("mcp_toolu_abc", "Success")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "mcp_tool_result",
                "tool_use_id": "mcp_toolu_abc",
                "is_error": false,
                "content": "Success",
                "cache_control": {
                    "type": "ephemeral",
                    "ttl": "1h"
                }
            })
        );
    }

    #[test]
    fn test_deserialize_string_content() {
        let json = json!({
            "type": "mcp_tool_result",
            "tool_use_id": "mcp_toolu_123",
            "is_error": false,
            "content": "File content"
        });

        let result: AnthropicMcpToolResultContent = serde_json::from_value(json).unwrap();

        assert_eq!(result.content_type, "mcp_tool_result");
        assert_eq!(result.tool_use_id, "mcp_toolu_123");
        assert!(!result.is_error);
        match result.content {
            McpToolResultContentType::String(s) => assert_eq!(s, "File content"),
            _ => panic!("Expected String content"),
        }
        assert!(result.cache_control.is_none());
    }

    #[test]
    fn test_deserialize_array_content() {
        let json = json!({
            "type": "mcp_tool_result",
            "tool_use_id": "mcp_toolu_456",
            "is_error": false,
            "content": [
                {"type": "text", "text": "First line"},
                {"type": "text", "text": "Second line"}
            ]
        });

        let result: AnthropicMcpToolResultContent = serde_json::from_value(json).unwrap();

        match result.content {
            McpToolResultContentType::Array(arr) => {
                assert_eq!(arr.len(), 2);
                assert_eq!(arr[0].text, "First line");
                assert_eq!(arr[1].text, "Second line");
            }
            _ => panic!("Expected Array content"),
        }
    }

    #[test]
    fn test_deserialize_with_error() {
        let json = json!({
            "type": "mcp_tool_result",
            "tool_use_id": "mcp_toolu_789",
            "is_error": true,
            "content": "Error message"
        });

        let result: AnthropicMcpToolResultContent = serde_json::from_value(json).unwrap();

        assert!(result.is_error);
    }

    #[test]
    fn test_deserialize_with_cache() {
        let json = json!({
            "type": "mcp_tool_result",
            "tool_use_id": "mcp_toolu_abc",
            "is_error": false,
            "content": "Success",
            "cache_control": {
                "type": "ephemeral",
                "ttl": "5m"
            }
        });

        let result: AnthropicMcpToolResultContent = serde_json::from_value(json).unwrap();

        assert_eq!(
            result.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
        );
    }

    #[test]
    fn test_equality() {
        let result1 = AnthropicMcpToolResultContent::success("id", "Success");
        let result2 = AnthropicMcpToolResultContent::success("id", "Success");
        let result3 = AnthropicMcpToolResultContent::error("id", "Error");

        assert_eq!(result1, result2);
        assert_ne!(result1, result3);
    }

    #[test]
    fn test_clone() {
        let result1 = AnthropicMcpToolResultContent::success("id", "Success")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let result2 = result1.clone();

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_builder_chaining() {
        let result = AnthropicMcpToolResultContent::new(
            "id1",
            false,
            McpToolResultContentType::String("Old".to_string()),
        )
        .with_tool_use_id("id2")
        .with_error(true)
        .with_content(McpToolResultContentType::String("New".to_string()))
        .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
        .with_tool_use_id("id3")
        .without_cache_control();

        assert_eq!(result.tool_use_id, "id3");
        assert!(result.is_error);
        match result.content {
            McpToolResultContentType::String(s) => assert_eq!(s, "New"),
            _ => panic!("Expected String content"),
        }
        assert!(result.cache_control.is_none());
    }

    #[test]
    fn test_roundtrip_serialization_string() {
        let original =
            AnthropicMcpToolResultContent::success("mcp_toolu_roundtrip", "Test content")
                .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicMcpToolResultContent = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_roundtrip_serialization_array() {
        let original = AnthropicMcpToolResultContent::from_array(
            "mcp_toolu_roundtrip",
            false,
            vec![
                McpNestedTextContent::new("Line 1"),
                McpNestedTextContent::new("Line 2"),
            ],
        )
        .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicMcpToolResultContent = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_empty_array() {
        let result = AnthropicMcpToolResultContent::from_array("mcp_toolu_123", false, vec![]);

        match result.content {
            McpToolResultContentType::Array(arr) => assert!(arr.is_empty()),
            _ => panic!("Expected Array content"),
        }
    }

    #[test]
    fn test_unicode_content() {
        let result =
            AnthropicMcpToolResultContent::success("mcp_toolu_123", "Hello 世界 🌍 Здравствуй мир");

        match result.content {
            McpToolResultContentType::String(s) => assert_eq!(s, "Hello 世界 🌍 Здравствуй мир"),
            _ => panic!("Expected String content"),
        }
    }
}
