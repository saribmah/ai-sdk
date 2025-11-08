use crate::error::AISDKError;
use ai_sdk_provider::speech_model::SpeechModelResponseMetadata;

/// Builder for [`AISDKError::NoSpeechGenerated`].
///
/// # Examples
///
/// ```
/// use ai_sdk_core::error::{AISDKError, NoSpeechGeneratedErrorBuilder};
/// use ai_sdk_provider::speech_model::SpeechModelResponseMetadata;
///
/// let metadata = SpeechModelResponseMetadata::new("tts-1");
/// let error = NoSpeechGeneratedErrorBuilder::new(vec![metadata])
///     .build();
///
/// match error {
///     AISDKError::NoSpeechGenerated { message, responses } => {
///         assert_eq!(message, "No speech audio generated.");
///         assert_eq!(responses.len(), 1);
///     }
///     _ => panic!("Expected NoSpeechGenerated"),
/// }
/// ```
#[derive(Debug, Clone)]
pub struct NoSpeechGeneratedErrorBuilder {
    responses: Vec<SpeechModelResponseMetadata>,
}

impl NoSpeechGeneratedErrorBuilder {
    /// Creates a new builder for a no speech generated error.
    ///
    /// # Arguments
    ///
    /// * `responses` - Response metadata from provider calls
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::NoSpeechGeneratedErrorBuilder;
    /// use ai_sdk_provider::speech_model::SpeechModelResponseMetadata;
    ///
    /// let metadata = SpeechModelResponseMetadata::new("tts-1");
    /// let builder = NoSpeechGeneratedErrorBuilder::new(vec![metadata]);
    /// ```
    pub fn new(responses: Vec<SpeechModelResponseMetadata>) -> Self {
        Self { responses }
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
    /// use ai_sdk_core::error::NoSpeechGeneratedErrorBuilder;
    /// use ai_sdk_provider::speech_model::SpeechModelResponseMetadata;
    ///
    /// let metadata1 = SpeechModelResponseMetadata::new("tts-1");
    /// let metadata2 = SpeechModelResponseMetadata::new("tts-1");
    /// let builder = NoSpeechGeneratedErrorBuilder::new(vec![metadata1])
    ///     .add_response(metadata2);
    /// ```
    pub fn add_response(mut self, response: SpeechModelResponseMetadata) -> Self {
        self.responses.push(response);
        self
    }

    /// Builds the [`AISDKError::NoSpeechGenerated`] error.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::NoSpeechGeneratedErrorBuilder;
    /// use ai_sdk_provider::speech_model::SpeechModelResponseMetadata;
    ///
    /// let metadata = SpeechModelResponseMetadata::new("tts-1");
    /// let error = NoSpeechGeneratedErrorBuilder::new(vec![metadata])
    ///     .build();
    /// ```
    pub fn build(self) -> AISDKError {
        AISDKError::NoSpeechGenerated {
            message: "No speech audio generated.".to_string(),
            responses: self.responses,
        }
    }
}

impl AISDKError {
    /// Creates a new no speech generated error.
    ///
    /// # Arguments
    ///
    /// * `responses` - Response metadata from provider calls
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::AISDKError;
    /// use ai_sdk_provider::speech_model::SpeechModelResponseMetadata;
    ///
    /// let metadata = SpeechModelResponseMetadata::new("tts-1");
    /// let error = AISDKError::no_speech_generated(vec![metadata]);
    ///
    /// match error {
    ///     AISDKError::NoSpeechGenerated { message, responses } => {
    ///         assert_eq!(message, "No speech audio generated.");
    ///         assert_eq!(responses.len(), 1);
    ///     }
    ///     _ => panic!("Expected NoSpeechGenerated"),
    /// }
    /// ```
    pub fn no_speech_generated(responses: Vec<SpeechModelResponseMetadata>) -> Self {
        Self::NoSpeechGenerated {
            message: "No speech audio generated.".to_string(),
            responses,
        }
    }

