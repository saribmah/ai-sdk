use crate::error::AISDKError;
use crate::generate_text::{GeneratedFile, RequestMetadata, ResponseMetadata, StepResult};
use crate::output::{Output, ReasoningOutput, TextOutput};
use crate::stream_text::TextStreamPart;
use crate::tool::{ToolCall, ToolResult};
use ai_sdk_provider::language_model::call_warning::LanguageModelCallWarning;
use ai_sdk_provider::language_model::content::source::LanguageModelSource;
use ai_sdk_provider::language_model::finish_reason::LanguageModelFinishReason;
use ai_sdk_provider::language_model::usage::LanguageModelUsage;
use ai_sdk_provider::shared::provider_metadata::SharedProviderMetadata;
use futures_util::StreamExt;
use futures_util::stream::Stream;
use std::pin::Pin;
use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;

/// A type alias for error handlers used in stream consumption.
///
/// Error handlers receive an `AISDKError` and can perform custom error handling logic.
pub type ErrorHandler = Arc<dyn Fn(AISDKError) + Send + Sync>;

/// Options for consuming a stream.
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::stream_text::ConsumeStreamOptions;
///
/// let options = ConsumeStreamOptions {
///     on_error: Some(Arc::new(|error| {
///         eprintln!("Stream error: {}", error);
///     })),
/// };
/// ```
#[derive(Clone)]
pub struct ConsumeStreamOptions {
    /// Optional error handler that will be called if an error occurs during stream consumption.
    pub on_error: Option<ErrorHandler>,
}

impl Default for ConsumeStreamOptions {
    fn default() -> Self {
        Self { on_error: None }
    }
}

impl ConsumeStreamOptions {
    /// Creates a new `ConsumeStreamOptions` with no error handler.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the error handler.
    pub fn with_error_handler(mut self, handler: ErrorHandler) -> Self {
        self.on_error = Some(handler);
        self
    }
}

/// A type alias for async iterable streams that can be used as both an async iterator and a stream.
///
/// This type represents a stream that produces items of type `T` and can be polled
/// using async/await or consumed as a stream.
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::stream_text::AsyncIterableStream;
///
/// let stream: AsyncIterableStream<String> = /* ... */;
///
/// // Use as async iterator
/// while let Some(item) = stream.next().await {
///     println!("Item: {}", item);
/// }
/// ```
pub type AsyncIterableStream<T> = Pin<Box<dyn Stream<Item = T> + Send>>;

/// Internal state shared between all accessors in a `StreamTextResult`.
///
/// This struct holds the accumulated results from consuming the stream, allowing
/// multiple accessors to read the same data without re-consuming the stream.
#[derive(Clone)]
struct StreamState {
    content: Vec<Output>,
    text: String,
    reasoning: Vec<ReasoningOutput>,
    reasoning_text: Option<String>,
    files: Vec<GeneratedFile>,
    sources: Vec<LanguageModelSource>,
    tool_calls: Vec<ToolCall>,
    tool_results: Vec<ToolResult>,
    finish_reason: LanguageModelFinishReason,
    usage: LanguageModelUsage,
    total_usage: LanguageModelUsage,
    warnings: Option<Vec<LanguageModelCallWarning>>,
    steps: Vec<StepResult>,
    request: RequestMetadata,
    response: ResponseMetadata,
    provider_metadata: Option<SharedProviderMetadata>,
}

impl Default for StreamState {
    fn default() -> Self {
        Self {
            content: Vec::new(),
            text: String::new(),
            reasoning: Vec::new(),
            reasoning_text: None,
            files: Vec::new(),
            sources: Vec::new(),
            tool_calls: Vec::new(),
            tool_results: Vec::new(),
            finish_reason: LanguageModelFinishReason::Unknown,
            usage: LanguageModelUsage::default(),
            total_usage: LanguageModelUsage::default(),
            warnings: None,
            steps: Vec::new(),
            request: RequestMetadata::default(),
            response: ResponseMetadata::new(Vec::new()),
            provider_metadata: None,
        }
    }
}

