use serde::{Deserialize, Serialize};

/// Response from ElevenLabs transcription API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElevenLabsTranscriptionResponse {
    /// Detected language code (ISO-639)
    pub language_code: String,

    /// Probability of the detected language
    pub language_probability: f64,

    /// Full transcribed text
    pub text: String,

    /// Word-level details with timestamps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<TranscriptionWord>>,
}

/// Word-level transcription details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionWord {
    /// The transcribed text
    pub text: String,

    /// Type of the word (word, spacing, audio_event)
    #[serde(rename = "type")]
    pub word_type: WordType,

    /// Start time in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,

    /// End time in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,

    /// Speaker ID (if diarization is enabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_id: Option<String>,

    /// Character-level details (if character timestamps requested)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characters: Option<Vec<CharacterTimestamp>>,
}

/// Type of transcription element.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WordType {
    Word,
    Spacing,
    AudioEvent,
}

/// Character-level timestamp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterTimestamp {
    /// The character
    pub text: String,

    /// Start time in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,

    /// End time in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transcription_response_deserialization() {
        let json = r#"{
            "language_code": "en",
            "language_probability": 0.99,
            "text": "Hello world",
            "words": [
                {
                    "text": "Hello",
                    "type": "word",
                    "start": 0.0,
                    "end": 0.5
                },
                {
                    "text": " ",
                    "type": "spacing"
                },
                {
                    "text": "world",
                    "type": "word",
                    "start": 0.5,
                    "end": 1.0
                }
            ]
        }"#;

        let response: ElevenLabsTranscriptionResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.language_code, "en");
        assert_eq!(response.text, "Hello world");
        assert_eq!(response.words.as_ref().unwrap().len(), 3);
    }
}
