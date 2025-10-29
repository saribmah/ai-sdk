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
    use crate::stream_text::convert_stream_part::convert_stream_part_to_text_stream_part;
    use crate::stream_text::EnrichedStreamPart;
    use ai_sdk_provider::language_model::call_options::CallOptions;
    use futures_util::StreamExt;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use std::collections::HashMap;

    // Step 1: Validate and standardize prompt
    let standardized_prompt = validate_and_standardize(prompt)?;

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
    // StreamPart -> TextStreamPart -> EnrichedStreamPart

    // Wrap callbacks in Arc for sharing across async tasks
    let on_chunk = Arc::new(on_chunk);
    let on_error = Arc::new(on_error);

    // Create shared state for event processor
    let active_text_content: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let active_reasoning_content: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let recorded_content: Arc<Mutex<Vec<crate::generate_text::ContentPart<Value, Value>>>> = Arc::new(Mutex::new(Vec::new()));
    let recorded_finish_reason = Arc::new(Mutex::new(None));
    let recorded_total_usage = Arc::new(Mutex::new(None));

    // Convert provider StreamPart to TextStreamPart
    let text_stream = provider_stream.map(|stream_part| {
        let text_part: TextStreamPart<Value, Value> =
            convert_stream_part_to_text_stream_part(stream_part);
        text_part
    });

    // Wrap in EnrichedStreamPart (no partial output parsing in Phase 2)
    let enriched_stream = text_stream.map(|text_part| {
        EnrichedStreamPart::new(text_part)
    });

    // Box the stream to make it Pin<Box<dyn Stream>>
    let boxed_enriched_stream: AsyncIterableStream<EnrichedStreamPart<Value, Value>> =
        Box::pin(enriched_stream);

    // Step 7: Apply event processor
    // Create a simple event processor that forwards chunks and tracks state
    let processed_stream = Box::pin(async_stream::stream! {
        use futures_util::pin_mut;

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

            // Track active text content (simplified for Phase 2)
            match part {
                TextStreamPart::TextStart { id, .. } => {
                    let mut active_texts = active_text_content.lock().await;
                    active_texts.insert(id.clone(), String::new());
                }
                TextStreamPart::TextDelta { id, text, .. } => {
                    let mut active_texts = active_text_content.lock().await;
                    if let Some(active_text) = active_texts.get_mut(id) {
                        active_text.push_str(text);
                    }
                }
                TextStreamPart::TextEnd { id, .. } => {
                    let mut active_texts = active_text_content.lock().await;
                    active_texts.remove(id);
                }
                TextStreamPart::ReasoningStart { id, .. } => {
                    let mut active_reasoning = active_reasoning_content.lock().await;
                    active_reasoning.insert(id.clone(), String::new());
                }
                TextStreamPart::ReasoningDelta { id, text, .. } => {
                    let mut active_reasoning = active_reasoning_content.lock().await;
                    if let Some(active) = active_reasoning.get_mut(id) {
                        active.push_str(text);
                    }
                }
                TextStreamPart::ReasoningEnd { id, .. } => {
                    let mut active_reasoning = active_reasoning_content.lock().await;
                    active_reasoning.remove(id);
                }
                TextStreamPart::Finish { finish_reason, total_usage, .. } => {
                    // Record finish reason
                    let mut recorded_finish = recorded_finish_reason.lock().await;
                    *recorded_finish = Some(finish_reason.clone());

                    // Record usage
                    let mut recorded_usage = recorded_total_usage.lock().await;
                    *recorded_usage = Some(total_usage.clone());
                }
                _ => {}
            }
        }
    });

    // Step 8: Create StreamTextResult wrapper
    // For Phase 2, we create a simple StreamTextResult that wraps our stream
    // The actual DefaultStreamTextResult integration will come in subsequent phases

    todo!("Phase 2 (partial): Stream pipeline complete. Next: Create StreamTextResult wrapper and implement accessors.")
}
