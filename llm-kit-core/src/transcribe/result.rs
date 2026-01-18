use llm_kit_provider::shared::provider_metadata::SharedProviderMetadata;
use llm_kit_provider::transcription_model::call_warning::TranscriptionModelCallWarning;
use llm_kit_provider::transcription_model::{
    TranscriptSegment, TranscriptionModelResponseMetadata,
};
use serde::{Deserialize, Serialize};

/// The result of a `transcribe` call.
/// It contains the transcript and additional information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptionResult {
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

    /// Response metadata from the provider. There may be multiple responses
    /// if we made multiple calls to the model.
    pub responses: Vec<TranscriptionModelResponseMetadata>,

    /// Provider-specific metadata. They are passed through from the provider
    /// to the LLM Kit and enable provider-specific results that can be fully
    /// encapsulated in the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,
}

impl TranscriptionResult {
    /// Create a new transcription result.
    ///
    /// # Arguments
    ///
    /// * `text` - The complete transcribed text
    /// * `responses` - Response metadata from provider calls
    ///
    /// # Returns
    ///
    /// A new `TranscriptionResult` instance
    pub fn new(
        text: impl Into<String>,
        responses: Vec<TranscriptionModelResponseMetadata>,
    ) -> Self {
        Self {
            text: text.into(),
            segments: Vec::new(),
            language: None,
            duration_in_seconds: None,
            warnings: Vec::new(),
            responses,
            provider_metadata: None,
        }
    }

    /// Add segments to the result.
    pub fn with_segments(mut self, segments: Vec<TranscriptSegment>) -> Self {
        self.segments = segments;
        self
    }

    /// Add a single segment to the result.
    pub fn with_segment(mut self, segment: TranscriptSegment) -> Self {
        self.segments.push(segment);
        self
    }

    /// Add language to the result.
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Add duration to the result.
    pub fn with_duration(mut self, duration_in_seconds: f64) -> Self {
        self.duration_in_seconds = Some(duration_in_seconds);
        self
    }

    /// Add warnings to the result.
    pub fn with_warnings(mut self, warnings: Vec<TranscriptionModelCallWarning>) -> Self {
        self.warnings = warnings;
        self
    }

    /// Add a single warning to the result.
    pub fn with_warning(mut self, warning: TranscriptionModelCallWarning) -> Self {
        self.warnings.push(warning);
        self
    }

    /// Add provider metadata to the result.
    pub fn with_provider_metadata(mut self, metadata: SharedProviderMetadata) -> Self {
        self.provider_metadata = Some(metadata);
        self
    }

