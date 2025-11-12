use ai_sdk_provider::language_model::content::{
    LanguageModelContent, reasoning::LanguageModelReasoning, source::LanguageModelSource,
    text::LanguageModelText, tool_call::LanguageModelToolCall,
    tool_result::LanguageModelToolResult,
};
use serde_json::Value;

use super::response_schema::{
    BashCodeExecutionContent, Citation, CodeExecutionContent, ContentBlock, McpToolResultContent,
    TextEditorCodeExecutionContent, WebFetchContent, WebSearchContent,
};

/// Process a text content block
///
/// # Arguments
///
/// * `text` - The text content
/// * `citations` - Optional citations for the text
///
/// # Returns
///
/// A vector of `LanguageModelContent` items (text + citation sources)
pub fn process_text_block(
    text: String,
    citations: Option<Vec<Citation>>,
) -> Vec<LanguageModelContent> {
    let mut content = Vec::new();

    // Add text content
    content.push(LanguageModelContent::Text(LanguageModelText::new(text)));

    // Add citation sources
    if let Some(cites) = citations {
        content.extend(create_citation_sources(cites));
    }

    content
}

/// Process a thinking content block
///
/// # Arguments
///
/// * `thinking` - The thinking/reasoning content
/// * `signature` - The signature for verification
///
/// # Returns
///
/// A `LanguageModelContent::Reasoning` item
pub fn process_thinking_block(thinking: String, _signature: String) -> LanguageModelContent {
    // Note: signature is not currently used in the provider model
    // but we keep it as a parameter for future use
    LanguageModelContent::Reasoning(LanguageModelReasoning::init(thinking))
}

/// Process a tool use content block
///
/// # Arguments
///
/// * `id` - Tool call ID
/// * `name` - Tool name
/// * `input` - Tool input arguments
///
/// # Returns
///
/// A `LanguageModelContent::ToolCall` item
pub fn process_tool_use_block(id: String, name: String, input: Value) -> LanguageModelContent {
    let input_str = serde_json::to_string(&input).unwrap_or_else(|_| "{}".to_string());
    LanguageModelContent::ToolCall(LanguageModelToolCall::new(id, name, input_str))
}

/// Process a server tool use content block (web_fetch, web_search, code_execution)
///
/// These are server-executed tools, so we treat them as tool calls
///
/// # Arguments
///
/// * `id` - Tool call ID
/// * `name` - Tool name
/// * `input` - Optional tool input arguments
///
/// # Returns
///
/// A `LanguageModelContent::ToolCall` item
pub fn process_server_tool_use_block(
    id: String,
    name: String,
    input: Option<Value>,
) -> LanguageModelContent {
    let input_value = input.unwrap_or(Value::Object(serde_json::Map::new()));
    let input_str = serde_json::to_string(&input_value).unwrap_or_else(|_| "{}".to_string());
    LanguageModelContent::ToolCall(LanguageModelToolCall::new(id, name, input_str))
}

/// Process an MCP tool use content block
///
/// # Arguments
///
/// * `id` - Tool call ID
/// * `name` - Tool name
/// * `server_name` - MCP server name
/// * `input` - Tool input arguments
///
/// # Returns
///
/// A `LanguageModelContent::ToolCall` item with server_name in metadata
pub fn process_mcp_tool_use_block(
    id: String,
    name: String,
    server_name: String,
    input: Value,
) -> LanguageModelContent {
    // For MCP tools, we include the server_name in the tool name
    let full_name = format!("{}@{}", name, server_name);
    let input_str = serde_json::to_string(&input).unwrap_or_else(|_| "{}".to_string());
    LanguageModelContent::ToolCall(LanguageModelToolCall::new(id, full_name, input_str))
}

/// Process an MCP tool result content block
///
/// # Arguments
///
/// * `tool_use_id` - Tool call ID that generated this result
/// * `is_error` - Whether the result is an error
/// * `content` - The result content (string or structured)
///
/// # Returns
///
/// A `LanguageModelContent::ToolResult` item
pub fn process_mcp_tool_result_block(
    tool_use_id: String,
    is_error: bool,
    content: Vec<McpToolResultContent>,
) -> LanguageModelContent {
    // Convert MCP content to string
    let result_string = content
        .into_iter()
        .map(|c| match c {
            McpToolResultContent::String(s) => s,
            McpToolResultContent::Text { text, .. } => text,
        })
        .collect::<Vec<_>>()
        .join("\n");

    let result_value = if is_error {
        Value::String(format!("Error: {}", result_string))
    } else {
        Value::String(result_string)
    };

    let mut tool_result = LanguageModelToolResult::new(tool_use_id, "mcp_tool", result_value);
    tool_result.is_error = Some(is_error);
    LanguageModelContent::ToolResult(tool_result)
}

