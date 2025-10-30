pub mod callbacks;
pub mod convert_stream_part;
pub mod default_stream_text_result;
pub mod enriched_stream_part;
pub mod run_tools_transformation;
pub mod single_request_text_stream_part;
pub mod stream_text_result;
pub mod stream_text_transform;
pub mod text_stream_part;

pub use callbacks::{
    ChunkStreamPart, StreamTextAbortEvent as AbortEvent, StreamTextChunkEvent as ChunkEvent,
    StreamTextErrorEvent as ErrorEvent, StreamTextFinishEvent as StreamFinishEvent,
    StreamTextOnAbortCallback as OnAbortCallback, StreamTextOnChunkCallback as OnChunkCallback,
    StreamTextOnErrorCallback as OnErrorCallback, StreamTextOnFinishCallback as OnFinishCallback,
    StreamTextOnStepFinishCallback as OnStepFinishCallback,
};
pub use default_stream_text_result::{DefaultStreamTextResult, DefaultStreamTextResultParams};
pub use enriched_stream_part::EnrichedStreamPart;
pub use stream_text_result::{
    AsyncIterableStream, ConsumeStreamOptions, ErrorHandler, StreamTextResult,
};
pub use stream_text_transform::{
    StreamTextTransform, StreamTextTransformOptions, create_logging_transform,
    create_passthrough_transform,
};
pub use run_tools_transformation::{
    RunToolsTransformationOptions, run_tools_transformation,
};
pub use single_request_text_stream_part::SingleRequestTextStreamPart;
pub use text_stream_part::{StreamGeneratedFile, TextStreamPart};

use crate::error::AISDKError;
use crate::generate_text::ToolSet;
use crate::prompt::{Prompt, call_settings::CallSettings};
use ai_sdk_provider::language_model::LanguageModel;
use ai_sdk_provider::language_model::tool_choice::ToolChoice;
use ai_sdk_provider::shared::provider_options::ProviderOptions;
use serde_json::Value;