    /// Get the total duration from segments if not explicitly set.
    ///
    /// Returns the duration from the result if set, otherwise calculates it
    /// from the last segment's end time.
    pub fn get_duration(&self) -> Option<f64> {
        self.duration_in_seconds
            .or_else(|| self.segments.last().map(|segment| segment.end_second))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_transcription_result_new() {
        let responses = vec![TranscriptionModelResponseMetadata::new("whisper-1")];
        let result = TranscriptionResult::new("Hello world", responses);

        assert_eq!(result.text, "Hello world");
        assert!(result.segments.is_empty());
        assert!(result.language.is_none());
        assert!(result.duration_in_seconds.is_none());
        assert!(result.warnings.is_empty());
        assert_eq!(result.responses.len(), 1);
        assert!(result.provider_metadata.is_none());
    }

    #[test]
    fn test_transcription_result_with_segments() {
        let responses = vec![TranscriptionModelResponseMetadata::new("whisper-1")];
        let segments = vec![
            TranscriptSegment::new("Hello", 0.0, 1.0),
            TranscriptSegment::new("world", 1.0, 2.0),
        ];

        let result = TranscriptionResult::new("Hello world", responses).with_segments(segments);

        assert_eq!(result.segments.len(), 2);
        assert_eq!(result.segments[0].text, "Hello");
        assert_eq!(result.segments[1].text, "world");
    }

    #[test]
    fn test_transcription_result_with_segment() {
        let responses = vec![TranscriptionModelResponseMetadata::new("whisper-1")];
        let segment = TranscriptSegment::new("Hello", 0.0, 1.0);

        let result = TranscriptionResult::new("Hello", responses).with_segment(segment);

        assert_eq!(result.segments.len(), 1);
        assert_eq!(result.segments[0].text, "Hello");
    }

    #[test]
    fn test_transcription_result_with_language() {
        let responses = vec![TranscriptionModelResponseMetadata::new("whisper-1")];
        let result = TranscriptionResult::new("Hello", responses).with_language("en");

        assert_eq!(result.language, Some("en".to_string()));
    }

    #[test]
    fn test_transcription_result_with_duration() {
        let responses = vec![TranscriptionModelResponseMetadata::new("whisper-1")];
        let result = TranscriptionResult::new("Hello", responses).with_duration(5.5);

        assert_eq!(result.duration_in_seconds, Some(5.5));
    }

    #[test]
    fn test_transcription_result_with_warnings() {
        let responses = vec![TranscriptionModelResponseMetadata::new("whisper-1")];
        let warnings = vec![
            TranscriptionModelCallWarning::unsupported_setting("prompt"),
            TranscriptionModelCallWarning::unsupported_setting("temperature"),
        ];

        let result = TranscriptionResult::new("Hello", responses).with_warnings(warnings);

        assert_eq!(result.warnings.len(), 2);
    }

    #[test]
    fn test_transcription_result_with_warning() {
        let responses = vec![TranscriptionModelResponseMetadata::new("whisper-1")];
        let warning = TranscriptionModelCallWarning::unsupported_setting("prompt");

        let result = TranscriptionResult::new("Hello", responses).with_warning(warning);

        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_transcription_result_with_provider_metadata() {
        let responses = vec![TranscriptionModelResponseMetadata::new("whisper-1")];

        let mut metadata = HashMap::new();
        let mut openai_data = HashMap::new();
        openai_data.insert("model".to_string(), serde_json::json!("whisper-1"));
        metadata.insert("openai".to_string(), openai_data);

        let result =
            TranscriptionResult::new("Hello", responses).with_provider_metadata(metadata.clone());

        assert!(result.provider_metadata.is_some());
        assert_eq!(result.provider_metadata.unwrap(), metadata);
    }

    #[test]
    fn test_transcription_result_builder_pattern() {
        let responses = vec![TranscriptionModelResponseMetadata::new("whisper-1")];
        let segments = vec![
            TranscriptSegment::new("Hello", 0.0, 1.0),
            TranscriptSegment::new("world", 1.0, 2.0),
        ];
        let warnings = vec![TranscriptionModelCallWarning::unsupported_setting("prompt")];

        let mut metadata = HashMap::new();
        let mut openai_data = HashMap::new();
        openai_data.insert("test".to_string(), serde_json::json!("value"));
        metadata.insert("openai".to_string(), openai_data);

        let result = TranscriptionResult::new("Hello world", responses)
            .with_segments(segments)
            .with_language("en")
            .with_duration(2.0)
            .with_warnings(warnings)
            .with_provider_metadata(metadata.clone());

        assert_eq!(result.text, "Hello world");
        assert_eq!(result.segments.len(), 2);
        assert_eq!(result.language, Some("en".to_string()));
        assert_eq!(result.duration_in_seconds, Some(2.0));
        assert_eq!(result.warnings.len(), 1);
        assert!(result.provider_metadata.is_some());
    }

    #[test]
    fn test_transcription_result_serialization() {
        let responses = vec![TranscriptionModelResponseMetadata::new("whisper-1")];
        let result = TranscriptionResult::new("Hello", responses);
        let json = serde_json::to_value(&result).unwrap();

        assert!(json.get("text").is_some());
        assert!(json.get("segments").is_some());
        assert!(json.get("responses").is_some());
        // Optional fields should be omitted when None/empty
        assert!(json.get("language").is_none());
        assert!(json.get("durationInSeconds").is_none());
        assert!(json.get("warnings").is_none());
        assert!(json.get("providerMetadata").is_none());
    }

    #[test]
    fn test_transcription_result_serialization_with_all_fields() {
        let responses = vec![TranscriptionModelResponseMetadata::new("whisper-1")];
        let segments = vec![TranscriptSegment::new("Hello", 0.0, 1.0)];
        let warnings = vec![TranscriptionModelCallWarning::unsupported_setting("prompt")];

        let mut metadata = HashMap::new();
        let mut openai_data = HashMap::new();
        openai_data.insert("test".to_string(), serde_json::json!("value"));
        metadata.insert("openai".to_string(), openai_data);

        let result = TranscriptionResult::new("Hello", responses)
            .with_segments(segments)
            .with_language("en")
            .with_duration(1.0)
            .with_warnings(warnings)
            .with_provider_metadata(metadata);

        let json = serde_json::to_value(&result).unwrap();

        assert!(json.get("text").is_some());
        assert!(json.get("segments").is_some());
        assert!(json.get("language").is_some());
        assert!(json.get("durationInSeconds").is_some());
        assert!(json.get("warnings").is_some());
        assert!(json.get("providerMetadata").is_some());
    }

    #[test]
    fn test_transcription_result_round_trip() {
        let responses = vec![TranscriptionModelResponseMetadata::new("whisper-1")];
        let segments = vec![
            TranscriptSegment::new("Hello", 0.0, 1.0),
            TranscriptSegment::new("world", 1.0, 2.0),
        ];

        let result = TranscriptionResult::new("Hello world", responses)
            .with_segments(segments)
            .with_language("en")
            .with_duration(2.0);

        // Serialize and deserialize
        let json = serde_json::to_value(&result).unwrap();
        let deserialized: TranscriptionResult = serde_json::from_value(json).unwrap();

        assert_eq!(deserialized.text, "Hello world");
        assert_eq!(deserialized.segments.len(), 2);
        assert_eq!(deserialized.language, Some("en".to_string()));
        assert_eq!(deserialized.duration_in_seconds, Some(2.0));
    }

    #[test]
    fn test_get_duration_explicit() {
        let responses = vec![TranscriptionModelResponseMetadata::new("whisper-1")];
        let result = TranscriptionResult::new("Hello", responses).with_duration(5.5);

        assert_eq!(result.get_duration(), Some(5.5));
    }

    #[test]
    fn test_get_duration_from_segments() {
        let responses = vec![TranscriptionModelResponseMetadata::new("whisper-1")];
        let segments = vec![
            TranscriptSegment::new("Hello", 0.0, 1.0),
            TranscriptSegment::new("world", 1.0, 2.5),
        ];

        let result = TranscriptionResult::new("Hello world", responses).with_segments(segments);

        // Should use the last segment's end time
        assert_eq!(result.get_duration(), Some(2.5));
    }

    #[test]
    fn test_get_duration_none() {
        let responses = vec![TranscriptionModelResponseMetadata::new("whisper-1")];
        let result = TranscriptionResult::new("Hello", responses);

        assert_eq!(result.get_duration(), None);
    }

    #[test]
    fn test_get_duration_prefers_explicit() {
        let responses = vec![TranscriptionModelResponseMetadata::new("whisper-1")];
        let segments = vec![TranscriptSegment::new("Hello", 0.0, 2.5)];

        let result = TranscriptionResult::new("Hello", responses)
            .with_segments(segments)
            .with_duration(3.0);

        // Should prefer the explicitly set duration over segment calculation
        assert_eq!(result.get_duration(), Some(3.0));
    }

    #[test]
    fn test_multiple_responses() {
        let responses = vec![
            TranscriptionModelResponseMetadata::new("whisper-1"),
            TranscriptionModelResponseMetadata::new("whisper-1"),
        ];

        let result = TranscriptionResult::new("Hello", responses);

        assert_eq!(result.responses.len(), 2);
    }

    #[test]
    fn test_multiple_warnings() {
        let responses = vec![TranscriptionModelResponseMetadata::new("whisper-1")];
        let result = TranscriptionResult::new("Hello", responses)
            .with_warning(TranscriptionModelCallWarning::unsupported_setting("prompt"))
            .with_warning(TranscriptionModelCallWarning::unsupported_setting(
                "temperature",
            ))
            .with_warning(TranscriptionModelCallWarning::other("Custom warning"));

        assert_eq!(result.warnings.len(), 3);
    }

    #[test]
    fn test_language_codes() {
        let responses = vec![TranscriptionModelResponseMetadata::new("whisper-1")];

        // Test various ISO-639-1 codes
        let languages = vec!["en", "es", "fr", "de", "ja", "zh"];

        for lang in languages {
            let result = TranscriptionResult::new("Test", responses.clone()).with_language(lang);
            assert_eq!(result.language, Some(lang.to_string()));
        }
    }

    #[test]
    fn test_segment_timing() {
        let responses = vec![TranscriptionModelResponseMetadata::new("whisper-1")];
        let segments = vec![
            TranscriptSegment::new("First", 0.0, 1.5),
            TranscriptSegment::new("Second", 1.5, 3.2),
            TranscriptSegment::new("Third", 3.2, 5.0),
        ];

        let result = TranscriptionResult::new("First Second Third", responses)
            .with_segments(segments.clone());

        assert_eq!(result.segments.len(), 3);
        assert_eq!(result.segments[0].start_second, 0.0);
        assert_eq!(result.segments[0].end_second, 1.5);
        assert_eq!(result.segments[2].end_second, 5.0);
    }

    #[test]
    fn test_provider_metadata_structure() {
        let responses = vec![TranscriptionModelResponseMetadata::new("whisper-1")];

        let mut metadata = HashMap::new();
        let mut openai_data = HashMap::new();
        openai_data.insert("model".to_string(), serde_json::json!("whisper-1"));
        openai_data.insert("language".to_string(), serde_json::json!("en"));
        openai_data.insert("duration".to_string(), serde_json::json!(5.5));
        metadata.insert("openai".to_string(), openai_data);

        let result = TranscriptionResult::new("Hello", responses).with_provider_metadata(metadata);

        let provider_data = result.provider_metadata.unwrap();
        let openai = provider_data.get("openai").unwrap();
        assert_eq!(
            openai.get("model").unwrap(),
            &serde_json::json!("whisper-1")
        );
        assert_eq!(openai.get("language").unwrap(), &serde_json::json!("en"));
        assert_eq!(openai.get("duration").unwrap(), &serde_json::json!(5.5));
    }
}
