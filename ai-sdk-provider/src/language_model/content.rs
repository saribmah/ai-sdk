pub mod file;
pub mod reasoning;
pub mod source;
pub mod text;
pub mod tool_call;
pub mod tool_result;

use file::LanguageModelFile;
use reasoning::LanguageModelReasoning;
use serde::{Deserialize, Serialize};
use source::LanguageModelSource;
use text::LanguageModelText;
use tool_call::LanguageModelToolCall;
use tool_result::LanguageModelToolResult;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
    Text(LanguageModelText),
    Reasoning(LanguageModelReasoning),
    File(LanguageModelFile),
    Source(LanguageModelSource),
    ToolCall(LanguageModelToolCall),
    ToolResult(LanguageModelToolResult),
}