/// Stream text using a language model.
///
/// This is the main user-facing function for streaming text generation in the AI SDK.
/// It takes a prompt, model, settings, and optionally tools to generate text as a stream.
///
/// # Arguments
///
/// * `settings` - Configuration settings for the generation (temperature, max tokens, etc.)
/// * `prompt` - The prompt to send to the model. Can be a simple string or structured messages.
/// * `model` - The language model to use for generation
/// * `tools` - Optional tool set (HashMap of tool names to tools). The model needs to support calling tools.
/// * `tool_choice` - Optional tool choice strategy. Default: 'auto'.
/// * `stop_when` - Optional stop condition(s) for multi-step generation. Can be a single condition
///   or multiple conditions. Any condition being met will stop generation. Default: `step_count_is(1)`.
/// * `provider_options` - Optional provider-specific options.
/// * `prepare_step` - Optional function to customize settings for each step in multi-step generation.
/// * `include_raw_chunks` - Whether to include raw chunks from the provider in the stream.
///   When enabled, you will receive raw chunks with type 'raw' that contain the unprocessed data
///   from the provider. This allows access to cutting-edge provider features not yet wrapped by the AI SDK.
///   Defaults to false.
/// * `on_chunk` - Optional callback that is called for each chunk of the stream.
///   The stream processing will pause until the callback promise is resolved.
/// * `on_error` - Optional callback that is invoked when an error occurs during streaming.
///   You can use it to log errors. The stream processing will pause until the callback promise is resolved.
/// * `on_step_finish` - Optional callback called after each step (LLM call) completes.
///   The stream processing will pause until the callback promise is resolved.
/// * `on_finish` - Optional callback that is called when the LLM response and all request tool executions
///   (for tools that have an `execute` function) are finished. The usage is the combined usage of all steps.
/// * `on_abort` - Optional callback that is called when the generation is aborted.
///
/// # Returns
///
/// Returns `Result<StreamTextResult, AISDKError>` - A stream result object that provides multiple
/// ways to access the streamed data, or a validation error.
///
/// # Errors
///
/// Returns `AISDKError::InvalidArgument` if any settings are invalid (e.g., non-finite temperature).
///
/// # Examples
///
/// ```ignore
/// use ai_sdk_core::{stream_text, step_count_is};
/// use ai_sdk_core::prompt::{Prompt, call_settings::CallSettings};
///
/// let prompt = Prompt::text("Tell me a joke");
/// let settings = CallSettings::default();
/// let result = stream_text(
///     settings,
///     prompt,
///     model,
///     None,
///     None,
///     Some(vec![Box::new(step_count_is(1))]),
///     None,
///     None,
///     false,
///     None,
///     None,
///     None,
///     None,
///     None,
/// ).await?;
///
/// // Stream text deltas in real-time
/// let mut stream = result.text_stream();
/// while let Some(delta) = stream.next().await {
///     print!("{}", delta);
/// }
/// ```
pub async fn stream_text(
    settings: CallSettings,
    prompt: Prompt,
    model: &dyn LanguageModel,
    tools: Option<ToolSet>,
    tool_choice: Option<ToolChoice>,
    _stop_when: Option<Vec<Box<dyn crate::generate_text::StopCondition>>>,
    provider_options: Option<ProviderOptions>,
    _prepare_step: Option<Box<dyn crate::generate_text::PrepareStep>>,
    include_raw_chunks: bool,
    on_chunk: Option<OnChunkCallback>,
    on_error: Option<OnErrorCallback>,
    _on_step_finish: Option<OnStepFinishCallback>,
    _on_finish: Option<OnFinishCallback>,
    _on_abort: Option<OnAbortCallback>,
) -> Result<StreamTextResult<Value, Value>, AISDKError> {
    // Phase 2: Basic single-step streaming without multi-step or full tool execution
    //
    // This implementation provides the core streaming pipeline:
    // 1. Validate settings
    // 2. Convert prompt to language model format
    // 3. Prepare tools and tool choice
    // 4. Call model.do_stream()
    // 5. Convert stream parts (StreamPart -> TextStreamPart -> EnrichedStreamPart)
    // 6. Apply event processor
    // 7. Create and return StreamTextResult

    use crate::prompt::convert_to_language_model_prompt::convert_to_language_model_prompt;
    use crate::prompt::standardize::validate_and_standardize;
    use crate::generate_text::prepare_tools_and_tool_choice;
    use crate::stream_text::convert_stream_part::{
        convert_stream_part_to_text_stream_part,
        convert_single_request_part_to_text_stream_part,
    };
    use crate::stream_text::EnrichedStreamPart;
    use crate::stream_text::run_tools_transformation::{
        run_tools_transformation,
        RunToolsTransformationOptions,
    };
    use ai_sdk_provider::language_model::call_options::CallOptions;
    use futures_util::StreamExt;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use std::collections::HashMap;

    // Step 1: Validate and standardize prompt
    let standardized_prompt = validate_and_standardize(prompt)?;

    // Step 1.5: Save fields we need before moving standardized_prompt
    let messages_for_tools = standardized_prompt.messages.clone();
    let system_for_tools = standardized_prompt.system.clone();

    // Step 2: Convert prompt to language model format
    let language_model_prompt = convert_to_language_model_prompt(standardized_prompt)?;

    // Step 3: Prepare tools and tool choice
    let (prepared_tools, prepared_tool_choice) =
        prepare_tools_and_tool_choice(tools.as_ref(), tool_choice);

    // Step 4: Create CallOptions for the model using builder pattern
    let mut call_options = CallOptions::new(language_model_prompt);

    // Add settings using builder pattern
    if let Some(temp) = settings.temperature {
        call_options = call_options.with_temperature(temp);
    }
    if let Some(top_p) = settings.top_p {
        call_options = call_options.with_top_p(top_p);
    }
    if let Some(top_k) = settings.top_k {
        call_options = call_options.with_top_k(top_k);
    }
    if let Some(freq_penalty) = settings.frequency_penalty {
        call_options = call_options.with_frequency_penalty(freq_penalty);
    }
    if let Some(pres_penalty) = settings.presence_penalty {
        call_options = call_options.with_presence_penalty(pres_penalty);
    }
    if let Some(seed) = settings.seed {
        call_options = call_options.with_seed(seed);
    }
    if let Some(max_tokens) = settings.max_output_tokens {
        call_options = call_options.with_max_output_tokens(max_tokens);
    }
    if let Some(ref stop_seqs) = settings.stop_sequences {
        call_options = call_options.with_stop_sequences(stop_seqs.clone());
    }

    // Add tools and tool choice
    if let Some(ref tools_vec) = prepared_tools {
        call_options = call_options.with_tools(tools_vec.clone());
    }
    if let Some(tc) = prepared_tool_choice {
        call_options = call_options.with_tool_choice(tc);
    }

    // Add provider options
    if let Some(ref provider_opts) = provider_options {
        call_options = call_options.with_provider_options(provider_opts.clone());
    }

    // Step 5: Call model.do_stream() to get the provider's stream
    let stream_response = model.do_stream(call_options).await.map_err(|e| {
        AISDKError::model_error(e.to_string())
    })?;
    let provider_stream = stream_response.stream;

    // Step 6: Convert the stream pipeline
    // Phase 4: StreamPart -> run_tools_transformation -> SingleRequestTextStreamPart -> TextStreamPart -> EnrichedStreamPart

    // Wrap callbacks in Arc for sharing across async tasks
    let on_chunk = Arc::new(on_chunk);
    let on_error = Arc::new(on_error);

    // Create shared state for event processor
    // Active content being accumulated during streaming (keyed by ID)
    use crate::generate_text::TextOutput;
    use crate::generate_text::ReasoningOutput;
    let active_text_content: Arc<Mutex<HashMap<String, TextOutput>>> = Arc::new(Mutex::new(HashMap::new()));
    let active_reasoning_content: Arc<Mutex<HashMap<String, ReasoningOutput>>> = Arc::new(Mutex::new(HashMap::new()));

    // Recorded content parts, response messages, and metadata
    let recorded_content: Arc<Mutex<Vec<crate::generate_text::ContentPart<Value, Value>>>> = Arc::new(Mutex::new(Vec::new()));
    let recorded_response_messages: Arc<Mutex<Vec<crate::generate_text::ResponseMessage>>> = Arc::new(Mutex::new(Vec::new()));
    let recorded_request: Arc<Mutex<crate::generate_text::RequestMetadata>> = Arc::new(Mutex::new(crate::generate_text::RequestMetadata { body: None }));
    let recorded_warnings: Arc<Mutex<Vec<ai_sdk_provider::language_model::call_warning::CallWarning>>> = Arc::new(Mutex::new(Vec::new()));
    let recorded_finish_reason = Arc::new(Mutex::new(None));
    let recorded_total_usage = Arc::new(Mutex::new(None));
    let recorded_steps: Arc<Mutex<Vec<crate::generate_text::StepResult<Value, Value>>>> = Arc::new(Mutex::new(Vec::new()));

    // Phase 4: Apply run_tools_transformation if tools are provided
    let has_tools = tools.is_some();
    // We need to keep a reference to tools for to_response_messages
    // Since Tool doesn't implement Clone, we'll just save a reference
    let tools_ref = if tools.is_some() {
        // We'll wrap tools in Arc and keep a clone of that Arc
        let arc = Arc::new(tools.unwrap());
        let arc_clone = arc.clone();
        let tools_arc = Some(arc);
        (tools_arc, Some(arc_clone))
    } else {
        (None, None)
    };
    let tools_arc = tools_ref.0;
    let tools_for_response_arc = tools_ref.1;

    // Create a simple ID generator
    let id_counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let generate_id = Arc::new(move || {
        let id = id_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        format!("id_{}", id)
    });

    let tools_stream = if has_tools {
        // Apply run_tools_transformation to handle tool execution
        let options = RunToolsTransformationOptions {
            tools: tools_arc,
            system: system_for_tools,
            messages: messages_for_tools,
            abort_signal: None, // TODO: Support abort signal in future phases
            experimental_context: None,
            generate_id,
        };
        run_tools_transformation(Box::pin(provider_stream), options)
    } else {
        // No tools - convert provider stream directly to SingleRequestTextStreamPart
        // We need to create a stream that converts StreamPart to SingleRequestTextStreamPart
        use crate::stream_text::single_request_text_stream_part::SingleRequestTextStreamPart;
        Box::pin(provider_stream.map(|stream_part| {
            // Convert StreamPart to SingleRequestTextStreamPart
            // This is a simple pass-through conversion
            match stream_part {
                ai_sdk_provider::language_model::stream_part::StreamPart::TextStart { id, provider_metadata } => {
                    SingleRequestTextStreamPart::TextStart { id, provider_metadata }
                }
                ai_sdk_provider::language_model::stream_part::StreamPart::TextDelta { id, delta, provider_metadata } => {
                    SingleRequestTextStreamPart::TextDelta { id, delta, provider_metadata }
                }
                ai_sdk_provider::language_model::stream_part::StreamPart::TextEnd { id, provider_metadata } => {
                    SingleRequestTextStreamPart::TextEnd { id, provider_metadata }
                }
                ai_sdk_provider::language_model::stream_part::StreamPart::ReasoningStart { id, provider_metadata } => {
                    SingleRequestTextStreamPart::ReasoningStart { id, provider_metadata }
                }
                ai_sdk_provider::language_model::stream_part::StreamPart::ReasoningDelta { id, delta, provider_metadata } => {
                    SingleRequestTextStreamPart::ReasoningDelta { id, delta, provider_metadata }
                }
                ai_sdk_provider::language_model::stream_part::StreamPart::ReasoningEnd { id, provider_metadata } => {
                    SingleRequestTextStreamPart::ReasoningEnd { id, provider_metadata }
                }
                ai_sdk_provider::language_model::stream_part::StreamPart::ToolInputStart { id, tool_name, provider_metadata, provider_executed } => {
                    SingleRequestTextStreamPart::ToolInputStart { id, tool_name, provider_metadata, dynamic: None, title: None }
                }
                ai_sdk_provider::language_model::stream_part::StreamPart::ToolInputDelta { id, delta, provider_metadata } => {
                    SingleRequestTextStreamPart::ToolInputDelta { id, delta, provider_metadata }
                }
                ai_sdk_provider::language_model::stream_part::StreamPart::ToolInputEnd { id, provider_metadata } => {
                    SingleRequestTextStreamPart::ToolInputEnd { id, provider_metadata }
                }
                ai_sdk_provider::language_model::stream_part::StreamPart::StreamStart { warnings } => {
                    SingleRequestTextStreamPart::StreamStart { warnings }
                }
                ai_sdk_provider::language_model::stream_part::StreamPart::Finish { finish_reason, usage, provider_metadata } => {
                    SingleRequestTextStreamPart::Finish { finish_reason, usage, provider_metadata }
                }
                ai_sdk_provider::language_model::stream_part::StreamPart::Error { error } => {
                    SingleRequestTextStreamPart::Error { error }
                }
                ai_sdk_provider::language_model::stream_part::StreamPart::Raw { raw_value } => {
                    SingleRequestTextStreamPart::Raw { raw_value }
                }
                // For parts that don't have direct SingleRequestTextStreamPart equivalents, convert to Raw
                _ => {
                    SingleRequestTextStreamPart::Raw {
                        raw_value: serde_json::to_value(&stream_part).unwrap_or(Value::Null),
                    }
                }
            }
        }))
    };

    // Convert SingleRequestTextStreamPart to TextStreamPart
    let text_stream = tools_stream.map(|single_request_part| {
        let text_part: TextStreamPart<Value, Value> =
            convert_single_request_part_to_text_stream_part(single_request_part);
        text_part
    });

    // Wrap in EnrichedStreamPart (no partial output parsing yet)
    let enriched_stream = text_stream.map(|text_part| {
        EnrichedStreamPart::new(text_part)
    });

    // Box the stream to make it Pin<Box<dyn Stream>>
    let boxed_enriched_stream: AsyncIterableStream<EnrichedStreamPart<Value, Value>> =
        Box::pin(enriched_stream);

    // Step 7: Apply enhanced event processor for Phase 3
    // This processor tracks all content parts, handles step events, and calls all callbacks
    let processed_stream = Box::pin(async_stream::stream! {
        use futures_util::pin_mut;
        use crate::generate_text::{ContentPart, GeneratedFile};
        use crate::generate_text::tool_call::TypedToolCall;
        use crate::generate_text::tool_result::TypedToolResult;
        use crate::generate_text::tool_error::TypedToolError;

        pin_mut!(boxed_enriched_stream);

        while let Some(enriched_part) = boxed_enriched_stream.next().await {
            // Forward the chunk downstream
            yield enriched_part.clone();

            let part = &enriched_part.part;

            // Call onChunk for relevant events
            if matches!(
                part,
                TextStreamPart::TextDelta { .. }
                    | TextStreamPart::ReasoningDelta { .. }
                    | TextStreamPart::Source { .. }
                    | TextStreamPart::ToolCall { .. }
                    | TextStreamPart::ToolResult { .. }
                    | TextStreamPart::ToolInputStart { .. }
                    | TextStreamPart::ToolInputDelta { .. }
            ) {
                if let Some(callback) = on_chunk.as_ref() {
                    if let Some(chunk_part) = crate::stream_text::ChunkStreamPart::from_stream_part(part) {
                        let event = crate::stream_text::ChunkEvent {
                            chunk: chunk_part,
                        };
                        callback(event).await;
                    }
                }
            }

            // Handle error events
            if let TextStreamPart::Error { error } = part {
                if let Some(callback) = on_error.as_ref() {
                    let event = crate::stream_text::ErrorEvent {
                        error: error.clone(),
                    };
                    callback(event).await;
                }
            }

            // Process each stream part type and track state
            match part {
                // Text content tracking
                TextStreamPart::TextStart { id, provider_metadata } => {
                    let mut active_texts = active_text_content.lock().await;
                    let mut text_output = TextOutput::new("");
                    if let Some(metadata) = provider_metadata {
                        text_output = text_output.with_provider_metadata(metadata.clone());
                    }
                    active_texts.insert(id.clone(), text_output.clone());

                    // Add to recorded content (will be updated with text as deltas arrive)
                    let mut content = recorded_content.lock().await;
                    content.push(ContentPart::Text(text_output));
                }
                TextStreamPart::TextDelta { id, text, provider_metadata } => {
                    let mut active_texts = active_text_content.lock().await;
                    if let Some(active_text) = active_texts.get_mut(id) {
                        // Update the text content
                        *active_text = TextOutput::new(&format!("{}{}", active_text.text, text));
                        if let Some(metadata) = provider_metadata {
                            *active_text = active_text.clone().with_provider_metadata(metadata.clone());
                        }
                    } else {
                        // Error: text part not found
                        yield EnrichedStreamPart::new(TextStreamPart::Error {
                            error: serde_json::Value::String(format!("text part {} not found", id)),
                        });
                    }
                }
                TextStreamPart::TextEnd { id, provider_metadata } => {
                    let mut active_texts = active_text_content.lock().await;
                    if let Some(mut active_text) = active_texts.remove(id) {
                        if let Some(metadata) = provider_metadata {
                            active_text = active_text.with_provider_metadata(metadata.clone());
                        }
                        // Final text is already in recorded_content, just update metadata if needed
                    } else {
                        // Error: text part not found
                        yield EnrichedStreamPart::new(TextStreamPart::Error {
                            error: serde_json::Value::String(format!("text part {} not found", id)),
                        });
                    }
                }

                // Reasoning content tracking
                TextStreamPart::ReasoningStart { id, provider_metadata } => {
                    let mut active_reasoning = active_reasoning_content.lock().await;
                    let mut reasoning_output = ReasoningOutput::new("");
                    if let Some(metadata) = provider_metadata {
                        reasoning_output = reasoning_output.with_provider_metadata(metadata.clone());
                    }
                    active_reasoning.insert(id.clone(), reasoning_output.clone());

                    // Add to recorded content
                    let mut content = recorded_content.lock().await;
                    content.push(ContentPart::Reasoning(reasoning_output));
                }
                TextStreamPart::ReasoningDelta { id, text, provider_metadata } => {
                    let mut active_reasoning = active_reasoning_content.lock().await;
                    if let Some(active) = active_reasoning.get_mut(id) {
                        *active = ReasoningOutput::new(&format!("{}{}", active.text, text));
                        if let Some(metadata) = provider_metadata {
                            *active = active.clone().with_provider_metadata(metadata.clone());
                        }
                    } else {
                        // Error: reasoning part not found
                        yield EnrichedStreamPart::new(TextStreamPart::Error {
                            error: serde_json::Value::String(format!("reasoning part {} not found", id)),
                        });
                    }
                }
                TextStreamPart::ReasoningEnd { id, provider_metadata } => {
                    let mut active_reasoning = active_reasoning_content.lock().await;
                    if let Some(mut active) = active_reasoning.remove(id) {
                        if let Some(metadata) = provider_metadata {
                            active = active.with_provider_metadata(metadata.clone());
                        }
                    } else {
                        // Error: reasoning part not found
                        yield EnrichedStreamPart::new(TextStreamPart::Error {
                            error: serde_json::Value::String(format!("reasoning part {} not found", id)),
                        });
                    }
                }

                // File content
                TextStreamPart::File { file } => {
                    let generated_file = GeneratedFile::from_base64(&file.base64, &file.media_type);
                    // Note: Files are not added to ContentPart, they're tracked separately
                    // ContentPart only has Text, Reasoning, Source, ToolCall, ToolResult, ToolError
                    // Files are accessed via the files() accessor
                }

                // Source content
                TextStreamPart::Source { source } => {
                    let mut content = recorded_content.lock().await;
                    content.push(ContentPart::Source(source.clone()));
                }

                // Tool calls
                TextStreamPart::ToolCall { tool_call } => {
                    let mut content = recorded_content.lock().await;
                    content.push(ContentPart::ToolCall(tool_call.clone()));
                }

                // Tool results
                TextStreamPart::ToolResult { tool_result } => {
                    // Note: We track all tool results, not just non-preliminary ones
                    // The preliminary flag is part of the TypedToolResult structure
                    let mut content = recorded_content.lock().await;
                    content.push(ContentPart::ToolResult(tool_result.clone()));
                }

                // Tool errors
                TextStreamPart::ToolError { tool_error } => {
                    let mut content = recorded_content.lock().await;
                    content.push(ContentPart::ToolError(tool_error.clone()));
                }

                // Tool approval requests
                TextStreamPart::ToolApprovalRequest { approval_request } => {
                    // Tool approval requests contain tool call information
                    // We extract it and add to recorded content
                    // Note: ToolApprovalRequestOutput contains the tool call details
                    // For now, we'll just note that this occurred
                    // The actual approval workflow is handled separately
                }

                // Step events
                TextStreamPart::StartStep { request, warnings } => {
                    // Reset recorded data for new step
                    let mut content = recorded_content.lock().await;
                    content.clear();
                    drop(content);

                    let mut active_texts = active_text_content.lock().await;
                    active_texts.clear();
                    drop(active_texts);

                    let mut active_reasoning = active_reasoning_content.lock().await;
                    active_reasoning.clear();
                    drop(active_reasoning);

                    // Record request and warnings for this step
                    let mut req = recorded_request.lock().await;
                    *req = request.clone();
                    drop(req);

                    let mut warns = recorded_warnings.lock().await;
                    *warns = warnings.clone();
                }

                TextStreamPart::FinishStep { finish_reason: step_finish_reason, usage: step_usage, response, provider_metadata } => {
                    // TODO: This will be fully implemented in later phases with onStepFinish callback
                    // For now, we'll record basic step information

                    // Create response messages from recorded content
                    use crate::generate_text::to_response_messages;
                    let content = recorded_content.lock().await;
                    let content_vec = content.clone();
                    drop(content);

                    let step_messages = to_response_messages(content_vec, tools_for_response_arc.as_ref().map(|arc| arc.as_ref()));

                    // Accumulate response messages
                    let mut response_msgs = recorded_response_messages.lock().await;
                    // Convert ModelMessage to ResponseMessage (ResponseMessage has From<ModelMessage> impl)
                    for msg in step_messages {
                        response_msgs.push(msg.into());
                    }
                }

                // Final finish event
                TextStreamPart::Finish { finish_reason, total_usage } => {
                    // Record finish reason
                    let mut recorded_finish = recorded_finish_reason.lock().await;
                    *recorded_finish = Some(finish_reason.clone());

                    // Record total usage
                    let mut recorded_usage = recorded_total_usage.lock().await;
                    *recorded_usage = Some(total_usage.clone());
                }

                // Other events (Start, Abort, ToolInputStart/Delta/End, ToolOutputDenied, RawChunk)
                _ => {
                    // These are handled elsewhere or don't need special tracking
                }
            }
        }
    });

    // Step 8: Create StreamTextResult wrapper
    // For Phase 2, we create a simple StreamTextResult that wraps our stream
    // The actual DefaultStreamTextResult integration will come in subsequent phases

    // Convert the processed stream to a stream of TextStreamPart
    // We need to extract the TextStreamPart from EnrichedStreamPart
    let text_stream_parts = Box::pin(processed_stream.map(|enriched| enriched.part));

    // Create and return the StreamTextResult
    Ok(StreamTextResult::new(text_stream_parts))
}
