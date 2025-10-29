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
    _settings: CallSettings,
    _prompt: Prompt,
    _model: &dyn LanguageModel,
    _tools: Option<ToolSet>,
    _tool_choice: Option<ToolChoice>,
    _stop_when: Option<Vec<Box<dyn crate::generate_text::StopCondition>>>,
    _provider_options: Option<ProviderOptions>,
    _prepare_step: Option<Box<dyn crate::generate_text::PrepareStep>>,
    _include_raw_chunks: bool,
    _on_chunk: Option<OnChunkCallback>,
    _on_error: Option<OnErrorCallback>,
    _on_step_finish: Option<OnStepFinishCallback>,
    _on_finish: Option<OnFinishCallback>,
    _on_abort: Option<OnAbortCallback>,
) -> Result<StreamTextResult<Value, Value>, AISDKError> {
    // TODO: Implement stream_text functionality
    // This will involve:
    // 1. Validating and preparing settings (similar to generate_text)
    // 2. Converting the prompt to language model format
    // 3. Calling model.do_stream instead of model.do_generate
    // 4. Handling multi-step streaming with tool calls
    // 5. Creating and returning a StreamTextResult

    unimplemented!("stream_text will be implemented in a future step")
}
