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
//! # Quick Start
//!
//! ## Recommended: Using the Builder Pattern
//!
//! ```no_run
//! use ai_sdk_elevenlabs::ElevenLabsClient;
//! use ai_sdk_provider::{Provider, SpeechModel, SpeechSettings};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create provider using builder
//! let provider = ElevenLabsClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.speech_model("eleven_multilingual_v2")?;
//!
//! // Generate speech
//! let settings = SpeechSettings::default()
//!     .with_voice("21m00Tcm4TlvDq8ikWAM")  // Rachel voice
//!     .with_output_format("mp3");
//!
//! let result = model.do_generate("Hello, how are you?", settings).await?;
//!
//! println!("Generated {} bytes of audio", result.audio.bytes().len());
//! # Ok(())
//! # }
//! ```
//!
//! ## Alternative: Direct Instantiation
//!
//! ```no_run
//! use ai_sdk_elevenlabs::{ElevenLabsProvider, ElevenLabsProviderSettings};
//! use ai_sdk_provider::Provider;
//!
//! let provider = ElevenLabsProvider::new(
//!     ElevenLabsProviderSettings::new()
//!         .with_api_key("your-api-key")
//! );
//! ```
//!
//! # Examples
//!
//! ## Text-to-Speech
//!
//! ```no_run
//! use ai_sdk_elevenlabs::ElevenLabsClient;
//! use ai_sdk_provider::{Provider, SpeechModel, SpeechSettings};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = ElevenLabsClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.speech_model("eleven_multilingual_v2")?;
//!
//! let settings = SpeechSettings::default()
//!     .with_voice("21m00Tcm4TlvDq8ikWAM")
//!     .with_output_format("mp3");
//!
//! let result = model.do_generate("Hello, world!", settings).await?;
//!
//! std::fs::write("output.mp3", result.audio.bytes())?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Speech-to-Text Transcription
//!
//! ```ignore
//! use ai_sdk_elevenlabs::ElevenLabsClient;
//! use ai_sdk_provider::{Provider, TranscriptionModel, TranscriptionInput, TranscriptionSettings};
//! use ai_sdk_provider_utils::DataContent;
//! use std::fs;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = ElevenLabsClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.transcription_model("scribe_v1")?;
//!
//! let audio_bytes = fs::read("audio.mp3")?;
//! let input = TranscriptionInput::Data(DataContent::from_bytes(audio_bytes, "audio/mpeg"));
//! let result = model.do_transcribe(input, TranscriptionSettings::default()).await?;
//!
//! println!("Transcription: {}", result.text);
//! # Ok(())
//! # }
//! ```
//!
//! ## Custom Configuration
//!
//! ```no_run
//! use ai_sdk_elevenlabs::ElevenLabsClient;
//!
//! let provider = ElevenLabsClient::new()
//!     .api_key("your-api-key")
//!     .base_url("https://custom-api.elevenlabs.io")
//!     .header("X-Custom-Header", "custom-value")
//!     .build();
//! ```
//!
//! ## Provider-Specific Options
//!
//! ```ignore
//! use ai_sdk_elevenlabs::ElevenLabsClient;
//! use ai_sdk_provider::{Provider, SpeechModel, SpeechSettings};
//! use std::collections::HashMap;
//! use serde_json::json;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = ElevenLabsClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
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
//! let settings = SpeechSettings::default().with_provider_options(provider_options);
//! let result = model.do_generate("Hello with custom voice settings!", settings).await?;
//! # Ok(())
//! # }
//! ```

/// Builder for creating ElevenLabs provider.
pub mod client;

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
pub use client::ElevenLabsClient;
pub use provider::ElevenLabsProvider;
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
