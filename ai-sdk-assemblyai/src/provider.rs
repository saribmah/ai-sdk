use ai_sdk_provider::error::ProviderError;
use ai_sdk_provider::provider::Provider;
use ai_sdk_provider::{
    EmbeddingModel, ImageModel, LanguageModel, RerankingModel, SpeechModel, TranscriptionModel,
};
use std::sync::Arc;

use crate::settings::AssemblyAIProviderSettings;
use crate::transcription::model::{AssemblyAITranscriptionConfig, AssemblyAITranscriptionModel};
use crate::transcription::AssemblyAITranscriptionModelId;

/// AssemblyAI provider implementation.
///
/// Provides methods to create transcription models.
pub struct AssemblyAIProvider {
    settings: AssemblyAIProviderSettings,
}

impl AssemblyAIProvider {
    /// Creates a new AssemblyAI provider.
    pub fn new(settings: AssemblyAIProviderSettings) -> Self {
        Self { settings }
    }

    /// Creates a transcription model with the given model ID.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The model ID ("best" or "nano")
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ai_sdk_assemblyai::{AssemblyAIClient, AssemblyAIProvider};
    ///
    /// let provider = AssemblyAIClient::new()
    ///     .api_key("your-api-key")
    ///     .build();
    ///
    /// let model = provider.transcription_model("best");
    /// ```
    pub fn transcription_model(
        &self,
        model_id: impl Into<AssemblyAITranscriptionModelId>,
    ) -> Arc<dyn TranscriptionModel> {
        let model_id = model_id.into();
        let config = self.create_transcription_config();

        Arc::new(AssemblyAITranscriptionModel::new(model_id, config))
    }

    /// Creates the configuration for transcription models.
    fn create_transcription_config(&self) -> AssemblyAITranscriptionConfig {
        AssemblyAITranscriptionConfig {
            provider: "assemblyai.transcription".to_string(),
            base_url: self.settings.base_url.clone(),
            api_key: self.settings.api_key.clone(),
            headers: self.settings.headers.clone().unwrap_or_default(),
            polling_interval_ms: self.settings.polling_interval_ms,
        }
    }

    /// Gets the base URL.
    pub fn base_url(&self) -> &str {
        &self.settings.base_url
    }
}

// Implement the Provider trait
impl Provider for AssemblyAIProvider {
    fn language_model(&self, model_id: &str) -> Result<Arc<dyn LanguageModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "assemblyai.language-model-not-supported".to_string(),
        ))
    }

    fn text_embedding_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn EmbeddingModel<String>>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "assemblyai.embedding-model-not-supported".to_string(),
        ))
    }

    fn image_model(&self, model_id: &str) -> Result<Arc<dyn ImageModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "assemblyai.image-model-not-supported".to_string(),
        ))
    }

    fn transcription_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn TranscriptionModel>, ProviderError> {
        Ok(self.transcription_model(model_id))
    }

    fn speech_model(&self, model_id: &str) -> Result<Arc<dyn SpeechModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "assemblyai.speech-model-not-supported".to_string(),
        ))
    }

    fn reranking_model(&self, model_id: &str) -> Result<Arc<dyn RerankingModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "assemblyai.reranking-model-not-supported".to_string(),
        ))
    }
}

/// Creates an AssemblyAI provider.
///
/// This function creates a provider that can be used with AssemblyAI's transcription API.
///
/// # Arguments
///
/// * `settings` - Configuration settings for the provider
///
/// # Returns
///
/// An `AssemblyAIProvider` instance.
///
/// # Examples
///
/// ```no_run
/// use ai_sdk_assemblyai::{create_assemblyai, AssemblyAIProviderSettings};
///
/// let provider = create_assemblyai(
///     AssemblyAIProviderSettings::new()
///         .with_api_key("your-api-key")
/// );
///
/// let model = provider.transcription_model("best");
/// ```
pub fn create_assemblyai(settings: AssemblyAIProviderSettings) -> AssemblyAIProvider {
    AssemblyAIProvider::new(settings)
}

/// Default AssemblyAI provider instance.
///
/// Uses the `ASSEMBLYAI_API_KEY` environment variable for authentication.
///
/// # Example
///
/// ```no_run
/// use ai_sdk_assemblyai::create_assemblyai;
/// use ai_sdk_assemblyai::AssemblyAIProviderSettings;
///
/// let provider = create_assemblyai(AssemblyAIProviderSettings::new());
/// let model = provider.transcription_model("best");
/// ```
pub fn assemblyai() -> AssemblyAIProvider {
    create_assemblyai(AssemblyAIProviderSettings::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_provider() {
        let settings = AssemblyAIProviderSettings::new().with_api_key("test-key");
        let provider = create_assemblyai(settings);
        assert_eq!(provider.base_url(), "https://api.assemblyai.com");
    }

    #[test]
    fn test_transcription_model() {
        let settings = AssemblyAIProviderSettings::new().with_api_key("test-key");
        let provider = create_assemblyai(settings);
        let model = provider.transcription_model("best");

        assert_eq!(model.provider(), "assemblyai.transcription");
        assert_eq!(model.model_id(), "best");
    }

    #[test]
    fn test_unsupported_models() {
        let settings = AssemblyAIProviderSettings::new();
        let provider = create_assemblyai(settings);

        assert!(provider.language_model("test").is_err());
        assert!(provider.text_embedding_model("test").is_err());
        assert!(provider.image_model("test").is_err());
        assert!(provider.speech_model("test").is_err());
        assert!(provider.reranking_model("test").is_err());
    }
}