/// A result object for accessing different stream types and additional information.
///
/// This struct provides multiple ways to access streamed data:
/// - Promise-like accessors (e.g., `content()`, `text()`) that automatically consume the stream
/// - Stream accessors (e.g., `text_stream()`, `full_stream()`) for real-time streaming
///
/// All Promise-like accessors share the same underlying consumed stream data, so calling
/// multiple accessors does not require re-consuming the stream.
///
/// # Type Parameters
///
/// * `INPUT` - The input type for tools (defaults to `Value`)
/// * `OUTPUT` - The output type from tools and partial outputs (defaults to `Value`)
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::stream_text::StreamTextResult;
///
/// let result: StreamTextResult<Value, Value> = /* ... */;
///
/// // Access the final text (automatically consumes the stream)
/// let text = result.text().await?;
/// println!("Generated text: {}", text);
///
/// // Or stream text deltas in real-time
/// let mut stream = result.text_stream();
/// while let Some(delta) = stream.next().await {
///     print!("{}", delta);
/// }
/// ```
pub struct StreamTextResult {
    /// Shared state that holds the consumed stream data
    state: Arc<OnceLock<StreamState>>,

    /// The underlying full stream
    full_stream: Arc<Mutex<Option<AsyncIterableStream<TextStreamPart>>>>,

    /// Whether the stream has been consumed
    consumed: Arc<Mutex<bool>>,
}

impl StreamTextResult {
    /// Creates a new `StreamTextResult` from a full stream.
    ///
    /// # Arguments
    ///
    /// * `full_stream` - The stream of text stream parts
    ///
    /// # Example
    ///
    /// ```ignore
    /// use ai_sdk_core::stream_text::StreamTextResult;
    ///
    /// let result = StreamTextResult::new(stream);
    /// ```
    pub fn new(full_stream: AsyncIterableStream<TextStreamPart>) -> Self {
        Self {
            state: Arc::new(OnceLock::new()),
            full_stream: Arc::new(Mutex::new(Some(full_stream))),
            consumed: Arc::new(Mutex::new(false)),
        }
    }

    /// Consumes the stream and populates the internal state.
    ///
    /// This method is called automatically by all Promise-like accessors.
    /// It ensures the stream is only consumed once.
    async fn ensure_consumed(&self) -> Result<(), AISDKError> {
        // Check if already consumed
        if self.state.get().is_some() {
            return Ok(());
        }

        // Lock to ensure only one consumer
        let mut consumed = self.consumed.lock().await;
        if *consumed {
            // Wait for state to be available
            while self.state.get().is_none() {
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }
            return Ok(());
        }

        *consumed = true;
        drop(consumed);

        // Take ownership of the stream
        let stream = self
            .full_stream
            .lock()
            .await
            .take()
            .ok_or_else(|| AISDKError::model_error("Stream already consumed"))?;

        // Consume the stream
        let state = self.consume_stream_internal(stream).await?;

        // Store the state
        let _ = self
            .state
            .set(state)
            .map_err(|_| AISDKError::model_error("Failed to store stream state"))?;

        Ok(())
    }

