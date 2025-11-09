use crate::shared::provider_options::SharedProviderOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio_util::sync;

/// Speech model call options.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeechModelCallOptions {
    /// Text to convert to speech.
    pub text: String,

    /// The voice to use for speech synthesis.
    /// This is provider-specific and may be a voice ID, name, or other identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,

    /// The desired output format for the audio e.g. "mp3", "wav", etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,

    /// Instructions for the speech generation e.g. "Speak in a slow and steady tone".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// The speed of the speech generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,

    /// The language for speech generation. This should be an ISO 639-1 language code
    /// (e.g. "en", "es", "fr") or "auto" for automatic language detection.
    /// Provider support varies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

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
    /// openai_options.insert("model".to_string(), json!("tts-1"));
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

impl SpeechModelCallOptions {
    /// Create new call options with text to convert to speech.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            voice: None,
            output_format: None,
            instructions: None,
            speed: None,
            language: None,
            provider_options: None,
            headers: None,
            abort_signal: None,
        }
    }

    // Builder methods
    /// Set the voice to use for synthesis
    pub fn with_voice(mut self, voice: impl Into<String>) -> Self {
        self.voice = Some(voice.into());
        self
    }

    /// Set the output audio format (e.g., "mp3", "wav")
    pub fn with_output_format(mut self, format: impl Into<String>) -> Self {
        self.output_format = Some(format.into());
        self
    }

    /// Set additional instructions for the model
    pub fn with_instructions(mut self, instructions: impl Into<String>) -> Self {
        self.instructions = Some(instructions.into());
        self
    }

    /// Set the playback speed (e.g., 1.0 for normal, 0.5 for half speed, 2.0 for double speed)
    pub fn with_speed(mut self, speed: f64) -> Self {
        self.speed = Some(speed);
        self
    }

    /// Set the language code for the speech (e.g., "en", "es", "fr")
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Set provider-specific options
    pub fn with_provider_options(mut self, options: SharedProviderOptions) -> Self {
        self.provider_options = Some(options);
        self
    }

    /// Set HTTP headers for the request
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Set an abort signal to cancel the request
    pub fn with_abort_signal(mut self, signal: sync::CancellationToken) -> Self {
        self.abort_signal = Some(signal);
        self
    }
}

// Convenience constructors for common audio formats
impl SpeechModelCallOptions {
    /// Create options with MP3 output format
    pub fn with_mp3(text: impl Into<String>) -> Self {
        Self::new(text).with_output_format("mp3")
    }

    /// Create options with WAV output format
    pub fn with_wav(text: impl Into<String>) -> Self {
        Self::new(text).with_output_format("wav")
    }

    /// Create options with OGG output format
    pub fn with_ogg(text: impl Into<String>) -> Self {
        Self::new(text).with_output_format("ogg")
    }

    /// Create options with FLAC output format
    pub fn with_flac(text: impl Into<String>) -> Self {
        Self::new(text).with_output_format("flac")
    }

    /// Create options with OPUS output format
    pub fn with_opus(text: impl Into<String>) -> Self {
        Self::new(text).with_output_format("opus")
    }
}

// Common language codes
impl SpeechModelCallOptions {
    /// Set language to auto-detect
    pub fn auto_language(mut self) -> Self {
        self.language = Some("auto".to_string());
        self
    }

    /// Set language to English
    pub fn english(mut self) -> Self {
        self.language = Some("en".to_string());
        self
    }

    /// Set language to Spanish
    pub fn spanish(mut self) -> Self {
        self.language = Some("es".to_string());
        self
    }

    /// Set language to French
    pub fn french(mut self) -> Self {
        self.language = Some("fr".to_string());
        self
    }

    /// Set language to German
    pub fn german(mut self) -> Self {
        self.language = Some("de".to_string());
        self
    }

    /// Set language to Italian
    pub fn italian(mut self) -> Self {
        self.language = Some("it".to_string());
        self
    }

    /// Set language to Portuguese
    pub fn portuguese(mut self) -> Self {
        self.language = Some("pt".to_string());
        self
    }

    /// Set language to Chinese
    pub fn chinese(mut self) -> Self {
        self.language = Some("zh".to_string());
        self
    }

    /// Set language to Japanese
    pub fn japanese(mut self) -> Self {
        self.language = Some("ja".to_string());
        self
    }

    /// Set language to Korean
    pub fn korean(mut self) -> Self {
        self.language = Some("ko".to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let options = SpeechModelCallOptions::new("Hello, world!");
        assert_eq!(options.text, "Hello, world!");
        assert_eq!(options.voice, None);
        assert_eq!(options.output_format, None);
    }

    #[test]
    fn test_with_voice() {
        let options = SpeechModelCallOptions::new("Hello").with_voice("alloy");
        assert_eq!(options.voice, Some("alloy".to_string()));
    }

    #[test]
    fn test_with_mp3() {
        let options = SpeechModelCallOptions::with_mp3("Hello");
        assert_eq!(options.output_format, Some("mp3".to_string()));
    }

    #[test]
    fn test_with_speed() {
        let options = SpeechModelCallOptions::new("Hello").with_speed(1.5);
        assert_eq!(options.speed, Some(1.5));
    }

    #[test]
    fn test_english() {
        let options = SpeechModelCallOptions::new("Hello").english();
        assert_eq!(options.language, Some("en".to_string()));
    }

    #[test]
    fn test_auto_language() {
        let options = SpeechModelCallOptions::new("Hello").auto_language();
        assert_eq!(options.language, Some("auto".to_string()));
    }

    #[test]
    fn test_builder_chain() {
        let options = SpeechModelCallOptions::new("Hello, world!")
            .with_voice("alloy")
            .with_output_format("mp3")
            .with_speed(1.2)
            .english()
            .with_instructions("Speak slowly");

        assert_eq!(options.text, "Hello, world!");
        assert_eq!(options.voice, Some("alloy".to_string()));
        assert_eq!(options.output_format, Some("mp3".to_string()));
        assert_eq!(options.speed, Some(1.2));
        assert_eq!(options.language, Some("en".to_string()));
        assert_eq!(options.instructions, Some("Speak slowly".to_string()));
    }
}
