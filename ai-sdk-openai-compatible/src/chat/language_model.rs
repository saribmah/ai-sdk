use ai_sdk_provider::language_model::stream_part::LanguageModelStreamPart;
use ai_sdk_provider::language_model::{
    LanguageModel, LanguageModelGenerateResponse, LanguageModelStreamResponse,
    call_options::LanguageModelCallOptions, call_warning::LanguageModelCallWarning,
    content::LanguageModelContent, content::reasoning::LanguageModelReasoning,
    content::text::LanguageModelText, content::tool_call::LanguageModelToolCall,
    usage::LanguageModelUsage,
};
use async_trait::async_trait;
use futures_util::{Stream, StreamExt};
use regex::Regex;
use reqwest;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;

use crate::chat::{
    OpenAICompatibleChatModelId, convert_to_openai_compatible_chat_messages, prepare_tools,
};
use crate::utils::finish_reason::map_openai_compatible_finish_reason;
use crate::utils::response_metadata::get_response_metadata;

/// Type alias for URL generation function
pub type UrlGeneratorFn = Box<dyn Fn(&str, &str) -> String + Send + Sync>;

/// Type alias for supported URLs function
pub type SupportedUrlsFn = fn() -> HashMap<String, Vec<Regex>>;

/// Configuration for an OpenAI-compatible chat language model
pub struct OpenAICompatibleChatConfig {
    /// Provider name (e.g., "openai", "azure", "custom")
    pub provider: String,

    /// Function to generate headers for API requests
    pub headers: Box<dyn Fn() -> HashMap<String, String> + Send + Sync>,

    /// Function to generate the URL for API requests
    pub url: UrlGeneratorFn,

    /// Whether to include usage information in streaming responses
    pub include_usage: bool,

    /// Whether the model supports structured outputs
    pub supports_structured_outputs: bool,

    /// Function to get supported URLs for the model
    pub supported_urls: Option<SupportedUrlsFn>,
}

impl Default for OpenAICompatibleChatConfig {
    fn default() -> Self {
        Self {
            provider: "openai-compatible".to_string(),
            headers: Box::new(HashMap::new),
            url: Box::new(|_model_id, path| format!("https://api.openai.com/v1{}", path)),
            include_usage: false,
            supports_structured_outputs: false,
            supported_urls: None,
        }
    }
}

/// OpenAI-compatible chat language model implementation
pub struct OpenAICompatibleChatLanguageModel {
    /// The model identifier
    model_id: OpenAICompatibleChatModelId,

    /// Configuration for the model
    config: OpenAICompatibleChatConfig,
}

impl OpenAICompatibleChatLanguageModel {
    /// Create a new OpenAI-compatible chat language model
    ///
    /// # Arguments
    ///
    /// * `model_id` - The model identifier (e.g., "gpt-4", "gpt-3.5-turbo")
    /// * `config` - Configuration for the model
    ///
    /// # Returns
    ///
    /// A new `OpenAICompatibleChatLanguageModel` instance
    pub fn new(model_id: OpenAICompatibleChatModelId, config: OpenAICompatibleChatConfig) -> Self {
        Self { model_id, config }
    }

    /// Get the provider options name (first part of provider string before '.')
    fn provider_options_name(&self) -> &str {
        self.config
            .provider
            .split('.')
            .next()
            .unwrap_or(&self.config.provider)
    }

