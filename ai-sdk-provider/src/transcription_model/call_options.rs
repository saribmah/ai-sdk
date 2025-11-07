use crate::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio_util::sync;

/// Audio data for transcription.
/// Accepts binary data or base64 encoded string.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TranscriptionAudioData {
    /// Base64 encoded audio string
    Base64(String),
    /// Binary audio data
    Binary(Vec<u8>),
}

impl TranscriptionAudioData {
    /// Create audio data from a base64 string
    pub fn from_base64(data: impl Into<String>) -> Self {
        Self::Base64(data.into())
    }

    /// Create audio data from binary data
    pub fn from_binary(data: Vec<u8>) -> Self {
        Self::Binary(data)
    }

    /// Check if this is base64 data
    pub fn is_base64(&self) -> bool {
        matches!(self, Self::Base64(_))
    }

    /// Check if this is binary data
    pub fn is_binary(&self) -> bool {
        matches!(self, Self::Binary(_))
    }

    /// Get the base64 string if this is base64 data
    pub fn as_base64(&self) -> Option<&str> {
        match self {
            Self::Base64(s) => Some(s),
            Self::Binary(_) => None,
        }
    }

    /// Get the binary data if this is binary data
    pub fn as_binary(&self) -> Option<&[u8]> {
        match self {
            Self::Base64(_) => None,
            Self::Binary(b) => Some(b),
        }
    }

    /// Get the size of the audio data in bytes
    pub fn len(&self) -> usize {
        match self {
            Self::Base64(s) => s.len(),
            Self::Binary(b) => b.len(),
        }
    }

    /// Check if the audio data is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl From<Vec<u8>> for TranscriptionAudioData {
    fn from(data: Vec<u8>) -> Self {
        Self::Binary(data)
    }
}

impl From<String> for TranscriptionAudioData {
    fn from(data: String) -> Self {
        Self::Base64(data)
    }
}

impl From<&str> for TranscriptionAudioData {
    fn from(data: &str) -> Self {
        Self::Base64(data.to_string())
    }
}

/// Transcription model call options.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptionModelCallOptions {
    /// Audio data to transcribe.
    /// Accepts binary data (`Vec<u8>`) or base64 encoded string.
    pub audio: TranscriptionAudioData,

    /// The IANA media type of the audio data.
    ///
    /// Common media types:
    /// - `audio/mpeg` - MP3
    /// - `audio/wav` - WAV
    /// - `audio/ogg` - OGG
    /// - `audio/flac` - FLAC
    /// - `audio/webm` - WebM
    /// - `audio/mp4` - MP4/M4A
    ///
    /// See: https://www.iana.org/assignments/media-types/media-types.xhtml
    pub media_type: String,

    /// Additional provider-specific options that are passed through to the provider
    /// as body parameters.
    ///
    /// The outer map is keyed by the provider name, and the inner
    /// map is keyed by the provider-specific metadata key.
    /// ```rust
    /// use std::collections::HashMap;
    /// use serde_json::json;
    ///
    /// let mut provider_options = HashMap::new();
    /// let mut openai_options = HashMap::new();
    /// openai_options.insert("timestampGranularities".to_string(), json!(["word"]));
    /// provider_options.insert("openai".to_string(), openai_options);
    /// ```
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<SharedProviderOptions>,

    /// Additional HTTP headers to be sent with the request.
    /// Only applicable for HTTP-based providers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,

    /// Abort/cancellation signal (not serialized, used for runtime control).
    #[serde(skip)]
    pub abort_signal: Option<sync::CancellationToken>,
}

impl TranscriptionModelCallOptions {
    /// Create new call options with audio data and media type.
    pub fn new(audio: impl Into<TranscriptionAudioData>, media_type: impl Into<String>) -> Self {
        Self {
            audio: audio.into(),
            media_type: media_type.into(),
            provider_options: None,
            headers: None,
            abort_signal: None,
        }
    }

    // Builder methods
    pub fn with_provider_options(mut self, options: SharedProviderOptions) -> Self {
        self.provider_options = Some(options);
        self
    }

    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    pub fn with_abort_signal(mut self, signal: sync::CancellationToken) -> Self {
        self.abort_signal = Some(signal);
        self
    }
}

