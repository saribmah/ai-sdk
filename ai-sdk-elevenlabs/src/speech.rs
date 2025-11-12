/// API types for ElevenLabs speech API.
pub mod api_types;

/// Speech model implementation.
pub mod model;

/// Speech model options and identifiers.
pub mod options;

/// Provider-specific options for speech generation.
pub mod provider_options;

pub use model::ElevenLabsSpeechModel;
pub use options::{ElevenLabsSpeechModelId, ElevenLabsSpeechVoiceId};
pub use provider_options::ElevenLabsSpeechProviderOptions;