    /// Process SSE byte stream and convert to StreamPart events
    fn process_stream(
        byte_stream: impl Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Send + 'static,
        warnings: Vec<LanguageModelCallWarning>,
    ) -> impl Stream<Item = LanguageModelStreamPart> + Unpin + Send {
        let mut buffer = String::new();
        let mut state = StreamState {
            text_id: None,
            reasoning_id: None,
            tool_calls: HashMap::new(),
        };

        Box::pin(async_stream::stream! {
            // Emit stream start with warnings
            // This is outside the event loop and only emitted once per stream instance
            yield LanguageModelStreamPart::stream_start(warnings);

            let mut stream = Box::pin(byte_stream);

            while let Some(result) = stream.next().await {
                match result {
                    Ok(bytes) => {
                        // Append to buffer
                        if let Ok(text) = std::str::from_utf8(&bytes) {
                            buffer.push_str(text);

                            // Process complete lines
                            while let Some(pos) = buffer.find('\n') {
                                let line = buffer[..pos].trim().to_string();
                                buffer.drain(..=pos);

                                // Skip empty lines
                                if line.is_empty() {
                                    continue;
                                }

                                // Parse SSE format
                                if let Some(data) = line.strip_prefix("data: ") {
                                    // Check for [DONE] marker
                                    if data == "[DONE]" {
                                        break;
                                    }

                                    // Parse JSON chunk
                                    if let Ok(chunk) = serde_json::from_str::<OpenAIStreamChunk>(data) {
                                        // Process the chunk and emit stream parts
                                        for part in Self::process_chunk(&mut state, chunk) {
                                            yield part;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        yield LanguageModelStreamPart::error(json!({ "message": e.to_string() }));
                        break;
                    }
                }
            }
        })
    }

    /// Process a single streaming chunk and emit StreamPart events
    fn process_chunk(
        state: &mut StreamState,
        chunk: OpenAIStreamChunk,
    ) -> Vec<LanguageModelStreamPart> {
        let mut parts = Vec::new();

        // Get the first choice (we only handle single choice for now)
        let choice = match chunk.choices.first() {
            Some(c) => c,
            None => return parts,
        };

        let delta = &choice.delta;

        // Handle text content
        if let Some(content) = &delta.content
            && !content.is_empty()
        {
            if state.text_id.is_none() {
                let id = format!("text-{}", uuid::Uuid::new_v4());
                parts.push(LanguageModelStreamPart::text_start(&id));
                state.text_id = Some(id.clone());
            }

            if let Some(id) = &state.text_id {
                parts.push(LanguageModelStreamPart::text_delta(id, content));
            }
        }

        // Handle reasoning content
        let reasoning = delta
            .reasoning_content
            .as_ref()
            .or(delta.reasoning.as_ref());
        if let Some(reasoning_text) = reasoning
            && !reasoning_text.is_empty()
        {
            if state.reasoning_id.is_none() {
                let id = format!("reasoning-{}", uuid::Uuid::new_v4());
                parts.push(LanguageModelStreamPart::reasoning_start(&id));
                state.reasoning_id = Some(id.clone());
            }

            if let Some(id) = &state.reasoning_id {
                parts.push(LanguageModelStreamPart::reasoning_delta(id, reasoning_text));
            }
        }

        // Handle tool calls
        if let Some(tool_calls) = &delta.tool_calls {
            for tool_call in tool_calls {
                let index = tool_call.index.unwrap_or(0) as usize;

                // Get or create tool call state
                if !state.tool_calls.contains_key(&index)
                    && let Some(id) = &tool_call.id
                {
                    state.tool_calls.insert(
                        index,
                        ToolCallState {
                            id: id.clone(),
                            name: String::new(),
                            arguments: String::new(),
                            started: false,
                        },
                    );
                }

                if let Some(tool_state) = state.tool_calls.get_mut(&index) {
                    // Update tool name if present
                    if let Some(function) = &tool_call.function {
                        if let Some(name) = &function.name
                            && !name.is_empty()
                        {
                            tool_state.name = name.clone();
                        }

                        // Emit start event if we have both ID and name
                        if !tool_state.started
                            && !tool_state.id.is_empty()
                            && !tool_state.name.is_empty()
                        {
                            parts.push(LanguageModelStreamPart::tool_input_start(
                                &tool_state.id,
                                &tool_state.name,
                            ));
                            tool_state.started = true;
                        }

                        // Handle arguments delta
                        if let Some(args) = &function.arguments
                            && !args.is_empty()
                        {
                            tool_state.arguments.push_str(args);
                            if tool_state.started {
                                parts.push(LanguageModelStreamPart::tool_input_delta(
                                    &tool_state.id,
                                    args,
                                ));
                            }
                        }
                    }
                }
            }
        }

        // Handle finish reason
        if let Some(finish_reason) = &choice.finish_reason {
            // End any open text block
            if let Some(id) = state.text_id.take() {
                parts.push(LanguageModelStreamPart::text_end(&id));
            }

            // End any open reasoning block
            if let Some(id) = state.reasoning_id.take() {
                parts.push(LanguageModelStreamPart::reasoning_end(&id));
            }

            // End all open tool calls and emit ToolCall parts
            for tool_state in state.tool_calls.values() {
                if tool_state.started {
                    parts.push(LanguageModelStreamPart::tool_input_end(&tool_state.id));

                    // Emit the actual ToolCall with the complete arguments
                    let tool_call = LanguageModelToolCall::new(
                        &tool_state.id,
                        &tool_state.name,
                        &tool_state.arguments,
                    );
                    parts.push(LanguageModelStreamPart::ToolCall(tool_call));
                }
            }

            // Build usage information
            let usage = if let Some(api_usage) = &chunk.usage {
                LanguageModelUsage {
                    input_tokens: api_usage.prompt_tokens.unwrap_or(0),
                    output_tokens: api_usage.completion_tokens.unwrap_or(0),
                    total_tokens: api_usage.total_tokens.unwrap_or(0),
                    reasoning_tokens: api_usage
                        .completion_tokens_details
                        .as_ref()
                        .and_then(|d| d.reasoning_tokens)
                        .unwrap_or(0),
                    cached_input_tokens: api_usage
                        .prompt_tokens_details
                        .as_ref()
                        .and_then(|d| d.cached_tokens)
                        .unwrap_or(0),
                }
            } else {
                LanguageModelUsage::default()
            };

            // Map finish reason
            let mapped_finish_reason = map_openai_compatible_finish_reason(Some(finish_reason));

            // Emit finish event
            parts.push(LanguageModelStreamPart::finish(usage, mapped_finish_reason));
        }

        parts
    }

    /// Prepare arguments for API request
    fn prepare_request_body(
        &self,
        options: &LanguageModelCallOptions,
    ) -> Result<(Value, Vec<LanguageModelCallWarning>), Box<dyn std::error::Error>> {
        let mut warnings = Vec::new();

        // Prepare tools
        let tools_result = prepare_tools(options.tools.clone(), options.tool_choice.clone());
        warnings.extend(tools_result.tool_warnings);

        // Convert prompt to OpenAI format
        let messages = convert_to_openai_compatible_chat_messages(options.prompt.clone())?;

        // Build request body
        let mut body = json!({
            "model": self.model_id,
            "messages": messages,
        });

        // Add optional parameters
        if let Some(max_tokens) = options.max_output_tokens {
            body["max_tokens"] = json!(max_tokens);
        }
        if let Some(temperature) = options.temperature {
            body["temperature"] = json!(temperature);
        }
        if let Some(top_p) = options.top_p {
            body["top_p"] = json!(top_p);
        }
        if let Some(frequency_penalty) = options.frequency_penalty {
            body["frequency_penalty"] = json!(frequency_penalty);
        }
        if let Some(presence_penalty) = options.presence_penalty {
            body["presence_penalty"] = json!(presence_penalty);
        }
        if let Some(seed) = options.seed {
            body["seed"] = json!(seed);
        }
        if let Some(stop_sequences) = &options.stop_sequences {
            body["stop"] = json!(stop_sequences);
        }

        // Add tools if present
        if let Some(tools) = &tools_result.tools {
            body["tools"] = serde_json::to_value(tools)?;
        }
        if let Some(tool_choice) = &tools_result.tool_choice {
            body["tool_choice"] = serde_json::to_value(tool_choice)?;
        }

        // Add response_format if specified and supported
        if let Some(response_format) = &options.response_format
            && self.config.supports_structured_outputs
        {
            use ai_sdk_provider::language_model::call_options::LanguageModelResponseFormat;

            let format_json = match response_format {
                LanguageModelResponseFormat::Text => {
                    // Text is the default, no need to specify
                    None
                }
                LanguageModelResponseFormat::Json {
                    schema,
                    name,
                    description,
                } => {
                    if let Some(schema_value) = schema {
                        // Structured output with JSON schema
                        let mut json_schema = json!({
                            "type": "json_schema",
                            "json_schema": {
                                "schema": schema_value,
                                "strict": true
                            }
                        });

                        // Add optional name and description
                        if let Some(name_str) = name {
                            json_schema["json_schema"]["name"] = json!(name_str);
                        }
                        if let Some(desc_str) = description {
                            json_schema["json_schema"]["description"] = json!(desc_str);
                        }

                        Some(json_schema)
                    } else {
                        // Simple JSON mode without schema
                        Some(json!({
                            "type": "json_object"
                        }))
                    }
                }
            };

            if let Some(format) = format_json {
                body["response_format"] = format;
            }
        }

        Ok((body, warnings))
    }
}

/// OpenAI API response structure
#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    id: Option<String>,
    created: Option<i64>,
    model: Option<String>,
    choices: Vec<OpenAIChoice>,
    usage: Option<OpenAIUsage>,
}

#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAIMessage {
    #[allow(dead_code)]
    role: Option<String>,
    content: Option<String>,
    reasoning_content: Option<String>,
    reasoning: Option<String>,
    tool_calls: Option<Vec<OpenAIToolCall>>,
}

#[derive(Debug, Deserialize)]
struct OpenAIToolCall {
    id: Option<String>,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    tool_type: Option<String>,
    function: OpenAIFunction,
}

#[derive(Debug, Deserialize)]
struct OpenAIFunction {
    name: String,
    arguments: String,
}

#[derive(Debug, Deserialize)]
struct OpenAIUsage {
    prompt_tokens: Option<u64>,
    completion_tokens: Option<u64>,
    total_tokens: Option<u64>,
    completion_tokens_details: Option<OpenAICompletionTokensDetails>,
    prompt_tokens_details: Option<OpenAIPromptTokensDetails>,
}

#[derive(Debug, Deserialize)]
struct OpenAICompletionTokensDetails {
    reasoning_tokens: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct OpenAIPromptTokensDetails {
    cached_tokens: Option<u64>,
}

/// OpenAI streaming response chunk
#[derive(Debug, Deserialize)]
struct OpenAIStreamChunk {
    #[allow(dead_code)]
    id: Option<String>,
    #[allow(dead_code)]
    created: Option<i64>,
    #[allow(dead_code)]
    model: Option<String>,
    choices: Vec<OpenAIStreamChoice>,
    usage: Option<OpenAIUsage>,
}

#[derive(Debug, Deserialize)]
struct OpenAIStreamChoice {
    #[allow(dead_code)]
    index: Option<u64>,
    delta: OpenAIStreamDelta,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OpenAIStreamDelta {
    #[allow(dead_code)]
    role: Option<String>,
    content: Option<String>,
    reasoning_content: Option<String>,
    reasoning: Option<String>,
    tool_calls: Option<Vec<OpenAIStreamToolCall>>,
}

#[derive(Debug, Deserialize)]
struct OpenAIStreamToolCall {
    index: Option<u64>,
    id: Option<String>,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    tool_type: Option<String>,
    function: Option<OpenAIStreamFunction>,
}

#[derive(Debug, Deserialize)]
struct OpenAIStreamFunction {
    name: Option<String>,
    arguments: Option<String>,
}

/// Helper struct to track streaming state across chunks
struct StreamState {
    text_id: Option<String>,
    reasoning_id: Option<String>,
    tool_calls: HashMap<usize, ToolCallState>,
}

struct ToolCallState {
    id: String,
    name: String,
    arguments: String,
    started: bool,
}

#[async_trait]
impl LanguageModel for OpenAICompatibleChatLanguageModel {
    fn specification_version(&self) -> &str {
        "v3"
    }

    fn provider(&self) -> &str {
        &self.config.provider
    }

    fn model_id(&self) -> &str {
        &self.model_id
    }

    async fn supported_urls(&self) -> HashMap<String, Vec<Regex>> {
        if let Some(supported_urls_fn) = self.config.supported_urls {
            supported_urls_fn()
        } else {
            HashMap::new()
        }
    }

    async fn do_generate(
        &self,
        options: LanguageModelCallOptions,
    ) -> Result<LanguageModelGenerateResponse, Box<dyn std::error::Error>> {
        // Check if already cancelled before starting
        if let Some(signal) = &options.abort_signal
            && signal.is_cancelled()
        {
            return Err("Operation cancelled".into());
        }

        // Prepare request body
        let (body, warnings) = self.prepare_request_body(&options)?;
        let body_string = serde_json::to_string(&body)?;

        // Build URL
        let url = (self.config.url)(&self.model_id, "/chat/completions");

        // Build headers
        let mut headers = (self.config.headers)();
        if let Some(option_headers) = &options.headers {
            headers.extend(option_headers.clone());
        }

        // Make API request
        let client = reqwest::Client::new();
        let mut request = client.post(&url).header("Content-Type", "application/json");

        for (key, value) in headers {
            request = request.header(key, value);
        }

        // Send request with optional cancellation support
        let response = if let Some(signal) = &options.abort_signal {
            tokio::select! {
                result = request.body(body_string.clone()).send() => result?,
                _ = signal.cancelled() => {
                    return Err("Operation cancelled".into());
                }
            }
        } else {
            request.body(body_string.clone()).send().await?
        };
        let status = response.status();
        let response_headers = response.headers().clone();

        if !status.is_success() {
            let error_body = response.text().await?;

            // Check for Retry-After header to include in error message
            let retry_info = if let Some(retry_after) = response_headers.get("retry-after") {
                if let Ok(retry_str) = retry_after.to_str() {
                    // Try to parse as seconds (integer)
                    if let Ok(seconds) = retry_str.parse::<u64>() {
                        format!(" (Retry after {} seconds)", seconds)
                    } else {
                        // Could be HTTP-date format, but for now just include the raw value
                        format!(" (Retry-After: {})", retry_str)
                    }
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            return Err(format!(
                "API request failed with status {}: {}{}",
                status, error_body, retry_info
            )
            .into());
        }

        let response_body = response.text().await?;
        let api_response: OpenAIResponse = serde_json::from_str(&response_body)?;

        // Extract content from response
        let choice = api_response
            .choices
            .first()
            .ok_or("No choices in response")?;

        let mut content = Vec::new();

        // Add text content
        if let Some(text) = &choice.message.content
            && !text.is_empty()
        {
            content.push(LanguageModelContent::Text(LanguageModelText::new(
                text.clone(),
            )));
        }

        // Add reasoning content
        let reasoning = choice
            .message
            .reasoning_content
            .as_ref()
            .or(choice.message.reasoning.as_ref());
        if let Some(reasoning_text) = reasoning
            && !reasoning_text.is_empty()
        {
            content.push(LanguageModelContent::Reasoning(
                LanguageModelReasoning::init(reasoning_text.clone()),
            ));
        }

        // Add tool calls
        if let Some(tool_calls) = &choice.message.tool_calls {
            for tool_call in tool_calls {
                let tool_call_id = tool_call
                    .id
                    .clone()
                    .unwrap_or_else(|| "unknown".to_string());
                let tool_name = tool_call.function.name.clone();
                let input = tool_call.function.arguments.clone();
                content.push(LanguageModelContent::ToolCall(LanguageModelToolCall::new(
                    tool_call_id,
                    tool_name,
                    input,
                )));
            }
        }

        // Build usage information
        let usage = if let Some(api_usage) = &api_response.usage {
            LanguageModelUsage {
                input_tokens: api_usage.prompt_tokens.unwrap_or(0),
                output_tokens: api_usage.completion_tokens.unwrap_or(0),
                total_tokens: api_usage.total_tokens.unwrap_or(0),
                reasoning_tokens: api_usage
                    .completion_tokens_details
                    .as_ref()
                    .and_then(|d| d.reasoning_tokens)
                    .unwrap_or(0),
                cached_input_tokens: api_usage
                    .prompt_tokens_details
                    .as_ref()
                    .and_then(|d| d.cached_tokens)
                    .unwrap_or(0),
            }
        } else {
            LanguageModelUsage::default()
        };

        // Build provider metadata with response headers
        let mut provider_metadata = HashMap::new();
        let mut provider_data = HashMap::new();

        // Add HTTP response headers to provider metadata for debugging
        // Headers are prefixed with "header." to distinguish them from other metadata
        // Common useful headers: x-ratelimit-*, x-request-id, retry-after, etc.
        for (key, value) in response_headers.iter() {
            if let Ok(value_str) = value.to_str() {
                provider_data.insert(format!("header.{}", key.as_str()), json!(value_str));
            }
        }

        provider_metadata.insert(self.provider_options_name().to_string(), provider_data);

        // Build response metadata
        let response_metadata = get_response_metadata(
            api_response.id.clone(),
            api_response.model.clone(),
            api_response.created,
        );

        // Map finish reason
        let finish_reason = map_openai_compatible_finish_reason(choice.finish_reason.as_deref());

        Ok(LanguageModelGenerateResponse {
            content,
            finish_reason,
            usage,
            provider_metadata: Some(provider_metadata),
            request: Some(
                ai_sdk_provider::language_model::LanguageModelRequestMetadata { body: Some(body) },
            ),
            response: Some(response_metadata),
            warnings,
        })
    }

    async fn do_stream(
        &self,
        options: LanguageModelCallOptions,
    ) -> Result<LanguageModelStreamResponse, Box<dyn std::error::Error>> {
        // Check if already cancelled before starting
        if let Some(signal) = &options.abort_signal
            && signal.is_cancelled()
        {
            return Err("Operation cancelled".into());
        }

        // Prepare request body with streaming enabled
        let (mut body, warnings) = self.prepare_request_body(&options)?;
        body["stream"] = json!(true);

        // Add stream_options if include_usage is enabled
        if self.config.include_usage {
            body["stream_options"] = json!({
                "include_usage": true
            });
        }

        let body_string = serde_json::to_string(&body)?;

        // Build URL
        let url = (self.config.url)(&self.model_id, "/chat/completions");

        // Build headers
        let mut headers = (self.config.headers)();
        if let Some(option_headers) = &options.headers {
            headers.extend(option_headers.clone());
        }

        // Make streaming API request
        let client = reqwest::Client::new();
        let mut request = client.post(&url).header("Content-Type", "application/json");

        for (key, value) in headers {
            request = request.header(key, value);
        }

        // Send request with optional cancellation support
        let response = if let Some(signal) = &options.abort_signal {
            tokio::select! {
                result = request.body(body_string.clone()).send() => result?,
                _ = signal.cancelled() => {
                    return Err("Operation cancelled".into());
                }
            }
        } else {
            request.body(body_string.clone()).send().await?
        };
        let status = response.status();
        let response_headers = response.headers().clone();

        if !status.is_success() {
            let error_body = response.text().await?;

            // Check for Retry-After header to include in error message
            let retry_info = if let Some(retry_after) = response_headers.get("retry-after") {
                if let Ok(retry_str) = retry_after.to_str() {
                    // Try to parse as seconds (integer)
                    if let Ok(seconds) = retry_str.parse::<u64>() {
                        format!(" (Retry after {} seconds)", seconds)
                    } else {
                        // Could be HTTP-date format, but for now just include the raw value
                        format!(" (Retry-After: {})", retry_str)
                    }
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            return Err(format!(
                "API request failed with status {}: {}{}",
                status, error_body, retry_info
            )
            .into());
        }

        // Build headers map from HTTP response headers
        let mut headers_map = HashMap::new();
        for (key, value) in response_headers.iter() {
            if let Ok(value_str) = value.to_str() {
                headers_map.insert(key.as_str().to_string(), value_str.to_string());
            }
        }

        // Create the stream processor
        let byte_stream = response.bytes_stream();

        // Process SSE events and convert to StreamPart
        let stream = Self::process_stream(byte_stream, warnings);

        Ok(LanguageModelStreamResponse {
            stream: Box::new(stream),
            request: Some(
                ai_sdk_provider::language_model::LanguageModelRequestMetadata { body: Some(body) },
            ),
            response: Some(ai_sdk_provider::language_model::StreamResponseMetadata {
                headers: Some(headers_map),
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_model() {
        let config = OpenAICompatibleChatConfig::default();
        let model = OpenAICompatibleChatLanguageModel::new("gpt-4".to_string(), config);

        assert_eq!(model.model_id(), "gpt-4");
        assert_eq!(model.provider(), "openai-compatible");
        assert_eq!(model.specification_version(), "v3");
    }

    #[test]
    fn test_provider_options_name() {
        let config = OpenAICompatibleChatConfig {
            provider: "openai.azure".to_string(),
            ..Default::default()
        };
        let model = OpenAICompatibleChatLanguageModel::new("gpt-4".to_string(), config);

        assert_eq!(model.provider_options_name(), "openai");
    }

    #[test]
    fn test_provider_options_name_no_dot() {
        let config = OpenAICompatibleChatConfig {
            provider: "custom".to_string(),
            ..Default::default()
        };
        let model = OpenAICompatibleChatLanguageModel::new("gpt-4".to_string(), config);

        assert_eq!(model.provider_options_name(), "custom");
    }

    #[tokio::test]
    async fn test_supported_urls_none() {
        let config = OpenAICompatibleChatConfig::default();
        let model = OpenAICompatibleChatLanguageModel::new("gpt-4".to_string(), config);

        let urls = model.supported_urls().await;
        assert_eq!(urls.len(), 0);
    }

    #[tokio::test]
    async fn test_supported_urls_custom() {
        let config = OpenAICompatibleChatConfig {
            supported_urls: Some(|| {
                let mut map = HashMap::new();
                map.insert("test".to_string(), vec![]);
                map
            }),
            ..Default::default()
        };
        let model = OpenAICompatibleChatLanguageModel::new("gpt-4".to_string(), config);

        let urls = model.supported_urls().await;
        assert_eq!(urls.len(), 1);
        assert!(urls.contains_key("test"));
    }

    #[test]
    fn test_config_default() {
        let config = OpenAICompatibleChatConfig::default();

        assert_eq!(config.provider, "openai-compatible");
        assert!(!config.include_usage);
        assert!(!config.supports_structured_outputs);
        assert!(config.supported_urls.is_none());
    }

    #[test]
    fn test_config_custom_provider() {
        let config = OpenAICompatibleChatConfig {
            provider: "azure".to_string(),
            ..Default::default()
        };

        assert_eq!(config.provider, "azure");
    }

    #[test]
    fn test_config_structured_outputs() {
        let config = OpenAICompatibleChatConfig {
            supports_structured_outputs: true,
            ..Default::default()
        };

        assert!(config.supports_structured_outputs);
    }
}
