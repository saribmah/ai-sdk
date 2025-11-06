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
pub struct StreamTextChunkEvent<INPUT = Value, OUTPUT = Value> {
    /// The stream chunk.
    pub chunk: ChunkStreamPart<INPUT, OUTPUT>,
}

/// A subset of TextStreamPart that represents chunks that are streamed incrementally.
///
/// This includes only the stream parts that represent actual content being generated,
/// not lifecycle or metadata events.
#[derive(Debug, Clone, PartialEq)]
pub enum ChunkStreamPart<INPUT = Value, OUTPUT = Value> {
    /// A text delta (incremental update).
    TextDelta {
        id: String,
        provider_metadata:
            Option<ai_sdk_provider::shared::provider_metadata::SharedProviderMetadata>,
        text: String,
    },

    /// A reasoning delta (incremental update).
    ReasoningDelta {
        id: String,
        provider_metadata:
            Option<ai_sdk_provider::shared::provider_metadata::SharedProviderMetadata>,
        text: String,
    },

    /// A source/reference in the generation.
    Source {
        source: crate::generate_text::SourceOutput,
    },

    /// A tool call.
    ToolCall {
        tool_call: crate::generate_text::TypedToolCall<INPUT>,
    },

    /// Indicates the start of a tool input.
    ToolInputStart {
        id: String,
        tool_name: String,
        provider_metadata:
            Option<ai_sdk_provider::shared::provider_metadata::SharedProviderMetadata>,
        provider_executed: Option<bool>,
        dynamic: Option<bool>,
        title: Option<String>,
    },

    /// A tool input delta (incremental update).
    ToolInputDelta {
        id: String,
        delta: String,
        provider_metadata:
            Option<ai_sdk_provider::shared::provider_metadata::SharedProviderMetadata>,
    },

    /// A tool result.
    ToolResult {
        tool_result: crate::generate_text::TypedToolResult<INPUT, OUTPUT>,
    },

    /// A raw value from the provider (for debugging/custom handling).
    Raw { raw_value: Value },
}

