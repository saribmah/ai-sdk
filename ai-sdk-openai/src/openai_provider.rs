//! OpenAI Provider Implementation
//!
//! This module provides the main provider interface for creating OpenAI language models.

use crate::chat::openai_chat_language_model::OpenAIChatConfig;
use crate::chat::{OpenAIChatLanguageModel, OpenAIChatModelId};
use ai_sdk_provider::embedding_model::EmbeddingModel;
use ai_sdk_provider::error::ProviderError;
use ai_sdk_provider::image_model::ImageModel;
use ai_sdk_provider::language_model::LanguageModel;
use ai_sdk_provider::provider::Provider;
use ai_sdk_provider::reranking_model::RerankingModel;
use ai_sdk_provider::speech_model::SpeechModel;
use ai_sdk_provider::transcription_model::TranscriptionModel;
use std::collections::HashMap;
use std::sync::Arc;

/// Settings for configuring the OpenAI provider.
#[derive(Debug, Clone, Default)]
pub struct OpenAIProviderSettings {
    /// Base URL for the OpenAI API calls.
    /// Defaults to `https://api.openai.com/v1`.
    pub base_url: Option<String>,

    /// API key for authenticating requests.
    /// Defaults to the `OPENAI_API_KEY` environment variable.
    pub api_key: Option<String>,

    /// OpenAI Organization ID.
    pub organization: Option<String>,

    /// OpenAI project ID.
    pub project: Option<String>,

    /// Custom headers to include in the requests.
    pub headers: Option<HashMap<String, String>>,

    /// Provider name. Overrides the `openai` default name for 3rd party providers.
    pub name: Option<String>,
}

impl OpenAIProviderSettings {
    /// Create a new settings instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the base URL.
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Set the API key.
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Set the organization ID.
    pub fn with_organization(mut self, organization: impl Into<String>) -> Self {
        self.organization = Some(organization.into());
        self
    }

    /// Set the project ID.
    pub fn with_project(mut self, project: impl Into<String>) -> Self {
        self.project = Some(project.into());
        self
    }

    /// Set custom headers.
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Add a single header.
    pub fn add_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        if let Some(ref mut headers) = self.headers {
            headers.insert(key.into(), value.into());
        } else {
            let mut headers = HashMap::new();
            headers.insert(key.into(), value.into());
            self.headers = Some(headers);
        }
        self
    }

    /// Set the provider name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

/// OpenAI provider for creating language models.
#[derive(Clone)]
pub struct OpenAIProvider {
    base_url: String,
    provider_name: String,
    headers: Arc<dyn Fn() -> HashMap<String, String> + Send + Sync>,
}

impl OpenAIProvider {
    /// Create a chat model with the given model ID.
    pub fn chat(&self, model_id: impl Into<String>) -> OpenAIChatLanguageModel {
        self.create_chat_model(model_id.into())
    }

    /// Create a language model with the given model ID.
    pub fn language_model(&self, model_id: impl Into<String>) -> OpenAIChatLanguageModel {
        self.create_chat_model(model_id.into())
    }

    fn create_chat_model(&self, model_id: OpenAIChatModelId) -> OpenAIChatLanguageModel {
        let config = OpenAIChatConfig::new(
            format!("{}.chat", self.provider_name),
            self.base_url.clone(),
            self.headers.clone(),
        );

        OpenAIChatLanguageModel::new(model_id, config)
    }
}

impl Provider for OpenAIProvider {
    fn specification_version(&self) -> &str {
        "v3"
    }

    fn language_model(&self, model_id: &str) -> Result<Arc<dyn LanguageModel>, ProviderError> {
        Ok(Arc::new(self.create_chat_model(model_id.to_string())))
    }

    fn text_embedding_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn EmbeddingModel<String>>, ProviderError> {
        Err(ProviderError::no_such_model(model_id, "textEmbeddingModel"))
    }

    fn image_model(&self, model_id: &str) -> Result<Arc<dyn ImageModel>, ProviderError> {
        Err(ProviderError::no_such_model(model_id, "imageModel"))
    }

    fn transcription_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn TranscriptionModel>, ProviderError> {
        Err(ProviderError::no_such_model(model_id, "transcriptionModel"))
    }

    fn speech_model(&self, model_id: &str) -> Result<Arc<dyn SpeechModel>, ProviderError> {
        Err(ProviderError::no_such_model(model_id, "speechModel"))
    }

    fn reranking_model(&self, model_id: &str) -> Result<Arc<dyn RerankingModel>, ProviderError> {
        Err(ProviderError::no_such_model(model_id, "rerankingModel"))
    }
}

