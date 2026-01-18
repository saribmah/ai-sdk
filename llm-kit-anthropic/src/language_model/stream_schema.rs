use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::response_schema::{
    BashCodeExecutionContent, CodeExecutionContent, McpToolResultContent,
    TextEditorCodeExecutionContent, WebFetchContent, WebSearchContent,
};

/// Anthropic streaming chunk event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AnthropicChunk {
    /// Ping event (keep-alive)
    Ping,

    /// Message start event
    MessageStart { message: MessageStartData },

    /// Content block start event
    ContentBlockStart {
        index: u32,
        content_block: ContentBlockStart,
    },

    /// Content block delta event
    ContentBlockDelta {
        index: u32,
        delta: ContentBlockDelta,
    },

    /// Content block stop event
    ContentBlockStop { index: u32 },

    /// Message delta event
    MessageDelta {
        delta: MessageDelta,
        usage: DeltaUsage,
    },

    /// Message stop event
    MessageStop,

    /// Error event
    Error { error: ErrorData },
}

/// Message start data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageStartData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub usage: StartUsage,
}

/// Usage at message start
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartUsage {
    pub input_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_creation_input_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_read_input_tokens: Option<u32>,
}

/// Content block start data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlockStart {
    /// Text block start
    Text { text: String },

    /// Thinking block start
    Thinking { thinking: String },

    /// Redacted thinking block start
    RedactedThinking { data: String },

    /// Tool use block start
    #[serde(rename = "tool_use")]
    ToolUse { id: String, name: String },

    /// Server tool use block start
    #[serde(rename = "server_tool_use")]
    ServerToolUse {
        id: String,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        input: Option<Value>,
    },

    /// MCP tool use block start
    #[serde(rename = "mcp_tool_use")]
    McpToolUse {
        id: String,
        name: String,
        input: Value,
        server_name: String,
    },

    /// MCP tool result block start
    #[serde(rename = "mcp_tool_result")]
    McpToolResult {
        tool_use_id: String,
        is_error: bool,
        content: Vec<McpToolResultContent>,
    },

    /// Web fetch tool result block start
    #[serde(rename = "web_fetch_tool_result")]
    WebFetchToolResult {
        tool_use_id: String,
        content: WebFetchContent,
    },

    /// Web search tool result block start
    #[serde(rename = "web_search_tool_result")]
    WebSearchToolResult {
        tool_use_id: String,
        content: WebSearchContent,
    },

    /// Code execution tool result block start
    #[serde(rename = "code_execution_tool_result")]
    CodeExecutionToolResult {
        tool_use_id: String,
        content: CodeExecutionContent,
    },

    /// Bash code execution result block start
    #[serde(rename = "bash_code_execution_tool_result")]
    BashCodeExecutionToolResult {
        tool_use_id: String,
        content: BashCodeExecutionContent,
    },

    /// Text editor code execution result block start
    #[serde(rename = "text_editor_code_execution_tool_result")]
    TextEditorCodeExecutionToolResult {
        tool_use_id: String,
        content: TextEditorCodeExecutionContent,
    },
}

/// Content block delta data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlockDelta {
    /// Text delta
    TextDelta { text: String },

    /// Thinking delta
    ThinkingDelta { thinking: String },

    /// Input JSON delta (for tool use)
    InputJsonDelta { partial_json: String },
}

/// Message delta data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageDelta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequence: Option<String>,
}

/// Usage delta (output tokens)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeltaUsage {
    pub output_tokens: u32,
}