/// Process web fetch tool result
///
/// # Arguments
///
/// * `tool_use_id` - Tool call ID
/// * `content` - Web fetch result or error
///
/// # Returns
///
/// A `LanguageModelContent::ToolResult` item
pub fn process_web_fetch_tool_result(
    tool_use_id: String,
    content: WebFetchContent,
) -> LanguageModelContent {
    match content {
        WebFetchContent::Result {
            url,
            retrieved_at,
            content,
            ..
        } => {
            let result = serde_json::json!({
                "url": url,
                "retrieved_at": retrieved_at,
                "title": content.title,
                "data": match content.source {
                    super::response_schema::WebFetchSource::Text { data, .. } => data,
                }
            });
            LanguageModelContent::ToolResult(LanguageModelToolResult::new(
                tool_use_id,
                "web_fetch",
                result,
            ))
        }
        WebFetchContent::Error { error_code, .. } => {
            let error = serde_json::json!({
                "error": error_code
            });
            let mut tool_result = LanguageModelToolResult::new(tool_use_id, "web_fetch", error);
            tool_result.is_error = Some(true);
            LanguageModelContent::ToolResult(tool_result)
        }
    }
}

/// Process web search tool result
///
/// # Arguments
///
/// * `tool_use_id` - Tool call ID
/// * `content` - Web search results or error
///
/// # Returns
///
/// A `LanguageModelContent::ToolResult` item
pub fn process_web_search_tool_result(
    tool_use_id: String,
    content: WebSearchContent,
) -> LanguageModelContent {
    match content {
        WebSearchContent::Results(results) => {
            let result_json = serde_json::to_value(&results).unwrap_or(Value::Array(vec![]));
            LanguageModelContent::ToolResult(LanguageModelToolResult::new(
                tool_use_id,
                "web_search",
                result_json,
            ))
        }
        WebSearchContent::Error { error_code, .. } => {
            let error = serde_json::json!({
                "error": error_code
            });
            let mut tool_result = LanguageModelToolResult::new(tool_use_id, "web_search", error);
            tool_result.is_error = Some(true);
            LanguageModelContent::ToolResult(tool_result)
        }
    }
}

/// Process code execution tool result (code_execution_20250522)
///
/// # Arguments
///
/// * `tool_use_id` - Tool call ID
/// * `content` - Code execution result or error
///
/// # Returns
///
/// A `LanguageModelContent::ToolResult` item
pub fn process_code_execution_tool_result(
    tool_use_id: String,
    content: CodeExecutionContent,
) -> LanguageModelContent {
    match content {
        CodeExecutionContent::Result {
            stdout,
            stderr,
            return_code,
            ..
        } => {
            let result = serde_json::json!({
                "stdout": stdout,
                "stderr": stderr,
                "return_code": return_code
            });
            let mut tool_result =
                LanguageModelToolResult::new(tool_use_id, "code_execution", result);
            tool_result.is_error = Some(return_code != 0);
            LanguageModelContent::ToolResult(tool_result)
        }
        CodeExecutionContent::Error { error_code, .. } => {
            let error = serde_json::json!({
                "error": error_code
            });
            let mut tool_result =
                LanguageModelToolResult::new(tool_use_id, "code_execution", error);
            tool_result.is_error = Some(true);
            LanguageModelContent::ToolResult(tool_result)
        }
    }
}

