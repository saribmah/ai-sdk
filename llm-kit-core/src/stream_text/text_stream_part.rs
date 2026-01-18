use crate::generate_text::RequestMetadata;
use crate::output::SourceOutput;
use llm_kit_provider::language_model::call_warning::LanguageModelCallWarning;
use llm_kit_provider::language_model::finish_reason::LanguageModelFinishReason;
use llm_kit_provider::language_model::response_metadata::LanguageModelResponseMetadata;
use llm_kit_provider::language_model::usage::LanguageModelUsage;
use llm_kit_provider::shared::provider_metadata::SharedProviderMetadata;
use llm_kit_provider_utils::tool::{ToolApprovalRequestOutput, ToolCall, ToolError, ToolResult};
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
/// This enum represents all the different types of events that can occur during
/// streaming text generation. Stream parts are emitted in real-time as the model
/// generates content, allowing you to process text, tool calls, and metadata
/// incrementally.
///
/// # Stream Part Types
///
/// - **Text**: Text content (start, delta, end)
/// - **Reasoning**: Model's reasoning process (for models that support it)
/// - **Sources**: Citations and sources (for RAG models)
/// - **Tool Calls**: Function calls from the model
/// - **Tool Results**: Results from executed tools
/// - **File**: Generated files (images, audio, etc.)
/// - **Finish**: Generation completion with finish reason
/// - **Metadata**: Usage statistics and response information
/// - **Error**: Errors during streaming
///
/// # Usage Pattern
///
/// ```no_run
/// use llm_kit_core::StreamText;
/// use llm_kit_core::stream_text::TextStreamPart;
/// use llm_kit_core::prompt::Prompt;
/// use futures::StreamExt;
/// # use llm_kit_provider::LanguageModel;
/// # use std::sync::Arc;
/// # async fn example(model: Arc<dyn LanguageModel>) -> Result<(), Box<dyn std::error::Error>> {
/// # let prompt = Prompt::text("Hello");
///
/// let result = StreamText::new(model, prompt)
///     .execute()
///     .await?;
///
/// let mut stream = result.full_stream();
/// while let Some(part) = stream.next().await {
///     match part {
///         TextStreamPart::TextDelta { text, .. } => {
///             print!("{}", text); // Print text as it arrives
///         }
///         TextStreamPart::Finish { finish_reason, .. } => {
///             println!("\nFinished: {:?}", finish_reason);
///         }
///         _ => {}
///     }
/// }
/// # Ok(())
/// # }
/// ```
///
/// # Serialization
///
/// Stream parts are serializable to JSON with a `type` field discriminator,
/// making them suitable for transmission over WebSockets or other streaming
/// protocols.
///
/// # Examples
///
/// ## Text Delta
///
/// ```
/// use llm_kit_core::stream_text::TextStreamPart;
///
/// let part = TextStreamPart::TextDelta {
///     id: "text_123".to_string(),
///     provider_metadata: None,
///     text: "Hello ".to_string(),
/// };
/// ```
///
/// ## Tool Call
///
/// ```no_run
/// use llm_kit_core::stream_text::TextStreamPart;
/// use llm_kit_provider_utils::tool::ToolCall;
/// use serde_json::json;
///
/// let part = TextStreamPart::ToolCall {
///     tool_call: ToolCall::new(
///         "call_1".to_string(),
///         "get_weather".to_string(),
///         json!({"location": "Paris"}),
///     ),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum TextStreamPart {
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
        tool_call: ToolCall,
    },

    /// A tool result.
    ToolResult {
        /// The tool result data.
        #[serde(flatten)]
        tool_result: ToolResult,
    },

    /// A tool error.
    ToolError {
        /// The tool error data.
        #[serde(flatten)]
        tool_error: ToolError,
    },

    /// A tool output that was denied.
    ToolOutputDenied {
        /// The tool call ID.
        #[serde(rename = "toolCallId")]
        tool_call_id: String,

        /// Name of the tool.
        tool_name: String,

        /// The input that was provided to the tool.
        input: Value,

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
        approval_request: ToolApprovalRequestOutput,
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
        let part = TextStreamPart::TextDelta {
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
        let part = TextStreamPart::StartStep {
            request: RequestMetadata::default(),
            warnings: vec![],
        };

        let json = serde_json::to_value(&part).unwrap();
        assert_eq!(json["type"], "start-step");
    }

    #[test]
    fn test_finish_serialization() {
        let part = TextStreamPart::Finish {
            finish_reason: LanguageModelFinishReason::Stop,
            total_usage: LanguageModelUsage::new(100, 50),
        };

        let json = serde_json::to_value(&part).unwrap();
        assert_eq!(json["type"], "finish");
        assert_eq!(json["finishReason"], "Stop");
    }
}
