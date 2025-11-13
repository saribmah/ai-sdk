use ai_sdk_provider::error::ProviderError;
use ai_sdk_provider::language_model::LanguageModel;
use ai_sdk_provider::provider::Provider;
use std::collections::HashMap;
use std::sync::Arc;

use crate::chat::language_model::GroqChatLanguageModel;
use crate::settings::GroqProviderSettings;
use crate::transcription::model::{GroqTranscriptionConfig, GroqTranscriptionModel};
use ai_sdk_openai_compatible::OpenAICompatibleChatConfig;

/// Groq provider implementation.
///
/// Provides methods to create language models for Groq's chat models.
pub struct GroqProvider {
    settings: GroqProviderSettings,
}

impl GroqProvider {
    /// Creates a new Groq provider.
    pub fn new(settings: GroqProviderSettings) -> Self {
        Self { settings }
    }

    /// Returns the provider name.
    pub fn name(&self) -> &str {
        "groq"
    }

    /// Creates a chat language model with the given model ID.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ai_sdk_groq::GroqClient;
    ///
    /// let provider = GroqClient::new()
    ///     .api_key("your-api-key")
    ///     .build();
    ///
    /// let model = provider.chat_model("llama-3.1-8b-instant");
    /// ```
    pub fn chat_model(&self, model_id: impl Into<String>) -> Arc<dyn LanguageModel> {
        let model_id = model_id.into();
        let config = self.create_chat_config();

        Arc::new(GroqChatLanguageModel::new(model_id, config))
    }

    /// Alias for `chat_model()`.
    pub fn model(&self, model_id: impl Into<String>) -> Arc<dyn LanguageModel> {
        self.chat_model(model_id)
    }

    /// Creates a transcription model with the given model ID.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ai_sdk_groq::GroqClient;
    ///
    /// let provider = GroqClient::new()
    ///     .api_key("your-api-key")
    ///     .build();
    ///
    /// let model = provider.transcription_model("whisper-large-v3");
    /// ```
    pub fn transcription_model(
        &self,
        model_id: impl Into<String>,
    ) -> Arc<dyn ai_sdk_provider::TranscriptionModel> {
        let model_id = model_id.into();
        let config = self.create_transcription_config();

        Arc::new(GroqTranscriptionModel::new(model_id, config))
    }

    /// Creates the configuration for chat models.
    pub(crate) fn create_chat_config(&self) -> OpenAICompatibleChatConfig {
        let api_key = self.settings.api_key.clone();
        let custom_headers = self.settings.headers.clone().unwrap_or_default();
        let base_url = self.settings.base_url.clone();

        OpenAICompatibleChatConfig {
            provider: "groq.chat".to_string(),
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
            include_usage: true,               // Groq includes usage by default
            supports_structured_outputs: true, // Groq supports structured outputs
            supported_urls: Some(|| {
                let mut map = std::collections::HashMap::new();
                // Groq supports image URLs matching this pattern
                map.insert(
                    "image/*".to_string(),
                    vec![regex::Regex::new(r"^https?://.*$").unwrap()],
                );
                map
            }),
        }
    }

    /// Creates the configuration for transcription models.
    fn create_transcription_config(&self) -> GroqTranscriptionConfig {
        let api_key = self.settings.api_key.clone();
        let custom_headers = self.settings.headers.clone().unwrap_or_default();
        let base_url = self.settings.base_url.clone();

        GroqTranscriptionConfig {
            provider: "groq.transcription".to_string(),
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

// Implement the Provider trait for compatibility with the provider interface
impl Provider for GroqProvider {
    fn language_model(&self, model_id: &str) -> Result<Arc<dyn LanguageModel>, ProviderError> {
        Ok(self.chat_model(model_id))
    }

    fn text_embedding_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn ai_sdk_provider::EmbeddingModel<String>>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "groq.embedding-model-not-supported".to_string(),
        ))
    }

