use serde::{Deserialize, Serialize};

/// Request body for ElevenLabs text-to-speech API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElevenLabsSpeechRequest {
    /// The text to convert to speech.
    pub text: String,

    /// Model ID to use for generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,

    /// Language code (ISO-639-1 or ISO-639-3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,

    /// Voice settings for the generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<VoiceSettings>,

    /// Pronunciation dictionary locators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryLocator>>,

    /// Seed for deterministic generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,

    /// Previous text for context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_text: Option<String>,

    /// Next text for context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_text: Option<String>,

    /// Previous request IDs for context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_request_ids: Option<Vec<String>>,

    /// Next request IDs for context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_request_ids: Option<Vec<String>>,

    /// Text normalization mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_text_normalization: Option<TextNormalization>,

    /// Whether to apply language-specific text normalization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_language_text_normalization: Option<bool>,
}

/// Voice settings for speech generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSettings {
    /// Stability of the voice (0.0 to 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stability: Option<f32>,

    /// Similarity boost (0.0 to 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity_boost: Option<f32>,

    /// Style exaggeration (0.0 to 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<f32>,

    /// Whether to use speaker boost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_speaker_boost: Option<bool>,

    /// Speed of speech (0.25 to 4.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,
}

/// Pronunciation dictionary locator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PronunciationDictionaryLocator {
    /// Pronunciation dictionary ID.
    pub pronunciation_dictionary_id: String,

    /// Version ID (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

/// Text normalization mode.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TextNormalization {
    Auto,
    On,
    Off,
}

impl ElevenLabsSpeechRequest {
    /// Create a new speech request with text.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            model_id: None,
            language_code: None,
            voice_settings: None,
            pronunciation_dictionary_locators: None,
            seed: None,
            previous_text: None,
            next_text: None,
            previous_request_ids: None,
            next_request_ids: None,
            apply_text_normalization: None,
            apply_language_text_normalization: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speech_request_serialization() {
        let request = ElevenLabsSpeechRequest::new("Hello world");
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("Hello world"));
    }

    #[test]
    fn test_voice_settings_serialization() {
        let settings = VoiceSettings {
            stability: Some(0.5),
            similarity_boost: Some(0.8),
            style: Some(0.3),
            use_speaker_boost: Some(true),
            speed: Some(1.0),
        };
        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("0.5"));
        assert!(json.contains("0.8"));
    }
}