    /// Internal method to consume the stream and build the state.
    async fn consume_stream_internal(
        &self,
        mut stream: AsyncIterableStream<TextStreamPart>,
    ) -> Result<StreamState, AISDKError> {
        let mut state = StreamState::default();
        let mut current_text = String::new();
        let mut current_reasoning = String::new();

        while let Some(part) = stream.next().await {
            match part {
                TextStreamPart::TextDelta { text, .. } => {
                    current_text.push_str(&text);
                }
                TextStreamPart::TextEnd { .. } => {
                    if !current_text.is_empty() {
                        state.text.push_str(&current_text);
                        // Also add to content
                        state
                            .content
                            .push(Output::Text(TextOutput::new(current_text.clone())));
                        current_text.clear();
                    }
                }
                TextStreamPart::ReasoningDelta { text, .. } => {
                    current_reasoning.push_str(&text);
                }
                TextStreamPart::ReasoningEnd { .. } => {
                    if !current_reasoning.is_empty() {
                        state
                            .reasoning
                            .push(ReasoningOutput::new(current_reasoning.clone()));
                        // Also add to content
                        state.content.push(Output::Reasoning(ReasoningOutput::new(
                            current_reasoning.clone(),
                        )));
                        current_reasoning.clear();
                    }
                }
                TextStreamPart::Source { source } => {
                    state.sources.push(source.source.clone());
                    // Also add to content
                    state.content.push(Output::Source(source));
                }
                TextStreamPart::File { file } => {
                    state
                        .files
                        .push(GeneratedFile::from_base64(&file.base64, &file.media_type));
                }
                TextStreamPart::ToolCall { tool_call } => {
                    // Add to content and tool_calls list
                    state.content.push(Output::ToolCall(tool_call.clone()));
                    state.tool_calls.push(tool_call);
                }
                TextStreamPart::ToolResult { tool_result } => {
                    // Add to content and tool_results list
                    state.content.push(Output::ToolResult(tool_result.clone()));
                    state.tool_results.push(tool_result);
                }
                TextStreamPart::StartStep { request, warnings } => {
                    state.request = request;
                    if !warnings.is_empty() {
                        state.warnings = Some(warnings);
                    }
                }
                TextStreamPart::FinishStep {
                    response,
                    usage,
                    finish_reason,
                    provider_metadata,
                } => {
                    state.finish_reason = finish_reason;
                    state.usage = usage;
                    state.response = ResponseMetadata::new(Vec::new())
                        .with_id(response.id.unwrap_or_default())
                        .with_model_id(response.model_id.unwrap_or_default());
                    if let Some(timestamp) = response.timestamp {
                        state.response = state.response.with_timestamp(timestamp);
                    }
                    state.provider_metadata = provider_metadata;
                }
                TextStreamPart::Finish {
                    total_usage,
                    finish_reason,
                } => {
                    state.total_usage = total_usage;
                    state.finish_reason = finish_reason;
                }
                TextStreamPart::Error { error } => {
                    return Err(AISDKError::model_error(format!("Stream error: {}", error)));
                }
                _ => {
                    // Handle other variants as needed
                }
            }
        }

        // Flush any remaining text
        if !current_text.is_empty() {
            state.text.push_str(&current_text);
            state
                .content
                .push(Output::Text(TextOutput::new(current_text)));
        }

        // Flush any remaining reasoning
        if !current_reasoning.is_empty() {
            state
                .reasoning
                .push(ReasoningOutput::new(current_reasoning.clone()));
            state
                .content
                .push(Output::Reasoning(ReasoningOutput::new(current_reasoning)));
        }

        // Finalize reasoning text
        if !state.reasoning.is_empty() {
            state.reasoning_text = Some(
                state
                    .reasoning
                    .iter()
                    .map(|r| r.text.as_str())
                    .collect::<Vec<_>>()
                    .join(""),
            );
        }

        Ok(state)
    }

    /// Gets the content that was generated in the last step.
    ///
    /// Automatically consumes the stream.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let content = result.content().await?;
    /// for part in content {
    ///     // Process content parts
    /// }
    /// ```
    pub async fn content(&self) -> Result<Vec<Output>, AISDKError> {
        self.ensure_consumed().await?;
        Ok(self.state.get().unwrap().content.clone())
    }

    /// Gets the full text that has been generated by the last step.
    ///
    /// Automatically consumes the stream.
    pub async fn text(&self) -> Result<String, AISDKError> {
        self.ensure_consumed().await?;
        Ok(self.state.get().unwrap().text.clone())
    }

    /// Gets the full reasoning that the model has generated.
    ///
    /// Automatically consumes the stream.
    pub async fn reasoning(&self) -> Result<Vec<ReasoningOutput>, AISDKError> {
        self.ensure_consumed().await?;
        Ok(self.state.get().unwrap().reasoning.clone())
    }

    /// Gets the reasoning text that has been generated by the last step.
    ///
    /// Automatically consumes the stream.
    pub async fn reasoning_text(&self) -> Result<Option<String>, AISDKError> {
        self.ensure_consumed().await?;
        Ok(self.state.get().unwrap().reasoning_text.clone())
    }

    /// Gets the files that have been generated by the model in the last step.
    ///
    /// Automatically consumes the stream.
    pub async fn files(&self) -> Result<Vec<GeneratedFile>, AISDKError> {
        self.ensure_consumed().await?;
        Ok(self.state.get().unwrap().files.clone())
    }

    /// Gets the sources that have been used as references in the last step.
    ///
    /// Automatically consumes the stream.
    pub async fn sources(&self) -> Result<Vec<LanguageModelSource>, AISDKError> {
        self.ensure_consumed().await?;
        Ok(self.state.get().unwrap().sources.clone())
    }