/// Process bash code execution tool result (code_execution_20250825)
///
/// # Arguments
///
/// * `tool_use_id` - Tool call ID
/// * `content` - Bash execution result or error
///
/// # Returns
///
/// A `LanguageModelContent::ToolResult` item
pub fn process_bash_code_execution_tool_result(
    tool_use_id: String,
    content: BashCodeExecutionContent,
) -> LanguageModelContent {
    match content {
        BashCodeExecutionContent::Result {
            stdout,
            stderr,
            return_code,
            content,
        } => {
            let result = serde_json::json!({
                "stdout": stdout,
                "stderr": stderr,
                "return_code": return_code,
                "files": content
            });
            let mut tool_result =
                LanguageModelToolResult::new(tool_use_id, "bash_code_execution", result);
            tool_result.is_error = Some(return_code != 0);
            LanguageModelContent::ToolResult(tool_result)
        }
        BashCodeExecutionContent::Error { error_code } => {
            let error = serde_json::json!({
                "error": error_code
            });
            let mut tool_result =
                LanguageModelToolResult::new(tool_use_id, "bash_code_execution", error);
            tool_result.is_error = Some(true);
            LanguageModelContent::ToolResult(tool_result)
        }
    }
}

/// Process text editor code execution tool result (code_execution_20250825)
///
/// # Arguments
///
/// * `tool_use_id` - Tool call ID
/// * `content` - Text editor result or error
///
/// # Returns
///
/// A `LanguageModelContent::ToolResult` item
pub fn process_text_editor_code_execution_tool_result(
    tool_use_id: String,
    content: TextEditorCodeExecutionContent,
) -> LanguageModelContent {
    let (result, is_error) = match content {
        TextEditorCodeExecutionContent::Error { error_code } => {
            (serde_json::json!({ "error": error_code }), true)
        }
        TextEditorCodeExecutionContent::ViewResult {
            content,
            file_type,
            num_lines,
            start_line,
            total_lines,
        } => (
            serde_json::json!({
                "content": content,
                "file_type": file_type,
                "num_lines": num_lines,
                "start_line": start_line,
                "total_lines": total_lines
            }),
            false,
        ),
        TextEditorCodeExecutionContent::CreateResult { is_file_update } => (
            serde_json::json!({ "is_file_update": is_file_update }),
            false,
        ),
        TextEditorCodeExecutionContent::StrReplaceResult {
            lines,
            new_lines,
            new_start,
            old_lines,
            old_start,
        } => (
            serde_json::json!({
                "lines": lines,
                "new_lines": new_lines,
                "new_start": new_start,
                "old_lines": old_lines,
                "old_start": old_start
            }),
            false,
        ),
    };

    let mut tool_result =
        LanguageModelToolResult::new(tool_use_id, "text_editor_code_execution", result);
    tool_result.is_error = Some(is_error);
    LanguageModelContent::ToolResult(tool_result)
}

/// Create source citations from Anthropic citation data
///
/// # Arguments
///
/// * `citations` - Vector of Anthropic citations
///
/// # Returns
///
/// A vector of `LanguageModelContent::Source` items
pub fn create_citation_sources(citations: Vec<Citation>) -> Vec<LanguageModelContent> {
    citations
        .into_iter()
        .map(|citation| match citation {
            Citation::WebSearchResultLocation { url, title, .. } => {
                let id = uuid::Uuid::new_v4().to_string();
                let source = LanguageModelSource::Url {
                    id,
                    url: url.clone(),
                    title: Some(title),
                    provider_metadata: None,
                };
                LanguageModelContent::Source(source)
            }
            Citation::PageLocation {
                document_title,
                start_page_number,
                end_page_number,
                ..
            } => {
                let id = uuid::Uuid::new_v4().to_string();
                let title = document_title.unwrap_or_else(|| "Document".to_string());
                let media_type = "application/pdf".to_string();
                let filename = Some(format!("pages-{}-{}", start_page_number, end_page_number));
                let source = LanguageModelSource::Document {
                    id,
                    media_type,
                    title,
                    filename,
                    provider_metadata: None,
                };
                LanguageModelContent::Source(source)
            }
            Citation::CharLocation {
                document_title,
                start_char_index,
                end_char_index,
                ..
            } => {
                let id = uuid::Uuid::new_v4().to_string();
                let title = document_title.unwrap_or_else(|| "Document".to_string());
                let media_type = "text/plain".to_string();
                let filename = Some(format!("chars-{}-{}", start_char_index, end_char_index));
                let source = LanguageModelSource::Document {
                    id,
                    media_type,
                    title,
                    filename,
                    provider_metadata: None,
                };
                LanguageModelContent::Source(source)
            }
        })
        .collect()
}

