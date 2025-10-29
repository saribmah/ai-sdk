use crate::error::AISDKError;
use crate::generate_text::{
    PrepareStep, StepResult, StopCondition, ToolCallRepairFunction, ToolSet,
};
use crate::message::ModelMessage;
use crate::prompt::call_settings::CallSettings;
use crate::stream_text::{
    EnrichedStreamPart, OnAbortCallback, OnChunkCallback, OnErrorCallback, OnFinishCallback,
    OnStepFinishCallback, StreamTextTransform, TextStreamPart,
};
use ai_sdk_provider::language_model::LanguageModel;
use ai_sdk_provider::language_model::tool_choice::ToolChoice;
use ai_sdk_provider::language_model::{finish_reason::FinishReason, usage::Usage};
use ai_sdk_provider::shared::provider_options::ProviderOptions;
use futures_util::stream::{Stream, StreamExt};
use serde_json::Value;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};
use tokio_util::sync::CancellationToken;

/// Type alias for async iterable streams.
pub type AsyncIterableStream<T> = Pin<Box<dyn Stream<Item = T> + Send>>;

/// A delayed promise that can be resolved later.
///
/// This is similar to TypeScript's Promise with external resolve/reject functions.
/// It allows setting a value from one part of the code and awaiting it from another.
struct DelayedPromise<T> {
    /// The shared value that will be set when the promise is resolved.
    value: Arc<Mutex<Option<T>>>,
    /// Notifier to wake up tasks waiting on this promise.
    notify: Arc<Notify>,
}

impl<T> DelayedPromise<T>
where
    T: Clone,
{
    /// Creates a new unresolved delayed promise.
    fn new() -> Self {
        Self {
            value: Arc::new(Mutex::new(None)),
            notify: Arc::new(Notify::new()),
        }
    }

    /// Resolves the promise with a value.
    ///
    /// This will wake up any tasks waiting on `await_value()`.
    async fn resolve(&self, value: T) {
        let mut guard = self.value.lock().await;
        *guard = Some(value);
        drop(guard);
        self.notify.notify_waiters();
    }

    /// Waits for the promise to be resolved and returns the value.
    ///
    /// This will block until `resolve()` is called.
    async fn await_value(&self) -> T {
        loop {
            {
                let guard = self.value.lock().await;
                if let Some(ref val) = *guard {
                    return val.clone();
                }
            }
            self.notify.notified().await;
        }
    }

    /// Checks if the promise has been resolved.
    async fn is_resolved(&self) -> bool {
        let guard = self.value.lock().await;
        guard.is_some()
    }
}

impl<T> Clone for DelayedPromise<T> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            notify: Arc::clone(&self.notify),
        }
    }
}

/// Active text content being accumulated during streaming.
///
/// This tracks text deltas by their ID, accumulating the text as chunks arrive.
#[derive(Debug, Clone)]
struct ActiveTextContent {
    /// The accumulated text.
    text: String,
    /// Optional provider metadata.
    provider_metadata: Option<ai_sdk_provider::shared::provider_metadata::ProviderMetadata>,
}

/// Active reasoning content being accumulated during streaming.
///
/// This tracks reasoning deltas by their ID, accumulating the text as chunks arrive.
#[derive(Debug, Clone)]
struct ActiveReasoningContent {
    /// The accumulated reasoning text.
    text: String,
    /// Optional provider metadata.
    provider_metadata: Option<ai_sdk_provider::shared::provider_metadata::ProviderMetadata>,
}

/// The default implementation of a stream text result.
///
/// This struct is the internal implementation that manages streaming text generation,
/// tracking partial outputs, and providing access to the stream and final results.
///
/// It corresponds to the TypeScript `DefaultStreamTextResult` class.
///
/// # Type Parameters
///
/// * `INPUT` - The input type for tools (defaults to `Value`)
/// * `OUTPUT` - The output type from tools and final outputs (defaults to `Value`)
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::stream_text::DefaultStreamTextResult;
/// use serde_json::Value;
///
/// let result: DefaultStreamTextResult<Value, Value> = /* ... */;
/// ```
pub struct DefaultStreamTextResult<INPUT = Value, OUTPUT = Value> {
    /// Delayed promise for total usage across all steps.
    total_usage: DelayedPromise<Usage>,

