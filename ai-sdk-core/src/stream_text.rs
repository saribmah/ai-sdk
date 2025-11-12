/// Callback types for streaming events.
pub mod callbacks;
/// Output parsing for structured streaming.
pub mod output;
/// Result type for streaming operations.
pub mod stream_text_result;
/// Stream part types for text streaming.
pub mod text_stream_part;
/// Stream transformation utilities.
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

use crate::ResponseMessage;
use crate::error::AISDKError;
use crate::generate_text::{
    PrepareStep, PrepareStepOptions, StepResult, StopCondition, is_stop_condition_met,
    to_response_messages,
};
use crate::output::{Output, ReasoningOutput, SourceOutput, TextOutput};
use crate::prompt::{
    Prompt, call_settings::CallSettings, call_settings::prepare_call_settings,
    convert_to_language_model_prompt::convert_to_language_model_prompt,
    standardize::StandardizedPrompt, standardize::validate_and_standardize,
};
use crate::tool::{
    ToolSet, execute_tool_call, parse_provider_executed_dynamic_tool_call, parse_tool_call,
    prepare_tools_and_tool_choice,
};
use ai_sdk_provider::language_model::call_options::LanguageModelCallOptions;
use ai_sdk_provider::language_model::{
    LanguageModel, finish_reason::LanguageModelFinishReason, tool_choice::LanguageModelToolChoice,
    usage::LanguageModelUsage,
};
use ai_sdk_provider::shared::provider_options::SharedProviderOptions;
use ai_sdk_provider_utils::message::Message;
use ai_sdk_provider_utils::tool::ToolCall;
use serde_json::Value;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Result of streaming a single step.
///
/// This contains all the accumulated data from processing one streaming call to the model.
struct SingleStepStreamResult {
    /// The content parts accumulated during the step
    content: Vec<Output>,

