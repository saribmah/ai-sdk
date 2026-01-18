use crate::embedding_model::EmbeddingModel;
use crate::error::ProviderError;
use crate::image_model::ImageModel;
use crate::language_model::LanguageModel;
use crate::reranking_model::RerankingModel;
use crate::speech_model::SpeechModel;
use crate::transcription_model::TranscriptionModel;
use std::sync::Arc;

/// Provider for language, text embedding, image generation, speech, transcription,
/// and reranking models.
///
/// This trait defines the interface that all AI providers must implement to integrate
/// with the LLM Kit. Providers are responsible for creating and managing model instances.
///
/// # Example
///
/// ```no_run
/// use llm_kit_provider::{Provider, LanguageModel, EmbeddingModel, ImageModel, TranscriptionModel, SpeechModel, RerankingModel, ProviderError};
/// use std::sync::Arc;
///
/// struct MyProvider {
///     provider_id: String,
/// }
///
/// impl Provider for MyProvider {
///     fn specification_version(&self) -> &str {
///         "v3"
///     }
///
///     fn language_model(&self, model_id: &str) -> Result<Arc<dyn LanguageModel>, ProviderError> {
///         // Implementation
///         Err(ProviderError::no_such_model(model_id, &self.provider_id))
///     }
///
///     fn text_embedding_model(&self, model_id: &str) -> Result<Arc<dyn EmbeddingModel<String>>, ProviderError> {
///         Err(ProviderError::no_such_model(model_id, "embedding-not-supported"))
///     }
///
///     fn image_model(&self, model_id: &str) -> Result<Arc<dyn ImageModel>, ProviderError> {
///         Err(ProviderError::no_such_model(model_id, &self.provider_id))
///     }
///
///     fn transcription_model(&self, model_id: &str) -> Result<Arc<dyn TranscriptionModel>, ProviderError> {
///         Err(ProviderError::no_such_model(model_id, &self.provider_id))
///     }
///
///     fn speech_model(&self, model_id: &str) -> Result<Arc<dyn SpeechModel>, ProviderError> {
///         Err(ProviderError::no_such_model(model_id, &self.provider_id))
///     }
///
///     fn reranking_model(&self, model_id: &str) -> Result<Arc<dyn RerankingModel>, ProviderError> {
///         Err(ProviderError::no_such_model(model_id, &self.provider_id))
///     }
/// }
/// ```
#[allow(clippy::result_large_err)]
pub trait Provider: Send + Sync {
    /// The provider must specify which provider interface version it implements.
    /// This will allow us to evolve the provider interface and retain backwards compatibility.
    fn specification_version(&self) -> &str {
        "v3"
    }

    /// Returns the language model with the given id.
    ///
    /// The model id is provider-specific and is used to identify which model
    /// to instantiate and return.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The provider-specific identifier for the model
    ///
    /// # Returns
    ///
    /// * `Ok(Arc<dyn LanguageModel>)` - The language model instance wrapped in Arc
    /// * `Err(ProviderError::NoSuchModel)` - If the model ID is not recognized
    ///
    /// # Errors
    ///
    /// Returns `ProviderError::NoSuchModel` if no model with the given ID exists
    /// in this provider.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use llm_kit_provider::{Provider, LanguageModel, ProviderError, EmbeddingModel, ImageModel, TranscriptionModel, SpeechModel, RerankingModel};
    /// use std::sync::Arc;
    ///
    /// struct MyProvider;
    /// impl Provider for MyProvider {
    ///     fn language_model(&self, model_id: &str) -> Result<Arc<dyn LanguageModel>, ProviderError> {
    ///         unimplemented!()
    ///     }
    ///     fn text_embedding_model(&self, model_id: &str) -> Result<Arc<dyn EmbeddingModel<String>>, ProviderError> {
    ///         unimplemented!()
    ///     }
    ///     fn image_model(&self, model_id: &str) -> Result<Arc<dyn ImageModel>, ProviderError> {
    ///         unimplemented!()
    ///     }
    ///     fn transcription_model(&self, model_id: &str) -> Result<Arc<dyn TranscriptionModel>, ProviderError> {
    ///         unimplemented!()
    ///     }
    ///     fn speech_model(&self, model_id: &str) -> Result<Arc<dyn SpeechModel>, ProviderError> {
    ///         unimplemented!()
    ///     }
    ///     fn reranking_model(&self, model_id: &str) -> Result<Arc<dyn RerankingModel>, ProviderError> {
    ///         unimplemented!()
    ///     }
    /// }
    ///
    /// # fn example() -> Result<(), ProviderError> {
    /// let provider = MyProvider;
    /// let model = provider.language_model("gpt-4")?;
    /// # Ok(())
    /// # }
    /// ```
    fn language_model(&self, model_id: &str) -> Result<Arc<dyn LanguageModel>, ProviderError>;

