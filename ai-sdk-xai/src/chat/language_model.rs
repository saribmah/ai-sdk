use ai_sdk_provider::language_model::call_options::LanguageModelCallOptions;
use ai_sdk_provider::language_model::call_warning::LanguageModelCallWarning;
use ai_sdk_provider::language_model::content::LanguageModelContent;
use ai_sdk_provider::language_model::content::reasoning::LanguageModelReasoning;
use ai_sdk_provider::language_model::content::source::LanguageModelSource;
use ai_sdk_provider::language_model::content::text::LanguageModelText;
use ai_sdk_provider::language_model::content::tool_call::LanguageModelToolCall;
use ai_sdk_provider::language_model::finish_reason::LanguageModelFinishReason;
use ai_sdk_provider::language_model::stream_part::LanguageModelStreamPart;
use ai_sdk_provider::language_model::usage::LanguageModelUsage;
use ai_sdk_provider::language_model::{
    LanguageModel, LanguageModelGenerateResponse, LanguageModelRequestMetadata,
    LanguageModelStreamResponse, StreamResponseMetadata,
};
use async_trait::async_trait;
use futures_util::{Stream, StreamExt};
use regex::Regex;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

use super::options::XaiProviderOptions;
use super::prepare_tools::prepare_tools;
use super::prompt::convert_to_xai_chat_messages;
use crate::error::XaiErrorData;
use crate::provider::XaiChatConfig;

/// xAI chat language model implementation.
pub struct XaiChatLanguageModel {
    model_id: String,
    config: Arc<XaiChatConfig>,
    client: Client,
}

impl XaiChatLanguageModel {
    /// Creates a new xAI chat language model.
    pub fn new(model_id: String, config: XaiChatConfig) -> Self {
        Self {
            model_id,
            config: Arc::new(config),
            client: Client::new(),
        }
    }

    fn get_headers(&self) -> HashMap<String, String> {
        (self.config.headers)()
    }

    /// Parse xAI-specific provider options from call options
    fn parse_provider_options(
        &self,
        options: &LanguageModelCallOptions,
    ) -> Result<Option<XaiProviderOptions>, Box<dyn std::error::Error>> {
        // Extract the provider options map
        let provider_options = match &options.provider_options {
            Some(opts) => opts,
            None => return Ok(None),
        };

        // Get the xAI-specific options (inner HashMap<String, Value>)
        let xai_options_map = match provider_options.get("xai") {
            Some(map) => map,
            None => return Ok(None),
        };

        // Deserialize directly from the HashMap<String, Value>
        let xai_options: XaiProviderOptions =
            serde_json::from_value(serde_json::to_value(xai_options_map)?)?;

        Ok(Some(xai_options))
    }

    /// Apply xAI provider options to the request body
    fn apply_provider_options(
        &self,
        body: &mut serde_json::Value,
        xai_options: Option<XaiProviderOptions>,
    ) {
        if let Some(opts) = xai_options {
            // Add reasoning_effort if present
            if let Some(reasoning_effort) = opts.reasoning_effort {
                body["reasoning_effort"] = serde_json::json!(reasoning_effort);
            }

            // Add parallel_function_calling if present
            if let Some(parallel_calling) = opts.parallel_function_calling {
                body["parallel_tool_calls"] = serde_json::json!(parallel_calling);
            }

            // Add search_parameters if present
            if let Some(search_params) = opts.search_parameters {
                body["search"] =
                    serde_json::to_value(search_params).unwrap_or(serde_json::Value::Null);
            }
        }
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
                                    if let Ok(chunk) = serde_json::from_str::<XaiStreamChunk>(data) {
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
                        yield LanguageModelStreamPart::error(serde_json::json!({ "message": e.to_string() }));
                        break;
                    }
                }
            }
        })
    }

    /// Process a single streaming chunk and emit StreamPart events
    fn process_chunk(
        state: &mut StreamState,
        chunk: XaiStreamChunk,
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
        if let Some(reasoning_text) = &delta.reasoning_content
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
                    input_tokens: api_usage.prompt_tokens as u64,
                    output_tokens: api_usage.completion_tokens as u64,
                    total_tokens: api_usage.total_tokens as u64,
                    reasoning_tokens: api_usage
                        .completion_tokens_details
                        .as_ref()
                        .and_then(|d| d.reasoning_tokens)
                        .unwrap_or(0) as u64,
                    cached_input_tokens: 0,
                }
            } else {
                LanguageModelUsage::default()
            };

            // Map finish reason
            let mapped_finish_reason = map_finish_reason(Some(finish_reason));

            // Emit finish event
            parts.push(LanguageModelStreamPart::finish(usage, mapped_finish_reason));
        }

        parts
    }
}

