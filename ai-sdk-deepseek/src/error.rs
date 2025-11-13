use thiserror::Error;

/// DeepSeek-specific error data from API responses.
///
/// Re-exports OpenAI-compatible error structure since DeepSeek uses
/// the same error format.
pub use ai_sdk_openai_compatible::OpenAICompatibleErrorData as DeepSeekErrorData;

/// DeepSeek provider errors.
#[derive(Error, Debug)]
pub enum DeepSeekError {
    /// API error from DeepSeek
    #[error("DeepSeek API error: {0}")]
    ApiError(String),

    /// Network or HTTP error
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Invalid configuration
    #[error("Configuration error: {0}")]
    ConfigError(String),
}
