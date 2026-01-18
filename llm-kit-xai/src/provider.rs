use llm_kit_openai_compatible::OpenAICompatibleImageModel;
use llm_kit_provider::error::ProviderError;
use llm_kit_provider::language_model::LanguageModel;
use llm_kit_provider::provider::Provider;
use llm_kit_provider::{EmbeddingModel, ImageModel};
use std::collections::HashMap;
use std::sync::Arc;

use crate::chat::language_model::XaiChatLanguageModel;
use crate::settings::XaiProviderSettings;

/// xAI provider implementation.
///
/// Provides methods to create different types of language models for xAI's Grok models.
pub struct XaiProvider {
    settings: XaiProviderSettings,
}

impl XaiProvider {
    /// Creates a new xAI provider.
    pub fn new(settings: XaiProviderSettings) -> Self {
        Self { settings }
    }

    /// Returns the provider name.
    pub fn name(&self) -> &str {
        "xai"
    }

    /// Creates a chat language model with the given model ID.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use llm_kit_xai::XaiClient;
    ///
    /// let provider = XaiClient::new()
    ///     .api_key("your-api-key")
    ///     .build();
    ///
    /// let model = provider.chat_model("grok-4");
    /// ```
    pub fn chat_model(&self, model_id: impl Into<String>) -> Arc<dyn LanguageModel> {
        let model_id = model_id.into();
        let config = self.create_chat_config();

        Arc::new(XaiChatLanguageModel::new(model_id, config))
    }

    /// Alias for `chat_model()`.
    pub fn model(&self, model_id: impl Into<String>) -> Arc<dyn LanguageModel> {
        self.chat_model(model_id)
    }

    /// Creates an image model with the given model ID.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use llm_kit_xai::XaiClient;
    ///
    /// let provider = XaiClient::new()
    ///     .api_key("your-api-key")
    ///     .build();
    ///
    /// let model = provider.image_model("grok-2-image");
    /// ```
    pub fn image_model(&self, model_id: impl Into<String>) -> Arc<dyn ImageModel> {
        let model_id = model_id.into();
        let config = self.create_image_config();

        Arc::new(OpenAICompatibleImageModel::new(model_id, config))
    }

    /// Creates the configuration for chat models.
    fn create_chat_config(&self) -> XaiChatConfig {
        let api_key = self.settings.api_key.clone();
        let custom_headers = self.settings.headers.clone().unwrap_or_default();
        let base_url = self.settings.base_url.clone();

        XaiChatConfig {
            provider: "xai.chat".to_string(),
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

    /// Creates the configuration for image models.
    fn create_image_config(&self) -> llm_kit_openai_compatible::OpenAICompatibleImageModelConfig {
        let api_key = self.settings.api_key.clone();
        let custom_headers = self.settings.headers.clone().unwrap_or_default();
        let base_url = self.settings.base_url.clone();

        llm_kit_openai_compatible::OpenAICompatibleImageModelConfig {
            provider: "xai.image".to_string(),
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
            url: Box::new(move |_model_id: &str, path: &str| format!("{}{}", base_url, path)),
        }
    }

    /// Gets the provider base URL.
    pub fn base_url(&self) -> &str {
        &self.settings.base_url
    }
}

/// Configuration for xAI chat models.
pub struct XaiChatConfig {
    pub provider: String,
    pub base_url: String,
    pub headers: Box<dyn Fn() -> HashMap<String, String> + Send + Sync>,
}

// Implement the Provider trait for compatibility with the provider interface
impl Provider for XaiProvider {
    fn language_model(&self, model_id: &str) -> Result<Arc<dyn LanguageModel>, ProviderError> {
        Ok(self.chat_model(model_id))
    }

    fn text_embedding_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn EmbeddingModel<String>>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "xai.embedding-model-not-supported".to_string(),
        ))
    }

    fn image_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn llm_kit_provider::ImageModel>, ProviderError> {
        Ok(self.image_model(model_id))
    }

    fn transcription_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn llm_kit_provider::TranscriptionModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "xai.transcription-model-not-supported".to_string(),
        ))
    }

    fn speech_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn llm_kit_provider::SpeechModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "xai.speech-model-not-supported".to_string(),
        ))
    }

    fn reranking_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn llm_kit_provider::RerankingModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "xai.reranking-model-not-supported".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_new() {
        let settings = XaiProviderSettings::new().with_api_key("test-key");

        let provider = XaiProvider::new(settings);
        let model = provider.chat_model("grok-4");

        assert_eq!(model.provider(), "xai.chat");
        assert_eq!(model.model_id(), "grok-4");
    }

    #[test]
    fn test_chat_model() {
        let settings = XaiProviderSettings::new().with_api_key("test-key");

        let provider = XaiProvider::new(settings);
        let model = provider.chat_model("grok-3-fast");

        assert_eq!(model.provider(), "xai.chat");
        assert_eq!(model.model_id(), "grok-3-fast");
    }

    #[test]
    fn test_image_model() {
        let settings = XaiProviderSettings::new().with_api_key("test-key");

        let provider = XaiProvider::new(settings);
        let model = provider.image_model("grok-2-image");

        assert_eq!(model.provider(), "xai.image");
        assert_eq!(model.model_id(), "grok-2-image");
    }

    #[test]
    fn test_chained_usage() {
        // Test the chained usage pattern
        let model = XaiProvider::new(XaiProviderSettings::new().with_api_key("test-key"))
            .chat_model("grok-4-fast-reasoning");

        assert_eq!(model.provider(), "xai.chat");
        assert_eq!(model.model_id(), "grok-4-fast-reasoning");
    }

    #[test]
    fn test_provider_trait_implementation() {
        let settings = XaiProviderSettings::new().with_api_key("test-key");

        let provider = XaiProvider::new(settings);

        // Use Provider trait method via trait object
        let provider_trait: &dyn Provider = &provider;

        // Test language model
        let model = provider_trait.language_model("grok-4").unwrap();
        assert_eq!(model.provider(), "xai.chat");
        assert_eq!(model.model_id(), "grok-4");

        // Test image model
        let image_model = provider_trait.image_model("grok-2-image").unwrap();
        assert_eq!(image_model.provider(), "xai.image");
        assert_eq!(image_model.model_id(), "grok-2-image");

        // Test unsupported models
        assert!(provider_trait.text_embedding_model("some-model").is_err());
        assert!(provider_trait.transcription_model("some-model").is_err());
        assert!(provider_trait.speech_model("some-model").is_err());
        assert!(provider_trait.reranking_model("some-model").is_err());
    }

    #[test]
    fn test_base_url() {
        let settings = XaiProviderSettings::new()
            .with_api_key("test-key")
            .with_base_url("https://custom.x.ai/v1");

        let provider = XaiProvider::new(settings);
        assert_eq!(provider.base_url(), "https://custom.x.ai/v1");
    }
}