    /// Delayed promise for the finish reason of the final step.
    finish_reason: DelayedPromise<FinishReason>,

    /// Delayed promise for all steps in the generation.
    steps: DelayedPromise<Vec<StepResult<INPUT, OUTPUT>>>,

    /// The base stream of enriched stream parts.
    ///
    /// This is wrapped in a Mutex so it can be shared across multiple accessors
    /// and consumed only once.
    base_stream: Arc<Mutex<Option<AsyncIterableStream<EnrichedStreamPart<INPUT, OUTPUT>>>>>,

    /// Whether to include raw chunks from the provider in the stream.
    include_raw_chunks: bool,

    /// Optional tool set used during generation.
    tools: Option<ToolSet>,

    /// Optional output specification for structured outputs.
    output: Option<OUTPUT>,

    // Internal state tracking (wrapped in Arc<Mutex<>> for shared mutable access)
    /// Promise to ensure that a step has been fully processed before starting a new step.
    /// This is required because the continuation condition needs updated steps.
    step_finish: Arc<Mutex<Option<DelayedPromise<()>>>>,

    /// Recorded content parts accumulated during generation.
    recorded_content: Arc<Mutex<Vec<crate::generate_text::ContentPart<INPUT, OUTPUT>>>>,

    /// Recorded response messages from all steps.
    recorded_response_messages: Arc<Mutex<Vec<crate::generate_text::ResponseMessage>>>,

    /// Recorded finish reason from the final step.
    recorded_finish_reason: Arc<Mutex<Option<FinishReason>>>,

    /// Recorded total usage across all steps.
    recorded_total_usage: Arc<Mutex<Option<Usage>>>,

    /// Recorded request metadata.
    recorded_request: Arc<Mutex<crate::generate_text::RequestMetadata>>,

    /// Recorded warnings from the model provider.
    recorded_warnings: Arc<Mutex<Vec<ai_sdk_provider::language_model::call_warning::CallWarning>>>,

    /// Recorded steps from all generation rounds.
    recorded_steps: Arc<Mutex<Vec<StepResult<INPUT, OUTPUT>>>>,

    /// Active text content being accumulated, keyed by content ID.
    active_text_content: Arc<Mutex<HashMap<String, ActiveTextContent>>>,

    /// Active reasoning content being accumulated, keyed by content ID.
    active_reasoning_content: Arc<Mutex<HashMap<String, ActiveReasoningContent>>>,
}

/// Parameters for constructing a `DefaultStreamTextResult`.
///
/// This struct contains all the configuration and callbacks needed to create
/// a streaming text result. It mirrors the TypeScript constructor parameters.
///
/// # Type Parameters
///
/// * `INPUT` - The input type for tools (defaults to `Value`)
/// * `OUTPUT` - The output type from tools (defaults to `Value`)
pub struct DefaultStreamTextResultParams<INPUT = Value, OUTPUT = Value> {
    /// The language model to use for generation.
    pub model: Arc<dyn LanguageModel>,

    /// Optional HTTP headers to include in requests.
    pub headers: Option<HashMap<String, String>>,

    /// Call settings (temperature, max tokens, etc.) excluding abort signal and headers.
    pub settings: CallSettings,

    /// Maximum number of retries for failed requests.
    pub max_retries: Option<u32>,

    /// Optional cancellation token for aborting the operation.
    pub abort_signal: Option<CancellationToken>,

    /// Optional system prompt.
    pub system: Option<String>,

    /// Optional text prompt (simple string prompt).
    pub prompt: Option<String>,

    /// Optional structured messages.
    pub messages: Option<Vec<ModelMessage>>,

    /// Optional tool set that can be called by the model.
    pub tools: Option<ToolSet>,

    /// Optional tool choice strategy.
    pub tool_choice: Option<ToolChoice>,

    /// Transformations to apply to the stream.
    pub transforms: Vec<StreamTextTransform<INPUT, OUTPUT>>,

    /// Optional list of active tool names to restrict tool usage.
    pub active_tools: Option<Vec<String>>,

    /// Optional function to repair failed tool calls.
    pub repair_tool_call: Option<ToolCallRepairFunction>,

