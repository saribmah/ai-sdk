//! OpenAI Chat Language Model Implementation
//!
//! This module implements the `LanguageModel` trait for OpenAI's chat completion API.

use super::map_openai_finish_reason::map_openai_finish_reason;
use super::openai_chat_api::{OpenAIChatChunk, OpenAIChatResponse};
use super::openai_chat_options::{
    OpenAIChatLanguageModelOptions, OpenAIChatModelId, is_reasoning_model,
    supports_flex_processing, supports_priority_processing,
};
use super::openai_chat_prepare_tools::prepare_chat_tools;
use super::openai_chat_prompt::{SystemMessageMode, convert_to_openai_chat_messages};
use ai_sdk_provider::language_model::call_options::LanguageModelCallOptions;
use ai_sdk_provider::language_model::call_warning::LanguageModelCallWarning;
use ai_sdk_provider::language_model::content::LanguageModelContent;
use ai_sdk_provider::language_model::content::source::LanguageModelSource;
use ai_sdk_provider::language_model::content::text::{LanguageModelText, TextType};
use ai_sdk_provider::language_model::content::tool_call::{LanguageModelToolCall, ToolCallType};
use ai_sdk_provider::language_model::stream_part::LanguageModelStreamPart;
use ai_sdk_provider::language_model::usage::LanguageModelUsage;
use ai_sdk_provider::language_model::{
    LanguageModel, LanguageModelGenerateResponse, LanguageModelRequestMetadata,
    LanguageModelStreamResponse, StreamResponseMetadata,
};
use ai_sdk_provider::shared::provider_metadata::SharedProviderMetadata;
use async_trait::async_trait;
use futures_util::Stream;
use futures_util::StreamExt;
use regex::Regex;
use serde_json::{Value as JsonValue, json};
use std::collections::HashMap;

use std::sync::Arc;

/// Configuration for OpenAI chat language model
#[derive(Clone)]
pub struct OpenAIChatConfig {
    /// Provider name
    pub provider: String,
    /// Base URL for API calls
    pub base_url: String,
    /// Function to get headers
    pub headers: Arc<dyn Fn() -> HashMap<String, String> + Send + Sync>,
}

impl OpenAIChatConfig {
    /// Create a new config
    pub fn new(
        provider: String,
        base_url: String,
        headers: Arc<dyn Fn() -> HashMap<String, String> + Send + Sync>,
    ) -> Self {
        Self {
            provider,
            base_url,
            headers,
        }
    }
}

/// OpenAI Chat Language Model
pub struct OpenAIChatLanguageModel {
    model_id: OpenAIChatModelId,
    config: OpenAIChatConfig,
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

impl OpenAIChatLanguageModel {
    /// Create a new OpenAI chat language model
    pub fn new(model_id: OpenAIChatModelId, config: OpenAIChatConfig) -> Self {
        Self { model_id, config }
    }

    /// Get the system message mode based on the model
    fn get_system_message_mode(&self) -> SystemMessageMode {
        if is_reasoning_model(&self.model_id) {
            SystemMessageMode::Developer
        } else {
            SystemMessageMode::System
        }
    }

