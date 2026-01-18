/// Generated audio file types.
pub mod audio_file;
/// Result type for speech generation operations.
pub mod result;

pub use audio_file::{GeneratedAudioFile, GeneratedAudioFileWithType};
pub use result::{GenerateSpeechResult, audio_data_to_generated_audio_file};

use crate::error::AISDKError;
use crate::generate_text::prepare_retries;
use llm_kit_provider::shared::headers::SharedHeaders;
use llm_kit_provider::shared::provider_options::SharedProviderOptions;
use llm_kit_provider::speech_model::SpeechModel;
use llm_kit_provider::speech_model::call_options::SpeechModelCallOptions;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Builder for generating speech audio using a speech model.
///
/// # Example
///
/// ```no_run
/// use llm_kit_core::GenerateSpeech;
/// # use std::sync::Arc;
/// # use llm_kit_provider::speech_model::SpeechModel;
/// # async fn example(model: Arc<dyn SpeechModel>) -> Result<(), Box<dyn std::error::Error>> {
///
/// let result = GenerateSpeech::new(model, "Hello, welcome to the AI SDK!".to_string())
///     .voice("alloy")
///     .output_format("mp3")
///     .speed(1.0)
///     .language("en")
///     .max_retries(3)
///     .execute()
///     .await?;
///
/// println!("Generated audio format: {}", result.audio.format);
/// # Ok(())
/// # }
/// ```
pub struct GenerateSpeech {
    model: Arc<dyn SpeechModel>,
    text: String,
    voice: Option<String>,
    output_format: Option<String>,
    instructions: Option<String>,
    speed: Option<f64>,
    language: Option<String>,
    provider_options: Option<SharedProviderOptions>,
    max_retries: Option<u32>,
    abort_signal: Option<CancellationToken>,
    headers: Option<SharedHeaders>,
}

impl GenerateSpeech {
    /// Create a new GenerateSpeech builder.
    ///
    /// # Arguments
    ///
    /// * `model` - The speech model to use
    /// * `text` - The text to convert to speech
    pub fn new(model: Arc<dyn SpeechModel>, text: String) -> Self {
        Self {
            model,
            text,
            voice: None,
            output_format: None,
            instructions: None,
            speed: None,
            language: None,
            provider_options: None,
            max_retries: None,
            abort_signal: None,
            headers: None,
        }
    }

    /// Set the voice to use for speech generation.
    ///
    /// # Arguments
    ///
    /// * `voice` - The voice identifier (e.g., "alloy", "echo", "nova")
    pub fn voice(mut self, voice: impl Into<String>) -> Self {
        self.voice = Some(voice.into());
        self
    }

    /// Set the output format for the generated audio.
    ///
    /// # Arguments
    ///
    /// * `output_format` - The output format (e.g., "mp3", "wav", "opus")
    pub fn output_format(mut self, output_format: impl Into<String>) -> Self {
        self.output_format = Some(output_format.into());
        self
    }

    /// Set instructions for the speech generation.
    ///
    /// # Arguments
    ///
    /// * `instructions` - Instructions for the model (e.g., "Speak in a slow and steady tone")
    pub fn instructions(mut self, instructions: impl Into<String>) -> Self {
        self.instructions = Some(instructions.into());
        self
    }

    /// Set the speed of the speech generation.
    ///
    /// # Arguments
    ///
    /// * `speed` - The speed multiplier (e.g., 1.0 for normal, 0.5 for half speed, 2.0 for double speed)
    pub fn speed(mut self, speed: f64) -> Self {
        self.speed = Some(speed);
        self
    }

    /// Set the language for speech generation.
    ///
    /// # Arguments
    ///
    /// * `language` - ISO 639-1 language code (e.g., "en", "es", "fr") or "auto" for automatic detection
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
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

