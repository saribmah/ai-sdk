pub mod result;

pub use result::TranscriptionResult;

use crate::error::AISDKError;
use crate::generate_text::prepare_retries;
use crate::prompt::message::DataContent;
use ai_sdk_provider::shared::headers::SharedHeaders;
use ai_sdk_provider::shared::provider_options::SharedProviderOptions;
use ai_sdk_provider::transcription_model::TranscriptionModel;
use ai_sdk_provider::transcription_model::call_options::{
    TranscriptionAudioData, TranscriptionModelCallOptions,
};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Builder for generating transcripts using a transcription model.
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::{Transcribe, AudioInput};
/// use ai_sdk_core::prompt::message::DataContent;
///
/// let audio_data = DataContent::from(audio_bytes);
/// let result = Transcribe::new(model, AudioInput::Data(audio_data))
///     .max_retries(3)
///     .execute()
///     .await?;
///
/// println!("Transcript: {}", result.text);
/// ```
pub struct Transcribe {
    model: Arc<dyn TranscriptionModel>,
    audio: AudioInput,
    provider_options: Option<SharedProviderOptions>,
    max_retries: Option<u32>,
    abort_signal: Option<CancellationToken>,
    headers: Option<SharedHeaders>,
}

impl Transcribe {
    /// Create a new Transcribe builder.
    ///
    /// # Arguments
    ///
    /// * `model` - The transcription model to use
    /// * `audio` - The audio data to transcribe (URL, DataContent, or bytes)
    pub fn new(model: Arc<dyn TranscriptionModel>, audio: AudioInput) -> Self {
        Self {
            model,
            audio,
            provider_options: None,
            max_retries: None,
            abort_signal: None,
            headers: None,
        }
    }

    /// Set additional provider-specific options.
    ///
    /// # Arguments
    ///
    /// * `provider_options` - Additional provider-specific options
    pub fn provider_options(mut self, provider_options: SharedProviderOptions) -> Self {
        self.provider_options = Some(provider_options);
        self
    }

    /// Set the maximum number of retries.
    ///
    /// # Arguments
    ///
    /// * `max_retries` - Maximum number of retries. Set to 0 to disable retries. Default: 2.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    /// Set an abort signal that can be used to cancel the call.
    ///
    /// # Arguments
    ///
    /// * `abort_signal` - An optional abort signal
    pub fn abort_signal(mut self, abort_signal: CancellationToken) -> Self {
        self.abort_signal = Some(abort_signal);
        self
    }

