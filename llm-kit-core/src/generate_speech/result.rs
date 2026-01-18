use crate::generate_speech::GeneratedAudioFile;
use llm_kit_provider::shared::provider_metadata::SharedProviderMetadata;
use llm_kit_provider::speech_model::call_warning::SpeechModelCallWarning;
use llm_kit_provider::speech_model::{AudioData, SpeechModelResponseMetadata};
use serde::{Deserialize, Serialize};

/// The result of a `generate_speech` call.
/// It contains the audio data and additional information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateSpeechResult {
    /// The audio data.
    pub audio: GeneratedAudioFile,

    /// Warnings for the call, e.g. unsupported settings.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub warnings: Vec<SpeechModelCallWarning>,

    /// Response metadata from the provider. There may be multiple responses
    /// if we made multiple calls to the model.
    pub responses: Vec<SpeechModelResponseMetadata>,

    /// Provider-specific metadata. They are passed through from the provider
    /// to the LLM Kit and enable provider-specific results that can be fully
    /// encapsulated in the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,
}

impl GenerateSpeechResult {
    /// Create a new generate speech result.
    ///
    /// # Arguments
    ///
    /// * `audio` - The generated audio file
    /// * `responses` - Response metadata from provider calls
    ///
    /// # Returns
    ///
    /// A new `GenerateSpeechResult` instance
    pub fn new(audio: GeneratedAudioFile, responses: Vec<SpeechModelResponseMetadata>) -> Self {
        Self {
            audio,
            warnings: Vec::new(),
            responses,
            provider_metadata: None,
        }
    }

    /// Add warnings to the result.
    pub fn with_warnings(mut self, warnings: Vec<SpeechModelCallWarning>) -> Self {
        self.warnings = warnings;
        self
    }

    /// Add a single warning to the result.
    pub fn with_warning(mut self, warning: SpeechModelCallWarning) -> Self {
        self.warnings.push(warning);
        self
    }

    /// Add provider metadata to the result.
    pub fn with_provider_metadata(mut self, metadata: SharedProviderMetadata) -> Self {
        self.provider_metadata = Some(metadata);
        self
    }
}

