use ai_sdk_provider::language_model::{
    call_options::CallOptions, LanguageModel, LanguageModelGenerateResponse,
    LanguageModelStreamResponse,
};
use async_trait::async_trait;
use regex::Regex;
use std::collections::HashMap;
use std::sync::Arc;

/// Configuration for the OpenAI-compatible language model.
/// This is similar to OpenAICompatibleChatConfig in the TypeScript implementation.
#[derive(Clone)]
pub struct OpenAICompatibleChatConfig {
    /// The provider identifier (e.g., "openai.chat", "azure-openai.chat")
    pub provider: String,

    /// Function to get headers for requests
    pub headers: Arc<dyn Fn() -> HashMap<String, String> + Send + Sync>,

    /// Function to construct the URL for API calls
    pub url: Arc<dyn Fn(&UrlOptions) -> String + Send + Sync>,

    /// Optional API key for authentication
    pub api_key: Option<String>,

    /// Optional organization ID
    pub organization: Option<String>,

    /// Optional project ID
    pub project: Option<String>,

    /// Optional query parameters
    pub query_params: HashMap<String, String>,
}

/// Options for constructing URLs
pub struct UrlOptions {
    pub model_id: String,
    pub path: String,
}

impl std::fmt::Debug for OpenAICompatibleChatConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAICompatibleChatConfig")
            .field("provider", &self.provider)
            .field("api_key", &self.api_key.as_ref().map(|_| "***"))
            .field("organization", &self.organization)
            .field("project", &self.project)
            .field("query_params", &self.query_params)
            .finish()
    }
}

/// OpenAI-compatible language model implementation.
/// This is similar to OpenAICompatibleChatLanguageModel in the TypeScript implementation.
pub struct OpenAICompatibleLanguageModel {
    model_id: String,
    config: OpenAICompatibleChatConfig,
    http_client: reqwest::Client,
}

impl OpenAICompatibleLanguageModel {
    /// Creates a new OpenAI-compatible language model instance.
    pub fn new(model_id: impl Into<String>, config: OpenAICompatibleChatConfig) -> Self {
        let http_client = reqwest::Client::new();

        Self {
            model_id: model_id.into(),
            config,
            http_client,
        }
    }

    /// Gets the provider name
    fn get_provider(&self) -> &str {
        &self.config.provider
    }

    /// Gets the model ID
    fn get_model_id(&self) -> &str {
        &self.model_id
    }
}

/// Convert provider messages to OpenAI-specific format
fn convert_messages_to_openai(
    messages: Vec<ai_sdk_provider::language_model::prompt::Message>,
) -> Result<Vec<OpenAIRequestMessage>, Box<dyn std::error::Error>> {
    use ai_sdk_provider::language_model::prompt::{AssistantMessagePart, Message};

    let mut openai_messages = Vec::new();

    for message in messages {
        let openai_msg = match message {
            Message::System { content, .. } => OpenAIRequestMessage::System { content },

            Message::User { content, .. } => {
                // Serialize user content to JSON value
                let content_json = serde_json::to_value(&content)?;
                OpenAIRequestMessage::User {
                    content: content_json,
                }
            }

            Message::Assistant { content, .. } => {
                // Separate tool calls from other content
                let mut tool_calls_vec = Vec::new();
                let mut other_content = Vec::new();

                for part in content {
                    match part {
                        AssistantMessagePart::ToolCall {
                            tool_call_id,
                            tool_name,
                            input,
                            ..
                        } => {
                            // Convert Value to JSON string for OpenAI
                            let arguments_str = serde_json::to_string(&input)?;
                            tool_calls_vec.push(OpenAIRequestToolCall {
                                id: tool_call_id,
                                tool_type: "function".to_string(),
                                function: OpenAIRequestFunctionCall {
                                    name: tool_name,
                                    arguments: arguments_str,
                                },
                            });
                        }
                        _ => other_content.push(part),
                    }
                }

                // Build content value (text, reasoning, etc.)
                let content_value = if other_content.is_empty() {
                    None
                } else {
                    Some(serde_json::to_value(&other_content)?)
                };

                // Build tool_calls array
                let tool_calls = if tool_calls_vec.is_empty() {
                    None
                } else {
                    Some(tool_calls_vec)
                };

                OpenAIRequestMessage::Assistant {
                    content: content_value,
                    tool_calls,
                }
            }

            Message::Tool { content, .. } => {
                // Tool messages contain tool results
                // OpenAI requires one tool message per tool result with matching tool_call_id
                // So we need to expand this into multiple messages
                for result in content {
                    let result_content = serde_json::to_string(&result.output)
                        .unwrap_or_else(|_| "{}".to_string());

                    openai_messages.push(OpenAIRequestMessage::Tool {
                        content: result_content,
                        tool_call_id: result.tool_call_id.clone(),
                    });
                }
                continue; // Skip the push at the end since we already added messages
            }
        };

        openai_messages.push(openai_msg);
    }

    Ok(openai_messages)
}

