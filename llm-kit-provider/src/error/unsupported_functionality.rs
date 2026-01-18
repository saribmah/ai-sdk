use super::ProviderError;

/// Builder for constructing UnsupportedFunctionality error with a fluent interface
///
/// # Examples
///
/// ```
/// use llm_kit_provider::error::{UnsupportedFunctionalityErrorBuilder, ProviderError};
///
/// let error = UnsupportedFunctionalityErrorBuilder::new("streaming")
///     .message("This model does not support streaming responses")
///     .build();
///
/// match error {
///     ProviderError::UnsupportedFunctionality { functionality, message } => {
///         assert_eq!(functionality, "streaming");
///         assert_eq!(message, "This model does not support streaming responses");
///     }
///     _ => panic!("Expected UnsupportedFunctionality error"),
/// }
/// ```
#[derive(Debug, Default)]
pub struct UnsupportedFunctionalityErrorBuilder {
    functionality: String,
    message: Option<String>,
}

impl UnsupportedFunctionalityErrorBuilder {
    /// Create a new UnsupportedFunctionalityErrorBuilder with required fields
    pub fn new(functionality: impl Into<String>) -> Self {
        Self {
            functionality: functionality.into(),
            message: None,
        }
    }

    /// Set a custom error message
    ///
    /// If not set, defaults to "'{functionality}' functionality not supported"
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Build the ProviderError
    pub fn build(self) -> ProviderError {
        let message = self
            .message
            .unwrap_or_else(|| format!("'{}' functionality not supported", self.functionality));

        ProviderError::UnsupportedFunctionality {
            functionality: self.functionality,
            message,
        }
    }
}

impl ProviderError {
    /// Create a new UnsupportedFunctionality error with default message
    ///
    /// The default message will be "'{functionality}' functionality not supported"
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_provider::error::ProviderError;
    ///
    /// let error = ProviderError::unsupported_functionality("vision");
    ///
    /// match error {
    ///     ProviderError::UnsupportedFunctionality { functionality, message } => {
    ///         assert_eq!(functionality, "vision");
    ///         assert_eq!(message, "'vision' functionality not supported");
    ///     }
    ///     _ => panic!("Expected UnsupportedFunctionality error"),
    /// }
    /// ```
    pub fn unsupported_functionality(functionality: impl Into<String>) -> Self {
        let functionality_str = functionality.into();
        Self::UnsupportedFunctionality {
            message: format!("'{}' functionality not supported", functionality_str),
            functionality: functionality_str,
        }
    }

