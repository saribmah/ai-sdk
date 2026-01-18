use serde::{Deserialize, Serialize};

use crate::prompt::message::content::{
    bash_code_execution_tool_result::AnthropicBashCodeExecutionToolResultContent,
    code_execution_tool_result::AnthropicCodeExecutionToolResultContent,
    mcp_tool_result::AnthropicMcpToolResultContent, mcp_tool_use::AnthropicMcpToolUseContent,
    redacted_thinking::AnthropicRedactedThinkingContent,
    server_tool_use::AnthropicServerToolUseContent, text::AnthropicTextContent,
    text_editor_code_execution_tool_result::AnthropicTextEditorCodeExecutionToolResultContent,
    thinking::AnthropicThinkingContent, tool_call::AnthropicToolCallContent,
    web_fetch_tool_result::AnthropicWebFetchToolResultContent,
    web_search_tool_result::AnthropicWebSearchToolResultContent,
};

/// Content type for assistant messages.
///
/// Represents the different types of content that can appear in an assistant message,
/// including text, thinking (reasoning), tool calls, and tool results.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnthropicAssistantMessageContent {
    /// Plain text content
    Text(AnthropicTextContent),

    /// Thinking/reasoning content (extended thinking)
    Thinking(AnthropicThinkingContent),

    /// Redacted thinking content (for rate-limited thinking)
    RedactedThinking(AnthropicRedactedThinkingContent),

    /// Tool call content (for invoking tools)
    ToolCall(AnthropicToolCallContent),

    /// Server tool use content (for server-side tools)
    ServerToolUse(AnthropicServerToolUseContent),

    /// Code execution tool result (code_execution_20250522)
    CodeExecutionToolResult(AnthropicCodeExecutionToolResultContent),

    /// Web fetch tool result
    WebFetchToolResult(AnthropicWebFetchToolResultContent),

    /// Web search tool result
    WebSearchToolResult(AnthropicWebSearchToolResultContent),

    /// Bash code execution tool result (code_execution_20250825)
    BashCodeExecutionToolResult(AnthropicBashCodeExecutionToolResultContent),

    /// Text editor code execution tool result (code_execution_20250825)
    TextEditorCodeExecutionToolResult(AnthropicTextEditorCodeExecutionToolResultContent),

    /// MCP tool use content
    McpToolUse(AnthropicMcpToolUseContent),

    /// MCP tool result content
    McpToolResult(AnthropicMcpToolResultContent),
}

impl AnthropicAssistantMessageContent {
    /// Creates text content.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::assistant::AnthropicAssistantMessageContent;
    ///
    /// let content = AnthropicAssistantMessageContent::text("Hello, world!");
    /// ```
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text(AnthropicTextContent::new(text))
    }

    /// Creates thinking content.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::assistant::AnthropicAssistantMessageContent;
    ///
    /// let content = AnthropicAssistantMessageContent::thinking("Let me think about this...");
    /// ```
    pub fn thinking(thinking: impl Into<String>) -> Self {
        Self::Thinking(AnthropicThinkingContent::new(thinking, ""))
    }

    /// Creates a tool call.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::assistant::AnthropicAssistantMessageContent;
    /// use serde_json::json;
    ///
    /// let content = AnthropicAssistantMessageContent::tool_call(
    ///     "toolu_123",
    ///     "get_weather",
    ///     json!({"city": "San Francisco"})
    /// );
    /// ```
    pub fn tool_call(
        id: impl Into<String>,
        name: impl Into<String>,
        input: serde_json::Value,
    ) -> Self {
        Self::ToolCall(AnthropicToolCallContent::new(id, name, input))
    }
}

impl From<AnthropicTextContent> for AnthropicAssistantMessageContent {
    fn from(content: AnthropicTextContent) -> Self {
        Self::Text(content)
    }
}

impl From<AnthropicThinkingContent> for AnthropicAssistantMessageContent {
    fn from(content: AnthropicThinkingContent) -> Self {
        Self::Thinking(content)
    }
}

impl From<AnthropicRedactedThinkingContent> for AnthropicAssistantMessageContent {
    fn from(content: AnthropicRedactedThinkingContent) -> Self {
        Self::RedactedThinking(content)
    }
}

