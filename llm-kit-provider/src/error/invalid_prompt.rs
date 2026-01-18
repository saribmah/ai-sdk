use super::ProviderError;

/// Builder for constructing InvalidPrompt error with a fluent interface
///
/// # Examples
///
/// ```
/// use llm_kit_provider::error::{InvalidPromptErrorBuilder, ProviderError};
///
/// let error = InvalidPromptErrorBuilder::new(
///     "Prompt contains invalid characters",
///     r#"{"invalid": "prompt"}"#,
/// )
/// .build();
///
/// match error {
///     ProviderError::InvalidPrompt { message, prompt, .. } => {
///         assert!(message.contains("Prompt contains invalid characters"));
///         assert_eq!(prompt, r#"{"invalid": "prompt"}"#);
///     }
///     _ => panic!("Expected InvalidPrompt error"),
/// }
/// ```
#[derive(Debug, Default)]
pub struct InvalidPromptErrorBuilder {
    message: String,
    prompt: String,
    cause: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl InvalidPromptErrorBuilder {
    /// Create a new InvalidPromptErrorBuilder with required fields
    ///
    /// The message will be automatically prefixed with "Invalid prompt: "
    pub fn new(message: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            prompt: prompt.into(),
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
        ProviderError::InvalidPrompt {
            message: self.message,
            prompt: self.prompt,
            cause: self.cause,
        }
    }
}

impl ProviderError {
    /// Create a new InvalidPrompt error
    ///
    /// The message will be automatically prefixed with "Invalid prompt: " in the error display.
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_provider::error::ProviderError;
    ///
    /// let error = ProviderError::invalid_prompt(
    ///     "Prompt exceeds maximum length",
    ///     "very long prompt...",
    /// );
    ///
    /// match error {
    ///     ProviderError::InvalidPrompt { message, prompt, .. } => {
    ///         assert_eq!(message, "Prompt exceeds maximum length");
    ///         assert_eq!(prompt, "very long prompt...");
    ///     }
    ///     _ => panic!("Expected InvalidPrompt error"),
    /// }
    /// ```
    pub fn invalid_prompt(message: impl Into<String>, prompt: impl Into<String>) -> Self {
        Self::InvalidPrompt {
            message: message.into(),
            prompt: prompt.into(),
            cause: None,
        }
    }

