use crate::error::AISDKError;
use crate::generate_image::ImageModelResponseMetadata;

/// Builder for [`AISDKError::NoImageGenerated`].
///
/// # Examples
///
/// ```
/// use ai_sdk_core::error::{AISDKError, NoImageGeneratedErrorBuilder};
///
/// let error = NoImageGeneratedErrorBuilder::new()
///     .message("The model failed to generate any images")
///     .build();
///
/// match error {
///     AISDKError::NoImageGenerated { message, responses } => {
///         assert_eq!(message, "The model failed to generate any images");
///         assert!(responses.is_none());
///     }
///     _ => panic!("Expected NoImageGenerated"),
/// }
/// ```
#[derive(Debug, Clone, Default)]
pub struct NoImageGeneratedErrorBuilder {
    message: Option<String>,
    responses: Option<Vec<ImageModelResponseMetadata>>,
}

impl NoImageGeneratedErrorBuilder {
    /// Creates a new builder for a no image generated error.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::NoImageGeneratedErrorBuilder;
    ///
    /// let builder = NoImageGeneratedErrorBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the error message.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of why no image was generated
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::NoImageGeneratedErrorBuilder;
    ///
    /// let builder = NoImageGeneratedErrorBuilder::new()
    ///     .message("Model failed to generate a response");
    /// ```
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Sets the response metadata from the provider calls.
    ///
    /// # Arguments
    ///
    /// * `responses` - Response metadata for each call attempt
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::NoImageGeneratedErrorBuilder;
    /// use ai_sdk_core::generate_image::ImageModelResponseMetadata;
    ///
    /// let metadata = ImageModelResponseMetadata::new("dall-e-3");
    /// let builder = NoImageGeneratedErrorBuilder::new()
    ///     .responses(vec![metadata]);
    /// ```
    pub fn responses(mut self, responses: Vec<ImageModelResponseMetadata>) -> Self {
        self.responses = Some(responses);
        self
    }

    /// Adds a single response metadata to the list.
    ///
    /// # Arguments
    ///
    /// * `response` - Response metadata to add
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::NoImageGeneratedErrorBuilder;
    /// use ai_sdk_core::generate_image::ImageModelResponseMetadata;
    ///
    /// let metadata = ImageModelResponseMetadata::new("dall-e-3");
    /// let builder = NoImageGeneratedErrorBuilder::new()
    ///     .add_response(metadata);
    /// ```
    pub fn add_response(mut self, response: ImageModelResponseMetadata) -> Self {
        self.responses.get_or_insert_with(Vec::new).push(response);
        self
    }

    /// Builds the [`AISDKError::NoImageGenerated`] error.
    ///
    /// If no custom message is provided, defaults to "No image generated."
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::NoImageGeneratedErrorBuilder;
    ///
    /// let error = NoImageGeneratedErrorBuilder::new()
    ///     .message("Model failed to generate response")
    ///     .build();
    /// ```
    pub fn build(self) -> AISDKError {
        AISDKError::NoImageGenerated {
            message: self
                .message
                .unwrap_or_else(|| "No image generated.".to_string()),
            responses: self.responses,
        }
    }
}

impl AISDKError {
    /// Creates a new no image generated error with default message.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::AISDKError;
    ///
    /// let error = AISDKError::no_image_generated();
    ///
    /// match error {
    ///     AISDKError::NoImageGenerated { message, responses } => {
    ///         assert_eq!(message, "No image generated.");
    ///         assert!(responses.is_none());
    ///     }
    ///     _ => panic!("Expected NoImageGenerated"),
    /// }
    /// ```
    pub fn no_image_generated() -> Self {
        Self::NoImageGenerated {
            message: "No image generated.".to_string(),
            responses: None,
        }
    }

    /// Creates a new no image generated error with a custom message.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of why no image was generated
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::AISDKError;
    ///
    /// let error = AISDKError::no_image_generated_with_message("Model failed to generate response");
    ///
    /// match error {
    ///     AISDKError::NoImageGenerated { message, responses } => {
    ///         assert_eq!(message, "Model failed to generate response");
    ///         assert!(responses.is_none());
    ///     }
    ///     _ => panic!("Expected NoImageGenerated"),
    /// }
    /// ```
    pub fn no_image_generated_with_message(message: impl Into<String>) -> Self {
        Self::NoImageGenerated {
            message: message.into(),
            responses: None,
        }
    }

    /// Creates a new no image generated error with response metadata.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of why no image was generated
    /// * `responses` - Response metadata from provider calls
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::AISDKError;
    /// use ai_sdk_core::generate_image::ImageModelResponseMetadata;
    ///
    /// let metadata = ImageModelResponseMetadata::new("dall-e-3");
    /// let error = AISDKError::no_image_generated_with_responses(
    ///     "Model returned empty response",
    ///     vec![metadata]
    /// );
    ///
    /// match error {
    ///     AISDKError::NoImageGenerated { message, responses } => {
    ///         assert_eq!(message, "Model returned empty response");
    ///         assert!(responses.is_some());
    ///     }
    ///     _ => panic!("Expected NoImageGenerated"),
    /// }
    /// ```
    pub fn no_image_generated_with_responses(
        message: impl Into<String>,
        responses: Vec<ImageModelResponseMetadata>,
    ) -> Self {
        Self::NoImageGenerated {
            message: message.into(),
            responses: Some(responses),
        }
    }

