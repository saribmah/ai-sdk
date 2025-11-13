//! AssemblyAI provider implementation for the AI SDK.
//!
//! This crate provides a provider implementation for AssemblyAI's transcription API,
//! supporting speech-to-text functionality with advanced features like speaker
//! diarization, sentiment analysis, and content moderation.
//!
//! # Examples
//!
//! ## Basic Transcription
//!
//! ```no_run
//! use ai_sdk_assemblyai::{AssemblyAIClient, AssemblyAIProvider};
//! use ai_sdk_core::Transcribe;
//! use ai_sdk_provider::transcription_model::AudioInput;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a provider using the client builder
//! let provider = AssemblyAIClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.transcription_model("best");
//!
//! // Transcribe audio from a URL
//! let audio_url = "https://example.com/audio.mp3";
//! let audio_data = reqwest::get(audio_url).await?.bytes().await?;
//!
//! let result = Transcribe::new(model, AudioInput::Data(audio_data.to_vec()))
//!     .execute()
//!     .await?;
//!
//! println!("Transcription: {}", result.text);
//! # Ok(())
//! # }
//! ```
//!
//! ## Using Provider Options
//!
//! ```no_run
//! use ai_sdk_assemblyai::{AssemblyAIClient, AssemblyAIProvider};
//! use ai_sdk_core::Transcribe;
//! use ai_sdk_provider::transcription_model::AudioInput;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = AssemblyAIClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.transcription_model("best");
//!
//! let audio_url = "https://example.com/audio.mp3";
//! let audio_data = reqwest::get(audio_url).await?.bytes().await?;
//!
//! // Use provider options for advanced features
//! let result = Transcribe::new(model, AudioInput::Data(audio_data.to_vec()))
//!     .with_provider_options(serde_json::json!({
//!         "speakerLabels": true,
//!         "speakersExpected": 2,
//!         "sentimentAnalysis": true,
//!         "autoChapters": true
//!     }))
//!     .execute()
//!     .await?;
//!
//! println!("Transcription: {}", result.text);
//! println!("Segments: {}", result.segments.len());
//! # Ok(())
//! # }
//! ```

/// Client builder for creating AssemblyAI providers.
pub mod client;
/// Error types for AssemblyAI provider operations.
pub mod error;
/// Provider implementation and creation functions.
pub mod provider;
/// Settings and configuration for AssemblyAI providers.
pub mod settings;
/// Transcription model implementation.
pub mod transcription;

// Re-export main types
pub use client::AssemblyAIClient;
pub use error::AssemblyAIError;
pub use provider::{create_assemblyai, AssemblyAIProvider};
pub use settings::AssemblyAIProviderSettings;
pub use transcription::{
    AssemblyAITranscriptionModel, AssemblyAITranscriptionModelId, AssemblyAITranscriptionOptions,
};

/// The version of this crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