    /// Build request arguments from call options
    async fn build_request_args(
        &self,
        options: &LanguageModelCallOptions,
    ) -> Result<(JsonValue, Vec<LanguageModelCallWarning>), Box<dyn std::error::Error>> {
        let mut warnings = Vec::new();

        // Parse provider options
        let openai_options: OpenAIChatLanguageModelOptions =
            if let Some(ref provider_options) = options.provider_options {
                // Extract "openai" specific options
                if let Some(openai_opts) = provider_options.get("openai") {
                    serde_json::from_value(serde_json::to_value(openai_opts)?).unwrap_or_default()
                } else {
                    OpenAIChatLanguageModelOptions::default()
                }
            } else {
                OpenAIChatLanguageModelOptions::default()
            };

        let structured_outputs = openai_options.structured_outputs.unwrap_or(true);
        let strict_json_schema = openai_options.strict_json_schema.unwrap_or(false);

        // Check unsupported settings
        if options.top_k.is_some() {
            warnings.push(LanguageModelCallWarning::unsupported_setting("topK"));
        }

        // Convert messages
        let (messages, message_warnings) =
            convert_to_openai_chat_messages(&options.prompt, self.get_system_message_mode());
        warnings.extend(message_warnings);

        // Prepare tools
        let (tools, tool_choice, tool_warnings) = prepare_chat_tools(
            options.tools.as_deref(),
            options.tool_choice.as_ref(),
            structured_outputs,
            strict_json_schema,
        );
        warnings.extend(tool_warnings);

        // Build base arguments
        let mut args = json!({
            "model": self.model_id,
            "messages": messages,
        });

        // Add optional parameters
        if let Some(max_tokens) = options.max_output_tokens {
            args["max_tokens"] = json!(max_tokens);
        }
        if let Some(temp) = options.temperature {
            args["temperature"] = json!(temp);
        }
        if let Some(top_p) = options.top_p {
            args["top_p"] = json!(top_p);
        }
        if let Some(freq_penalty) = options.frequency_penalty {
            args["frequency_penalty"] = json!(freq_penalty);
        }
        if let Some(pres_penalty) = options.presence_penalty {
            args["presence_penalty"] = json!(pres_penalty);
        }
        if let Some(ref stop) = options.stop_sequences {
            args["stop"] = json!(stop);
        }
        if let Some(seed) = options.seed {
            args["seed"] = json!(seed);
        }

        // Add response format if specified
        if let Some(ref response_format) = options.response_format {
            use ai_sdk_provider::language_model::call_options::LanguageModelResponseFormat;
            match response_format {
                LanguageModelResponseFormat::Text => {
                    // No need to set response_format for text
                }
                LanguageModelResponseFormat::Json {
                    schema,
                    name,
                    description,
                } => {
                    if let Some(schema_val) = schema {
                        if structured_outputs {
                            args["response_format"] = json!({
                                "type": "json_schema",
                                "json_schema": {
                                    "schema": schema_val,
                                    "strict": strict_json_schema,
                                    "name": name.as_ref().unwrap_or(&"response".to_string()),
                                    "description": description
                                }
                            });
                        } else {
                            args["response_format"] = json!({ "type": "json_object" });
                        }
                    } else {
                        args["response_format"] = json!({ "type": "json_object" });
                    }
                }
            }
        }

        // Add tools
        if let Some(tools) = tools {
            args["tools"] = json!(tools);
        }
        if let Some(tool_choice) = tool_choice {
            args["tool_choice"] = json!(tool_choice);
        }

        // Add OpenAI-specific options
        if let Some(ref logit_bias) = openai_options.logit_bias {
            args["logit_bias"] = json!(logit_bias);
        }
        if let Some(ref logprobs) = openai_options.logprobs {
            match logprobs {
                super::openai_chat_options::LogprobsOption::Bool(b) => {
                    args["logprobs"] = json!(b);
                }
                super::openai_chat_options::LogprobsOption::Number(n) => {
                    args["logprobs"] = json!(true);
                    args["top_logprobs"] = json!(n);
                }
            }
        }
        if let Some(parallel) = openai_options.parallel_tool_calls {
            args["parallel_tool_calls"] = json!(parallel);
        }
        if let Some(ref user) = openai_options.user {
            args["user"] = json!(user);
        }
        if let Some(ref reasoning_effort) = openai_options.reasoning_effort {
            args["reasoning_effort"] = json!(reasoning_effort);
        }
        if let Some(max_completion) = openai_options.max_completion_tokens {
            args["max_completion_tokens"] = json!(max_completion);
        }
        if let Some(store) = openai_options.store {
            args["store"] = json!(store);
        }
        if let Some(ref metadata) = openai_options.metadata {
            args["metadata"] = json!(metadata);
        }
        if let Some(ref prediction) = openai_options.prediction {
            args["prediction"] = json!(prediction);
        }
        if let Some(ref service_tier) = openai_options.service_tier {
            args["service_tier"] = json!(service_tier);
        }
        if let Some(ref text_verbosity) = openai_options.text_verbosity {
            args["verbosity"] = json!(text_verbosity);
        }
        if let Some(ref prompt_cache_key) = openai_options.prompt_cache_key {
            args["prompt_cache_key"] = json!(prompt_cache_key);
        }
        if let Some(ref safety_id) = openai_options.safety_identifier {
            args["safety_identifier"] = json!(safety_id);
        }

        // Handle reasoning models
        if is_reasoning_model(&self.model_id) {
            // Remove unsupported settings
            if args.get("temperature").is_some() {
                args.as_object_mut().unwrap().remove("temperature");
                warnings.push(LanguageModelCallWarning::unsupported_setting_with_details(
                    "temperature",
                    "temperature is not supported for reasoning models",
                ));
            }
            if args.get("top_p").is_some() {
                args.as_object_mut().unwrap().remove("top_p");
                warnings.push(LanguageModelCallWarning::unsupported_setting_with_details(
                    "topP",
                    "topP is not supported for reasoning models",
                ));
            }
            if args.get("frequency_penalty").is_some() {
                args.as_object_mut().unwrap().remove("frequency_penalty");
                warnings.push(LanguageModelCallWarning::unsupported_setting_with_details(
                    "frequencyPenalty",
                    "frequencyPenalty is not supported for reasoning models",
                ));
            }
            if args.get("presence_penalty").is_some() {
                args.as_object_mut().unwrap().remove("presence_penalty");
                warnings.push(LanguageModelCallWarning::unsupported_setting_with_details(
                    "presencePenalty",
                    "presencePenalty is not supported for reasoning models",
                ));
            }

            // Use max_completion_tokens instead of max_tokens
            if let Some(max_tokens) = args.get("max_tokens") {
                if args.get("max_completion_tokens").is_none() {
                    args["max_completion_tokens"] = max_tokens.clone();
                }
                args.as_object_mut().unwrap().remove("max_tokens");
            }
        }

        // Validate service tier support
        if let Some(ref service_tier) = openai_options.service_tier {
            use super::openai_chat_options::ServiceTier;
            match service_tier {
                ServiceTier::Flex => {
                    if !supports_flex_processing(&self.model_id) {
                        warnings.push(LanguageModelCallWarning::unsupported_setting_with_details(
                            "serviceTier",
                            "flex processing is only available for o3, o4-mini, and gpt-5 models",
                        ));
                        args.as_object_mut().unwrap().remove("service_tier");
                    }
                }
                ServiceTier::Priority => {
                    if !supports_priority_processing(&self.model_id) {
                        warnings.push(LanguageModelCallWarning::unsupported_setting_with_details(
                            "serviceTier",
                            "priority processing is only available for supported models and requires Enterprise access"
                        ));
                        args.as_object_mut().unwrap().remove("service_tier");
                    }
                }
                _ => {}
            }
        }

        Ok((args, warnings))
    }

