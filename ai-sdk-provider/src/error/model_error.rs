use super::ProviderError;

/// Builder for constructing ModelError with a fluent interface
///
/// # Examples
///
/// ```
/// use llm_kit_provider::error::{ModelErrorBuilder, ProviderError};
/// use std::io;
///
/// let source_error = io::Error::new(io::ErrorKind::Other, "Connection failed");
///
/// let error = ModelErrorBuilder::new("Failed to initialize model")
///     .source(source_error)
///     .build();
///
/// match error {
///     ProviderError::ModelError { message, .. } => {
///         assert_eq!(message, "Failed to initialize model");
///     }
///     _ => panic!("Expected ModelError"),
/// }
/// ```
#[derive(Debug, Default)]
pub struct ModelErrorBuilder {
    message: String,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl ModelErrorBuilder {
    /// Create a new ModelErrorBuilder with required fields
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    /// Set the source error
    pub fn source(mut self, source: impl std::error::Error + Send + Sync + 'static) -> Self {
        self.source = Some(Box::new(source));
        self
    }

    /// Build the ProviderError
    pub fn build(self) -> ProviderError {
        ProviderError::ModelError {
            message: self.message,
            source: self.source,
        }
    }
}

impl ProviderError {
    /// Create a new ModelError
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_provider::error::ProviderError;
    ///
    /// let error = ProviderError::model_error("Failed to initialize model");
    ///
    /// match error {
    ///     ProviderError::ModelError { message, .. } => {
    ///         assert_eq!(message, "Failed to initialize model");
    ///     }
    ///     _ => panic!("Expected ModelError"),
    /// }
    /// ```
    pub fn model_error(message: impl Into<String>) -> Self {
        Self::ModelError {
            message: message.into(),
            source: None,
        }
    }

    /// Create a new ModelError with a source error
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_provider::error::ProviderError;
    /// use std::io;
    ///
    /// let source_error = io::Error::new(io::ErrorKind::Other, "Connection refused");
    /// let error = ProviderError::model_error_with_source(
    ///     "Failed to connect to model",
    ///     source_error,
    /// );
    ///
    /// match error {
    ///     ProviderError::ModelError { message, .. } => {
    ///         assert_eq!(message, "Failed to connect to model");
    ///     }
    ///     _ => panic!("Expected ModelError"),
    /// }
    /// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_error_basic() {
        let error = ProviderError::model_error("Test error");

        match error {
            ProviderError::ModelError { message, source } => {
                assert_eq!(message, "Test error");
                assert!(source.is_none());
            }
            _ => panic!("Expected ModelError"),
        }
    }

    #[test]
    fn test_model_error_with_source() {
        use std::io;

        let source = io::Error::other("Connection failed");
        let error = ProviderError::model_error_with_source("Failed to connect", source);

        match error {
            ProviderError::ModelError { message, source } => {
                assert_eq!(message, "Failed to connect");
                assert!(source.is_some());
            }
            _ => panic!("Expected ModelError"),
        }
    }

    #[test]
    fn test_model_error_builder() {
        use std::io;

        let source = io::Error::other("Network error");
        let error = ModelErrorBuilder::new("Failed to initialize")
            .source(source)
            .build();

        match error {
            ProviderError::ModelError { message, source } => {
                assert_eq!(message, "Failed to initialize");
                assert!(source.is_some());
            }
            _ => panic!("Expected ModelError"),
        }
    }

    #[test]
    fn test_model_error_builder_without_source() {
        let error = ModelErrorBuilder::new("Simple error").build();

        match error {
            ProviderError::ModelError { message, source } => {
                assert_eq!(message, "Simple error");
                assert!(source.is_none());
            }
            _ => panic!("Expected ModelError"),
        }
    }

    #[test]
    fn test_model_error_not_retryable() {
        let error = ProviderError::model_error("Model error");
        assert!(!error.is_retryable());
        assert!(error.status_code().is_none());
        assert!(error.url().is_none());
    }

    #[test]
    fn test_error_display() {
        let error = ProviderError::model_error("Test error");
        let display = format!("{}", error);
        assert!(display.contains("Model error"));
        assert!(display.contains("Test error"));
    }

    #[test]
    fn test_error_with_complex_source() {
        use std::io;

        let inner_error = io::Error::new(io::ErrorKind::TimedOut, "Connection timed out");
        let error =
            ProviderError::model_error_with_source("Model initialization failed", inner_error);

        match error {
            ProviderError::ModelError { message, source } => {
                assert_eq!(message, "Model initialization failed");
                assert!(source.is_some());
                // The source should contain our IO error information
                let source_msg = source.unwrap().to_string();
                assert!(source_msg.contains("Connection timed out"));
            }
            _ => panic!("Expected ModelError"),
        }
    }

    #[test]
    fn test_builder_method_chaining() {
        use std::io;

        let error1 = ModelErrorBuilder::new("Error 1").build();
        let error2 = ModelErrorBuilder::new("Error 2")
            .source(io::Error::other("Source"))
            .build();

        // Both should be valid ModelErrors
        assert!(matches!(error1, ProviderError::ModelError { .. }));
        assert!(matches!(error2, ProviderError::ModelError { .. }));
    }
}
