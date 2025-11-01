pub mod text;
pub mod reasoning;
pub mod file;
pub mod source;
pub mod tool_call;
pub mod tool_result;

use file::File;
use reasoning::Reasoning;
use source::Source;
use text::Text;
use tool_call::ToolCall;
use tool_result::ToolResult;
use serde::{Deserialize, Serialize};

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
