use crate::error::AISDKError;
use crate::generate_text::execute_tool_call::{OnPreliminaryToolResult, execute_tool_call};
use crate::generate_text::is_approval_needed::is_approval_needed;
use base64::Engine;
use crate::generate_text::parse_tool_call::{parse_tool_call, parse_provider_executed_dynamic_tool_call};
use crate::generate_text::tool_approval_request_output::ToolApprovalRequestOutput;
use crate::generate_text::tool_call::TypedToolCall;
use crate::generate_text::tool_error::{DynamicToolError, TypedToolError};
use crate::generate_text::tool_output::ToolOutput;
use crate::generate_text::tool_result::{DynamicToolResult, TypedToolResult};
use crate::generate_text::ToolSet;
use crate::message::ModelMessage;
use ai_sdk_provider::language_model::stream_part::StreamPart;
use futures::stream::{Stream, StreamExt};
use serde_json::Value;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use super::single_request_text_stream_part::SingleRequestTextStreamPart;
use super::text_stream_part::StreamGeneratedFile;

/// Options for the run_tools_transformation function.
pub struct RunToolsTransformationOptions {
    /// The set of available tools (wrapped in Arc for sharing across async tasks).
    pub tools: Option<Arc<ToolSet>>,

    /// The system prompt (for tool call repair).
    pub system: Option<String>,

    /// The conversation messages for context.
    pub messages: Vec<ModelMessage>,

    /// Optional abort signal to cancel execution.
    pub abort_signal: Option<CancellationToken>,

    /// Optional experimental context data.
    pub experimental_context: Option<Value>,

    /// Function to generate unique IDs.
    pub generate_id: Arc<dyn Fn() -> String + Send + Sync>,
}