    /// Set additional HTTP headers to be sent with the request.
    ///
    /// # Arguments
    ///
    /// * `headers` - Additional HTTP headers
    pub fn headers(mut self, headers: SharedHeaders) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Execute the transcription operation.
    ///
    /// # Returns
    ///
    /// A result object that contains the generated transcript.
    pub async fn execute(self) -> Result<TranscriptionResult, AISDKError> {
        // Check specification version
        if self.model.specification_version() != "v3" {
            return Err(AISDKError::model_error(format!(
                "Unsupported model version: {}. Provider: {}, Model ID: {}",
                self.model.specification_version(),
                self.model.provider(),
                self.model.model_id()
            )));
        }

        // Add user agent to headers
        let headers_with_user_agent =
            add_user_agent_suffix(self.headers, format!("ai/{}", VERSION));

        // Prepare retry configuration
        let retry_config = prepare_retries(self.max_retries, self.abort_signal.clone())?;

        // Convert audio input to bytes
        let audio_bytes = match self.audio {
            AudioInput::Url(url) => {
                // Download the audio file from URL
                download_from_url(&url).await?
            }
            AudioInput::Data(data) => {
                // Convert DataContent to bytes
                data.to_bytes().map_err(|e| {
                    AISDKError::invalid_argument(
                        "audio",
                        format!("{:?}", data),
                        format!("Failed to decode base64 audio data: {}", e),
                    )
                })?
            }
        };

        // Detect media type from audio bytes
        let media_type = detect_media_type_from_bytes(&audio_bytes);

        // Clone model for logging later
        let provider_name = self.model.provider().to_string();
        let model_id = self.model.model_id().to_string();

        // Execute the model call with retry logic
        let result = retry_config
            .execute_with_boxed_error(move || {
                let model = self.model.clone();
                let audio_bytes = audio_bytes.clone();
                let provider_options = self.provider_options.clone();
                let headers_with_user_agent = headers_with_user_agent.clone();
                let abort_signal = self.abort_signal.clone();
                let media_type = media_type.clone();

                Box::pin(async move {
                    model
                        .do_generate(
                            TranscriptionModelCallOptions::new(
                                TranscriptionAudioData::from_binary(audio_bytes),
                                media_type,
                            )
                            .with_provider_options(provider_options.unwrap_or_default())
                            .with_headers(
                                headers_with_user_agent
                                    .clone()
                                    .unwrap_or_default()
                                    .into_iter()
                                    .collect(),
                            )
                            .with_abort_signal(abort_signal.unwrap_or_default()),
                        )
                        .await
                })
            })
            .await?;

        // Log warnings if any
        if !result.warnings.is_empty() {
            log::warn!(
                "Transcription warnings from {} (model: {}): {} warning(s)",
                provider_name,
                model_id,
                result.warnings.len()
            );
            for warning in &result.warnings {
                log::warn!("  - {:?}", warning);
            }
        }

        // Check if we got a transcript
        if result.text.is_empty() {
            return Err(AISDKError::no_transcript_generated(vec![
                result.response.clone(),
            ]));
        }

        // Build the transcription result
        let mut transcription_result = TranscriptionResult::new(result.text, vec![result.response])
            .with_segments(result.segments);

        if let Some(language) = result.language {
            transcription_result = transcription_result.with_language(language);
        }

        if let Some(duration) = result.duration_in_seconds {
            transcription_result = transcription_result.with_duration(duration);
        }

        transcription_result = transcription_result.with_warnings(result.warnings);

        if let Some(provider_metadata) = result.provider_metadata {
            transcription_result = transcription_result.with_provider_metadata(provider_metadata);
        }

        Ok(transcription_result)
    }
}

