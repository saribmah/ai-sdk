use crate::generate_text::{
    RequestMetadata, SourceOutput, ToolApprovalRequestOutput, TypedToolError,
    TypedToolResult,
};
use crate::tool::TypedToolCall;
use ai_sdk_provider::language_model::call_warning::LanguageModelCallWarning;
use ai_sdk_provider::language_model::finish_reason::LanguageModelFinishReason;
use ai_sdk_provider::language_model::response_metadata::LanguageModelResponseMetadata;
use ai_sdk_provider::language_model::usage::LanguageModelUsage;
use ai_sdk_provider::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A serializable representation of a generated file for streaming.
///
/// This is used in stream parts to represent files that are being generated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamGeneratedFile {
    /// The file content as a base64-encoded string.
    pub base64: String,

    /// The IANA media type of the file.
    pub media_type: String,

    /// Optional filename.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// A part of a text stream produced during streaming text generation.
///
/// This enum represents all the different types of stream parts that can be
/// produced during a streaming text generation operation. It closely mirrors
/// the TypeScript implementation from Vercel's AI SDK.
///
/// # Type Parameters
///
/// * `INPUT` - The input type for tools (defaults to `Value`)
/// * `OUTPUT` - The output type from tools (defaults to `Value`)
///
/// # Example
///
/// ```
/// use ai_sdk_core::stream_text::TextStreamPart;
/// use serde_json::Value;
///
/// // A text delta part
/// let part = TextStreamPart::<Value, Value>::TextDelta {
///     id: "text_123".to_string(),
///     provider_metadata: None,
///     text: "Hello ".to_string(),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum TextStreamPart<INPUT = Value, OUTPUT = Value> {
    /// Indicates the start of a text segment.
    #[serde(rename_all = "camelCase")]
    TextStart {
        /// ID of the text segment.
        id: String,

        /// Provider-specific metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<SharedProviderMetadata>,
    },

    /// Indicates the end of a text segment.
    #[serde(rename_all = "camelCase")]
    TextEnd {
        /// ID of the text segment.
        id: String,

        /// Provider-specific metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<SharedProviderMetadata>,
    },

    /// A text delta (incremental update).
    #[serde(rename_all = "camelCase")]
    TextDelta {
        /// ID of the text segment.
        id: String,

        /// Provider-specific metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<SharedProviderMetadata>,

        /// The text content.
        text: String,
    },

    /// Indicates the start of a reasoning segment.
    #[serde(rename_all = "camelCase")]
    ReasoningStart {
        /// ID of the reasoning segment.
        id: String,

        /// Provider-specific metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<SharedProviderMetadata>,
    },

    /// Indicates the end of a reasoning segment.
    #[serde(rename_all = "camelCase")]
    ReasoningEnd {
        /// ID of the reasoning segment.
        id: String,

        /// Provider-specific metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<SharedProviderMetadata>,
    },

    /// A reasoning delta (incremental update).
    #[serde(rename_all = "camelCase")]
    ReasoningDelta {
        /// ID of the reasoning segment.
        id: String,

        /// Provider-specific metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<SharedProviderMetadata>,

        /// The reasoning text content.
        text: String,
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
        provider_metadata: Option<SharedProviderMetadata>,

        /// Whether the provider executed this tool.
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_executed: Option<bool>,

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
        provider_metadata: Option<SharedProviderMetadata>,
    },

    /// A tool input delta (incremental update).
    #[serde(rename_all = "camelCase")]
    ToolInputDelta {
        /// ID of the tool input.
        id: String,

        /// The incremental input data.
        delta: String,

        /// Provider-specific metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<SharedProviderMetadata>,
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

    /// A tool output that was denied.
    ToolOutputDenied {
        /// The tool call ID.
        #[serde(rename = "toolCallId")]
        tool_call_id: String,

        /// Name of the tool.
        tool_name: String,

        /// The input that was provided to the tool.
        input: INPUT,

        /// Optional reason for denial.
        #[serde(skip_serializing_if = "Option::is_none")]
        reason: Option<String>,

        /// Whether the provider executed this tool.
        #[serde(rename = "providerExecuted", skip_serializing_if = "Option::is_none")]
        provider_executed: Option<bool>,

        /// Whether this is a dynamic tool.
        #[serde(skip_serializing_if = "Option::is_none")]
        dynamic: Option<bool>,
    },

    /// A tool approval request.
    ToolApprovalRequest {
        /// The approval request data.
        #[serde(flatten)]
        approval_request: ToolApprovalRequestOutput<INPUT>,
    },

    /// Indicates the start of a generation step.
    #[serde(rename_all = "camelCase")]
    StartStep {
        /// Request metadata for this step.
        request: RequestMetadata,

        /// Warnings from the provider.
        warnings: Vec<LanguageModelCallWarning>,
    },

    /// Indicates the completion of a generation step.
    #[serde(rename_all = "camelCase")]
    FinishStep {
        /// Response metadata for this step.
        response: LanguageModelResponseMetadata,

        /// Token usage for this step.
        usage: LanguageModelUsage,

        /// The reason why generation finished.
        finish_reason: LanguageModelFinishReason,

        /// Provider-specific metadata.
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<SharedProviderMetadata>,
    },

    /// Indicates the start of the entire generation.
    Start,

    /// Indicates the completion of the entire generation.
    #[serde(rename_all = "camelCase")]
    Finish {
        /// The reason why generation finished.
        finish_reason: LanguageModelFinishReason,

        /// Total token usage across all steps.
        total_usage: LanguageModelUsage,
    },

    /// Indicates the generation was aborted.
    Abort,

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_delta_serialization() {
        let part = TextStreamPart::<Value, Value>::TextDelta {
            id: "text_123".to_string(),
            provider_metadata: None,
            text: "Hello".to_string(),
        };

        let json = serde_json::to_value(&part).unwrap();
        assert_eq!(json["type"], "text-delta");
        assert_eq!(json["id"], "text_123");
        assert_eq!(json["text"], "Hello");
    }

    #[test]
    fn test_start_step_serialization() {
        let part = TextStreamPart::<Value, Value>::StartStep {
            request: RequestMetadata::default(),
            warnings: vec![],
        };

        let json = serde_json::to_value(&part).unwrap();
        assert_eq!(json["type"], "start-step");
    }

    #[test]
    fn test_finish_serialization() {
        let part = TextStreamPart::<Value, Value>::Finish {
            finish_reason: LanguageModelFinishReason::Stop,
            total_usage: LanguageModelUsage::new(100, 50),
        };

        let json = serde_json::to_value(&part).unwrap();
        assert_eq!(json["type"], "finish");
        assert_eq!(json["finishReason"], "Stop");
    }
}
