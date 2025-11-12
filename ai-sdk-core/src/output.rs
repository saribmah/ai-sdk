/// Reasoning output type for chain-of-thought content.
pub mod reasoning;
/// Source output type for references and citations.
pub mod source;
/// Text output type for plain text content.
pub mod text;

pub use reasoning::ReasoningOutput;
pub use source::SourceOutput;
pub use text::TextOutput;

use crate::generate_text::GeneratedFile;
use ai_sdk_provider_utils::tool::{ToolCall, ToolError, ToolResult};
use serde::{Deserialize, Serialize};

/// An output part that can appear in an assistant message.
///
/// This represents all possible outputs that can be included in a response,
/// including text, reasoning, sources, files, tool calls, tool results, and tool errors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Output {
    /// A text content part with optional provider metadata.
    Text(TextOutput),

    /// A reasoning content part (chain of thought) with optional provider metadata.
    Reasoning(ReasoningOutput),

    /// A source content part with optional provider metadata.
    Source(SourceOutput),

    /// A file content part.
    File(GeneratedFile),

    /// A tool call that was made.
    ToolCall(ToolCall),

    /// A tool result.
    ToolResult(ToolResult),

    /// A tool error.
    ToolError(ToolError),
}

impl Output {
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

    /// Returns true if this is a file content part.
    pub fn is_file(&self) -> bool {
        matches!(self, Output::File(_))
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
    use ai_sdk_provider_utils::tool::{ToolCall, ToolError, ToolResult};
    use serde_json::json;

    #[test]
    fn test_output_text() {
        let text = TextOutput::new("Hello, world!");
        let part = Output::Text(text);

        assert!(part.is_text());
        assert!(!part.is_reasoning());
        assert!(!part.is_tool_call());
    }

    #[test]
    fn test_output_reasoning() {
        let reasoning = ReasoningOutput::new("Let me think...");
        let part = Output::Reasoning(reasoning);

        assert!(part.is_reasoning());
        assert!(!part.is_text());
        assert!(!part.is_tool_call());
    }

    #[test]
    fn test_output_tool_call() {
        let call = ToolCall::new("call_1", "tool", json!({}));
        let part = Output::ToolCall(call);

        assert!(part.is_tool_call());
        assert!(!part.is_text());
        assert!(!part.is_tool_result());
    }

    #[test]
    fn test_output_tool_result() {
        let result = ToolResult::new("call_1", "tool", json!({}), json!({}));
        let part = Output::ToolResult(result);

        assert!(part.is_tool_result());
        assert!(!part.is_tool_error());
        assert!(!part.is_tool_call());
    }

    #[test]
    fn test_output_tool_error() {
        let error = ToolError::new("call_1", "tool", json!({}), json!({"error": "failed"}));
        let part = Output::ToolError(error);

        assert!(part.is_tool_error());
        assert!(!part.is_tool_result());
        assert!(!part.is_tool_call());
    }

    #[test]
    fn test_output_file() {
        use crate::generate_text::GeneratedFile;

        let file = GeneratedFile::from_base64("SGVsbG8gV29ybGQh", "text/plain");
        let part = Output::File(file);

        assert!(part.is_file());
        assert!(!part.is_text());
        assert!(!part.is_reasoning());
        assert!(!part.is_tool_call());
    }
}