    /// Creates a builder for a no image generated error.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::AISDKError;
    /// use ai_sdk_core::generate_image::ImageModelResponseMetadata;
    ///
    /// let metadata = ImageModelResponseMetadata::new("dall-e-3");
    /// let error = AISDKError::no_image_generated_builder()
    ///     .message("Model failed to generate")
    ///     .add_response(metadata)
    ///     .build();
    /// ```
    pub fn no_image_generated_builder() -> NoImageGeneratedErrorBuilder {
        NoImageGeneratedErrorBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_image_generated_default() {
        let error = AISDKError::no_image_generated();

        match error {
            AISDKError::NoImageGenerated { message, responses } => {
                assert_eq!(message, "No image generated.");
                assert!(responses.is_none());
            }
            _ => panic!("Expected NoImageGenerated"),
        }
    }

    #[test]
    fn test_no_image_generated_with_message() {
        let error =
            AISDKError::no_image_generated_with_message("Model failed to generate response");

        match error {
            AISDKError::NoImageGenerated { message, responses } => {
                assert_eq!(message, "Model failed to generate response");
                assert!(responses.is_none());
            }
            _ => panic!("Expected NoImageGenerated"),
        }
    }

    #[test]
    fn test_no_image_generated_with_responses() {
        let metadata1 = ImageModelResponseMetadata::new("dall-e-3");
        let metadata2 = ImageModelResponseMetadata::new("dall-e-3");

        let error = AISDKError::no_image_generated_with_responses(
            "No images were generated",
            vec![metadata1, metadata2],
        );

        match error {
            AISDKError::NoImageGenerated { message, responses } => {
                assert_eq!(message, "No images were generated");
                assert!(responses.is_some());
                assert_eq!(responses.unwrap().len(), 2);
            }
            _ => panic!("Expected NoImageGenerated"),
        }
    }

    #[test]
    fn test_no_image_generated_builder() {
        let error = NoImageGeneratedErrorBuilder::new()
            .message("Custom error message")
            .build();

        match error {
            AISDKError::NoImageGenerated { message, responses } => {
                assert_eq!(message, "Custom error message");
                assert!(responses.is_none());
            }
            _ => panic!("Expected NoImageGenerated"),
        }
    }

    #[test]
    fn test_no_image_generated_builder_via_error() {
        let metadata = ImageModelResponseMetadata::new("dall-e-3");

        let error = AISDKError::no_image_generated_builder()
            .message("Model parsing failed")
            .add_response(metadata)
            .build();

        match error {
            AISDKError::NoImageGenerated { message, responses } => {
                assert_eq!(message, "Model parsing failed");
                assert!(responses.is_some());
                assert_eq!(responses.unwrap().len(), 1);
            }
            _ => panic!("Expected NoImageGenerated"),
        }
    }

    #[test]
    fn test_no_image_generated_builder_default_message() {
        let error = NoImageGeneratedErrorBuilder::new().build();

        match error {
            AISDKError::NoImageGenerated { message, responses } => {
                assert_eq!(message, "No image generated.");
                assert!(responses.is_none());
            }
            _ => panic!("Expected NoImageGenerated"),
        }
    }

    #[test]
    fn test_no_image_generated_builder_multiple_responses() {
        let metadata1 = ImageModelResponseMetadata::new("model-1");
        let metadata2 = ImageModelResponseMetadata::new("model-2");
        let metadata3 = ImageModelResponseMetadata::new("model-3");

        let error = NoImageGeneratedErrorBuilder::new()
            .message("Failed after multiple retries")
            .add_response(metadata1)
            .add_response(metadata2)
            .add_response(metadata3)
            .build();

        match error {
            AISDKError::NoImageGenerated { message, responses } => {
                assert_eq!(message, "Failed after multiple retries");
                let resp = responses.unwrap();
                assert_eq!(resp.len(), 3);
                assert_eq!(resp[0].model_id, "model-1");
                assert_eq!(resp[1].model_id, "model-2");
                assert_eq!(resp[2].model_id, "model-3");
            }
            _ => panic!("Expected NoImageGenerated"),
        }
    }

    #[test]
    fn test_no_image_generated_display() {
        let error = AISDKError::no_image_generated_with_message("Test error");
        let display = format!("{}", error);
        assert!(display.contains("No image generated"));
        assert!(display.contains("Test error"));
    }

    #[test]
    fn test_no_image_generated_builder_replace_responses() {
        let metadata1 = ImageModelResponseMetadata::new("model-1");
        let metadata2 = ImageModelResponseMetadata::new("model-2");
        let metadata3 = ImageModelResponseMetadata::new("model-3");

        let error = NoImageGeneratedErrorBuilder::new()
            .add_response(metadata1)
            .responses(vec![metadata2, metadata3])
            .build();

        match error {
            AISDKError::NoImageGenerated { responses, .. } => {
                let resp = responses.unwrap();
                // responses() should replace previous add_response()
                assert_eq!(resp.len(), 2);
                assert_eq!(resp[0].model_id, "model-2");
                assert_eq!(resp[1].model_id, "model-3");
            }
            _ => panic!("Expected NoImageGenerated"),
        }
    }
}
