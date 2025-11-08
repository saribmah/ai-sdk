use super::ProviderError;

/// Builder for constructing EmptyResponseBody error with a fluent interface
///
/// # Examples
///
/// ```
/// use ai_sdk_provider::error::{EmptyResponseBodyErrorBuilder, ProviderError};
///
/// let error = EmptyResponseBodyErrorBuilder::new()
///     .message("Expected response but got empty body")
///     .build();
///
/// match error {
///     ProviderError::EmptyResponseBody { message } => {
///         assert_eq!(message, "Expected response but got empty body");
///     }
///     _ => panic!("Expected EmptyResponseBody error"),
/// }
/// ```
#[derive(Debug, Default)]
pub struct EmptyResponseBodyErrorBuilder {
    message: Option<String>,
}

impl EmptyResponseBodyErrorBuilder {
    /// Create a new EmptyResponseBodyErrorBuilder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a custom error message
    ///
    /// If not set, defaults to "Empty response body"
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Build the ProviderError
    pub fn build(self) -> ProviderError {
        ProviderError::EmptyResponseBody {
            message: self
                .message
                .unwrap_or_else(|| "Empty response body".to_string()),
        }
    }
}

impl ProviderError {
    /// Create a new EmptyResponseBody error with default message
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_provider::error::ProviderError;
    ///
    /// let error = ProviderError::empty_response_body();
    ///
    /// match error {
    ///     ProviderError::EmptyResponseBody { message } => {
    ///         assert_eq!(message, "Empty response body");
    ///     }
    ///     _ => panic!("Expected EmptyResponseBody error"),
    /// }
    /// ```
    pub fn empty_response_body() -> Self {
        Self::EmptyResponseBody {
            message: "Empty response body".to_string(),
        }
    }

    /// Create a new EmptyResponseBody error with a custom message
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_provider::error::ProviderError;
    ///
    /// let error = ProviderError::empty_response_body_with_message(
    ///     "Expected streaming response but received empty body"
    /// );
    ///
    /// match error {
    ///     ProviderError::EmptyResponseBody { message } => {
    ///         assert_eq!(message, "Expected streaming response but received empty body");
    ///     }
    ///     _ => panic!("Expected EmptyResponseBody error"),
    /// }
    /// ```
    pub fn empty_response_body_with_message(message: impl Into<String>) -> Self {
        Self::EmptyResponseBody {
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_response_body_default() {
        let error = ProviderError::empty_response_body();

        match error {
            ProviderError::EmptyResponseBody { message } => {
                assert_eq!(message, "Empty response body");
            }
            _ => panic!("Expected EmptyResponseBody error"),
        }
    }

    #[test]
    fn test_empty_response_body_with_message() {
        let error = ProviderError::empty_response_body_with_message("Custom empty body message");

        match error {
            ProviderError::EmptyResponseBody { message } => {
                assert_eq!(message, "Custom empty body message");
            }
            _ => panic!("Expected EmptyResponseBody error"),
        }
    }

    #[test]
    fn test_empty_response_body_builder_default() {
        let error = EmptyResponseBodyErrorBuilder::new().build();

        match error {
            ProviderError::EmptyResponseBody { message } => {
                assert_eq!(message, "Empty response body");
            }
            _ => panic!("Expected EmptyResponseBody error"),
        }
    }

    #[test]
    fn test_empty_response_body_builder_with_message() {
        let error = EmptyResponseBodyErrorBuilder::new()
            .message("Builder custom message")
            .build();

        match error {
            ProviderError::EmptyResponseBody { message } => {
                assert_eq!(message, "Builder custom message");
            }
            _ => panic!("Expected EmptyResponseBody error"),
        }
    }

    #[test]
    fn test_empty_response_body_not_retryable() {
        let error = ProviderError::empty_response_body();
        assert!(!error.is_retryable());
        assert!(error.status_code().is_none());
        assert!(error.url().is_none());
    }

    #[test]
    fn test_error_display() {
        let error = ProviderError::empty_response_body();
        let display = format!("{}", error);
        assert!(display.contains("Empty response body"));
    }

    #[test]
    fn test_builder_method_chaining() {
        let error1 = EmptyResponseBodyErrorBuilder::new().build();
        let error2 = EmptyResponseBodyErrorBuilder::new()
            .message("Chain test")
            .build();

        // Both should be valid EmptyResponseBody errors
        assert!(matches!(error1, ProviderError::EmptyResponseBody { .. }));
        assert!(matches!(error2, ProviderError::EmptyResponseBody { .. }));
    }

    #[test]
    fn test_empty_response_body_different_messages() {
        let error1 = ProviderError::empty_response_body();
        let error2 = ProviderError::empty_response_body_with_message("Custom message");

        match (error1, error2) {
            (
                ProviderError::EmptyResponseBody { message: msg1 },
                ProviderError::EmptyResponseBody { message: msg2 },
            ) => {
                assert_eq!(msg1, "Empty response body");
                assert_eq!(msg2, "Custom message");
            }
            _ => panic!("Expected EmptyResponseBody errors"),
        }
    }
}