    /// Gets the tool calls that have been executed in the last step.
    ///
    /// Automatically consumes the stream.
    pub async fn tool_calls(&self) -> Result<Vec<ToolCall>, AISDKError> {
        self.ensure_consumed().await?;
        Ok(self.state.get().unwrap().tool_calls.clone())
    }

    /// Gets the tool results that have been generated in the last step.
    ///
    /// Automatically consumes the stream.
    pub async fn tool_results(&self) -> Result<Vec<ToolResult>, AISDKError> {
        self.ensure_consumed().await?;
        Ok(self.state.get().unwrap().tool_results.clone())
    }

    /// Gets the reason why the generation finished. Taken from the last step.
    ///
    /// Automatically consumes the stream.
    pub async fn finish_reason(&self) -> Result<LanguageModelFinishReason, AISDKError> {
        self.ensure_consumed().await?;
        Ok(self.state.get().unwrap().finish_reason.clone())
    }

    /// Gets the token usage of the last step.
    ///
    /// Automatically consumes the stream.
    pub async fn usage(&self) -> Result<LanguageModelUsage, AISDKError> {
        self.ensure_consumed().await?;
        Ok(self.state.get().unwrap().usage.clone())
    }

    /// Gets the total token usage of the generated response.
    /// When there are multiple steps, the usage is the sum of all step usages.
    ///
    /// Automatically consumes the stream.
    pub async fn total_usage(&self) -> Result<LanguageModelUsage, AISDKError> {
        self.ensure_consumed().await?;
        Ok(self.state.get().unwrap().total_usage.clone())
    }

    /// Gets warnings from the model provider (e.g. unsupported settings) for the first step.
    ///
    /// Automatically consumes the stream.
    pub async fn warnings(&self) -> Result<Option<Vec<LanguageModelCallWarning>>, AISDKError> {
        self.ensure_consumed().await?;
        Ok(self.state.get().unwrap().warnings.clone())
    }

    /// Gets details for all steps.
    /// You can use this to get information about intermediate steps,
    /// such as the tool calls or the response headers.
    ///
    /// Automatically consumes the stream.
    pub async fn steps(&self) -> Result<Vec<StepResult>, AISDKError> {
        self.ensure_consumed().await?;
        Ok(self.state.get().unwrap().steps.clone())
    }

    /// Gets additional request information from the last step.
    ///
    /// Automatically consumes the stream.
    pub async fn request(&self) -> Result<RequestMetadata, AISDKError> {
        self.ensure_consumed().await?;
        Ok(self.state.get().unwrap().request.clone())
    }

    /// Gets additional response information from the last step.
    ///
    /// Automatically consumes the stream.
    pub async fn response(&self) -> Result<ResponseMetadata, AISDKError> {
        self.ensure_consumed().await?;
        Ok(self.state.get().unwrap().response.clone())
    }

    /// Gets additional provider-specific metadata from the last step.
    ///
    /// Automatically consumes the stream.
    pub async fn provider_metadata(&self) -> Result<Option<SharedProviderMetadata>, AISDKError> {
        self.ensure_consumed().await?;
        Ok(self.state.get().unwrap().provider_metadata.clone())
    }

    /// Gets a text stream that returns only the generated text deltas.
    ///
    /// You can use it as either an AsyncIterable or a Stream. When an error occurs,
    /// the stream will yield the error and then terminate.
    ///
    /// Note: This consumes the full stream, so it cannot be called after other
    /// consuming operations.
    pub fn text_stream(&self) -> AsyncIterableStream<String> {
        let full_stream = self.full_stream.clone();

        Box::pin(async_stream::stream! {
            let mut stream = full_stream.lock().await;
            if let Some(mut s) = stream.take() {
                while let Some(part) = s.next().await {
                    if let TextStreamPart::TextDelta { text, .. } = part {
                        yield text;
                    }
                }
            }
        })
    }

    /// Gets a stream with all events, including text deltas, tool calls, tool results, and errors.
    ///
    /// You can use it as either an AsyncIterable or a Stream.
    /// Only errors that stop the stream, such as network errors, are yielded.
    pub fn full_stream(&self) -> AsyncIterableStream<TextStreamPart> {
        let full_stream = self.full_stream.clone();

        Box::pin(async_stream::stream! {
            let mut stream = full_stream.lock().await;
            if let Some(mut s) = stream.take() {
                while let Some(part) = s.next().await {
                    yield part;
                }
            }
        })
    }

