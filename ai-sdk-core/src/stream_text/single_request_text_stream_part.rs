use crate::generate_text::{
    SourceOutput, ToolApprovalRequestOutput, TypedToolCall, TypedToolError, TypedToolResult,
};
use ai_sdk_provider::language_model::call_warning::CallWarning;
use ai_sdk_provider::language_model::finish_reason::FinishReason;
use ai_sdk_provider::language_model::response_metadata::ResponseMetadata;
use ai_sdk_provider::language_model::usage::Usage;
use ai_sdk_provider::shared::provider_metadata::ProviderMetadata;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::text_stream_part::StreamGeneratedFile;

/// A part of a text stream from a single LLM request (before multi-step processing).
///
/// This enum represents all the different types of stream parts that can be
/// produced during a single request to an LLM. It is used internally by the
/// streaming pipeline before tool execution and multi-step processing.
///
/// # Type Parameters
///
/// * `INPUT` - The input type for tools (defaults to `Value`)
/// * `OUTPUT` - The output type from tools (defaults to `Value`)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum SingleRequestTextStreamPart<INPUT = Value, OUTPUT = Value> {
    /// Indicates the start of a text segment.
    #[serde(rename_all = "camelCase")]
    TextStart {
        /// ID of the text segment.
        id: String,

        /// Provider-specific metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,
    },

    /// Indicates the end of a text segment.
    #[serde(rename_all = "camelCase")]
    TextEnd {
        /// ID of the text segment.
        id: String,

        /// Provider-specific metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,
    },

    /// A text delta (incremental update).
    #[serde(rename_all = "camelCase")]
    TextDelta {
        /// ID of the text segment.
        id: String,

        /// Provider-specific metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,

        /// The text content delta.
        delta: String,
    },

    /// Indicates the start of a reasoning segment.
    #[serde(rename_all = "camelCase")]
    ReasoningStart {
        /// ID of the reasoning segment.
        id: String,

        /// Provider-specific metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,
    },

    /// Indicates the end of a reasoning segment.
    #[serde(rename_all = "camelCase")]
    ReasoningEnd {
        /// ID of the reasoning segment.
        id: String,

        /// Provider-specific metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,
    },

    /// A reasoning delta (incremental update).
    #[serde(rename_all = "camelCase")]
    ReasoningDelta {
        /// ID of the reasoning segment.
        id: String,

        /// Provider-specific metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,

        /// The reasoning text content delta.
        delta: String,
    },

    /// Indicates the start of a tool input.
    #[serde(rename_all = "camelCase")]
    ToolInputStart {
        /// ID of the tool input.
        id: String,

        /// Name of the tool.
        tool_name: String,

        /// Provider-specific metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,

        /// Whether this is a dynamic tool.
        #[serde(skip_serializing_if = "Option::is_none")]
        dynamic: Option<bool>,

        /// Optional title for the tool.
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },

    /// Indicates the end of a tool input.
    #[serde(rename_all = "camelCase")]
    ToolInputEnd {
        /// ID of the tool input.
        id: String,

        /// Provider-specific metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,
    },

    /// A tool input delta (incremental update).
    #[serde(rename_all = "camelCase")]
    ToolInputDelta {
        /// ID of the tool input.
        id: String,

        /// The incremental input data delta.
        delta: String,

        /// Provider-specific metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,
    },

    /// A tool approval request.
    #[serde(rename = "tool-approval-request")]
    ToolApprovalRequest {
        /// The approval request data.
        #[serde(flatten)]
        approval_request: ToolApprovalRequestOutput<INPUT>,
    },

    /// A source/reference in the generation.
    Source {
        /// The source data.
        #[serde(flatten)]
        source: SourceOutput,
    },

    /// A generated file.
    File {
        /// The generated file.
        file: StreamGeneratedFile,
    },

    /// A tool call.
    ToolCall {
        /// The tool call data.
        #[serde(flatten)]
        tool_call: TypedToolCall<INPUT>,
    },

    /// A tool result.
    ToolResult {
        /// The tool result data.
        #[serde(flatten)]
        tool_result: TypedToolResult<INPUT, OUTPUT>,
    },

    /// A tool error.
    ToolError {
        /// The tool error data.
        #[serde(flatten)]
        tool_error: TypedToolError<INPUT>,
    },

    /// Indicates the start of the stream.
    #[serde(rename = "stream-start")]
    StreamStart {
        /// Warnings from the provider.
        warnings: Vec<CallWarning>,
    },

    /// Response metadata.
    #[serde(rename = "response-metadata")]
    ResponseMetadata {
        /// Optional response ID.
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,

        /// Optional timestamp.
        #[serde(skip_serializing_if = "Option::is_none")]
        timestamp: Option<String>,

        /// Optional model ID.
        #[serde(rename = "modelId", skip_serializing_if = "Option::is_none")]
        model_id: Option<String>,
    },

    /// Indicates the completion of the generation.
    #[serde(rename_all = "camelCase")]
    Finish {
        /// The reason why generation finished.
        finish_reason: FinishReason,

        /// Token usage for this request.
        usage: Usage,

        /// Provider-specific metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,
    },

    /// Indicates an error occurred during generation.
    Error {
        /// The error value.
        error: Value,
    },

    /// A raw value from the provider (for debugging/custom handling).
    Raw {
        /// The raw value.
        #[serde(rename = "rawValue")]
        raw_value: Value,
    },
}