#[async_trait]
impl LanguageModel for OpenAICompatibleLanguageModel {
    fn provider(&self) -> &str {
        self.get_provider()
    }

    fn model_id(&self) -> &str {
        self.get_model_id()
    }

    async fn supported_urls(&self) -> HashMap<String, Vec<Regex>> {
        // OpenAI-compatible APIs typically support HTTP(S) URLs for images
        let mut urls = HashMap::new();
        urls.insert(
            "image".to_string(),
            vec![
                Regex::new(r"^https?://").unwrap(),
                Regex::new(r"^data:image/").unwrap(),
            ],
        );
        urls
    }

    async fn do_generate(
        &self,
        options: CallOptions,
    ) -> Result<LanguageModelGenerateResponse, Box<dyn std::error::Error>> {
        use ai_sdk_provider::language_model::content::Content;
        use ai_sdk_provider::language_model::reasoning::Reasoning;
        use ai_sdk_provider::language_model::response_metadata::ResponseMetadata;
        use ai_sdk_provider::language_model::text::Text;
        use ai_sdk_provider::language_model::tool_call::ToolCall;
        use ai_sdk_provider::language_model::usage::Usage;
        use ai_sdk_provider::language_model::RequestMetadata;

        // Convert provider messages to OpenAI format
        let openai_messages = convert_messages_to_openai(options.prompt.clone())?;

        // Build the request body
        let request = OpenAIChatRequest {
            model: self.model_id.clone(),
            messages: openai_messages,
            max_tokens: options.max_output_tokens,
            temperature: options.temperature,
            top_p: options.top_p,
            presence_penalty: options.presence_penalty,
            frequency_penalty: options.frequency_penalty,
            stop: options.stop_sequences.clone(),
            seed: options.seed,
            tools: options.tools.clone().map(convert_tools_to_openai),
            tool_choice: options.tool_choice.as_ref().map(convert_tool_choice),
            response_format: options.response_format.as_ref().map(|format| {
                use ai_sdk_provider::language_model::call_options::ResponseFormat;
                match format {
                    ResponseFormat::Text => serde_json::json!({"type": "text"}),
                    ResponseFormat::Json { schema, name, description } => {
                        let mut json_obj = serde_json::json!({"type": "json_object"});
                        if let Some(s) = schema {
                            json_obj["schema"] = s.clone();
                        }
                        if let Some(n) = name {
                            json_obj["name"] = Value::String(n.clone());
                        }
                        if let Some(d) = description {
                            json_obj["description"] = Value::String(d.clone());
                        }
                        json_obj
                    }
                }
            }),
        };

        // Build headers
        let mut headers = (self.config.headers)();
        if let Some(extra_headers) = &options.headers {
            headers.extend(extra_headers.clone());
        }

        // Build URL
        let url = (self.config.url)(&UrlOptions {
            model_id: self.model_id.clone(),
            path: "/chat/completions".to_string(),
        });

        // Serialize request body
        let body_json = serde_json::to_string(&request)?;

        // Make the HTTP request
        let mut req_builder = self.http_client.post(&url);

        // Add headers
        for (key, value) in &headers {
            req_builder = req_builder.header(key, value);
        }

        // Add body
        req_builder = req_builder
            .header("Content-Type", "application/json")
            .body(body_json.clone());

        // Send request
        let response = req_builder.send().await?;

        // Check status
        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("API request failed with status {}: {}", status, error_text).into());
        }

        // Parse response
        let response_text = response.text().await?;
        let response_body: OpenAIChatResponse = serde_json::from_str(&response_text)?;

        // Extract the first choice
        let choice = response_body.choices.get(0)
            .ok_or("No choices in response")?;

        // Build content array
        let mut content: Vec<Content> = Vec::new();

        // Add text content
        if let Some(text) = &choice.message.content {
            if !text.is_empty() {
                content.push(Content::Text(Text::new(text)));
            }
        }

        // Add reasoning content
        let reasoning = choice.message.reasoning_content.as_ref()
            .or(choice.message.reasoning.as_ref());
        if let Some(reasoning_text) = reasoning {
            if !reasoning_text.is_empty() {
                content.push(Content::Reasoning(Reasoning::init(reasoning_text)));
            }
        }

        // Add tool calls
        if let Some(tool_calls) = &choice.message.tool_calls {
            for tool_call in tool_calls {
                let tool_call_id = tool_call.id.clone()
                    .unwrap_or_else(generate_id);

                content.push(Content::ToolCall(ToolCall::new(
                    tool_call_id,
                    &tool_call.function.name,
                    &tool_call.function.arguments,
                )));
            }
        }

        // Build usage
        let usage = if let Some(usage_data) = &response_body.usage {
            Usage {
                input_tokens: usage_data.prompt_tokens.unwrap_or(0),
                output_tokens: usage_data.completion_tokens.unwrap_or(0),
                total_tokens: usage_data.total_tokens.unwrap_or(0),
                reasoning_tokens: usage_data.completion_tokens_details
                    .as_ref()
                    .and_then(|d| d.reasoning_tokens)
                    .unwrap_or(0),
                cached_input_tokens: usage_data.prompt_tokens_details
                    .as_ref()
                    .and_then(|d| d.cached_tokens)
                    .unwrap_or(0),
            }
        } else {
            Usage::default()
        };

        // Build response
        Ok(LanguageModelGenerateResponse {
            content,
            finish_reason: map_finish_reason(choice.finish_reason.clone()),
            usage,
            provider_metadata: None,
            request: Some(RequestMetadata {
                body: Some(serde_json::from_str(&body_json)?),
            }),
            response: Some(ResponseMetadata {
                id: Some(response_body.id),
                model_id: Some(response_body.model),
                timestamp: Some(response_body.created as i64),
            }),
            warnings: Vec::new(),
        })
    }

    async fn do_stream(
        &self,
        _options: CallOptions,
    ) -> Result<LanguageModelStreamResponse, Box<dyn std::error::Error>> {
        // TODO: Implement actual streaming logic
        Err("Not yet implemented".into())
    }
}

