use ai_sdk_provider::language_model::{
    LanguageModel, LanguageModelGenerateResponse, LanguageModelRequestMetadata,
    LanguageModelStreamResponse, call_options::LanguageModelCallOptions,
    call_warning::LanguageModelCallWarning, content::LanguageModelContent,
    response_metadata::LanguageModelResponseMetadata, stream_part::LanguageModelStreamPart,
    usage::LanguageModelUsage,
};
use async_trait::async_trait;
use futures_util::{Stream, StreamExt};
use regex::Regex;
use reqwest;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;

use crate::client::HuggingFaceClientConfig;
use crate::error::HuggingFaceErrorData;
use crate::responses::{
    HuggingFaceResponsesModelId, convert_to_huggingface_responses_messages,
    map_huggingface_responses_finish_reason, prepare_responses_tools,
};

/// Hugging Face Responses API language model implementation.
pub struct HuggingFaceResponsesLanguageModel {
    /// The model identifier.
    model_id: HuggingFaceResponsesModelId,

    /// Configuration for the model.
    config: HuggingFaceClientConfig,
}

impl HuggingFaceResponsesLanguageModel {
    /// Creates a new Hugging Face Responses language model.
    pub fn new(model_id: HuggingFaceResponsesModelId, config: HuggingFaceClientConfig) -> Self {
        Self { model_id, config }
    }

    /// Prepares the request body for API calls.
    async fn prepare_request_body(
        &self,
        options: &LanguageModelCallOptions,
    ) -> Result<(Value, Vec<LanguageModelCallWarning>), Box<dyn std::error::Error>> {
        let mut warnings = Vec::new();

        // Check for unsupported settings
        if options.top_k.is_some() {
            warnings.push(LanguageModelCallWarning::UnsupportedSetting {
                setting: "topK".to_string(),
                details: Some("topK is not supported by Hugging Face Responses API".to_string()),
            });
        }

        if options.seed.is_some() {
            warnings.push(LanguageModelCallWarning::UnsupportedSetting {
                setting: "seed".to_string(),
                details: Some("seed is not supported by Hugging Face Responses API".to_string()),
            });
        }

        if options.presence_penalty.is_some() {
            warnings.push(LanguageModelCallWarning::UnsupportedSetting {
                setting: "presencePenalty".to_string(),
                details: Some(
                    "presencePenalty is not supported by Hugging Face Responses API".to_string(),
                ),
            });
        }

        if options.frequency_penalty.is_some() {
            warnings.push(LanguageModelCallWarning::UnsupportedSetting {
                setting: "frequencyPenalty".to_string(),
                details: Some(
                    "frequencyPenalty is not supported by Hugging Face Responses API".to_string(),
                ),
            });
        }

        if options.stop_sequences.is_some() {
            warnings.push(LanguageModelCallWarning::UnsupportedSetting {
                setting: "stopSequences".to_string(),
                details: Some(
                    "stopSequences is not supported by Hugging Face Responses API".to_string(),
                ),
            });
        }

        // Convert prompt to HF format
        let (input, message_warnings) =
            convert_to_huggingface_responses_messages(options.prompt.clone()).await?;
        warnings.extend(message_warnings);

        // Prepare tools
        let tools_result =
            prepare_responses_tools(options.tools.clone(), options.tool_choice.clone());
        warnings.extend(tools_result.tool_warnings);

        // Build base request body
        let mut body = json!({
            "model": self.model_id,
            "input": input,
        });

        // Add optional parameters
        if let Some(temperature) = options.temperature {
            body["temperature"] = json!(temperature);
        }

        if let Some(top_p) = options.top_p {
            body["top_p"] = json!(top_p);
        }

        if let Some(max_tokens) = options.max_output_tokens {
            body["max_output_tokens"] = json!(max_tokens);
        }

        // Add tools if present
        if let Some(tools) = tools_result.tools {
            body["tools"] = serde_json::to_value(tools)?;
        }

        if let Some(tool_choice) = tools_result.tool_choice {
            body["tool_choice"] = serde_json::to_value(tool_choice)?;
        }

        // Handle response format (structured output)
        if let Some(response_format) = &options.response_format {
            use ai_sdk_provider::language_model::call_options::LanguageModelResponseFormat;

            match response_format {
                LanguageModelResponseFormat::Text => {
                    // Default, no need to specify
                }
                LanguageModelResponseFormat::Json {
                    schema,
                    name,
                    description,
                } => {
                    if let Some(schema_value) = schema {
                        // Structured output with JSON schema
                        let mut text_format = json!({
                            "type": "json_schema",
                            "strict": false,
                            "schema": schema_value,
                        });

                        if let Some(name_str) = name {
                            text_format["name"] = json!(name_str);
                        }

                        if let Some(desc_str) = description {
                            text_format["description"] = json!(desc_str);
                        }

                        body["text"] = json!({
                            "format": text_format
                        });
                    }
                }
            }
        }

        // TODO: Handle provider-specific options (metadata, instructions, strictJsonSchema)
        // This would require parsing options.provider_options

        Ok((body, warnings))
    }

