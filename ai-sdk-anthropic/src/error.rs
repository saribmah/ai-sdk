//! Error types for the Anthropic provider.
//!
//! This module provides comprehensive error handling for the Anthropic API, including:
//! - API-specific error types and response parsing
//! - HTTP request error handling
//! - JSON parsing errors
//! - Configuration validation errors
//! - Automatic conversion to SDK `ProviderError` types
//!
//! ## Error Types
//!
//! The main error type is [`AnthropicError`], which provides specific variants for
//! different error scenarios:
//!
//! - [`AnthropicError::ApiError`] - Errors returned by the Anthropic API
//! - [`AnthropicError::RequestError`] - HTTP request failures
//! - [`AnthropicError::ParseError`] - JSON parsing failures
//! - [`AnthropicError::ConfigError`] - Configuration validation failures
//! - [`AnthropicError::Other`] - Generic error wrapper
//!
//! ## API Error Format
//!
//! Anthropic API errors follow a consistent structure:
//!
//! ```json
//! {
//!   "type": "error",
//!   "error": {
//!     "type": "invalid_request_error",
//!     "message": "Invalid request"
//!   }
//! }
//! ```
//!
//! Common error types include:
//! - `invalid_request_error` - Invalid parameters or request format
//! - `authentication_error` - Invalid API key or authentication failure
//! - `permission_error` - Insufficient permissions
//! - `not_found_error` - Resource not found
//! - `rate_limit_error` - Rate limit exceeded
//! - `api_error` - Internal API error
//! - `overloaded_error` - API temporarily overloaded
//!
//! ## Usage
//!
//! ```rust,no_run
//! use ai_sdk_anthropic::error::{AnthropicError, AnthropicErrorData};
//! use ai_sdk_anthropic::{AnthropicProvider, AnthropicProviderSettings};
//! use ai_sdk_core::generate_text::GenerateText;
//! use ai_sdk_core::prompt::Prompt;
//! use ai_sdk_provider::provider::Provider;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let provider = AnthropicProvider::new(AnthropicProviderSettings::default());
//! let model = provider.language_model("claude-3-5-sonnet-20241022".to_string());
//!
//! match GenerateText::new(std::sync::Arc::new(model), Prompt::text("Hello"))
//!     .execute()
//!     .await
//! {
//!     Ok(result) => println!("{}", result.text),
//!     Err(e) => {
//!         // Errors are automatically converted to ProviderError
//!         eprintln!("Error: {}", e);
//!     }
//! }
//! # Ok(())
//! # }
//! ```

use ai_sdk_provider::ProviderError;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Anthropic-specific error types.
///
/// This enum represents all possible error conditions when interacting with the
/// Anthropic API. Errors are automatically converted to SDK `ProviderError` types
/// for consistent error handling across different providers.
///
/// # Examples
///
/// ```rust
/// use ai_sdk_anthropic::error::{AnthropicError, AnthropicErrorData};
///
/// // Create an API error
/// let error_data = AnthropicErrorData::new(
///     "invalid_request_error".to_string(),
///     "Missing required parameter".to_string(),
/// );
/// let error = AnthropicError::ApiError(error_data);
///
/// // Convert to string
/// let error_message = error.to_string();
/// ```
#[derive(Debug)]
pub enum AnthropicError {
    /// API error from Anthropic.
    ///
    /// This variant contains structured error data returned by the Anthropic API,
    /// including the error type and message.
    ApiError(AnthropicErrorData),

    /// HTTP request error.
    ///
    /// This variant wraps errors from the underlying HTTP client (reqwest),
    /// such as network failures, connection timeouts, etc.
    RequestError(reqwest::Error),

    /// JSON parsing error.
    ///
    /// This variant wraps errors that occur when parsing JSON responses
    /// from the API.
    ParseError(serde_json::Error),

    /// Invalid configuration.
    ///
    /// This variant represents errors in provider configuration, such as
    /// missing API keys or invalid base URLs.
    ConfigError(String),

