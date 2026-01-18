use crate::shared::headers::SharedHeaders;
use crate::shared::provider_metadata::SharedProviderMetadata;
use crate::speech_model::call_options::SpeechModelCallOptions;
use crate::speech_model::call_warning::SpeechModelCallWarning;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::SystemTime;

/// Options for calling speech models
pub mod call_options;
/// Warnings that can be emitted during speech model calls
pub mod call_warning;

/// Speech synthesis model trait.
///
/// This trait defines the interface for text-to-speech (TTS) models. Implementations
/// handle converting text input into synthesized speech audio using provider-specific APIs.
///
/// # Specification Version
///
/// This implements version 3 of the speech model interface, which supports:
/// - Text-to-speech synthesis
/// - Multiple voice options
/// - Various audio formats (MP3, WAV, PCM, etc.)
/// - Speed and pitch control
/// - Base64 and binary audio output
///
/// # Examples
///
/// ```no_run
/// use llm_kit_provider::SpeechModel;
/// use llm_kit_provider::speech_model::call_options::SpeechModelCallOptions;
/// use llm_kit_provider::speech_model::SpeechModelResponse;
/// use async_trait::async_trait;
///
/// struct MySpeechModel {
///     model_id: String,
///     api_key: String,
/// }
///
/// #[async_trait]
/// impl SpeechModel for MySpeechModel {
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
///         options: SpeechModelCallOptions,
///     ) -> Result<SpeechModelResponse, Box<dyn std::error::Error>> {
///         // Implementation
///         todo!()
///     }
/// }
/// ```
#[async_trait]
pub trait SpeechModel: Send + Sync {
    /// Returns the specification version this model implements.
    ///
    /// Defaults to "v3" for the current SDK version. This allows us to evolve
    /// the speech model interface and retain backwards compatibility.
    fn specification_version(&self) -> &str {
        "v3"
    }

    /// Name of the provider for logging purposes.
    ///
    /// Examples: "openai", "elevenlabs", "google"
    fn provider(&self) -> &str;

    /// Provider-specific model ID for logging purposes.
    ///
    /// Examples: "tts-1", "tts-1-hd", "eleven_monolingual_v1"
    fn model_id(&self) -> &str;

    /// Generates speech audio from text.
    ///
    /// # Arguments
    ///
    /// * `options` - Call options including text, voice, format, speed, etc.
    ///
    /// # Returns
    ///
    /// A [`SpeechModelResponse`] containing:
    /// - Generated audio (as base64 or binary data)
    /// - Provider-specific metadata
    /// - Warnings about unsupported settings
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The API request fails
    /// - The text is invalid or too long
    /// - The voice or format is unsupported
    /// - Audio generation fails
    async fn do_generate(
        &self,
        options: SpeechModelCallOptions,
    ) -> Result<SpeechModelResponse, Box<dyn std::error::Error>>;
}

/// Response from a speech model.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeechModelResponse {
    /// Generated audio as base64 string or binary data.
    /// The audio should be returned without any unnecessary conversion.
    /// If the API returns base64 encoded strings, the audio should be returned
    /// as base64 encoded strings. If the API returns binary data, the audio
    /// should be returned as binary data.
    pub audio: AudioData,

    /// Warnings for the call, e.g. unsupported settings.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub warnings: Vec<SpeechModelCallWarning>,

    /// Optional request information for telemetry and debugging purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<SpeechModelRequestMetadata>,

    /// Response information for telemetry and debugging purposes.
    pub response: SpeechModelResponseMetadata,

    /// Additional provider-specific metadata. They are passed through
    /// from the provider to the LLM Kit and enable provider-specific
    /// results that can be fully encapsulated in the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,
}

/// Audio data can be either base64 encoded strings or binary data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AudioData {
    /// Base64 encoded audio string
    Base64(String),
    /// Binary audio data (serialized as array of numbers)
    Binary(Vec<u8>),
}

impl AudioData {
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

/// Request metadata for telemetry and debugging purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeechModelRequestMetadata {
    /// Request body (available only for providers that use HTTP requests).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Value>,
}

/// Response metadata for telemetry and debugging purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeechModelResponseMetadata {
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

impl SpeechModelResponse {
    /// Create a new response with audio.
    pub fn new(audio: AudioData, response: SpeechModelResponseMetadata) -> Self {
        Self {
            audio,
            warnings: Vec::new(),
            request: None,
            response,
            provider_metadata: None,
        }
    }

    /// Add warnings to the response.
    pub fn with_warnings(mut self, warnings: Vec<SpeechModelCallWarning>) -> Self {
        self.warnings = warnings;
        self
    }

    /// Add a single warning to the response.
    pub fn with_warning(mut self, warning: SpeechModelCallWarning) -> Self {
        self.warnings.push(warning);
        self
    }

    /// Add request metadata to the response.
    pub fn with_request_metadata(mut self, metadata: SpeechModelRequestMetadata) -> Self {
        self.request = Some(metadata);
        self
    }

    /// Add provider metadata to the response.
    pub fn with_provider_metadata(mut self, metadata: SharedProviderMetadata) -> Self {
        self.provider_metadata = Some(metadata);
        self
    }
}

impl SpeechModelRequestMetadata {
    /// Create new request metadata.
    pub fn new() -> Self {
        Self { body: None }
    }

    /// Add body to the request metadata.
    pub fn with_body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }
}

impl Default for SpeechModelRequestMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl SpeechModelResponseMetadata {
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
    fn test_audio_data_base64() {
        let audio = AudioData::from_base64("SGVsbG8gV29ybGQ=");
        assert!(audio.is_base64());
        assert!(!audio.is_binary());
        assert_eq!(audio.as_base64(), Some("SGVsbG8gV29ybGQ="));
        assert_eq!(audio.as_binary(), None);
    }

    #[test]
    fn test_audio_data_binary() {
        let data = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f];
        let audio = AudioData::from_binary(data.clone());
        assert!(!audio.is_base64());
        assert!(audio.is_binary());
        assert_eq!(audio.as_binary(), Some(data.as_slice()));
        assert_eq!(audio.as_base64(), None);
    }

    #[test]
    fn test_audio_data_len() {
        let base64 = AudioData::from_base64("SGVsbG8=");
        assert_eq!(base64.len(), 8);

        let binary = AudioData::from_binary(vec![1, 2, 3, 4, 5]);
        assert_eq!(binary.len(), 5);
    }

    #[test]
    fn test_audio_data_is_empty() {
        let empty_base64 = AudioData::from_base64("");
        assert!(empty_base64.is_empty());

        let empty_binary = AudioData::from_binary(vec![]);
        assert!(empty_binary.is_empty());

        let non_empty = AudioData::from_base64("SGVsbG8=");
        assert!(!non_empty.is_empty());
    }

    #[test]
    fn test_response_builder() {
        let metadata = SpeechModelResponseMetadata::new("tts-1");
        let audio = AudioData::from_base64("SGVsbG8=");

        let response = SpeechModelResponse::new(audio, metadata)
            .with_warning(SpeechModelCallWarning::other("Test warning"));

        assert_eq!(response.warnings.len(), 1);
        assert!(response.audio.is_base64());
    }
}
