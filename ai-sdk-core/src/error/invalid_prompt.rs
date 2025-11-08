use crate::error::AISDKError;

/// Builder for [`AISDKError::InvalidPrompt`].
///
/// # Examples
///
/// ```
/// use ai_sdk_core::error::{AISDKError, InvalidPromptErrorBuilder};
///
/// let error = InvalidPromptErrorBuilder::new("Prompt cannot be empty")
///     .build();
///
/// match error {
///     AISDKError::InvalidPrompt { message } => {
///         assert_eq!(message, "Prompt cannot be empty");
///     }
///     _ => panic!("Expected InvalidPrompt"),
/// }
/// ```
#[derive(Debug, Clone)]
pub struct InvalidPromptErrorBuilder {
    message: String,
}

impl InvalidPromptErrorBuilder {
    /// Creates a new builder for an invalid prompt error.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of why the prompt is invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::InvalidPromptErrorBuilder;
    ///
    /// let builder = InvalidPromptErrorBuilder::new("Prompt must contain at least one message");
    /// ```
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Builds the [`AISDKError::InvalidPrompt`] error.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::InvalidPromptErrorBuilder;
    ///
    /// let error = InvalidPromptErrorBuilder::new("Invalid prompt format")
    ///     .build();
    /// ```
    pub fn build(self) -> AISDKError {
        AISDKError::InvalidPrompt {
            message: self.message,
        }
    }
}

impl AISDKError {
    /// Creates a new invalid prompt error.
    ///
    /// This is a convenience method that creates an error with a message in one call.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of why the prompt is invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::AISDKError;
    ///
    /// let error = AISDKError::invalid_prompt("Prompt cannot be empty");
    ///
    /// match error {
    ///     AISDKError::InvalidPrompt { message } => {
    ///         assert_eq!(message, "Prompt cannot be empty");
    ///     }
    ///     _ => panic!("Expected InvalidPrompt"),
    /// }
    /// ```
    pub fn invalid_prompt(message: impl Into<String>) -> Self {
        Self::InvalidPrompt {
            message: message.into(),
        }
    }

    /// Creates a builder for an invalid prompt error.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of why the prompt is invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::AISDKError;
    ///
    /// let error = AISDKError::invalid_prompt_builder("Prompt must contain at least one message")
    ///     .build();
    /// ```
    pub fn invalid_prompt_builder(message: impl Into<String>) -> InvalidPromptErrorBuilder {
        InvalidPromptErrorBuilder::new(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_prompt_simple() {
        let error = AISDKError::invalid_prompt("Prompt cannot be empty");

        match error {
            AISDKError::InvalidPrompt { message } => {
                assert_eq!(message, "Prompt cannot be empty");
            }
            _ => panic!("Expected InvalidPrompt"),
        }
    }

    #[test]
    fn test_invalid_prompt_builder() {
        let error = InvalidPromptErrorBuilder::new("Invalid message format").build();

        match error {
            AISDKError::InvalidPrompt { message } => {
                assert_eq!(message, "Invalid message format");
            }
            _ => panic!("Expected InvalidPrompt"),
        }
    }

    #[test]
    fn test_invalid_prompt_builder_via_error() {
        let error =
            AISDKError::invalid_prompt_builder("Prompt must contain text or images").build();

        match error {
            AISDKError::InvalidPrompt { message } => {
                assert_eq!(message, "Prompt must contain text or images");
            }
            _ => panic!("Expected InvalidPrompt"),
        }
    }

    #[test]
    fn test_invalid_prompt_display() {
        let error = AISDKError::invalid_prompt("System message must come first");
        let display = format!("{}", error);
        assert!(display.contains("Invalid prompt"));
        assert!(display.contains("System message must come first"));
    }

    #[test]
    fn test_invalid_prompt_with_details() {
        let error = AISDKError::invalid_prompt(
            "Prompt contains unsupported message type: function call is not supported",
        );

        match error {
            AISDKError::InvalidPrompt { message } => {
                assert!(message.contains("unsupported message type"));
                assert!(message.contains("function call"));
            }
            _ => panic!("Expected InvalidPrompt"),
        }
    }
}
