use ai_sdk_provider::error::ProviderError;
use ai_sdk_provider::language_model::LanguageModel;
use ai_sdk_provider::provider::Provider;
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
    ) -> Result<Arc<dyn ai_sdk_provider::EmbeddingModel<String>>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "huggingface.text-embedding-not-supported",
        ))
    }

    fn image_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn ai_sdk_provider::ImageModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "huggingface.image-not-supported",
        ))
    }

    fn transcription_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn ai_sdk_provider::TranscriptionModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "huggingface.transcription-not-supported",
        ))
    }

    fn speech_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn ai_sdk_provider::SpeechModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "huggingface.speech-not-supported",
        ))
    }

    fn reranking_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn ai_sdk_provider::RerankingModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "huggingface.reranking-not-supported",
        ))
    }
}

/// Creates a Hugging Face provider.
///
/// # Example
///
/// ```no_run
/// use ai_sdk_huggingface::{create_huggingface, HuggingFaceProviderSettings};
///
/// let provider = create_huggingface(
///     HuggingFaceProviderSettings::new()
///         .with_api_key("your-api-key")
/// );
///
/// let model = provider.responses("meta-llama/Llama-3.1-8B-Instruct");
/// ```
pub fn create_huggingface(settings: HuggingFaceProviderSettings) -> HuggingFaceProvider {
    HuggingFaceProvider::new(settings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_provider() {
        let settings = HuggingFaceProviderSettings::new().with_api_key("test-key");

        let provider = create_huggingface(settings);

        // Test unsupported models
        assert!(provider.text_embedding_model("model").is_err());
        assert!(provider.image_model("model").is_err());
        assert!(provider.transcription_model("model").is_err());
        assert!(provider.speech_model("model").is_err());
        assert!(provider.reranking_model("model").is_err());
    }
}
