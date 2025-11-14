use serde::{Deserialize, Serialize};

/// Groq transcription model identifier.
///
/// Reference: <https://console.groq.com/docs/speech-to-text>
pub type GroqTranscriptionModelId = String;

/// Groq-specific provider options for transcription.
///
/// These options are passed to the Groq API via the provider_options parameter.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroqTranscriptionOptions {
    /// The language of the input audio in ISO-639-1 format.
    /// Supplying the input language improves accuracy and latency.
    ///
    /// Example: "en" for English, "es" for Spanish
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// An optional text to guide the model's style or continue a previous audio segment.
    /// The prompt should match the audio language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// The format of the transcript output.
    ///
    /// Options:
    /// - "json" (default): JSON format with only the text
    /// - "verbose_json": JSON with additional metadata (timestamps, segments, etc.)
    /// - "text": Plain text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,

    /// The sampling temperature, between 0 and 1.
    ///
    /// Higher values like 0.8 will make the output more random,
    /// while lower values like 0.2 will make it more focused and deterministic.
    ///
    /// Default: 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// The timestamp granularities to populate for this transcription.
    ///
    /// Options:
    /// - "segment": Segment-level timestamps
    /// - "word": Word-level timestamps
    ///
    /// Default: ["segment"]
    ///
    /// Note: word-level timestamps are only available with `response_format: "verbose_json"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_granularities: Option<Vec<String>>,
}

impl GroqTranscriptionOptions {
    /// Create new transcription options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the language of the input audio.
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Set a prompt to guide the model.
    pub fn with_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    /// Set the response format.
    pub fn with_response_format(mut self, format: impl Into<String>) -> Self {
        self.response_format = Some(format.into());
        self
    }

    /// Set the sampling temperature (0.0 to 1.0).
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set timestamp granularities.
    pub fn with_timestamp_granularities(mut self, granularities: Vec<String>) -> Self {
        self.timestamp_granularities = Some(granularities);
        self
    }

    /// Enable verbose JSON response format with segments and timestamps.
    pub fn with_verbose_json(self) -> Self {
        self.with_response_format("verbose_json")
    }

    /// Enable word-level timestamps (requires verbose_json format).
    pub fn with_word_timestamps(mut self) -> Self {
        self.response_format = Some("verbose_json".to_string());
        self.timestamp_granularities = Some(vec!["word".to_string()]);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_builder() {
        let options = GroqTranscriptionOptions::new()
            .with_language("en")
            .with_prompt("Transcribe this audio")
            .with_response_format("verbose_json")
            .with_temperature(0.5)
            .with_timestamp_granularities(vec!["segment".to_string()]);

        assert_eq!(options.language, Some("en".to_string()));
        assert_eq!(options.prompt, Some("Transcribe this audio".to_string()));
        assert_eq!(options.response_format, Some("verbose_json".to_string()));
        assert_eq!(options.temperature, Some(0.5));
        assert_eq!(
            options.timestamp_granularities,
            Some(vec!["segment".to_string()])
        );
    }

    #[test]
    fn test_verbose_json_helper() {
        let options = GroqTranscriptionOptions::new().with_verbose_json();
        assert_eq!(options.response_format, Some("verbose_json".to_string()));
    }

    #[test]
    fn test_word_timestamps_helper() {
        let options = GroqTranscriptionOptions::new().with_word_timestamps();
        assert_eq!(options.response_format, Some("verbose_json".to_string()));
        assert_eq!(
            options.timestamp_granularities,
            Some(vec!["word".to_string()])
        );
    }

    #[test]
    fn test_default_options() {
        let options = GroqTranscriptionOptions::default();
        assert!(options.language.is_none());
        assert!(options.prompt.is_none());
        assert!(options.response_format.is_none());
        assert!(options.temperature.is_none());
        assert!(options.timestamp_granularities.is_none());
    }

    #[test]
    fn test_serialization() {
        let options = GroqTranscriptionOptions::new()
            .with_language("en")
            .with_temperature(0.7);

        let json = serde_json::to_value(&options).unwrap();
        assert_eq!(json["language"], "en");
        // Check temperature is approximately 0.7 (floating point precision)
        let temp = json["temperature"].as_f64().unwrap();
        assert!((temp - 0.7).abs() < 0.01);
    }
}