/// Process all content blocks from an Anthropic response
///
/// # Arguments
///
/// * `content_blocks` - Vector of content blocks from response
///
/// # Returns
///
/// A vector of `LanguageModelContent` items
pub fn process_content_blocks(content_blocks: Vec<ContentBlock>) -> Vec<LanguageModelContent> {
    let mut content = Vec::new();

    for block in content_blocks {
        match block {
            ContentBlock::Text { text, citations } => {
                content.extend(process_text_block(text, citations));
            }
            ContentBlock::Thinking {
                thinking,
                signature,
            } => {
                content.push(process_thinking_block(thinking, signature));
            }
            ContentBlock::RedactedThinking { data } => {
                // Treat redacted thinking as regular reasoning
                content.push(LanguageModelContent::Reasoning(
                    LanguageModelReasoning::init(format!("[Redacted: {}]", data)),
                ));
            }
            ContentBlock::ToolUse { id, name, input } => {
                content.push(process_tool_use_block(id, name, input));
            }
            ContentBlock::ServerToolUse { id, name, input } => {
                content.push(process_server_tool_use_block(id, name, input));
            }
            ContentBlock::McpToolUse {
                id,
                name,
                server_name,
                input,
            } => {
                content.push(process_mcp_tool_use_block(id, name, server_name, input));
            }
            ContentBlock::McpToolResult {
                tool_use_id,
                is_error,
                content: mcp_content,
            } => {
                content.push(process_mcp_tool_result_block(
                    tool_use_id,
                    is_error,
                    mcp_content,
                ));
            }
            ContentBlock::WebFetchToolResult {
                tool_use_id,
                content: web_content,
            } => {
                content.push(process_web_fetch_tool_result(tool_use_id, web_content));
            }
            ContentBlock::WebSearchToolResult {
                tool_use_id,
                content: search_content,
            } => {
                content.push(process_web_search_tool_result(tool_use_id, search_content));
            }
            ContentBlock::CodeExecutionToolResult {
                tool_use_id,
                content: exec_content,
            } => {
                content.push(process_code_execution_tool_result(
                    tool_use_id,
                    exec_content,
                ));
            }
            ContentBlock::BashCodeExecutionToolResult {
                tool_use_id,
                content: bash_content,
            } => {
                content.push(process_bash_code_execution_tool_result(
                    tool_use_id,
                    bash_content,
                ));
            }
            ContentBlock::TextEditorCodeExecutionToolResult {
                tool_use_id,
                content: editor_content,
            } => {
                content.push(process_text_editor_code_execution_tool_result(
                    tool_use_id,
                    editor_content,
                ));
            }
        }
    }

    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_text_block() {
        let content = process_text_block("Hello world".to_string(), None);
        assert_eq!(content.len(), 1);
        match &content[0] {
            LanguageModelContent::Text(text) => assert_eq!(text.text, "Hello world"),
            _ => panic!("Expected text content"),
        }
    }

    #[test]
    fn test_process_thinking_block() {
        let content = process_thinking_block("Let me think...".to_string(), "sig123".to_string());
        match content {
            LanguageModelContent::Reasoning(reasoning) => {
                assert_eq!(reasoning.text, "Let me think...");
            }
            _ => panic!("Expected reasoning content"),
        }
    }

    #[test]
    fn test_process_tool_use_block() {
        let content = process_tool_use_block(
            "tool_123".to_string(),
            "get_weather".to_string(),
            serde_json::json!({"city": "SF"}),
        );
        match content {
            LanguageModelContent::ToolCall(tool_call) => {
                assert_eq!(tool_call.tool_call_id, "tool_123");
                assert_eq!(tool_call.tool_name, "get_weather");
            }
            _ => panic!("Expected tool call"),
        }
    }

    #[test]
    fn test_create_citation_sources() {
        let citations = vec![Citation::WebSearchResultLocation {
            cited_text: "Some text".to_string(),
            url: "https://example.com".to_string(),
            title: "Example".to_string(),
            encrypted_index: "abc123".to_string(),
        }];

        let sources = create_citation_sources(citations);
        assert_eq!(sources.len(), 1);
        match &sources[0] {
            LanguageModelContent::Source(LanguageModelSource::Url { url, title, .. }) => {
                assert_eq!(url, "https://example.com");
                assert_eq!(title.as_ref().unwrap(), "Example");
            }
            _ => panic!("Expected source content"),
        }
    }
}
