use crate::shared::headers::SharedHeaders;
use crate::shared::provider_metadata::SharedProviderMetadata;
use crate::transcription_model::call_options::TranscriptionModelCallOptions;
use crate::transcription_model::call_warning::TranscriptionModelCallWarning;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::SystemTime;

/// Options for calling transcription models
pub mod call_options;
/// Warnings that can be emitted during transcription model calls
pub mod call_warning;

/// Audio transcription model trait.
///
/// This trait defines the interface for speech-to-text (STT) models. Implementations
/// handle converting audio input into transcribed text using provider-specific APIs.
///
/// # Specification Version
///
/// This implements version 3 of the transcription model interface, which supports:
/// - Audio-to-text transcription
/// - Multiple audio formats (MP3, WAV, M4A, etc.)
/// - Language detection
/// - Timestamped segments
/// - Duration information
///
/// # Examples
///
/// ```no_run
/// use ai_sdk_provider::TranscriptionModel;
/// use ai_sdk_provider::transcription_model::call_options::TranscriptionModelCallOptions;
/// use ai_sdk_provider::transcription_model::TranscriptionModelResponse;
/// use async_trait::async_trait;
///
/// struct MyTranscriptionModel {
///     model_id: String,
///     api_key: String,
/// }
///
/// #[async_trait]
/// impl TranscriptionModel for MyTranscriptionModel {
///     fn provider(&self) -> &str {
///         "my-provider"
///     }
///
///     fn model_id(&self) -> &str {
///         &self.model_id
///     }
///
///     async fn do_generate(
///         &self,
///         options: TranscriptionModelCallOptions,
///     ) -> Result<TranscriptionModelResponse, Box<dyn std::error::Error>> {
///         // Implementation
///         todo!()
///     }
/// }
/// ```
#[async_trait]
pub trait TranscriptionModel: Send + Sync {
    /// Returns the specification version this model implements.
    ///
    /// Defaults to "v3" for the current SDK version. This allows us to evolve
    /// the transcription model interface and retain backwards compatibility.
    fn specification_version(&self) -> &str {
        "v3"
    }

    /// Name of the provider for logging purposes.
    ///
    /// Examples: "openai", "assemblyai", "deepgram"
    fn provider(&self) -> &str;

    /// Provider-specific model ID for logging purposes.
    ///
    /// Examples: "whisper-1", "nova-2", "base"
    fn model_id(&self) -> &str;

    /// Transcribes audio to text.
    ///
    /// # Arguments
    ///
    /// * `options` - Call options including audio data, language, format, etc.
    ///
    /// # Returns
    ///
    /// A [`TranscriptionModelResponse`] containing:
    /// - Transcribed text
    /// - Timestamped segments
    /// - Detected language
    /// - Audio duration
    /// - Provider-specific metadata
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The API request fails
    /// - The audio format is unsupported
    /// - The audio is too long or corrupted
    /// - Transcription fails
    async fn do_generate(
        &self,
        options: TranscriptionModelCallOptions,
    ) -> Result<TranscriptionModelResponse, Box<dyn std::error::Error>>;
}

/// Response from a transcription model.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptionModelResponse {
    /// The complete transcribed text from the audio.
    pub text: String,

    /// Array of transcript segments with timing information.
    /// Each segment represents a portion of the transcribed text with start and end times.
    pub segments: Vec<TranscriptSegment>,

    /// The detected language of the audio content, as an ISO-639-1 code (e.g., 'en' for English).
    /// May be None if the language couldn't be detected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// The total duration of the audio file in seconds.
    /// May be None if the duration couldn't be determined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<f64>,

    /// Warnings for the call, e.g. unsupported settings.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub warnings: Vec<TranscriptionModelCallWarning>,

    /// Optional request information for telemetry and debugging purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<TranscriptionModelRequestMetadata>,

    /// Response information for telemetry and debugging purposes.
    pub response: TranscriptionModelResponseMetadata,

    /// Additional provider-specific metadata. They are passed through
    /// from the provider to the AI SDK and enable provider-specific
    /// results that can be fully encapsulated in the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,
}

/// A segment of transcribed text with timing information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptSegment {
    /// The text content of this segment.
    pub text: String,

    /// The start time of this segment in seconds.
    pub start_second: f64,

    /// The end time of this segment in seconds.
    pub end_second: f64,
}

impl TranscriptSegment {
    /// Create a new transcript segment.
    pub fn new(text: impl Into<String>, start_second: f64, end_second: f64) -> Self {
        Self {
            text: text.into(),
            start_second,
            end_second,
        }
    }

    /// Get the duration of this segment in seconds.
    pub fn duration(&self) -> f64 {
        self.end_second - self.start_second
    }
}

/// Request metadata for telemetry and debugging purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptionModelRequestMetadata {
    /// Raw request HTTP body that was sent to the provider API as a string
    /// (JSON should be stringified).
    /// Non-HTTP(s) providers should not set this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

/// Response metadata for telemetry and debugging purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptionModelResponseMetadata {
    /// Timestamp for the start of the generated response.
    #[serde(with = "system_time_as_timestamp")]
    pub timestamp: SystemTime,

    /// The ID of the response model that was used to generate the response.
    pub model_id: String,

    /// Response headers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<SharedHeaders>,

    /// Response body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Value>,
}