/// Creates a new OpenAI-compatible language model.
///
/// # Arguments
///
/// * `model_id` - The model ID to use (e.g., "gpt-4", "gpt-3.5-turbo")
/// * `config` - Configuration for the language model
///
/// # Returns
///
/// A boxed `LanguageModel` trait object.
pub fn create_language_model(
    model_id: impl Into<String>,
    config: OpenAICompatibleChatConfig,
) -> Box<dyn LanguageModel> {
    Box::new(OpenAICompatibleLanguageModel::new(model_id, config))
}

// ============================================================================
// OpenAI API Request/Response Types
// ============================================================================

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// OpenAI-specific message format for requests
#[derive(Debug, Serialize)]
#[serde(tag = "role", rename_all = "lowercase")]
enum OpenAIRequestMessage {
    System {
        content: String,
    },
    User {
        content: Value,
    },
    Assistant {
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<OpenAIRequestToolCall>>,
    },
    Tool {
        content: String,
        tool_call_id: String,
    },
}

/// OpenAI tool call in assistant message for requests
#[derive(Debug, Serialize)]
struct OpenAIRequestToolCall {
    id: String,
    #[serde(rename = "type")]
    tool_type: String,
    function: OpenAIRequestFunctionCall,
}

/// OpenAI function call details for requests
#[derive(Debug, Serialize)]
struct OpenAIRequestFunctionCall {
    name: String,
    arguments: String,
}

/// OpenAI chat completion request
#[derive(Debug, Serialize)]
struct OpenAIChatRequest {
    model: String,
    messages: Vec<OpenAIRequestMessage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OpenAITool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    response_format: Option<Value>,
}

/// OpenAI tool definition
#[derive(Debug, Serialize)]
struct OpenAITool {
    #[serde(rename = "type")]
    tool_type: String,
    function: OpenAIFunction,
}

/// OpenAI function definition
#[derive(Debug, Serialize)]
struct OpenAIFunction {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    parameters: Value,
}

/// OpenAI chat completion response
#[derive(Debug, Deserialize)]
struct OpenAIChatResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<OpenAIChoice>,
    usage: Option<OpenAIUsage>,
}

/// OpenAI choice in response
#[derive(Debug, Deserialize)]
struct OpenAIChoice {
    index: u32,
    message: OpenAIMessage,
    finish_reason: Option<String>,
}