    /// Process SSE byte stream and convert to StreamPart events.
    fn process_stream(
        byte_stream: impl Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Send + 'static,
        warnings: Vec<LanguageModelCallWarning>,
    ) -> impl Stream<Item = LanguageModelStreamPart> + Unpin + Send {
        let mut buffer = String::new();
        let mut state = StreamState::default();

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
                                    if let Ok(chunk) = serde_json::from_str::<HuggingFaceStreamChunk>(data) {
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

    /// Process a single streaming chunk and emit StreamPart events.
    fn process_chunk(
        state: &mut StreamState,
        chunk: HuggingFaceStreamChunk,
    ) -> Vec<LanguageModelStreamPart> {
        let mut parts = Vec::new();

        match chunk.event_type.as_str() {
            "response.created" => {
                if let Some(response) = chunk.response {
                    state.response_id = Some(response.id.clone());
                    parts.push(LanguageModelStreamPart::ResponseMetadata(
                        LanguageModelResponseMetadata {
                            id: Some(response.id),
                            timestamp: Some(response.created_at),
                            model_id: Some(response.model),
                        },
                    ));
                }
            }

            "response.output_item.added" => {
                if let Some(item) = chunk.item {
                    if item.item_type == "message" && item.role == Some("assistant".to_string()) {
                        let id = item
                            .id
                            .unwrap_or_else(|| format!("text-{}", uuid::Uuid::new_v4()));
                        state.current_text_id = Some(id.clone());
                        parts.push(LanguageModelStreamPart::text_start(&id));
                    } else if item.item_type == "function_call" {
                        let call_id = item
                            .call_id
                            .clone()
                            .unwrap_or_else(|| format!("call-{}", uuid::Uuid::new_v4()));
                        let name = item.name.clone().unwrap_or_default();
                        state.tool_calls.insert(
                            call_id.clone(),
                            ToolCallState {
                                id: call_id.clone(),
                                name: name.clone(),
                                arguments: String::new(),
                                started: false,
                            },
                        );
                        parts.push(LanguageModelStreamPart::tool_input_start(&call_id, &name));
                        if let Some(tool_state) = state.tool_calls.get_mut(&call_id) {
                            tool_state.started = true;
                        }
                    }
                }
            }

            "response.output_text.delta" => {
                if let (Some(item_id), Some(delta)) = (chunk.item_id.as_ref(), chunk.delta.as_ref())
                    && let Some(text_id) = &state.current_text_id
                    && item_id == text_id
                {
                    parts.push(LanguageModelStreamPart::text_delta(text_id, delta));
                }
            }

            "response.output_item.done" => {
                if let Some(item) = chunk.item {
                    if item.item_type == "message" {
                        if let Some(text_id) = state.current_text_id.take() {
                            parts.push(LanguageModelStreamPart::text_end(&text_id));
                        }
                    } else if item.item_type == "function_call"
                        && let Some(call_id) = item.call_id.as_ref()
                        && let Some(tool_state) = state.tool_calls.get(call_id)
                        && tool_state.started
                    {
                        parts.push(LanguageModelStreamPart::tool_input_end(&tool_state.id));

                        // Emit the ToolCall with complete arguments
                        parts.push(LanguageModelStreamPart::ToolCall(
                                        ai_sdk_provider::language_model::content::tool_call::LanguageModelToolCall::new(
                                            tool_state.id.clone(),
                                            tool_state.name.clone(),
                                            tool_state.arguments.clone(),
                                        )
                                    ));
                    }
                }
            }

            "response.completed" => {
                if let Some(response) = chunk.response {
                    state.response_id = Some(response.id);

                    // Build usage information
                    let usage = if let Some(api_usage) = response.usage {
                        LanguageModelUsage {
                            input_tokens: api_usage.input_tokens,
                            output_tokens: api_usage.output_tokens,
                            total_tokens: api_usage
                                .total_tokens
                                .unwrap_or(api_usage.input_tokens + api_usage.output_tokens),
                            reasoning_tokens: api_usage
                                .output_tokens_details
                                .map(|d| d.reasoning_tokens)
                                .unwrap_or(0),
                            cached_input_tokens: api_usage
                                .input_tokens_details
                                .map(|d| d.cached_tokens)
                                .unwrap_or(0),
                        }
                    } else {
                        LanguageModelUsage::default()
                    };

                    // Map finish reason
                    let finish_reason = map_huggingface_responses_finish_reason(
                        response
                            .incomplete_details
                            .as_ref()
                            .and_then(|d| d.reason.as_deref()),
                    );

                    // Emit finish event
                    parts.push(LanguageModelStreamPart::finish(usage, finish_reason));
                }
            }

            _ => {
                // Unknown event type, ignore
            }
        }

        parts
    }
}

/// Helper struct to track streaming state across chunks.
#[derive(Default)]
struct StreamState {
    response_id: Option<String>,
    current_text_id: Option<String>,
    tool_calls: HashMap<String, ToolCallState>,
}

struct ToolCallState {
    id: String,
    name: String,
    arguments: String,
    started: bool,
}

/// Streaming chunk from Hugging Face Responses API.
#[derive(Debug, Deserialize)]
struct HuggingFaceStreamChunk {
    #[serde(rename = "type")]
    event_type: String,

    #[serde(default)]
    response: Option<HuggingFaceResponse>,

    #[serde(default)]
    item: Option<HuggingFaceOutputItem>,

    #[serde(default)]
    item_id: Option<String>,

    #[serde(default)]
    delta: Option<String>,
}

/// Response structure from Hugging Face API.
#[derive(Debug, Deserialize)]
struct HuggingFaceResponse {
    id: String,
    model: String,
    created_at: i64,

    #[serde(default)]
    usage: Option<HuggingFaceUsage>,

    #[serde(default)]
    incomplete_details: Option<HuggingFaceIncompleteDetails>,

    #[serde(default)]
    output: Vec<HuggingFaceOutputItem>,
}

#[derive(Debug, Deserialize)]
struct HuggingFaceUsage {
    input_tokens: u64,
    output_tokens: u64,
    total_tokens: Option<u64>,

    #[serde(default)]
    input_tokens_details: Option<HuggingFaceInputTokensDetails>,

    #[serde(default)]
    output_tokens_details: Option<HuggingFaceOutputTokensDetails>,
}

#[derive(Debug, Deserialize)]
struct HuggingFaceInputTokensDetails {
    cached_tokens: u64,
}

#[derive(Debug, Deserialize)]
struct HuggingFaceOutputTokensDetails {
    reasoning_tokens: u64,
}

#[derive(Debug, Deserialize)]
struct HuggingFaceIncompleteDetails {
    reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct HuggingFaceOutputItem {
    #[serde(rename = "type")]
    item_type: String,

    #[serde(default)]
    id: Option<String>,

    #[serde(default)]
    role: Option<String>,

    #[serde(default)]
    content: Vec<HuggingFaceContentPart>,

    // For function_call
    #[serde(default)]
    call_id: Option<String>,

    #[serde(default)]
    name: Option<String>,

    #[serde(default)]
    arguments: Option<String>,

    #[serde(default)]
    output: Option<String>,
}

#[derive(Debug, Deserialize)]
struct HuggingFaceContentPart {
    #[serde(default)]
    text: Option<String>,

    #[serde(default)]
    annotations: Vec<HuggingFaceAnnotation>,
}

#[derive(Debug, Deserialize)]
struct HuggingFaceAnnotation {
    url: String,

    #[serde(default)]
    title: Option<String>,
}

#[async_trait]
impl LanguageModel for HuggingFaceResponsesLanguageModel {
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
        let mut map = HashMap::new();
        // Support any HTTP/HTTPS URL for images
        map.insert(
            "image/*".to_string(),
            vec![Regex::new(r"^https?://.*$").unwrap()],
        );
        map
    }

    async fn do_generate(
        &self,
        options: LanguageModelCallOptions,
    ) -> Result<LanguageModelGenerateResponse, Box<dyn std::error::Error>> {
        // Prepare request body
        let (body, warnings) = self.prepare_request_body(&options).await?;

        // Build URL
        let url = (self.config.url)(&self.model_id, "/responses");

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

        let response = request.body(serde_json::to_string(&body)?).send().await?;
        let status = response.status();

        if !status.is_success() {
            let error_body = response.text().await?;

            // Try to parse as HuggingFace error
            if let Ok(hf_error) = serde_json::from_str::<HuggingFaceErrorData>(&error_body) {
                return Err(format!("API error: {}", hf_error.message()).into());
            }

            return Err(
                format!("API request failed with status {}: {}", status, error_body).into(),
            );
        }

        let response_body = response.text().await?;
        let api_response: HuggingFaceResponse = serde_json::from_str(&response_body)?;

        // Extract content from response
        let mut content = Vec::new();

        // Process output array
        for item in &api_response.output {
            match item.item_type.as_str() {
                "message" => {
                    for content_part in &item.content {
                        if let Some(text) = &content_part.text {
                            content.push(LanguageModelContent::Text(
                                ai_sdk_provider::language_model::content::text::LanguageModelText::new(
                                    text.clone()
                                )
                            ));
                        }

                        // Process annotations as sources
                        for annotation in &content_part.annotations {
                            content.push(LanguageModelContent::Source(
                                ai_sdk_provider::language_model::content::source::LanguageModelSource::Url {
                                    id: format!("source-{}", uuid::Uuid::new_v4()),
                                    url: annotation.url.clone(),
                                    title: annotation.title.clone(),
                                    provider_metadata: None,
                                }
                            ));
                        }
                    }
                }

                "function_call" => {
                    if let (Some(call_id), Some(name), Some(arguments)) =
                        (&item.call_id, &item.name, &item.arguments)
                    {
                        // Check if this is provider-executed (has output)
                        let provider_executed = item.output.is_some();

                        content.push(LanguageModelContent::ToolCall(
                            ai_sdk_provider::language_model::content::tool_call::LanguageModelToolCall::with_options(
                                call_id.clone(),
                                name.clone(),
                                arguments.clone(),
                                Some(provider_executed),
                                None,
                            )
                        ));

                        // If there's output (provider-executed tool), add tool result
                        if let Some(output) = &item.output {
                            // Parse output as JSON Value
                            let result_value =
                                serde_json::from_str(output).unwrap_or_else(|_| json!(output));

                            content.push(LanguageModelContent::ToolResult(
                                ai_sdk_provider::language_model::content::tool_result::LanguageModelToolResult::with_options(
                                    call_id.clone(),
                                    name.clone(),
                                    result_value,
                                    None,
                                    Some(true),
                                    None,
                                )
                            ));
                        }
                    }
                }

                _ => {
                    // Unknown output type, skip
                }
            }
        }

        // Build usage information
        let usage = if let Some(api_usage) = &api_response.usage {
            LanguageModelUsage {
                input_tokens: api_usage.input_tokens,
                output_tokens: api_usage.output_tokens,
                total_tokens: api_usage
                    .total_tokens
                    .unwrap_or(api_usage.input_tokens + api_usage.output_tokens),
                reasoning_tokens: api_usage
                    .output_tokens_details
                    .as_ref()
                    .map(|d| d.reasoning_tokens)
                    .unwrap_or(0),
                cached_input_tokens: api_usage
                    .input_tokens_details
                    .as_ref()
                    .map(|d| d.cached_tokens)
                    .unwrap_or(0),
            }
        } else {
            LanguageModelUsage::default()
        };

        // Map finish reason
        let finish_reason = map_huggingface_responses_finish_reason(
            api_response
                .incomplete_details
                .as_ref()
                .and_then(|d| d.reason.as_deref()),
        );

        Ok(LanguageModelGenerateResponse {
            content,
            finish_reason,
            usage,
            provider_metadata: None,
            request: Some(LanguageModelRequestMetadata { body: Some(body) }),
            response: Some(LanguageModelResponseMetadata {
                id: Some(api_response.id),
                timestamp: Some(api_response.created_at),
                model_id: Some(api_response.model),
            }),
            warnings,
        })
    }

    async fn do_stream(
        &self,
        options: LanguageModelCallOptions,
    ) -> Result<LanguageModelStreamResponse, Box<dyn std::error::Error>> {
        // Prepare request body with streaming enabled
        let (mut body, warnings) = self.prepare_request_body(&options).await?;
        body["stream"] = json!(true);

        // Build URL
        let url = (self.config.url)(&self.model_id, "/responses");

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

        let response = request.body(serde_json::to_string(&body)?).send().await?;
        let status = response.status();

        if !status.is_success() {
            let error_body = response.text().await?;

            // Try to parse as HuggingFace error
            if let Ok(hf_error) = serde_json::from_str::<HuggingFaceErrorData>(&error_body) {
                return Err(format!("API error: {}", hf_error.message()).into());
            }

            return Err(
                format!("API request failed with status {}: {}", status, error_body).into(),
            );
        }

        // Create the stream processor
        let byte_stream = response.bytes_stream();
        let stream = Self::process_stream(byte_stream, warnings);

        Ok(LanguageModelStreamResponse {
            stream: Box::new(stream),
            request: Some(LanguageModelRequestMetadata { body: Some(body) }),
            response: None,
        })
    }
}