impl TranscriptionModelResponse {
    /// Create a new response with text and response metadata.
    pub fn new(text: impl Into<String>, response: TranscriptionModelResponseMetadata) -> Self {
        Self {
            text: text.into(),
            segments: Vec::new(),
            language: None,
            duration_in_seconds: None,
            warnings: Vec::new(),
            request: None,
            response,
            provider_metadata: None,
        }
    }

    /// Add segments to the response.
    pub fn with_segments(mut self, segments: Vec<TranscriptSegment>) -> Self {
        self.segments = segments;
        self
    }

    /// Add a single segment to the response.
    pub fn with_segment(mut self, segment: TranscriptSegment) -> Self {
        self.segments.push(segment);
        self
    }

    /// Add language to the response.
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Add duration to the response.
    pub fn with_duration(mut self, duration_in_seconds: f64) -> Self {
        self.duration_in_seconds = Some(duration_in_seconds);
        self
    }

    /// Add warnings to the response.
    pub fn with_warnings(mut self, warnings: Vec<TranscriptionModelCallWarning>) -> Self {
        self.warnings = warnings;
        self
    }

    /// Add a single warning to the response.
    pub fn with_warning(mut self, warning: TranscriptionModelCallWarning) -> Self {
        self.warnings.push(warning);
        self
    }

    /// Add request metadata to the response.
    pub fn with_request_metadata(mut self, metadata: TranscriptionModelRequestMetadata) -> Self {
        self.request = Some(metadata);
        self
    }

    /// Add provider metadata to the response.
    pub fn with_provider_metadata(mut self, metadata: SharedProviderMetadata) -> Self {
        self.provider_metadata = Some(metadata);
        self
    }
}

impl TranscriptionModelRequestMetadata {
    /// Create new request metadata.
    pub fn new() -> Self {
        Self { body: None }
    }

    /// Add body to the request metadata.
    pub fn with_body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
}

impl Default for TranscriptionModelRequestMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl TranscriptionModelResponseMetadata {
    /// Create new response metadata.
    pub fn new(model_id: impl Into<String>) -> Self {
        Self {
            timestamp: SystemTime::now(),
            model_id: model_id.into(),
            headers: None,
            body: None,
        }
    }

    /// Create response metadata with a specific timestamp.
    pub fn with_timestamp(model_id: impl Into<String>, timestamp: SystemTime) -> Self {
        Self {
            timestamp,
            model_id: model_id.into(),
            headers: None,
            body: None,
        }
    }

    /// Add headers to the response metadata.
    pub fn with_headers(mut self, headers: SharedHeaders) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Add body to the response metadata.
    pub fn with_body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }
}

// Serde module for SystemTime serialization
mod system_time_as_timestamp {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = time
            .duration_since(UNIX_EPOCH)
            .map_err(serde::ser::Error::custom)?;
        serializer.serialize_u64(duration.as_millis() as u64)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        Ok(UNIX_EPOCH + std::time::Duration::from_millis(millis))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transcript_segment() {
        let segment = TranscriptSegment::new("Hello world", 0.0, 2.5);
        assert_eq!(segment.text, "Hello world");
        assert_eq!(segment.start_second, 0.0);
        assert_eq!(segment.end_second, 2.5);
        assert_eq!(segment.duration(), 2.5);
    }

    #[test]
    fn test_segment_duration() {
        let segment = TranscriptSegment::new("Test", 1.5, 3.7);
        assert!((segment.duration() - 2.2).abs() < 0.0001);
    }

    #[test]
    fn test_response_builder() {
        let metadata = TranscriptionModelResponseMetadata::new("whisper-1");
        let segment = TranscriptSegment::new("Hello", 0.0, 1.0);

        let response = TranscriptionModelResponse::new("Hello world", metadata)
            .with_segment(segment)
            .with_language("en")
            .with_duration(5.0);

        assert_eq!(response.text, "Hello world");
        assert_eq!(response.segments.len(), 1);
        assert_eq!(response.language, Some("en".to_string()));
        assert_eq!(response.duration_in_seconds, Some(5.0));
    }

    #[test]
    fn test_response_with_segments() {
        let metadata = TranscriptionModelResponseMetadata::new("whisper-1");
        let segments = vec![
            TranscriptSegment::new("Hello", 0.0, 1.0),
            TranscriptSegment::new("world", 1.0, 2.0),
        ];

        let response = TranscriptionModelResponse::new("Hello world", metadata)
            .with_segments(segments.clone());

        assert_eq!(response.segments.len(), 2);
        assert_eq!(response.segments[0].text, "Hello");
        assert_eq!(response.segments[1].text, "world");
    }

    #[test]
    fn test_response_with_warning() {
        let metadata = TranscriptionModelResponseMetadata::new("whisper-1");
        let warning = TranscriptionModelCallWarning::other("Test warning");

        let response = TranscriptionModelResponse::new("Test", metadata).with_warning(warning);

        assert_eq!(response.warnings.len(), 1);
    }

    #[test]
    fn test_request_metadata() {
        let metadata = TranscriptionModelRequestMetadata::new().with_body(r#"{"key":"value"}"#);

        assert_eq!(metadata.body, Some(r#"{"key":"value"}"#.to_string()));
    }
}
