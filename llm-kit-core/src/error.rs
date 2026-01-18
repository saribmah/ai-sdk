use std::time::Duration;
use thiserror::Error;

mod invalid_argument;
mod invalid_prompt;
mod invalid_stream_part;
mod invalid_tool_input;
mod model_error;
mod no_image_generated;
mod no_output_generated;
mod no_speech_generated;
mod no_such_tool;
mod no_transcript_generated;
mod retryable_error;
mod unsupported_model_version;

// Re-export builders for public API
pub use invalid_argument::InvalidArgumentErrorBuilder;
pub use invalid_prompt::InvalidPromptErrorBuilder;
pub use invalid_stream_part::InvalidStreamPartErrorBuilder;
pub use invalid_tool_input::InvalidToolInputErrorBuilder;
pub use model_error::ModelErrorBuilder;
pub use no_image_generated::NoImageGeneratedErrorBuilder;
pub use no_output_generated::NoOutputGeneratedErrorBuilder;
pub use no_speech_generated::NoSpeechGeneratedErrorBuilder;
pub use no_such_tool::NoSuchToolErrorBuilder;
pub use no_transcript_generated::NoTranscriptGeneratedErrorBuilder;
pub use retryable_error::RetryableErrorBuilder;
pub use unsupported_model_version::UnsupportedModelVersionErrorBuilder;

/// Errors that can occur when working with the LLM Kit Core.
///
/// This enum represents all possible errors that can occur when working with
/// the core LLM Kit functionality. Each variant contains specific information
/// relevant to that error type.
///
/// # Examples
///
/// ## Pattern matching on error types
///
/// ```
/// use llm_kit_core::error::AISDKError;
///
/// let error = AISDKError::invalid_argument("temperature", 3.0, "must be between 0 and 2");
///
/// match &error {
///     AISDKError::InvalidArgument { parameter, value, message } => {
///         println!("Invalid argument '{}': {} (value: {})", parameter, message, value);
///     }
///     AISDKError::ModelError { message } => {
///         println!("Model error: {}", message);
///     }
///     AISDKError::RetryableError { message, retry_after } => {
///         println!("Retryable error: {}", message);
///         if let Some(delay) = retry_after {
///             println!("Retry after: {:?}", delay);
///         }
///     }
///     _ => {
///         println!("Other error: {}", error);
///     }
/// }
/// ```
///
/// ## Using builder patterns
///
/// ```
/// use llm_kit_core::error::AISDKError;
/// use std::time::Duration;
///
/// let error = AISDKError::retryable_error_builder("Rate limited")
///     .retry_after(Duration::from_secs(60))
///     .build();
/// ```
#[derive(Debug, Clone, Error)]
pub enum AISDKError {
    /// An invalid argument error.
    ///
    /// This error occurs when a function argument is invalid.
    #[error("Invalid argument for parameter '{parameter}': {message} (value: {value})")]
    InvalidArgument {
        /// The name of the parameter that has an invalid value
        parameter: String,
        /// The invalid value (serialized as string for debugging)
        value: String,
        /// Description of why the value is invalid
        message: String,
    },

    /// An invalid prompt error.
    ///
    /// This error occurs when a prompt is invalid and cannot be processed.
    #[error("Invalid prompt: {message}")]
    InvalidPrompt {
        /// The error message
        message: String,
    },

    /// An invalid stream part error.
    ///
    /// This error occurs when a stream part is invalid or unexpected.
    #[error("Invalid stream part: {message}")]
    InvalidStreamPart {
        /// The invalid stream chunk (serialized as string for debugging)
        chunk: String,
        /// The error message
        message: String,
    },

    /// A model error.
    ///
    /// This error occurs when there's an error with the model initialization or usage.
    #[error("Model error: {message}")]
    ModelError {
        /// The error message
        message: String,
    },

    /// A retryable error.
    ///
    /// This error indicates that the operation can be retried, potentially after
    /// a delay specified in `retry_after`.
    #[error("Retryable error: {message}")]
    RetryableError {
        /// The error message
        message: String,
        /// Optional retry delay hint from the provider (e.g., from Retry-After header)
        retry_after: Option<Duration>,
    },

    /// An invalid tool input error.
    ///
    /// This error occurs when a tool receives invalid input.
    #[error("Invalid tool input for tool '{tool_name}': {message}")]
    InvalidToolInput {
        /// The name of the tool that received invalid input
        tool_name: String,
        /// The invalid input (serialized as string for debugging)
        tool_input: String,
        /// Description of why the input is invalid
        message: String,
    },

    /// A no such tool error.
    ///
    /// This error occurs when a requested tool is not available.
    #[error("No such tool: '{tool_name}'. Available tools: {available_tools:?}")]
    NoSuchTool {
        /// The name of the tool that was not found
        tool_name: String,
        /// List of available tool names
        available_tools: Vec<String>,
    },

    /// A no image generated error.
    ///
    /// This error occurs when no image could be generated. This can have multiple causes:
    /// - The model failed to generate a response.
    /// - The model generated a response that could not be parsed.
    #[error("No image generated: {message}")]
    NoImageGenerated {
        /// The error message
        message: String,
        /// Response metadata from provider calls (if available)
        responses: Option<Vec<crate::generate_image::ImageModelResponseMetadata>>,
    },

    /// A no output generated error.
    ///
    /// This error occurs when no LLM output was generated, e.g. because of errors.
    #[error("No output generated: {message}")]
    NoOutputGenerated {
        /// The error message
        message: String,
    },

    /// A no speech generated error.
    ///
    /// This error occurs when no speech audio was generated.
    #[error("No speech audio generated: {message}")]
    NoSpeechGenerated {
        /// The error message
        message: String,
        /// Response metadata from provider calls
        responses: Vec<llm_kit_provider::speech_model::SpeechModelResponseMetadata>,
    },

    /// A no transcript generated error.
    ///
    /// This error occurs when no transcript was generated.
    #[error("No transcript generated: {message}")]
    NoTranscriptGenerated {
        /// The error message
        message: String,
        /// Response metadata from provider calls
        responses: Vec<llm_kit_provider::transcription_model::TranscriptionModelResponseMetadata>,
    },

    /// An unsupported model version error.
    ///
    /// This error occurs when a model with an unsupported version is used.
    /// LLM Kit only supports models that implement specification version "v2".
    #[error(
        "Unsupported model version {version} for provider \"{provider}\" and model \"{model_id}\". LLM Kit only supports models that implement specification version \"v2\"."
    )]
    UnsupportedModelVersion {
        /// The unsupported version string
        version: String,
        /// The provider name
        provider: String,
        /// The model identifier
        model_id: String,
    },
}