    /// Returns the text embedding model with the given id.
    ///
    /// The model id is provider-specific and is used to identify which embedding
    /// model to instantiate and return.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The provider-specific identifier for the embedding model
    ///
    /// # Returns
    ///
    /// * `Ok(Arc<dyn EmbeddingModel<String>>)` - The text embedding model instance
    /// * `Err(ProviderError::NoSuchModel)` - If the model ID is not recognized
    ///
    /// # Errors
    ///
    /// Returns `ProviderError::NoSuchModel` if no embedding model with the given ID
    /// exists in this provider.
    fn text_embedding_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn EmbeddingModel<String>>, ProviderError>;

    /// Returns the image model with the given id.
    ///
    /// The model id is provider-specific and is used to identify which image
    /// generation model to instantiate and return.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The provider-specific identifier for the image model
    ///
    /// # Returns
    ///
    /// * `Ok(Arc<dyn ImageModel>)` - The image model instance
    /// * `Err(ProviderError::NoSuchModel)` - If the model ID is not recognized
    ///
    /// # Errors
    ///
    /// Returns `ProviderError::NoSuchModel` if no image model with the given ID
    /// exists in this provider.
    fn image_model(&self, model_id: &str) -> Result<Arc<dyn ImageModel>, ProviderError>;

    /// Returns the transcription model with the given id.
    ///
    /// The model id is provider-specific and is used to identify which
    /// transcription model to instantiate and return.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The provider-specific identifier for the transcription model
    ///
    /// # Returns
    ///
    /// * `Ok(Arc<dyn TranscriptionModel>)` - The transcription model instance
    /// * `Err(ProviderError::NoSuchModel)` - If the model ID is not recognized
    ///
    /// # Errors
    ///
    /// Returns `ProviderError::NoSuchModel` if no transcription model with the given ID
    /// exists in this provider.
    fn transcription_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn TranscriptionModel>, ProviderError>;

    /// Returns the speech model with the given id.
    ///
    /// The model id is provider-specific and is used to identify which
    /// speech synthesis model to instantiate and return.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The provider-specific identifier for the speech model
    ///
    /// # Returns
    ///
    /// * `Ok(Arc<dyn SpeechModel>)` - The speech model instance
    /// * `Err(ProviderError::NoSuchModel)` - If the model ID is not recognized
    ///
    /// # Errors
    ///
    /// Returns `ProviderError::NoSuchModel` if no speech model with the given ID
    /// exists in this provider.
    fn speech_model(&self, model_id: &str) -> Result<Arc<dyn SpeechModel>, ProviderError>;

    /// Returns the reranking model with the given id.
    ///
    /// The model id is provider-specific and is used to identify which
    /// reranking model to instantiate and return.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The provider-specific identifier for the reranking model
    ///
    /// # Returns
    ///
    /// * `Ok(Arc<dyn RerankingModel>)` - The reranking model instance
    /// * `Err(ProviderError::NoSuchModel)` - If the model ID is not recognized
    ///
    /// # Errors
    ///
    /// Returns `ProviderError::NoSuchModel` if no reranking model with the given ID
    /// exists in this provider.
    fn reranking_model(&self, model_id: &str) -> Result<Arc<dyn RerankingModel>, ProviderError>;
}