/// OpenAI message in response
#[derive(Debug, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: Option<String>,
    #[serde(default)]
    tool_calls: Option<Vec<OpenAIToolCall>>,

    // Extended fields for reasoning (some providers)
    #[serde(default)]
    reasoning: Option<String>,
    #[serde(default)]
    reasoning_content: Option<String>,
}

/// OpenAI tool call in response
#[derive(Debug, Deserialize)]
struct OpenAIToolCall {
    id: Option<String>,
    #[serde(rename = "type")]
    tool_type: String,
    function: OpenAIToolCallFunction,
}

/// OpenAI tool call function
#[derive(Debug, Deserialize)]
struct OpenAIToolCallFunction {
    name: String,
    arguments: String,
}

/// OpenAI usage information
#[derive(Debug, Deserialize)]
struct OpenAIUsage {
    prompt_tokens: Option<u64>,
    completion_tokens: Option<u64>,
    total_tokens: Option<u64>,
    #[serde(default)]
    completion_tokens_details: Option<OpenAICompletionTokensDetails>,
    #[serde(default)]
    prompt_tokens_details: Option<OpenAIPromptTokensDetails>,
}

/// OpenAI completion tokens details
#[derive(Debug, Deserialize)]
struct OpenAICompletionTokensDetails {
    reasoning_tokens: Option<u64>,
    accepted_prediction_tokens: Option<u64>,
    rejected_prediction_tokens: Option<u64>,
}

/// OpenAI prompt tokens details
#[derive(Debug, Deserialize)]
struct OpenAIPromptTokensDetails {
    cached_tokens: Option<u64>,
}

// ============================================================================
// Helper Functions
// ============================================================================

use ai_sdk_provider::language_model::call_options::Tool as ProviderTool;
use ai_sdk_provider::language_model::finish_reason::FinishReason;

/// Maps OpenAI finish reason to our FinishReason enum
fn map_finish_reason(finish_reason: Option<String>) -> FinishReason {
    match finish_reason.as_deref() {
        Some("stop") => FinishReason::Stop,
        Some("length") => FinishReason::Length,
        Some("content_filter") => FinishReason::ContentFilter,
        Some("tool_calls") => FinishReason::ToolCalls,
        Some("error") => FinishReason::Error,
        None => FinishReason::Unknown,
        _ => FinishReason::Other,
    }
}

/// Converts provider tools to OpenAI format
fn convert_tools_to_openai(tools: Vec<ProviderTool>) -> Vec<OpenAITool> {
    tools
        .into_iter()
        .filter_map(|tool| match tool {
            ProviderTool::Function(func_tool) => Some(OpenAITool {
                tool_type: "function".to_string(),
                function: OpenAIFunction {
                    name: func_tool.name,
                    description: func_tool.description,
                    parameters: func_tool.input_schema,
                },
            }),
            // Skip provider-defined tools as they're provider-specific
            ProviderTool::ProviderDefined(_) => None,
        })
        .collect()
}

/// Converts tool choice to OpenAI format
fn convert_tool_choice(choice: &ai_sdk_provider::language_model::tool_choice::ToolChoice) -> Value {
    use ai_sdk_provider::language_model::tool_choice::ToolChoice;

    match choice {
        ToolChoice::Auto => Value::String("auto".to_string()),
        ToolChoice::None => Value::String("none".to_string()),
        ToolChoice::Required => Value::String("required".to_string()),
        ToolChoice::Tool { name } => {
            serde_json::json!({
                "type": "function",
                "function": {
                    "name": name
                }
            })
        }
    }
}

/// Generates a unique ID for tool calls that don't have one
fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("call_{}", timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_language_model() {
        let base_url = "https://api.openai.com/v1".to_string();
        let config = OpenAICompatibleChatConfig {
            provider: "openai.chat".to_string(),
            headers: Arc::new(move || {
                let mut headers = HashMap::new();
                headers.insert("Authorization".to_string(), "Bearer test-key".to_string());
                headers
            }),
            url: Arc::new(move |opts: &UrlOptions| {
                format!("{}{}", base_url, opts.path)
            }),
            api_key: Some("test-key".to_string()),
            organization: None,
            project: None,
            query_params: HashMap::new(),
        };

        let model = create_language_model("gpt-4", config);
        assert_eq!(model.provider(), "openai.chat");
        assert_eq!(model.model_id(), "gpt-4");
    }
}