impl<INPUT, OUTPUT> SingleRequestTextStreamPart<INPUT, OUTPUT> {
    /// Creates a text start part.
    pub fn text_start(id: impl Into<String>) -> Self {
        Self::TextStart {
            id: id.into(),
            provider_metadata: None,
        }
    }

    /// Creates a text delta part.
    pub fn text_delta(id: impl Into<String>, delta: impl Into<String>) -> Self {
        Self::TextDelta {
            id: id.into(),
            provider_metadata: None,
            delta: delta.into(),
        }
    }

    /// Creates a text end part.
    pub fn text_end(id: impl Into<String>) -> Self {
        Self::TextEnd {
            id: id.into(),
            provider_metadata: None,
        }
    }

    /// Creates a reasoning start part.
    pub fn reasoning_start(id: impl Into<String>) -> Self {
        Self::ReasoningStart {
            id: id.into(),
            provider_metadata: None,
        }
    }

    /// Creates a reasoning delta part.
    pub fn reasoning_delta(id: impl Into<String>, delta: impl Into<String>) -> Self {
        Self::ReasoningDelta {
            id: id.into(),
            provider_metadata: None,
            delta: delta.into(),
        }
    }

    /// Creates a reasoning end part.
    pub fn reasoning_end(id: impl Into<String>) -> Self {
        Self::ReasoningEnd {
            id: id.into(),
            provider_metadata: None,
        }
    }

    /// Creates a tool input start part.
    pub fn tool_input_start(id: impl Into<String>, tool_name: impl Into<String>) -> Self {
        Self::ToolInputStart {
            id: id.into(),
            tool_name: tool_name.into(),
            provider_metadata: None,
            dynamic: None,
            title: None,
        }
    }

    /// Creates a tool input delta part.
    pub fn tool_input_delta(id: impl Into<String>, delta: impl Into<String>) -> Self {
        Self::ToolInputDelta {
            id: id.into(),
            delta: delta.into(),
            provider_metadata: None,
        }
    }

    /// Creates a tool input end part.
    pub fn tool_input_end(id: impl Into<String>) -> Self {
        Self::ToolInputEnd {
            id: id.into(),
            provider_metadata: None,
        }
    }

    /// Creates a stream start part.
    pub fn stream_start(warnings: Vec<CallWarning>) -> Self {
        Self::StreamStart { warnings }
    }

    /// Creates a finish part.
    pub fn finish(finish_reason: FinishReason, usage: Usage) -> Self {
        Self::Finish {
            finish_reason,
            usage,
            provider_metadata: None,
        }
    }

    /// Creates an error part.
    pub fn error(error: Value) -> Self {
        Self::Error { error }
    }

    /// Creates a raw part.
    pub fn raw(raw_value: Value) -> Self {
        Self::Raw { raw_value }
    }
}
