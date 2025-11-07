use crate::error::ProviderError;
use crate::language_model::LanguageModel;
use std::sync::Arc;

/// Provider for language models, text embedding, and image generation models.
///
/// This trait defines the interface that all AI providers must implement to integrate
/// with the AI SDK. Providers are responsible for creating and managing model instances.
///
/// # Example
///
/// ```ignore
/// use ai_sdk_provider::{Provider, LanguageModel, ProviderError};
///
/// struct MyProvider {
///     provider_id: String,
/// }
///
/// impl Provider for MyProvider {
///     fn language_model(&self, model_id: &str) -> Result<Arc<dyn LanguageModel>, ProviderError> {
///         match model_id {
///             "my-model-1" => Ok(Arc::new(MyModel::new(model_id))),
///             _ => Err(ProviderError::no_such_model(model_id, &self.provider_id)),
///         }
///     }
/// }
/// ```
pub trait Provider: Send + Sync {
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
    /// ```ignore
    /// let provider = MyProvider::new();
    /// let model = provider.language_model("gpt-4")?;
    /// ```
    fn language_model(&self, model_id: &str) -> Result<Arc<dyn LanguageModel>, ProviderError>;

    /*
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
    /// * `Ok(Box<dyn TextEmbeddingModel>)` - The text embedding model instance
    /// * `Err(ProviderError::NoSuchModel)` - If the model ID is not recognized
    ///
    /// # Errors
    ///
    /// Returns `ProviderError::NoSuchModel` if no embedding model with the given ID
    /// exists in this provider.
    ///
    /// # Note
    ///
    /// This method has a default implementation that returns an error, as not all
    /// providers support text embedding models. Override this method to provide
    /// text embedding model support.
    // fn text_embedding_model(&self, model_id: &str) -> Result<Box<dyn std::any::Any>, ProviderError> {
    //     Err(ProviderError::no_such_model(
    //         model_id,
    //         "text-embedding-not-supported",
    //     ))
    // } */
}
