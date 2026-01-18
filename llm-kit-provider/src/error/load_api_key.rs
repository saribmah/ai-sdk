use super::ProviderError;

/// Builder for constructing LoadAPIKey error with a fluent interface
///
/// # Examples
///
/// ```
/// use llm_kit_provider::error::{LoadAPIKeyErrorBuilder, ProviderError};
///
/// let error = LoadAPIKeyErrorBuilder::new(
///     "API key not found in environment variable OPENAI_API_KEY"
/// )
/// .build();
///
/// match error {
///     ProviderError::LoadAPIKey { message } => {
///         assert_eq!(message, "API key not found in environment variable OPENAI_API_KEY");
///     }
///     _ => panic!("Expected LoadAPIKey error"),
/// }
/// ```
#[derive(Debug, Default)]
pub struct LoadAPIKeyErrorBuilder {
    message: String,
}

impl LoadAPIKeyErrorBuilder {
    /// Create a new LoadAPIKeyErrorBuilder with required fields
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Build the ProviderError
    pub fn build(self) -> ProviderError {
        ProviderError::LoadAPIKey {
            message: self.message,
        }
    }
}

impl ProviderError {
    /// Create a new LoadAPIKey error
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_provider::error::ProviderError;
    ///
    /// let error = ProviderError::load_api_key_error(
    ///     "OPENAI_API_KEY environment variable is not set"
    /// );
    ///
    /// match error {
    ///     ProviderError::LoadAPIKey { message } => {
    ///         assert_eq!(message, "OPENAI_API_KEY environment variable is not set");
    ///     }
    ///     _ => panic!("Expected LoadAPIKey error"),
    /// }
    /// ```
    pub fn load_api_key_error(message: impl Into<String>) -> Self {
        Self::LoadAPIKey {
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_api_key_error_basic() {
        let error = ProviderError::load_api_key_error("API key is missing");

        match error {
            ProviderError::LoadAPIKey { message } => {
                assert_eq!(message, "API key is missing");
            }
            _ => panic!("Expected LoadAPIKey error"),
        }
    }

    #[test]
    fn test_load_api_key_error_builder() {
        let error = LoadAPIKeyErrorBuilder::new("Failed to load API key from environment").build();

        match error {
            ProviderError::LoadAPIKey { message } => {
                assert_eq!(message, "Failed to load API key from environment");
            }
            _ => panic!("Expected LoadAPIKey error"),
        }
    }

    #[test]
    fn test_load_api_key_error_not_retryable() {
        let error = ProviderError::load_api_key_error("test");
        assert!(!error.is_retryable());
        assert!(error.status_code().is_none());
        assert!(error.url().is_none());
    }

    #[test]
    fn test_error_display() {
        let error = ProviderError::load_api_key_error("API key not found");
        let display = format!("{}", error);
        assert!(display.contains("Load API key error"));
        assert!(display.contains("API key not found"));
    }

    #[test]
    fn test_different_error_messages() {
        let messages = vec![
            "OPENAI_API_KEY environment variable is not set",
            "API key file not found at ~/.openai/key",
            "Failed to read API key from configuration",
            "API key is empty or invalid format",
        ];

        for msg in messages {
            let error = ProviderError::load_api_key_error(msg);
            if let ProviderError::LoadAPIKey { message } = error {
                assert_eq!(message, msg);
            } else {
                panic!("Expected LoadAPIKey error");
            }
        }
    }

    #[test]
    fn test_builder_creates_same_as_direct() {
        let msg = "API key error";
        let error1 = ProviderError::load_api_key_error(msg);
        let error2 = LoadAPIKeyErrorBuilder::new(msg).build();

        match (error1, error2) {
            (
                ProviderError::LoadAPIKey { message: msg1 },
                ProviderError::LoadAPIKey { message: msg2 },
            ) => {
                assert_eq!(msg1, msg2);
            }
            _ => panic!("Expected LoadAPIKey errors"),
        }
    }

    #[test]
    fn test_env_var_specific_messages() {
        let error = ProviderError::load_api_key_error(
            "Environment variable 'ANTHROPIC_API_KEY' is not set. Please set it to your API key.",
        );

        if let ProviderError::LoadAPIKey { message } = error {
            assert!(message.contains("ANTHROPIC_API_KEY"));
            assert!(message.contains("not set"));
        } else {
            panic!("Expected LoadAPIKey error");
        }
    }

    #[test]
    fn test_empty_message() {
        let error = ProviderError::load_api_key_error("");
        match error {
            ProviderError::LoadAPIKey { message } => {
                assert_eq!(message, "");
            }
            _ => panic!("Expected LoadAPIKey error"),
        }
    }

    #[test]
    fn test_long_message() {
        let long_msg = format!(
            "Failed to load API key: {}",
            "detailed explanation ".repeat(50)
        );
        let error = ProviderError::load_api_key_error(&long_msg);

        if let ProviderError::LoadAPIKey { message } = error {
            assert_eq!(message, long_msg);
        } else {
            panic!("Expected LoadAPIKey error");
        }
    }
}