#[async_trait]
impl LanguageModel for XaiChatLanguageModel {
    fn model_id(&self) -> &str {
        &self.model_id
    }

    fn provider(&self) -> &str {
        &self.config.provider
    }

    async fn supported_urls(&self) -> HashMap<String, Vec<Regex>> {
        let mut urls = HashMap::new();
        // xAI supports image URLs
        urls.insert(
            "image/*".to_string(),
            vec![Regex::new(r"^https?://.*$").unwrap()],
        );
        urls
    }

    async fn do_generate(
        &self,
        options: LanguageModelCallOptions,
    ) -> Result<LanguageModelGenerateResponse, Box<dyn std::error::Error>> {
        // Convert prompt to xAI format
        let (messages, _warnings) = convert_to_xai_chat_messages(&options.prompt)
            .map_err(|e| format!("Failed to convert prompt: {}", e))?;

        // Build request body
        let mut body = serde_json::json!({
            "model": self.model_id,
            "messages": messages,
        });

        // Add optional settings
        if let Some(temp) = options.temperature {
            body["temperature"] = serde_json::json!(temp);
        }
        if let Some(max_tokens) = options.max_output_tokens {
            body["max_tokens"] = serde_json::json!(max_tokens);
        }
        if let Some(top_p) = options.top_p {
            body["top_p"] = serde_json::json!(top_p);
        }
        if let Some(seed) = options.seed {
            body["seed"] = serde_json::json!(seed);
        }

        // Parse and apply provider-specific options
        let xai_options = self.parse_provider_options(&options)?;
        self.apply_provider_options(&mut body, xai_options);

        // Handle tools and tool_choice
        let tool_result = prepare_tools(options.tools, options.tool_choice);
        let warnings = tool_result.tool_warnings;

        if let Some(tools) = tool_result.tools {
            body["tools"] = serde_json::to_value(tools)?;
        }

        if let Some(tool_choice) = tool_result.tool_choice {
            body["tool_choice"] = serde_json::to_value(tool_choice)?;
        }

        // Handle response_format
        if let Some(response_format) = &options.response_format {
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
                        let mut json_schema = serde_json::json!({
                            "type": "json_schema",
                            "json_schema": {
                                "schema": schema_value,
                                "strict": true
                            }
                        });

                        // Add optional name and description
                        if let Some(name_str) = name {
                            json_schema["json_schema"]["name"] = serde_json::json!(name_str);
                        }
                        if let Some(desc_str) = description {
                            json_schema["json_schema"]["description"] = serde_json::json!(desc_str);
                        }

                        Some(json_schema)
                    } else {
                        // Simple JSON mode without schema
                        Some(serde_json::json!({
                            "type": "json_object"
                        }))
                    }
                }
            };

            if let Some(format) = format_json {
                body["response_format"] = format;
            }
        }

        // Make API request
        let url = format!("{}/chat/completions", self.config.base_url);
        let response = self
            .client
            .post(&url)
            .headers(
                self.get_headers()
                    .into_iter()
                    .filter_map(|(k, v)| {
                        Some((
                            reqwest::header::HeaderName::from_bytes(k.as_bytes()).ok()?,
                            reqwest::header::HeaderValue::from_str(&v).ok()?,
                        ))
                    })
                    .collect(),
            )
            .json(&body)
            .send()
            .await?;

        let status = response.status();

        // Handle error responses
        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_default();
            if let Ok(error_data) = serde_json::from_str::<XaiErrorData>(&error_body) {
                let provider_error = error_data.to_provider_error(status.as_u16(), &url);
                return Err(Box::new(provider_error));
            }
            return Err(Box::new(std::io::Error::other(format!(
                "xAI API error ({}): {}",
                status, error_body
            ))));
        }

        // Parse response
        let api_response: XaiChatResponse = response.json().await?;

        // Extract content from response
        let choice = &api_response.choices[0];
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
        if let Some(reasoning) = &choice.message.reasoning_content
            && !reasoning.is_empty()
        {
            content.push(LanguageModelContent::Reasoning(
                LanguageModelReasoning::init(reasoning.clone()),
            ));
        }

        // Add tool calls
        if let Some(tool_calls) = &choice.message.tool_calls {
            for tool_call in tool_calls {
                content.push(LanguageModelContent::ToolCall(LanguageModelToolCall::new(
                    tool_call.id.clone(),
                    tool_call.function.name.clone(),
                    tool_call.function.arguments.clone(),
                )));
            }
        }

        // Add citations as sources
        if let Some(citations) = &api_response.citations {
            for url in citations {
                content.push(LanguageModelContent::Source(LanguageModelSource::Url {
                    id: uuid::Uuid::new_v4().to_string(),
                    url: url.clone(),
                    title: None,
                    provider_metadata: None,
                }));
            }
        }

        Ok(LanguageModelGenerateResponse {
            content,
            finish_reason: map_finish_reason(choice.finish_reason.as_deref()),
            usage: LanguageModelUsage {
                input_tokens: api_response.usage.prompt_tokens as u64,
                output_tokens: api_response.usage.completion_tokens as u64,
                total_tokens: api_response.usage.total_tokens as u64,
                reasoning_tokens: api_response
                    .usage
                    .completion_tokens_details
                    .as_ref()
                    .and_then(|d| d.reasoning_tokens)
                    .unwrap_or(0) as u64,
                cached_input_tokens: 0,
            },
            provider_metadata: None,
            request: Some(LanguageModelRequestMetadata { body: Some(body) }),
            response: None,
            warnings,
        })
    }

    async fn do_stream(
        &self,
        options: LanguageModelCallOptions,
    ) -> Result<LanguageModelStreamResponse, Box<dyn std::error::Error>> {
        // Convert prompt to xAI format
        let (messages, _warnings) = convert_to_xai_chat_messages(&options.prompt)
            .map_err(|e| format!("Failed to convert prompt: {}", e))?;

        // Build request body with streaming enabled
        let mut body = serde_json::json!({
            "model": self.model_id,
            "messages": messages,
            "stream": true,
        });

        // Add stream_options to get usage information
        body["stream_options"] = serde_json::json!({
            "include_usage": true
        });

        // Add optional settings
        if let Some(temp) = options.temperature {
            body["temperature"] = serde_json::json!(temp);
        }
        if let Some(max_tokens) = options.max_output_tokens {
            body["max_tokens"] = serde_json::json!(max_tokens);
        }
        if let Some(top_p) = options.top_p {
            body["top_p"] = serde_json::json!(top_p);
        }
        if let Some(seed) = options.seed {
            body["seed"] = serde_json::json!(seed);
        }

        // Parse and apply provider-specific options
        let xai_options = self.parse_provider_options(&options)?;
        self.apply_provider_options(&mut body, xai_options);

        // Handle tools and tool_choice
        let tool_result = prepare_tools(options.tools, options.tool_choice);
        let warnings = tool_result.tool_warnings;

        if let Some(tools) = tool_result.tools {
            body["tools"] = serde_json::to_value(tools)?;
        }

        if let Some(tool_choice) = tool_result.tool_choice {
            body["tool_choice"] = serde_json::to_value(tool_choice)?;
        }

        // Handle response_format
        if let Some(response_format) = &options.response_format {
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
                        let mut json_schema = serde_json::json!({
                            "type": "json_schema",
                            "json_schema": {
                                "schema": schema_value,
                                "strict": true
                            }
                        });

                        // Add optional name and description
                        if let Some(name_str) = name {
                            json_schema["json_schema"]["name"] = serde_json::json!(name_str);
                        }
                        if let Some(desc_str) = description {
                            json_schema["json_schema"]["description"] = serde_json::json!(desc_str);
                        }

                        Some(json_schema)
                    } else {
                        // Simple JSON mode without schema
                        Some(serde_json::json!({
                            "type": "json_object"
                        }))
                    }
                }
            };

            if let Some(format) = format_json {
                body["response_format"] = format;
            }
        }

        // Make streaming API request
        let url = format!("{}/chat/completions", self.config.base_url);
        let response = self
            .client
            .post(&url)
            .headers(
                self.get_headers()
                    .into_iter()
                    .filter_map(|(k, v)| {
                        Some((
                            reqwest::header::HeaderName::from_bytes(k.as_bytes()).ok()?,
                            reqwest::header::HeaderValue::from_str(&v).ok()?,
                        ))
                    })
                    .collect(),
            )
            .json(&body)
            .send()
            .await?;

        let status = response.status();
        let response_headers = response.headers().clone();

        // Handle error responses
        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_default();
            if let Ok(error_data) = serde_json::from_str::<XaiErrorData>(&error_body) {
                let provider_error = error_data.to_provider_error(status.as_u16(), &url);
                return Err(Box::new(provider_error));
            }
            return Err(Box::new(std::io::Error::other(format!(
                "xAI API error ({}): {}",
                status, error_body
            ))));
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
            request: Some(LanguageModelRequestMetadata { body: Some(body) }),
            response: Some(StreamResponseMetadata {
                headers: Some(headers_map),
            }),
        })
    }
}

