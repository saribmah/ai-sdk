/// API types for ElevenLabs transcription API.
pub mod api_types;

/// Transcription model implementation.
pub mod model;

/// Transcription model options and identifiers.
pub mod options;

/// Provider-specific options for transcription.
pub mod provider_options;

pub use model::ElevenLabsTranscriptionModel;
pub use options::ElevenLabsTranscriptionModelId;
pub use provider_options::ElevenLabsTranscriptionProviderOptions;