impl From<AnthropicToolCallContent> for AnthropicAssistantMessageContent {
    fn from(content: AnthropicToolCallContent) -> Self {
        Self::ToolCall(content)
    }
}

impl From<AnthropicServerToolUseContent> for AnthropicAssistantMessageContent {
    fn from(content: AnthropicServerToolUseContent) -> Self {
        Self::ServerToolUse(content)
    }
}

impl From<AnthropicCodeExecutionToolResultContent> for AnthropicAssistantMessageContent {
    fn from(content: AnthropicCodeExecutionToolResultContent) -> Self {
        Self::CodeExecutionToolResult(content)
    }
}

impl From<AnthropicWebFetchToolResultContent> for AnthropicAssistantMessageContent {
    fn from(content: AnthropicWebFetchToolResultContent) -> Self {
        Self::WebFetchToolResult(content)
    }
}

impl From<AnthropicWebSearchToolResultContent> for AnthropicAssistantMessageContent {
    fn from(content: AnthropicWebSearchToolResultContent) -> Self {
        Self::WebSearchToolResult(content)
    }
}

impl From<AnthropicBashCodeExecutionToolResultContent> for AnthropicAssistantMessageContent {
    fn from(content: AnthropicBashCodeExecutionToolResultContent) -> Self {
        Self::BashCodeExecutionToolResult(content)
    }
}

impl From<AnthropicTextEditorCodeExecutionToolResultContent> for AnthropicAssistantMessageContent {
    fn from(content: AnthropicTextEditorCodeExecutionToolResultContent) -> Self {
        Self::TextEditorCodeExecutionToolResult(content)
    }
}

impl From<AnthropicMcpToolUseContent> for AnthropicAssistantMessageContent {
    fn from(content: AnthropicMcpToolUseContent) -> Self {
        Self::McpToolUse(content)
    }
}

impl From<AnthropicMcpToolResultContent> for AnthropicAssistantMessageContent {
    fn from(content: AnthropicMcpToolResultContent) -> Self {
        Self::McpToolResult(content)
    }
}

/// Anthropic assistant message.
///
/// Represents a message from the AI assistant, which can contain text, thinking/reasoning,
/// tool calls, and tool results.
///
/// # Example
///
/// ```
/// use llm_kit_anthropic::prompt::message::assistant::{
///     AnthropicAssistantMessage, AnthropicAssistantMessageContent
/// };
///
/// // Simple text message
/// let message = AnthropicAssistantMessage::new(vec![
///     AnthropicAssistantMessageContent::text("Hello! How can I help you?")
/// ]);
///
/// // Message with thinking and text
/// let message_with_thinking = AnthropicAssistantMessage::new(vec![
///     AnthropicAssistantMessageContent::thinking("I need to analyze this request..."),
///     AnthropicAssistantMessageContent::text("Based on my analysis, here's my answer.")
/// ]);
///
/// // Message with tool call
/// use serde_json::json;
/// let message_with_tool = AnthropicAssistantMessage::new(vec![
///     AnthropicAssistantMessageContent::tool_call(
///         "toolu_123",
///         "get_weather",
///         json!({"city": "San Francisco"})
///     )
/// ]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicAssistantMessage {
    /// The role (always "assistant")
    pub role: String,

    /// The content of the message
    pub content: Vec<AnthropicAssistantMessageContent>,
}