    /// Process SSE byte stream into LanguageModelStreamPart events
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
                                    if let Ok(chunk) = serde_json::from_str::<OpenAIChatChunk>(data) {
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
        chunk: OpenAIChatChunk,
    ) -> Vec<LanguageModelStreamPart> {
        let mut parts = Vec::new();

        // Get the first choice (we only handle single choice for now)
        let choice = match chunk.choices.first() {
            Some(c) => c,
            None => return parts,
        };

        let delta = match &choice.delta {
            Some(d) => d,
            None => return parts,
        };

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
            let mapped_finish_reason = map_openai_finish_reason(Some(finish_reason));

            // Emit finish event
            parts.push(LanguageModelStreamPart::finish(usage, mapped_finish_reason));
        }

        parts
    }
}

#[async_trait]
impl LanguageModel for OpenAIChatLanguageModel {
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
        let mut urls = HashMap::new();
        // Support all HTTPS image URLs
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
        let (body, warnings) = self.build_request_args(&options).await?;

        // Make HTTP request
        let client = reqwest::Client::new();
        let headers = (self.config.headers)();
        let url = format!("{}/chat/completions", self.config.base_url);

        let mut request = client.post(&url).json(&body);
        for (key, value) in headers {
            request = request.header(key, value);
        }

        let response = request.send().await?;
        let _response_headers = response.headers().clone();
        let response_body: OpenAIChatResponse = response.json().await?;

        // Process response
        let choice = &response_body.choices[0];
        let mut content = Vec::new();

        // Add text content
        if let Some(ref text) = choice.message.content
            && !text.is_empty()
        {
            content.push(LanguageModelContent::Text(LanguageModelText {
                content_type: TextType,
                text: text.clone(),
                provider_metadata: None,
            }));
        }

