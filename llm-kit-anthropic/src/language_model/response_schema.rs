use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Anthropic Messages API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicMessagesResponse {
    /// Always "message" for successful responses
    #[serde(rename = "type")]
    pub response_type: String,

    /// Unique identifier for this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Model that generated the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// Content blocks in the response
    pub content: Vec<ContentBlock>,

    /// Reason why the model stopped generating
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,

    /// The stop sequence that caused generation to stop
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequence: Option<String>,

    /// Token usage information
    pub usage: Usage,

    /// Container metadata (for extended thinking and MCP)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<Container>,
}

/// Content block in an Anthropic response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    /// Regular text content
    Text {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        citations: Option<Vec<Citation>>,
    },
    /// Extended thinking content (reasoning)
    Thinking { thinking: String, signature: String },
    /// Redacted thinking content
    RedactedThinking { data: String },
    /// Tool use request
    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        name: String,
        input: Value,
    },
    /// Server-side tool use (web_fetch, web_search, code_execution, etc.)
    #[serde(rename = "server_tool_use")]
    ServerToolUse {
        id: String,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        input: Option<Value>,
    },
    /// MCP (Model Context Protocol) tool use
    #[serde(rename = "mcp_tool_use")]
    McpToolUse {
        id: String,
        name: String,
        input: Value,
        server_name: String,
    },
    /// MCP tool result
    #[serde(rename = "mcp_tool_result")]
    McpToolResult {
        tool_use_id: String,
        is_error: bool,
        content: Vec<McpToolResultContent>,
    },
    /// Web fetch tool result
    #[serde(rename = "web_fetch_tool_result")]
    WebFetchToolResult {
        tool_use_id: String,
        content: WebFetchContent,
    },
    /// Web search tool result
    #[serde(rename = "web_search_tool_result")]
    WebSearchToolResult {
        tool_use_id: String,
        content: WebSearchContent,
    },
    /// Code execution tool result (code_execution_20250522)
    #[serde(rename = "code_execution_tool_result")]
    CodeExecutionToolResult {
        tool_use_id: String,
        content: CodeExecutionContent,
    },
    /// Bash code execution result (code_execution_20250825)
    #[serde(rename = "bash_code_execution_tool_result")]
    BashCodeExecutionToolResult {
        tool_use_id: String,
        content: BashCodeExecutionContent,
    },
    /// Text editor code execution result (code_execution_20250825)
    #[serde(rename = "text_editor_code_execution_tool_result")]
    TextEditorCodeExecutionToolResult {
        tool_use_id: String,
        content: TextEditorCodeExecutionContent,
    },
}

/// Citation in text content
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Citation {
    /// Web search result citation
    #[serde(rename = "web_search_result_location")]
    WebSearchResultLocation {
        cited_text: String,
        url: String,
        title: String,
        encrypted_index: String,
    },
    /// Page location citation (for document uploads)
    #[serde(rename = "page_location")]
    PageLocation {
        cited_text: String,
        document_index: u32,
        document_title: Option<String>,
        start_page_number: u32,
        end_page_number: u32,
    },
    /// Character location citation (for document uploads)
    #[serde(rename = "char_location")]
    CharLocation {
        cited_text: String,
        document_index: u32,
        document_title: Option<String>,
        start_char_index: u32,
        end_char_index: u32,
    },
}

/// MCP tool result content
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum McpToolResultContent {
    /// String content
    String(String),
    /// Structured text content
    Text { r#type: String, text: String },
}

/// Web fetch content
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebFetchContent {
    /// Successful fetch result
    Result {
        r#type: String, // "web_fetch_result"
        url: String,
        retrieved_at: String,
        content: WebFetchDocument,
    },
    /// Error result
    Error {
        r#type: String, // "web_fetch_tool_result_error"
        error_code: String,
    },
}

/// Web fetch document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebFetchDocument {
    pub r#type: String, // "document"
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citations: Option<CitationsConfig>,
    pub source: WebFetchSource,
}

/// Citations configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationsConfig {
    pub enabled: bool,
}

/// Web fetch source
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WebFetchSource {
    /// Text content
    Text { media_type: String, data: String },
}

/// Web search content
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebSearchContent {
    /// Array of search results
    Results(Vec<WebSearchResult>),
    /// Error result
    Error {
        r#type: String, // "web_search_tool_result_error"
        error_code: String,
    },
}

/// Web search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSearchResult {
    pub r#type: String, // "web_search_result"
    pub url: String,
    pub title: String,
    pub encrypted_content: String,
    pub page_age: Option<String>,
}

/// Code execution content (code_execution_20250522)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeExecutionContent {
    /// Successful execution result
    Result {
        r#type: String, // "code_execution_result"
        stdout: String,
        stderr: String,
        return_code: i32,
    },
    /// Error result
    Error {
        r#type: String, // "code_execution_tool_result_error"
        error_code: String,
    },
}

/// Bash code execution content (code_execution_20250825)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BashCodeExecutionContent {
    /// Successful bash execution result
    #[serde(rename = "bash_code_execution_result")]
    Result {
        content: Vec<BashCodeExecutionOutput>,
        stdout: String,
        stderr: String,
        return_code: i32,
    },
    /// Error result
    #[serde(rename = "bash_code_execution_tool_result_error")]
    Error { error_code: String },
}

