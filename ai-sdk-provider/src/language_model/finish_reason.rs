use serde::{Deserialize, Serialize};

/// Reason why a language model generation finished
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LanguageModelFinishReason {
    /// Generation completed naturally
    Stop,
    /// Generation stopped due to length limit
    Length,
    /// Generation stopped due to content filter
    ContentFilter,
    /// Generation stopped to execute tool calls
    ToolCalls,
    /// Generation stopped due to an error
    Error,
    /// Other finish reason
    Other,
    /// Unknown finish reason
    Unknown,
}
