/// File content types for language model responses
pub mod file;
/// Reasoning content types for language model responses
pub mod reasoning;
/// Source content types for language model responses
pub mod source;
/// Text content types for language model responses
pub mod text;
/// Tool call content types for language model responses
pub mod tool_call;
/// Tool result content types for language model responses
pub mod tool_result;

use file::LanguageModelFile;
use reasoning::LanguageModelReasoning;
use serde::{Deserialize, Serialize};
use source::LanguageModelSource;
use text::LanguageModelText;
use tool_call::LanguageModelToolCall;
use tool_result::LanguageModelToolResult;

/// Content types that can appear in language model responses
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LanguageModelContent {
    /// Plain text content
    Text(LanguageModelText),
    /// Reasoning or thought process content
    Reasoning(LanguageModelReasoning),
    /// File or binary data content
    File(LanguageModelFile),
    /// Source or reference content
    Source(LanguageModelSource),
    /// Tool call request
    ToolCall(LanguageModelToolCall),
    /// Tool call result
    ToolResult(LanguageModelToolResult),
}