/// Error data in stream
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorData {
    pub r#type: String,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_ping() {
        let json = r#"{"type": "ping"}"#;
        let chunk: AnthropicChunk = serde_json::from_str(json).unwrap();
        assert!(matches!(chunk, AnthropicChunk::Ping));
    }

    #[test]
    fn test_deserialize_message_start() {
        let json = r#"{
            "type": "message_start",
            "message": {
                "id": "msg_123",
                "model": "claude-3-5-sonnet-20241022",
                "usage": {
                    "input_tokens": 100,
                    "cache_creation_input_tokens": 20,
                    "cache_read_input_tokens": 10
                }
            }
        }"#;

        let chunk: AnthropicChunk = serde_json::from_str(json).unwrap();
        match chunk {
            AnthropicChunk::MessageStart { message } => {
                assert_eq!(message.id, Some("msg_123".to_string()));
                assert_eq!(message.usage.input_tokens, 100);
                assert_eq!(message.usage.cache_creation_input_tokens, Some(20));
            }
            _ => panic!("Expected MessageStart"),
        }
    }

    #[test]
    fn test_deserialize_content_block_start_text() {
        let json = r#"{
            "type": "content_block_start",
            "index": 0,
            "content_block": {
                "type": "text",
                "text": ""
            }
        }"#;

        let chunk: AnthropicChunk = serde_json::from_str(json).unwrap();
        match chunk {
            AnthropicChunk::ContentBlockStart {
                index,
                content_block,
            } => {
                assert_eq!(index, 0);
                match content_block {
                    ContentBlockStart::Text { text } => assert_eq!(text, ""),
                    _ => panic!("Expected Text content block"),
                }
            }
            _ => panic!("Expected ContentBlockStart"),
        }
    }

    #[test]
    fn test_deserialize_content_block_start_tool_use() {
        let json = r#"{
            "type": "content_block_start",
            "index": 1,
            "content_block": {
                "type": "tool_use",
                "id": "tool_abc",
                "name": "get_weather"
            }
        }"#;

        let chunk: AnthropicChunk = serde_json::from_str(json).unwrap();
        match chunk {
            AnthropicChunk::ContentBlockStart {
                index,
                content_block,
            } => {
                assert_eq!(index, 1);
                match content_block {
                    ContentBlockStart::ToolUse { id, name } => {
                        assert_eq!(id, "tool_abc");
                        assert_eq!(name, "get_weather");
                    }
                    _ => panic!("Expected ToolUse content block"),
                }
            }
            _ => panic!("Expected ContentBlockStart"),
        }
    }

    #[test]
    fn test_deserialize_content_block_delta_text() {
        let json = r#"{
            "type": "content_block_delta",
            "index": 0,
            "delta": {
                "type": "text_delta",
                "text": "Hello"
            }
        }"#;

        let chunk: AnthropicChunk = serde_json::from_str(json).unwrap();
        match chunk {
            AnthropicChunk::ContentBlockDelta { index, delta } => {
                assert_eq!(index, 0);
                match delta {
                    ContentBlockDelta::TextDelta { text } => assert_eq!(text, "Hello"),
                    _ => panic!("Expected TextDelta"),
                }
            }
            _ => panic!("Expected ContentBlockDelta"),
        }
    }

    #[test]
    fn test_deserialize_content_block_delta_thinking() {
        let json = r#"{
            "type": "content_block_delta",
            "index": 0,
            "delta": {
                "type": "thinking_delta",
                "thinking": "Let me consider..."
            }
        }"#;

        let chunk: AnthropicChunk = serde_json::from_str(json).unwrap();
        match chunk {
            AnthropicChunk::ContentBlockDelta { index, delta } => {
                assert_eq!(index, 0);
                match delta {
                    ContentBlockDelta::ThinkingDelta { thinking } => {
                        assert_eq!(thinking, "Let me consider...")
                    }
                    _ => panic!("Expected ThinkingDelta"),
                }
            }
            _ => panic!("Expected ContentBlockDelta"),
        }
    }

    #[test]
    fn test_deserialize_content_block_delta_input_json() {
        let json = r#"{
            "type": "content_block_delta",
            "index": 1,
            "delta": {
                "type": "input_json_delta",
                "partial_json": "{\"city\":"
            }
        }"#;

        let chunk: AnthropicChunk = serde_json::from_str(json).unwrap();
        match chunk {
            AnthropicChunk::ContentBlockDelta { index, delta } => {
                assert_eq!(index, 1);
                match delta {
                    ContentBlockDelta::InputJsonDelta { partial_json } => {
                        assert_eq!(partial_json, "{\"city\":")
                    }
                    _ => panic!("Expected InputJsonDelta"),
                }
            }
            _ => panic!("Expected ContentBlockDelta"),
        }
    }

    #[test]
    fn test_deserialize_content_block_stop() {
        let json = r#"{"type": "content_block_stop", "index": 0}"#;
        let chunk: AnthropicChunk = serde_json::from_str(json).unwrap();
        match chunk {
            AnthropicChunk::ContentBlockStop { index } => assert_eq!(index, 0),
            _ => panic!("Expected ContentBlockStop"),
        }
    }

    #[test]
    fn test_deserialize_message_delta() {
        let json = r#"{
            "type": "message_delta",
            "delta": {
                "stop_reason": "end_turn"
            },
            "usage": {
                "output_tokens": 50
            }
        }"#;

        let chunk: AnthropicChunk = serde_json::from_str(json).unwrap();
        match chunk {
            AnthropicChunk::MessageDelta { delta, usage } => {
                assert_eq!(delta.stop_reason, Some("end_turn".to_string()));
                assert_eq!(usage.output_tokens, 50);
            }
            _ => panic!("Expected MessageDelta"),
        }
    }

    #[test]
    fn test_deserialize_message_stop() {
        let json = r#"{"type": "message_stop"}"#;
        let chunk: AnthropicChunk = serde_json::from_str(json).unwrap();
        assert!(matches!(chunk, AnthropicChunk::MessageStop));
    }

    #[test]
    fn test_deserialize_error() {
        let json = r#"{
            "type": "error",
            "error": {
                "type": "rate_limit_error",
                "message": "Rate limit exceeded"
            }
        }"#;

        let chunk: AnthropicChunk = serde_json::from_str(json).unwrap();
        match chunk {
            AnthropicChunk::Error { error } => {
                assert_eq!(error.r#type, "rate_limit_error");
                assert_eq!(error.message, "Rate limit exceeded");
            }
            _ => panic!("Expected Error"),
        }
    }
}