/// Generates transcripts using a transcription model.
///
/// **Deprecated:** Use the `Transcribe` builder instead.
///
/// # Arguments
///
/// * `model` - The transcription model to use.
/// * `audio` - The audio data to transcribe as DataContent (string | bytes) or a URL.
/// * `provider_options` - Additional provider-specific options that are passed through to the provider
///   as body parameters.
/// * `max_retries` - Maximum number of retries. Set to 0 to disable retries. Default: 2.
/// * `abort_signal` - An optional abort signal that can be used to cancel the call.
/// * `headers` - Additional HTTP headers to be sent with the request. Only applicable for HTTP-based providers.
///
/// # Returns
///
/// A result object that contains the generated transcript.
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::transcribe::transcribe;
/// use ai_sdk_core::prompt::message::DataContent;
/// use std::sync::Arc;
///
/// let audio_data = DataContent::from(audio_bytes);
/// let result = transcribe(
///     model,
///     AudioInput::Data(audio_data),
///     None,
///     None,
///     None,
///     None,
/// ).await?;
///
/// println!("Transcript: {}", result.text);
/// ```
#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
async fn transcribe(
    model: Arc<dyn TranscriptionModel>,
    audio: AudioInput,
    provider_options: Option<SharedProviderOptions>,
    max_retries: Option<u32>,
    abort_signal: Option<CancellationToken>,
    headers: Option<SharedHeaders>,
) -> Result<TranscriptionResult, AISDKError> {
    // Check specification version
    if model.specification_version() != "v3" {
        return Err(AISDKError::model_error(format!(
            "Unsupported model version: {}. Provider: {}, Model ID: {}",
            model.specification_version(),
            model.provider(),
            model.model_id()
        )));
    }

    // Add user agent to headers
    let headers_with_user_agent = add_user_agent_suffix(headers, format!("ai/{}", VERSION));

    // Prepare retry configuration
    let retry_config = prepare_retries(max_retries, abort_signal.clone())?;

    // Convert audio input to bytes
    let audio_bytes = match audio {
        AudioInput::Url(url) => {
            // Download the audio file from URL
            download_from_url(&url).await?
        }
        AudioInput::Data(data) => {
            // Convert DataContent to bytes
            data.to_bytes().map_err(|e| {
                AISDKError::invalid_argument(
                    "audio",
                    format!("{:?}", data),
                    format!("Failed to decode base64 audio data: {}", e),
                )
            })?
        }
    };

    // Detect media type from audio bytes
    let media_type = detect_media_type_from_bytes(&audio_bytes);

    // Clone model for logging later
    let provider_name = model.provider().to_string();
    let model_id = model.model_id().to_string();

    // Execute the model call with retry logic
    let result = retry_config
        .execute_with_boxed_error(move || {
            let model = model.clone();
            let audio_bytes = audio_bytes.clone();
            let provider_options = provider_options.clone();
            let headers_with_user_agent = headers_with_user_agent.clone();
            let abort_signal = abort_signal.clone();
            let media_type = media_type.clone();

            Box::pin(async move {
                model
                    .do_generate(
                        TranscriptionModelCallOptions::new(
                            TranscriptionAudioData::from_binary(audio_bytes),
                            media_type,
                        )
                        .with_provider_options(provider_options.unwrap_or_default())
                        .with_headers(
                            headers_with_user_agent
                                .clone()
                                .unwrap_or_default()
                                .into_iter()
                                .collect(),
                        )
                        .with_abort_signal(abort_signal.unwrap_or_default()),
                    )
                    .await
            })
        })
        .await?;

    // Log warnings if any
    if !result.warnings.is_empty() {
        log::warn!(
            "Transcription warnings from {} (model: {}): {} warning(s)",
            provider_name,
            model_id,
            result.warnings.len()
        );
        for warning in &result.warnings {
            log::warn!("  - {:?}", warning);
        }
    }

    // Check if we got a transcript
    if result.text.is_empty() {
        return Err(AISDKError::no_transcript_generated(vec![
            result.response.clone(),
        ]));
    }

    // Build the transcription result
    let mut transcription_result =
        TranscriptionResult::new(result.text, vec![result.response]).with_segments(result.segments);

    if let Some(language) = result.language {
        transcription_result = transcription_result.with_language(language);
    }

    if let Some(duration) = result.duration_in_seconds {
        transcription_result = transcription_result.with_duration(duration);
    }

    transcription_result = transcription_result.with_warnings(result.warnings);

    if let Some(provider_metadata) = result.provider_metadata {
        transcription_result = transcription_result.with_provider_metadata(provider_metadata);
    }

    Ok(transcription_result)
}

/// Audio input for transcription.
/// Can be either a URL or raw DataContent (base64 or bytes).
#[derive(Debug, Clone)]
pub enum AudioInput {
    /// URL to an audio file
    Url(String),
    /// Raw audio data
    Data(DataContent),
}

impl From<DataContent> for AudioInput {
    fn from(data: DataContent) -> Self {
        Self::Data(data)
    }
}

impl From<String> for AudioInput {
    fn from(s: String) -> Self {
        // Check if it looks like a URL
        if s.starts_with("http://") || s.starts_with("https://") {
            Self::Url(s)
        } else {
            // Treat as base64 data
            Self::Data(DataContent::Base64(s))
        }
    }
}

impl From<&str> for AudioInput {
    fn from(s: &str) -> Self {
        String::from(s).into()
    }
}

impl From<Vec<u8>> for AudioInput {
    fn from(bytes: Vec<u8>) -> Self {
        Self::Data(DataContent::Bytes(bytes))
    }
}

/// Downloads audio data from a URL.
async fn download_from_url(url: &str) -> Result<Vec<u8>, AISDKError> {
    let response = reqwest::get(url).await.map_err(|e| {
        AISDKError::model_error(format!("Failed to download audio from URL {}: {}", url, e))
    })?;

    if !response.status().is_success() {
        return Err(AISDKError::model_error(format!(
            "Failed to download audio from URL {}: HTTP {}",
            url,
            response.status()
        )));
    }

    let bytes = response.bytes().await.map_err(|e| {
        AISDKError::model_error(format!("Failed to read audio data from URL {}: {}", url, e))
    })?;

    Ok(bytes.to_vec())
}