        // Add tool calls
        if let Some(ref tool_calls) = choice.message.tool_calls {
            for tool_call in tool_calls {
                content.push(LanguageModelContent::ToolCall(LanguageModelToolCall {
                    content_type: ToolCallType,
                    tool_call_id: tool_call
                        .id
                        .clone()
                        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
                    tool_name: tool_call.function.name.clone(),
                    input: tool_call.function.arguments.clone(),
                    provider_executed: None,
                    provider_metadata: None,
                }));
            }
        }

        // Add annotations (sources)
        if let Some(ref annotations) = choice.message.annotations {
            for annotation in annotations {
                content.push(LanguageModelContent::Source(LanguageModelSource::Url {
                    id: uuid::Uuid::new_v4().to_string(),
                    url: annotation.url.clone(),
                    title: Some(annotation.title.clone()),
                    provider_metadata: None,
                }));
            }
        }

        // Build usage
        let usage = LanguageModelUsage {
            input_tokens: response_body
                .usage
                .as_ref()
                .and_then(|u| u.prompt_tokens)
                .unwrap_or(0),
            output_tokens: response_body
                .usage
                .as_ref()
                .and_then(|u| u.completion_tokens)
                .unwrap_or(0),
            total_tokens: response_body
                .usage
                .as_ref()
                .and_then(|u| u.total_tokens)
                .unwrap_or(0),
            reasoning_tokens: response_body
                .usage
                .as_ref()
                .and_then(|u| u.completion_tokens_details.as_ref())
                .and_then(|d| d.reasoning_tokens)
                .unwrap_or(0),
            cached_input_tokens: response_body
                .usage
                .as_ref()
                .and_then(|u| u.prompt_tokens_details.as_ref())
                .and_then(|d| d.cached_tokens)
                .unwrap_or(0),
        };

        // Build provider metadata
        let mut provider_metadata = SharedProviderMetadata::new();
        if let Some(ref usage) = response_body.usage
            && let Some(ref completion_details) = usage.completion_tokens_details
            && let Some(reasoning) = completion_details.reasoning_tokens
        {
            let mut openai_metadata = std::collections::HashMap::new();
            openai_metadata.insert("reasoning_tokens".to_string(), json!(reasoning));
            provider_metadata.insert("openai".to_string(), openai_metadata);
        }

        Ok(LanguageModelGenerateResponse {
            content,
            finish_reason: map_openai_finish_reason(choice.finish_reason.as_deref()),
            usage,
            provider_metadata: Some(provider_metadata),
            request: Some(LanguageModelRequestMetadata { body: Some(body) }),
            response: Some(
                ai_sdk_provider::language_model::response_metadata::LanguageModelResponseMetadata {
                    id: response_body.id,
                    model_id: response_body.model,
                    timestamp: response_body.created,
                },
            ),
            warnings,
        })
    }

    async fn do_stream(
        &self,
        options: LanguageModelCallOptions,
    ) -> Result<LanguageModelStreamResponse, Box<dyn std::error::Error>> {
        let (mut body, warnings) = self.build_request_args(&options).await?;

        // Add streaming parameters
        body["stream"] = json!(true);
        body["stream_options"] = json!({ "include_usage": true });

        // Make HTTP request
        let client = reqwest::Client::new();
        let headers = (self.config.headers)();
        let url = format!("{}/chat/completions", self.config.base_url);

        let mut request = client.post(&url).json(&body);
        for (key, value) in headers {
            request = request.header(key, value);
        }

        let response = request.send().await?;
        let response_headers = response.headers().clone();

        // Create the stream processor
        let byte_stream = response.bytes_stream();
        let stream = Self::process_stream(byte_stream, warnings);

        Ok(LanguageModelStreamResponse {
            stream: Box::new(stream),
            request: Some(LanguageModelRequestMetadata { body: Some(body) }),
            response: Some(StreamResponseMetadata {
                headers: Some(
                    response_headers
                        .iter()
                        .map(|(k, v)| {
                            (k.as_str().to_string(), v.to_str().unwrap_or("").to_string())
                        })
                        .collect(),
                ),
            }),
        })
    }
}
