pub mod reasoning;
pub mod source;
pub mod text;

pub use reasoning::ReasoningOutput;
pub use source::SourceOutput;
pub use text::TextOutput;

use crate::tool::{TypedToolCall, TypedToolError, TypedToolResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// An output part that can appear in an assistant message.
///
/// This represents all possible outputs that can be included in a response,
/// including text, reasoning, sources, tool calls, tool results, and tool errors.
///
/// # Type Parameters
///
/// * `INPUT` - The input type for tool calls, results, and errors
/// * `OUTPUT` - The output type for tool results
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Output<INPUT = Value, OUTPUT = Value> {
    /// A text content part with optional provider metadata.
    Text(TextOutput),

    /// A reasoning content part (chain of thought) with optional provider metadata.
    Reasoning(ReasoningOutput),

    /// A source content part with optional provider metadata.
    Source(SourceOutput),

    /// A tool call that was made.
    ToolCall(TypedToolCall<INPUT>),

    /// A tool result.
    ToolResult(TypedToolResult<INPUT, OUTPUT>),

    /// A tool error.
    ToolError(TypedToolError<INPUT>),
}

impl<INPUT, OUTPUT> Output<INPUT, OUTPUT> {
    /// Returns true if this is a text content part.
    pub fn is_text(&self) -> bool {
        matches!(self, Output::Text(_))
    }

    /// Returns true if this is a reasoning content part.
    pub fn is_reasoning(&self) -> bool {
        matches!(self, Output::Reasoning(_))
    }

    /// Returns true if this is a source content part.
    pub fn is_source(&self) -> bool {
        matches!(self, Output::Source(_))
    }

    /// Returns true if this is a tool call.
    pub fn is_tool_call(&self) -> bool {
        matches!(self, Output::ToolCall(_))
    }

    /// Returns true if this is a tool result.
    pub fn is_tool_result(&self) -> bool {
        matches!(self, Output::ToolResult(_))
    }

    /// Returns true if this is a tool error.
    pub fn is_tool_error(&self) -> bool {
        matches!(self, Output::ToolError(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool::{StaticToolCall, StaticToolError, StaticToolResult};
    use serde_json::json;

    #[test]
    fn test_content_part_text() {
        let text = TextOutput::new("Hello, world!");
        let part: Output<Value, Value> = Output::Text(text);

        assert!(part.is_text());
        assert!(!part.is_reasoning());
        assert!(!part.is_tool_call());
    }

    #[test]
    fn test_content_part_reasoning() {
        let reasoning = ReasoningOutput::new("Let me think...");
        let part: Output<Value, Value> = Output::Reasoning(reasoning);

        assert!(part.is_reasoning());
        assert!(!part.is_text());
        assert!(!part.is_tool_call());
    }

    #[test]
    fn test_content_part_tool_call() {
        let call = StaticToolCall::new("call_1", "tool", json!({}));
        let part: Output<Value, Value> = Output::ToolCall(TypedToolCall::Static(call));

        assert!(part.is_tool_call());
        assert!(!part.is_text());
        assert!(!part.is_tool_result());
    }

    #[test]
    fn test_content_part_tool_result() {
        let result = StaticToolResult::new("call_1", "tool", json!({}), json!({}));
        let part: Output<Value, Value> = Output::ToolResult(TypedToolResult::Static(result));

        assert!(part.is_tool_result());
        assert!(!part.is_tool_error());
        assert!(!part.is_tool_call());
    }

    #[test]
    fn test_content_part_tool_error() {
        let error = StaticToolError::new("call_1", "tool", json!({}), json!({"error": "failed"}));
        let part: Output<Value, Value> = Output::ToolError(TypedToolError::Static(error));

        assert!(part.is_tool_error());
        assert!(!part.is_tool_result());
        assert!(!part.is_tool_call());
    }
}
