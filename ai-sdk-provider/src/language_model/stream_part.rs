use crate::language_model::call_warning::CallWarning;
use crate::language_model::content::file::File;
use crate::language_model::finish_reason::FinishReason;
use crate::language_model::response_metadata::ResponseMetadata;
use crate::language_model::content::source::Source;
use crate::language_model::content::tool_call::ToolCall;
use crate::language_model::content::tool_result::ToolResult;
use crate::language_model::usage::Usage;
use crate::shared::provider_metadata::ProviderMetadata;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Stream parts that can be emitted during model generation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum StreamPart {
    // Text blocks
    /// Start of a text block
    #[serde(rename_all = "camelCase")]
    TextStart {
        /// Unique identifier for this text block
        id: String,

        /// Provider-specific metadata
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,
    },

    /// Text delta (incremental text content)
    #[serde(rename_all = "camelCase")]
    TextDelta {
        /// ID of the text block
        id: String,

        /// The text delta/chunk
        delta: String,

        /// Provider-specific metadata
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,
    },

    /// End of a text block
    #[serde(rename_all = "camelCase")]
    TextEnd {
        /// ID of the text block
        id: String,

        /// Provider-specific metadata
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,
    },

    // Reasoning blocks
    /// Start of a reasoning block
    #[serde(rename_all = "camelCase")]
    ReasoningStart {
        /// Unique identifier for this reasoning block
        id: String,

        /// Provider-specific metadata
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,
    },

    /// Reasoning delta (incremental reasoning content)
    #[serde(rename_all = "camelCase")]
    ReasoningDelta {
        /// ID of the reasoning block
        id: String,

        /// The reasoning delta/chunk
        delta: String,

        /// Provider-specific metadata
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,
    },

    /// End of a reasoning block
    #[serde(rename_all = "camelCase")]
    ReasoningEnd {
        /// ID of the reasoning block
        id: String,

        /// Provider-specific metadata
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,
    },

    // Tool input blocks
    /// Start of tool input
    #[serde(rename_all = "camelCase")]
    ToolInputStart {
        /// Unique identifier for this tool call
        id: String,

        /// Name of the tool being called
        tool_name: String,

        /// Provider-specific metadata
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,

        /// Whether the tool will be executed by the provider
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_executed: Option<bool>,
    },

    /// Tool input delta (incremental tool arguments)
    #[serde(rename_all = "camelCase")]
    ToolInputDelta {
        /// ID of the tool call
        id: String,

        /// The input delta/chunk (JSON string)
        delta: String,

        /// Provider-specific metadata
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,
    },

    /// End of tool input
    #[serde(rename_all = "camelCase")]
    ToolInputEnd {
        /// ID of the tool call
        id: String,

        /// Provider-specific metadata
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,
    },

    // Complete tool call and result
    /// Complete tool call
    #[serde(rename = "tool-call")]
    ToolCall(ToolCall),

    /// Tool result
    #[serde(rename = "tool-result")]
    ToolResult(ToolResult),

    // Files and sources
    /// File content
    #[serde(rename = "file")]
    File(File),

    /// Source content
    #[serde(rename = "source")]
    Source(Source),

    // Stream metadata events
    /// Stream start event with warnings
    #[serde(rename = "stream-start")]
    StreamStart {
        /// Warnings for the call (e.g., unsupported settings)
        warnings: Vec<CallWarning>,
    },

    /// Response metadata
    #[serde(rename = "response-metadata")]
    ResponseMetadata(ResponseMetadata),

    /// Stream finish event
    #[serde(rename_all = "camelCase")]
    Finish {
        /// Usage information
        usage: Usage,

        /// Reason for finishing
        finish_reason: FinishReason,

        /// Provider-specific metadata
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,
    },

    // Raw chunks and errors
    /// Raw chunk from provider (if enabled)
    #[serde(rename = "raw")]
    Raw {
        /// The raw value from the provider
        #[serde(rename = "rawValue")]
        raw_value: Value,
    },

    /// Error event
    #[serde(rename = "error")]
    Error {
        /// The error value
        error: Value,
    },
}

// Helper implementations
impl StreamPart {
    /// Check if this is a text-related part
    pub fn is_text(&self) -> bool {
        matches!(
            self,
            Self::TextStart { .. } | Self::TextDelta { .. } | Self::TextEnd { .. }
        )
    }