    /// Gets a stream of partial outputs parsed as JSON values.
    ///
    /// This streams partial JSON objects as they are being constructed from the text output.
    /// It attempts to parse incomplete JSON by repairing it (closing brackets/braces).
    ///
    /// Note: This is the Rust equivalent of `partialOutputStream` from the TypeScript SDK.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut partial_stream = result.partial_output_stream();
    /// while let Some(partial_value) = partial_stream.next().await {
    ///     println!("Partial JSON: {:?}", partial_value);
    /// }
    /// ```
    pub fn partial_output_stream<OUTPUT>(&self) -> AsyncIterableStream<OUTPUT>
    where
        OUTPUT: for<'de> serde::Deserialize<'de> + Send + 'static,
    {
        use crate::stream_text::output::repair_partial_json;

        let full_stream = self.full_stream.clone();

        Box::pin(async_stream::stream! {
            let mut stream = full_stream.lock().await;
            if let Some(mut s) = stream.take() {
                let mut accumulated_text = String::new();
                let mut last_parsed: Option<String> = None;

                while let Some(part) = s.next().await {
                    // Accumulate text deltas
                    if let TextStreamPart::TextDelta { text, .. } = part {
                        accumulated_text.push_str(&text);

                        // Try to parse the accumulated text
                        let repaired = repair_partial_json(&accumulated_text);

                        // Only emit if we haven't already emitted this exact JSON
                        if Some(&repaired) != last_parsed.as_ref() {
                            if let Ok(value) = serde_json::from_str::<OUTPUT>(&repaired) {
                                last_parsed = Some(repaired);
                                yield value;
                            }
                        }
                    }
                }
            }
        })
    }

    /// Consumes the stream without processing the parts.
    ///
    /// This is useful to force the stream to finish. It effectively removes the backpressure
    /// and allows the stream to finish, triggering the `onFinish` callback and the promise resolution.
    ///
    /// If an error occurs, it is passed to the optional `onError` callback.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional configuration for error handling
    ///
    /// # Example
    ///
    /// ```ignore
    /// use ai_sdk_core::stream_text::{ConsumeStreamOptions, StreamTextResult};
    /// use std::sync::Arc;
    ///
    /// let result: StreamTextResult = /* ... */;
    ///
    /// // Consume without error handling
    /// result.consume_stream(None).await?;
    ///
    /// // Consume with error handling
    /// let options = ConsumeStreamOptions {
    ///     on_error: Some(Arc::new(|error| {
    ///         eprintln!("Error: {}", error);
    ///     })),
    /// };
    /// result.consume_stream(Some(options)).await?;
    /// ```
    pub async fn consume_stream(
        &self,
        options: Option<ConsumeStreamOptions>,
    ) -> Result<(), AISDKError> {
        let result = self.ensure_consumed().await;

        if let Err(e) = result {
            if let Some(opts) = options {
                if let Some(handler) = opts.on_error {
                    handler(e.clone());
                }
            }
            return Err(e);
        }

        Ok(())
    }
}

impl Clone for StreamTextResult {
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
            full_stream: Arc::clone(&self.full_stream),
            consumed: Arc::clone(&self.consumed),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::stream;

    #[tokio::test]
    async fn test_stream_text_result_text() {
        let parts = vec![
            TextStreamPart::TextStart {
                id: "text1".to_string(),
                provider_metadata: None,
            },
            TextStreamPart::TextDelta {
                id: "text1".to_string(),
                provider_metadata: None,
                text: "Hello ".to_string(),
            },
            TextStreamPart::TextDelta {
                id: "text1".to_string(),
                provider_metadata: None,
                text: "world".to_string(),
            },
            TextStreamPart::TextEnd {
                id: "text1".to_string(),
                provider_metadata: None,
            },
        ];

        let stream: AsyncIterableStream<TextStreamPart> = Box::pin(stream::iter(parts));
        let result = StreamTextResult::new(stream);

        let text = result.text().await.unwrap();
        assert_eq!(text, "Hello world");
    }

