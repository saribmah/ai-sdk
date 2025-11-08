use super::ProviderError;

/// Builder for constructing NoSuchModel error with a fluent interface
///
/// # Examples
///
/// ```
/// use ai_sdk_provider::error::{NoSuchModelErrorBuilder, ProviderError};
/// use std::io;
///
/// let source_error = io::Error::new(io::ErrorKind::NotFound, "Model file not found");
///
/// let error = NoSuchModelErrorBuilder::new("gpt-4", "openai")
///     .message(source_error)
///     .build();
///
/// match error {
///     ProviderError::NoSuchModel { model_id, provider_id, .. } => {
///         assert_eq!(model_id, "gpt-4");
///         assert_eq!(provider_id, "openai");
///     }
///     _ => panic!("Expected NoSuchModel error"),
/// }
/// ```
#[derive(Debug, Default)]
pub struct NoSuchModelErrorBuilder {
    model_id: String,
    provider_id: String,
    message: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl NoSuchModelErrorBuilder {
    /// Create a new NoSuchModelErrorBuilder with required fields
    pub fn new(model_id: impl Into<String>, provider_id: impl Into<String>) -> Self {
        Self {
            model_id: model_id.into(),
            provider_id: provider_id.into(),
            message: None,
        }
    }

    /// Set the source error message
    pub fn message(mut self, message: impl std::error::Error + Send + Sync + 'static) -> Self {
        self.message = Some(Box::new(message));
        self
    }

    /// Build the ProviderError
    pub fn build(self) -> ProviderError {
        ProviderError::NoSuchModel {
            model_id: self.model_id,
            provider_id: self.provider_id,
            message: self.message,
        }
    }
}

impl ProviderError {
    /// Create a new NoSuchModel error
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_provider::error::ProviderError;
    ///
    /// let error = ProviderError::no_such_model("gpt-4", "openai");
    ///
    /// match error {
    ///     ProviderError::NoSuchModel { model_id, provider_id, .. } => {
    ///         assert_eq!(model_id, "gpt-4");
    ///         assert_eq!(provider_id, "openai");
    ///     }
    ///     _ => panic!("Expected NoSuchModel error"),
    /// }
    /// ```
    pub fn no_such_model(model_id: impl Into<String>, provider_id: impl Into<String>) -> Self {
        Self::NoSuchModel {
            model_id: model_id.into(),
            provider_id: provider_id.into(),
            message: None,
        }
    }

    /// Create a new NoSuchModel error with a message
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_provider::error::ProviderError;
    /// use std::io;
    ///
    /// let source_error = io::Error::new(io::ErrorKind::NotFound, "Model file not found");
    /// let error = ProviderError::no_such_model_with_message(
    ///     "gpt-4",
    ///     "openai",
    ///     source_error,
    /// );
    ///
    /// match error {
    ///     ProviderError::NoSuchModel { model_id, provider_id, .. } => {
    ///         assert_eq!(model_id, "gpt-4");
    ///         assert_eq!(provider_id, "openai");
    ///     }
    ///     _ => panic!("Expected NoSuchModel error"),
    /// }
    /// ```
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_such_model_basic() {
        let error = ProviderError::no_such_model("gpt-4", "openai");

        match error {
            ProviderError::NoSuchModel {
                model_id,
                provider_id,
                message,
            } => {
                assert_eq!(model_id, "gpt-4");
                assert_eq!(provider_id, "openai");
                assert!(message.is_none());
            }
            _ => panic!("Expected NoSuchModel error"),
        }
    }

    #[test]
    fn test_no_such_model_with_message() {
        use std::io;

        let source = io::Error::new(io::ErrorKind::NotFound, "Not found");
        let error = ProviderError::no_such_model_with_message("gpt-4", "openai", source);

        match error {
            ProviderError::NoSuchModel {
                model_id,
                provider_id,
                message,
            } => {
                assert_eq!(model_id, "gpt-4");
                assert_eq!(provider_id, "openai");
                assert!(message.is_some());
            }
            _ => panic!("Expected NoSuchModel error"),
        }
    }

    #[test]
    fn test_no_such_model_builder() {
        use std::io;

        let source = io::Error::new(io::ErrorKind::NotFound, "Model not found");
        let error = NoSuchModelErrorBuilder::new("gpt-4", "openai")
            .message(source)
            .build();

        match error {
            ProviderError::NoSuchModel {
                model_id,
                provider_id,
                message,
            } => {
                assert_eq!(model_id, "gpt-4");
                assert_eq!(provider_id, "openai");
                assert!(message.is_some());
            }
            _ => panic!("Expected NoSuchModel error"),
        }
    }

    #[test]
    fn test_no_such_model_builder_without_message() {
        let error = NoSuchModelErrorBuilder::new("gpt-4", "openai").build();

        match error {
            ProviderError::NoSuchModel {
                model_id,
                provider_id,
                message,
            } => {
                assert_eq!(model_id, "gpt-4");
                assert_eq!(provider_id, "openai");
                assert!(message.is_none());
            }
            _ => panic!("Expected NoSuchModel error"),
        }
    }

    #[test]
    fn test_no_such_model_not_retryable() {
        let error = ProviderError::no_such_model("gpt-4", "openai");
        assert!(!error.is_retryable());
        assert!(error.status_code().is_none());
        assert!(error.url().is_none());
    }

    #[test]
    fn test_error_display() {
        let error = ProviderError::no_such_model("gpt-4", "openai");
        let display = format!("{}", error);
        assert!(display.contains("No such model"));
        assert!(display.contains("gpt-4"));
        assert!(display.contains("openai"));
    }

    #[test]
    fn test_error_source_chain() {
        use std::io;

        let source = io::Error::new(io::ErrorKind::NotFound, "File not found");
        let error = ProviderError::no_such_model_with_message("gpt-4", "openai", source);

        // Verify the error can be downcast to access the source
        match error {
            ProviderError::NoSuchModel { message, .. } => {
                assert!(message.is_some());
            }
            _ => panic!("Expected NoSuchModel error"),
        }
    }
}