    /// Stop conditions for multi-step generation.
    pub stop_conditions: Vec<Box<dyn StopCondition>>,

    /// Optional output specification for structured outputs.
    pub output: Option<OUTPUT>,

    /// Optional provider-specific options.
    pub provider_options: Option<ProviderOptions>,

    /// Optional function to customize each step.
    pub prepare_step: Option<Box<dyn PrepareStep>>,

    /// Whether to include raw chunks from the provider.
    pub include_raw_chunks: bool,

    /// Function to get current timestamp in milliseconds since UNIX epoch.
    pub now: Arc<dyn Fn() -> u64 + Send + Sync>,

    /// Function to get current date as ISO 8601 string.
    pub current_date: Arc<dyn Fn() -> String + Send + Sync>,

    /// Function to generate unique IDs.
    pub generate_id: Arc<dyn Fn() -> String + Send + Sync>,

    /// Experimental context (for future extensions).
    pub experimental_context: Option<Value>,

    // Callbacks
    /// Optional callback for each chunk.
    pub on_chunk: Option<OnChunkCallback<INPUT, OUTPUT>>,

    /// Optional callback for errors.
    pub on_error: Option<OnErrorCallback>,

    /// Optional callback when generation finishes.
    pub on_finish: Option<OnFinishCallback<INPUT, OUTPUT>>,

    /// Optional callback when generation is aborted.
    pub on_abort: Option<OnAbortCallback<INPUT, OUTPUT>>,

    /// Optional callback when each step finishes.
    pub on_step_finish: Option<OnStepFinishCallback<INPUT, OUTPUT>>,
}