/// Helper function to convert AudioData to GeneratedAudioFile.
///
/// # Arguments
///
/// * `audio_data` - The audio data from the provider
/// * `media_type` - The IANA media type of the audio
///
/// # Returns
///
/// A `GeneratedAudioFile` with the audio data
pub fn audio_data_to_generated_audio_file(
    audio_data: AudioData,
    media_type: impl Into<String>,
) -> GeneratedAudioFile {
    match audio_data {
        AudioData::Base64(base64) => GeneratedAudioFile::from_base64(base64, media_type),
        AudioData::Binary(bytes) => GeneratedAudioFile::from_bytes(bytes, media_type),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_generate_speech_result_new() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        let responses = vec![SpeechModelResponseMetadata::new("tts-1")];

        let result = GenerateSpeechResult::new(audio.clone(), responses);

        assert_eq!(result.audio.format, "mp3");
        assert!(result.warnings.is_empty());
        assert_eq!(result.responses.len(), 1);
        assert!(result.provider_metadata.is_none());
    }

    #[test]
    fn test_generate_speech_result_with_warnings() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        let responses = vec![SpeechModelResponseMetadata::new("tts-1")];

        let warnings = vec![
            SpeechModelCallWarning::unsupported_setting("speed"),
            SpeechModelCallWarning::unsupported_setting("language"),
        ];

        let result = GenerateSpeechResult::new(audio, responses).with_warnings(warnings.clone());

        assert_eq!(result.warnings.len(), 2);
    }

    #[test]
    fn test_generate_speech_result_with_warning() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        let responses = vec![SpeechModelResponseMetadata::new("tts-1")];

        let warning = SpeechModelCallWarning::unsupported_setting("speed");

        let result = GenerateSpeechResult::new(audio, responses).with_warning(warning);

        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_generate_speech_result_with_provider_metadata() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        let responses = vec![SpeechModelResponseMetadata::new("tts-1")];

        let mut metadata = HashMap::new();
        let mut openai_data = HashMap::new();
        openai_data.insert("voice".to_string(), serde_json::json!("alloy"));
        metadata.insert("openai".to_string(), openai_data);

        let result =
            GenerateSpeechResult::new(audio, responses).with_provider_metadata(metadata.clone());

        assert!(result.provider_metadata.is_some());
        assert_eq!(result.provider_metadata.unwrap(), metadata);
    }

    #[test]
    fn test_generate_speech_result_builder_pattern() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/wav");
        let responses = vec![SpeechModelResponseMetadata::new("tts-1")];

        let warnings = vec![SpeechModelCallWarning::unsupported_setting("speed")];

        let mut metadata = HashMap::new();
        let mut openai_data = HashMap::new();
        openai_data.insert("test".to_string(), serde_json::json!("value"));
        metadata.insert("openai".to_string(), openai_data);

        let result = GenerateSpeechResult::new(audio.clone(), responses)
            .with_warnings(warnings)
            .with_provider_metadata(metadata.clone());

        assert_eq!(result.audio.format, "wav");
        assert_eq!(result.warnings.len(), 1);
        assert!(result.provider_metadata.is_some());
    }

    #[test]
    fn test_audio_data_to_generated_audio_file_base64() {
        let audio_data = AudioData::Base64("SGVsbG8gV29ybGQh".to_string());
        let file = audio_data_to_generated_audio_file(audio_data, "audio/mp3");

        assert_eq!(file.media_type(), "audio/mp3");
        assert_eq!(file.format, "mp3");
        assert_eq!(file.base64(), "SGVsbG8gV29ybGQh");
    }

    #[test]
    fn test_audio_data_to_generated_audio_file_binary() {
        let audio_data = AudioData::Binary(b"Hello World!".to_vec());
        let file = audio_data_to_generated_audio_file(audio_data, "audio/wav");

        assert_eq!(file.media_type(), "audio/wav");
        assert_eq!(file.format, "wav");
        assert_eq!(file.bytes(), b"Hello World!");
    }

    #[test]
    fn test_generate_speech_result_serialization() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        let responses = vec![SpeechModelResponseMetadata::new("tts-1")];

        let result = GenerateSpeechResult::new(audio, responses);
        let json = serde_json::to_value(&result).unwrap();

        assert!(json.get("audio").is_some());
        assert!(json.get("responses").is_some());
        // warnings is empty so it should be omitted
        assert!(json.get("warnings").is_none());
        // providerMetadata is None so it should be omitted
        assert!(json.get("providerMetadata").is_none());
    }

    #[test]
    fn test_generate_speech_result_serialization_with_warnings() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        let responses = vec![SpeechModelResponseMetadata::new("tts-1")];
        let warnings = vec![SpeechModelCallWarning::unsupported_setting("speed")];

        let result = GenerateSpeechResult::new(audio, responses).with_warnings(warnings);
        let json = serde_json::to_value(&result).unwrap();

        assert!(json.get("warnings").is_some());
    }

    #[test]
    fn test_generate_speech_result_round_trip() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        let responses = vec![SpeechModelResponseMetadata::new("tts-1")];

        let result = GenerateSpeechResult::new(audio, responses);

        // Serialize and deserialize
        let json = serde_json::to_value(&result).unwrap();
        let deserialized: GenerateSpeechResult = serde_json::from_value(json).unwrap();

        assert_eq!(deserialized.audio.format, "mp3");
        assert_eq!(deserialized.responses.len(), 1);
        assert_eq!(deserialized.responses[0].model_id, "tts-1");
        assert!(deserialized.warnings.is_empty());
    }

    #[test]
    fn test_multiple_warnings() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        let responses = vec![SpeechModelResponseMetadata::new("tts-1")];

        let result = GenerateSpeechResult::new(audio, responses)
            .with_warning(SpeechModelCallWarning::unsupported_setting("speed"))
            .with_warning(SpeechModelCallWarning::unsupported_setting("language"))
            .with_warning(SpeechModelCallWarning::other("Custom warning"));

        assert_eq!(result.warnings.len(), 3);
    }

    #[test]
    fn test_multiple_responses() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        let responses = vec![
            SpeechModelResponseMetadata::new("tts-1"),
            SpeechModelResponseMetadata::new("tts-1-hd"),
        ];

        let result = GenerateSpeechResult::new(audio, responses);

        assert_eq!(result.responses.len(), 2);
    }

    #[test]
    fn test_different_audio_formats() {
        let formats = vec![
            ("audio/mp3", "mp3"),
            ("audio/mpeg", "mp3"),
            ("audio/wav", "wav"),
            ("audio/ogg", "ogg"),
            ("audio/flac", "flac"),
        ];

        for (media_type, expected_format) in formats {
            let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", media_type);
            let responses = vec![SpeechModelResponseMetadata::new("tts-1")];
            let result = GenerateSpeechResult::new(audio, responses);

            assert_eq!(
                result.audio.format, expected_format,
                "Failed for media type: {}",
                media_type
            );
        }
    }

    #[test]
    fn test_audio_with_name() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3")
            .with_name("speech.mp3");
        let responses = vec![SpeechModelResponseMetadata::new("tts-1")];

        let result = GenerateSpeechResult::new(audio, responses);

        assert_eq!(result.audio.name(), Some("speech.mp3"));
    }

    #[test]
    fn test_audio_data_conversion_mpeg_format() {
        let audio_data = AudioData::Base64("SGVsbG8gV29ybGQh".to_string());
        let file = audio_data_to_generated_audio_file(audio_data, "audio/mpeg");

        // audio/mpeg should map to mp3 format
        assert_eq!(file.format, "mp3");
        assert_eq!(file.media_type(), "audio/mpeg");
    }

    #[test]
    fn test_provider_metadata_structure() {
        let audio = GeneratedAudioFile::from_base64("SGVsbG8gV29ybGQh", "audio/mp3");
        let responses = vec![SpeechModelResponseMetadata::new("tts-1")];

        let mut metadata = HashMap::new();
        let mut openai_data = HashMap::new();
        openai_data.insert("voice".to_string(), serde_json::json!("alloy"));
        openai_data.insert("speed".to_string(), serde_json::json!(1.0));
        metadata.insert("openai".to_string(), openai_data);

        let result = GenerateSpeechResult::new(audio, responses).with_provider_metadata(metadata);

        let provider_data = result.provider_metadata.unwrap();
        let openai = provider_data.get("openai").unwrap();
        assert_eq!(openai.get("voice").unwrap(), &serde_json::json!("alloy"));
        assert_eq!(openai.get("speed").unwrap(), &serde_json::json!(1.0));
    }
}