    /// Execute the speech generation operation.
    ///
    /// # Returns
    ///
    /// A result object that contains the generated audio data.
    pub async fn execute(self) -> Result<GenerateSpeechResult, AISDKError> {
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

        // Execute the model call with retry logic
        let result = retry_config
            .execute_with_boxed_error(move || {
                let model = self.model.clone();
                let text = self.text.clone();
                let voice = self.voice.clone();
                let output_format = self.output_format.clone();
                let instructions = self.instructions.clone();
                let speed = self.speed;
                let language = self.language.clone();
                let provider_options = self.provider_options.clone();
                let headers = headers_with_user_agent.clone();
                async move {
                    let options = SpeechModelCallOptions {
                        text,
                        voice,
                        output_format,
                        instructions,
                        speed,
                        language,
                        provider_options,
                        headers,
                        abort_signal: None,
                    };
                    model.do_generate(options).await
                }
            })
            .await?;

        // Check if audio was generated
        if result.audio.is_empty() {
            return Err(AISDKError::no_speech_generated(vec![
                result.response.clone(),
            ]));
        }

        // Detect media type from audio data
        let media_type = detect_audio_media_type(&result.audio);

        // Convert audio data to GeneratedAudioFile
        let audio = audio_data_to_generated_audio_file(result.audio, media_type);

        // Build the result
        let mut speech_result = GenerateSpeechResult::new(audio, vec![result.response]);

        if !result.warnings.is_empty() {
            speech_result = speech_result.with_warnings(result.warnings);
        }

        if let Some(metadata) = result.provider_metadata {
            speech_result = speech_result.with_provider_metadata(metadata);
        }

        Ok(speech_result)
    }
}

/// Detect media type from audio data.
///
/// Detects the media type based on the magic bytes at the start of the data.
fn detect_audio_media_type(audio_data: &llm_kit_provider::speech_model::AudioData) -> String {
    use llm_kit_provider::speech_model::AudioData;

    match audio_data {
        AudioData::Base64(base64_str) => {
            // Decode base64 and check magic bytes
            use base64::Engine;
            if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(base64_str) {
                detect_media_type_from_bytes(&bytes)
            } else {
                "audio/mp3".to_string() // Default fallback
            }
        }
        AudioData::Binary(bytes) => detect_media_type_from_bytes(bytes),
    }
}

/// Detect media type from raw bytes based on magic bytes.
fn detect_media_type_from_bytes(bytes: &[u8]) -> String {
    // Check ID3v2 tag (MP3 with metadata) - only needs 3 bytes
    if bytes.starts_with(b"ID3") {
        return "audio/mpeg".to_string();
    }

    if bytes.len() < 4 {
        return "audio/mp3".to_string();
    }

    // Check WAV signature (RIFF header with WAVE format)
    if bytes.len() >= 12
        && bytes[0..4] == [0x52, 0x49, 0x46, 0x46] // "RIFF"
        && bytes[8..12] == [0x57, 0x41, 0x56, 0x45]
    // "WAVE"
    {
        return "audio/wav".to_string();
    }

    // Check OGG signature
    if bytes.starts_with(b"OggS") {
        return "audio/ogg".to_string();
    }

    // Check FLAC signature
    if bytes.starts_with(b"fLaC") {
        return "audio/flac".to_string();
    }

    // Check WebM/Matroska signature (EBML)
    if bytes.len() >= 4 && bytes[0..4] == [0x1A, 0x45, 0xDF, 0xA3] {
        return "audio/webm".to_string();
    }

    // Check for frame sync patterns (0xFF followed by specific bits)
    if bytes.len() >= 2 && bytes[0] == 0xFF {
        // AAC ADTS: 0xFF followed by 0xF0-0xFF (1111xxxx)
        // MP3 MPEG: 0xFF followed by 0xE0-0xFF (111xxxxx) but more specifically:
        //   - MPEG-1 Layer 3: 0xFF 0xFB
        //   - Other MPEG audio: 0xFF 0xF2, 0xF3, 0xFA, etc.

        // AAC has more strict pattern: 0xFFF (12 bits set)
        if bytes[1] & 0xF6 == 0xF0 {
            return "audio/aac".to_string();
        }

        // MP3/MPEG audio frame sync
        if bytes[1] & 0xE0 == 0xE0 {
            return "audio/mpeg".to_string();
        }
    }

    // Default to MP3
    "audio/mp3".to_string()
}