/// Detects the media type from audio bytes based on magic numbers (file signatures).
fn detect_media_type_from_bytes(bytes: &[u8]) -> String {
    if bytes.len() < 4 {
        return "audio/wav".to_string(); // Default fallback
    }

    // MP3 - ID3 tag (ID3v2)
    if bytes.len() >= 3 && bytes[0..3] == [0x49, 0x44, 0x33] {
        return "audio/mpeg".to_string();
    }

    // MP3 - Frame sync (0xFF 0xFB or 0xFF 0xFA)
    if bytes.len() >= 2 && bytes[0] == 0xFF && (bytes[1] == 0xFB || bytes[1] == 0xFA) {
        return "audio/mpeg".to_string();
    }

    // WAV - RIFF header
    if bytes.len() >= 12
        && bytes[0..4] == [0x52, 0x49, 0x46, 0x46] // "RIFF"
        && bytes[8..12] == [0x57, 0x41, 0x56, 0x45]
    // "WAVE"
    {
        return "audio/wav".to_string();
    }

    // OGG - "OggS" header
    if bytes.len() >= 4 && bytes[0..4] == [0x4F, 0x67, 0x67, 0x53] {
        return "audio/ogg".to_string();
    }

    // FLAC - "fLaC" header
    if bytes.len() >= 4 && bytes[0..4] == [0x66, 0x4C, 0x61, 0x43] {
        return "audio/flac".to_string();
    }

    // AAC - ADTS header (0xFF 0xF1 or 0xFF 0xF9)
    if bytes.len() >= 2 && bytes[0] == 0xFF && (bytes[1] == 0xF1 || bytes[1] == 0xF9) {
        return "audio/aac".to_string();
    }

    // WebM/Matroska - EBML header
    if bytes.len() >= 4 && bytes[0..4] == [0x1A, 0x45, 0xDF, 0xA3] {
        return "audio/webm".to_string();
    }

    // M4A/MP4 - ftyp box
    if bytes.len() >= 8 && bytes[4..8] == [0x66, 0x74, 0x79, 0x70] {
        return "audio/mp4".to_string();
    }

    // Default fallback
    "audio/wav".to_string()
}