// Convenience constructors for common media types
impl TranscriptionModelCallOptions {
    /// Create options for MP3 audio
    pub fn mp3(audio: impl Into<TranscriptionAudioData>) -> Self {
        Self::new(audio, "audio/mpeg")
    }

    /// Create options for WAV audio
    pub fn wav(audio: impl Into<TranscriptionAudioData>) -> Self {
        Self::new(audio, "audio/wav")
    }

    /// Create options for OGG audio
    pub fn ogg(audio: impl Into<TranscriptionAudioData>) -> Self {
        Self::new(audio, "audio/ogg")
    }

    /// Create options for FLAC audio
    pub fn flac(audio: impl Into<TranscriptionAudioData>) -> Self {
        Self::new(audio, "audio/flac")
    }

    /// Create options for WebM audio
    pub fn webm(audio: impl Into<TranscriptionAudioData>) -> Self {
        Self::new(audio, "audio/webm")
    }

    /// Create options for MP4/M4A audio
    pub fn mp4(audio: impl Into<TranscriptionAudioData>) -> Self {
        Self::new(audio, "audio/mp4")
    }

    /// Create options for OPUS audio (in Ogg container)
    pub fn opus(audio: impl Into<TranscriptionAudioData>) -> Self {
        Self::new(audio, "audio/ogg; codecs=opus")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_data_base64() {
        let audio = TranscriptionAudioData::from_base64("SGVsbG8gV29ybGQ=");
        assert!(audio.is_base64());
        assert!(!audio.is_binary());
        assert_eq!(audio.as_base64(), Some("SGVsbG8gV29ybGQ="));
    }

    #[test]
    fn test_audio_data_binary() {
        let data = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f];
        let audio = TranscriptionAudioData::from_binary(data.clone());
        assert!(!audio.is_base64());
        assert!(audio.is_binary());
        assert_eq!(audio.as_binary(), Some(data.as_slice()));
    }

    #[test]
    fn test_audio_data_from_vec() {
        let data = vec![1, 2, 3, 4, 5];
        let audio: TranscriptionAudioData = data.clone().into();
        assert!(audio.is_binary());
        assert_eq!(audio.as_binary(), Some(data.as_slice()));
    }

    #[test]
    fn test_audio_data_from_string() {
        let audio: TranscriptionAudioData = "base64data".to_string().into();
        assert!(audio.is_base64());
        assert_eq!(audio.as_base64(), Some("base64data"));
    }

    #[test]
    fn test_new() {
        let audio = vec![1, 2, 3, 4, 5];
        let options = TranscriptionModelCallOptions::new(audio, "audio/mpeg");
        assert_eq!(options.media_type, "audio/mpeg");
        assert!(options.audio.is_binary());
    }

    #[test]
    fn test_mp3_helper() {
        let audio = vec![1, 2, 3];
        let options = TranscriptionModelCallOptions::mp3(audio);
        assert_eq!(options.media_type, "audio/mpeg");
    }

    #[test]
    fn test_wav_helper() {
        let audio = "base64data";
        let options = TranscriptionModelCallOptions::wav(audio);
        assert_eq!(options.media_type, "audio/wav");
        assert!(options.audio.is_base64());
    }

    #[test]
    fn test_builder_chain() {
        let audio = vec![1, 2, 3];
        let mut headers = HashMap::new();
        headers.insert("X-Custom".to_string(), "value".to_string());

        let options = TranscriptionModelCallOptions::mp3(audio).with_headers(headers.clone());

        assert_eq!(options.media_type, "audio/mpeg");
        assert_eq!(options.headers, Some(headers));
    }

    #[test]
    fn test_audio_data_len() {
        let base64 = TranscriptionAudioData::from_base64("SGVsbG8=");
        assert_eq!(base64.len(), 8);

        let binary = TranscriptionAudioData::from_binary(vec![1, 2, 3, 4, 5]);
        assert_eq!(binary.len(), 5);
    }

    #[test]
    fn test_audio_data_is_empty() {
        let empty_base64 = TranscriptionAudioData::from_base64("");
        assert!(empty_base64.is_empty());

        let empty_binary = TranscriptionAudioData::from_binary(vec![]);
        assert!(empty_binary.is_empty());

        let non_empty = TranscriptionAudioData::from_base64("SGVsbG8=");
        assert!(!non_empty.is_empty());
    }
}
