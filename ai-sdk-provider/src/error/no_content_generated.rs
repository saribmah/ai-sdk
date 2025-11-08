use super::ProviderError;

/// Builder for constructing NoContentGenerated error with a fluent interface
///
/// # Examples
///
/// ```
/// use ai_sdk_provider::error::{NoContentGeneratedErrorBuilder, ProviderError};
///
/// let error = NoContentGeneratedErrorBuilder::new()
///     .message("Model returned empty response with no generated content")
///     .build();
///
/// match error {
///     ProviderError::NoContentGenerated { message } => {
///         assert_eq!(message, "Model returned empty response with no generated content");
///     }
///     _ => panic!("Expected NoContentGenerated error"),
/// }
/// ```
#[derive(Debug, Default)]
pub struct NoContentGeneratedErrorBuilder {
    message: Option<String>,
}

impl NoContentGeneratedErrorBuilder {
    /// Create a new NoContentGeneratedErrorBuilder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a custom error message
    ///
    /// If not set, defaults to "No content generated"
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Build the ProviderError
    pub fn build(self) -> ProviderError {
        ProviderError::NoContentGenerated {
            message: self
                .message
                .unwrap_or_else(|| "No content generated".to_string()),
        }
    }
}

impl ProviderError {
    /// Create a new NoContentGenerated error with default message
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_provider::error::ProviderError;
    ///
    /// let error = ProviderError::no_content_generated();
    ///
    /// match error {
    ///     ProviderError::NoContentGenerated { message } => {
    ///         assert_eq!(message, "No content generated");
    ///     }
    ///     _ => panic!("Expected NoContentGenerated error"),
    /// }
    /// ```
    pub fn no_content_generated() -> Self {
        Self::NoContentGenerated {
            message: "No content generated".to_string(),
        }
    }

    /// Create a new NoContentGenerated error with a custom message
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_provider::error::ProviderError;
    ///
    /// let error = ProviderError::no_content_generated_with_message(
    ///     "AI provider returned a response but generated no text content"
    /// );
    ///
    /// match error {
    ///     ProviderError::NoContentGenerated { message } => {
    ///         assert_eq!(message, "AI provider returned a response but generated no text content");
    ///     }
    ///     _ => panic!("Expected NoContentGenerated error"),
    /// }
    /// ```
    pub fn no_content_generated_with_message(message: impl Into<String>) -> Self {
        Self::NoContentGenerated {
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_content_generated_default() {
        let error = ProviderError::no_content_generated();

        match error {
            ProviderError::NoContentGenerated { message } => {
                assert_eq!(message, "No content generated");
            }
            _ => panic!("Expected NoContentGenerated error"),
        }
    }

    #[test]
    fn test_no_content_generated_with_message() {
        let error = ProviderError::no_content_generated_with_message(
            "Model completed but produced no content",
        );

        match error {
            ProviderError::NoContentGenerated { message } => {
                assert_eq!(message, "Model completed but produced no content");
            }
            _ => panic!("Expected NoContentGenerated error"),
        }
    }

    #[test]
    fn test_no_content_generated_builder_default() {
        let error = NoContentGeneratedErrorBuilder::new().build();

        match error {
            ProviderError::NoContentGenerated { message } => {
                assert_eq!(message, "No content generated");
            }
            _ => panic!("Expected NoContentGenerated error"),
        }
    }

    #[test]
    fn test_no_content_generated_builder_with_message() {
        let error = NoContentGeneratedErrorBuilder::new()
            .message("Empty response from AI model")
            .build();

        match error {
            ProviderError::NoContentGenerated { message } => {
                assert_eq!(message, "Empty response from AI model");
            }
            _ => panic!("Expected NoContentGenerated error"),
        }
    }

    #[test]
    fn test_no_content_generated_not_retryable() {
        let error = ProviderError::no_content_generated();
        assert!(!error.is_retryable());
        assert!(error.status_code().is_none());
        assert!(error.url().is_none());
    }

    #[test]
    fn test_error_display() {
        let error = ProviderError::no_content_generated();
        let display = format!("{}", error);
        assert!(display.contains("No content generated"));
    }

    #[test]
    fn test_builder_method_chaining() {
        let error1 = NoContentGeneratedErrorBuilder::new().build();
        let error2 = NoContentGeneratedErrorBuilder::new()
            .message("Custom message")
            .build();

        // Both should be valid NoContentGenerated errors
        assert!(matches!(error1, ProviderError::NoContentGenerated { .. }));
        assert!(matches!(error2, ProviderError::NoContentGenerated { .. }));
    }

    #[test]
    fn test_different_message_scenarios() {
        let messages = vec![
            "No content generated",
            "Model returned empty choices array",
            "Response contained no text in any choice",
            "All generated content was filtered by content policy",
            "Stream ended without producing any text tokens",
        ];

        for msg in messages {
            let error = ProviderError::no_content_generated_with_message(msg);
            if let ProviderError::NoContentGenerated { message } = error {
                assert_eq!(message, msg);
            } else {
                panic!("Expected NoContentGenerated error");
            }
        }
    }

    #[test]
    fn test_builder_creates_same_as_direct_default() {
        let error1 = ProviderError::no_content_generated();
        let error2 = NoContentGeneratedErrorBuilder::new().build();

        match (error1, error2) {
            (
                ProviderError::NoContentGenerated { message: msg1 },
                ProviderError::NoContentGenerated { message: msg2 },
            ) => {
                assert_eq!(msg1, msg2);
                assert_eq!(msg1, "No content generated");
            }
            _ => panic!("Expected NoContentGenerated errors"),
        }
    }

    #[test]
    fn test_builder_creates_same_as_direct_custom() {
        let msg = "Custom no content message";
        let error1 = ProviderError::no_content_generated_with_message(msg);
        let error2 = NoContentGeneratedErrorBuilder::new().message(msg).build();

        match (error1, error2) {
            (
                ProviderError::NoContentGenerated { message: msg1 },
                ProviderError::NoContentGenerated { message: msg2 },
            ) => {
                assert_eq!(msg1, msg2);
                assert_eq!(msg1, msg);
            }
            _ => panic!("Expected NoContentGenerated errors"),
        }
    }

    #[test]
    fn test_empty_content_scenarios() {
        let scenarios = vec![
            (
                "finish_reason was 'length' but no content was generated",
                "length limit",
            ),
            (
                "finish_reason was 'content_filter' and content was removed",
                "content filter",
            ),
            (
                "Model stopped early with finish_reason 'stop' but produced no text",
                "early stop",
            ),
        ];

        for (msg, scenario) in scenarios {
            let error = ProviderError::no_content_generated_with_message(msg);
            if let ProviderError::NoContentGenerated { message } = error {
                assert!(
                    message.contains(msg),
                    "Message doesn't match for scenario: {}",
                    scenario
                );
            } else {
                panic!("Expected NoContentGenerated error for: {}", scenario);
            }
        }
    }

    #[test]
    fn test_empty_message_string() {
        let error = ProviderError::no_content_generated_with_message("");
        match error {
            ProviderError::NoContentGenerated { message } => {
                assert_eq!(message, "");
            }
            _ => panic!("Expected NoContentGenerated error"),
        }
    }
}
