use llm_kit_provider::error::ProviderError;
use llm_kit_provider::language_model::LanguageModel;
use llm_kit_provider::provider::Provider;
use std::collections::HashMap;
use std::sync::Arc;

use crate::client::HuggingFaceClientConfig;
use crate::responses::HuggingFaceResponsesLanguageModel;
use crate::settings::HuggingFaceProviderSettings;

/// Hugging Face provider implementation.
pub struct HuggingFaceProvider {
    settings: HuggingFaceProviderSettings,
}

impl HuggingFaceProvider {
    /// Creates a new Hugging Face provider.
    pub fn new(settings: HuggingFaceProviderSettings) -> Self {
        Self { settings }
    }

    /// Creates a Hugging Face Responses API language model.
    pub fn responses(&self, model_id: impl Into<String>) -> Arc<dyn LanguageModel> {
        let model_id = model_id.into();
        let config = self.create_client_config();

        Arc::new(HuggingFaceResponsesLanguageModel::new(model_id, config))
    }

    /// Alias for `responses()` - creates a language model.
    pub fn language_model(&self, model_id: impl Into<String>) -> Arc<dyn LanguageModel> {
        self.responses(model_id)
    }

    /// Returns the base URL for the provider.
    pub fn base_url(&self) -> &str {
        &self.settings.base_url
    }

    /// Creates the client configuration for models.
    fn create_client_config(&self) -> HuggingFaceClientConfig {
        let api_key = self.settings.api_key.clone();
        let custom_headers = self.settings.headers.clone().unwrap_or_default();
        let base_url = self.settings.base_url.clone();

        HuggingFaceClientConfig::new(
            "huggingface.responses",
            Box::new(move |_model_id: &str, path: &str| format!("{}{}", base_url, path)),
            Box::new(move || {
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
        )
    }
}

impl Provider for HuggingFaceProvider {
    fn language_model(&self, model_id: &str) -> Result<Arc<dyn LanguageModel>, ProviderError> {
        Ok(self.responses(model_id))
    }

    fn text_embedding_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn llm_kit_provider::EmbeddingModel<String>>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "huggingface.text-embedding-not-supported",
        ))
    }

    fn image_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn llm_kit_provider::ImageModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "huggingface.image-not-supported",
        ))
    }

    fn transcription_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn llm_kit_provider::TranscriptionModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "huggingface.transcription-not-supported",
        ))
    }

    fn speech_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn llm_kit_provider::SpeechModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "huggingface.speech-not-supported",
        ))
    }

    fn reranking_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn llm_kit_provider::RerankingModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "huggingface.reranking-not-supported",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_provider() {
        let settings = HuggingFaceProviderSettings::new().with_api_key("test-key");
        let provider = HuggingFaceProvider::new(settings);

        // Test unsupported models
        assert!(provider.text_embedding_model("model").is_err());
        assert!(provider.image_model("model").is_err());
        assert!(provider.transcription_model("model").is_err());
        assert!(provider.speech_model("model").is_err());
        assert!(provider.reranking_model("model").is_err());
    }

    #[test]
    fn test_base_url_getter() {
        let settings = HuggingFaceProviderSettings::new()
            .with_api_key("test-key")
            .with_base_url("https://custom.api.com/v1");
        let provider = HuggingFaceProvider::new(settings);

        assert_eq!(provider.base_url(), "https://custom.api.com/v1");
    }
}