impl AnthropicAssistantMessage {
    /// Creates a new assistant message.
    ///
    /// # Arguments
    ///
    /// * `content` - The content of the message
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::assistant::{
    ///     AnthropicAssistantMessage, AnthropicAssistantMessageContent
    /// };
    ///
    /// let message = AnthropicAssistantMessage::new(vec![
    ///     AnthropicAssistantMessageContent::text("Hello!")
    /// ]);
    /// ```
    pub fn new(content: Vec<AnthropicAssistantMessageContent>) -> Self {
        Self {
            role: "assistant".to_string(),
            content,
        }
    }

    /// Creates a simple text message.
    ///
    /// # Arguments
    ///
    /// * `text` - The text content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::assistant::AnthropicAssistantMessage;
    ///
    /// let message = AnthropicAssistantMessage::text("Hello, how can I help you?");
    /// ```
    pub fn text(text: impl Into<String>) -> Self {
        Self::new(vec![AnthropicAssistantMessageContent::text(text)])
    }

    /// Creates a message with thinking content.
    ///
    /// # Arguments
    ///
    /// * `thinking` - The thinking/reasoning content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::assistant::AnthropicAssistantMessage;
    ///
    /// let message = AnthropicAssistantMessage::thinking("Let me analyze this problem...");
    /// ```
    pub fn thinking(thinking: impl Into<String>) -> Self {
        Self::new(vec![AnthropicAssistantMessageContent::thinking(thinking)])
    }

    /// Creates a message with both thinking and text.
    ///
    /// # Arguments
    ///
    /// * `thinking` - The thinking/reasoning content
    /// * `text` - The text content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::assistant::AnthropicAssistantMessage;
    ///
    /// let message = AnthropicAssistantMessage::thinking_and_text(
    ///     "I need to consider multiple factors...",
    ///     "Based on my analysis, here's my recommendation."
    /// );
    /// ```
    pub fn thinking_and_text(thinking: impl Into<String>, text: impl Into<String>) -> Self {
        Self::new(vec![
            AnthropicAssistantMessageContent::thinking(thinking),
            AnthropicAssistantMessageContent::text(text),
        ])
    }

    /// Adds content to the message.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::assistant::{
    ///     AnthropicAssistantMessage, AnthropicAssistantMessageContent
    /// };
    ///
    /// let message = AnthropicAssistantMessage::text("First part")
    ///     .with_content(AnthropicAssistantMessageContent::text("Second part"));
    /// ```
    pub fn with_content(mut self, content: AnthropicAssistantMessageContent) -> Self {
        self.content.push(content);
        self
    }

    /// Adds text content to the message.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::assistant::AnthropicAssistantMessage;
    ///
    /// let message = AnthropicAssistantMessage::text("Hello!")
    ///     .with_text("How are you?");
    /// ```
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.content
            .push(AnthropicAssistantMessageContent::text(text));
        self
    }

    /// Adds thinking content to the message.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::assistant::AnthropicAssistantMessage;
    ///
    /// let message = AnthropicAssistantMessage::text("Answer")
    ///     .with_thinking("Let me reconsider...");
    /// ```
    pub fn with_thinking(mut self, thinking: impl Into<String>) -> Self {
        self.content
            .push(AnthropicAssistantMessageContent::thinking(thinking));
        self
    }

    /// Adds a tool call to the message.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::assistant::AnthropicAssistantMessage;
    /// use serde_json::json;
    ///
    /// let message = AnthropicAssistantMessage::text("Let me check the weather")
    ///     .with_tool_call("toolu_123", "get_weather", json!({"city": "NYC"}));
    /// ```
    pub fn with_tool_call(
        mut self,
        id: impl Into<String>,
        name: impl Into<String>,
        input: serde_json::Value,
    ) -> Self {
        self.content
            .push(AnthropicAssistantMessageContent::tool_call(id, name, input));
        self
    }

    /// Sets all content at once.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::assistant::{
    ///     AnthropicAssistantMessage, AnthropicAssistantMessageContent
    /// };
    ///
    /// let message = AnthropicAssistantMessage::text("Old content")
    ///     .with_all_content(vec![
    ///         AnthropicAssistantMessageContent::text("New content 1"),
    ///         AnthropicAssistantMessageContent::text("New content 2")
    ///     ]);
    /// ```
    pub fn with_all_content(mut self, content: Vec<AnthropicAssistantMessageContent>) -> Self {
        self.content = content;
        self
    }
}

impl From<String> for AnthropicAssistantMessage {
    /// Creates an assistant message from a string.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::assistant::AnthropicAssistantMessage;
    ///
    /// let message: AnthropicAssistantMessage = "Hello!".to_string().into();
    /// ```
    fn from(text: String) -> Self {
        Self::text(text)
    }
}