impl<INPUT, OUTPUT> ChunkStreamPart<INPUT, OUTPUT> {
    /// Attempts to extract a ChunkStreamPart from a TextStreamPart.
    ///
    /// Returns Some if the part is a chunk type, None otherwise.
    pub fn from_stream_part(part: &TextStreamPart<INPUT, OUTPUT>) -> Option<Self>
    where
        INPUT: Clone,
        OUTPUT: Clone,
    {
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
pub struct StreamTextFinishEvent<INPUT = Value, OUTPUT = Value> {
    /// The final step result.
    pub step_result: StepResult<INPUT, OUTPUT>,

    /// Details for all steps in the generation.
    pub steps: Vec<StepResult<INPUT, OUTPUT>>,

    /// Total usage for all steps. This is the sum of the usage of all steps.
    pub total_usage: LanguageModelUsage,
}

/// Event passed to the `on_abort` callback during streaming.
#[derive(Debug, Clone, PartialEq)]
pub struct StreamTextAbortEvent<INPUT = Value, OUTPUT = Value> {
    /// Details for all previously finished steps.
    pub steps: Vec<StepResult<INPUT, OUTPUT>>,
}

/// Callback that is called when an error occurs during streaming.
///
/// Equivalent to TypeScript's `StreamTextOnErrorCallback`.
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
/// Equivalent to TypeScript's `StreamTextOnStepFinishCallback<TOOLS>`.
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
/// let callback: StreamTextOnStepFinishCallback<Value, Value> = Box::new(|step_result: StepResult<Value, Value>| {
///     Box::pin(async move {
///         println!("Step finished with {} tokens", step_result.usage.total_tokens);
///     })
/// });
/// ```
pub type StreamTextOnStepFinishCallback<INPUT = Value, OUTPUT = Value> = Box<
    dyn Fn(StepResult<INPUT, OUTPUT>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync,
>;

/// Callback that is called for each chunk during streaming.
///
/// Equivalent to TypeScript's `StreamTextOnChunkCallback<TOOLS>`.
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
/// let callback: StreamTextOnChunkCallback<Value, Value> = Box::new(|event: StreamTextChunkEvent<Value, Value>| {
///     Box::pin(async move {
///         println!("Received chunk: {:?}", event.chunk);
///     })
/// });
/// ```
pub type StreamTextOnChunkCallback<INPUT = Value, OUTPUT = Value> = Box<
    dyn Fn(StreamTextChunkEvent<INPUT, OUTPUT>) -> Pin<Box<dyn Future<Output = ()> + Send>>
        + Send
        + Sync,
>;

/// Callback that is called when the entire generation finishes.
///
/// Equivalent to TypeScript's `StreamTextOnFinishCallback<TOOLS>`.
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
/// let callback: StreamTextOnFinishCallback<Value, Value> = Box::new(|event: StreamTextFinishEvent<Value, Value>| {
///     Box::pin(async move {
///         println!("Generation finished. Total steps: {}", event.steps.len());
///         println!("Total tokens used: {}", event.total_usage.total_tokens);
///     })
/// });
/// ```
pub type StreamTextOnFinishCallback<INPUT = Value, OUTPUT = Value> = Box<
    dyn Fn(StreamTextFinishEvent<INPUT, OUTPUT>) -> Pin<Box<dyn Future<Output = ()> + Send>>
        + Send
        + Sync,
>;

/// Callback that is called when the generation is aborted.
///
/// Equivalent to TypeScript's `StreamTextOnAbortCallback<TOOLS>`.
///
/// # Example
///
/// ```rust
/// use ai_sdk_core::stream_text::callbacks::{StreamTextOnAbortCallback, StreamTextAbortEvent};
/// use serde_json::Value;
/// use std::pin::Pin;
/// use std::future::Future;
///
/// let callback: StreamTextOnAbortCallback<Value, Value> = Box::new(|event: StreamTextAbortEvent<Value, Value>| {
///     Box::pin(async move {
///         println!("Generation aborted. Steps completed: {}", event.steps.len());
///     })
/// });
/// ```
pub type StreamTextOnAbortCallback<INPUT = Value, OUTPUT = Value> = Box<
    dyn Fn(StreamTextAbortEvent<INPUT, OUTPUT>) -> Pin<Box<dyn Future<Output = ()> + Send>>
        + Send
        + Sync,
>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_text::{StaticToolCall, TypedToolCall};

    #[test]
    fn test_chunk_from_text_delta() {
        let part = TextStreamPart::<Value, Value>::TextDelta {
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
        let part = TextStreamPart::<Value, Value>::Start;

        let chunk = ChunkStreamPart::from_stream_part(&part);
        assert!(chunk.is_none());
    }

    #[test]
    fn test_chunk_from_tool_call() {
        let tool_call = TypedToolCall::Static(StaticToolCall::new(
            "call_123",
            "test_tool",
            serde_json::json!({"arg": "value"}),
        ));

        let part = TextStreamPart::<Value, Value>::ToolCall {
            tool_call: tool_call.clone(),
        };

        let chunk = ChunkStreamPart::from_stream_part(&part);
        assert!(chunk.is_some());

        if let Some(ChunkStreamPart::ToolCall { tool_call: tc }) = chunk {
            match tc {
                TypedToolCall::Static(sc) => {
                    assert_eq!(sc.tool_call_id, "call_123");
                    assert_eq!(sc.tool_name, "test_tool");
                }
                _ => panic!("Expected Static tool call"),
            }
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
        let step_result: StepResult<Value, Value> = StepResult::new(
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
        let event = StreamTextAbortEvent::<Value, Value> { steps: vec![] };

        assert_eq!(event.steps.len(), 0);
    }
}
