use crate::error::AISDKError;

/// Builder for [`AISDKError::ModelError`].
///
/// # Examples
///
/// ```
/// use ai_sdk_core::error::{AISDKError, ModelErrorBuilder};
///
/// let error = ModelErrorBuilder::new("Model initialization failed")
///     .build();
///
/// match error {
///     AISDKError::ModelError { message } => {
///         assert_eq!(message, "Model initialization failed");
///     }
///     _ => panic!("Expected ModelError"),
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ModelErrorBuilder {
    message: String,
}

impl ModelErrorBuilder {
    /// Creates a new builder for a model error.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of the model error
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::ModelErrorBuilder;
    ///
    /// let builder = ModelErrorBuilder::new("Model not found");
    /// ```
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Builds the [`AISDKError::ModelError`] error.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::ModelErrorBuilder;
    ///
    /// let error = ModelErrorBuilder::new("Model configuration invalid")
    ///     .build();
    /// ```
    pub fn build(self) -> AISDKError {
        AISDKError::ModelError {
            message: self.message,
        }
    }
}

impl AISDKError {
    /// Creates a new model error.
    ///
    /// This is a convenience method that creates an error with a message in one call.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of the model error
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::AISDKError;
    ///
    /// let error = AISDKError::model_error("Model not responding");
    ///
    /// match error {
    ///     AISDKError::ModelError { message } => {
    ///         assert_eq!(message, "Model not responding");
    ///     }
    ///     _ => panic!("Expected ModelError"),
    /// }
    /// ```
    pub fn model_error(message: impl Into<String>) -> Self {
        Self::ModelError {
            message: message.into(),
        }
    }

    /// Creates a builder for a model error.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of the model error
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::AISDKError;
    ///
    /// let error = AISDKError::model_error_builder("Model initialization failed")
    ///     .build();
    /// ```
    pub fn model_error_builder(message: impl Into<String>) -> ModelErrorBuilder {
        ModelErrorBuilder::new(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_error_simple() {
        let error = AISDKError::model_error("Model not found");

        match error {
            AISDKError::ModelError { message } => {
                assert_eq!(message, "Model not found");
            }
            _ => panic!("Expected ModelError"),
        }
    }

    #[test]
    fn test_model_error_builder() {
        let error = ModelErrorBuilder::new("Failed to load model weights").build();

        match error {
            AISDKError::ModelError { message } => {
                assert_eq!(message, "Failed to load model weights");
            }
            _ => panic!("Expected ModelError"),
        }
    }

    #[test]
    fn test_model_error_builder_via_error() {
        let error = AISDKError::model_error_builder("Model configuration is invalid").build();

        match error {
            AISDKError::ModelError { message } => {
                assert_eq!(message, "Model configuration is invalid");
            }
            _ => panic!("Expected ModelError"),
        }
    }

    #[test]
    fn test_model_error_display() {
        let error = AISDKError::model_error("Model timeout");
        let display = format!("{}", error);
        assert!(display.contains("Model error"));
        assert!(display.contains("Model timeout"));
    }

    #[test]
    fn test_model_error_with_details() {
        let error = AISDKError::model_error(
            "Model gpt-4 requires API key but none was provided in environment",
        );

        match error {
            AISDKError::ModelError { message } => {
                assert!(message.contains("gpt-4"));
                assert!(message.contains("API key"));
            }
            _ => panic!("Expected ModelError"),
        }
    }
}
