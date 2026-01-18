//! OpenAI Provider Implementation
//!
//! This module provides the main provider interface for creating OpenAI language models.

use crate::chat::openai_chat_language_model::OpenAIChatConfig;
use crate::chat::{OpenAIChatLanguageModel, OpenAIChatModelId};
use crate::settings::OpenAIProviderSettings;
use llm_kit_provider::embedding_model::EmbeddingModel;
use llm_kit_provider::error::ProviderError;
use llm_kit_provider::image_model::ImageModel;
use llm_kit_provider::language_model::LanguageModel;
use llm_kit_provider::provider::Provider;
use llm_kit_provider::reranking_model::RerankingModel;
use llm_kit_provider::speech_model::SpeechModel;
use llm_kit_provider::transcription_model::TranscriptionModel;
use std::collections::HashMap;
use std::sync::Arc;

/// OpenAI provider for creating language models.
#[derive(Clone)]
pub struct OpenAIProvider {
    base_url: String,
    provider_name: String,
    headers: Arc<dyn Fn() -> HashMap<String, String> + Send + Sync>,
}

impl OpenAIProvider {
    /// Create a new OpenAI provider from settings.
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
    /// use llm_kit_openai::{OpenAIProvider, OpenAIProviderSettings};
    ///
    /// // Create with default settings (uses OPENAI_API_KEY env var)
    /// let provider = OpenAIProvider::new(OpenAIProviderSettings::default());
    ///
    /// // Create with custom settings
    /// let provider = OpenAIProvider::new(
    ///     OpenAIProviderSettings::new()
    ///         .with_api_key("your-api-key")
    ///         .with_base_url("https://api.openai.com/v1")
    /// );
    ///
    /// // Create a model
    /// let model = provider.chat("gpt-4");
    /// ```
    pub fn new(settings: OpenAIProviderSettings) -> Self {
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
            // headers.insert("User-Agent".to_string(), format!("llm-kit/openai/{}", VERSION));

            headers
        });

        OpenAIProvider {
            base_url,
            provider_name,
            headers: headers_fn,
        }
    }

    /// Create a chat model with the given model ID.
    pub fn chat(&self, model_id: impl Into<String>) -> OpenAIChatLanguageModel {
        self.create_chat_model(model_id.into())
    }

    /// Create a language model with the given model ID.
    pub fn language_model(&self, model_id: impl Into<String>) -> OpenAIChatLanguageModel {
        self.create_chat_model(model_id.into())
    }

    /// Get the base URL for this provider (for testing).
    #[doc(hidden)]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get the provider name (for testing).
    #[doc(hidden)]
    pub fn provider_name(&self) -> &str {
        &self.provider_name
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_provider_removes_trailing_slash() {
        let provider = OpenAIProvider::new(
            OpenAIProviderSettings::new()
                .with_api_key("test-key")
                .with_base_url("https://api.openai.com/v1/"),
        );

        assert_eq!(provider.base_url(), "https://api.openai.com/v1");
    }

    #[test]
    fn test_provider_name_default() {
        let provider = OpenAIProvider::new(OpenAIProviderSettings::new().with_api_key("test-key"));

        assert_eq!(provider.provider_name(), "openai");
    }

    #[test]
    fn test_provider_name_custom() {
        let provider = OpenAIProvider::new(
            OpenAIProviderSettings::new()
                .with_api_key("test-key")
                .with_name("my-provider"),
        );

        assert_eq!(provider.provider_name(), "my-provider");
    }

    #[test]
    fn test_headers_function() {
        let provider = OpenAIProvider::new(
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
        let provider = OpenAIProvider::new(OpenAIProviderSettings::new().with_api_key("test-key"));

        let model = provider.chat("gpt-4");
        assert_eq!(model.model_id(), "gpt-4");
        assert_eq!(model.provider(), "openai.chat");
    }

    #[test]
    fn test_provider_trait() {
        let provider = OpenAIProvider::new(OpenAIProviderSettings::new().with_api_key("test-key"));

        assert_eq!(provider.specification_version(), "v3");
        assert_eq!(provider.provider_name(), "openai");
    }
}
