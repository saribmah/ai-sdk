use thiserror::Error;

/// Groq-specific error data from API responses.
///
/// Re-exports OpenAI-compatible error structure since Groq uses
/// the same error format.
pub use llm_kit_openai_compatible::OpenAICompatibleErrorData as GroqErrorData;

/// Groq provider errors.
#[derive(Error, Debug)]
pub enum GroqError {
    /// API error from Groq
    #[error("Groq API error: {0}")]
    ApiError(String),

    /// Network or HTTP error
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Invalid configuration
    #[error("Configuration error: {0}")]
    ConfigError(String),
}
