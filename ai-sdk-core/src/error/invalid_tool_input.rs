use crate::error::AISDKError;

/// Builder for [`AISDKError::InvalidToolInput`].
///
/// # Examples
///
/// ```
/// use ai_sdk_core::error::{AISDKError, InvalidToolInputErrorBuilder};
///
/// let error = InvalidToolInputErrorBuilder::new("calculate", r#"{"invalid": "json"#)
///     .message("Failed to parse JSON")
///     .build();
///
/// match error {
///     AISDKError::InvalidToolInput { tool_name, tool_input, message } => {
///         assert_eq!(tool_name, "calculate");
///         assert!(tool_input.contains("invalid"));
///         assert_eq!(message, "Failed to parse JSON");
///     }
///     _ => panic!("Expected InvalidToolInput"),
/// }
/// ```
#[derive(Debug, Clone)]
pub struct InvalidToolInputErrorBuilder {
    tool_name: String,
    tool_input: String,
    message: Option<String>,
}

impl InvalidToolInputErrorBuilder {
    /// Creates a new builder for an invalid tool input error.
    ///
    /// # Arguments
    ///
    /// * `tool_name` - The name of the tool that received invalid input
    /// * `tool_input` - The invalid input that was provided (serialized as string)
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::InvalidToolInputErrorBuilder;
    ///
    /// let builder = InvalidToolInputErrorBuilder::new("search", r#"{"query": null}"#);
    /// ```
    pub fn new(tool_name: impl Into<String>, tool_input: impl Into<String>) -> Self {
        Self {
            tool_name: tool_name.into(),
            tool_input: tool_input.into(),
            message: None,
        }
    }

    /// Sets the error message explaining why the tool input is invalid.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of why the tool input is invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::InvalidToolInputErrorBuilder;
    ///
    /// let builder = InvalidToolInputErrorBuilder::new("calculate", r#"{"op": "unknown"}"#)
    ///     .message("Unknown operation type");
    /// ```
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Builds the [`AISDKError::InvalidToolInput`] error.
    ///
    /// If no custom message is provided, a default message will be generated
    /// using the tool name.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::InvalidToolInputErrorBuilder;
    ///
    /// let error = InvalidToolInputErrorBuilder::new("search", r#"{"query": ""}"#)
    ///     .message("Query cannot be empty")
    ///     .build();
    /// ```
    pub fn build(self) -> AISDKError {
        let message = self
            .message
            .unwrap_or_else(|| format!("Invalid input for tool {}", self.tool_name));

        AISDKError::InvalidToolInput {
            tool_name: self.tool_name,
            tool_input: self.tool_input,
            message,
        }
    }
}

impl AISDKError {
    /// Creates a new invalid tool input error.
    ///
    /// This is a convenience method that creates an error with a tool name,
    /// tool input, and message in one call.
    ///
    /// # Arguments
    ///
    /// * `tool_name` - The name of the tool that received invalid input
    /// * `tool_input` - The invalid input that was provided (serialized as string)
    /// * `message` - Description of why the tool input is invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::AISDKError;
    ///
    /// let error = AISDKError::invalid_tool_input(
    ///     "calculate",
    ///     r#"{"operation": "unknown"}"#,
    ///     "Unknown operation type"
    /// );
    ///
    /// match error {
    ///     AISDKError::InvalidToolInput { tool_name, tool_input, message } => {
    ///         assert_eq!(tool_name, "calculate");
    ///         assert!(tool_input.contains("unknown"));
    ///         assert_eq!(message, "Unknown operation type");
    ///     }
    ///     _ => panic!("Expected InvalidToolInput"),
    /// }
    /// ```
    pub fn invalid_tool_input(
        tool_name: impl Into<String>,
        tool_input: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::InvalidToolInput {
            tool_name: tool_name.into(),
            tool_input: tool_input.into(),
            message: message.into(),
        }
    }

