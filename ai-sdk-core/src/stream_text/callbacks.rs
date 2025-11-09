use crate::generate_text::StepResult;
use crate::stream_text::TextStreamPart;
use ai_sdk_provider::language_model::usage::LanguageModelUsage;
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;

/// Event passed to the `on_error` callback during streaming.
#[derive(Debug, Clone)]
pub struct StreamTextErrorEvent {
    /// The error that occurred.
    pub error: Value,
}

/// Event passed to the `on_chunk` callback during streaming.
///
/// Contains a chunk that represents one of the streamable content types.
#[derive(Debug, Clone, PartialEq)]
pub struct StreamTextChunkEvent {
    /// The stream chunk.
    pub chunk: ChunkStreamPart,
}

/// A subset of TextStreamPart that represents chunks that are streamed incrementally.
///
/// This includes only the stream parts that represent actual content being generated,
/// not lifecycle or metadata events.
#[derive(Debug, Clone, PartialEq)]
pub enum ChunkStreamPart {
    /// A text delta (incremental update).
    TextDelta {
        /// ID of the text stream.
        id: String,
        /// Provider-specific metadata.
        provider_metadata:
            Option<ai_sdk_provider::shared::provider_metadata::SharedProviderMetadata>,
        /// The text delta content.
        text: String,
    },

    /// A reasoning delta (incremental update).
    ReasoningDelta {
        /// ID of the reasoning stream.
        id: String,
        /// Provider-specific metadata.
        provider_metadata:
            Option<ai_sdk_provider::shared::provider_metadata::SharedProviderMetadata>,
        /// The reasoning delta content.
        text: String,
    },

    /// A source/reference in the generation.
    Source {
        /// The source output.
        source: crate::output::SourceOutput,
    },

    /// A tool call.
    ToolCall {
        /// The tool call.
        tool_call: crate::tool::ToolCall,
    },

    /// Indicates the start of a tool input.
    ToolInputStart {
        /// ID of the tool call.
        id: String,
        /// Name of the tool being called.
        tool_name: String,
        /// Provider-specific metadata.
        provider_metadata:
            Option<ai_sdk_provider::shared::provider_metadata::SharedProviderMetadata>,
        /// Whether the tool was executed by the provider.
        provider_executed: Option<bool>,
        /// Whether this is a dynamic tool.
        dynamic: Option<bool>,
        /// Optional title for the tool call.
        title: Option<String>,
    },

    /// A tool input delta (incremental update).
    ToolInputDelta {
        /// ID of the tool call.
        id: String,
        /// The input delta content.
        delta: String,
        /// Provider-specific metadata.
        provider_metadata:
            Option<ai_sdk_provider::shared::provider_metadata::SharedProviderMetadata>,
    },

    /// A tool result.
    ToolResult {
        /// The tool result.
        tool_result: crate::tool::ToolResult,
    },

    /// A raw value from the provider (for debugging/custom handling).
    Raw {
        /// The raw value from the provider.
        raw_value: Value,
    },
}

impl ChunkStreamPart {
    /// Attempts to extract a ChunkStreamPart from a TextStreamPart.
    ///
    /// Returns Some if the part is a chunk type, None otherwise.
    pub fn from_stream_part(part: &TextStreamPart) -> Option<Self> {
        match part {
            TextStreamPart::TextDelta {
                id,
                provider_metadata,
                text,
            } => Some(ChunkStreamPart::TextDelta {
                id: id.clone(),
                provider_metadata: provider_metadata.clone(),
                text: text.clone(),
            }),
            TextStreamPart::ReasoningDelta {
                id,
                provider_metadata,
                text,
            } => Some(ChunkStreamPart::ReasoningDelta {
                id: id.clone(),
                provider_metadata: provider_metadata.clone(),
                text: text.clone(),
            }),
            TextStreamPart::Source { source } => Some(ChunkStreamPart::Source {
                source: source.clone(),
            }),
            TextStreamPart::ToolCall { tool_call } => Some(ChunkStreamPart::ToolCall {
                tool_call: tool_call.clone(),
            }),
            TextStreamPart::ToolInputStart {
                id,
                tool_name,
                provider_metadata,
                provider_executed,
                dynamic,
                title,
            } => Some(ChunkStreamPart::ToolInputStart {
                id: id.clone(),
                tool_name: tool_name.clone(),
                provider_metadata: provider_metadata.clone(),
                provider_executed: *provider_executed,
                dynamic: *dynamic,
                title: title.clone(),
            }),
            TextStreamPart::ToolInputDelta {
                id,
                delta,
                provider_metadata,
            } => Some(ChunkStreamPart::ToolInputDelta {
                id: id.clone(),
                delta: delta.clone(),
                provider_metadata: provider_metadata.clone(),
            }),
            TextStreamPart::ToolResult { tool_result } => Some(ChunkStreamPart::ToolResult {
                tool_result: tool_result.clone(),
            }),
            TextStreamPart::Raw { raw_value } => Some(ChunkStreamPart::Raw {
                raw_value: raw_value.clone(),
            }),
            _ => None,
        }
    }
}