impl<INPUT, OUTPUT> DefaultStreamTextResult<INPUT, OUTPUT>
where
    INPUT: Clone + Send + Sync + 'static,
    OUTPUT: Clone + Send + Sync + 'static,
{
    /// Creates a new `DefaultStreamTextResult` with comprehensive configuration.
    ///
    /// This constructor accepts all parameters needed for streaming text generation,
    /// including model configuration, callbacks, transformations, and more.
    ///
    /// # Arguments
    ///
    /// * `params` - All configuration parameters for the stream text result
    ///
    /// # Example
    ///
    /// ```ignore
    /// use ai_sdk_core::stream_text::{DefaultStreamTextResult, DefaultStreamTextResultParams};
    /// use std::sync::Arc;
    ///
    /// let params = DefaultStreamTextResultParams {
    ///     model: Arc::new(my_model),
    ///     headers: None,
    ///     settings: CallSettings::default(),
    ///     max_retries: Some(3),
    ///     abort_signal: None,
    ///     system: Some("You are helpful".to_string()),
    ///     prompt: Some("Hello".to_string()),
    ///     messages: None,
    ///     tools: None,
    ///     tool_choice: None,
    ///     transforms: vec![],
    ///     active_tools: None,
    ///     repair_tool_call: None,
    ///     stop_conditions: vec![],
    ///     output: None,
    ///     provider_options: None,
    ///     prepare_step: None,
    ///     include_raw_chunks: false,
    ///     now: Arc::new(|| 0),
    ///     current_date: Arc::new(|| chrono::Utc::now()),
    ///     generate_id: Arc::new(|| uuid::Uuid::new_v4().to_string()),
    ///     experimental_context: None,
    ///     on_chunk: None,
    ///     on_error: None,
    ///     on_finish: None,
    ///     on_abort: None,
    ///     on_step_finish: None,
    /// };
    ///
    /// let result = DefaultStreamTextResult::new(params);
    /// ```
    pub fn new(params: DefaultStreamTextResultParams<INPUT, OUTPUT>) -> Self {
        // Extract key parameters
        let output = params.output;
        let include_raw_chunks = params.include_raw_chunks;
        let tools = params.tools;

        // Initialize state tracking variables
        // Promise to ensure that the step has been fully processed by the event processor
        // before a new step is started. This is required because the continuation condition
        // needs the updated steps to determine if another step is needed.
        let step_finish: Arc<Mutex<Option<DelayedPromise<()>>>> = Arc::new(Mutex::new(None));

        // Initialize recorded state
        let recorded_content = Arc::new(Mutex::new(Vec::new()));
        let recorded_response_messages = Arc::new(Mutex::new(Vec::new()));
        let recorded_finish_reason = Arc::new(Mutex::new(None));
        let recorded_total_usage = Arc::new(Mutex::new(None));
        let recorded_request = Arc::new(Mutex::new(crate::generate_text::RequestMetadata {
            body: None,
        }));
        let recorded_warnings = Arc::new(Mutex::new(Vec::new()));
        let recorded_steps = Arc::new(Mutex::new(Vec::new()));

        // Initialize active content tracking
        let active_text_content = Arc::new(Mutex::new(HashMap::new()));
        let active_reasoning_content = Arc::new(Mutex::new(HashMap::new()));

        // TODO: Implementation will continue with:
        // 1. Validate and standardize the prompt
        // 2. Prepare tools and tool choice
        // 3. Set up the streaming pipeline with transforms
        // 4. Initialize the base stream
        // 5. Set up callbacks and error handling
        // 6. Start the generation process

        // For now, create a placeholder empty stream
        let empty_stream: AsyncIterableStream<EnrichedStreamPart<INPUT, OUTPUT>> =
            Box::pin(futures_util::stream::empty());

        Self {
            total_usage: DelayedPromise::new(),
            finish_reason: DelayedPromise::new(),
            steps: DelayedPromise::new(),
            base_stream: Arc::new(Mutex::new(Some(empty_stream))),
            include_raw_chunks,
            tools,
            output,
            step_finish,
            recorded_content,
            recorded_response_messages,
            recorded_finish_reason,
            recorded_total_usage,
            recorded_request,
            recorded_warnings,
            recorded_steps,
            active_text_content,
            active_reasoning_content,
        }
    }

    /// Gets the total usage across all steps.
    ///
    /// This method waits until the stream is fully consumed and returns the total
    /// token usage.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let usage = result.total_usage().await?;
    /// println!("Total tokens: {}", usage.total_tokens);
    /// ```
    pub async fn total_usage(&self) -> Usage {
        self.total_usage.await_value().await
    }

    /// Gets the finish reason from the final step.
    ///
    /// This method waits until the stream is fully consumed and returns the finish reason.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let finish_reason = result.finish_reason().await?;
    /// println!("Finished: {:?}", finish_reason);
    /// ```
    pub async fn finish_reason(&self) -> FinishReason {
        self.finish_reason.await_value().await
    }

    /// Gets all steps from the generation.
    ///
    /// This method waits until the stream is fully consumed and returns all step results.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let steps = result.steps().await?;
    /// println!("Total steps: {}", steps.len());
    /// ```
    pub async fn steps(&self) -> Vec<StepResult<INPUT, OUTPUT>> {
        self.steps.await_value().await
    }

    /// Checks if the result includes raw chunks from the provider.
    pub fn includes_raw_chunks(&self) -> bool {
        self.include_raw_chunks
    }

    /// Gets a reference to the tool set, if provided.
    pub fn tools(&self) -> Option<&ToolSet> {
        self.tools.as_ref()
    }

    /// Creates an event processor transform stream that processes all events.
    ///
    /// The event processor:
    /// - Forwards all chunks downstream
    /// - Executes callbacks (onChunk, onError) for relevant events
    /// - Accumulates state (active text/reasoning content, recorded content)
    /// - Tracks usage and finish reason
    ///
    /// This is a simplified version for Phase 2 that handles basic text streaming.
    fn create_event_processor(
        &self,
        on_chunk: Arc<Option<OnChunkCallback<INPUT, OUTPUT>>>,
        on_error: Arc<Option<OnErrorCallback>>,
        active_text_content: Arc<Mutex<HashMap<String, ActiveTextContent>>>,
        active_reasoning_content: Arc<Mutex<HashMap<String, ActiveReasoningContent>>>,
        recorded_content: Arc<Mutex<Vec<crate::generate_text::ContentPart<INPUT, OUTPUT>>>>,
        recorded_finish_reason: Arc<Mutex<Option<FinishReason>>>,
        recorded_total_usage: Arc<Mutex<Option<Usage>>>,
    ) -> impl Fn(
        AsyncIterableStream<EnrichedStreamPart<INPUT, OUTPUT>>,
    ) -> AsyncIterableStream<EnrichedStreamPart<INPUT, OUTPUT>>
           + Send
           + 'static {
        move |input_stream| {
            let on_chunk = on_chunk.clone();
            let on_error = on_error.clone();
            let active_text_content = active_text_content.clone();
            let active_reasoning_content = active_reasoning_content.clone();
            let recorded_content = recorded_content.clone();
            let recorded_finish_reason = recorded_finish_reason.clone();
            let recorded_total_usage = recorded_total_usage.clone();

            Box::pin(async_stream::stream! {
                tokio::pin!(input_stream);

                while let Some(enriched_part) = input_stream.next().await {
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

                    // Track active text content
                    match part {
                        TextStreamPart::TextStart { id, provider_metadata } => {
                            let mut active_texts = active_text_content.lock().await;
                            let content = ActiveTextContent {
                                text: String::new(),
                                provider_metadata: provider_metadata.clone(),
                            };
                            active_texts.insert(id.clone(), content.clone());

                            // Record the text content - we'll update it as deltas come in
                            // For now, we just track it in active_text_content
                        }
                        TextStreamPart::TextDelta { id, text, provider_metadata } => {
                            let mut active_texts = active_text_content.lock().await;
                            if let Some(active_text) = active_texts.get_mut(id) {
                                active_text.text.push_str(text);
                                if provider_metadata.is_some() {
                                    active_text.provider_metadata = provider_metadata.clone();
                                }
                            }
                        }
                        TextStreamPart::TextEnd { id, provider_metadata } => {
                            let mut active_texts = active_text_content.lock().await;
                            if let Some(mut active_text) = active_texts.remove(id) {
                                if provider_metadata.is_some() {
                                    active_text.provider_metadata = provider_metadata.clone();
                                }
                            }
                        }
                        TextStreamPart::ReasoningStart { id, provider_metadata } => {
                            let mut active_reasoning = active_reasoning_content.lock().await;
                            let content = ActiveReasoningContent {
                                text: String::new(),
                                provider_metadata: provider_metadata.clone(),
                            };
                            active_reasoning.insert(id.clone(), content.clone());

                            // Record the reasoning content - we'll update it as deltas come in
                            // For now, we just track it in active_reasoning_content
                        }
                        TextStreamPart::ReasoningDelta { id, text, provider_metadata } => {
                            let mut active_reasoning = active_reasoning_content.lock().await;
                            if let Some(active) = active_reasoning.get_mut(id) {
                                active.text.push_str(text);
                                if provider_metadata.is_some() {
                                    active.provider_metadata = provider_metadata.clone();
                                }
                            }
                        }
                        TextStreamPart::ReasoningEnd { id, provider_metadata } => {
                            let mut active_reasoning = active_reasoning_content.lock().await;
                            if let Some(mut active) = active_reasoning.remove(id) {
                                if provider_metadata.is_some() {
                                    active.provider_metadata = provider_metadata.clone();
                                }
                            }
                        }
                        TextStreamPart::Finish { finish_reason, total_usage, .. } => {
                            // Record finish reason
                            let mut recorded_finish = recorded_finish_reason.lock().await;
                            *recorded_finish = Some(finish_reason.clone());

                            // Record/accumulate usage
                            let mut recorded_usage = recorded_total_usage.lock().await;
                            *recorded_usage = Some(total_usage.clone());
                        }
                        _ => {}
                    }
                }
            })
        }
    }

    // TODO: Additional methods will be added:
    // - add_stream() - Method to add additional streams for multi-step generation
    // - close_stream() - Method to close the stream and resolve delayed promises
    // - Stream accessors (text_stream, full_stream, etc.)
    // - Promise accessors (content, text, reasoning, etc.)
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::stream;

    #[tokio::test]
    async fn test_delayed_promise() {
        let promise = DelayedPromise::new();

        // Check that it's not resolved initially
        assert!(!promise.is_resolved().await);

        // Resolve it in a separate task
        let promise_clone = promise.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            promise_clone.resolve(42).await;
        });

        // Await the value
        let value = promise.await_value().await;
        assert_eq!(value, 42);
        assert!(promise.is_resolved().await);
    }

    // TODO: Add tests for DefaultStreamTextResult constructor once we have
    // mock implementations of LanguageModel and other required dependencies
}
