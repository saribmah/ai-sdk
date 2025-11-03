pub mod error;
pub mod finish;
pub mod raw;
pub mod reasoning_delta;
pub mod reasoning_end;
pub mod reasoning_start;
pub mod stream_start;
pub mod text_delta;
pub mod text_end;
pub mod text_start;
pub mod tool_input_delta;
pub mod tool_input_end;
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
pub enum StreamPart {
    // Text blocks
    /// Start of a text block
    TextStart(text_start::TextStart),

    /// Text delta (incremental text content)
    TextDelta(text_delta::TextDelta),

    /// End of a text block
    TextEnd(text_end::TextEnd),

    // Reasoning blocks
    /// Start of a reasoning block
    ReasoningStart(reasoning_start::ReasoningStart),

    /// Reasoning delta (incremental reasoning content)
    ReasoningDelta(reasoning_delta::ReasoningDelta),

    /// End of a reasoning block
    ReasoningEnd(reasoning_end::ReasoningEnd),

    // Tool input blocks
    /// Start of tool input
    ToolInputStart(tool_input_start::ToolInputStart),

    /// Tool input delta (incremental tool arguments)
    ToolInputDelta(tool_input_delta::ToolInputDelta),

    /// End of tool input
    ToolInputEnd(tool_input_end::ToolInputEnd),

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
    StreamStart(stream_start::StreamStart),

    /// Response metadata
    ResponseMetadata(LanguageModelResponseMetadata),

    /// Stream finish event
    Finish(finish::Finish),

    // Raw chunks and errors
    /// Raw chunk from provider (if enabled)
    Raw(raw::Raw),

    /// Error event
    Error(error::Error),
}

// Helper implementations
impl StreamPart {
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
impl StreamPart {
    /// Create a text start event
    pub fn text_start(id: impl Into<String>) -> Self {
        Self::TextStart(text_start::TextStart::new(id))
    }

    /// Create a text delta event
    pub fn text_delta(id: impl Into<String>, delta: impl Into<String>) -> Self {
        Self::TextDelta(text_delta::TextDelta::new(id, delta))
    }

    /// Create a text end event
    pub fn text_end(id: impl Into<String>) -> Self {
        Self::TextEnd(text_end::TextEnd::new(id))
    }

    /// Create a reasoning start event
    pub fn reasoning_start(id: impl Into<String>) -> Self {
        Self::ReasoningStart(reasoning_start::ReasoningStart::new(id))
    }

    /// Create a reasoning delta event
    pub fn reasoning_delta(id: impl Into<String>, delta: impl Into<String>) -> Self {
        Self::ReasoningDelta(reasoning_delta::ReasoningDelta::new(id, delta))
    }

    /// Create a reasoning end event
    pub fn reasoning_end(id: impl Into<String>) -> Self {
        Self::ReasoningEnd(reasoning_end::ReasoningEnd::new(id))
    }

    /// Create a tool input start event
    pub fn tool_input_start(id: impl Into<String>, tool_name: impl Into<String>) -> Self {
        Self::ToolInputStart(tool_input_start::ToolInputStart::new(id, tool_name))
    }

    /// Create a tool input delta event
    pub fn tool_input_delta(id: impl Into<String>, delta: impl Into<String>) -> Self {
        Self::ToolInputDelta(tool_input_delta::ToolInputDelta::new(id, delta))
    }

    /// Create a tool input end event
    pub fn tool_input_end(id: impl Into<String>) -> Self {
        Self::ToolInputEnd(tool_input_end::ToolInputEnd::new(id))
    }

    /// Create a stream start event
    pub fn stream_start(warnings: Vec<LanguageModelCallWarning>) -> Self {
        Self::StreamStart(stream_start::StreamStart::new(warnings))
    }

    /// Create a finish event
    pub fn finish(usage: LanguageModelUsage, finish_reason: LanguageModelFinishReason) -> Self {
        Self::Finish(finish::Finish::new(usage, finish_reason))
    }

    /// Create a raw chunk event
    pub fn raw(raw_value: Value) -> Self {
        Self::Raw(raw::Raw::new(raw_value))
    }

    /// Create an error event
    pub fn error(error: Value) -> Self {
        Self::Error(error::Error::new(error))
    }
}
