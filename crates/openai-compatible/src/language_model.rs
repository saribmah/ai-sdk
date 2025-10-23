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
    config: OpenAICompatibleChatConfig,
) -> Box<dyn LanguageModel> {
    Box::new(OpenAICompatibleLanguageModel::new(model_id, config))
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
