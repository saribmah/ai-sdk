use llm_kit_openai_compatible::OpenAICompatibleChatConfig;
use llm_kit_openai_compatible::OpenAICompatibleChatLanguageModel;
use llm_kit_provider::error::ProviderError;
use llm_kit_provider::language_model::LanguageModel;
use llm_kit_provider::provider::Provider;
use std::collections::HashMap;
use std::sync::Arc;

use crate::settings::CerebrasProviderSettings;

/// Cerebras provider implementation.
///
/// Provides methods to create Cerebras language models for chat completion.
/// Cerebras offers high-speed AI model inference powered by Wafer-Scale Engines.
///
/// # Examples
///
/// ```no_run
/// use llm_kit_cerebras::{CerebrasProvider, CerebrasProviderSettings};
///
/// let settings = CerebrasProviderSettings::new("https://api.cerebras.ai/v1")
///     .with_api_key("your-api-key");
///
/// let provider = CerebrasProvider::new(settings);
/// let model = provider.chat_model("llama-3.3-70b");
/// ```
pub struct CerebrasProvider {
    settings: CerebrasProviderSettings,
}

impl CerebrasProvider {
    /// Creates a new Cerebras provider.
    ///
    /// # Arguments
    ///
    /// * `settings` - Configuration settings for the provider
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use llm_kit_cerebras::{CerebrasProvider, CerebrasProviderSettings};
    ///
    /// let settings = CerebrasProviderSettings::new("https://api.cerebras.ai/v1")
    ///     .with_api_key("your-api-key");
    ///
    /// let provider = CerebrasProvider::new(settings);
    /// ```
    pub fn new(settings: CerebrasProviderSettings) -> Self {
        Self { settings }
    }

    /// Creates a language model with the given model ID.
    ///
    /// This is an alias for `chat_model()`.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The model identifier (e.g., "llama-3.3-70b")
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use llm_kit_cerebras::{CerebrasProvider, CerebrasProviderSettings};
    ///
    /// let settings = CerebrasProviderSettings::default()
    ///     .with_api_key("your-api-key");
    /// let provider = CerebrasProvider::new(settings);
    ///
    /// let model = provider.model("llama-3.3-70b");
    /// ```
    pub fn model(&self, model_id: impl Into<String>) -> Arc<dyn LanguageModel> {
        self.chat_model(model_id)
    }

    /// Creates a chat language model with the given model ID.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The model identifier (e.g., "llama-3.3-70b")
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use llm_kit_cerebras::{CerebrasProvider, CerebrasProviderSettings, chat::models};
    ///
    /// let settings = CerebrasProviderSettings::default()
    ///     .with_api_key("your-api-key");
    /// let provider = CerebrasProvider::new(settings);
    ///
    /// // Using model constant
    /// let model = provider.chat_model(models::LLAMA_3_3_70B);
    ///
    /// // Using string
    /// let model2 = provider.chat_model("llama-3.3-70b");
    /// ```
    pub fn chat_model(&self, model_id: impl Into<String>) -> Arc<dyn LanguageModel> {
        let model_id = model_id.into();
        let config = self.create_chat_config();

        Arc::new(OpenAICompatibleChatLanguageModel::new(model_id, config))
    }

    /// Creates the configuration for chat models
    fn create_chat_config(&self) -> OpenAICompatibleChatConfig {
        let api_key = self.settings.api_key.clone();
        let custom_headers = self.settings.headers.clone().unwrap_or_default();
        let base_url = self.settings.base_url.clone();

        OpenAICompatibleChatConfig {
            provider: "cerebras.chat".to_string(),
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
            include_usage: false,
            supports_structured_outputs: true,
            supported_urls: None,
        }
    }

    /// Gets the provider name.
    pub fn name(&self) -> &str {
        "cerebras"
    }

    /// Gets the base URL.
    pub fn base_url(&self) -> &str {
        &self.settings.base_url
    }
}

// Implement the Provider trait for compatibility with the provider interface
impl Provider for CerebrasProvider {
    fn language_model(&self, model_id: &str) -> Result<Arc<dyn LanguageModel>, ProviderError> {
        Ok(self.chat_model(model_id))
    }

    fn text_embedding_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn llm_kit_provider::EmbeddingModel<String>>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "cerebras.embedding-model-not-supported",
        ))
    }

    fn image_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn llm_kit_provider::ImageModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "cerebras.image-model-not-supported",
        ))
    }

    fn transcription_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn llm_kit_provider::TranscriptionModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "cerebras.transcription-model-not-supported",
        ))
    }

    fn speech_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn llm_kit_provider::SpeechModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "cerebras.speech-model-not-supported",
        ))
    }

    fn reranking_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn llm_kit_provider::RerankingModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "cerebras.reranking-model-not-supported",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_provider() {
        let settings =
            CerebrasProviderSettings::new("https://api.cerebras.ai/v1").with_api_key("test-key");

        let provider = CerebrasProvider::new(settings);
        let model = provider.chat_model("llama-3.3-70b");

        assert_eq!(model.provider(), "cerebras.chat");
        assert_eq!(model.model_id(), "llama-3.3-70b");
    }

    #[test]
    fn test_model_alias() {
        let settings = CerebrasProviderSettings::default().with_api_key("test-key");
        let provider = CerebrasProvider::new(settings);

        let model = provider.model("llama-3.3-70b");
        assert_eq!(model.provider(), "cerebras.chat");
        assert_eq!(model.model_id(), "llama-3.3-70b");
    }

    #[test]
    fn test_provider_getters() {
        let settings = CerebrasProviderSettings::new("https://api.cerebras.ai/v1");
        let provider = CerebrasProvider::new(settings);

        assert_eq!(provider.name(), "cerebras");
        assert_eq!(provider.base_url(), "https://api.cerebras.ai/v1");
    }

    #[test]
    fn test_provider_trait_language_model() {
        let settings = CerebrasProviderSettings::default().with_api_key("test-key");
        let provider = CerebrasProvider::new(settings);

        // Use Provider trait method via trait object
        let provider_trait: &dyn Provider = &provider;
        let model = provider_trait.language_model("llama-3.3-70b").unwrap();

        assert_eq!(model.provider(), "cerebras.chat");
        assert_eq!(model.model_id(), "llama-3.3-70b");
    }

    #[test]
    fn test_unsupported_models() {
        let settings = CerebrasProviderSettings::default();
        let provider = CerebrasProvider::new(settings);

        // Use Provider trait to test unsupported models
        let provider_trait: &dyn Provider = &provider;

        assert!(provider_trait.text_embedding_model("test").is_err());
        assert!(provider_trait.image_model("test").is_err());
        assert!(provider_trait.transcription_model("test").is_err());
        assert!(provider_trait.speech_model("test").is_err());
        assert!(provider_trait.reranking_model("test").is_err());
    }
}