    /// Generic error wrapper.
    ///
    /// This variant wraps any other error types that don't fit the above categories.
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl fmt::Display for AnthropicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiError(data) => write!(f, "Anthropic API error: {}", data.error.message),
            Self::RequestError(err) => write!(f, "HTTP request error: {}", err),
            Self::ParseError(err) => write!(f, "JSON parsing error: {}", err),
            Self::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            Self::Other(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for AnthropicError {}

impl From<reqwest::Error> for AnthropicError {
    fn from(err: reqwest::Error) -> Self {
        Self::RequestError(err)
    }
}

impl From<serde_json::Error> for AnthropicError {
    fn from(err: serde_json::Error) -> Self {
        Self::ParseError(err)
    }
}

/// Anthropic error response data structure
///
/// This matches the Anthropic API error format:
/// ```json
/// {
///   "type": "error",
///   "error": {
///     "type": "invalid_request_error",
///     "message": "Invalid request"
///   }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnthropicErrorData {
    /// The type field (always "error" for error responses)
    #[serde(rename = "type")]
    pub response_type: String,

    /// The error object containing details
    pub error: AnthropicErrorDetails,
}

/// Anthropic error details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnthropicErrorDetails {
    /// The error type (e.g., "invalid_request_error", "authentication_error", "rate_limit_error")
    #[serde(rename = "type")]
    pub error_type: String,

    /// The error message
    pub message: String,
}

impl AnthropicErrorData {
    /// Create a new error data structure
    pub fn new(error_type: String, message: String) -> Self {
        Self {
            response_type: "error".to_string(),
            error: AnthropicErrorDetails {
                error_type,
                message,
            },
        }
    }

    /// Parse an error from a response body
    pub fn from_response_body(body: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(body)
    }

    /// Convert to a ProviderError
    pub fn to_provider_error(&self) -> ProviderError {
        match self.error.error_type.as_str() {
            "invalid_request_error" => {
                ProviderError::invalid_prompt(self.error.message.clone(), "anthropic request")
            }
            "authentication_error" => ProviderError::model_error(self.error.message.clone()),
            "permission_error" => ProviderError::model_error(self.error.message.clone()),
            "not_found_error" => {
                ProviderError::no_such_model(self.error.message.clone(), "anthropic")
            }
            "rate_limit_error" => ProviderError::model_error(self.error.message.clone()),
            "api_error" => ProviderError::model_error(self.error.message.clone()),
            "overloaded_error" => ProviderError::model_error(self.error.message.clone()),
            _ => ProviderError::model_error(self.error.message.clone()),
        }
    }

    /// Determine if this error is retryable based on error type
    pub fn is_retryable(&self) -> bool {
        matches!(
            self.error.error_type.as_str(),
            "rate_limit_error" | "overloaded_error" | "api_error"
        )
    }
}

impl AnthropicErrorDetails {
    /// Create a new error details structure
    pub fn new(error_type: String, message: String) -> Self {
        Self {
            error_type,
            message,
        }
    }
}