    /// Check if this is a reasoning-related part
    pub fn is_reasoning(&self) -> bool {
        matches!(
            self,
            Self::ReasoningStart { .. } | Self::ReasoningDelta { .. } | Self::ReasoningEnd { .. }
        )
    }

    /// Check if this is a tool-related part
    pub fn is_tool(&self) -> bool {
        matches!(
            self,
            Self::ToolInputStart { .. }
                | Self::ToolInputDelta { .. }
                | Self::ToolInputEnd { .. }
                | Self::ToolCall(_)
                | Self::ToolResult(_)
        )
    }

    /// Check if this is a delta (incremental content)
    pub fn is_delta(&self) -> bool {
        matches!(
            self,
            Self::TextDelta { .. } | Self::ReasoningDelta { .. } | Self::ToolInputDelta { .. }
        )
    }

    /// Get the delta content if this is a delta part
    pub fn delta(&self) -> Option<&str> {
        match self {
            Self::TextDelta { delta, .. }
            | Self::ReasoningDelta { delta, .. }
            | Self::ToolInputDelta { delta, .. } => Some(delta),
            _ => None,
        }
    }

    /// Check if this is a finish event
    pub fn is_finish(&self) -> bool {
        matches!(self, Self::Finish { .. })
    }

    /// Check if this is an error event
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error { .. })
    }

    /// Get the ID if this part has one
    pub fn id(&self) -> Option<&str> {
        match self {
            Self::TextStart { id, .. }
            | Self::TextDelta { id, .. }
            | Self::TextEnd { id, .. }
            | Self::ReasoningStart { id, .. }
            | Self::ReasoningDelta { id, .. }
            | Self::ReasoningEnd { id, .. }
            | Self::ToolInputStart { id, .. }
            | Self::ToolInputDelta { id, .. }
            | Self::ToolInputEnd { id, .. } => Some(id),
            _ => None,
        }
    }
}

// Constructors for common cases
impl StreamPart {
    /// Create a text start event
    pub fn text_start(id: impl Into<String>) -> Self {
        Self::TextStart {
            id: id.into(),
            provider_metadata: None,
        }
    }

    /// Create a text delta event
    pub fn text_delta(id: impl Into<String>, delta: impl Into<String>) -> Self {
        Self::TextDelta {
            id: id.into(),
            delta: delta.into(),
            provider_metadata: None,
        }
    }

    /// Create a text end event
    pub fn text_end(id: impl Into<String>) -> Self {
        Self::TextEnd {
            id: id.into(),
            provider_metadata: None,
        }
    }

    /// Create a reasoning start event
    pub fn reasoning_start(id: impl Into<String>) -> Self {
        Self::ReasoningStart {
            id: id.into(),
            provider_metadata: None,
        }
    }

    /// Create a reasoning delta event
    pub fn reasoning_delta(id: impl Into<String>, delta: impl Into<String>) -> Self {
        Self::ReasoningDelta {
            id: id.into(),
            delta: delta.into(),
            provider_metadata: None,
        }
    }

    /// Create a reasoning end event
    pub fn reasoning_end(id: impl Into<String>) -> Self {
        Self::ReasoningEnd {
            id: id.into(),
            provider_metadata: None,
        }
    }

    /// Create a tool input start event
    pub fn tool_input_start(id: impl Into<String>, tool_name: impl Into<String>) -> Self {
        Self::ToolInputStart {
            id: id.into(),
            tool_name: tool_name.into(),
            provider_metadata: None,
            provider_executed: None,
        }
    }

    /// Create a tool input delta event
    pub fn tool_input_delta(id: impl Into<String>, delta: impl Into<String>) -> Self {
        Self::ToolInputDelta {
            id: id.into(),
            delta: delta.into(),
            provider_metadata: None,
        }
    }

    /// Create a tool input end event
    pub fn tool_input_end(id: impl Into<String>) -> Self {
        Self::ToolInputEnd {
            id: id.into(),
            provider_metadata: None,
        }
    }

    /// Create a stream start event
    pub fn stream_start(warnings: Vec<CallWarning>) -> Self {
        Self::StreamStart { warnings }
    }

    /// Create a finish event
    pub fn finish(usage: Usage, finish_reason: FinishReason) -> Self {
        Self::Finish {
            usage,
            finish_reason,
            provider_metadata: None,
        }
    }

    /// Create a raw chunk event
    pub fn raw(raw_value: Value) -> Self {
        Self::Raw { raw_value }
    }

    /// Create an error event
    pub fn error(error: Value) -> Self {
        Self::Error { error }
    }
}
