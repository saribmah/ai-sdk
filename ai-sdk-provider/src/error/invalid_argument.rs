use super::ProviderError;

/// Builder for constructing InvalidArgument error with a fluent interface
///
/// # Examples
///
/// ```
/// use ai_sdk_provider::error::{InvalidArgumentErrorBuilder, ProviderError};
///
/// let error = InvalidArgumentErrorBuilder::new("temperature", "Temperature must be between 0 and 2")
///     .build();
///
/// match error {
///     ProviderError::InvalidArgument { argument, message, .. } => {
///         assert_eq!(argument, "temperature");
///         assert_eq!(message, "Temperature must be between 0 and 2");
///     }
///     _ => panic!("Expected InvalidArgument error"),
/// }
/// ```
#[derive(Debug, Default)]
pub struct InvalidArgumentErrorBuilder {
    argument: String,
    message: String,
    cause: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl InvalidArgumentErrorBuilder {
    /// Create a new InvalidArgumentErrorBuilder with required fields
    pub fn new(argument: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            argument: argument.into(),
            message: message.into(),
            cause: None,
        }
    }

    /// Set the cause error
    pub fn cause(mut self, cause: impl std::error::Error + Send + Sync + 'static) -> Self {
        self.cause = Some(Box::new(cause));
        self
    }

    /// Build the ProviderError
    pub fn build(self) -> ProviderError {
        ProviderError::InvalidArgument {
            argument: self.argument,
            message: self.message,
            cause: self.cause,
        }
    }
}

impl ProviderError {
    /// Create a new InvalidArgument error
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_provider::error::ProviderError;
    ///
    /// let error = ProviderError::invalid_argument(
    ///     "max_tokens",
    ///     "max_tokens must be a positive integer"
    /// );
    ///
    /// match error {
    ///     ProviderError::InvalidArgument { argument, message, .. } => {
    ///         assert_eq!(argument, "max_tokens");
    ///         assert_eq!(message, "max_tokens must be a positive integer");
    ///     }
    ///     _ => panic!("Expected InvalidArgument error"),
    /// }
    /// ```
    pub fn invalid_argument(argument: impl Into<String>, message: impl Into<String>) -> Self {
        Self::InvalidArgument {
            argument: argument.into(),
            message: message.into(),
            cause: None,
        }
    }

