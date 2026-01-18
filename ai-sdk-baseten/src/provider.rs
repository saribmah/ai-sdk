use llm_kit_openai_compatible::{OpenAICompatibleProvider, OpenAICompatibleProviderSettings};
use llm_kit_provider::EmbeddingModel;
use llm_kit_provider::ImageModel;
use llm_kit_provider::error::ProviderError;
use llm_kit_provider::language_model::LanguageModel;
use llm_kit_provider::provider::Provider;
use std::sync::Arc;

use crate::settings::BasetenProviderSettings;

/// Baseten provider implementation.
///
/// Wraps the OpenAI-compatible provider with Baseten-specific configuration.
pub struct BasetenProvider {
    settings: BasetenProviderSettings,
}

impl BasetenProvider {
    /// Creates a new Baseten provider with the given settings.
    pub fn new(settings: BasetenProviderSettings) -> Self {
        Self { settings }
    }

    /// Creates a language model (alias for chat_model).
    pub fn language_model(&self, model_id: Option<&str>) -> Arc<dyn LanguageModel> {
        self.chat_model(model_id)
    }

    /// Creates a chat language model.
    ///
    /// # Arguments
    ///
    /// * `model_id` - Optional model ID. If not provided, defaults to "chat".
    ///   For Model APIs, use model IDs like "deepseek-ai/DeepSeek-V3-0324".
    ///   For custom models, the model_id is used as a placeholder.
    ///
    /// # Panics
    ///
    /// Panics if a custom model URL uses a `/predict` endpoint (not supported).
    pub fn chat_model(&self, model_id: Option<&str>) -> Arc<dyn LanguageModel> {
        let model_id = model_id.unwrap_or("chat").to_string();
        let custom_url = self.settings.model_url.as_ref();

        if let Some(url) = custom_url {
            // Check if this is a /sync/v1 endpoint (OpenAI-compatible)
            let is_openai_compatible = url.contains("/sync/v1");

            if is_openai_compatible {
                // For /sync/v1 endpoints, use standard OpenAI-compatible format
                let provider = self.create_openai_compatible_provider("chat", Some(url));
                return provider.chat_model(model_id);
            } else if url.contains("/predict") {
                panic!("Not supported. You must use a /sync/v1 endpoint for chat models.");
            }
        }

        // Use default Model APIs
        let provider = self.create_openai_compatible_provider("chat", None);
        provider.chat_model(model_id)
    }

    /// Creates a text embedding model.
    ///
    /// # Arguments
    ///
    /// * `model_id` - Optional model ID. If not provided, defaults to "embeddings".
    ///   The model ID is primarily used for identification.
    ///
    /// # Panics
    ///
    /// Panics if no model URL is provided (embeddings require a custom model URL).
    /// Panics if the model URL uses a `/predict` endpoint (not supported).
    pub fn text_embedding_model(&self, model_id: Option<&str>) -> Arc<dyn EmbeddingModel<String>> {
        let model_id = model_id.unwrap_or("embeddings").to_string();
        let custom_url = self.settings.model_url.as_ref();

        if custom_url.is_none() {
            panic!(
                "No model URL provided for embeddings. Please set modelURL option for embeddings."
            );
        }

        let url = custom_url.unwrap();

        // Check if this is a /sync or /sync/v1 endpoint (OpenAI-compatible)
        let is_openai_compatible = url.contains("/sync");

        if is_openai_compatible {
            // For embeddings with /sync URLs (but not /sync/v1), we need to add /v1
            let adjusted_url = if url.contains("/sync") && !url.contains("/sync/v1") {
                format!("{}/v1", url)
            } else {
                url.clone()
            };

            let provider = self.create_openai_compatible_provider("embedding", Some(&adjusted_url));
            provider.text_embedding_model(model_id)
        } else {
            panic!("Not supported. You must use a /sync or /sync/v1 endpoint for embeddings.");
        }
    }

    /// Helper to create an OpenAI-compatible provider with Baseten configuration.
    fn create_openai_compatible_provider(
        &self,
        model_type: &str,
        custom_url: Option<&str>,
    ) -> OpenAICompatibleProvider {
        let base_url = custom_url
            .map(|s| s.to_string())
            .unwrap_or_else(|| self.settings.base_url.clone());

        let mut compat_settings =
            OpenAICompatibleProviderSettings::new(base_url, format!("baseten.{}", model_type));

        // Set API key
        if let Some(ref api_key) = self.settings.api_key {
            compat_settings = compat_settings.with_api_key(api_key);
        }

        // Set custom headers
        if let Some(ref headers) = self.settings.headers {
            compat_settings = compat_settings.with_headers(headers.clone());
        }

        OpenAICompatibleProvider::new(compat_settings)
    }

    /// Gets the base URL for Model APIs.
    pub fn base_url(&self) -> &str {
        &self.settings.base_url
    }

    /// Gets the custom model URL, if set.
    pub fn model_url(&self) -> Option<&str> {
        self.settings.model_url.as_deref()
    }
}

// Implement the Provider trait
impl Provider for BasetenProvider {
    fn language_model(&self, model_id: &str) -> Result<Arc<dyn LanguageModel>, ProviderError> {
        Ok(self.chat_model(Some(model_id)))
    }

    fn text_embedding_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn EmbeddingModel<String>>, ProviderError> {
        Ok(self.text_embedding_model(Some(model_id)))
    }

