use std::collections::HashMap;
use thiserror::Error;

mod api_call;
mod empty_response_body;
mod invalid_argument;
mod invalid_prompt;
mod invalid_response_data;
mod json_parse;
mod load_api_key;
mod load_setting;
mod model_error;
mod no_content_generated;
mod no_such_model;
mod too_many_embedding_values_for_call;
mod type_validation;
mod unsupported_functionality;

// Re-export builders for public API
pub use api_call::APICallErrorBuilder;
pub use empty_response_body::EmptyResponseBodyErrorBuilder;
pub use invalid_argument::InvalidArgumentErrorBuilder;
pub use invalid_prompt::InvalidPromptErrorBuilder;
pub use invalid_response_data::InvalidResponseDataErrorBuilder;
pub use json_parse::JSONParseErrorBuilder;
pub use load_api_key::LoadAPIKeyErrorBuilder;
pub use load_setting::LoadSettingErrorBuilder;
pub use model_error::ModelErrorBuilder;
pub use no_content_generated::NoContentGeneratedErrorBuilder;
pub use no_such_model::NoSuchModelErrorBuilder;
pub use too_many_embedding_values_for_call::TooManyEmbeddingValuesForCallErrorBuilder;
pub use type_validation::TypeValidationErrorBuilder;
pub use unsupported_functionality::UnsupportedFunctionalityErrorBuilder;

