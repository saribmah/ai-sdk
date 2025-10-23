use thiserror::Error;

/// Errors that can occur when working with providers.
#[derive(Debug, Error)]
pub enum ProviderError {
    /// The requested model was not found in this provider.
    #[error("No such model: {model_id} (provider: {provider_id})")]
    NoSuchModel {
        /// The ID of the model that was not found
        model_id: String,
        /// The ID of the provider
        provider_id: String,
        /// Optional message with more details
        #[source]
        message: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// An error occurred while initializing or using the model.
    #[error("Model error: {message}")]
    ModelError {
        /// The error message
        message: String,
        /// Optional source error
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

impl ProviderError {
    /// Create a new NoSuchModel error
    pub fn no_such_model(model_id: impl Into<String>, provider_id: impl Into<String>) -> Self {
        Self::NoSuchModel {
            model_id: model_id.into(),
            provider_id: provider_id.into(),
            message: None,
        }
    }

    /// Create a new NoSuchModel error with a message
    pub fn no_such_model_with_message(
        model_id: impl Into<String>,
        provider_id: impl Into<String>,
        message: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::NoSuchModel {
            model_id: model_id.into(),
            provider_id: provider_id.into(),
            message: Some(Box::new(message)),
        }
    }

    /// Create a new ModelError
    pub fn model_error(message: impl Into<String>) -> Self {
        Self::ModelError {
            message: message.into(),
            source: None,
        }
    }

    /// Create a new ModelError with a source error
    pub fn model_error_with_source(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::ModelError {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }
}
