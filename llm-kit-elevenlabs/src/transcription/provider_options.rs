use serde::{Deserialize, Serialize};

/// Provider-specific options for ElevenLabs transcription (user-facing, camelCase).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElevenLabsTranscriptionProviderOptions {
    /// Language code (ISO-639-1 or ISO-639-3) for the audio.
    /// If not provided, language will be auto-detected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,

    /// Whether to tag audio events like (laughter), (footsteps), etc.
    /// Default: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_audio_events: Option<bool>,

    /// Maximum number of speakers in the audio (1-32).
    /// Helps with speaker diarization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_speakers: Option<u32>,

    /// Granularity of timestamps in the transcription.
    /// Options: "none", "word", "character"
    /// Default: "word"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps_granularity: Option<String>,

    /// Whether to annotate which speaker is talking.
    /// Default: false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diarize: Option<bool>,

    /// Format of the input audio file.
    /// Options: "pcm_s16le_16", "other"
    /// Default: "other"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<String>,
}

impl ElevenLabsTranscriptionProviderOptions {
    /// Validate the provider options.
    pub fn validate(&self) -> Result<(), String> {
        // Validate num_speakers
        if let Some(num_speakers) = self.num_speakers
            && (!(1..=32).contains(&num_speakers))
        {
            return Err("num_speakers must be between 1 and 32".to_string());
        }

        // Validate timestamps_granularity
        if let Some(ref granularity) = self.timestamps_granularity
            && !["none", "word", "character"].contains(&granularity.as_str())
        {
            return Err(
                "timestamps_granularity must be 'none', 'word', or 'character'".to_string(),
            );
        }

        // Validate file_format
        if let Some(ref format) = self.file_format
            && !["pcm_s16le_16", "other"].contains(&format.as_str())
        {
            return Err("file_format must be 'pcm_s16le_16' or 'other'".to_string());
        }

        Ok(())
    }
}

impl Default for ElevenLabsTranscriptionProviderOptions {
    fn default() -> Self {
        Self {
            language_code: None,
            tag_audio_events: Some(true),
            num_speakers: None,
            timestamps_granularity: Some("word".to_string()),
            diarize: Some(false),
            file_format: Some("other".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_options() {
        let options = ElevenLabsTranscriptionProviderOptions {
            num_speakers: Some(5),
            timestamps_granularity: Some("word".to_string()),
            ..Default::default()
        };
        assert!(options.validate().is_ok());
    }

    #[test]
    fn test_invalid_num_speakers() {
        let options = ElevenLabsTranscriptionProviderOptions {
            num_speakers: Some(50),
            ..Default::default()
        };
        assert!(options.validate().is_err());
    }

    #[test]
    fn test_invalid_granularity() {
        let options = ElevenLabsTranscriptionProviderOptions {
            timestamps_granularity: Some("invalid".to_string()),
            ..Default::default()
        };
        assert!(options.validate().is_err());
    }
}