/// Errors that can occur when working with providers.
///
/// This enum represents all possible errors that can occur when working with AI providers.
/// Each variant contains specific information relevant to that error type.
///
/// # Examples
///
/// ## Pattern matching on error types
///
/// ```
/// use ai_sdk_provider::error::ProviderError;
///
/// let error = ProviderError::api_call_error(
///     "Failed to connect",
///     "https://api.example.com",
///     "{}",
/// );
///
/// match &error {
///     ProviderError::APICallError {
///         url,
///         status_code,
///         is_retryable,
///         ..
///     } => {
///         println!("API call to {} failed", url);
///         if *is_retryable {
///             println!("This error can be retried");
///         }
///     }
///     ProviderError::NoSuchModel { model_id, .. } => {
///         println!("Model {} not found", model_id);
///     }
///     ProviderError::ModelError { message, .. } => {
///         println!("Model error: {}", message);
///     }
///     ProviderError::EmptyResponseBody { message } => {
///         println!("Empty response: {}", message);
///     }
///     _ => {
///         println!("Other error: {}", error);
///     }
/// }
/// ```
///
/// ## Using helper methods
///
/// Helper methods like `is_retryable()`, `status_code()`, `response_body()`, and `url()`
/// work across all error variants, returning appropriate values for API call errors
/// and `None`/`false` for other error types.
///
/// ```
/// use ai_sdk_provider::error::ProviderError;
///
/// let api_error = ProviderError::api_call_error_with_details(
///     "Rate limited",
///     "https://api.example.com",
///     "{}",
///     Some(429),
///     None,
///     None,
///     None,
///     None,
///     None,
/// );
///
/// if api_error.is_retryable() {
///     println!("Error can be retried");
///     if let Some(code) = api_error.status_code() {
///         println!("Status code: {}", code);
///     }
/// }
///
/// // Helper methods work on all variants
/// let model_error = ProviderError::model_error("test");
/// assert!(!model_error.is_retryable());
/// assert_eq!(model_error.status_code(), None);
/// ```
#[derive(Debug, Error)]
pub enum ProviderError {
    /// The requested model was not found in this provider.
    #[error("No such model: {model_id} (provider: {provider_id})")]
    NoSuchModel {
        /// The ID of the model that was not found
        model_id: String,
        /// The ID of the provider
        provider_id: String,
        /// Optional message with more details
        #[source]
        message: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// An error occurred while initializing or using the model.
    #[error("Model error: {message}")]
    ModelError {
        /// The error message
        message: String,
        /// Optional source error
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// An API call error with detailed information about the failed request.
    ///
    /// This error type contains comprehensive information about a failed API call,
    /// including the request details, response information, and whether the error
    /// is retryable.
    ///
    /// # Retryability
    ///
    /// The error automatically determines if it's retryable based on the HTTP status code:
    /// - 408 (Request Timeout) → retryable
    /// - 409 (Conflict) → retryable
    /// - 429 (Too Many Requests) → retryable
    /// - 500+ (Server Errors) → retryable
    ///
    /// This can be overridden by explicitly setting `is_retryable` when constructing the error.
    #[error("API call failed: {message}")]
    APICallError {
        /// The error message
        message: String,
        /// The URL that was called
        url: String,
        /// The request body values (serialized as string for debugging)
        request_body_values: String,
        /// HTTP status code (if available)
        status_code: Option<u16>,
        /// Response headers (if available)
        response_headers: Option<HashMap<String, String>>,
        /// Response body (if available)
        response_body: Option<String>,
        /// Whether this error is retryable
        is_retryable: bool,
        /// Additional error data (serialized as string)
        data: Option<String>,
        /// Optional source error
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// An empty response body error.
    ///
    /// This error occurs when a response is expected to have a body but receives
    /// an empty response instead. This is typically an indication of a protocol
    /// error or an unexpected server behavior.
    #[error("Empty response body: {message}")]
    EmptyResponseBody {
        /// The error message
        message: String,
    },

    /// An invalid argument error.
    ///
    /// This error occurs when a function argument is invalid.
    #[error("Invalid argument '{argument}': {message}")]
    InvalidArgument {
        /// The name of the invalid argument
        argument: String,
        /// The error message
        message: String,
        /// Optional cause error
        #[source]
        cause: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// An invalid prompt error.
    ///
    /// This error occurs when a prompt is invalid and cannot be processed by the provider.
    #[error("Invalid prompt: {message}")]
    InvalidPrompt {
        /// The error message
        message: String,
        /// The invalid prompt (serialized as string for debugging)
        prompt: String,
        /// Optional cause error
        #[source]
        cause: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// An invalid response data error.
    ///
    /// This error occurs when the server returns a response with invalid data content
    /// that cannot be parsed or processed by the provider.
    #[error("Invalid response data: {message}")]
    InvalidResponseData {
        /// The error message
        message: String,
        /// The invalid data (serialized as string for debugging)
        data: String,
    },

    /// A JSON parse error.
    ///
    /// This error occurs when JSON parsing fails.
    #[error("JSON parsing failed: Text: {text}")]
    JSONParse {
        /// The text that failed to parse
        text: String,
        /// Optional cause error
        #[source]
        cause: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// A load API key error.
    ///
    /// This error occurs when an API key cannot be loaded from the environment
    /// or configuration.
    #[error("Load API key error: {message}")]
    LoadAPIKey {
        /// The error message
        message: String,
    },

    /// A load setting error.
    ///
    /// This error occurs when a configuration setting cannot be loaded.
    #[error("Load setting error: {message}")]
    LoadSetting {
        /// The error message
        message: String,
    },

    /// A no content generated error.
    ///
    /// This error occurs when the AI provider fails to generate any content.
    #[error("No content generated: {message}")]
    NoContentGenerated {
        /// The error message
        message: String,
    },

    /// A too many embedding values for call error.
    ///
    /// This error occurs when too many values are provided for a single embedding call,
    /// exceeding the provider's limit.
    #[error(
        "Too many embedding values for call: The {provider} model '{model_id}' can only embed up to {max_embeddings_per_call} values per call, but {values_count} values were provided"
    )]
    TooManyEmbeddingValuesForCall {
        /// The provider name
        provider: String,
        /// The model ID
        model_id: String,
        /// Maximum embeddings allowed per call
        max_embeddings_per_call: usize,
        /// Number of values provided
        values_count: usize,
    },

    /// A type validation error.
    ///
    /// This error occurs when type validation fails.
    #[error("Type validation failed: Value: {value}")]
    TypeValidation {
        /// The value that failed validation (serialized as string for debugging)
        value: String,
        /// Optional cause error
        #[source]
        cause: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// An unsupported functionality error.
    ///
    /// This error occurs when a requested functionality is not supported by the provider or model.
    #[error("Unsupported functionality: {message}")]
    UnsupportedFunctionality {
        /// The name of the unsupported functionality
        functionality: String,
        /// The error message
        message: String,
    },
}