    /// Create a new UnsupportedFunctionality error with a custom message
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_provider::error::ProviderError;
    ///
    /// let error = ProviderError::unsupported_functionality_with_message(
    ///     "function_calling",
    ///     "This model version does not support function calling. Please upgrade to the latest version.",
    /// );
    ///
    /// match error {
    ///     ProviderError::UnsupportedFunctionality { functionality, message } => {
    ///         assert_eq!(functionality, "function_calling");
    ///         assert!(message.contains("does not support function calling"));
    ///     }
    ///     _ => panic!("Expected UnsupportedFunctionality error"),
    /// }
    /// ```
    pub fn unsupported_functionality_with_message(
        functionality: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::UnsupportedFunctionality {
            functionality: functionality.into(),
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsupported_functionality_default() {
        let error = ProviderError::unsupported_functionality("streaming");

        match error {
            ProviderError::UnsupportedFunctionality {
                functionality,
                message,
            } => {
                assert_eq!(functionality, "streaming");
                assert_eq!(message, "'streaming' functionality not supported");
            }
            _ => panic!("Expected UnsupportedFunctionality error"),
        }
    }

    #[test]
    fn test_unsupported_functionality_with_message() {
        let error = ProviderError::unsupported_functionality_with_message(
            "vision",
            "Vision capabilities require GPT-4 Vision model",
        );

        match error {
            ProviderError::UnsupportedFunctionality {
                functionality,
                message,
            } => {
                assert_eq!(functionality, "vision");
                assert_eq!(message, "Vision capabilities require GPT-4 Vision model");
            }
            _ => panic!("Expected UnsupportedFunctionality error"),
        }
    }

    #[test]
    fn test_unsupported_functionality_builder_default() {
        let error = UnsupportedFunctionalityErrorBuilder::new("tool_calling").build();

        match error {
            ProviderError::UnsupportedFunctionality {
                functionality,
                message,
            } => {
                assert_eq!(functionality, "tool_calling");
                assert_eq!(message, "'tool_calling' functionality not supported");
            }
            _ => panic!("Expected UnsupportedFunctionality error"),
        }
    }

    #[test]
    fn test_unsupported_functionality_builder_with_message() {
        let error = UnsupportedFunctionalityErrorBuilder::new("parallel_tool_calls")
            .message("Parallel tool calling is only available in GPT-4 Turbo and later")
            .build();

        match error {
            ProviderError::UnsupportedFunctionality {
                functionality,
                message,
            } => {
                assert_eq!(functionality, "parallel_tool_calls");
                assert!(message.contains("GPT-4 Turbo"));
            }
            _ => panic!("Expected UnsupportedFunctionality error"),
        }
    }

    #[test]
    fn test_unsupported_functionality_not_retryable() {
        let error = ProviderError::unsupported_functionality("test");
        assert!(!error.is_retryable());
        assert!(error.status_code().is_none());
        assert!(error.url().is_none());
    }

    #[test]
    fn test_error_display() {
        let error = ProviderError::unsupported_functionality("audio");
        let display = format!("{}", error);
        assert!(display.contains("Unsupported functionality"));
        assert!(display.contains("audio"));
    }

    #[test]
    fn test_builder_method_chaining() {
        let error1 = UnsupportedFunctionalityErrorBuilder::new("feature1").build();
        let error2 = UnsupportedFunctionalityErrorBuilder::new("feature2")
            .message("Custom message")
            .build();

        // Both should be valid UnsupportedFunctionality errors
        assert!(matches!(
            error1,
            ProviderError::UnsupportedFunctionality { .. }
        ));
        assert!(matches!(
            error2,
            ProviderError::UnsupportedFunctionality { .. }
        ));
    }

    #[test]
    fn test_different_functionalities() {
        let functionalities = vec![
            ("streaming", "Streaming responses"),
            ("vision", "Image understanding"),
            ("function_calling", "Function/tool calling"),
            ("json_mode", "JSON response format"),
            ("system_messages", "System message role"),
            ("tool_choice", "Forcing specific tool usage"),
        ];

        for (functionality, _description) in functionalities {
            let error = ProviderError::unsupported_functionality(functionality);
            match error {
                ProviderError::UnsupportedFunctionality {
                    functionality: f, ..
                } => {
                    assert_eq!(f, functionality);
                }
                _ => panic!("Expected UnsupportedFunctionality error"),
            }
        }
    }

    #[test]
    fn test_builder_creates_same_as_direct_default() {
        let functionality = "test_feature";
        let error1 = ProviderError::unsupported_functionality(functionality);
        let error2 = UnsupportedFunctionalityErrorBuilder::new(functionality).build();

        match (error1, error2) {
            (
                ProviderError::UnsupportedFunctionality {
                    functionality: f1,
                    message: m1,
                },
                ProviderError::UnsupportedFunctionality {
                    functionality: f2,
                    message: m2,
                },
            ) => {
                assert_eq!(f1, f2);
                assert_eq!(m1, m2);
            }
            _ => panic!("Expected UnsupportedFunctionality errors"),
        }
    }

    #[test]
    fn test_builder_creates_same_as_direct_custom() {
        let functionality = "custom_feature";
        let message = "Custom unsupported message";
        let error1 = ProviderError::unsupported_functionality_with_message(functionality, message);
        let error2 = UnsupportedFunctionalityErrorBuilder::new(functionality)
            .message(message)
            .build();

        match (error1, error2) {
            (
                ProviderError::UnsupportedFunctionality {
                    functionality: f1,
                    message: m1,
                },
                ProviderError::UnsupportedFunctionality {
                    functionality: f2,
                    message: m2,
                },
            ) => {
                assert_eq!(f1, f2);
                assert_eq!(m1, m2);
            }
            _ => panic!("Expected UnsupportedFunctionality errors"),
        }
    }

    #[test]
    fn test_realistic_scenarios() {
        let scenarios = vec![
            (
                "streaming",
                "This model does not support streaming. Use a non-streaming endpoint instead.",
            ),
            (
                "vision",
                "Vision input (images) is only supported in GPT-4 Vision and GPT-4 Turbo models.",
            ),
            (
                "json_mode",
                "JSON mode requires model version 2023-12-01 or later.",
            ),
            (
                "parallel_tool_calls",
                "Parallel tool calling is not available in this model. Tools will be called sequentially.",
            ),
        ];

        for (functionality, message) in scenarios {
            let error =
                ProviderError::unsupported_functionality_with_message(functionality, message);
            match error {
                ProviderError::UnsupportedFunctionality {
                    functionality: f,
                    message: m,
                } => {
                    assert_eq!(f, functionality);
                    assert_eq!(m, message);
                }
                _ => panic!("Expected UnsupportedFunctionality error"),
            }
        }
    }

    #[test]
    fn test_empty_functionality_name() {
        let error = ProviderError::unsupported_functionality("");
        match error {
            ProviderError::UnsupportedFunctionality { functionality, .. } => {
                assert_eq!(functionality, "");
            }
            _ => panic!("Expected UnsupportedFunctionality error"),
        }
    }

    #[test]
    fn test_long_functionality_name() {
        let long_name = "very_long_functionality_name_that_describes_something_complex";
        let error = ProviderError::unsupported_functionality(long_name);
        match error {
            ProviderError::UnsupportedFunctionality { functionality, .. } => {
                assert_eq!(functionality, long_name);
            }
            _ => panic!("Expected UnsupportedFunctionality error"),
        }
    }

    #[test]
    fn test_functionality_with_special_characters() {
        let special_name = "function-calling_v2.0";
        let error = ProviderError::unsupported_functionality(special_name);
        match error {
            ProviderError::UnsupportedFunctionality { functionality, .. } => {
                assert_eq!(functionality, special_name);
            }
            _ => panic!("Expected UnsupportedFunctionality error"),
        }
    }
}
