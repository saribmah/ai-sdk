use ai_sdk_provider::language_model::{
    call_options::CallOptions, LanguageModel, LanguageModelGenerateResponse,
    LanguageModelStreamResponse,
};
use async_trait::async_trait;
use regex::Regex;
use std::collections::HashMap;

/// Configuration for the OpenAI-compatible language model.
#[derive(Debug, Clone)]
pub struct OpenAICompatibleLanguageModelConfig {
    /// The provider identifier (e.g., "openai", "azure-openai")
    pub provider: String,

    /// The base URL for the API
    pub base_url: String,

    /// The model ID to use
    pub model_id: String,

    /// Optional API key for authentication
    pub api_key: Option<String>,

    /// Optional organization ID
    pub organization: Option<String>,

    /// Optional project ID
    pub project: Option<String>,

    /// Additional headers to send with requests
    pub headers: HashMap<String, String>,
}

/// OpenAI-compatible language model implementation.
pub struct OpenAICompatibleLanguageModel {
    config: OpenAICompatibleLanguageModelConfig,
    http_client: reqwest::Client,
}

impl OpenAICompatibleLanguageModel {
    /// Creates a new OpenAI-compatible language model instance.
    pub fn new(config: OpenAICompatibleLanguageModelConfig) -> Self {
        let http_client = reqwest::Client::new();

        Self {
            config,
            http_client,
        }
    }
}

#[async_trait]
impl LanguageModel for OpenAICompatibleLanguageModel {
    fn provider(&self) -> &str {
        &self.config.provider
    }

    fn model_id(&self) -> &str {
        &self.config.model_id
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
        _options: CallOptions,
    ) -> Result<LanguageModelGenerateResponse, Box<dyn std::error::Error>> {
        // TODO: Implement actual generation logic
        Err("Not yet implemented".into())
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
    config: OpenAICompatibleLanguageModelConfig,
) -> Box<dyn LanguageModel> {
    let mut model_config = config;
    model_config.model_id = model_id.into();

    Box::new(OpenAICompatibleLanguageModel::new(model_config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_language_model() {
        let config = OpenAICompatibleLanguageModelConfig {
            provider: "openai".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            model_id: "".to_string(), // Will be set by create_language_model
            api_key: Some("test-key".to_string()),
            organization: None,
            project: None,
            headers: HashMap::new(),
        };

        let model = create_language_model("gpt-4", config);
        assert_eq!(model.provider(), "openai");
        assert_eq!(model.model_id(), "gpt-4");
    }
}