/// Adds a user agent suffix to the headers.
fn add_user_agent_suffix(headers: Option<SharedHeaders>, suffix: String) -> Option<SharedHeaders> {
    let mut headers = headers.unwrap_or_default();

    // Get existing user agent or use empty string
    let existing_user_agent = headers.get("user-agent").cloned().unwrap_or_default();

    // Append suffix
    let new_user_agent = if existing_user_agent.is_empty() {
        suffix
    } else {
        format!("{} {}", existing_user_agent, suffix)
    };

    headers.insert("user-agent".to_string(), new_user_agent);
    Some(headers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_input_from_data_content() {
        let data = DataContent::base64("SGVsbG8=");
        let input = AudioInput::from(data.clone());
        match input {
            AudioInput::Data(d) => assert_eq!(d, data),
            _ => panic!("Expected AudioInput::Data"),
        }
    }

    #[test]
    fn test_audio_input_from_url_string() {
        let input = AudioInput::from("https://example.com/audio.mp3".to_string());
        match input {
            AudioInput::Url(url) => assert_eq!(url, "https://example.com/audio.mp3"),
            _ => panic!("Expected AudioInput::Url"),
        }
    }

    #[test]
    fn test_audio_input_from_base64_string() {
        let input = AudioInput::from("SGVsbG8gV29ybGQ=".to_string());
        match input {
            AudioInput::Data(DataContent::Base64(s)) => assert_eq!(s, "SGVsbG8gV29ybGQ="),
            _ => panic!("Expected AudioInput::Data with Base64"),
        }
    }

    #[test]
    fn test_audio_input_from_bytes() {
        let bytes = vec![1, 2, 3, 4, 5];
        let input = AudioInput::from(bytes.clone());
        match input {
            AudioInput::Data(DataContent::Bytes(b)) => assert_eq!(b, bytes),
            _ => panic!("Expected AudioInput::Data with Bytes"),
        }
    }

    #[test]
    fn test_detect_media_type_mp3_id3() {
        let mp3_signature = &[0x49, 0x44, 0x33, 0x04, 0x00];
        assert_eq!(detect_media_type_from_bytes(mp3_signature), "audio/mpeg");
    }

    #[test]
    fn test_detect_media_type_mp3_frame_sync() {
        let mp3_signature = &[0xFF, 0xFB, 0x90, 0x00];
        assert_eq!(detect_media_type_from_bytes(mp3_signature), "audio/mpeg");
    }

    #[test]
    fn test_detect_media_type_wav() {
        let wav_signature = &[
            0x52, 0x49, 0x46, 0x46, // "RIFF"
            0x00, 0x00, 0x00, 0x00, // File size
            0x57, 0x41, 0x56, 0x45, // "WAVE"
        ];
        assert_eq!(detect_media_type_from_bytes(wav_signature), "audio/wav");
    }

    #[test]
    fn test_detect_media_type_ogg() {
        let ogg_signature = &[0x4F, 0x67, 0x67, 0x53];
        assert_eq!(detect_media_type_from_bytes(ogg_signature), "audio/ogg");
    }

    #[test]
    fn test_detect_media_type_flac() {
        let flac_signature = &[0x66, 0x4C, 0x61, 0x43];
        assert_eq!(detect_media_type_from_bytes(flac_signature), "audio/flac");
    }

    #[test]
    fn test_detect_media_type_aac() {
        let aac_signature = &[0xFF, 0xF1, 0x50, 0x80];
        assert_eq!(detect_media_type_from_bytes(aac_signature), "audio/aac");
    }

    #[test]
    fn test_detect_media_type_webm() {
        let webm_signature = &[0x1A, 0x45, 0xDF, 0xA3];
        assert_eq!(detect_media_type_from_bytes(webm_signature), "audio/webm");
    }

    #[test]
    fn test_detect_media_type_m4a() {
        let m4a_signature = &[0x00, 0x00, 0x00, 0x20, 0x66, 0x74, 0x79, 0x70];
        assert_eq!(detect_media_type_from_bytes(m4a_signature), "audio/mp4");
    }

    #[test]
    fn test_detect_media_type_unknown() {
        let unknown = &[0x00, 0x01, 0x02, 0x03];
        assert_eq!(detect_media_type_from_bytes(unknown), "audio/wav");
    }

    #[test]
    fn test_detect_media_type_short() {
        let short = &[0x00, 0x01];
        assert_eq!(detect_media_type_from_bytes(short), "audio/wav");
    }

    #[test]
    fn test_add_user_agent_suffix_no_existing() {
        let headers = add_user_agent_suffix(None, "ai/1.0.0".to_string());
        assert!(headers.is_some());
        assert_eq!(headers.unwrap().get("user-agent").unwrap(), "ai/1.0.0");
    }

    #[test]
    fn test_add_user_agent_suffix_with_existing() {
        let mut existing_headers = SharedHeaders::new();
        existing_headers.insert("user-agent".to_string(), "myapp/1.0".to_string());

        let headers = add_user_agent_suffix(Some(existing_headers), "ai/1.0.0".to_string());
        assert!(headers.is_some());
        assert_eq!(
            headers.unwrap().get("user-agent").unwrap(),
            "myapp/1.0 ai/1.0.0"
        );
    }

    #[test]
    fn test_add_user_agent_suffix_preserves_other_headers() {
        let mut existing_headers = SharedHeaders::new();
        existing_headers.insert("authorization".to_string(), "Bearer token123".to_string());

        let headers = add_user_agent_suffix(Some(existing_headers), "ai/1.0.0".to_string());
        let headers = headers.unwrap();
        assert_eq!(headers.get("user-agent").unwrap(), "ai/1.0.0");
        assert_eq!(headers.get("authorization").unwrap(), "Bearer token123");
    }
}
