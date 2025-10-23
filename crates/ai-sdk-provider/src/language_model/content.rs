use serde::{Deserialize, Serialize};
use crate::language_model::file::File;
use crate::language_model::reasoning::Reasoning;
use crate::language_model::source::Source;
use crate::language_model::text::Text;
use crate::language_model::tool_call::ToolCall;
use crate::language_model::tool_result::ToolResult;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
    Text(Text),
    Reasoning(Reasoning),
    File(File),
    Source(Source),
    ToolCall(ToolCall),
    ToolResult(ToolResult),
}