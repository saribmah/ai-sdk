/// Error events in streaming responses
pub mod error;
/// Finish events in streaming responses
pub mod finish;
/// Raw chunk events from providers
pub mod raw;
/// Reasoning delta events (incremental reasoning content)
pub mod reasoning_delta;
/// Reasoning end events
pub mod reasoning_end;
/// Reasoning start events
pub mod reasoning_start;
/// Stream start events
pub mod stream_start;
/// Text delta events (incremental text content)
pub mod text_delta;
/// Text end events
pub mod text_end;
/// Text start events
pub mod text_start;
/// Tool input delta events (incremental tool arguments)
pub mod tool_input_delta;
/// Tool input end events
pub mod tool_input_end;
/// Tool input start events
pub mod tool_input_start;

use crate::language_model::call_warning::LanguageModelCallWarning;
use crate::language_model::content::file::LanguageModelFile;
use crate::language_model::content::source::LanguageModelSource;
use crate::language_model::content::tool_call::LanguageModelToolCall;
use crate::language_model::content::tool_result::LanguageModelToolResult;
use crate::language_model::finish_reason::LanguageModelFinishReason;
use crate::language_model::response_metadata::LanguageModelResponseMetadata;
use crate::language_model::usage::LanguageModelUsage;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Stream parts that can be emitted during model generation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LanguageModelStreamPart {
    // Text blocks
    /// Start of a text block
    TextStart(text_start::LanguageModelStreamTextStart),

    /// Text delta (incremental text content)
    TextDelta(text_delta::LanguageModelStreamTextDelta),

    /// End of a text block
    TextEnd(text_end::LanguageModelStreamTextEnd),

    // Reasoning blocks
    /// Start of a reasoning block
    ReasoningStart(reasoning_start::LanguageModelStreamReasoningStart),

    /// Reasoning delta (incremental reasoning content)
    ReasoningDelta(reasoning_delta::LanguageModelStreamReasoningDelta),

    /// End of a reasoning block
    ReasoningEnd(reasoning_end::LanguageModelStreamReasoningEnd),

    // Tool input blocks
    /// Start of tool input
    ToolInputStart(tool_input_start::LanguageModelStreamToolInputStart),

    /// Tool input delta (incremental tool arguments)
    ToolInputDelta(tool_input_delta::LanguageModelStreamToolInputDelta),

    /// End of tool input
    ToolInputEnd(tool_input_end::LanguageModelStreamToolInputEnd),

    // Complete tool call and result
    /// Complete tool call
    ToolCall(LanguageModelToolCall),

    /// Tool result
    ToolResult(LanguageModelToolResult),

    // Files and sources
    /// File content
    File(LanguageModelFile),

    /// Source content
    Source(LanguageModelSource),

    // Stream metadata events
    /// Stream start event with warnings
    StreamStart(stream_start::LanguageModelStreamStart),

    /// Response metadata
    ResponseMetadata(LanguageModelResponseMetadata),

    /// Stream finish event
    Finish(finish::LanguageModelStreamFinish),

    // Raw chunks and errors
    /// Raw chunk from provider (if enabled)
    Raw(raw::LanguageModelStreamRaw),

    /// Error event
    Error(error::LanguageModelStreamError),
}

// Helper implementations
impl LanguageModelStreamPart {
    /// Check if this is a text-related part
    pub fn is_text(&self) -> bool {
        matches!(
            self,
            Self::TextStart(_) | Self::TextDelta(_) | Self::TextEnd(_)
        )
    }

    /// Check if this is a reasoning-related part
    pub fn is_reasoning(&self) -> bool {
        matches!(
            self,
            Self::ReasoningStart(_) | Self::ReasoningDelta(_) | Self::ReasoningEnd(_)
        )
    }

    /// Check if this is a tool-related part
    pub fn is_tool(&self) -> bool {
        matches!(
            self,
            Self::ToolInputStart(_)
                | Self::ToolInputDelta(_)
                | Self::ToolInputEnd(_)
                | Self::ToolCall(_)
                | Self::ToolResult(_)
        )
    }