    /// Tool calls made during the step
    tool_calls: Vec<ToolCall>,

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
    tx: &mpsc::UnboundedSender<TextStreamPart>,
    on_chunk: Option<&Arc<OnChunkCallback>>,
    on_error: Option<&Arc<OnErrorCallback>>,
) -> Result<SingleStepStreamResult, AISDKError> {
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
    let mut step_content: Vec<Output> = Vec::new();
    let mut step_tool_calls: Vec<ToolCall> = Vec::new();
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

                step_usage = f.usage;
                step_finish_reason = f.finish_reason.clone();
                step_provider_metadata = f.provider_metadata.clone();

                TextStreamPart::FinishStep {
                    response: ai_sdk_provider::language_model::response_metadata::LanguageModelResponseMetadata::default(),
                    usage: f.usage,
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

                let typed_tool_call = if let Some(tool_set) = tools {
                    parse_tool_call(&provider_tool_call, tool_set)
                } else {
                    // No tools provided, treat as dynamic
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
                use ai_sdk_provider_utils::tool::ToolResult;

                // Find the matching tool call to get the input
                let matching_call = step_tool_calls
                    .iter()
                    .find(|tc| tc.tool_call_id == provider_tool_result.tool_call_id);

                // Check if this is a dynamic tool based on whether we have it in our tool set
                let _is_dynamic = if let Some(tool_set) = tools {
                    !tool_set.contains_key(&provider_tool_result.tool_name)
                } else {
                    true // No tools provided, treat as dynamic
                };

                let input = matching_call
                    .map(|tc| tc.input.clone())
                    .unwrap_or(Value::Null);

                let mut typed_result = ToolResult::new(
                    provider_tool_result.tool_call_id.clone(),
                    provider_tool_result.tool_name.clone(),
                    input,
                    provider_tool_result.result.clone(),
                );
                if let Some(provider_executed) = provider_tool_result.provider_executed {
                    typed_result = typed_result.with_provider_executed(provider_executed);
                }

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
        if let Some(callback) = on_chunk
            && let Some(chunk) = callbacks::ChunkStreamPart::from_stream_part(&text_stream_part)
        {
            let event = callbacks::StreamTextChunkEvent { chunk };
            callback(event).await;
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

/// Builder for streaming text using a language model with fluent API.
///
/// This builder provides a chainable interface for configuring text streaming.
///
/// # Examples
///
/// ```no_run
/// use ai_sdk_core::StreamText;
/// use ai_sdk_core::prompt::Prompt;
/// use std::sync::Arc;
/// use futures::StreamExt;
/// # use ai_sdk_provider::LanguageModel;
/// # async fn example(model: Arc<dyn LanguageModel>) -> Result<(), Box<dyn std::error::Error>> {
///
/// let result = StreamText::new(model, Prompt::text("Tell me a story"))
///     .temperature(0.8)
///     .max_output_tokens(500)
///     .include_raw_chunks(true)
///     .execute()
///     .await?;
///
/// // Stream text deltas in real-time
/// let mut stream = result.text_stream();
/// while let Some(delta) = stream.next().await {
///     print!("{}", delta);
/// }
/// # Ok(())
/// # }
/// ```
pub struct StreamText {
    model: Arc<dyn LanguageModel>,
    prompt: Prompt,
    settings: CallSettings,
    tools: Option<ToolSet>,
    tool_choice: Option<LanguageModelToolChoice>,
    provider_options: Option<SharedProviderOptions>,
    stop_when: Option<Vec<Box<dyn StopCondition>>>,
    prepare_step: Option<Box<dyn PrepareStep>>,
    include_raw_chunks: bool,
    transforms: Option<Vec<Box<dyn StreamTransform>>>,
    on_chunk: Option<OnChunkCallback>,
    on_error: Option<OnErrorCallback>,
    on_step_finish: Option<OnStepFinishCallback>,
    on_finish: Option<OnFinishCallback>,
    #[cfg(feature = "storage")]
    storage: Option<Arc<dyn ai_sdk_storage::Storage>>,
    #[cfg(feature = "storage")]
    session_id: Option<String>,
    #[cfg(feature = "storage")]
    load_history: bool,
}

impl StreamText {
    /// Creates a new builder with the required model and prompt.
    pub fn new(model: Arc<dyn LanguageModel>, prompt: Prompt) -> Self {
        Self {
            model,
            prompt,
            settings: CallSettings::default(),
            tools: None,
            tool_choice: None,
            provider_options: None,
            stop_when: None,
            prepare_step: None,
            include_raw_chunks: false,
            transforms: None,
            on_chunk: None,
            on_error: None,
            on_step_finish: None,
            on_finish: None,
            #[cfg(feature = "storage")]
            storage: None,
            #[cfg(feature = "storage")]
            session_id: None,
            #[cfg(feature = "storage")]
            load_history: true, // Default to true for automatic history loading
        }
    }

    /// Sets the complete call settings.
    pub fn settings(mut self, settings: CallSettings) -> Self {
        self.settings = settings;
        self
    }

    /// Sets the temperature for generation.
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.settings = self.settings.with_temperature(temperature);
        self
    }

    /// Sets the maximum output tokens.
    pub fn max_output_tokens(mut self, max_tokens: u32) -> Self {
        self.settings = self.settings.with_max_output_tokens(max_tokens);
        self
    }

    /// Sets the top_p sampling parameter.
    pub fn top_p(mut self, top_p: f64) -> Self {
        self.settings = self.settings.with_top_p(top_p);
        self
    }

    /// Sets the top_k sampling parameter.
    pub fn top_k(mut self, top_k: u32) -> Self {
        self.settings = self.settings.with_top_k(top_k);
        self
    }

    /// Sets the presence penalty.
    pub fn presence_penalty(mut self, penalty: f64) -> Self {
        self.settings = self.settings.with_presence_penalty(penalty);
        self
    }

    /// Sets the frequency penalty.
    pub fn frequency_penalty(mut self, penalty: f64) -> Self {
        self.settings = self.settings.with_frequency_penalty(penalty);
        self
    }

    /// Sets the random seed for deterministic generation.
    pub fn seed(mut self, seed: u32) -> Self {
        self.settings = self.settings.with_seed(seed);
        self
    }

    /// Sets the stop sequences.
    pub fn stop_sequences(mut self, sequences: Vec<String>) -> Self {
        self.settings = self.settings.with_stop_sequences(sequences);
        self
    }

    /// Sets the maximum number of retries.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.settings = self.settings.with_max_retries(max_retries);
        self
    }

    /// Sets custom headers for the request.
    pub fn headers(mut self, headers: std::collections::HashMap<String, String>) -> Self {
        self.settings = self.settings.with_headers(headers);
        self
    }

    /// Sets the abort signal for cancellation.
    pub fn abort_signal(mut self, signal: tokio_util::sync::CancellationToken) -> Self {
        self.settings = self.settings.with_abort_signal(signal);
        self
    }

    /// Sets the tools available for the model to use.
    pub fn tools(mut self, tools: ToolSet) -> Self {
        self.tools = Some(tools);
        self
    }

    /// Sets the tool choice strategy.
    pub fn tool_choice(mut self, choice: LanguageModelToolChoice) -> Self {
        self.tool_choice = Some(choice);
        self
    }

    /// Sets provider-specific options.
    pub fn provider_options(mut self, options: SharedProviderOptions) -> Self {
        self.provider_options = Some(options);
        self
    }

    /// Sets stop conditions for multi-step generation.
    pub fn stop_when(mut self, conditions: Vec<Box<dyn StopCondition>>) -> Self {
        self.stop_when = Some(conditions);
        self
    }

    /// Sets the prepare step callback.
    pub fn prepare_step(mut self, callback: Box<dyn PrepareStep>) -> Self {
        self.prepare_step = Some(callback);
        self
    }

    /// Enables or disables inclusion of raw chunks from the provider.
    pub fn include_raw_chunks(mut self, include: bool) -> Self {
        self.include_raw_chunks = include;
        self
    }

    /// Sets stream transformations to apply to the output.
    pub fn transforms(mut self, transforms: Vec<Box<dyn StreamTransform>>) -> Self {
        self.transforms = Some(transforms);
        self
    }

    /// Sets the on_chunk callback.
    pub fn on_chunk(mut self, callback: OnChunkCallback) -> Self {
        self.on_chunk = Some(callback);
        self
    }

    /// Sets the on_error callback.
    pub fn on_error(mut self, callback: OnErrorCallback) -> Self {
        self.on_error = Some(callback);
        self
    }

    /// Sets the on_step_finish callback.
    pub fn on_step_finish(mut self, callback: OnStepFinishCallback) -> Self {
        self.on_step_finish = Some(callback);
        self
    }

    /// Sets the on_finish callback.
    pub fn on_finish(mut self, callback: OnFinishCallback) -> Self {
        self.on_finish = Some(callback);
        self
    }

    /// Enable storage for conversation persistence.
    ///
    /// When storage is configured with a session ID, the system will:
    /// 1. Load previous conversation history (if `load_history` is true)
    /// 2. Prepend history to the current prompt
    /// 3. Store both user and assistant messages after streaming completes
    ///
    /// # Example
    ///
    /// ```ignore
    /// use ai_sdk_storage_filesystem::FilesystemStorage;
    /// use std::sync::Arc;
    ///
    /// let storage = Arc::new(FilesystemStorage::new("./storage")?);
    /// storage.initialize().await?;
    ///
    /// let result = StreamText::new(model, prompt)
    ///     .with_storage(storage)
    ///     .with_session_id("my-session".to_string())
    ///     .execute()
    ///     .await?;
    /// ```
    #[cfg(feature = "storage")]
    pub fn with_storage(mut self, storage: Arc<dyn ai_sdk_storage::Storage>) -> Self {
        self.storage = Some(storage);
        self.load_history = true; // Enable history loading by default
        self
    }

    /// Set a session ID for conversation continuity.
    ///
    /// This identifies which conversation this streaming belongs to.
    /// If the session doesn't exist, it will be created automatically.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let result = StreamText::new(model, prompt)
    ///     .with_storage(storage)
    ///     .with_session_id("session-123".to_string())
    ///     .execute()
    ///     .await?;
    /// ```
    #[cfg(feature = "storage")]
    pub fn with_session_id(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    /// Disable automatic history loading.
    ///
    /// Useful for the first message in a session or when you want to
    /// provide the full context manually.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let result = StreamText::new(model, prompt)
    ///     .with_storage(storage)
    ///     .with_session_id(session_id)
    ///     .without_history() // First message, no history to load
    ///     .execute()
    ///     .await?;
    /// ```
    #[cfg(feature = "storage")]
    pub fn without_history(mut self) -> Self {
        self.load_history = false;
        self
    }

    /// Executes the text streaming with the configured settings.
    pub async fn execute(self) -> Result<StreamTextResult, AISDKError> {
        // Initialize stop conditions with default if not provided
        let stop_conditions = Arc::new(
            self.stop_when
                .unwrap_or_else(|| vec![Box::new(crate::generate_text::step_count_is(1))]),
        );

        // Prepare and validate call settings
        let prepared_settings = prepare_call_settings(&self.settings)?;

        // Validate and standardize the prompt
        let standardized_prompt = validate_and_standardize(self.prompt)?;

        // Store user message and load conversation history if storage is configured
        #[cfg(feature = "storage")]
        let standardized_prompt = if let (Some(storage), Some(session_id)) =
            (&self.storage, &self.session_id)
        {
            use crate::storage_conversion::{load_conversation_history, user_message_to_storage};

            // Create session if it doesn't exist
            if storage.get_session(session_id).await.is_err() {
                let session = ai_sdk_storage::Session::new(session_id.clone());
                if let Err(e) = storage.store_session(&session).await {
                    log::warn!("Failed to create session: {}", e);
                }
            }

            // Store the new user message immediately
            if let Some(user_msg) = standardized_prompt.messages.iter().find(|m| m.is_user())
                && let ai_sdk_provider_utils::message::Message::User(user_message) = user_msg
            {
                let (storage_msg, parts) =
                    user_message_to_storage(storage, session_id.clone(), user_message);
                if let Err(e) = storage.store_user_message(&storage_msg, &parts).await {
                    log::warn!("Failed to store user message: {}", e);
                }
            }

            // Load conversation history if enabled (includes the just-stored user message)
            if self.load_history {
                match load_conversation_history(storage, session_id).await {
                    Ok(history) => {
                        if !history.is_empty() {
                            crate::prompt::standardize::StandardizedPrompt {
                                messages: history,
                                system: standardized_prompt.system.clone(),
                            }
                        } else {
                            standardized_prompt
                        }
                    }
                    Err(e) => {
                        log::warn!("Failed to load conversation history: {}", e);
                        standardized_prompt
                    }
                }
            } else {
                standardized_prompt
            }
        } else {
            standardized_prompt
        };

        // Prepare tools and tool choice
        let (provider_tools, prepared_tool_choice) =
            prepare_tools_and_tool_choice(self.tools.as_ref(), self.tool_choice);

        // Create a channel to emit TextStreamPart events
        let (tx, rx) = mpsc::unbounded_channel::<TextStreamPart>();

        // Wrap callbacks in Arc for sharing in the spawned task
        let on_chunk_arc = self.on_chunk.map(Arc::new);
        let on_error_arc = self.on_error.map(Arc::new);
        let on_step_finish_arc = self.on_step_finish.map(Arc::new);
        let on_finish_arc = self.on_finish.map(Arc::new);

        // Wrap data in Arc for sharing in the spawned task
        let tools_for_task = self.tools.map(Arc::new);
        let settings_arc = Arc::new(self.settings);
        let standardized_prompt_arc = Arc::new(standardized_prompt);
        let provider_options_arc = self.provider_options.map(Arc::new);
        let prepare_step_arc = self.prepare_step.map(Arc::new);
        let model_arc = self.model; // model is already Arc<dyn LanguageModel>
        let stop_conditions_arc = stop_conditions;
        let include_raw_chunks = self.include_raw_chunks;
        #[cfg(feature = "storage")]
        let storage_arc = self.storage;
        #[cfg(feature = "storage")]
        let session_id_arc = self.session_id;

        // Spawn a task to handle the multi-step streaming
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            // Emit Start event
            let _ = tx_clone.send(TextStreamPart::Start);

            // Accumulate all steps
            let mut all_steps: Vec<StepResult> = Vec::new();
            let mut total_usage = LanguageModelUsage::default();

            let mut response_messages: Vec<ResponseMessage> = Vec::new();

            // Start with the initial prompt messages
            let mut step_input_messages = standardized_prompt_arc.messages.clone();

            // Multi-step loop
            loop {
                for response_msg in &response_messages {
                    let model_msg = match response_msg {
                        ResponseMessage::Assistant(msg) => Message::Assistant(msg.clone()),
                        ResponseMessage::Tool(msg) => Message::Tool(msg.clone()),
                    };
                    step_input_messages.push(model_msg);
                }
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
                    reasoning_tokens: total_usage.reasoning_tokens
                        + step_result.usage.reasoning_tokens,
                    cached_input_tokens: total_usage.cached_input_tokens
                        + step_result.usage.cached_input_tokens,
                };

                // Filter client tool calls (those not executed by the provider)
                let client_tool_calls: Vec<&ToolCall> = step_result
                    .tool_calls
                    .iter()
                    .filter(|tool_call| tool_call.provider_executed != Some(true))
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
                                ai_sdk_provider_utils::tool::ToolOutput::Result(result) => {
                                    let _ = tx_clone.send(TextStreamPart::ToolResult {
                                        tool_result: result.clone(),
                                    });
                                }
                                ai_sdk_provider_utils::tool::ToolOutput::Error(error) => {
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
                        ai_sdk_provider_utils::tool::ToolOutput::Result(tool_result) => {
                            step_content.push(Output::ToolResult(tool_result));
                        }
                        ai_sdk_provider_utils::tool::ToolOutput::Error(tool_error) => {
                            step_content.push(Output::ToolError(tool_error));
                        }
                    }
                }

                // Create StepResult
                let current_step_result = StepResult::new(
                    step_content.clone(),
                    step_result.finish_reason.clone(),
                    step_result.usage,
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
                    response_messages.push(msg);
                }

                // Check loop termination conditions
                let should_continue = !client_tool_calls.is_empty()
                    && client_tool_outputs_count == client_tool_calls.len()
                    && !is_stop_condition_met(&stop_conditions_arc, &all_steps).await;

                if !should_continue {
                    break;
                }
            }

            // Emit final Finish event
            if let Some(last_step) = all_steps.last() {
                let _ = tx_clone.send(TextStreamPart::Finish {
                    finish_reason: last_step.finish_reason.clone(),
                    total_usage,
                });
            }

            // Call on_finish callback if provided
            if let Some(ref callback) = on_finish_arc
                && let Some(last_step) = all_steps.last()
            {
                let event = callbacks::StreamTextFinishEvent {
                    step_result: last_step.clone(),
                    steps: all_steps.clone(),
                    total_usage,
                };
                callback(event).await;
            }

            // Store all response messages if storage is configured (user message already stored)
            #[cfg(feature = "storage")]
            if let (Some(storage), Some(session_id)) = (&storage_arc, &session_id_arc)
                && !all_steps.is_empty()
            {
                use crate::storage_conversion::response_messages_to_storage;

                // Build a result from the stream for metadata
                let stream_result =
                    crate::generate_text::GenerateTextResult::from_steps(all_steps, total_usage);

                // Convert all response messages (assistant + tool) to storage format
                let storage_messages = response_messages_to_storage(
                    storage,
                    session_id.clone(),
                    &response_messages,
                    &stream_result,
                );

                // Store each message
                for (storage_msg, parts) in storage_messages {
                    if let Err(e) = storage.store_assistant_message(&storage_msg, &parts).await {
                        log::warn!("Failed to store response message: {}", e);
                    }
                }
            }
        });

        // Step 7: Create an AsyncIterableStream from the receiver
        let mut stream: Pin<Box<dyn futures_util::Stream<Item = TextStreamPart> + Send>> =
            Box::pin(async_stream::stream! {
                let mut rx = rx;
                while let Some(part) = rx.recv().await {
                    yield part;
                }
            });

        // Step 8: Apply transforms if provided
        if let Some(transform_list) = self.transforms {
            let transform_options = TransformOptions::new();
            for transform in transform_list {
                stream = transform.transform(stream, transform_options.clone());
            }
        }

        // Step 9: Create and return StreamTextResult
        Ok(StreamTextResult::new(stream))
    }
}