    #[tokio::test]
    async fn test_stream_text_result_reasoning() {
        let parts = vec![
            TextStreamPart::ReasoningStart {
                id: "reasoning1".to_string(),
                provider_metadata: None,
            },
            TextStreamPart::ReasoningDelta {
                id: "reasoning1".to_string(),
                provider_metadata: None,
                text: "Thinking... ".to_string(),
            },
            TextStreamPart::ReasoningDelta {
                id: "reasoning1".to_string(),
                provider_metadata: None,
                text: "Done.".to_string(),
            },
            TextStreamPart::ReasoningEnd {
                id: "reasoning1".to_string(),
                provider_metadata: None,
            },
        ];

        let stream: AsyncIterableStream<TextStreamPart> = Box::pin(stream::iter(parts));
        let result = StreamTextResult::new(stream);

        let reasoning = result.reasoning().await.unwrap();
        assert_eq!(reasoning.len(), 1);
        assert_eq!(reasoning[0].text, "Thinking... Done.");

        let reasoning_text = result.reasoning_text().await.unwrap();
        assert_eq!(reasoning_text, Some("Thinking... Done.".to_string()));
    }

    #[tokio::test]
    async fn test_stream_text_result_finish_reason() {
        let parts = vec![TextStreamPart::Finish {
            finish_reason: LanguageModelFinishReason::Stop,
            total_usage: LanguageModelUsage::new(10, 20),
        }];

        let stream: AsyncIterableStream<TextStreamPart> = Box::pin(stream::iter(parts));
        let result = StreamTextResult::new(stream);

        let finish_reason = result.finish_reason().await.unwrap();
        assert_eq!(finish_reason, LanguageModelFinishReason::Stop);

        let total_usage = result.total_usage().await.unwrap();
        assert_eq!(total_usage.input_tokens, 10);
        assert_eq!(total_usage.output_tokens, 20);
    }

    #[tokio::test]
    async fn test_stream_text_result_multiple_accessors() {
        let parts = vec![
            TextStreamPart::TextDelta {
                id: "text1".to_string(),
                provider_metadata: None,
                text: "Hello".to_string(),
            },
            TextStreamPart::TextEnd {
                id: "text1".to_string(),
                provider_metadata: None,
            },
            TextStreamPart::Finish {
                finish_reason: LanguageModelFinishReason::Stop,
                total_usage: LanguageModelUsage::new(5, 10),
            },
        ];

        let stream: AsyncIterableStream<TextStreamPart> = Box::pin(stream::iter(parts));
        let result = StreamTextResult::new(stream);

        // Multiple accessors should work without re-consuming the stream
        let text1 = result.text().await.unwrap();
        let text2 = result.text().await.unwrap();
        let finish = result.finish_reason().await.unwrap();

        assert_eq!(text1, "Hello");
        assert_eq!(text2, "Hello");
        assert_eq!(finish, LanguageModelFinishReason::Stop);
    }

    #[tokio::test]
    async fn test_consume_stream() {
        let parts = vec![
            TextStreamPart::TextDelta {
                id: "text1".to_string(),
                provider_metadata: None,
                text: "Test".to_string(),
            },
            TextStreamPart::Finish {
                finish_reason: LanguageModelFinishReason::Stop,
                total_usage: LanguageModelUsage::new(1, 1),
            },
        ];

        let stream: AsyncIterableStream<TextStreamPart> = Box::pin(stream::iter(parts));
        let result = StreamTextResult::new(stream);

        // Consume the stream
        result.consume_stream(None).await.unwrap();

        // Verify we can still access the data
        let text = result.text().await.unwrap();
        assert_eq!(text, "Test");
    }

    #[tokio::test]
    async fn test_consume_stream_with_error_handler() {
        use std::sync::atomic::{AtomicBool, Ordering};

        let error_handled = Arc::new(AtomicBool::new(false));
        let error_handled_clone = error_handled.clone();

        let parts = vec![TextStreamPart::Error {
            error: serde_json::json!({"message": "Test error"}),
        }];

        let stream: AsyncIterableStream<TextStreamPart> = Box::pin(stream::iter(parts));
        let result = StreamTextResult::new(stream);

        let options = ConsumeStreamOptions {
            on_error: Some(Arc::new(move |_error| {
                error_handled_clone.store(true, Ordering::SeqCst);
            })),
        };

        let _ = result.consume_stream(Some(options)).await;

        // Verify error handler was called
        assert!(error_handled.load(Ordering::SeqCst));
    }
}