    /// Create a new InvalidArgument error with a cause
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_provider::error::ProviderError;
    /// use std::io;
    ///
    /// let cause_error = io::Error::new(io::ErrorKind::InvalidInput, "Parse error");
    /// let error = ProviderError::invalid_argument_with_cause(
    ///     "temperature",
    ///     "Failed to parse temperature value",
    ///     cause_error,
    /// );
    ///
    /// match error {
    ///     ProviderError::InvalidArgument { argument, message, .. } => {
    ///         assert_eq!(argument, "temperature");
    ///         assert_eq!(message, "Failed to parse temperature value");
    ///     }
    ///     _ => panic!("Expected InvalidArgument error"),
    /// }
    /// ```
    pub fn invalid_argument_with_cause(
        argument: impl Into<String>,
        message: impl Into<String>,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::InvalidArgument {
            argument: argument.into(),
            message: message.into(),
            cause: Some(Box::new(cause)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_argument_basic() {
        let error = ProviderError::invalid_argument("temperature", "Invalid temperature value");

        match error {
            ProviderError::InvalidArgument {
                argument,
                message,
                cause,
            } => {
                assert_eq!(argument, "temperature");
                assert_eq!(message, "Invalid temperature value");
                assert!(cause.is_none());
            }
            _ => panic!("Expected InvalidArgument error"),
        }
    }

    #[test]
    fn test_invalid_argument_with_cause() {
        use std::io;

        let cause = io::Error::new(io::ErrorKind::InvalidInput, "Parse failed");
        let error = ProviderError::invalid_argument_with_cause(
            "max_tokens",
            "Failed to parse max_tokens",
            cause,
        );

        match error {
            ProviderError::InvalidArgument {
                argument,
                message,
                cause,
            } => {
                assert_eq!(argument, "max_tokens");
                assert_eq!(message, "Failed to parse max_tokens");
                assert!(cause.is_some());
            }
            _ => panic!("Expected InvalidArgument error"),
        }
    }

    #[test]
    fn test_invalid_argument_builder() {
        let error = InvalidArgumentErrorBuilder::new("model", "Model name cannot be empty").build();

        match error {
            ProviderError::InvalidArgument {
                argument,
                message,
                cause,
            } => {
                assert_eq!(argument, "model");
                assert_eq!(message, "Model name cannot be empty");
                assert!(cause.is_none());
            }
            _ => panic!("Expected InvalidArgument error"),
        }
    }

    #[test]
    fn test_invalid_argument_builder_with_cause() {
        use std::io;

        let cause = io::Error::new(io::ErrorKind::InvalidInput, "Invalid format");
        let error = InvalidArgumentErrorBuilder::new("api_key", "Invalid API key format")
            .cause(cause)
            .build();

        match error {
            ProviderError::InvalidArgument {
                argument,
                message,
                cause,
            } => {
                assert_eq!(argument, "api_key");
                assert_eq!(message, "Invalid API key format");
                assert!(cause.is_some());
            }
            _ => panic!("Expected InvalidArgument error"),
        }
    }

    #[test]
    fn test_invalid_argument_not_retryable() {
        let error = ProviderError::invalid_argument("test", "test message");
        assert!(!error.is_retryable());
        assert!(error.status_code().is_none());
        assert!(error.url().is_none());
    }

    #[test]
    fn test_error_display() {
        let error = ProviderError::invalid_argument("temperature", "Must be between 0 and 2");
        let display = format!("{}", error);
        assert!(display.contains("Invalid argument"));
        assert!(display.contains("temperature"));
        assert!(display.contains("Must be between 0 and 2"));
    }

    #[test]
    fn test_builder_method_chaining() {
        use std::io;

        let error1 = InvalidArgumentErrorBuilder::new("arg1", "Error 1").build();
        let error2 = InvalidArgumentErrorBuilder::new("arg2", "Error 2")
            .cause(io::Error::other("Source"))
            .build();

        // Both should be valid InvalidArgument errors
        assert!(matches!(error1, ProviderError::InvalidArgument { .. }));
        assert!(matches!(error2, ProviderError::InvalidArgument { .. }));
    }

    #[test]
    fn test_multiple_invalid_arguments() {
        let error1 = ProviderError::invalid_argument("temperature", "Too high");
        let error2 = ProviderError::invalid_argument("max_tokens", "Too low");

        match (error1, error2) {
            (
                ProviderError::InvalidArgument {
                    argument: arg1,
                    message: msg1,
                    ..
                },
                ProviderError::InvalidArgument {
                    argument: arg2,
                    message: msg2,
                    ..
                },
            ) => {
                assert_eq!(arg1, "temperature");
                assert_eq!(msg1, "Too high");
                assert_eq!(arg2, "max_tokens");
                assert_eq!(msg2, "Too low");
            }
            _ => panic!("Expected InvalidArgument errors"),
        }
    }

    #[test]
    fn test_error_source_chain() {
        use std::io;

        let inner_error = io::Error::new(io::ErrorKind::InvalidData, "Data corruption");
        let error =
            ProviderError::invalid_argument_with_cause("data", "Invalid data format", inner_error);

        match error {
            ProviderError::InvalidArgument { cause, .. } => {
                assert!(cause.is_some());
                let cause_msg = cause.unwrap().to_string();
                assert!(cause_msg.contains("Data corruption"));
            }
            _ => panic!("Expected InvalidArgument error"),
        }
    }

    #[test]
    fn test_different_argument_names() {
        let errors = [
            ProviderError::invalid_argument("temperature", "Invalid temperature"),
            ProviderError::invalid_argument("max_tokens", "Invalid max_tokens"),
            ProviderError::invalid_argument("top_p", "Invalid top_p"),
            ProviderError::invalid_argument("frequency_penalty", "Invalid frequency_penalty"),
        ];

        let expected_args = ["temperature", "max_tokens", "top_p", "frequency_penalty"];

        for (error, expected_arg) in errors.iter().zip(expected_args.iter()) {
            if let ProviderError::InvalidArgument { argument, .. } = error {
                assert_eq!(argument, expected_arg);
            } else {
                panic!("Expected InvalidArgument error");
            }
        }
    }
}