/// xAI chat completion response.
#[derive(Debug, Deserialize)]
struct XaiChatResponse {
    #[allow(dead_code)]
    id: Option<String>,
    choices: Vec<XaiChoice>,
    usage: XaiUsage,
    citations: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct XaiChoice {
    message: XaiResponseMessage,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct XaiResponseMessage {
    #[allow(dead_code)]
    role: String,
    content: Option<String>,
    reasoning_content: Option<String>,
    tool_calls: Option<Vec<XaiToolCallResponse>>,
}

#[derive(Debug, Deserialize)]
struct XaiToolCallResponse {
    id: String,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    call_type: String,
    function: XaiFunctionResponse,
}

#[derive(Debug, Deserialize)]
struct XaiFunctionResponse {
    name: String,
    arguments: String,
}

#[derive(Debug, Deserialize)]
struct XaiUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
    #[allow(dead_code)]
    completion_tokens_details: Option<XaiCompletionTokensDetails>,
}

#[derive(Debug, Deserialize)]
struct XaiCompletionTokensDetails {
    #[allow(dead_code)]
    reasoning_tokens: Option<u32>,
}

/// xAI streaming response chunk
#[derive(Debug, Deserialize)]
struct XaiStreamChunk {
    #[allow(dead_code)]
    id: Option<String>,
    #[allow(dead_code)]
    created: Option<i64>,
    #[allow(dead_code)]
    model: Option<String>,
    choices: Vec<XaiStreamChoice>,
    usage: Option<XaiUsage>,
}

#[derive(Debug, Deserialize)]
struct XaiStreamChoice {
    #[allow(dead_code)]
    index: Option<u32>,
    delta: XaiStreamDelta,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct XaiStreamDelta {
    #[allow(dead_code)]
    role: Option<String>,
    content: Option<String>,
    reasoning_content: Option<String>,
    tool_calls: Option<Vec<XaiStreamToolCall>>,
}

#[derive(Debug, Deserialize)]
struct XaiStreamToolCall {
    index: Option<u32>,
    id: Option<String>,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    call_type: Option<String>,
    function: Option<XaiStreamFunction>,
}

#[derive(Debug, Deserialize)]
struct XaiStreamFunction {
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

fn map_finish_reason(reason: Option<&str>) -> LanguageModelFinishReason {
    match reason {
        Some("stop") => LanguageModelFinishReason::Stop,
        Some("length") => LanguageModelFinishReason::Length,
        Some("tool_calls") => LanguageModelFinishReason::ToolCalls,
        Some("content_filter") => LanguageModelFinishReason::ContentFilter,
        _ => LanguageModelFinishReason::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finish_reason_mapping() {
        assert_eq!(
            map_finish_reason(Some("stop")),
            LanguageModelFinishReason::Stop
        );
        assert_eq!(
            map_finish_reason(Some("length")),
            LanguageModelFinishReason::Length
        );
        assert_eq!(
            map_finish_reason(Some("tool_calls")),
            LanguageModelFinishReason::ToolCalls
        );
        assert_eq!(map_finish_reason(None), LanguageModelFinishReason::Unknown);
    }

    #[test]
    fn test_stream_chunk_deserialization() {
        let json = r#"{
            "id": "chatcmpl-123",
            "created": 1677652288,
            "model": "grok-2-1212",
            "choices": [{
                "index": 0,
                "delta": {
                    "role": "assistant",
                    "content": "Hello"
                },
                "finish_reason": null
            }]
        }"#;

        let chunk: Result<XaiStreamChunk, _> = serde_json::from_str(json);
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();
        assert_eq!(chunk.choices[0].delta.content, Some("Hello".to_string()));
    }

    #[test]
    fn test_stream_chunk_with_reasoning() {
        let json = r#"{
            "id": "chatcmpl-123",
            "choices": [{
                "index": 0,
                "delta": {
                    "reasoning_content": "Let me think..."
                },
                "finish_reason": null
            }]
        }"#;

        let chunk: Result<XaiStreamChunk, _> = serde_json::from_str(json);
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();
        assert_eq!(
            chunk.choices[0].delta.reasoning_content,
            Some("Let me think...".to_string())
        );
    }