    /// Creates a builder for an invalid tool input error.
    ///
    /// # Arguments
    ///
    /// * `tool_name` - The name of the tool that received invalid input
    /// * `tool_input` - The invalid input that was provided (serialized as string)
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::AISDKError;
    ///
    /// let error = AISDKError::invalid_tool_input_builder("search", r#"{"query": null}"#)
    ///     .message("Query cannot be null")
    ///     .build();
    /// ```
    pub fn invalid_tool_input_builder(
        tool_name: impl Into<String>,
        tool_input: impl Into<String>,
    ) -> InvalidToolInputErrorBuilder {
        InvalidToolInputErrorBuilder::new(tool_name, tool_input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_tool_input_simple() {
        let error = AISDKError::invalid_tool_input(
            "calculate",
            r#"{"operation": "unknown"}"#,
            "Unknown operation type",
        );

        match error {
            AISDKError::InvalidToolInput {
                tool_name,
                tool_input,
                message,
            } => {
                assert_eq!(tool_name, "calculate");
                assert!(tool_input.contains("unknown"));
                assert_eq!(message, "Unknown operation type");
            }
            _ => panic!("Expected InvalidToolInput"),
        }
    }

    #[test]
    fn test_invalid_tool_input_builder() {
        let error = InvalidToolInputErrorBuilder::new("search", r#"{"query": ""}"#)
            .message("Query cannot be empty")
            .build();

        match error {
            AISDKError::InvalidToolInput {
                tool_name,
                tool_input,
                message,
            } => {
                assert_eq!(tool_name, "search");
                assert!(tool_input.contains("query"));
                assert_eq!(message, "Query cannot be empty");
            }
            _ => panic!("Expected InvalidToolInput"),
        }
    }

    #[test]
    fn test_invalid_tool_input_builder_via_error() {
        let error = AISDKError::invalid_tool_input_builder("weather", r#"{"location": null}"#)
            .message("Location cannot be null")
            .build();

        match error {
            AISDKError::InvalidToolInput {
                tool_name,
                tool_input,
                message,
            } => {
                assert_eq!(tool_name, "weather");
                assert!(tool_input.contains("location"));
                assert_eq!(message, "Location cannot be null");
            }
            _ => panic!("Expected InvalidToolInput"),
        }
    }

    #[test]
    fn test_invalid_tool_input_builder_default_message() {
        let error =
            InvalidToolInputErrorBuilder::new("test_tool", r#"{"invalid": "data"}"#).build();

        match error {
            AISDKError::InvalidToolInput {
                tool_name,
                tool_input,
                message,
            } => {
                assert_eq!(tool_name, "test_tool");
                assert!(tool_input.contains("invalid"));
                assert_eq!(message, "Invalid input for tool test_tool");
            }
            _ => panic!("Expected InvalidToolInput"),
        }
    }

    #[test]
    fn test_invalid_tool_input_display() {
        let error = AISDKError::invalid_tool_input(
            "calculate",
            r#"{"x": "not a number"}"#,
            "Expected number for parameter x",
        );
        let display = format!("{}", error);
        assert!(display.contains("calculate"));
        assert!(display.contains("Expected number for parameter x"));
    }

    #[test]
    fn test_invalid_tool_input_with_json_parse_error() {
        let error = AISDKError::invalid_tool_input(
            "parse_data",
            r#"{"malformed": json"#,
            "Failed to parse JSON: unexpected end of input",
        );

        match error {
            AISDKError::InvalidToolInput {
                tool_name,
                tool_input,
                message,
            } => {
                assert_eq!(tool_name, "parse_data");
                assert!(tool_input.contains("malformed"));
                assert!(message.contains("Failed to parse JSON"));
            }
            _ => panic!("Expected InvalidToolInput"),
        }
    }

    #[test]
    fn test_invalid_tool_input_with_validation_error() {
        let error = AISDKError::invalid_tool_input(
            "create_user",
            r#"{"email": "invalid-email", "age": -5}"#,
            "Validation failed: email must be valid, age must be positive",
        );

        match error {
            AISDKError::InvalidToolInput {
                tool_name,
                tool_input,
                message,
            } => {
                assert_eq!(tool_name, "create_user");
                assert!(tool_input.contains("invalid-email"));
                assert!(tool_input.contains("-5"));
                assert!(message.contains("Validation failed"));
            }
            _ => panic!("Expected InvalidToolInput"),
        }
    }

    #[test]
    fn test_invalid_tool_input_with_empty_input() {
        let error = AISDKError::invalid_tool_input("test_tool", "", "Input cannot be empty");

        match error {
            AISDKError::InvalidToolInput {
                tool_name,
                tool_input,
                message,
            } => {
                assert_eq!(tool_name, "test_tool");
                assert_eq!(tool_input, "");
                assert_eq!(message, "Input cannot be empty");
            }
            _ => panic!("Expected InvalidToolInput"),
        }
    }
}