/// Add a user agent suffix to headers.
fn add_user_agent_suffix(headers: Option<SharedHeaders>, suffix: String) -> Option<SharedHeaders> {
    let mut headers = headers.unwrap_or_default();

    // Get existing user agent or use empty string
    let existing_user_agent = headers.get("user-agent").cloned().unwrap_or_default();

    // Add suffix
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
    use llm_kit_provider::speech_model::AudioData;

    #[test]
    fn test_detect_media_type_mp3_id3() {
        let mp3_signature = b"ID3\x04\x00\x00\x00\x00\x00\x00";
        assert_eq!(detect_media_type_from_bytes(mp3_signature), "audio/mpeg");
    }

    #[test]
    fn test_detect_media_type_mp3_frame_sync() {
        let mp3_signature = vec![0xFF, 0xFB, 0x90, 0x00];
        assert_eq!(detect_media_type_from_bytes(&mp3_signature), "audio/mpeg");
    }

    #[test]
    fn test_detect_media_type_wav() {
        let wav_signature = vec![
            0x52, 0x49, 0x46, 0x46, // "RIFF"
            0x00, 0x00, 0x00, 0x00, // file size (placeholder)
            0x57, 0x41, 0x56, 0x45, // "WAVE"
        ];
        assert_eq!(detect_media_type_from_bytes(&wav_signature), "audio/wav");
    }

    #[test]
    fn test_detect_media_type_ogg() {
        let ogg_signature = b"OggS\x00\x02\x00\x00\x00\x00\x00\x00";
        assert_eq!(detect_media_type_from_bytes(ogg_signature), "audio/ogg");
    }

    #[test]
    fn test_detect_media_type_flac() {
        let flac_signature = b"fLaC\x00\x00\x00\x22";
        assert_eq!(detect_media_type_from_bytes(flac_signature), "audio/flac");
    }

    #[test]
    fn test_detect_media_type_aac() {
        let aac_signature = vec![0xFF, 0xF1, 0x50, 0x80];
        assert_eq!(detect_media_type_from_bytes(&aac_signature), "audio/aac");
    }

    #[test]
    fn test_detect_media_type_webm() {
        let webm_signature = vec![0x1A, 0x45, 0xDF, 0xA3];
        assert_eq!(detect_media_type_from_bytes(&webm_signature), "audio/webm");
    }

    #[test]
    fn test_detect_media_type_unknown() {
        let unknown = vec![0x00, 0x01, 0x02, 0x03];
        assert_eq!(detect_media_type_from_bytes(&unknown), "audio/mp3");
    }

    #[test]
    fn test_detect_media_type_short() {
        let short = vec![0x00];
        assert_eq!(detect_media_type_from_bytes(&short), "audio/mp3");
    }

    #[test]
    fn test_detect_audio_media_type_base64() {
        // Base64 encoded "ID3" prefix (MP3 with ID3 tag)
        // "SUQz" decodes to "ID3" which should be detected as audio/mpeg
        let audio_data = AudioData::from_base64("SUQz");
        let result = detect_audio_media_type(&audio_data);
        // The base64 "SUQz" decodes to bytes [0x49, 0x44, 0x33] which is "ID3"
        assert_eq!(result, "audio/mpeg");
    }

    #[test]
    fn test_detect_audio_media_type_binary() {
        let audio_data = AudioData::from_binary(b"fLaC\x00\x00\x00\x22".to_vec());
        assert_eq!(detect_audio_media_type(&audio_data), "audio/flac");
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