impl From<&str> for AnthropicAssistantMessage {
    /// Creates an assistant message from a string slice.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::assistant::AnthropicAssistantMessage;
    ///
    /// let message: AnthropicAssistantMessage = "Hello!".into();
    /// ```
    fn from(text: &str) -> Self {
        Self::text(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::content::text::AnthropicTextContent;
    use crate::prompt::message::content::thinking::AnthropicThinkingContent;
    use serde_json::json;

    #[test]
    fn test_assistant_message_new() {
        let message =
            AnthropicAssistantMessage::new(vec![AnthropicAssistantMessageContent::text("Hello")]);

        assert_eq!(message.role, "assistant");
        assert_eq!(message.content.len(), 1);
    }

    #[test]
    fn test_assistant_message_text() {
        let message = AnthropicAssistantMessage::text("Hello, world!");

        assert_eq!(message.role, "assistant");
        assert_eq!(message.content.len(), 1);
        match &message.content[0] {
            AnthropicAssistantMessageContent::Text(text) => {
                assert_eq!(text.text, "Hello, world!");
            }
            _ => panic!("Expected Text content"),
        }
    }

    #[test]
    fn test_assistant_message_thinking() {
        let message = AnthropicAssistantMessage::thinking("Let me think...");

        assert_eq!(message.content.len(), 1);
        match &message.content[0] {
            AnthropicAssistantMessageContent::Thinking(thinking) => {
                assert_eq!(thinking.thinking, "Let me think...");
            }
            _ => panic!("Expected Thinking content"),
        }
    }

    #[test]
    fn test_assistant_message_thinking_and_text() {
        let message = AnthropicAssistantMessage::thinking_and_text("Analysis...", "Conclusion");

        assert_eq!(message.content.len(), 2);
        match &message.content[0] {
            AnthropicAssistantMessageContent::Thinking(thinking) => {
                assert_eq!(thinking.thinking, "Analysis...");
            }
            _ => panic!("Expected Thinking content"),
        }
        match &message.content[1] {
            AnthropicAssistantMessageContent::Text(text) => {
                assert_eq!(text.text, "Conclusion");
            }
            _ => panic!("Expected Text content"),
        }
    }

    #[test]
    fn test_with_content() {
        let message = AnthropicAssistantMessage::text("First")
            .with_content(AnthropicAssistantMessageContent::text("Second"));

        assert_eq!(message.content.len(), 2);
    }

    #[test]
    fn test_with_text() {
        let message = AnthropicAssistantMessage::text("First").with_text("Second");

        assert_eq!(message.content.len(), 2);
    }

    #[test]
    fn test_with_thinking() {
        let message = AnthropicAssistantMessage::text("Answer").with_thinking("Reasoning");

        assert_eq!(message.content.len(), 2);
    }

    #[test]
    fn test_with_tool_call() {
        let message = AnthropicAssistantMessage::text("Let me check").with_tool_call(
            "toolu_123",
            "get_data",
            json!({"key": "value"}),
        );

        assert_eq!(message.content.len(), 2);
        match &message.content[1] {
            AnthropicAssistantMessageContent::ToolCall(tool_call) => {
                assert_eq!(tool_call.id, "toolu_123");
                assert_eq!(tool_call.name, "get_data");
            }
            _ => panic!("Expected ToolCall content"),
        }
    }

    #[test]
    fn test_with_all_content() {
        let message = AnthropicAssistantMessage::text("Old").with_all_content(vec![
            AnthropicAssistantMessageContent::text("New 1"),
            AnthropicAssistantMessageContent::text("New 2"),
        ]);

        assert_eq!(message.content.len(), 2);
    }

    #[test]
    fn test_from_string() {
        let message: AnthropicAssistantMessage = "Hello".to_string().into();

        assert_eq!(message.role, "assistant");
        assert_eq!(message.content.len(), 1);
    }

    #[test]
    fn test_from_str() {
        let message: AnthropicAssistantMessage = "Hello".into();

        assert_eq!(message.role, "assistant");
        assert_eq!(message.content.len(), 1);
    }

    #[test]
    fn test_content_from_text() {
        let text_content = AnthropicTextContent::new("Hello");
        let content: AnthropicAssistantMessageContent = text_content.into();

        match content {
            AnthropicAssistantMessageContent::Text(text) => {
                assert_eq!(text.text, "Hello");
            }
            _ => panic!("Expected Text content"),
        }
    }

    #[test]
    fn test_content_from_thinking() {
        let thinking_content = AnthropicThinkingContent::new("Thinking", "sig_123");
        let content: AnthropicAssistantMessageContent = thinking_content.into();

        match content {
            AnthropicAssistantMessageContent::Thinking(thinking) => {
                assert_eq!(thinking.thinking, "Thinking");
            }
            _ => panic!("Expected Thinking content"),
        }
    }

    #[test]
    fn test_serialize_simple_text() {
        let message = AnthropicAssistantMessage::text("Hello");
        let json = serde_json::to_value(&message).unwrap();

        assert_eq!(
            json,
            json!({
                "role": "assistant",
                "content": [
                    {
                        "type": "text",
                        "text": "Hello"
                    }
                ]
            })
        );
    }

    #[test]
    fn test_serialize_with_thinking() {
        let message = AnthropicAssistantMessage::thinking_and_text("Analysis", "Conclusion");
        let json = serde_json::to_value(&message).unwrap();

        let content_array = json["content"].as_array().unwrap();
        assert_eq!(content_array.len(), 2);
        assert_eq!(content_array[0]["type"], "thinking");
        assert_eq!(content_array[0]["thinking"], "Analysis");
        assert_eq!(content_array[1]["type"], "text");
        assert_eq!(content_array[1]["text"], "Conclusion");
    }

    #[test]
    fn test_serialize_with_tool_call() {
        let message =
            AnthropicAssistantMessage::new(vec![AnthropicAssistantMessageContent::tool_call(
                "toolu_123",
                "get_weather",
                json!({"city": "SF"}),
            )]);
        let json = serde_json::to_value(&message).unwrap();

        let content_array = json["content"].as_array().unwrap();
        assert_eq!(content_array.len(), 1);
        assert_eq!(content_array[0]["type"], "tool_use");
        assert_eq!(content_array[0]["id"], "toolu_123");
        assert_eq!(content_array[0]["name"], "get_weather");
    }

    #[test]
    fn test_deserialize_simple_text() {
        let json = json!({
            "role": "assistant",
            "content": [
                {
                    "type": "text",
                    "text": "Hello"
                }
            ]
        });

        let message: AnthropicAssistantMessage = serde_json::from_value(json).unwrap();

        assert_eq!(message.role, "assistant");
        assert_eq!(message.content.len(), 1);
        match &message.content[0] {
            AnthropicAssistantMessageContent::Text(text) => {
                assert_eq!(text.text, "Hello");
            }
            _ => panic!("Expected Text content"),
        }
    }

    #[test]
    fn test_deserialize_with_thinking() {
        let json = json!({
            "role": "assistant",
            "content": [
                {
                    "type": "thinking",
                    "thinking": "Analysis",
                    "signature": "sig_test123"
                },
                {
                    "type": "text",
                    "text": "Conclusion"
                }
            ]
        });

        let message: AnthropicAssistantMessage = serde_json::from_value(json).unwrap();

        assert_eq!(message.content.len(), 2);
    }

    #[test]
    fn test_equality() {
        let message1 = AnthropicAssistantMessage::text("Same");
        let message2 = AnthropicAssistantMessage::text("Same");
        let message3 = AnthropicAssistantMessage::text("Different");

        assert_eq!(message1, message2);
        assert_ne!(message1, message3);
    }

    #[test]
    fn test_clone() {
        let message1 = AnthropicAssistantMessage::thinking_and_text("Think", "Text");
        let message2 = message1.clone();

        assert_eq!(message1, message2);
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original = AnthropicAssistantMessage::new(vec![
            AnthropicAssistantMessageContent::thinking("Analyzing..."),
            AnthropicAssistantMessageContent::text("Here's my answer"),
            AnthropicAssistantMessageContent::tool_call(
                "toolu_123",
                "search",
                json!({"query": "test"}),
            ),
        ]);

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicAssistantMessage = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_builder_chaining() {
        let message = AnthropicAssistantMessage::text("First")
            .with_text("Second")
            .with_thinking("Thinking")
            .with_tool_call("id", "name", json!({}));

        assert_eq!(message.content.len(), 4);
    }
}