/// Create an OpenAI provider instance.
///
/// # Arguments
///
/// * `settings` - Configuration settings for the provider
///
/// # Returns
///
/// An `OpenAIProvider` instance that can create language models.
///
/// # Example
///
/// ```no_run
/// use ai_sdk_openai::{create_openai, OpenAIProviderSettings};
///
/// // Create with default settings (uses OPENAI_API_KEY env var)
/// let provider = create_openai(OpenAIProviderSettings::default());
///
/// // Create with custom settings
/// let provider = create_openai(
///     OpenAIProviderSettings::new()
///         .with_api_key("your-api-key")
///         .with_base_url("https://api.openai.com/v1")
/// );
///
/// // Create a model
/// let model = provider.chat("gpt-4");
/// ```
pub fn create_openai(settings: OpenAIProviderSettings) -> OpenAIProvider {
    // Get base URL from settings or environment variable
    let base_url = settings
        .base_url
        .or_else(|| std::env::var("OPENAI_BASE_URL").ok())
        .unwrap_or_else(|| "https://api.openai.com/v1".to_string());

    // Remove trailing slash if present
    let base_url = base_url.trim_end_matches('/').to_string();

    // Get provider name
    let provider_name = settings.name.unwrap_or_else(|| "openai".to_string());

    // Get API key
    let api_key = settings
        .api_key
        .or_else(|| std::env::var("OPENAI_API_KEY").ok())
        .expect("OPENAI_API_KEY must be set either in settings or environment variable");

    let organization = settings.organization;
    let project = settings.project;
    let custom_headers = settings.headers.unwrap_or_default();

    // Create headers closure
    let headers_fn = Arc::new(move || {
        let mut headers = HashMap::new();

        // Required OpenAI headers
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));

        // Optional headers
        if let Some(ref org) = organization {
            headers.insert("OpenAI-Organization".to_string(), org.clone());
        }
        if let Some(ref proj) = project {
            headers.insert("OpenAI-Project".to_string(), proj.clone());
        }

        // Add custom headers
        headers.extend(custom_headers.clone());

        // TODO: Add User-Agent header with SDK version
        // headers.insert("User-Agent".to_string(), format!("ai-sdk/openai/{}", VERSION));

        headers
    });

    OpenAIProvider {
        base_url,
        provider_name,
        headers: headers_fn,
    }
}

/// Default OpenAI provider instance.
///
/// # Panics
///
/// Panics if the `OPENAI_API_KEY` environment variable is not set.
///
/// # Example
///
/// ```no_run
/// use ai_sdk_openai::openai;
///
/// let model = openai().chat("gpt-4");
/// ```
pub fn openai() -> OpenAIProvider {
    create_openai(OpenAIProviderSettings::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_builder() {
        let settings = OpenAIProviderSettings::new()
            .with_api_key("test-key")
            .with_base_url("https://custom.api.com")
            .with_name("custom-provider");

        assert_eq!(settings.api_key, Some("test-key".to_string()));
        assert_eq!(
            settings.base_url,
            Some("https://custom.api.com".to_string())
        );
        assert_eq!(settings.name, Some("custom-provider".to_string()));
    }

    #[test]
    fn test_settings_add_header() {
        let settings = OpenAIProviderSettings::new()
            .add_header("Custom-Header", "value1")
            .add_header("Another-Header", "value2");

        let headers = settings.headers.unwrap();
        assert_eq!(headers.get("Custom-Header"), Some(&"value1".to_string()));
        assert_eq!(headers.get("Another-Header"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_create_provider_removes_trailing_slash() {
        let provider = create_openai(
            OpenAIProviderSettings::new()
                .with_api_key("test-key")
                .with_base_url("https://api.openai.com/v1/"),
        );

        assert_eq!(provider.base_url, "https://api.openai.com/v1");
    }

    #[test]
    fn test_provider_name_default() {
        let provider = create_openai(OpenAIProviderSettings::new().with_api_key("test-key"));

        assert_eq!(provider.provider_name, "openai");
    }

    #[test]
    fn test_provider_name_custom() {
        let provider = create_openai(
            OpenAIProviderSettings::new()
                .with_api_key("test-key")
                .with_name("my-provider"),
        );

        assert_eq!(provider.provider_name, "my-provider");
    }

    #[test]
    fn test_headers_function() {
        let provider = create_openai(
            OpenAIProviderSettings::new()
                .with_api_key("test-key")
                .add_header("Custom", "value"),
        );

        let headers = (provider.headers)();
        assert_eq!(
            headers.get("Authorization"),
            Some(&"Bearer test-key".to_string())
        );
        assert_eq!(headers.get("Custom"), Some(&"value".to_string()));
    }

    #[test]
    fn test_create_chat_model() {
        let provider = create_openai(OpenAIProviderSettings::new().with_api_key("test-key"));

        let model = provider.chat("gpt-4");
        assert_eq!(model.model_id(), "gpt-4");
        assert_eq!(model.provider(), "openai.chat");
    }

    #[test]
    fn test_provider_trait() {
        let provider = create_openai(OpenAIProviderSettings::new().with_api_key("test-key"));

        assert_eq!(provider.specification_version(), "v3");
        assert_eq!(provider.provider_name, "openai");
    }
}
