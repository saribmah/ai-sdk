use crate::error::AISDKError;
use std::fmt;

/// Builder for [`AISDKError::InvalidArgument`].
///
/// # Examples
///
/// ```
/// use llm_kit_core::error::{AISDKError, InvalidArgumentErrorBuilder};
///
/// let error = InvalidArgumentErrorBuilder::new("temperature")
///     .value(2.5)
///     .message("must be between 0 and 2")
///     .build();
///
/// match error {
///     AISDKError::InvalidArgument { parameter, value, message } => {
///         assert_eq!(parameter, "temperature");
///         assert_eq!(value, "2.5");
///         assert_eq!(message, "must be between 0 and 2");
///     }
///     _ => panic!("Expected InvalidArgument"),
/// }
/// ```
#[derive(Debug, Clone)]
pub struct InvalidArgumentErrorBuilder {
    parameter: String,
    value: Option<String>,
    message: Option<String>,
}

impl InvalidArgumentErrorBuilder {
    /// Creates a new builder for an invalid argument error.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The name of the parameter that has an invalid value
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::InvalidArgumentErrorBuilder;
    ///
    /// let builder = InvalidArgumentErrorBuilder::new("max_tokens");
    /// ```
    pub fn new(parameter: impl Into<String>) -> Self {
        Self {
            parameter: parameter.into(),
            value: None,
            message: None,
        }
    }

    /// Sets the invalid value.
    ///
    /// # Arguments
    ///
    /// * `value` - The invalid value (will be converted to string for debugging)
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::InvalidArgumentErrorBuilder;
    ///
    /// let builder = InvalidArgumentErrorBuilder::new("temperature")
    ///     .value(3.0);
    /// ```
    pub fn value(mut self, value: impl fmt::Display) -> Self {
        self.value = Some(value.to_string());
        self
    }

    /// Sets the error message explaining why the value is invalid.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of why the value is invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::InvalidArgumentErrorBuilder;
    ///
    /// let builder = InvalidArgumentErrorBuilder::new("temperature")
    ///     .message("must be between 0 and 2");
    /// ```
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Builds the [`AISDKError::InvalidArgument`] error.
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::InvalidArgumentErrorBuilder;
    ///
    /// let error = InvalidArgumentErrorBuilder::new("temperature")
    ///     .value(3.0)
    ///     .message("must be between 0 and 2")
    ///     .build();
    /// ```
    pub fn build(self) -> AISDKError {
        AISDKError::InvalidArgument {
            parameter: self.parameter,
            value: self.value.unwrap_or_else(|| "unknown".to_string()),
            message: self.message.unwrap_or_else(|| "invalid value".to_string()),
        }
    }
}

impl AISDKError {
    /// Creates a new invalid argument error.
    ///
    /// This is a convenience method that creates an error with a parameter name,
    /// value, and message in one call.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The name of the parameter that has an invalid value
    /// * `value` - The invalid value (will be converted to string for debugging)
    /// * `message` - Description of why the value is invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::AISDKError;
    ///
    /// let error = AISDKError::invalid_argument("temperature", 3.0, "must be between 0 and 2");
    ///
    /// match error {
    ///     AISDKError::InvalidArgument { parameter, value, message } => {
    ///         assert_eq!(parameter, "temperature");
    ///         assert_eq!(value, "3");
    ///         assert_eq!(message, "must be between 0 and 2");
    ///     }
    ///     _ => panic!("Expected InvalidArgument"),
    /// }
    /// ```
    pub fn invalid_argument(
        parameter: impl Into<String>,
        value: impl fmt::Display,
        message: impl Into<String>,
    ) -> Self {
        Self::InvalidArgument {
            parameter: parameter.into(),
            value: value.to_string(),
            message: message.into(),
        }
    }

    /// Creates a builder for an invalid argument error.
    ///
    /// # Arguments
    ///
    /// * `parameter` - The name of the parameter that has an invalid value
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::AISDKError;
    ///
    /// let error = AISDKError::invalid_argument_builder("temperature")
    ///     .value(3.0)
    ///     .message("must be between 0 and 2")
    ///     .build();
    /// ```
    pub fn invalid_argument_builder(parameter: impl Into<String>) -> InvalidArgumentErrorBuilder {
        InvalidArgumentErrorBuilder::new(parameter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_argument_simple() {
        let error = AISDKError::invalid_argument("temperature", 3.0, "must be between 0 and 2");

        match error {
            AISDKError::InvalidArgument {
                parameter,
                value,
                message,
            } => {
                assert_eq!(parameter, "temperature");
                assert_eq!(value, "3");
                assert_eq!(message, "must be between 0 and 2");
            }
            _ => panic!("Expected InvalidArgument"),
        }
    }

    #[test]
    fn test_invalid_argument_builder() {
        let error = InvalidArgumentErrorBuilder::new("max_tokens")
            .value(-100)
            .message("must be positive")
            .build();

        match error {
            AISDKError::InvalidArgument {
                parameter,
                value,
                message,
            } => {
                assert_eq!(parameter, "max_tokens");
                assert_eq!(value, "-100");
                assert_eq!(message, "must be positive");
            }
            _ => panic!("Expected InvalidArgument"),
        }
    }

    #[test]
    fn test_invalid_argument_builder_via_error() {
        let error = AISDKError::invalid_argument_builder("frequency_penalty")
            .value(5.0)
            .message("must be between -2.0 and 2.0")
            .build();

        match error {
            AISDKError::InvalidArgument {
                parameter,
                value,
                message,
            } => {
                assert_eq!(parameter, "frequency_penalty");
                assert_eq!(value, "5");
                assert_eq!(message, "must be between -2.0 and 2.0");
            }
            _ => panic!("Expected InvalidArgument"),
        }
    }

    #[test]
    fn test_invalid_argument_builder_minimal() {
        let error = InvalidArgumentErrorBuilder::new("param").build();

        match error {
            AISDKError::InvalidArgument {
                parameter,
                value,
                message,
            } => {
                assert_eq!(parameter, "param");
                assert_eq!(value, "unknown");
                assert_eq!(message, "invalid value");
            }
            _ => panic!("Expected InvalidArgument"),
        }
    }

    #[test]
    fn test_invalid_argument_display() {
        let error = AISDKError::invalid_argument("temperature", 3.0, "must be between 0 and 2");
        let display = format!("{}", error);
        assert!(display.contains("temperature"));
        assert!(display.contains("must be between 0 and 2"));
        assert!(display.contains("3"));
    }

    #[test]
    fn test_invalid_argument_with_string_value() {
        let error = AISDKError::invalid_argument("model", "invalid-model", "model not found");

        match error {
            AISDKError::InvalidArgument {
                parameter,
                value,
                message,
            } => {
                assert_eq!(parameter, "model");
                assert_eq!(value, "invalid-model");
                assert_eq!(message, "model not found");
            }
            _ => panic!("Expected InvalidArgument"),
        }
    }

    #[test]
    fn test_invalid_argument_with_complex_type() {
        #[derive(Debug)]
        struct ComplexType {
            field: String,
        }

        impl fmt::Display for ComplexType {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "ComplexType({})", self.field)
            }
        }

        let complex = ComplexType {
            field: "test".to_string(),
        };
        let error = AISDKError::invalid_argument("config", complex, "invalid configuration");

        match error {
            AISDKError::InvalidArgument {
                parameter,
                value,
                message,
            } => {
                assert_eq!(parameter, "config");
                assert_eq!(value, "ComplexType(test)");
                assert_eq!(message, "invalid configuration");
            }
            _ => panic!("Expected InvalidArgument"),
        }
    }
}