    /// Check if this is a delta (incremental content)
    pub fn is_delta(&self) -> bool {
        matches!(
            self,
            Self::TextDelta(_) | Self::ReasoningDelta(_) | Self::ToolInputDelta(_)
        )
    }

    /// Get the delta content if this is a delta part
    pub fn delta(&self) -> Option<&str> {
        match self {
            Self::TextDelta(d) => Some(&d.delta),
            Self::ReasoningDelta(d) => Some(&d.delta),
            Self::ToolInputDelta(d) => Some(&d.delta),
            _ => None,
        }
    }

    /// Check if this is a finish event
    pub fn is_finish(&self) -> bool {
        matches!(self, Self::Finish(_))
    }

    /// Check if this is an error event
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error(_))
    }

    /// Get the ID if this part has one
    pub fn id(&self) -> Option<&str> {
        match self {
            Self::TextStart(t) => Some(&t.id),
            Self::TextDelta(t) => Some(&t.id),
            Self::TextEnd(t) => Some(&t.id),
            Self::ReasoningStart(r) => Some(&r.id),
            Self::ReasoningDelta(r) => Some(&r.id),
            Self::ReasoningEnd(r) => Some(&r.id),
            Self::ToolInputStart(t) => Some(&t.id),
            Self::ToolInputDelta(t) => Some(&t.id),
            Self::ToolInputEnd(t) => Some(&t.id),
            _ => None,
        }
    }
}

// Constructors for common cases
impl LanguageModelStreamPart {
    /// Create a text start event
    pub fn text_start(id: impl Into<String>) -> Self {
        Self::TextStart(text_start::LanguageModelStreamTextStart::new(id))
    }

    /// Create a text delta event
    pub fn text_delta(id: impl Into<String>, delta: impl Into<String>) -> Self {
        Self::TextDelta(text_delta::LanguageModelStreamTextDelta::new(id, delta))
    }

    /// Create a text end event
    pub fn text_end(id: impl Into<String>) -> Self {
        Self::TextEnd(text_end::LanguageModelStreamTextEnd::new(id))
    }

    /// Create a reasoning start event
    pub fn reasoning_start(id: impl Into<String>) -> Self {
        Self::ReasoningStart(reasoning_start::LanguageModelStreamReasoningStart::new(id))
    }

    /// Create a reasoning delta event
    pub fn reasoning_delta(id: impl Into<String>, delta: impl Into<String>) -> Self {
        Self::ReasoningDelta(reasoning_delta::LanguageModelStreamReasoningDelta::new(
            id, delta,
        ))
    }

    /// Create a reasoning end event
    pub fn reasoning_end(id: impl Into<String>) -> Self {
        Self::ReasoningEnd(reasoning_end::LanguageModelStreamReasoningEnd::new(id))
    }

    /// Create a tool input start event
    pub fn tool_input_start(id: impl Into<String>, tool_name: impl Into<String>) -> Self {
        Self::ToolInputStart(tool_input_start::LanguageModelStreamToolInputStart::new(
            id, tool_name,
        ))
    }

    /// Create a tool input delta event
    pub fn tool_input_delta(id: impl Into<String>, delta: impl Into<String>) -> Self {
        Self::ToolInputDelta(tool_input_delta::LanguageModelStreamToolInputDelta::new(
            id, delta,
        ))
    }

    /// Create a tool input end event
    pub fn tool_input_end(id: impl Into<String>) -> Self {
        Self::ToolInputEnd(tool_input_end::LanguageModelStreamToolInputEnd::new(id))
    }

    /// Create a stream start event
    pub fn stream_start(warnings: Vec<LanguageModelCallWarning>) -> Self {
        Self::StreamStart(stream_start::LanguageModelStreamStart::new(warnings))
    }

    /// Create a finish event
    pub fn finish(usage: LanguageModelUsage, finish_reason: LanguageModelFinishReason) -> Self {
        Self::Finish(finish::LanguageModelStreamFinish::new(usage, finish_reason))
    }

    /// Create a raw chunk event
    pub fn raw(raw_value: Value) -> Self {
        Self::Raw(raw::LanguageModelStreamRaw::new(raw_value))
    }

    /// Create an error event
    pub fn error(error: Value) -> Self {
        Self::Error(error::LanguageModelStreamError::new(error))
    }
}
