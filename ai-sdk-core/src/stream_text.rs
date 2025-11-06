pub mod callbacks;
pub mod output;
pub mod stream_text_result;
pub mod text_stream_part;
pub mod transform;

pub use callbacks::{
    ChunkStreamPart, StreamTextAbortEvent as AbortEvent, StreamTextChunkEvent as ChunkEvent,
    StreamTextErrorEvent as ErrorEvent, StreamTextFinishEvent as StreamFinishEvent,
    StreamTextOnAbortCallback as OnAbortCallback, StreamTextOnChunkCallback as OnChunkCallback,
    StreamTextOnErrorCallback as OnErrorCallback, StreamTextOnFinishCallback as OnFinishCallback,
    StreamTextOnStepFinishCallback as OnStepFinishCallback,
};
pub use stream_text_result::{
    AsyncIterableStream, ConsumeStreamOptions, ErrorHandler, StreamTextResult,
};
pub use text_stream_part::{StreamGeneratedFile, TextStreamPart};
pub use transform::{
    BatchTextTransform, FilterTransform, MapTransform, StopStreamHandle, StreamTransform,
    ThrottleTransform, TransformOptions, batch_text_transform, filter_transform, map_transform,
    throttle_transform,
};

use crate::error::AISDKError;
use crate::generate_text::{
    Output, PrepareStep, PrepareStepOptions, StepResult, StopCondition,
    TypedToolCall, execute_tool_call, is_stop_condition_met, prepare_tools_and_tool_choice,
    to_response_messages,
};
use crate::prompt::{
    Prompt, call_settings::CallSettings, call_settings::prepare_call_settings,
    convert_to_language_model_prompt::convert_to_language_model_prompt,
    standardize::StandardizedPrompt, standardize::validate_and_standardize,
};
use ai_sdk_provider::language_model::call_options::LanguageModelCallOptions;
use ai_sdk_provider::language_model::{
    LanguageModel, finish_reason::LanguageModelFinishReason, tool_choice::LanguageModelToolChoice,
    usage::LanguageModelUsage,
};
use ai_sdk_provider::shared::provider_options::SharedProviderOptions;
use serde_json::Value;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::tool::ToolSet;

/// Result of streaming a single step.
///
/// This contains all the accumulated data from processing one streaming call to the model.
struct SingleStepStreamResult {
    /// The content parts accumulated during the step
    content: Vec<Output<Value, Value>>,

    /// Tool calls made during the step
    tool_calls: Vec<TypedToolCall<Value>>,

    /// Finish reason for the step
    finish_reason: LanguageModelFinishReason,

    /// Usage for this step
    usage: LanguageModelUsage,

    /// Request metadata
    request: crate::generate_text::RequestMetadata,

    /// Response metadata
    response: crate::generate_text::StepResponseMetadata,

    /// Provider metadata
    provider_metadata: Option<ai_sdk_provider::shared::provider_metadata::SharedProviderMetadata>,

    /// Warnings from the provider
    warnings: Option<Vec<ai_sdk_provider::language_model::call_warning::LanguageModelCallWarning>>,
}

