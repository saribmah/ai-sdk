use crate::error::AISDKError;
use ai_sdk_provider::transcription_model::TranscriptionModelResponseMetadata;

/// Builder for [`AISDKError::NoTranscriptGenerated`].
///
/// # Examples
///
/// ```
/// use ai_sdk_core::error::{AISDKError, NoTranscriptGeneratedErrorBuilder};
/// use ai_sdk_provider::transcription_model::TranscriptionModelResponseMetadata;
///
/// let metadata = TranscriptionModelResponseMetadata::new("whisper-1");
/// let error = NoTranscriptGeneratedErrorBuilder::new(vec![metadata])
///     .build();
///
/// match error {
///     AISDKError::NoTranscriptGenerated { message, responses } => {
///         assert_eq!(message, "No transcript generated.");
///         assert_eq!(responses.len(), 1);
///     }
///     _ => panic!("Expected NoTranscriptGenerated"),
/// }
/// ```
#[derive(Debug, Clone)]
pub struct NoTranscriptGeneratedErrorBuilder {
    responses: Vec<TranscriptionModelResponseMetadata>,
}

impl NoTranscriptGeneratedErrorBuilder {
    /// Creates a new builder for a no transcript generated error.
    ///
    /// # Arguments
    ///
    /// * `responses` - Response metadata from provider calls
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::NoTranscriptGeneratedErrorBuilder;
    /// use ai_sdk_provider::transcription_model::TranscriptionModelResponseMetadata;
    ///
    /// let metadata = TranscriptionModelResponseMetadata::new("whisper-1");
    /// let builder = NoTranscriptGeneratedErrorBuilder::new(vec![metadata]);
    /// ```
    pub fn new(responses: Vec<TranscriptionModelResponseMetadata>) -> Self {
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
    /// use ai_sdk_core::error::NoTranscriptGeneratedErrorBuilder;
    /// use ai_sdk_provider::transcription_model::TranscriptionModelResponseMetadata;
    ///
    /// let metadata1 = TranscriptionModelResponseMetadata::new("whisper-1");
    /// let metadata2 = TranscriptionModelResponseMetadata::new("whisper-1");
    /// let builder = NoTranscriptGeneratedErrorBuilder::new(vec![metadata1])
    ///     .add_response(metadata2);
    /// ```
    pub fn add_response(mut self, response: TranscriptionModelResponseMetadata) -> Self {
        self.responses.push(response);
        self
    }

    /// Builds the [`AISDKError::NoTranscriptGenerated`] error.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::NoTranscriptGeneratedErrorBuilder;
    /// use ai_sdk_provider::transcription_model::TranscriptionModelResponseMetadata;
    ///
    /// let metadata = TranscriptionModelResponseMetadata::new("whisper-1");
    /// let error = NoTranscriptGeneratedErrorBuilder::new(vec![metadata])
    ///     .build();
    /// ```
    pub fn build(self) -> AISDKError {
        AISDKError::NoTranscriptGenerated {
            message: "No transcript generated.".to_string(),
            responses: self.responses,
        }
    }
}

impl AISDKError {
    /// Creates a new no transcript generated error.
    ///
    /// # Arguments
    ///
    /// * `responses` - Response metadata from provider calls
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::AISDKError;
    /// use ai_sdk_provider::transcription_model::TranscriptionModelResponseMetadata;
    ///
    /// let metadata = TranscriptionModelResponseMetadata::new("whisper-1");
    /// let error = AISDKError::no_transcript_generated(vec![metadata]);
    ///
    /// match error {
    ///     AISDKError::NoTranscriptGenerated { message, responses } => {
    ///         assert_eq!(message, "No transcript generated.");
    ///         assert_eq!(responses.len(), 1);
    ///     }
    ///     _ => panic!("Expected NoTranscriptGenerated"),
    /// }
    /// ```
    pub fn no_transcript_generated(responses: Vec<TranscriptionModelResponseMetadata>) -> Self {
        Self::NoTranscriptGenerated {
            message: "No transcript generated.".to_string(),
            responses,
        }
    }