    fn image_model(&self, model_id: &str) -> Result<Arc<dyn ImageModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "baseten.image-model-not-supported",
        ))
    }

    fn transcription_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn llm_kit_provider::TranscriptionModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "baseten.transcription-model-not-supported",
        ))
    }

    fn speech_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn llm_kit_provider::SpeechModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "baseten.speech-model-not-supported",
        ))
    }

    fn reranking_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn llm_kit_provider::RerankingModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "baseten.reranking-model-not-supported",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::BasetenProviderSettings;

    #[test]
    fn test_create_chat_model_with_model_apis() {
        let settings = BasetenProviderSettings::new().with_api_key("test-key");
        let provider = BasetenProvider::new(settings);
        let model = provider.chat_model(Some("deepseek-ai/DeepSeek-V3-0324"));

        assert_eq!(model.provider(), "baseten.chat.chat");
        assert_eq!(model.model_id(), "deepseek-ai/DeepSeek-V3-0324");
    }

    #[test]
    fn test_create_chat_model_default_id() {
        let settings = BasetenProviderSettings::new().with_api_key("test-key");
        let provider = BasetenProvider::new(settings);
        let model = provider.chat_model(None);

        assert_eq!(model.provider(), "baseten.chat.chat");
        assert_eq!(model.model_id(), "chat");
    }

    #[test]
    fn test_create_chat_model_with_custom_url() {
        let settings = BasetenProviderSettings::new()
            .with_api_key("test-key")
            .with_model_url("https://model-abc.api.baseten.co/sync/v1");
        let provider = BasetenProvider::new(settings);
        let model = provider.chat_model(Some("custom-model"));

        assert_eq!(model.provider(), "baseten.chat.chat");
        assert_eq!(model.model_id(), "custom-model");
    }

    #[test]
    #[should_panic(expected = "You must use a /sync/v1 endpoint for chat models")]
    fn test_chat_model_rejects_predict_endpoint() {
        let settings = BasetenProviderSettings::new()
            .with_api_key("test-key")
            .with_model_url("https://model-abc.api.baseten.co/predict");
        let provider = BasetenProvider::new(settings);
        provider.chat_model(Some("model"));
    }

    #[test]
    #[should_panic(expected = "No model URL provided for embeddings")]
    fn test_embedding_model_requires_url() {
        let settings = BasetenProviderSettings::new().with_api_key("test-key");
        let provider = BasetenProvider::new(settings);
        provider.text_embedding_model(None);
    }

    #[test]
    fn test_embedding_model_with_sync_url() {
        let settings = BasetenProviderSettings::new()
            .with_api_key("test-key")
            .with_model_url("https://model-xyz.api.baseten.co/sync");
        let provider = BasetenProvider::new(settings);
        let model = provider.text_embedding_model(Some("embeddings"));

        assert_eq!(model.provider(), "baseten.embedding.embedding");
        assert_eq!(model.model_id(), "embeddings");
    }

    #[test]
    fn test_embedding_model_with_sync_v1_url() {
        let settings = BasetenProviderSettings::new()
            .with_api_key("test-key")
            .with_model_url("https://model-xyz.api.baseten.co/sync/v1");
        let provider = BasetenProvider::new(settings);
        let model = provider.text_embedding_model(None);

        assert_eq!(model.provider(), "baseten.embedding.embedding");
        assert_eq!(model.model_id(), "embeddings");
    }

    #[test]
    #[should_panic(expected = "You must use a /sync or /sync/v1 endpoint for embeddings")]
    fn test_embedding_model_rejects_predict_endpoint() {
        let settings = BasetenProviderSettings::new()
            .with_api_key("test-key")
            .with_model_url("https://model-xyz.api.baseten.co/predict");
        let provider = BasetenProvider::new(settings);
        provider.text_embedding_model(Some("model"));
    }

    #[test]
    fn test_provider_trait_language_model() {
        let settings = BasetenProviderSettings::new().with_api_key("test-key");
        let provider = BasetenProvider::new(settings);
        let provider_trait: &dyn Provider = &provider;

        let model = provider_trait
            .language_model("deepseek-ai/DeepSeek-V3-0324")
            .unwrap();
        assert_eq!(model.provider(), "baseten.chat.chat");
        assert_eq!(model.model_id(), "deepseek-ai/DeepSeek-V3-0324");
    }

    #[test]
    fn test_provider_trait_unsupported_models() {
        let settings = BasetenProviderSettings::new().with_api_key("test-key");
        let provider = BasetenProvider::new(settings);
        let provider_trait: &dyn Provider = &provider;

        assert!(provider_trait.image_model("model").is_err());
        assert!(provider_trait.transcription_model("model").is_err());
        assert!(provider_trait.speech_model("model").is_err());
        assert!(provider_trait.reranking_model("model").is_err());
    }

    #[test]
    fn test_base_url() {
        let settings = BasetenProviderSettings::new();
        let provider = BasetenProvider::new(settings);
        assert_eq!(provider.base_url(), "https://inference.baseten.co/v1");
    }

    #[test]
    fn test_model_url() {
        let settings = BasetenProviderSettings::new()
            .with_model_url("https://model-abc.api.baseten.co/sync/v1");
        let provider = BasetenProvider::new(settings);
        assert_eq!(
            provider.model_url(),
            Some("https://model-abc.api.baseten.co/sync/v1")
        );
    }
}