/// Transforms a stream of provider parts into a stream of single-request text stream parts,
/// handling tool execution asynchronously.
///
/// This function:
/// 1. Forwards most stream parts directly (text, reasoning, tool-input, etc.)
/// 2. Parses and validates tool calls
/// 3. Checks for tool approval requirements
/// 4. Executes tools asynchronously (if they have execute functions)
/// 5. Merges tool results back into the stream
/// 6. Delays the finish chunk until all tool results are received
///
/// # Arguments
///
/// * `generator_stream` - The input stream of provider parts
/// * `options` - Configuration options including tools, messages, etc.
///
/// # Returns
///
/// Returns a stream of `SingleRequestTextStreamPart` that includes both forwarded
/// parts and tool execution results.
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::stream_text::run_tools_transformation;
///
/// let output_stream = run_tools_transformation(provider_stream, options);
/// ```
pub fn run_tools_transformation(
    generator_stream: Pin<Box<dyn Stream<Item = StreamPart> + Send>>,
    options: RunToolsTransformationOptions,
) -> Pin<Box<dyn Stream<Item = SingleRequestTextStreamPart<Value, Value>> + Send>> {
    // Channel for tool results
    let (tool_results_tx, mut tool_results_rx) =
        mpsc::unbounded_channel::<SingleRequestTextStreamPart<Value, Value>>();

    // Track outstanding tool results
    let outstanding_tool_results = Arc::new(AtomicUsize::new(0));
    let outstanding_tool_results_clone = outstanding_tool_results.clone();

    // Track tool inputs for provider-side tool results
    let tool_inputs = Arc::new(tokio::sync::Mutex::new(HashMap::<String, Value>::new()));
    let tool_inputs_clone = tool_inputs.clone();

    // Track whether we can close
    let can_close = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let can_close_clone = can_close.clone();

    // Store the finish chunk
    let finish_chunk = Arc::new(tokio::sync::Mutex::new(
        None::<SingleRequestTextStreamPart<Value, Value>>,
    ));
    let finish_chunk_clone = finish_chunk.clone();

    // Clone tool_results_tx for use in the chain closure later
    let tool_results_tx_for_chain = tool_results_tx.clone();

    // Process the generator stream
    let forward_stream = generator_stream.then(move |chunk| {
        let tool_results_tx = tool_results_tx.clone();
        let outstanding_tool_results = outstanding_tool_results.clone();
        let tool_inputs = tool_inputs.clone();
        let can_close = can_close.clone();
        let finish_chunk = finish_chunk.clone();
        let tools = options.tools.clone();
        let system = options.system.clone();
        let messages = options.messages.clone();
        let abort_signal = options.abort_signal.clone();
        let experimental_context = options.experimental_context.clone();
        let generate_id = options.generate_id.clone();

        async move {
            match chunk {
                // Forward these chunk types directly
                StreamPart::TextStart {
                    id,
                    provider_metadata,
                } => Some(SingleRequestTextStreamPart::TextStart {
                    id,
                    provider_metadata,
                }),

                StreamPart::TextDelta {
                    id,
                    delta,
                    provider_metadata,
                } => Some(SingleRequestTextStreamPart::TextDelta {
                    id,
                    delta,
                    provider_metadata,
                }),

                StreamPart::TextEnd {
                    id,
                    provider_metadata,
                } => Some(SingleRequestTextStreamPart::TextEnd {
                    id,
                    provider_metadata,
                }),

                StreamPart::ReasoningStart {
                    id,
                    provider_metadata,
                } => Some(SingleRequestTextStreamPart::ReasoningStart {
                    id,
                    provider_metadata,
                }),

                StreamPart::ReasoningDelta {
                    id,
                    delta,
                    provider_metadata,
                } => Some(SingleRequestTextStreamPart::ReasoningDelta {
                    id,
                    delta,
                    provider_metadata,
                }),

                StreamPart::ReasoningEnd {
                    id,
                    provider_metadata,
                } => Some(SingleRequestTextStreamPart::ReasoningEnd {
                    id,
                    provider_metadata,
                }),

                StreamPart::ToolInputStart {
                    id,
                    tool_name,
                    provider_metadata,
                    provider_executed,
                } => Some(SingleRequestTextStreamPart::ToolInputStart {
                    id,
                    tool_name,
                    provider_metadata,
                    dynamic: provider_executed,
                    title: None,
                }),

                StreamPart::ToolInputDelta {
                    id,
                    delta,
                    provider_metadata,
                } => Some(SingleRequestTextStreamPart::ToolInputDelta {
                    id,
                    delta,
                    provider_metadata,
                }),

                StreamPart::ToolInputEnd {
                    id,
                    provider_metadata,
                } => Some(SingleRequestTextStreamPart::ToolInputEnd {
                    id,
                    provider_metadata,
                }),

                StreamPart::Source(source) => {
                    // Convert provider Source to SourceOutput
                    let source_output = crate::generate_text::SourceOutput::new(source);
                    Some(SingleRequestTextStreamPart::Source {
                        source: source_output,
                    })
                }

                StreamPart::ResponseMetadata(metadata) => {
                    Some(SingleRequestTextStreamPart::ResponseMetadata {
                        id: metadata.id,
                        timestamp: metadata.timestamp.map(|ts| ts.to_string()),
                        model_id: metadata.model_id,
                    })
                }

                StreamPart::Error { error } => Some(SingleRequestTextStreamPart::Error { error }),

                StreamPart::Raw { raw_value } => Some(SingleRequestTextStreamPart::Raw { raw_value }),

                StreamPart::StreamStart { warnings } => {
                    Some(SingleRequestTextStreamPart::StreamStart { warnings })
                }

                StreamPart::File(file) => {
                    // Convert provider File to StreamGeneratedFile
                    // Convert FileData to base64 string
                    use ai_sdk_provider::language_model::file::FileData;
                    let base64_data = match file.data {
                        FileData::Base64(s) => s,
                        FileData::Binary(bytes) => base64::engine::general_purpose::STANDARD.encode(&bytes),
                    };

                    Some(SingleRequestTextStreamPart::File {
                        file: StreamGeneratedFile {
                            base64: base64_data,
                            media_type: file.media_type,
                            name: None, // File doesn't have a name field
                        },
                    })
                }

                // Process finish chunk - delay until all tool results are in
                StreamPart::Finish {
                    usage,
                    finish_reason,
                    provider_metadata,
                } => {
                    let chunk = SingleRequestTextStreamPart::Finish {
                        finish_reason,
                        usage,
                        provider_metadata,
                    };

                    // Store the finish chunk
                    let mut finish_guard = finish_chunk.lock().await;
                    *finish_guard = Some(chunk);
                    drop(finish_guard);

                    // Try to send the finish chunk if all tool results are in
                    attempt_send_finish(
                        &outstanding_tool_results,
                        &can_close,
                        &finish_chunk,
                        &tool_results_tx,
                    )
                    .await;

                    None
                }

                // Process tool call
                StreamPart::ToolCall(provider_tool_call) => {
                    // Parse the tool call
                    let tool_call_result = if let Some(ref tools) = tools {
                        parse_tool_call(&provider_tool_call, tools.as_ref())
                    } else if provider_tool_call.provider_executed == Some(true) {
                        parse_provider_executed_dynamic_tool_call(&provider_tool_call)
                    } else {
                        Err(AISDKError::no_such_tool(&provider_tool_call.tool_name, vec![]))
                    };

                    let tool_call = match tool_call_result {
                        Ok(call) => call,
                        Err(err) => {
                            // Send error through tool results channel
                            let error_part = SingleRequestTextStreamPart::Error {
                                error: serde_json::json!({ "error": err.to_string() }),
                            };
                            let _ = tool_results_tx.send(error_part);
                            return None;
                        }
                    };

                    // Forward the tool call
                    let tool_call_part = SingleRequestTextStreamPart::ToolCall {
                        tool_call: tool_call.clone(),
                    };

                    // Get tool call details
                    let (tool_call_id, tool_name, input, _is_dynamic) = match &tool_call {
                        TypedToolCall::Static(call) => (
                            call.tool_call_id.clone(),
                            call.tool_name.clone(),
                            call.input.clone(),
                            false,
                        ),
                        TypedToolCall::Dynamic(call) => (
                            call.tool_call_id.clone(),
                            call.tool_name.clone(),
                            call.input.clone(),
                            true,
                        ),
                    };

                    // Store tool input
                    {
                        let mut inputs_guard = tool_inputs.lock().await;
                        inputs_guard.insert(tool_call_id.clone(), input.clone());
                    }

                    // Check if tool exists in our tool set
                    let tool = tools.as_ref().and_then(|t| t.as_ref().get(&tool_name));

                    if let Some(tool) = tool {
                        // Check if approval is needed
                        let needs_approval = is_approval_needed(
                            tool,
                            tool_call_id.clone(),
                            input.clone(),
                            messages.clone(),
                            experimental_context.clone(),
                        )
                        .await;

                        if needs_approval {
                            // Send approval request
                            let approval_id = generate_id();
                            let approval_request = ToolApprovalRequestOutput::new(approval_id, tool_call.clone());
                            let approval_part = SingleRequestTextStreamPart::ToolApprovalRequest {
                                approval_request,
                            };
                            let _ = tool_results_tx.send(approval_part);
                            return Some(tool_call_part);
                        }

                        // Execute tool if it has an execute function and is not provider-executed
                        let provider_executed = match &tool_call {
                            TypedToolCall::Static(call) => call.provider_executed == Some(true),
                            TypedToolCall::Dynamic(call) => call.provider_executed == Some(true),
                        };

                        if tool.execute.is_some() && !provider_executed {
                            // Increment outstanding tool results counter
                            outstanding_tool_results.fetch_add(1, Ordering::SeqCst);

                            // Create callback for preliminary results
                            let tool_results_tx_clone = tool_results_tx.clone();
                            let preliminary_callback: OnPreliminaryToolResult =
                                Arc::new(move |result| {
                                    let part = SingleRequestTextStreamPart::ToolResult {
                                        tool_result: result,
                                    };
                                    let _ = tool_results_tx_clone.send(part);
                                });

                            // Execute tool asynchronously
                            let tool_results_tx_clone = tool_results_tx.clone();
                            let outstanding_clone = outstanding_tool_results.clone();
                            let can_close_clone = can_close.clone();
                            let finish_chunk_clone = finish_chunk.clone();
                            let tools_clone = tools.clone();
                            let messages_clone = messages.clone();
                            let abort_signal_clone = abort_signal.clone();
                            let experimental_context_clone = experimental_context.clone();

                            tokio::spawn(async move {
                                let result = execute_tool_call(
                                    tool_call,
                                    tools_clone.as_ref().unwrap().as_ref(),
                                    messages_clone,
                                    abort_signal_clone,
                                    experimental_context_clone,
                                    Some(preliminary_callback),
                                )
                                .await;

                                // Send the final result
                                if let Some(output) = result {
                                    let part = match output {
                                        ToolOutput::Result(result) => {
                                            SingleRequestTextStreamPart::ToolResult {
                                                tool_result: result,
                                            }
                                        }
                                        ToolOutput::Error(error) => {
                                            SingleRequestTextStreamPart::ToolError { tool_error: error }
                                        }
                                    };
                                    let _ = tool_results_tx_clone.send(part);
                                }

                                // Decrement outstanding tool results
                                outstanding_clone.fetch_sub(1, Ordering::SeqCst);

                                // Try to send finish chunk
                                attempt_send_finish(
                                    &outstanding_clone,
                                    &can_close_clone,
                                    &finish_chunk_clone,
                                    &tool_results_tx_clone,
                                )
                                .await;
                            });
                        }
                    }

                    Some(tool_call_part)
                }

                // Process tool result from provider
                StreamPart::ToolResult(provider_result) => {
                    let tool_name = provider_result.tool_name.clone();
                    let tool_call_id = provider_result.tool_call_id.clone();

                    // Get the stored input
                    let input = {
                        let inputs_guard = tool_inputs.lock().await;
                        inputs_guard.get(&tool_call_id).cloned()
                    };

                    if provider_result.is_error.unwrap_or(false) {
                        // Error result
                        let error = TypedToolError::Dynamic(DynamicToolError::new(
                            &tool_call_id,
                            &tool_name,
                            input.unwrap_or(Value::Null),
                            provider_result.result,
                        ).with_provider_executed(provider_result.provider_executed.unwrap_or(false)));

                        Some(SingleRequestTextStreamPart::ToolError { tool_error: error })
                    } else {
                        // Success result
                        let result = TypedToolResult::Dynamic(DynamicToolResult::new(
                            &tool_call_id,
                            &tool_name,
                            input.unwrap_or(Value::Null),
                            provider_result.result,
                        ).with_provider_executed(provider_result.provider_executed.unwrap_or(false)));

                        Some(SingleRequestTextStreamPart::ToolResult {
                            tool_result: result,
                        })
                    }
                }
            }
        }
    });

    // Filter out None values
    let forward_stream = forward_stream.filter_map(|item| async move { item });

    // Merge the forward stream and tool results stream
    let combined_stream = futures::stream::select(
        forward_stream,
        tokio_stream::wrappers::UnboundedReceiverStream::new(tool_results_rx),
    );

    // Mark that we can close after the forward stream is done
    let combined_stream = combined_stream.chain(futures::stream::once(async move {
        can_close_clone.store(true, Ordering::SeqCst);

        // Try to send finish chunk one last time
        attempt_send_finish(
            &outstanding_tool_results_clone,
            &can_close_clone,
            &finish_chunk_clone,
            &tool_results_tx_for_chain,
        )
        .await;

        // Return a dummy value that will be filtered out
        SingleRequestTextStreamPart::Error {
            error: Value::Null,
        }
    }))
    .filter(|part| {
        let should_include = !matches!(
            part,
            SingleRequestTextStreamPart::Error { error } if matches!(error, Value::Null)
        );
        async move { should_include }
    });

    Box::pin(combined_stream)
}

