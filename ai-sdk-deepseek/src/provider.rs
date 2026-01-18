use llm_kit_provider::EmbeddingModel;
use llm_kit_provider::error::ProviderError;
use llm_kit_provider::language_model::LanguageModel;
use llm_kit_provider::provider::Provider;
use std::collections::HashMap;
use std::sync::Arc;

use crate::chat::language_model::DeepSeekChatLanguageModel;
use crate::settings::DeepSeekProviderSettings;

/// DeepSeek provider implementation.
///
/// Provides methods to create language models for DeepSeek's chat and reasoning models.
pub struct DeepSeekProvider {
    settings: DeepSeekProviderSettings,
}

impl DeepSeekProvider {
    /// Creates a new DeepSeek provider.
    pub fn new(settings: DeepSeekProviderSettings) -> Self {
        Self { settings }
    }

    /// Returns the provider name.
    pub fn name(&self) -> &str {
        "deepseek"
    }

    /// Creates a chat language model with the given model ID.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use llm_kit_deepseek::DeepSeekClient;
    ///
    /// let provider = DeepSeekClient::new()
    ///     .api_key("your-api-key")
    ///     .build();
    ///
    /// let model = provider.chat_model("deepseek-chat");
    /// ```
    pub fn chat_model(&self, model_id: impl Into<String>) -> Arc<dyn LanguageModel> {
        let model_id = model_id.into();
        let config = self.create_chat_config();

        Arc::new(DeepSeekChatLanguageModel::new(model_id, config))
    }

    /// Alias for `chat_model()`.
    pub fn model(&self, model_id: impl Into<String>) -> Arc<dyn LanguageModel> {
        self.chat_model(model_id)
    }

    /// Creates the configuration for chat models.
    pub(crate) fn create_chat_config(&self) -> DeepSeekChatConfig {
        let api_key = self.settings.api_key.clone();
        let custom_headers = self.settings.headers.clone().unwrap_or_default();
        let base_url = self.settings.base_url.clone();

        DeepSeekChatConfig {
            provider: "deepseek.chat".to_string(),
            base_url,
            headers: Box::new(move || {
                let mut headers = HashMap::new();

                // Add Authorization header if API key is present
                if let Some(ref key) = api_key {
                    headers.insert("Authorization".to_string(), format!("Bearer {}", key));
                }

                // Add custom headers
                for (key, value) in &custom_headers {
                    headers.insert(key.clone(), value.clone());
                }

                headers
            }),
        }
    }

    /// Gets the provider base URL.
    pub fn base_url(&self) -> &str {
        &self.settings.base_url
    }
}

/// Configuration for DeepSeek chat models.
pub struct DeepSeekChatConfig {
    pub provider: String,
    pub base_url: String,
    pub headers: Box<dyn Fn() -> HashMap<String, String> + Send + Sync>,
}

// Implement the Provider trait for compatibility with the provider interface
impl Provider for DeepSeekProvider {
    fn language_model(&self, model_id: &str) -> Result<Arc<dyn LanguageModel>, ProviderError> {
        Ok(self.chat_model(model_id))
    }

    fn text_embedding_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn EmbeddingModel<String>>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "deepseek.embedding-model-not-supported".to_string(),
        ))
    }

    fn image_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn llm_kit_provider::ImageModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "deepseek.image-model-not-supported".to_string(),
        ))
    }

    fn transcription_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn llm_kit_provider::TranscriptionModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "deepseek.transcription-model-not-supported".to_string(),
        ))
    }

    fn speech_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn llm_kit_provider::SpeechModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "deepseek.speech-model-not-supported".to_string(),
        ))
    }

    fn reranking_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn llm_kit_provider::RerankingModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "deepseek.reranking-model-not-supported".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_new() {
        let settings = DeepSeekProviderSettings::new().with_api_key("test-key");

        let provider = DeepSeekProvider::new(settings);
        let model = provider.chat_model("deepseek-chat");

        assert_eq!(model.provider(), "deepseek.chat");
        assert_eq!(model.model_id(), "deepseek-chat");
    }

    #[test]
    fn test_chat_model() {
        let settings = DeepSeekProviderSettings::new().with_api_key("test-key");

        let provider = DeepSeekProvider::new(settings);
        let model = provider.chat_model("deepseek-reasoner");

        assert_eq!(model.provider(), "deepseek.chat");
        assert_eq!(model.model_id(), "deepseek-reasoner");
    }

    #[test]
    fn test_chained_usage() {
        // Test the chained usage pattern
        let model = DeepSeekProvider::new(DeepSeekProviderSettings::new().with_api_key("test-key"))
            .chat_model("deepseek-chat");

        assert_eq!(model.provider(), "deepseek.chat");
        assert_eq!(model.model_id(), "deepseek-chat");
    }

    #[test]
    fn test_provider_trait_implementation() {
        let settings = DeepSeekProviderSettings::new().with_api_key("test-key");

        let provider = DeepSeekProvider::new(settings);

        // Use Provider trait method via trait object
        let provider_trait: &dyn Provider = &provider;

        // Test language model
        let model = provider_trait.language_model("deepseek-chat").unwrap();
        assert_eq!(model.provider(), "deepseek.chat");
        assert_eq!(model.model_id(), "deepseek-chat");

        // Test unsupported models
        assert!(provider_trait.text_embedding_model("some-model").is_err());
        assert!(provider_trait.image_model("some-model").is_err());
        assert!(provider_trait.transcription_model("some-model").is_err());
        assert!(provider_trait.speech_model("some-model").is_err());
        assert!(provider_trait.reranking_model("some-model").is_err());
    }

    #[test]
    fn test_base_url() {
        let settings = DeepSeekProviderSettings::new()
            .with_api_key("test-key")
            .with_base_url("https://custom.deepseek.com/v1");

        let provider = DeepSeekProvider::new(settings);
        assert_eq!(provider.base_url(), "https://custom.deepseek.com/v1");
    }
}
