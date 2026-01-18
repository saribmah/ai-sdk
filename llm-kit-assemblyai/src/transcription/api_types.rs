use serde::{Deserialize, Serialize};

/// Response from the upload endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyAIUploadResponse {
    /// The URL where the audio was uploaded
    pub upload_url: String,
}

/// Response from the submit transcript endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyAISubmitResponse {
    /// The transcript ID
    pub id: String,
    /// The current status
    pub status: TranscriptionStatus,
}

/// Transcription status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TranscriptionStatus {
    /// Transcription is queued
    Queued,
    /// Transcription is processing
    Processing,
    /// Transcription is completed
    Completed,
    /// Transcription failed
    Error,
}

/// Response from the get transcript endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyAITranscriptionResponse {
    /// The transcript ID
    pub id: String,
    /// The current status
    pub status: TranscriptionStatus,
    /// The transcribed text (available when completed)
    pub text: Option<String>,
    /// Detected language code
    pub language_code: Option<String>,
    /// Words with timestamps
    pub words: Option<Vec<TranscriptWord>>,
    /// Audio duration in seconds
    pub audio_duration: Option<f64>,
    /// Error message (if status is error)
    pub error: Option<String>,
}

/// A word in the transcript with timing information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptWord {
    /// Start time in seconds
    pub start: f64,
    /// End time in seconds
    pub end: f64,
    /// The word text
    pub text: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_response_parsing() {
        let json = r#"{"upload_url": "https://cdn.assemblyai.com/upload/abc123"}"#;
        let response: AssemblyAIUploadResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            response.upload_url,
            "https://cdn.assemblyai.com/upload/abc123"
        );
    }

    #[test]
    fn test_submit_response_parsing() {
        let json = r#"{"id": "abc123", "status": "queued"}"#;
        let response: AssemblyAISubmitResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.id, "abc123");
        assert_eq!(response.status, TranscriptionStatus::Queued);
    }

    #[test]
    fn test_transcription_response_parsing() {
        let json = r#"{
            "id": "abc123",
            "status": "completed",
            "text": "Hello world",
            "language_code": "en",
            "words": [
                {"start": 0.0, "end": 0.5, "text": "Hello"},
                {"start": 0.5, "end": 1.0, "text": "world"}
            ],
            "audio_duration": 1.0
        }"#;

        let response: AssemblyAITranscriptionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.status, TranscriptionStatus::Completed);
        assert_eq!(response.text, Some("Hello world".to_string()));
        assert_eq!(response.words.as_ref().unwrap().len(), 2);
        assert_eq!(response.audio_duration, Some(1.0));
    }
}