    #[test]
    fn test_stream_chunk_with_tool_calls() {
        let json = r#"{
            "id": "chatcmpl-123",
            "choices": [{
                "index": 0,
                "delta": {
                    "tool_calls": [{
                        "index": 0,
                        "id": "call_123",
                        "type": "function",
                        "function": {
                            "name": "get_weather",
                            "arguments": "{\"location\":"
                        }
                    }]
                },
                "finish_reason": null
            }]
        }"#;

        let chunk: Result<XaiStreamChunk, _> = serde_json::from_str(json);
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();
        let tool_calls = chunk.choices[0].delta.tool_calls.as_ref().unwrap();
        assert_eq!(tool_calls[0].id, Some("call_123".to_string()));
    }

    #[test]
    fn test_stream_chunk_finish() {
        let json = r#"{
            "id": "chatcmpl-123",
            "choices": [{
                "index": 0,
                "delta": {},
                "finish_reason": "stop"
            }],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 20,
                "total_tokens": 30,
                "completion_tokens_details": {
                    "reasoning_tokens": 5
                }
            }
        }"#;

        let chunk: Result<XaiStreamChunk, _> = serde_json::from_str(json);
        assert!(chunk.is_ok());
        let chunk = chunk.unwrap();
        assert_eq!(chunk.choices[0].finish_reason, Some("stop".to_string()));
        assert!(chunk.usage.is_some());
        let usage = chunk.usage.unwrap();
        assert_eq!(usage.prompt_tokens, 10);
        assert_eq!(usage.completion_tokens, 20);
        assert_eq!(usage.total_tokens, 30);
    }

    #[test]
    fn test_stream_state_initialization() {
        let state = StreamState {
            text_id: None,
            reasoning_id: None,
            tool_calls: HashMap::new(),
        };

        assert!(state.text_id.is_none());
        assert!(state.reasoning_id.is_none());
        assert_eq!(state.tool_calls.len(), 0);
    }

    #[test]
    fn test_tool_call_state() {
        let state = ToolCallState {
            id: "call_123".to_string(),
            name: "get_weather".to_string(),
            arguments: "{\"location\": \"NYC\"}".to_string(),
            started: true,
        };

        assert_eq!(state.id, "call_123");
        assert_eq!(state.name, "get_weather");
        assert!(state.started);
    }

    #[test]
    fn test_parse_provider_options_none() {
        use ai_sdk_provider::language_model::call_options::LanguageModelCallOptions;
        use ai_sdk_provider::language_model::prompt::LanguageModelPrompt;

        let config = XaiChatConfig {
            provider: "xai".to_string(),
            base_url: "https://api.x.ai/v1".to_string(),
            headers: Box::new(HashMap::new),
        };
        let model = XaiChatLanguageModel::new("grok-2-1212".to_string(), config);

        let options = LanguageModelCallOptions::new(LanguageModelPrompt::default());
        let result = model.parse_provider_options(&options);

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_parse_provider_options_reasoning_effort() {
        use ai_sdk_provider::language_model::call_options::LanguageModelCallOptions;
        use ai_sdk_provider::language_model::prompt::LanguageModelPrompt;
        use std::collections::HashMap;

        let config = XaiChatConfig {
            provider: "xai".to_string(),
            base_url: "https://api.x.ai/v1".to_string(),
            headers: Box::new(HashMap::new),
        };
        let model = XaiChatLanguageModel::new("grok-2-1212".to_string(), config);

        let mut xai_options = HashMap::new();
        xai_options.insert(
            "reasoningEffort".to_string(), // camelCase for serde
            serde_json::json!("high"),
        );

        let mut provider_options = HashMap::new();
        provider_options.insert("xai".to_string(), xai_options);

        let mut options = LanguageModelCallOptions::new(LanguageModelPrompt::default());
        options.provider_options = Some(provider_options);

        let result = model.parse_provider_options(&options);

        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert!(parsed.is_some());
        assert_eq!(parsed.unwrap().reasoning_effort, Some("high".to_string()));
    }

    #[test]
    fn test_parse_provider_options_search_parameters() {
        use ai_sdk_provider::language_model::call_options::LanguageModelCallOptions;
        use ai_sdk_provider::language_model::prompt::LanguageModelPrompt;
        use std::collections::HashMap;

        let config = XaiChatConfig {
            provider: "xai".to_string(),
            base_url: "https://api.x.ai/v1".to_string(),
            headers: Box::new(HashMap::new),
        };
        let model = XaiChatLanguageModel::new("grok-2-1212".to_string(), config);

        let mut xai_options = HashMap::new();
        xai_options.insert(
            "searchParameters".to_string(), // camelCase for serde
            serde_json::json!({
                "mode": "on",
                "return_citations": true
            }),
        );

        let mut provider_options = HashMap::new();
        provider_options.insert("xai".to_string(), xai_options);

        let mut options = LanguageModelCallOptions::new(LanguageModelPrompt::default());
        options.provider_options = Some(provider_options);

        let result = model.parse_provider_options(&options);

        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert!(parsed.is_some());
        let xai_opts = parsed.unwrap();
        assert!(xai_opts.search_parameters.is_some());
        assert_eq!(xai_opts.search_parameters.unwrap().mode, "on");
    }

    #[test]
    fn test_apply_provider_options_empty() {
        let mut body = serde_json::json!({
            "model": "grok-2-1212",
            "messages": []
        });

        let config = XaiChatConfig {
            provider: "xai".to_string(),
            base_url: "https://api.x.ai/v1".to_string(),
            headers: Box::new(HashMap::new),
        };
        let model = XaiChatLanguageModel::new("grok-2-1212".to_string(), config);

        model.apply_provider_options(&mut body, None);

        // Body should be unchanged
        assert!(body.get("reasoning_effort").is_none());
        assert!(body.get("search").is_none());
        assert!(body.get("parallel_tool_calls").is_none());
    }

    #[test]
    fn test_apply_provider_options_reasoning_effort() {
        use crate::chat::options::XaiProviderOptions;

        let mut body = serde_json::json!({
            "model": "grok-2-1212",
            "messages": []
        });

        let config = XaiChatConfig {
            provider: "xai".to_string(),
            base_url: "https://api.x.ai/v1".to_string(),
            headers: Box::new(HashMap::new),
        };
        let model = XaiChatLanguageModel::new("grok-2-1212".to_string(), config);

        let xai_opts = XaiProviderOptions {
            reasoning_effort: Some("high".to_string()),
            parallel_function_calling: None,
            search_parameters: None,
        };

        model.apply_provider_options(&mut body, Some(xai_opts));

        assert_eq!(body["reasoning_effort"], "high");
    }

    #[test]
    fn test_apply_provider_options_all_fields() {
        use crate::chat::options::{SearchParameters, XaiProviderOptions};

        let mut body = serde_json::json!({
            "model": "grok-2-1212",
            "messages": []
        });

        let config = XaiChatConfig {
            provider: "xai".to_string(),
            base_url: "https://api.x.ai/v1".to_string(),
            headers: Box::new(HashMap::new),
        };
        let model = XaiChatLanguageModel::new("grok-2-1212".to_string(), config);

        let xai_opts = XaiProviderOptions {
            reasoning_effort: Some("high".to_string()),
            parallel_function_calling: Some(false),
            search_parameters: Some(SearchParameters {
                mode: "on".to_string(),
                return_citations: Some(true),
                from_date: None,
                to_date: None,
                max_search_results: None,
                sources: None,
            }),
        };

        model.apply_provider_options(&mut body, Some(xai_opts));

        assert_eq!(body["reasoning_effort"], "high");
        assert_eq!(body["parallel_tool_calls"], false);
        assert!(body.get("search").is_some());
        assert_eq!(body["search"]["mode"], "on");
    }
}
