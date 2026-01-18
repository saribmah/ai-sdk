use crate::error::AISDKError;

/// Builder for [`AISDKError::NoOutputGenerated`].
///
/// # Examples
///
/// ```
/// use llm_kit_core::error::{AISDKError, NoOutputGeneratedErrorBuilder};
///
/// let error = NoOutputGeneratedErrorBuilder::new()
///     .message("The model failed to generate any output")
///     .build();
///
/// match error {
///     AISDKError::NoOutputGenerated { message } => {
///         assert_eq!(message, "The model failed to generate any output");
///     }
///     _ => panic!("Expected NoOutputGenerated"),
/// }
/// ```
#[derive(Debug, Clone, Default)]
pub struct NoOutputGeneratedErrorBuilder {
    message: Option<String>,
}

impl NoOutputGeneratedErrorBuilder {
    /// Creates a new builder for a no output generated error.
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::NoOutputGeneratedErrorBuilder;
    ///
    /// let builder = NoOutputGeneratedErrorBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the error message.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of why no output was generated
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::NoOutputGeneratedErrorBuilder;
    ///
    /// let builder = NoOutputGeneratedErrorBuilder::new()
    ///     .message("Model failed to generate a response");
    /// ```
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Builds the [`AISDKError::NoOutputGenerated`] error.
    ///
    /// If no custom message is provided, defaults to "No output generated."
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::NoOutputGeneratedErrorBuilder;
    ///
    /// let error = NoOutputGeneratedErrorBuilder::new()
    ///     .message("Model failed to generate response")
    ///     .build();
    /// ```
    pub fn build(self) -> AISDKError {
        AISDKError::NoOutputGenerated {
            message: self
                .message
                .unwrap_or_else(|| "No output generated.".to_string()),
        }
    }
}

impl AISDKError {
    /// Creates a new no output generated error with default message.
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::AISDKError;
    ///
    /// let error = AISDKError::no_output_generated();
    ///
    /// match error {
    ///     AISDKError::NoOutputGenerated { message } => {
    ///         assert_eq!(message, "No output generated.");
    ///     }
    ///     _ => panic!("Expected NoOutputGenerated"),
    /// }
    /// ```
    pub fn no_output_generated() -> Self {
        Self::NoOutputGenerated {
            message: "No output generated.".to_string(),
        }
    }

    /// Creates a new no output generated error with a custom message.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of why no output was generated
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::AISDKError;
    ///
    /// let error = AISDKError::no_output_generated_with_message("Model failed to generate response");
    ///
    /// match error {
    ///     AISDKError::NoOutputGenerated { message } => {
    ///         assert_eq!(message, "Model failed to generate response");
    ///     }
    ///     _ => panic!("Expected NoOutputGenerated"),
    /// }
    /// ```
    pub fn no_output_generated_with_message(message: impl Into<String>) -> Self {
        Self::NoOutputGenerated {
            message: message.into(),
        }
    }

    /// Creates a builder for a no output generated error.
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::AISDKError;
    ///
    /// let error = AISDKError::no_output_generated_builder()
    ///     .message("Model failed to generate")
    ///     .build();
    /// ```
    pub fn no_output_generated_builder() -> NoOutputGeneratedErrorBuilder {
        NoOutputGeneratedErrorBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_output_generated_default() {
        let error = AISDKError::no_output_generated();

        match error {
            AISDKError::NoOutputGenerated { message } => {
                assert_eq!(message, "No output generated.");
            }
            _ => panic!("Expected NoOutputGenerated"),
        }
    }

    #[test]
    fn test_no_output_generated_with_message() {
        let error =
            AISDKError::no_output_generated_with_message("Model failed to generate response");

        match error {
            AISDKError::NoOutputGenerated { message } => {
                assert_eq!(message, "Model failed to generate response");
            }
            _ => panic!("Expected NoOutputGenerated"),
        }
    }

    #[test]
    fn test_no_output_generated_builder() {
        let error = NoOutputGeneratedErrorBuilder::new()
            .message("Custom error message")
            .build();

        match error {
            AISDKError::NoOutputGenerated { message } => {
                assert_eq!(message, "Custom error message");
            }
            _ => panic!("Expected NoOutputGenerated"),
        }
    }

    #[test]
    fn test_no_output_generated_builder_via_error() {
        let error = AISDKError::no_output_generated_builder()
            .message("LLM returned empty response")
            .build();

        match error {
            AISDKError::NoOutputGenerated { message } => {
                assert_eq!(message, "LLM returned empty response");
            }
            _ => panic!("Expected NoOutputGenerated"),
        }
    }

    #[test]
    fn test_no_output_generated_builder_default_message() {
        let error = NoOutputGeneratedErrorBuilder::new().build();

        match error {
            AISDKError::NoOutputGenerated { message } => {
                assert_eq!(message, "No output generated.");
            }
            _ => panic!("Expected NoOutputGenerated"),
        }
    }

    #[test]
    fn test_no_output_generated_display() {
        let error = AISDKError::no_output_generated_with_message("Test error");
        let display = format!("{}", error);
        assert!(display.contains("No output generated"));
        assert!(display.contains("Test error"));
    }

    #[test]
    fn test_no_output_generated_because_of_errors() {
        let error = AISDKError::no_output_generated_with_message(
            "No output generated because of API errors",
        );

        match error {
            AISDKError::NoOutputGenerated { message } => {
                assert!(message.contains("API errors"));
            }
            _ => panic!("Expected NoOutputGenerated"),
        }
    }

    #[test]
    fn test_no_output_generated_parsing_failure() {
        let error =
            AISDKError::no_output_generated_with_message("Model response could not be parsed");

        match error {
            AISDKError::NoOutputGenerated { message } => {
                assert!(message.contains("could not be parsed"));
            }
            _ => panic!("Expected NoOutputGenerated"),
        }
    }
}