    /// Creates a builder for a no transcript generated error.
    ///
    /// # Arguments
    ///
    /// * `responses` - Response metadata from provider calls
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::error::AISDKError;
    /// use ai_sdk_provider::transcription_model::TranscriptionModelResponseMetadata;
    ///
    /// let metadata = TranscriptionModelResponseMetadata::new("whisper-1");
    /// let error = AISDKError::no_transcript_generated_builder(vec![metadata])
    ///     .build();
    /// ```
    pub fn no_transcript_generated_builder(
        responses: Vec<TranscriptionModelResponseMetadata>,
    ) -> NoTranscriptGeneratedErrorBuilder {
        NoTranscriptGeneratedErrorBuilder::new(responses)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_transcript_generated_simple() {
        let metadata = TranscriptionModelResponseMetadata::new("whisper-1");
        let error = AISDKError::no_transcript_generated(vec![metadata]);

        match error {
            AISDKError::NoTranscriptGenerated { message, responses } => {
                assert_eq!(message, "No transcript generated.");
                assert_eq!(responses.len(), 1);
                assert_eq!(responses[0].model_id, "whisper-1");
            }
            _ => panic!("Expected NoTranscriptGenerated"),
        }
    }

    #[test]
    fn test_no_transcript_generated_multiple_responses() {
        let metadata1 = TranscriptionModelResponseMetadata::new("whisper-1");
        let metadata2 = TranscriptionModelResponseMetadata::new("whisper-2");

        let error = AISDKError::no_transcript_generated(vec![metadata1, metadata2]);

        match error {
            AISDKError::NoTranscriptGenerated { message, responses } => {
                assert_eq!(message, "No transcript generated.");
                assert_eq!(responses.len(), 2);
                assert_eq!(responses[0].model_id, "whisper-1");
                assert_eq!(responses[1].model_id, "whisper-2");
            }
            _ => panic!("Expected NoTranscriptGenerated"),
        }
    }

    #[test]
    fn test_no_transcript_generated_builder() {
        let metadata = TranscriptionModelResponseMetadata::new("whisper-1");
        let error = NoTranscriptGeneratedErrorBuilder::new(vec![metadata]).build();

        match error {
            AISDKError::NoTranscriptGenerated { message, responses } => {
                assert_eq!(message, "No transcript generated.");
                assert_eq!(responses.len(), 1);
            }
            _ => panic!("Expected NoTranscriptGenerated"),
        }
    }

    #[test]
    fn test_no_transcript_generated_builder_via_error() {
        let metadata = TranscriptionModelResponseMetadata::new("whisper-1");
        let error = AISDKError::no_transcript_generated_builder(vec![metadata]).build();

        match error {
            AISDKError::NoTranscriptGenerated { message, responses } => {
                assert_eq!(message, "No transcript generated.");
                assert_eq!(responses.len(), 1);
            }
            _ => panic!("Expected NoTranscriptGenerated"),
        }
    }

    #[test]
    fn test_no_transcript_generated_builder_add_response() {
        let metadata1 = TranscriptionModelResponseMetadata::new("whisper-1");
        let metadata2 = TranscriptionModelResponseMetadata::new("whisper-2");
        let metadata3 = TranscriptionModelResponseMetadata::new("whisper-3");

        let error = NoTranscriptGeneratedErrorBuilder::new(vec![metadata1])
            .add_response(metadata2)
            .add_response(metadata3)
            .build();

        match error {
            AISDKError::NoTranscriptGenerated { message, responses } => {
                assert_eq!(message, "No transcript generated.");
                assert_eq!(responses.len(), 3);
                assert_eq!(responses[0].model_id, "whisper-1");
                assert_eq!(responses[1].model_id, "whisper-2");
                assert_eq!(responses[2].model_id, "whisper-3");
            }
            _ => panic!("Expected NoTranscriptGenerated"),
        }
    }

    #[test]
    fn test_no_transcript_generated_display() {
        let metadata = TranscriptionModelResponseMetadata::new("whisper-1");
        let error = AISDKError::no_transcript_generated(vec![metadata]);
        let display = format!("{}", error);
        assert!(display.contains("No transcript generated"));
    }

    #[test]
    fn test_no_transcript_generated_empty_responses() {
        let error = AISDKError::no_transcript_generated(vec![]);

        match error {
            AISDKError::NoTranscriptGenerated { message, responses } => {
                assert_eq!(message, "No transcript generated.");
                assert_eq!(responses.len(), 0);
            }
            _ => panic!("Expected NoTranscriptGenerated"),
        }
    }
}
