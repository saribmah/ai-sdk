pub mod callbacks;
pub mod stream_text_result;
pub mod text_stream_part;

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
    stop_when: Option<Vec<Box<dyn crate::generate_text::StopCondition>>>,
    provider_options: Option<ProviderOptions>,
    prepare_step: Option<Box<dyn crate::generate_text::PrepareStep>>,
    include_raw_chunks: bool,
    on_chunk: Option<OnChunkCallback>,
    on_error: Option<OnErrorCallback>,
    on_step_finish: Option<OnStepFinishCallback>,
    on_finish: Option<OnFinishCallback>,
    on_abort: Option<OnAbortCallback>,
) -> Result<StreamTextResult<Value, Value>, AISDKError> {
    use crate::generate_text::prepare_tools_and_tool_choice;
    use crate::prompt::{
        call_settings::prepare_call_settings,
        convert_to_language_model_prompt::convert_to_language_model_prompt,
        standardize::validate_and_standardize,
    };
    use ai_sdk_provider::language_model::call_options::CallOptions;
    use ai_sdk_provider::language_model::stream_part::StreamPart;
    use futures_util::StreamExt;
    use std::sync::Arc;
    use tokio::sync::mpsc;

    // Step 1: Prepare and validate call settings
    let prepared_settings = prepare_call_settings(&settings)?;

    // Step 2: Validate and standardize the prompt
    let standardized_prompt = validate_and_standardize(prompt)?;

    // Step 3: Prepare tools and tool choice
    let (provider_tools, prepared_tool_choice) =
        prepare_tools_and_tool_choice(tools.as_ref(), tool_choice);

    // Step 4: Convert to language model format
    let messages = convert_to_language_model_prompt(standardized_prompt.clone())?;

    // Step 5: Build CallOptions
    let mut call_options = CallOptions::new(messages);

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

    // Add tools and tool choice
    if let Some(ref tools_vec) = provider_tools {
        call_options = call_options.with_tools(tools_vec.clone());
    }
    if let Some(ref choice) = prepared_tool_choice {
        call_options = call_options.with_tool_choice(choice.clone());
    }

    // Add headers and abort signal
    if let Some(ref headers) = settings.headers {
        call_options = call_options.with_headers(headers.clone());
    }
    if let Some(signal) = settings.abort_signal.clone() {
        call_options = call_options.with_abort_signal(signal);
    }

    // Add provider options
    if let Some(ref opts) = provider_options {
        call_options = call_options.with_provider_options(opts.clone());
    }

    // Step 6: Call model.do_stream
    let stream_response = model
        .do_stream(call_options)
        .await
        .map_err(|e| AISDKError::model_error(e.to_string()))?;

    // Extract metadata before moving stream
    let request_body = stream_response.request.as_ref().and_then(|r| r.body.clone());

    // Step 7: Create a channel to emit TextStreamPart events
    let (tx, rx) = mpsc::unbounded_channel::<TextStreamPart<Value, Value>>();

    // Step 8: Process the provider stream and convert to TextStreamPart
    let mut provider_stream = stream_response.stream;
    let tx_clone = tx.clone();
    // Wrap callbacks in Arc for sharing in the spawned task
    let on_chunk_arc = on_chunk.map(Arc::new);
    let on_error_arc = on_error.map(Arc::new);
    let include_raw = include_raw_chunks;

    tokio::spawn(async move {
        // Emit Start event
        let _ = tx_clone.send(TextStreamPart::Start);

        while let Some(part) = provider_stream.next().await {
            // Convert StreamPart to TextStreamPart
            let text_stream_part = match part {
                StreamPart::TextStart { id, provider_metadata } => {
                    TextStreamPart::TextStart { id, provider_metadata }
                }
                StreamPart::TextDelta { id, delta, provider_metadata } => {
                    TextStreamPart::TextDelta {
                        id,
                        provider_metadata,
                        text: delta,
                    }
                }
                StreamPart::TextEnd { id, provider_metadata } => {
                    TextStreamPart::TextEnd { id, provider_metadata }
                }
                StreamPart::ReasoningStart { id, provider_metadata } => {
                    TextStreamPart::ReasoningStart { id, provider_metadata }
                }
                StreamPart::ReasoningDelta { id, delta, provider_metadata } => {
                    TextStreamPart::ReasoningDelta {
                        id,
                        provider_metadata,
                        text: delta,
                    }
                }
                StreamPart::ReasoningEnd { id, provider_metadata } => {
                    TextStreamPart::ReasoningEnd { id, provider_metadata }
                }
                StreamPart::ToolInputStart {
                    id,
                    tool_name,
                    provider_metadata,
                    provider_executed,
                } => TextStreamPart::ToolInputStart {
                    id,
                    tool_name,
                    provider_metadata,
                    provider_executed,
                    dynamic: None,
                    title: None,
                },
                StreamPart::ToolInputDelta { id, delta, provider_metadata } => {
                    TextStreamPart::ToolInputDelta {
                        id,
                        delta,
                        provider_metadata,
                    }
                }
                StreamPart::ToolInputEnd { id, provider_metadata } => {
                    TextStreamPart::ToolInputEnd { id, provider_metadata }
                }
                StreamPart::Source(source) => {
                    use crate::generate_text::SourceOutput;
                    TextStreamPart::Source {
                        source: SourceOutput::new(source),
                    }
                }
                StreamPart::File(file) => {
                    use ai_sdk_provider::language_model::file::FileData;
                    use crate::stream_text::StreamGeneratedFile;

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
                            name: None, // File type doesn't have a name field
                        },
                    }
                }
                StreamPart::StreamStart { warnings } => {
                    use crate::generate_text::RequestMetadata;
                    TextStreamPart::StartStep {
                        request: RequestMetadata {
                            body: request_body.clone(),
                        },
                        warnings,
                    }
                }
                StreamPart::Finish {
                    usage,
                    finish_reason,
                    provider_metadata,
                } => {
                    // Emit FinishStep first
                    let finish_step = TextStreamPart::FinishStep {
                        response: ai_sdk_provider::language_model::response_metadata::ResponseMetadata::default(),
                        usage: usage.clone(),
                        finish_reason: finish_reason.clone(),
                        provider_metadata: provider_metadata.clone(),
                    };
                    let _ = tx_clone.send(finish_step);

                    // Then emit the overall Finish event
                    TextStreamPart::Finish {
                        finish_reason,
                        total_usage: usage,
                    }
                }
                StreamPart::Raw { raw_value } => {
                    if include_raw {
                        TextStreamPart::Raw { raw_value }
                    } else {
                        continue;
                    }
                }
                StreamPart::Error { error } => {
                    // Call on_error callback if provided
                    if let Some(ref callback) = on_error_arc {
                        let event = callbacks::StreamTextErrorEvent { error: error.clone() };
                        callback(event).await;
                    }
                    TextStreamPart::Error { error }
                }
                // Handle tool calls and results - for now, skip them in Phase 1
                _ => continue,
            };

            // Call on_chunk callback if this is a chunk type
            if let Some(ref callback) = on_chunk_arc {
                if let Some(chunk) = callbacks::ChunkStreamPart::from_stream_part(&text_stream_part) {
                    let event = callbacks::StreamTextChunkEvent { chunk };
                    callback(event).await;
                }
            }

            // Send the part to the channel
            if tx_clone.send(text_stream_part).is_err() {
                break;
            }
        }

        // Stream completed - no need to send anything else
    });

    // Step 9: Create an AsyncIterableStream from the receiver
    let stream = Box::pin(async_stream::stream! {
        let mut rx = rx;
        while let Some(part) = rx.recv().await {
            yield part;
        }
    });

    // Step 10: Create and return StreamTextResult
    Ok(StreamTextResult::new(stream))
}
