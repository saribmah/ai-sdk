//! AssemblyAI provider implementation for the AI SDK.
//!
//! This crate provides a provider implementation for AssemblyAI's transcription API,
//! supporting speech-to-text functionality with advanced features like speaker
//! diarization, sentiment analysis, and content moderation.
//!
//! # Examples
//!
//! ## Basic Transcription (Provider-Only)
//!
//! ```no_run
//! use llm_kit_assemblyai::AssemblyAIClient;
//! use llm_kit_provider::transcription_model::call_options::TranscriptionModelCallOptions;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a provider using the client builder
//! let provider = AssemblyAIClient::new()
//!     .api_key("your-api-key")
//!     .build();
//!
//! let model = provider.transcription_model("best");
//!
//! // Download audio from a URL
//! let audio_url = "https://example.com/audio.mp3";
//! let audio_data = reqwest::get(audio_url).await?.bytes().await?;
//!
//! // Create call options with mp3 audio
//! let call_options = TranscriptionModelCallOptions::mp3(audio_data.to_vec());
//!
//! // Call do_generate() directly (provider trait method)
//! let result = model.do_generate(call_options).await?;
//!
//! println!("Transcription: {}", result.text);
//! # Ok(())
//! # }
//! ```
//!
//! ## Using Provider Options
//!
//! ```no_run
//! use llm_kit_assemblyai::AssemblyAIClient;
//! use llm_kit_provider::transcription_model::call_options::TranscriptionModelCallOptions;
//! use llm_kit_provider::shared::provider_options::SharedProviderOptions;
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
//! // Configure provider options for advanced features
//! let mut provider_options = SharedProviderOptions::new();
//! provider_options.insert(
//!     "assemblyai".to_string(),
//!     vec![
//!         ("speakerLabels".to_string(), serde_json::json!(true)),
//!         ("speakersExpected".to_string(), serde_json::json!(2)),
//!         ("sentimentAnalysis".to_string(), serde_json::json!(true)),
//!         ("autoChapters".to_string(), serde_json::json!(true)),
//!     ]
//!     .into_iter()
//!     .collect(),
//! );
//!
//! let call_options = TranscriptionModelCallOptions::mp3(audio_data.to_vec())
//!     .with_provider_options(provider_options);
//!
//! let result = model.do_generate(call_options).await?;
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
pub use provider::AssemblyAIProvider;
pub use settings::AssemblyAIProviderSettings;
pub use transcription::{
    AssemblyAITranscriptionModel, AssemblyAITranscriptionModelId, AssemblyAITranscriptionOptions,
};

/// The version of this crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