/// Attempts to send the finish chunk if all conditions are met.
async fn attempt_send_finish(
    outstanding_tool_results: &Arc<AtomicUsize>,
    can_close: &Arc<std::sync::atomic::AtomicBool>,
    finish_chunk: &Arc<tokio::sync::Mutex<Option<SingleRequestTextStreamPart<Value, Value>>>>,
    tool_results_tx: &mpsc::UnboundedSender<SingleRequestTextStreamPart<Value, Value>>,
) {
    let outstanding = outstanding_tool_results.load(Ordering::SeqCst);
    let can_close = can_close.load(Ordering::SeqCst);

    if can_close && outstanding == 0 {
        let mut finish_guard = finish_chunk.lock().await;
        if let Some(chunk) = finish_guard.take() {
            let _ = tool_results_tx.send(chunk);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_sdk_provider::language_model::finish_reason::FinishReason;
    use ai_sdk_provider::language_model::usage::Usage;
    use futures::stream;

    #[tokio::test]
    async fn test_forward_text_chunks() {
        let chunks = vec![
            StreamPart::text_start("text_1"),
            StreamPart::text_delta("text_1", "Hello"),
            StreamPart::text_end("text_1"),
        ];

        let generator_stream = Box::pin(stream::iter(chunks));

        let options = RunToolsTransformationOptions {
            tools: None,
            system: None,
            messages: vec![],
            abort_signal: None,
            experimental_context: None,
            generate_id: Arc::new(|| "test_id".to_string()),
        };

        let mut output_stream = run_tools_transformation(generator_stream, options);

        let part1 = output_stream.next().await.unwrap();
        assert!(matches!(part1, SingleRequestTextStreamPart::TextStart { .. }));

        let part2 = output_stream.next().await.unwrap();
        assert!(matches!(part2, SingleRequestTextStreamPart::TextDelta { .. }));

        let part3 = output_stream.next().await.unwrap();
        assert!(matches!(part3, SingleRequestTextStreamPart::TextEnd { .. }));
    }

    #[tokio::test]
    async fn test_finish_chunk_delay() {
        let chunks = vec![
            StreamPart::text_delta("text_1", "Hello"),
            StreamPart::finish(Usage::new(10, 5), FinishReason::Stop),
        ];

        let generator_stream = Box::pin(stream::iter(chunks));

        let options = RunToolsTransformationOptions {
            tools: None,
            system: None,
            messages: vec![],
            abort_signal: None,
            experimental_context: None,
            generate_id: Arc::new(|| "test_id".to_string()),
        };

        let mut output_stream = run_tools_transformation(generator_stream, options);

        // Should get text delta
        let part1 = output_stream.next().await.unwrap();
        assert!(matches!(part1, SingleRequestTextStreamPart::TextDelta { .. }));

        // Should get finish chunk (no tool executions to wait for)
        let part2 = output_stream.next().await.unwrap();
        assert!(matches!(part2, SingleRequestTextStreamPart::Finish { .. }));
    }
}
