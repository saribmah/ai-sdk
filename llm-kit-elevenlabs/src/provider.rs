use crate::config::ElevenLabsConfig;
use crate::settings::ElevenLabsProviderSettings;
use crate::speech::ElevenLabsSpeechModel;
use crate::transcription::ElevenLabsTranscriptionModel;
use llm_kit_provider::{
    EmbeddingModel, ImageModel, LanguageModel, Provider, ProviderError, RerankingModel,
    SpeechModel, TranscriptionModel,
};
use std::collections::HashMap;
use std::sync::Arc;

/// ElevenLabs provider for speech and transcription models.
#[derive(Clone)]
pub struct ElevenLabsProvider {
    settings: ElevenLabsProviderSettings,
}

impl ElevenLabsProvider {
    /// Create a new ElevenLabs provider with settings.
    pub fn new(settings: ElevenLabsProviderSettings) -> Self {
        Self { settings }
    }

    /// Get headers for API requests.
    fn get_headers(&self) -> Result<HashMap<String, String>, String> {
        let api_key = self.settings.get_api_key()?;
        let mut headers = HashMap::new();
        headers.insert("xi-api-key".to_string(), api_key);

        // Add custom headers
        if let Some(ref custom_headers) = self.settings.headers {
            headers.extend(custom_headers.clone());
        }

        Ok(headers)
    }

    /// Create a speech model configuration.
    fn create_speech_config(&self) -> Result<ElevenLabsConfig, String> {
        let headers = self.get_headers()?;
        Ok(ElevenLabsConfig::new(
            "elevenlabs.speech",
            self.settings.get_base_url(),
            headers,
        ))
    }

    /// Create a transcription model configuration.
    fn create_transcription_config(&self) -> Result<ElevenLabsConfig, String> {
        let headers = self.get_headers()?;
        Ok(ElevenLabsConfig::new(
            "elevenlabs.transcription",
            self.settings.get_base_url(),
            headers,
        ))
    }
}

impl Provider for ElevenLabsProvider {
    fn specification_version(&self) -> &str {
        "v3"
    }

    fn language_model(&self, model_id: &str) -> Result<Arc<dyn LanguageModel>, ProviderError> {
        Err(ProviderError::no_such_model_with_message(
            model_id,
            "elevenlabs",
            std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "ElevenLabs does not provide language models",
            ),
        ))
    }

    fn text_embedding_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn EmbeddingModel<String>>, ProviderError> {
        Err(ProviderError::no_such_model_with_message(
            model_id,
            "elevenlabs",
            std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "ElevenLabs does not provide text embedding models",
            ),
        ))
    }

    fn image_model(&self, model_id: &str) -> Result<Arc<dyn ImageModel>, ProviderError> {
        Err(ProviderError::no_such_model_with_message(
            model_id,
            "elevenlabs",
            std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "ElevenLabs does not provide image models",
            ),
        ))
    }

    fn transcription_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn TranscriptionModel>, ProviderError> {
        let config = self
            .create_transcription_config()
            .map_err(ProviderError::load_api_key_error)?;

        let model = ElevenLabsTranscriptionModel::new(model_id.to_string(), config);
        Ok(Arc::new(model))
    }

    fn speech_model(&self, model_id: &str) -> Result<Arc<dyn SpeechModel>, ProviderError> {
        let config = self
            .create_speech_config()
            .map_err(ProviderError::load_api_key_error)?;

        let model = ElevenLabsSpeechModel::new(model_id.to_string(), config);
        Ok(Arc::new(model))
    }

    fn reranking_model(&self, model_id: &str) -> Result<Arc<dyn RerankingModel>, ProviderError> {
        Err(ProviderError::no_such_model_with_message(
            model_id,
            "elevenlabs",
            std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "ElevenLabs does not provide reranking models",
            ),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        let provider = ElevenLabsProvider::new(ElevenLabsProviderSettings::default());
        assert_eq!(provider.specification_version(), "v3");
    }

    #[test]
    fn test_unsupported_models() {
        let provider = ElevenLabsProvider::new(ElevenLabsProviderSettings::default());

        // Language models not supported
        assert!(provider.language_model("test").is_err());

        // Embedding models not supported
        assert!(provider.text_embedding_model("test").is_err());

        // Image models not supported
        assert!(provider.image_model("test").is_err());

        // Reranking models not supported
        assert!(provider.reranking_model("test").is_err());
    }
}