/// Event passed to the `on_finish` callback during streaming.
///
/// This extends the step result with additional information about all steps
/// and total usage across the entire generation.
#[derive(Debug, Clone, PartialEq)]
pub struct StreamTextFinishEvent {
    /// The final step result.
    pub step_result: StepResult,

    /// Details for all steps in the generation.
    pub steps: Vec<StepResult>,

    /// Total usage for all steps. This is the sum of the usage of all steps.
    pub total_usage: LanguageModelUsage,
}

/// Event passed to the `on_abort` callback during streaming.
#[derive(Debug, Clone, PartialEq)]
pub struct StreamTextAbortEvent {
    /// Details for all previously finished steps.
    pub steps: Vec<StepResult>,
}

/// Callback that is called when an error occurs during streaming.
///
/// # Example
///
/// ```rust
/// use ai_sdk_core::stream_text::callbacks::{StreamTextOnErrorCallback, StreamTextErrorEvent};
/// use std::pin::Pin;
/// use std::future::Future;
///
/// let callback: StreamTextOnErrorCallback = Box::new(|event: StreamTextErrorEvent| {
///     Box::pin(async move {
///         eprintln!("Error occurred: {:?}", event.error);
///     })
/// });
/// ```
pub type StreamTextOnErrorCallback =
    Box<dyn Fn(StreamTextErrorEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Callback that is called when a step finishes during streaming.
///
/// # Example
///
/// ```rust
/// use ai_sdk_core::stream_text::callbacks::StreamTextOnStepFinishCallback;
/// use ai_sdk_core::StepResult;
/// use serde_json::Value;
/// use std::pin::Pin;
/// use std::future::Future;
///
/// let callback: StreamTextOnStepFinishCallback = Box::new(|step_result: StepResult| {
///     Box::pin(async move {
///         println!("Step finished with {} tokens", step_result.usage.total_tokens);
///     })
/// });
/// ```
pub type StreamTextOnStepFinishCallback =
    Box<dyn Fn(StepResult) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Callback that is called for each chunk during streaming.
///
/// Only receives chunks that represent actual content being generated
/// (text-delta, reasoning-delta, source, tool-call, tool-input-start,
/// tool-input-delta, tool-result, raw).
///
/// # Example
///
/// ```rust
/// use ai_sdk_core::stream_text::callbacks::{StreamTextOnChunkCallback, StreamTextChunkEvent};
/// use serde_json::Value;
/// use std::pin::Pin;
/// use std::future::Future;
///
/// let callback: StreamTextOnChunkCallback = Box::new(|event: StreamTextChunkEvent| {
///     Box::pin(async move {
///         println!("Received chunk: {:?}", event.chunk);
///     })
/// });
/// ```
pub type StreamTextOnChunkCallback =
    Box<dyn Fn(StreamTextChunkEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Callback that is called when the entire generation finishes.
///
/// Receives the final step result along with details for all steps
/// and the total usage across all steps.
///
/// # Example
///
/// ```rust
/// use ai_sdk_core::stream_text::callbacks::{StreamTextOnFinishCallback, StreamTextFinishEvent};
/// use serde_json::Value;
/// use std::pin::Pin;
/// use std::future::Future;
///
/// let callback: StreamTextOnFinishCallback = Box::new(|event: StreamTextFinishEvent| {
///     Box::pin(async move {
///         println!("Generation finished. Total steps: {}", event.steps.len());
///         println!("Total tokens used: {}", event.total_usage.total_tokens);
///     })
/// });
/// ```
pub type StreamTextOnFinishCallback =
    Box<dyn Fn(StreamTextFinishEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Callback that is called when the generation is aborted.
///
/// # Example
///
/// ```rust
/// use ai_sdk_core::stream_text::callbacks::{StreamTextOnAbortCallback, StreamTextAbortEvent};
/// use serde_json::Value;
/// use std::pin::Pin;
/// use std::future::Future;
///
/// let callback: StreamTextOnAbortCallback = Box::new(|event: StreamTextAbortEvent| {
///     Box::pin(async move {
///         println!("Generation aborted. Steps completed: {}", event.steps.len());
///     })
/// });
/// ```
pub type StreamTextOnAbortCallback =
    Box<dyn Fn(StreamTextAbortEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool::ToolCall;

    #[test]
    fn test_chunk_from_text_delta() {
        let part = TextStreamPart::TextDelta {
            id: "text_1".to_string(),
            provider_metadata: None,
            text: "Hello".to_string(),
        };

        let chunk = ChunkStreamPart::from_stream_part(&part);
        assert!(chunk.is_some());

        if let Some(ChunkStreamPart::TextDelta { text, .. }) = chunk {
            assert_eq!(text, "Hello");
        } else {
            panic!("Expected TextDelta chunk");
        }
    }

    #[test]
    fn test_chunk_from_non_chunk_part() {
        let part = TextStreamPart::Start;

        let chunk = ChunkStreamPart::from_stream_part(&part);
        assert!(chunk.is_none());
    }

    #[test]
    fn test_chunk_from_tool_call() {
        let tool_call = ToolCall::new(
            "call_123".to_string(),
            "test_tool".to_string(),
            serde_json::json!({"arg": "value"}),
        );

        let part = TextStreamPart::ToolCall {
            tool_call: tool_call.clone(),
        };

        let chunk = ChunkStreamPart::from_stream_part(&part);
        assert!(chunk.is_some());

        if let Some(ChunkStreamPart::ToolCall { tool_call: tc }) = chunk {
            assert_eq!(tc.tool_call_id, "call_123");
            assert_eq!(tc.tool_name, "test_tool");
        } else {
            panic!("Expected ToolCall chunk");
        }
    }

    #[tokio::test]
    async fn test_on_error_callback() {
        let called = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let called_clone = called.clone();

        let callback: StreamTextOnErrorCallback = Box::new(move |_event: StreamTextErrorEvent| {
            let called = called_clone.clone();
            Box::pin(async move {
                called.store(true, std::sync::atomic::Ordering::SeqCst);
            })
        });

        let event = StreamTextErrorEvent {
            error: serde_json::json!({"message": "test error"}),
        };

        callback(event).await;

        assert!(called.load(std::sync::atomic::Ordering::SeqCst));
    }

    #[test]
    fn test_finish_event_creation() {
        let step_result = StepResult::new(
            vec![],
            ai_sdk_provider::language_model::finish_reason::LanguageModelFinishReason::Stop,
            LanguageModelUsage::new(100, 50),
            None,
            Default::default(),
            Default::default(),
            None,
        );

        let event = StreamTextFinishEvent {
            step_result: step_result.clone(),
            steps: vec![step_result],
            total_usage: LanguageModelUsage::new(100, 50),
        };

        assert_eq!(event.steps.len(), 1);
        assert_eq!(event.total_usage.total_tokens, 150);
    }

    #[test]
    fn test_abort_event_creation() {
        let event = StreamTextAbortEvent { steps: vec![] };

        assert_eq!(event.steps.len(), 0);
    }
}