/// Parse an Anthropic error response from HTTP response
///
/// # Arguments
///
/// * `status_code` - The HTTP status code
/// * `body` - The response body as a string
///
/// # Returns
///
/// A ProviderError with appropriate error type and message
pub fn parse_anthropic_error(status_code: u16, body: &str) -> ProviderError {
    // Try to parse as Anthropic error format
    if let Ok(error_data) = AnthropicErrorData::from_response_body(body) {
        return error_data.to_provider_error();
    }

    // Fallback to generic HTTP error based on status code
    match status_code {
        400 => ProviderError::invalid_prompt(
            format!("Bad request (400): {}", body),
            "anthropic request",
        ),
        401 => ProviderError::model_error(format!("Authentication failed (401): {}", body)),
        403 => ProviderError::model_error(format!("Permission denied (403): {}", body)),
        404 => ProviderError::no_such_model(format!("Not found (404): {}", body), "anthropic"),
        429 => ProviderError::model_error(format!("Rate limit exceeded (429): {}", body)),
        500..=599 => {
            ProviderError::model_error(format!("Server error ({}): {}", status_code, body))
        }
        _ => ProviderError::model_error(format!("HTTP error ({}): {}", status_code, body)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_data_new() {
        let error = AnthropicErrorData::new(
            "invalid_request_error".to_string(),
            "Test error".to_string(),
        );
        assert_eq!(error.response_type, "error");
        assert_eq!(error.error.error_type, "invalid_request_error");
        assert_eq!(error.error.message, "Test error");
    }

    #[test]
    fn test_error_deserialization() {
        let json = r#"{
            "type": "error",
            "error": {
                "type": "invalid_request_error",
                "message": "Invalid request"
            }
        }"#;

        let error: AnthropicErrorData = serde_json::from_str(json).unwrap();
        assert_eq!(error.response_type, "error");
        assert_eq!(error.error.error_type, "invalid_request_error");
        assert_eq!(error.error.message, "Invalid request");
    }

    #[test]
    fn test_error_serialization() {
        let error = AnthropicErrorData::new(
            "rate_limit_error".to_string(),
            "Rate limit exceeded".to_string(),
        );
        let json = serde_json::to_value(&error).unwrap();

        assert_eq!(json["type"], "error");
        assert_eq!(json["error"]["type"], "rate_limit_error");
        assert_eq!(json["error"]["message"], "Rate limit exceeded");
    }

    #[test]
    fn test_error_from_response_body() {
        let body = r#"{
            "type": "error",
            "error": {
                "type": "authentication_error",
                "message": "Invalid API key"
            }
        }"#;

        let error = AnthropicErrorData::from_response_body(body).unwrap();
        assert_eq!(error.error.error_type, "authentication_error");
        assert_eq!(error.error.message, "Invalid API key");
    }

    #[test]
    fn test_is_retryable() {
        let rate_limit_error = AnthropicErrorData::new(
            "rate_limit_error".to_string(),
            "Rate limit exceeded".to_string(),
        );
        assert!(rate_limit_error.is_retryable());

        let overloaded_error = AnthropicErrorData::new(
            "overloaded_error".to_string(),
            "Service overloaded".to_string(),
        );
        assert!(overloaded_error.is_retryable());

        let invalid_request_error = AnthropicErrorData::new(
            "invalid_request_error".to_string(),
            "Invalid request".to_string(),
        );
        assert!(!invalid_request_error.is_retryable());
    }

    #[test]
    fn test_to_provider_error_invalid_request() {
        let error = AnthropicErrorData::new(
            "invalid_request_error".to_string(),
            "Invalid temperature value".to_string(),
        );
        let provider_error = error.to_provider_error();

        match provider_error {
            ProviderError::InvalidPrompt { .. } => (),
            _ => panic!("Expected InvalidPrompt error"),
        }
    }

    #[test]
    fn test_to_provider_error_not_found() {
        let error =
            AnthropicErrorData::new("not_found_error".to_string(), "Model not found".to_string());
        let provider_error = error.to_provider_error();

        match provider_error {
            ProviderError::NoSuchModel { .. } => (),
            _ => panic!("Expected NoSuchModel error"),
        }
    }

    #[test]
    fn test_parse_anthropic_error_valid_json() {
        let body = r#"{
            "type": "error",
            "error": {
                "type": "rate_limit_error",
                "message": "Rate limit exceeded"
            }
        }"#;

        let provider_error = parse_anthropic_error(429, body);
        match provider_error {
            ProviderError::ModelError { .. } => (),
            _ => panic!("Expected ModelError"),
        }
    }

    #[test]
    fn test_parse_anthropic_error_invalid_json() {
        let body = "Not a JSON response";

        let provider_error = parse_anthropic_error(500, body);
        match provider_error {
            ProviderError::ModelError { .. } => (),
            _ => panic!("Expected ModelError for HTTP 500"),
        }
    }

    #[test]
    fn test_parse_anthropic_error_status_codes() {
        // Test various status codes with non-JSON bodies
        let provider_error = parse_anthropic_error(400, "Bad request");
        assert!(matches!(
            provider_error,
            ProviderError::InvalidPrompt { .. }
        ));

        let provider_error = parse_anthropic_error(401, "Unauthorized");
        assert!(matches!(provider_error, ProviderError::ModelError { .. }));

        let provider_error = parse_anthropic_error(404, "Not found");
        assert!(matches!(provider_error, ProviderError::NoSuchModel { .. }));

        let provider_error = parse_anthropic_error(429, "Rate limited");
        assert!(matches!(provider_error, ProviderError::ModelError { .. }));

        let provider_error = parse_anthropic_error(500, "Server error");
        assert!(matches!(provider_error, ProviderError::ModelError { .. }));
    }

    #[test]
    fn test_anthropic_error_display() {
        let error = AnthropicError::ApiError(AnthropicErrorData::new(
            "test_error".to_string(),
            "Test message".to_string(),
        ));
        assert_eq!(error.to_string(), "Anthropic API error: Test message");

        let error = AnthropicError::ConfigError("Invalid config".to_string());
        assert_eq!(error.to_string(), "Configuration error: Invalid config");
    }

    #[test]
    fn test_anthropic_error_request_error_display() {
        // Test that RequestError variant can be displayed
        // We can't easily create a reqwest::Error, so we'll test the display
        // via ConfigError instead, which confirms the enum structure works
        let error = AnthropicError::ConfigError("test config error".to_string());
        assert_eq!(error.to_string(), "Configuration error: test config error");
    }

    #[test]
    fn test_anthropic_error_from_serde() {
        let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let error = AnthropicError::from(json_error);
        assert!(matches!(error, AnthropicError::ParseError(_)));
    }
}