/// Bash code execution output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BashCodeExecutionOutput {
    pub r#type: String, // "bash_code_execution_output"
    pub file_id: String,
}

/// Text editor code execution content (code_execution_20250825)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TextEditorCodeExecutionContent {
    /// Error result
    #[serde(rename = "text_editor_code_execution_tool_result_error")]
    Error { error_code: String },
    /// View file result
    #[serde(rename = "text_editor_code_execution_view_result")]
    ViewResult {
        content: String,
        file_type: String,
        num_lines: Option<u32>,
        start_line: Option<u32>,
        total_lines: Option<u32>,
    },
    /// Create file result
    #[serde(rename = "text_editor_code_execution_create_result")]
    CreateResult { is_file_update: bool },
    /// String replace result
    #[serde(rename = "text_editor_code_execution_str_replace_result")]
    StrReplaceResult {
        lines: Option<Vec<String>>,
        new_lines: Option<u32>,
        new_start: Option<u32>,
        old_lines: Option<u32>,
        old_start: Option<u32>,
    },
}

/// Token usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_creation_input_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_read_input_tokens: Option<u32>,
}

/// Container metadata for extended thinking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    pub expires_at: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<Skill>>,
}

/// Skill information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub r#type: SkillType,
    pub skill_id: String,
    pub version: String,
}

/// Skill type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SkillType {
    Anthropic,
    Custom,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_simple_text_response() {
        let json = r#"{
            "type": "message",
            "id": "msg_123",
            "model": "claude-3-5-sonnet-20241022",
            "content": [{
                "type": "text",
                "text": "Hello, world!"
            }],
            "stop_reason": "end_turn",
            "usage": {
                "input_tokens": 10,
                "output_tokens": 20
            }
        }"#;

        let response: AnthropicMessagesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.response_type, "message");
        assert_eq!(response.id, Some("msg_123".to_string()));
        assert_eq!(response.content.len(), 1);

        match &response.content[0] {
            ContentBlock::Text { text, citations } => {
                assert_eq!(text, "Hello, world!");
                assert!(citations.is_none());
            }
            _ => panic!("Expected Text content block"),
        }
    }

    #[test]
    fn test_deserialize_tool_use_response() {
        let json = r#"{
            "type": "message",
            "id": "msg_456",
            "model": "claude-3-5-sonnet-20241022",
            "content": [{
                "type": "tool_use",
                "id": "tool_123",
                "name": "get_weather",
                "input": {"city": "San Francisco"}
            }],
            "stop_reason": "tool_use",
            "usage": {
                "input_tokens": 15,
                "output_tokens": 25
            }
        }"#;

        let response: AnthropicMessagesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.stop_reason, Some("tool_use".to_string()));

        match &response.content[0] {
            ContentBlock::ToolUse { id, name, input } => {
                assert_eq!(id, "tool_123");
                assert_eq!(name, "get_weather");
                assert_eq!(input["city"], "San Francisco");
            }
            _ => panic!("Expected ToolUse content block"),
        }
    }

    #[test]
    fn test_deserialize_thinking_response() {
        let json = r#"{
            "type": "message",
            "content": [{
                "type": "thinking",
                "thinking": "Let me think about this...",
                "signature": "sig_abc123"
            }],
            "stop_reason": "end_turn",
            "usage": {
                "input_tokens": 10,
                "output_tokens": 30
            }
        }"#;

        let response: AnthropicMessagesResponse = serde_json::from_str(json).unwrap();

        match &response.content[0] {
            ContentBlock::Thinking {
                thinking,
                signature,
            } => {
                assert_eq!(thinking, "Let me think about this...");
                assert_eq!(signature, "sig_abc123");
            }
            _ => panic!("Expected Thinking content block"),
        }
    }

    #[test]
    fn test_deserialize_usage_with_cache() {
        let json = r#"{
            "type": "message",
            "content": [{"type": "text", "text": "Hi"}],
            "usage": {
                "input_tokens": 100,
                "output_tokens": 50,
                "cache_creation_input_tokens": 20,
                "cache_read_input_tokens": 10
            }
        }"#;

        let response: AnthropicMessagesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.usage.input_tokens, 100);
        assert_eq!(response.usage.output_tokens, 50);
        assert_eq!(response.usage.cache_creation_input_tokens, Some(20));
        assert_eq!(response.usage.cache_read_input_tokens, Some(10));
    }

    #[test]
    fn test_deserialize_web_search_result() {
        let json = r#"{
            "type": "message",
            "content": [{
                "type": "web_search_tool_result",
                "tool_use_id": "tool_789",
                "content": [{
                    "type": "web_search_result",
                    "url": "https://example.com",
                    "title": "Example",
                    "encrypted_content": "abc123",
                    "page_age": "1 day ago"
                }]
            }],
            "usage": {
                "input_tokens": 10,
                "output_tokens": 20
            }
        }"#;

        let response: AnthropicMessagesResponse = serde_json::from_str(json).unwrap();

        match &response.content[0] {
            ContentBlock::WebSearchToolResult {
                tool_use_id,
                content,
            } => {
                assert_eq!(tool_use_id, "tool_789");
                match content {
                    WebSearchContent::Results(results) => {
                        assert_eq!(results.len(), 1);
                        assert_eq!(results[0].url, "https://example.com");
                    }
                    _ => panic!("Expected search results"),
                }
            }
            _ => panic!("Expected WebSearchToolResult"),
        }
    }
}
