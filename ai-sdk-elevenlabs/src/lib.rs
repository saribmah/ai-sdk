//! ElevenLabs provider for the AI SDK.
//!
//! This crate provides an ElevenLabs provider implementation for the AI SDK,
//! supporting text-to-speech and speech-to-text models.
//!
//! # Features
//!
//! - **Text-to-Speech (TTS)**: Convert text to natural-sounding speech with multiple voices
//! - **Speech-to-Text (STT)**: Transcribe audio files to text with timestamps
//! - **Voice Settings**: Fine-tune stability, similarity, and style
//! - **Speaker Diarization**: Identify different speakers in transcriptions
//! - **Multiple Languages**: Support for 29+ languages
//!
//! # Examples
//!
//! ## Basic Text-to-Speech
//!
//! ```no_run
//! use ai_sdk_elevenlabs::{create_elevenlabs, ElevenLabsProviderSettings};
//! use ai_sdk_provider::Provider;
//! use ai_sdk_core::GenerateSpeech;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create provider
//! let provider = create_elevenlabs();
//! let model = provider.speech_model("eleven_multilingual_v2")?;
//!
//! // Generate speech
//! let result = GenerateSpeech::new(model, "Hello, how are you?".to_string())
//!     .voice("21m00Tcm4TlvDq8ikWAM")  // Rachel voice
//!     .output_format("mp3")
//!     .execute()
//!     .await?;
//!
//! println!("Generated {} bytes of audio", result.audio.bytes().len());
//! # Ok(())
//! # }
//! ```
//!
//! ## Speech-to-Text Transcription
//!
//! ```ignore
//! use ai_sdk_elevenlabs::{create_elevenlabs, ElevenLabsProviderSettings};
//! use ai_sdk_provider::Provider;
//! use ai_sdk_core::{Transcribe, AudioInput};
//! use ai_sdk_provider_utils::DataContent;
//! use std::fs;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create provider
//! let provider = create_elevenlabs();
//! let model = provider.transcription_model("scribe_v1")?;
//!
//! // Read audio file
//! let audio_bytes = fs::read("audio.mp3")?;
//!
//! // Transcribe
//! let result = Transcribe::new(
//!     model,
//!     AudioInput::Data(DataContent::from_bytes(audio_bytes, "audio/mpeg"))
//! ).execute().await?;
//!
//! println!("Transcription: {}", result.text);
//! # Ok(())
//! # }
//! ```
//!
//! ## Custom Provider Settings
//!
//! ```no_run
//! use ai_sdk_elevenlabs::ElevenLabsProviderSettings;
//! use ai_sdk_elevenlabs::create_elevenlabs_with_settings;
//!
//! let settings = ElevenLabsProviderSettings::new()
//!     .with_api_key("your-api-key")
//!     .with_base_url("https://api.elevenlabs.io");
//!
//! let provider = create_elevenlabs_with_settings(settings);
//! ```
//!
//! ## Provider-Specific Options
//!
//! ```ignore
//! use ai_sdk_elevenlabs::{create_elevenlabs, speech::ElevenLabsSpeechProviderOptions};
//! use ai_sdk_provider::Provider;
//! use ai_sdk_core::GenerateSpeech;
//! use std::collections::HashMap;
//! use serde_json::json;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = create_elevenlabs();
//! let model = provider.speech_model("eleven_multilingual_v2")?;
//!
//! // Provider options with voice settings
//! let mut provider_options = HashMap::new();
//! provider_options.insert(
//!     "elevenlabs".to_string(),
//!     json!({
//!         "voiceSettings": {
//!             "stability": 0.7,
//!             "similarityBoost": 0.8,
//!             "style": 0.5,
//!             "useSpeakerBoost": true
//!         },
//!         "seed": 12345
//!     })
//!     .as_object()
//!     .unwrap()
//!     .clone(),
//! );
//!
//! let result = GenerateSpeech::new(model, "Hello with custom voice settings!".to_string())
//!     .provider_options(provider_options)
//!     .execute()
//!     .await?;
//! # Ok(())
//! # }
//! ```

/// Configuration for ElevenLabs models.
pub mod config;

/// Error handling for ElevenLabs API.
pub mod error;

/// Provider implementation.
pub mod provider;

/// Provider settings and configuration.
pub mod settings;

/// Speech (text-to-speech) model implementation.
pub mod speech;

/// Transcription (speech-to-text) model implementation.
pub mod transcription;

// Re-export main types
pub use provider::{ElevenLabsProvider, create_elevenlabs, create_elevenlabs_with_settings};
pub use settings::ElevenLabsProviderSettings;

// Re-export speech types
pub use speech::{
    ElevenLabsSpeechModel, ElevenLabsSpeechModelId, ElevenLabsSpeechProviderOptions,
    ElevenLabsSpeechVoiceId,
};

// Re-export transcription types
pub use transcription::{
    ElevenLabsTranscriptionModel, ElevenLabsTranscriptionModelId,
    ElevenLabsTranscriptionProviderOptions,
};
