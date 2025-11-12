use serde::{Deserialize, Serialize};

/// Provider-specific options for ElevenLabs speech generation (user-facing, camelCase).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct ElevenLabsSpeechProviderOptions {
    /// Language code (ISO-639-1 or ISO-639-3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,

    /// Voice settings for fine-tuning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<VoiceSettingsOptions>,

    /// Pronunciation dictionary locators (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronunciation_dictionary_locators: Option<Vec<PronunciationDictionaryLocatorOptions>>,

    /// Seed for deterministic generation (0 to 4294967295).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,

    /// Previous text for context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_text: Option<String>,

    /// Next text for context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_text: Option<String>,

    /// Previous request IDs for context (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_request_ids: Option<Vec<String>>,

    /// Next request IDs for context (max 3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_request_ids: Option<Vec<String>>,

    /// Text normalization mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_text_normalization: Option<String>,

    /// Whether to apply language-specific text normalization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_language_text_normalization: Option<bool>,

    /// Whether to enable logging for this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_logging: Option<bool>,
}

/// Voice settings options (camelCase for user-facing API).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct VoiceSettingsOptions {
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
}

/// Pronunciation dictionary locator options (camelCase).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PronunciationDictionaryLocatorOptions {
    /// Pronunciation dictionary ID.
    pub pronunciation_dictionary_id: String,

    /// Version ID (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

impl ElevenLabsSpeechProviderOptions {
    /// Validate the provider options.
    pub fn validate(&self) -> Result<(), String> {
        // Validate voice settings ranges
        if let Some(ref settings) = self.voice_settings {
            if let Some(stability) = settings.stability
                && !(0.0..=1.0).contains(&stability)
            {
                return Err("stability must be between 0.0 and 1.0".to_string());
            }
            if let Some(similarity_boost) = settings.similarity_boost
                && !(0.0..=1.0).contains(&similarity_boost)
            {
                return Err("similarity_boost must be between 0.0 and 1.0".to_string());
            }
            if let Some(style) = settings.style
                && !(0.0..=1.0).contains(&style)
            {
                return Err("style must be between 0.0 and 1.0".to_string());
            }
        }

        // Validate pronunciation dictionaries limit
        if let Some(ref dictionaries) = self.pronunciation_dictionary_locators
            && dictionaries.len() > 3
        {
            return Err("maximum 3 pronunciation dictionaries allowed".to_string());
        }

        // Validate request IDs limits
        if let Some(ref ids) = self.previous_request_ids
            && ids.len() > 3
        {
            return Err("maximum 3 previous request IDs allowed".to_string());
        }
        if let Some(ref ids) = self.next_request_ids
            && ids.len() > 3
        {
            return Err("maximum 3 next request IDs allowed".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_voice_settings() {
        let options = ElevenLabsSpeechProviderOptions {
            voice_settings: Some(VoiceSettingsOptions {
                stability: Some(0.5),
                similarity_boost: Some(0.8),
                style: Some(0.3),
                use_speaker_boost: Some(true),
            }),
            ..Default::default()
        };
        assert!(options.validate().is_ok());
    }

    #[test]
    fn test_invalid_stability() {
        let options = ElevenLabsSpeechProviderOptions {
            voice_settings: Some(VoiceSettingsOptions {
                stability: Some(1.5),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(options.validate().is_err());
    }

    #[test]
    fn test_too_many_dictionaries() {
        let options = ElevenLabsSpeechProviderOptions {
            pronunciation_dictionary_locators: Some(vec![
                PronunciationDictionaryLocatorOptions {
                    pronunciation_dictionary_id: "1".to_string(),
                    version_id: None,
                },
                PronunciationDictionaryLocatorOptions {
                    pronunciation_dictionary_id: "2".to_string(),
                    version_id: None,
                },
                PronunciationDictionaryLocatorOptions {
                    pronunciation_dictionary_id: "3".to_string(),
                    version_id: None,
                },
                PronunciationDictionaryLocatorOptions {
                    pronunciation_dictionary_id: "4".to_string(),
                    version_id: None,
                },
            ]),
            ..Default::default()
        };
        assert!(options.validate().is_err());
    }
}