/// Streams a single step and accumulates the results.
///
/// This helper function handles one streaming call to the model and processes all the
/// stream parts, accumulating tool calls and content. It emits stream parts to the channel
/// as they arrive.
async fn stream_single_step(
    model: Arc<dyn LanguageModel>,
    call_options: ai_sdk_provider::language_model::call_options::LanguageModelCallOptions,
    tools: Option<&ToolSet>,
    include_raw_chunks: bool,
    tx: &mpsc::UnboundedSender<TextStreamPart<Value, Value>>,
    on_chunk: Option<&Arc<OnChunkCallback>>,
    on_error: Option<&Arc<OnErrorCallback>>,
) -> Result<SingleStepStreamResult, AISDKError> {
    use crate::generate_text::{ReasoningOutput, SourceOutput, TextOutput};
    use ai_sdk_provider::language_model::stream_part::LanguageModelStreamPart;
    use futures_util::StreamExt;

    // Call model.do_stream
    let stream_response = model
        .do_stream(call_options)
        .await
        .map_err(|e| AISDKError::model_error(e.to_string()))?;

    // Extract metadata before moving stream
    let request_body = stream_response
        .request
        .as_ref()
        .and_then(|r| r.body.clone());

    let mut provider_stream = stream_response.stream;

    // Accumulate step data
    let mut step_content: Vec<Output<Value, Value>> = Vec::new();
    let mut step_tool_calls: Vec<TypedToolCall<Value>> = Vec::new();
    let mut current_text = String::new();
    let mut current_reasoning = String::new();
    let step_request = crate::generate_text::RequestMetadata {
        body: request_body.clone(),
    };
    let step_response = crate::generate_text::StepResponseMetadata::default();
    let mut step_warnings = None;
    let mut step_finish_reason = LanguageModelFinishReason::Unknown;
    let mut step_usage = LanguageModelUsage::default();
    let mut step_provider_metadata = None;

    while let Some(part) = provider_stream.next().await {
        // Convert StreamPart to TextStreamPart
        let text_stream_part = match part {
            LanguageModelStreamPart::TextStart(ts) => TextStreamPart::TextStart {
                id: ts.id,
                provider_metadata: ts.provider_metadata,
            },
            LanguageModelStreamPart::TextDelta(td) => {
                current_text.push_str(&td.delta);
                TextStreamPart::TextDelta {
                    id: td.id,
                    provider_metadata: td.provider_metadata,
                    text: td.delta,
                }
            }
            LanguageModelStreamPart::TextEnd(te) => {
                // Add accumulated text to content
                if !current_text.is_empty() {
                    step_content.push(Output::Text(TextOutput::new(current_text.clone())));
                    current_text.clear();
                }
                TextStreamPart::TextEnd {
                    id: te.id,
                    provider_metadata: te.provider_metadata,
                }
            }
            LanguageModelStreamPart::ReasoningStart(rs) => TextStreamPart::ReasoningStart {
                id: rs.id,
                provider_metadata: rs.provider_metadata,
            },
            LanguageModelStreamPart::ReasoningDelta(rd) => {
                current_reasoning.push_str(&rd.delta);
                TextStreamPart::ReasoningDelta {
                    id: rd.id,
                    provider_metadata: rd.provider_metadata,
                    text: rd.delta,
                }
            }
            LanguageModelStreamPart::ReasoningEnd(re) => {
                // Add accumulated reasoning to content
                if !current_reasoning.is_empty() {
                    step_content.push(Output::Reasoning(ReasoningOutput::new(
                        current_reasoning.clone(),
                    )));
                    current_reasoning.clear();
                }
                TextStreamPart::ReasoningEnd {
                    id: re.id,
                    provider_metadata: re.provider_metadata,
                }
            }
            LanguageModelStreamPart::ToolInputStart(tis) => TextStreamPart::ToolInputStart {
                id: tis.id,
                tool_name: tis.tool_name,
                provider_metadata: tis.provider_metadata,
                provider_executed: tis.provider_executed,
                dynamic: None,
                title: None,
            },
            LanguageModelStreamPart::ToolInputDelta(tid) => TextStreamPart::ToolInputDelta {
                id: tid.id,
                delta: tid.delta,
                provider_metadata: tid.provider_metadata,
            },
            LanguageModelStreamPart::ToolInputEnd(tie) => TextStreamPart::ToolInputEnd {
                id: tie.id,
                provider_metadata: tie.provider_metadata,
            },
            LanguageModelStreamPart::Source(source) => {
                let source_output = SourceOutput::new(source.clone());
                step_content.push(Output::Source(source_output.clone()));
                TextStreamPart::Source {
                    source: source_output,
                }
            }
            LanguageModelStreamPart::File(file) => {
                use ai_sdk_provider::language_model::content::file::FileData;

                // Convert FileData to base64
                let base64 = match file.data {
                    FileData::Base64(s) => s,
                    FileData::Binary(bytes) => {
                        use base64::{Engine as _, engine::general_purpose};
                        general_purpose::STANDARD.encode(&bytes)
                    }
                };

                TextStreamPart::File {
                    file: StreamGeneratedFile {
                        base64,
                        media_type: file.media_type,
                        name: None,
                    },
                }
            }
            LanguageModelStreamPart::StreamStart(ss) => {
                step_warnings = if ss.warnings.is_empty() {
                    None
                } else {
                    Some(ss.warnings.clone())
                };
                TextStreamPart::StartStep {
                    request: crate::generate_text::RequestMetadata {
                        body: request_body.clone(),
                    },
                    warnings: ss.warnings,
                }
            }
            LanguageModelStreamPart::Finish(f) => {
                // Flush any remaining text/reasoning
                if !current_text.is_empty() {
                    step_content.push(Output::Text(TextOutput::new(current_text.clone())));
                    current_text.clear();
                }
                if !current_reasoning.is_empty() {
                    step_content.push(Output::Reasoning(ReasoningOutput::new(
                        current_reasoning.clone(),
                    )));
                    current_reasoning.clear();
                }

                step_usage = f.usage.clone();
                step_finish_reason = f.finish_reason.clone();
                step_provider_metadata = f.provider_metadata.clone();

                TextStreamPart::FinishStep {
                    response: ai_sdk_provider::language_model::response_metadata::LanguageModelResponseMetadata::default(),
                    usage: f.usage.clone(),
                    finish_reason: f.finish_reason.clone(),
                    provider_metadata: f.provider_metadata.clone(),
                }
            }
            LanguageModelStreamPart::Raw(r) => {
                if include_raw_chunks {
                    TextStreamPart::Raw {
                        raw_value: r.raw_value,
                    }
                } else {
                    continue;
                }
            }
            LanguageModelStreamPart::Error(e) => {
                // Call on_error callback if provided
                if let Some(callback) = on_error {
                    let event = callbacks::StreamTextErrorEvent {
                        error: e.error.clone(),
                    };
                    callback(event).await;
                }
                return Err(AISDKError::model_error(format!(
                    "Stream error: {}",
                    e.error
                )));
            }
            LanguageModelStreamPart::ToolCall(provider_tool_call) => {
                // Parse the tool call using parse_tool_call
                use crate::generate_text::parse_tool_call;

                let typed_tool_call = if let Some(tool_set) = tools {
                    parse_tool_call(&provider_tool_call, tool_set)
                } else {
                    // No tools provided, treat as dynamic
                    use crate::generate_text::parse_provider_executed_dynamic_tool_call;
                    parse_provider_executed_dynamic_tool_call(&provider_tool_call)
                };

                match typed_tool_call {
                    Ok(tool_call) => {
                        // Add to step content and tool_calls list
                        step_content.push(Output::ToolCall(tool_call.clone()));
                        step_tool_calls.push(tool_call.clone());

                        TextStreamPart::ToolCall { tool_call }
                    }
                    Err(e) => {
                        // Handle tool parsing error
                        if let Some(callback) = on_error {
                            let event = callbacks::StreamTextErrorEvent {
                                error: serde_json::json!({ "message": e.to_string() }),
                            };
                            callback(event).await;
                        }
                        return Err(e);
                    }
                }
            }
            LanguageModelStreamPart::ToolResult(provider_tool_result) => {
                // Convert provider tool result to typed tool result
                use crate::generate_text::{DynamicToolResult, StaticToolResult, TypedToolResult};

                // Check if this is a dynamic tool based on whether we have it in our tool set
                let is_dynamic = if let Some(tool_set) = tools {
                    !tool_set.contains_key(&provider_tool_result.tool_name)
                } else {
                    true // No tools provided, treat as dynamic
                };

                let typed_result = if is_dynamic {
                    let mut dynamic_result = DynamicToolResult::new(
                        &provider_tool_result.tool_call_id,
                        &provider_tool_result.tool_name,
                        serde_json::Value::Null, // input - we don't have it in tool result
                        provider_tool_result.result.clone(),
                    );
                    if let Some(provider_executed) = provider_tool_result.provider_executed {
                        dynamic_result = dynamic_result.with_provider_executed(provider_executed);
                    }
                    TypedToolResult::Dynamic(dynamic_result)
                } else {
                    let mut static_result = StaticToolResult::new(
                        &provider_tool_result.tool_call_id,
                        &provider_tool_result.tool_name,
                        serde_json::Value::Null, // input - we don't have it in tool result
                        provider_tool_result.result.clone(),
                    );
                    if let Some(provider_executed) = provider_tool_result.provider_executed {
                        static_result = static_result.with_provider_executed(provider_executed);
                    }
                    TypedToolResult::Static(static_result)
                };

                // Add to step content
                step_content.push(Output::ToolResult(typed_result.clone()));

                TextStreamPart::ToolResult {
                    tool_result: typed_result,
                }
            }
            // Handle response metadata
            LanguageModelStreamPart::ResponseMetadata(_) => {
                // Skip response metadata in stream parts
                continue;
            }
        };

        // Call on_chunk callback if this is a chunk type
        if let Some(callback) = on_chunk {
            if let Some(chunk) = callbacks::ChunkStreamPart::from_stream_part(&text_stream_part) {
                let event = callbacks::StreamTextChunkEvent { chunk };
                callback(event).await;
            }
        }

        // Send the part to the channel
        if tx.send(text_stream_part).is_err() {
            break;
        }
    }

    Ok(SingleStepStreamResult {
        content: step_content,
        tool_calls: step_tool_calls,
        finish_reason: step_finish_reason,
        usage: step_usage,
        request: step_request,
        response: step_response,
        provider_metadata: step_provider_metadata,
        warnings: step_warnings,
    })
}