    /// Create a new InvalidPrompt error with a cause
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_provider::error::ProviderError;
    /// use std::io;
    ///
    /// let cause_error = io::Error::new(io::ErrorKind::InvalidInput, "Parse error");
    /// let error = ProviderError::invalid_prompt_with_cause(
    ///     "Failed to parse prompt format",
    ///     r#"{"malformed": json}"#,
    ///     cause_error,
    /// );
    ///
    /// match error {
    ///     ProviderError::InvalidPrompt { message, prompt, .. } => {
    ///         assert_eq!(message, "Failed to parse prompt format");
    ///         assert_eq!(prompt, r#"{"malformed": json}"#);
    ///     }
    ///     _ => panic!("Expected InvalidPrompt error"),
    /// }
    /// ```
    pub fn invalid_prompt_with_cause(
        message: impl Into<String>,
        prompt: impl Into<String>,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::InvalidPrompt {
            message: message.into(),
            prompt: prompt.into(),
            cause: Some(Box::new(cause)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_prompt_basic() {
        let error = ProviderError::invalid_prompt("Empty prompt not allowed", "");

        match error {
            ProviderError::InvalidPrompt {
                message,
                prompt,
                cause,
            } => {
                assert_eq!(message, "Empty prompt not allowed");
                assert_eq!(prompt, "");
                assert!(cause.is_none());
            }
            _ => panic!("Expected InvalidPrompt error"),
        }
    }

    #[test]
    fn test_invalid_prompt_with_cause() {
        use std::io;

        let cause = io::Error::new(io::ErrorKind::InvalidInput, "JSON parse error");
        let error = ProviderError::invalid_prompt_with_cause(
            "Malformed JSON prompt",
            r#"{"invalid"}"#,
            cause,
        );

        match error {
            ProviderError::InvalidPrompt {
                message,
                prompt,
                cause,
            } => {
                assert_eq!(message, "Malformed JSON prompt");
                assert_eq!(prompt, r#"{"invalid"}"#);
                assert!(cause.is_some());
            }
            _ => panic!("Expected InvalidPrompt error"),
        }
    }

    #[test]
    fn test_invalid_prompt_builder() {
        let error = InvalidPromptErrorBuilder::new(
            "Prompt contains disallowed content",
            "inappropriate content",
        )
        .build();

        match error {
            ProviderError::InvalidPrompt {
                message,
                prompt,
                cause,
            } => {
                assert_eq!(message, "Prompt contains disallowed content");
                assert_eq!(prompt, "inappropriate content");
                assert!(cause.is_none());
            }
            _ => panic!("Expected InvalidPrompt error"),
        }
    }

    #[test]
    fn test_invalid_prompt_builder_with_cause() {
        use std::io;

        let cause = io::Error::new(io::ErrorKind::InvalidData, "Encoding error");
        let error = InvalidPromptErrorBuilder::new("Invalid encoding", "corrupt data")
            .cause(cause)
            .build();

        match error {
            ProviderError::InvalidPrompt {
                message,
                prompt,
                cause,
            } => {
                assert_eq!(message, "Invalid encoding");
                assert_eq!(prompt, "corrupt data");
                assert!(cause.is_some());
            }
            _ => panic!("Expected InvalidPrompt error"),
        }
    }

    #[test]
    fn test_invalid_prompt_not_retryable() {
        let error = ProviderError::invalid_prompt("test", "test prompt");
        assert!(!error.is_retryable());
        assert!(error.status_code().is_none());
        assert!(error.url().is_none());
    }

    #[test]
    fn test_error_display() {
        let error = ProviderError::invalid_prompt("Too long", "very long prompt");
        let display = format!("{}", error);
        assert!(display.contains("Invalid prompt"));
        assert!(display.contains("Too long"));
    }

    #[test]
    fn test_builder_method_chaining() {
        use std::io;

        let error1 = InvalidPromptErrorBuilder::new("Error 1", "prompt1").build();
        let error2 = InvalidPromptErrorBuilder::new("Error 2", "prompt2")
            .cause(io::Error::other("Source"))
            .build();

        // Both should be valid InvalidPrompt errors
        assert!(matches!(error1, ProviderError::InvalidPrompt { .. }));
        assert!(matches!(error2, ProviderError::InvalidPrompt { .. }));
    }

    #[test]
    fn test_different_prompt_types() {
        let errors = vec![
            ProviderError::invalid_prompt("Empty", ""),
            ProviderError::invalid_prompt("Too long", "a".repeat(10000)),
            ProviderError::invalid_prompt("Invalid JSON", r#"{"bad": json}"#),
            ProviderError::invalid_prompt("Contains nulls", "text\0with\0nulls"),
        ];

        assert_eq!(errors.len(), 4);
        for error in errors {
            assert!(matches!(error, ProviderError::InvalidPrompt { .. }));
        }
    }

    #[test]
    fn test_prompt_preservation() {
        let original_prompt = r#"{
            "messages": [
                {"role": "user", "content": "Hello"}
            ]
        }"#;

        let error = ProviderError::invalid_prompt("Missing required field", original_prompt);

        match error {
            ProviderError::InvalidPrompt { prompt, .. } => {
                assert_eq!(prompt, original_prompt);
            }
            _ => panic!("Expected InvalidPrompt error"),
        }
    }

    #[test]
    fn test_error_source_chain() {
        use std::io;

        let inner_error = io::Error::new(io::ErrorKind::InvalidData, "Validation failed");
        let error = ProviderError::invalid_prompt_with_cause(
            "Schema validation failed",
            "invalid schema",
            inner_error,
        );

        match error {
            ProviderError::InvalidPrompt { cause, .. } => {
                assert!(cause.is_some());
                let cause_msg = cause.unwrap().to_string();
                assert!(cause_msg.contains("Validation failed"));
            }
            _ => panic!("Expected InvalidPrompt error"),
        }
    }

    #[test]
    fn test_message_variations() {
        let messages = vec![
            "Prompt is empty",
            "Prompt exceeds maximum length of 4096 tokens",
            "Prompt contains invalid Unicode characters",
            "Prompt format not recognized",
        ];

        for msg in messages {
            let error = ProviderError::invalid_prompt(msg, "test");
            if let ProviderError::InvalidPrompt { message, .. } = error {
                assert_eq!(message, msg);
            } else {
                panic!("Expected InvalidPrompt error");
            }
        }
    }
}
