use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::prompt::message::cache_control::AnthropicCacheControl;

/// Server tool types supported by Anthropic.
///
/// Represents the built-in server-side tools provided by Anthropic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServerToolType {
    /// Web fetch tool for retrieving web content
    WebFetch,

    /// Web search tool for searching the web
    WebSearch,

    /// Code execution tool (version 20250522)
    CodeExecution,

    /// Bash code execution tool (version 20250825)
    BashCodeExecution,

    /// Text editor code execution tool (version 20250825)
    TextEditorCodeExecution,
}

/// Anthropic server tool use content block.
///
/// Represents a server-side tool use content block in an Anthropic message.
/// These are built-in tools provided by Anthropic (like web search, code execution, etc.).
///
/// # Example
///
/// ```
/// use llm_kit_anthropic::prompt::message::content::server_tool_use::{
///     AnthropicServerToolUseContent, ServerToolType
/// };
/// use serde_json::json;
///
/// // Create a web search tool use
/// let tool_use = AnthropicServerToolUseContent::new(
///     "server_toolu_123",
///     ServerToolType::WebSearch,
///     json!({"query": "Anthropic Claude"})
/// );
///
/// // Create a code execution tool use
/// let code_exec = AnthropicServerToolUseContent::new(
///     "server_toolu_456",
///     ServerToolType::BashCodeExecution,
///     json!({"code": "echo 'Hello, World!'"})
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicServerToolUseContent {
    /// The type of content (always "server_tool_use")
    #[serde(rename = "type")]
    pub content_type: String,

    /// Unique identifier for this server tool use
    pub id: String,

    /// Name of the server tool to use
    pub name: ServerToolType,

    /// Input/arguments for the server tool
    pub input: Value,

    /// Optional cache control configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}

impl AnthropicServerToolUseContent {
    /// Creates a new server tool use content block.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this server tool use
    /// * `name` - Name of the server tool to use
    /// * `input` - Input/arguments for the server tool
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::server_tool_use::{
    ///     AnthropicServerToolUseContent, ServerToolType
    /// };
    /// use serde_json::json;
    ///
    /// let tool_use = AnthropicServerToolUseContent::new(
    ///     "server_toolu_abc123",
    ///     ServerToolType::WebFetch,
    ///     json!({"url": "https://example.com"})
    /// );
    /// ```
    pub fn new(id: impl Into<String>, name: ServerToolType, input: Value) -> Self {
        Self {
            content_type: "server_tool_use".to_string(),
            id: id.into(),
            name,
            input,
            cache_control: None,
        }
    }

    /// Creates a new web fetch server tool use.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this server tool use
    /// * `input` - Input for the web fetch tool
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::server_tool_use::AnthropicServerToolUseContent;
    /// use serde_json::json;
    ///
    /// let tool_use = AnthropicServerToolUseContent::web_fetch(
    ///     "id",
    ///     json!({"url": "https://example.com"})
    /// );
    /// ```
    pub fn web_fetch(id: impl Into<String>, input: Value) -> Self {
        Self::new(id, ServerToolType::WebFetch, input)
    }

    /// Creates a new web search server tool use.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this server tool use
    /// * `input` - Input for the web search tool
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::server_tool_use::AnthropicServerToolUseContent;
    /// use serde_json::json;
    ///
    /// let tool_use = AnthropicServerToolUseContent::web_search(
    ///     "id",
    ///     json!({"query": "Rust programming"})
    /// );
    /// ```
    pub fn web_search(id: impl Into<String>, input: Value) -> Self {
        Self::new(id, ServerToolType::WebSearch, input)
    }

    /// Creates a new code execution server tool use.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this server tool use
    /// * `input` - Input for the code execution tool
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::server_tool_use::AnthropicServerToolUseContent;
    /// use serde_json::json;
    ///
    /// let tool_use = AnthropicServerToolUseContent::code_execution(
    ///     "id",
    ///     json!({"code": "print('Hello')"})
    /// );
    /// ```
    pub fn code_execution(id: impl Into<String>, input: Value) -> Self {
        Self::new(id, ServerToolType::CodeExecution, input)
    }

    /// Creates a new bash code execution server tool use.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this server tool use
    /// * `input` - Input for the bash code execution tool
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::server_tool_use::AnthropicServerToolUseContent;
    /// use serde_json::json;
    ///
    /// let tool_use = AnthropicServerToolUseContent::bash_code_execution(
    ///     "id",
    ///     json!({"code": "echo 'test'"})
    /// );
    /// ```
    pub fn bash_code_execution(id: impl Into<String>, input: Value) -> Self {
        Self::new(id, ServerToolType::BashCodeExecution, input)
    }

    /// Creates a new text editor code execution server tool use.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this server tool use
    /// * `input` - Input for the text editor code execution tool
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::server_tool_use::AnthropicServerToolUseContent;
    /// use serde_json::json;
    ///
    /// let tool_use = AnthropicServerToolUseContent::text_editor_code_execution(
    ///     "id",
    ///     json!({"code": "const x = 42;"})
    /// );
    /// ```
    pub fn text_editor_code_execution(id: impl Into<String>, input: Value) -> Self {
        Self::new(id, ServerToolType::TextEditorCodeExecution, input)
    }

    /// Updates the server tool use ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The new tool use ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Updates the server tool type.
    ///
    /// # Arguments
    ///
    /// * `name` - The new server tool type
    pub fn with_name(mut self, name: ServerToolType) -> Self {
        self.name = name;
        self
    }

    /// Updates the tool input.
    ///
    /// # Arguments
    ///
    /// * `input` - The new tool input
    pub fn with_input(mut self, input: Value) -> Self {
        self.input = input;
        self
    }

    /// Sets the cache control for this server tool use content block.
    ///
    /// # Arguments
    ///
    /// * `cache_control` - The cache control configuration
    pub fn with_cache_control(mut self, cache_control: AnthropicCacheControl) -> Self {
        self.cache_control = Some(cache_control);
        self
    }

    /// Removes cache control from this server tool use content block.
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
    fn test_server_tool_type_serialization() {
        assert_eq!(
            serde_json::to_value(ServerToolType::WebFetch).unwrap(),
            json!("web_fetch")
        );
        assert_eq!(
            serde_json::to_value(ServerToolType::WebSearch).unwrap(),
            json!("web_search")
        );
        assert_eq!(
            serde_json::to_value(ServerToolType::CodeExecution).unwrap(),
            json!("code_execution")
        );
        assert_eq!(
            serde_json::to_value(ServerToolType::BashCodeExecution).unwrap(),
            json!("bash_code_execution")
        );
        assert_eq!(
            serde_json::to_value(ServerToolType::TextEditorCodeExecution).unwrap(),
            json!("text_editor_code_execution")
        );
    }

    #[test]
    fn test_server_tool_type_deserialization() {
        assert_eq!(
            serde_json::from_value::<ServerToolType>(json!("web_fetch")).unwrap(),
            ServerToolType::WebFetch
        );
        assert_eq!(
            serde_json::from_value::<ServerToolType>(json!("web_search")).unwrap(),
            ServerToolType::WebSearch
        );
        assert_eq!(
            serde_json::from_value::<ServerToolType>(json!("code_execution")).unwrap(),
            ServerToolType::CodeExecution
        );
        assert_eq!(
            serde_json::from_value::<ServerToolType>(json!("bash_code_execution")).unwrap(),
            ServerToolType::BashCodeExecution
        );
        assert_eq!(
            serde_json::from_value::<ServerToolType>(json!("text_editor_code_execution")).unwrap(),
            ServerToolType::TextEditorCodeExecution
        );
    }

    #[test]
    fn test_new() {
        let tool_use = AnthropicServerToolUseContent::new(
            "server_toolu_123",
            ServerToolType::WebSearch,
            json!({"query": "test"}),
        );

        assert_eq!(tool_use.content_type, "server_tool_use");
        assert_eq!(tool_use.id, "server_toolu_123");
        assert_eq!(tool_use.name, ServerToolType::WebSearch);
        assert_eq!(tool_use.input, json!({"query": "test"}));
        assert!(tool_use.cache_control.is_none());
    }

    #[test]
    fn test_web_fetch() {
        let tool_use =
            AnthropicServerToolUseContent::web_fetch("id", json!({"url": "https://example.com"}));

        assert_eq!(tool_use.name, ServerToolType::WebFetch);
        assert_eq!(tool_use.input, json!({"url": "https://example.com"}));
    }

    #[test]
    fn test_web_search() {
        let tool_use = AnthropicServerToolUseContent::web_search("id", json!({"query": "rust"}));

        assert_eq!(tool_use.name, ServerToolType::WebSearch);
        assert_eq!(tool_use.input, json!({"query": "rust"}));
    }

    #[test]
    fn test_code_execution() {
        let tool_use =
            AnthropicServerToolUseContent::code_execution("id", json!({"code": "print('test')"}));

        assert_eq!(tool_use.name, ServerToolType::CodeExecution);
        assert_eq!(tool_use.input, json!({"code": "print('test')"}));
    }

    #[test]
    fn test_bash_code_execution() {
        let tool_use =
            AnthropicServerToolUseContent::bash_code_execution("id", json!({"code": "ls -la"}));

        assert_eq!(tool_use.name, ServerToolType::BashCodeExecution);
        assert_eq!(tool_use.input, json!({"code": "ls -la"}));
    }

    #[test]
    fn test_text_editor_code_execution() {
        let tool_use = AnthropicServerToolUseContent::text_editor_code_execution(
            "id",
            json!({"code": "const x = 1;"}),
        );

        assert_eq!(tool_use.name, ServerToolType::TextEditorCodeExecution);
        assert_eq!(tool_use.input, json!({"code": "const x = 1;"}));
    }

    #[test]
    fn test_with_id() {
        let tool_use =
            AnthropicServerToolUseContent::new("old_id", ServerToolType::WebFetch, json!({}))
                .with_id("new_id");

        assert_eq!(tool_use.id, "new_id");
    }

    #[test]
    fn test_with_name() {
        let tool_use =
            AnthropicServerToolUseContent::new("id", ServerToolType::WebFetch, json!({}))
                .with_name(ServerToolType::WebSearch);

        assert_eq!(tool_use.name, ServerToolType::WebSearch);
    }

    #[test]
    fn test_with_input() {
        let new_input = json!({"key": "value"});
        let tool_use =
            AnthropicServerToolUseContent::new("id", ServerToolType::WebFetch, json!({}))
                .with_input(new_input.clone());

        assert_eq!(tool_use.input, new_input);
    }

    #[test]
    fn test_with_cache_control() {
        let cache_control = AnthropicCacheControl::new(CacheTTL::FiveMinutes);
        let tool_use =
            AnthropicServerToolUseContent::new("id", ServerToolType::WebFetch, json!({}))
                .with_cache_control(cache_control.clone());

        assert_eq!(tool_use.cache_control, Some(cache_control));
    }

    #[test]
    fn test_without_cache_control() {
        let tool_use =
            AnthropicServerToolUseContent::new("id", ServerToolType::WebFetch, json!({}))
                .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour))
                .without_cache_control();

        assert!(tool_use.cache_control.is_none());
    }

    #[test]
    fn test_serialize_without_cache() {
        let tool_use = AnthropicServerToolUseContent::new(
            "server_toolu_abc",
            ServerToolType::WebSearch,
            json!({"query": "rust"}),
        );
        let json = serde_json::to_value(&tool_use).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "server_tool_use",
                "id": "server_toolu_abc",
                "name": "web_search",
                "input": {"query": "rust"}
            })
        );
    }

    #[test]
    fn test_serialize_with_cache() {
        let tool_use = AnthropicServerToolUseContent::new(
            "server_toolu_xyz",
            ServerToolType::CodeExecution,
            json!({"code": "x = 1"}),
        )
        .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&tool_use).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "server_tool_use",
                "id": "server_toolu_xyz",
                "name": "code_execution",
                "input": {"code": "x = 1"},
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
            "type": "server_tool_use",
            "id": "server_toolu_123",
            "name": "web_fetch",
            "input": {"url": "https://example.com"}
        });

        let tool_use: AnthropicServerToolUseContent = serde_json::from_value(json).unwrap();

        assert_eq!(tool_use.content_type, "server_tool_use");
        assert_eq!(tool_use.id, "server_toolu_123");
        assert_eq!(tool_use.name, ServerToolType::WebFetch);
        assert_eq!(tool_use.input, json!({"url": "https://example.com"}));
        assert!(tool_use.cache_control.is_none());
    }

    #[test]
    fn test_deserialize_with_cache() {
        let json = json!({
            "type": "server_tool_use",
            "id": "server_toolu_789",
            "name": "bash_code_execution",
            "input": {"code": "echo test"},
            "cache_control": {
                "type": "ephemeral",
                "ttl": "5m"
            }
        });

        let tool_use: AnthropicServerToolUseContent = serde_json::from_value(json).unwrap();

        assert_eq!(tool_use.name, ServerToolType::BashCodeExecution);
        assert_eq!(
            tool_use.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
        );
    }

    #[test]
    fn test_equality() {
        let tool1 = AnthropicServerToolUseContent::new("id", ServerToolType::WebFetch, json!({}));
        let tool2 = AnthropicServerToolUseContent::new("id", ServerToolType::WebFetch, json!({}));
        let tool3 = AnthropicServerToolUseContent::new("id", ServerToolType::WebSearch, json!({}));

        assert_eq!(tool1, tool2);
        assert_ne!(tool1, tool3);
    }

    #[test]
    fn test_clone() {
        let tool1 = AnthropicServerToolUseContent::new(
            "id",
            ServerToolType::CodeExecution,
            json!({"a": 1}),
        )
        .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let tool2 = tool1.clone();

        assert_eq!(tool1, tool2);
    }

    #[test]
    fn test_builder_chaining() {
        let tool_use =
            AnthropicServerToolUseContent::new("id1", ServerToolType::WebFetch, json!({}))
                .with_id("id2")
                .with_name(ServerToolType::WebSearch)
                .with_input(json!({"query": "test"}))
                .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
                .with_id("id3")
                .without_cache_control();

        assert_eq!(tool_use.id, "id3");
        assert_eq!(tool_use.name, ServerToolType::WebSearch);
        assert_eq!(tool_use.input, json!({"query": "test"}));
        assert!(tool_use.cache_control.is_none());
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original = AnthropicServerToolUseContent::new(
            "server_toolu_roundtrip",
            ServerToolType::TextEditorCodeExecution,
            json!({"code": "const x = 42;"}),
        )
        .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicServerToolUseContent = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_all_server_tool_types() {
        let types = vec![
            ServerToolType::WebFetch,
            ServerToolType::WebSearch,
            ServerToolType::CodeExecution,
            ServerToolType::BashCodeExecution,
            ServerToolType::TextEditorCodeExecution,
        ];

        for tool_type in types {
            let tool_use = AnthropicServerToolUseContent::new("id", tool_type, json!({}));
            assert_eq!(tool_use.name, tool_type);
        }
    }
}