/// Stream text using a language model with multi-step tool execution support.
///
/// This is the main user-facing function for streaming text generation in the AI SDK.
/// It supports multi-step generation where the model can call tools, receive results,
/// and continue generating text based on those results.
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
/// * `transforms` - Optional list of stream transformations to apply to the output stream.
/// * `on_chunk` - Optional callback that is called for each chunk of the stream.
/// * `on_error` - Optional callback that is invoked when an error occurs during streaming.
/// * `on_step_finish` - Optional callback called after each step (LLM call) completes.
/// * `on_finish` - Optional callback that is called when the LLM response and all tool executions are finished.
/// * `on_abort` - Optional callback that is called when the generation is aborted.
///
/// # Returns
///
/// Returns `Result<StreamTextResult, AISDKError>` - A stream result object that provides multiple
/// ways to access the streamed data, or a validation error.
///
/// # Examples
///
/// ```ignore
/// use ai_sdk_core::stream_text;
/// use ai_sdk_core::prompt::{Prompt, call_settings::CallSettings};
///
/// let result = stream_text(
///     settings,
///     prompt,
///     model,
///     None,
///     None,
///     None,
///     None,
///     None,
///     false,
///     None,
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
    model: Arc<dyn LanguageModel>,
    prompt: Prompt,
    settings: CallSettings,
    tools: Option<ToolSet>,
    tool_choice: Option<LanguageModelToolChoice>,
    provider_options: Option<SharedProviderOptions>,
    stop_when: Option<Vec<Box<dyn StopCondition>>>,
    prepare_step: Option<Box<dyn PrepareStep>>,
    include_raw_chunks: bool,
    transforms: Option<Vec<Box<dyn StreamTransform<Value, Value>>>>,
    on_chunk: Option<OnChunkCallback>,
    on_error: Option<OnErrorCallback>,
    on_step_finish: Option<OnStepFinishCallback>,
    on_finish: Option<OnFinishCallback>,
) -> Result<StreamTextResult<Value, Value>, AISDKError> {
    // Initialize stop conditions with default if not provided
    let stop_conditions = Arc::new(
        stop_when.unwrap_or_else(|| vec![Box::new(crate::generate_text::step_count_is(1))]),
    );

    // Prepare and validate call settings
    let prepared_settings = prepare_call_settings(&settings)?;

    // Validate and standardize the prompt
    let standardized_prompt = validate_and_standardize(prompt)?;

    // Prepare tools and tool choice
    let (provider_tools, prepared_tool_choice) =
        prepare_tools_and_tool_choice(tools.as_ref(), tool_choice);

    // Create a channel to emit TextStreamPart events
    let (tx, rx) = mpsc::unbounded_channel::<TextStreamPart<Value, Value>>();

    // Wrap callbacks in Arc for sharing in the spawned task
    let on_chunk_arc = on_chunk.map(Arc::new);
    let on_error_arc = on_error.map(Arc::new);
    let on_step_finish_arc = on_step_finish.map(Arc::new);
    let on_finish_arc = on_finish.map(Arc::new);

    // Wrap data in Arc for sharing in the spawned task
    let tools_for_task = tools.map(Arc::new);
    let settings_arc = Arc::new(settings);
    let standardized_prompt_arc = Arc::new(standardized_prompt);
    let provider_options_arc = provider_options.map(Arc::new);
    let prepare_step_arc = prepare_step.map(Arc::new);
    let model_arc = model; // model is already Arc<dyn LanguageModel>
    let stop_conditions_arc = stop_conditions;

    // Spawn a task to handle the multi-step streaming
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        // Emit Start event
        let _ = tx_clone.send(TextStreamPart::Start);

        // Accumulate all steps
        let mut all_steps: Vec<StepResult<Value, Value>> = Vec::new();
        let mut total_usage = LanguageModelUsage::default();

        // Start with the initial prompt messages
        let mut step_input_messages = standardized_prompt_arc.messages.clone();

        // Multi-step loop
        loop {
            // Apply prepare_step if provided
            let prepare_step_result = if let Some(ref prepare_fn) = prepare_step_arc {
                let step_number = all_steps.len();
                prepare_fn
                    .prepare(&PrepareStepOptions {
                        steps: &all_steps,
                        step_number,
                        messages: &step_input_messages,
                    })
                    .await
            } else {
                None
            };

            // Apply prepare_step overrides
            let step_system = prepare_step_result
                .as_ref()
                .and_then(|r| r.system.clone())
                .or_else(|| standardized_prompt_arc.system.clone());

            let step_messages = prepare_step_result
                .as_ref()
                .and_then(|r| r.messages.clone())
                .unwrap_or_else(|| step_input_messages.clone());

            let step_tool_choice = prepare_step_result
                .as_ref()
                .and_then(|r| r.tool_choice.clone())
                .or(prepared_tool_choice.clone());

            let step_active_tools = prepare_step_result
                .as_ref()
                .and_then(|r| r.active_tools.clone());

            // Convert current messages to language model format
            let messages = match convert_to_language_model_prompt(StandardizedPrompt {
                messages: step_messages.clone(),
                system: step_system,
            }) {
                Ok(m) => m,
                Err(e) => {
                    let _ = tx_clone.send(TextStreamPart::Error {
                        error: serde_json::json!({ "message": e.to_string() }),
                    });
                    break;
                }
            };

            // Build CallOptions
            let mut call_options = LanguageModelCallOptions::new(messages);

            // Add prepared settings
            if let Some(max_tokens) = prepared_settings.max_output_tokens {
                call_options = call_options.with_max_output_tokens(max_tokens);
            }
            if let Some(temp) = prepared_settings.temperature {
                call_options = call_options.with_temperature(temp);
            }
            if let Some(top_p) = prepared_settings.top_p {
                call_options = call_options.with_top_p(top_p);
            }
            if let Some(top_k) = prepared_settings.top_k {
                call_options = call_options.with_top_k(top_k);
            }
            if let Some(penalty) = prepared_settings.presence_penalty {
                call_options = call_options.with_presence_penalty(penalty);
            }
            if let Some(penalty) = prepared_settings.frequency_penalty {
                call_options = call_options.with_frequency_penalty(penalty);
            }
            if let Some(ref sequences) = prepared_settings.stop_sequences {
                call_options = call_options.with_stop_sequences(sequences.clone());
            }
            if let Some(seed) = prepared_settings.seed {
                call_options = call_options.with_seed(seed);
            }

            // Add tools and tool choice - filter by active_tools if provided
            let step_provider_tools = if let Some(ref active_tool_names) = step_active_tools {
                provider_tools.as_ref().map(|tools_vec| {
                    tools_vec
                        .iter()
                        .filter(|tool| {
                            active_tool_names.iter().any(|name| match tool {
                                ai_sdk_provider::language_model::tool::LanguageModelTool::Function(f) => {
                                    f.name == *name
                                }
                                ai_sdk_provider::language_model::tool::LanguageModelTool::ProviderDefined(p) => {
                                    p.name == *name
                                }
                            })
                        })
                        .cloned()
                        .collect()
                })
            } else {
                provider_tools.clone()
            };

            if let Some(ref tools_vec) = step_provider_tools {
                call_options = call_options.with_tools(tools_vec.clone());
            }
            if let Some(ref choice) = step_tool_choice {
                call_options = call_options.with_tool_choice(choice.clone());
            }

            // Add headers and abort signal
            if let Some(ref headers) = settings_arc.headers {
                call_options = call_options.with_headers(headers.clone());
            }
            let abort_signal_for_tools = settings_arc.abort_signal.clone();
            if let Some(signal) = settings_arc.abort_signal.clone() {
                call_options = call_options.with_abort_signal(signal);
            }

            // Add provider options
            if let Some(ref opts_arc) = provider_options_arc {
                call_options = call_options.with_provider_options((**opts_arc).clone());
            }

            // Stream this single step
            let step_result = match stream_single_step(
                model_arc.clone(),
                call_options,
                tools_for_task.as_ref().map(|arc| arc.as_ref()),
                include_raw_chunks,
                &tx_clone,
                on_chunk_arc.as_ref(),
                on_error_arc.as_ref(),
            )
            .await
            {
                Ok(r) => r,
                Err(e) => {
                    if let Some(callback) = on_error_arc.as_ref() {
                        let event = callbacks::StreamTextErrorEvent {
                            error: serde_json::json!({ "message": e.to_string() }),
                        };
                        callback(event).await;
                    }
                    let _ = tx_clone.send(TextStreamPart::Error {
                        error: serde_json::json!({ "message": e.to_string() }),
                    });
                    break;
                }
            };

            // Update total usage
            total_usage = LanguageModelUsage {
                input_tokens: total_usage.input_tokens + step_result.usage.input_tokens,
                output_tokens: total_usage.output_tokens + step_result.usage.output_tokens,
                total_tokens: total_usage.total_tokens + step_result.usage.total_tokens,
                reasoning_tokens: total_usage.reasoning_tokens + step_result.usage.reasoning_tokens,
                cached_input_tokens: total_usage.cached_input_tokens
                    + step_result.usage.cached_input_tokens,
            };

            // Filter client tool calls (those not executed by the provider)
            let client_tool_calls: Vec<&TypedToolCall<Value>> = step_result
                .tool_calls
                .iter()
                .filter(|tool_call| {
                    let provider_executed = match tool_call {
                        TypedToolCall::Static(c) => c.provider_executed,
                        TypedToolCall::Dynamic(c) => c.provider_executed,
                    };
                    provider_executed != Some(true)
                })
                .collect();

            // Execute client tool calls
            let mut client_tool_outputs = Vec::new();
            if let Some(tool_set) = tools_for_task.as_ref() {
                for &tool_call in &client_tool_calls {
                    if let Some(output) = execute_tool_call(
                        tool_call.clone(),
                        tool_set,
                        step_messages.clone(),
                        abort_signal_for_tools.clone(),
                        None, // experimental_context
                        None, // on_preliminary_tool_result
                    )
                    .await
                    {
                        // Emit tool result to the stream
                        match &output {
                            crate::generate_text::ToolOutput::Result(result) => {
                                let _ = tx_clone.send(TextStreamPart::ToolResult {
                                    tool_result: result.clone(),
                                });
                            }
                            crate::generate_text::ToolOutput::Error(error) => {
                                let _ = tx_clone.send(TextStreamPart::ToolError {
                                    tool_error: error.clone(),
                                });
                            }
                        }
                        client_tool_outputs.push(output);
                    }
                }
            }

            let client_tool_outputs_count = client_tool_outputs.len();

            // Create step content by combining provider content with client tool outputs
            let mut step_content = step_result.content.clone();
            for output in client_tool_outputs {
                match output {
                    crate::generate_text::ToolOutput::Result(tool_result) => {
                        step_content.push(Output::ToolResult(tool_result));
                    }
                    crate::generate_text::ToolOutput::Error(tool_error) => {
                        step_content.push(Output::ToolError(tool_error));
                    }
                }
            }

            // Create StepResult
            let current_step_result = StepResult::new(
                step_content.clone(),
                step_result.finish_reason.clone(),
                step_result.usage.clone(),
                step_result.warnings.clone(),
                step_result.request.clone(),
                step_result.response.clone(),
                step_result.provider_metadata.clone(),
            );

            all_steps.push(current_step_result.clone());

            // Call on_step_finish callback
            if let Some(ref callback) = on_step_finish_arc {
                callback(current_step_result).await;
            }

            // Append to messages for potential next step
            let step_response_messages = to_response_messages(
                step_content,
                tools_for_task.as_ref().map(|arc| arc.as_ref()),
            );
            for msg in step_response_messages {
                step_input_messages.push(msg);
            }

            // Check loop termination conditions
            let should_continue = !client_tool_calls.is_empty()
                && client_tool_outputs_count == client_tool_calls.len()
                && !is_stop_condition_met(&*stop_conditions_arc, &all_steps).await;

            if !should_continue {
                break;
            }
        }

        // Emit final Finish event
        if let Some(last_step) = all_steps.last() {
            let _ = tx_clone.send(TextStreamPart::Finish {
                finish_reason: last_step.finish_reason.clone(),
                total_usage: total_usage.clone(),
            });
        }

        // Call on_finish callback if provided
        if let Some(ref callback) = on_finish_arc {
            if let Some(last_step) = all_steps.last() {
                let event = callbacks::StreamTextFinishEvent {
                    step_result: last_step.clone(),
                    steps: all_steps,
                    total_usage,
                };
                callback(event).await;
            }
        }
    });

    // Step 7: Create an AsyncIterableStream from the receiver
    let mut stream: Pin<Box<dyn futures_util::Stream<Item = TextStreamPart<Value, Value>> + Send>> =
        Box::pin(async_stream::stream! {
            let mut rx = rx;
            while let Some(part) = rx.recv().await {
                yield part;
            }
        });

    // Step 8: Apply transforms if provided
    if let Some(transform_list) = transforms {
        let transform_options = TransformOptions::new();
        for transform in transform_list {
            stream = transform.transform(stream, transform_options.clone());
        }
    }

    // Step 9: Create and return StreamTextResult
    Ok(StreamTextResult::new(stream))
}
