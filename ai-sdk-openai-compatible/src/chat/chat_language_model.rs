use ai_sdk_provider::language_model::{
    call_options::CallOptions, call_warning::CallWarning, content::Content,
    finish_reason::FinishReason, reasoning::Reasoning, response_metadata::ResponseMetadata,
    text::Text, tool_call::ToolCall, usage::Usage, LanguageModel, LanguageModelGenerateResponse,
    LanguageModelStreamResponse,
};
use ai_sdk_provider::shared::provider_metadata::ProviderMetadata;
use async_trait::async_trait;
use regex::Regex;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::chat::{
    convert_to_openai_compatible_chat_messages, get_response_metadata,
    map_openai_compatible_finish_reason, prepare_tools, OpenAICompatibleChatModelId,
};
use crate::error::{DefaultOpenAICompatibleErrorStructure, ProviderErrorStructure};
use crate::chat::MetadataExtractor;

/// Configuration for an OpenAI-compatible chat language model
pub struct OpenAICompatibleChatConfig {
    /// Provider name (e.g., "openai", "azure", "custom")
    pub provider: String,

    /// Function to generate headers for API requests
    pub headers: fn() -> HashMap<String, String>,

    /// Function to generate the URL for API requests
    pub url: fn(model_id: &str, path: &str) -> String,

    /// Optional custom fetch function
    pub fetch: Option<fn()>, // TODO: proper fetch function type

    /// Whether to include usage information in streaming responses
    pub include_usage: bool,

    /// Whether the model supports structured outputs
    pub supports_structured_outputs: bool,

    /// Function to get supported URLs for the model
    pub supported_urls: Option<fn() -> HashMap<String, Vec<Regex>>>,
}

impl Default for OpenAICompatibleChatConfig {
    fn default() -> Self {
        Self {
            provider: "openai-compatible".to_string(),
            headers: || HashMap::new(),
            url: |_model_id, path| format!("https://api.openai.com/v1{}", path),
            fetch: None,
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

    /// Prepare arguments for API request
    fn prepare_request_body(&self, options: &CallOptions) -> Result<(Value, Vec<CallWarning>), Box<dyn std::error::Error>> {
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
        options: CallOptions,
    ) -> Result<LanguageModelGenerateResponse, Box<dyn std::error::Error>> {
        // Prepare request body
        let (body, mut warnings) = self.prepare_request_body(&options)?;
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

        let response = request.body(body_string.clone()).send().await?;
        let status = response.status();
        let response_headers = response.headers().clone();

        if !status.is_success() {
            let error_body = response.text().await?;
            return Err(format!("API request failed with status {}: {}", status, error_body).into());
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
        if let Some(text) = &choice.message.content {
            if !text.is_empty() {
                content.push(Content::Text(Text::new(text.clone())));
            }
        }

        // Add reasoning content
        let reasoning = choice
            .message
            .reasoning_content
            .as_ref()
            .or(choice.message.reasoning.as_ref());
        if let Some(reasoning_text) = reasoning {
            if !reasoning_text.is_empty() {
                content.push(Content::Reasoning(Reasoning::init(reasoning_text.clone())));
            }
        }

        // Add tool calls
        if let Some(tool_calls) = &choice.message.tool_calls {
            for tool_call in tool_calls {
                let tool_call_id = tool_call.id.clone().unwrap_or_else(|| "unknown".to_string());
                let tool_name = tool_call.function.name.clone();
                let input = tool_call.function.arguments.clone();
                content.push(Content::ToolCall(ToolCall::new(
                    tool_call_id,
                    tool_name,
                    input,
                )));
            }
        }

        // Build usage information
        let usage = if let Some(api_usage) = &api_response.usage {
            Usage {
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
            Usage::default()
        };

        // Build provider metadata
        let mut provider_metadata = HashMap::new();
        let mut provider_data = HashMap::new();
        provider_metadata.insert(self.provider_options_name().to_string(), provider_data);

        // Build response metadata
        let response_metadata = get_response_metadata(
            api_response.id.clone(),
            api_response.model.clone(),
            api_response.created,
        );

        // Map finish reason
        let finish_reason = map_openai_compatible_finish_reason(
            choice.finish_reason.as_deref()
        );

        Ok(LanguageModelGenerateResponse {
            content,
            finish_reason,
            usage,
            provider_metadata: Some(provider_metadata),
            request: Some(ai_sdk_provider::language_model::RequestMetadata {
                body: Some(body),
            }),
            response: Some(response_metadata),
            warnings,
        })
    }

    async fn do_stream(
        &self,
        _options: CallOptions,
    ) -> Result<LanguageModelStreamResponse, Box<dyn std::error::Error>> {
        // TODO: Implement in Stage 3
        todo!("do_stream will be implemented in Stage 3")
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
        let mut config = OpenAICompatibleChatConfig::default();
        config.provider = "openai.azure".to_string();
        let model = OpenAICompatibleChatLanguageModel::new("gpt-4".to_string(), config);

        assert_eq!(model.provider_options_name(), "openai");
    }

    #[test]
    fn test_provider_options_name_no_dot() {
        let mut config = OpenAICompatibleChatConfig::default();
        config.provider = "custom".to_string();
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
        let mut config = OpenAICompatibleChatConfig::default();
        config.supported_urls = Some(|| {
            let mut map = HashMap::new();
            map.insert("test".to_string(), vec![]);
            map
        });
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
        let mut config = OpenAICompatibleChatConfig::default();
        config.provider = "azure".to_string();

        assert_eq!(config.provider, "azure");
    }

    #[test]
    fn test_config_structured_outputs() {
        let mut config = OpenAICompatibleChatConfig::default();
        config.supports_structured_outputs = true;

        assert!(config.supports_structured_outputs);
    }
}