    fn image_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn ai_sdk_provider::ImageModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "groq.image-model-not-supported".to_string(),
        ))
    }

    fn transcription_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn ai_sdk_provider::TranscriptionModel>, ProviderError> {
        Ok(self.transcription_model(model_id))
    }

    fn speech_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn ai_sdk_provider::SpeechModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "groq.speech-model-not-supported".to_string(),
        ))
    }

    fn reranking_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn ai_sdk_provider::RerankingModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "groq.reranking-model-not-supported".to_string(),
        ))
    }
}

/// Creates a Groq provider.
///
/// This function creates a provider that can be used with Groq's chat models.
///
/// # Arguments
///
/// * `settings` - Configuration settings for the provider
///
/// # Returns
///
/// A `GroqProvider` instance.
///
/// # Examples
///
/// ```no_run
/// use ai_sdk_groq::{create_groq, GroqProviderSettings};
///
/// // Create a Groq provider and get a chat model
/// let provider = create_groq(
///     GroqProviderSettings::new()
///         .with_api_key("your-api-key")
/// );
///
/// let chat_model = provider.chat_model("llama-3.1-8b-instant");
///
/// // Or chain it directly
/// let model = create_groq(
///     GroqProviderSettings::new()
///         .with_api_key("your-api-key")
/// )
/// .chat_model("llama-3.3-70b-versatile");
/// ```
pub fn create_groq(settings: GroqProviderSettings) -> GroqProvider {
    GroqProvider::new(settings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_groq() {
        let settings = GroqProviderSettings::new().with_api_key("test-key");

        let provider = create_groq(settings);
        let model = provider.chat_model("llama-3.1-8b-instant");

        assert_eq!(model.provider(), "groq.chat");
        assert_eq!(model.model_id(), "llama-3.1-8b-instant");
    }

    #[test]
    fn test_chat_model() {
        let settings = GroqProviderSettings::new().with_api_key("test-key");

        let provider = create_groq(settings);
        let model = provider.chat_model("llama-3.3-70b-versatile");

        assert_eq!(model.provider(), "groq.chat");
        assert_eq!(model.model_id(), "llama-3.3-70b-versatile");
    }

    #[test]
    fn test_chained_usage() {
        // Test the chained usage pattern
        let model = create_groq(GroqProviderSettings::new().with_api_key("test-key"))
            .chat_model("gemma2-9b-it");

        assert_eq!(model.provider(), "groq.chat");
        assert_eq!(model.model_id(), "gemma2-9b-it");
    }

    #[test]
    fn test_provider_trait_implementation() {
        let settings = GroqProviderSettings::new().with_api_key("test-key");

        let provider = create_groq(settings);

        // Use Provider trait method via trait object
        let provider_trait: &dyn Provider = &provider;

        // Test language model
        let model = provider_trait
            .language_model("llama-3.1-8b-instant")
            .unwrap();
        assert_eq!(model.provider(), "groq.chat");
        assert_eq!(model.model_id(), "llama-3.1-8b-instant");

        // Test unsupported models
        assert!(provider_trait.text_embedding_model("some-model").is_err());
        assert!(provider_trait.image_model("some-model").is_err());
        assert!(provider_trait.speech_model("some-model").is_err());
        assert!(provider_trait.reranking_model("some-model").is_err());
    }

    #[test]
    fn test_base_url() {
        let settings = GroqProviderSettings::new()
            .with_api_key("test-key")
            .with_base_url("https://custom.groq.com/openai/v1");

        let provider = create_groq(settings);
        assert_eq!(provider.base_url(), "https://custom.groq.com/openai/v1");
    }

    #[test]
    fn test_default_base_url() {
        let settings = GroqProviderSettings::new().with_api_key("test-key");

        let provider = create_groq(settings);
        assert_eq!(provider.base_url(), "https://api.groq.com/openai/v1");
    }
}