    /// Creates a builder for a no speech generated error.
    ///
    /// # Arguments
    ///
    /// * `responses` - Response metadata from provider calls
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::AISDKError;
    /// use ai_sdk_provider::speech_model::SpeechModelResponseMetadata;
    ///
    /// let metadata = SpeechModelResponseMetadata::new("tts-1");
    /// let error = AISDKError::no_speech_generated_builder(vec![metadata])
    ///     .build();
    /// ```
    pub fn no_speech_generated_builder(
        responses: Vec<SpeechModelResponseMetadata>,
    ) -> NoSpeechGeneratedErrorBuilder {
        NoSpeechGeneratedErrorBuilder::new(responses)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_speech_generated_simple() {
        let metadata = SpeechModelResponseMetadata::new("tts-1");
        let error = AISDKError::no_speech_generated(vec![metadata]);

        match error {
            AISDKError::NoSpeechGenerated { message, responses } => {
                assert_eq!(message, "No speech audio generated.");
                assert_eq!(responses.len(), 1);
                assert_eq!(responses[0].model_id, "tts-1");
            }
            _ => panic!("Expected NoSpeechGenerated"),
        }
    }

    #[test]
    fn test_no_speech_generated_multiple_responses() {
        let metadata1 = SpeechModelResponseMetadata::new("tts-1");
        let metadata2 = SpeechModelResponseMetadata::new("tts-1-hd");

        let error = AISDKError::no_speech_generated(vec![metadata1, metadata2]);

        match error {
            AISDKError::NoSpeechGenerated { message, responses } => {
                assert_eq!(message, "No speech audio generated.");
                assert_eq!(responses.len(), 2);
                assert_eq!(responses[0].model_id, "tts-1");
                assert_eq!(responses[1].model_id, "tts-1-hd");
            }
            _ => panic!("Expected NoSpeechGenerated"),
        }
    }

    #[test]
    fn test_no_speech_generated_builder() {
        let metadata = SpeechModelResponseMetadata::new("tts-1");
        let error = NoSpeechGeneratedErrorBuilder::new(vec![metadata]).build();

        match error {
            AISDKError::NoSpeechGenerated { message, responses } => {
                assert_eq!(message, "No speech audio generated.");
                assert_eq!(responses.len(), 1);
            }
            _ => panic!("Expected NoSpeechGenerated"),
        }
    }

    #[test]
    fn test_no_speech_generated_builder_via_error() {
        let metadata = SpeechModelResponseMetadata::new("tts-1");
        let error = AISDKError::no_speech_generated_builder(vec![metadata]).build();

        match error {
            AISDKError::NoSpeechGenerated { message, responses } => {
                assert_eq!(message, "No speech audio generated.");
                assert_eq!(responses.len(), 1);
            }
            _ => panic!("Expected NoSpeechGenerated"),
        }
    }

    #[test]
    fn test_no_speech_generated_builder_add_response() {
        let metadata1 = SpeechModelResponseMetadata::new("tts-1");
        let metadata2 = SpeechModelResponseMetadata::new("tts-1-hd");
        let metadata3 = SpeechModelResponseMetadata::new("tts-2");

        let error = NoSpeechGeneratedErrorBuilder::new(vec![metadata1])
            .add_response(metadata2)
            .add_response(metadata3)
            .build();

        match error {
            AISDKError::NoSpeechGenerated { message, responses } => {
                assert_eq!(message, "No speech audio generated.");
                assert_eq!(responses.len(), 3);
                assert_eq!(responses[0].model_id, "tts-1");
                assert_eq!(responses[1].model_id, "tts-1-hd");
                assert_eq!(responses[2].model_id, "tts-2");
            }
            _ => panic!("Expected NoSpeechGenerated"),
        }
    }

    #[test]
    fn test_no_speech_generated_display() {
        let metadata = SpeechModelResponseMetadata::new("tts-1");
        let error = AISDKError::no_speech_generated(vec![metadata]);
        let display = format!("{}", error);
        assert!(display.contains("No speech audio generated"));
    }

    #[test]
    fn test_no_speech_generated_empty_responses() {
        let error = AISDKError::no_speech_generated(vec![]);

        match error {
            AISDKError::NoSpeechGenerated { message, responses } => {
                assert_eq!(message, "No speech audio generated.");
                assert_eq!(responses.len(), 0);
            }
            _ => panic!("Expected NoSpeechGenerated"),
        }
    }
}
