use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

/// OpenAI-compatible error types
#[derive(Debug)]
pub enum OpenAICompatibleError {
    /// API error with status code and body
    ApiError { status: u16, body: Value },
    /// Too many embedding values for a single call
    TooManyEmbeddingValues {
        provider: String,
        model_id: String,
        max_embeddings_per_call: usize,
        values_count: usize,
    },
    /// Generic error wrapper
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl fmt::Display for OpenAICompatibleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApiError { status, body } => {
                write!(f, "API error (status {}): {}", status, body)
            }
            Self::TooManyEmbeddingValues {
                provider,
                model_id,
                max_embeddings_per_call,
                values_count,
            } => {
                write!(
                    f,
                    "Too many embedding values for call. Provider: {}, Model: {}, Max: {}, Provided: {}",
                    provider, model_id, max_embeddings_per_call, values_count
                )
            }
            Self::Other(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for OpenAICompatibleError {}

/// OpenAI-compatible error response data structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAICompatibleErrorData {
    /// The error object
    pub error: OpenAICompatibleErrorDetails,
}

/// OpenAI-compatible error details
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAICompatibleErrorDetails {
    /// The error message
    pub message: String,

    /// The error type (optional, handled loosely to support various providers)
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,

    /// The parameter that caused the error (optional, can be any type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<Value>,

    /// The error code (optional, can be string or number)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<ErrorCode>,
}

/// Error code that can be either a string or a number
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ErrorCode {
    /// String error code
    String(String),
    /// Numeric error code
    Number(i64),
}

/// Trait for handling provider-specific error structures
///
/// This trait defines how to parse errors from API responses and convert them
/// to human-readable messages. It also allows determining if an error is retryable.
pub trait ProviderErrorStructure {
    /// The error type this structure handles
    type Error: for<'de> Deserialize<'de>;

    /// Parse an error from a response body
    ///
    /// # Arguments
    ///
    /// * `body` - The response body as a string
    ///
    /// # Returns
    ///
    /// The parsed error or a serde JSON error
    fn parse_error(&self, body: &str) -> Result<Self::Error, serde_json::Error>;

    /// Convert an error to a human-readable message
    ///
    /// # Arguments
    ///
    /// * `error` - The parsed error
    ///
    /// # Returns
    ///
    /// A human-readable error message
    fn error_to_message(&self, error: &Self::Error) -> String;

    /// Determine if an error is retryable
    ///
    /// # Arguments
    ///
    /// * `status_code` - The HTTP status code
    /// * `error` - The parsed error (if available)
    ///
    /// # Returns
    ///
    /// `true` if the error should be retried, `false` otherwise
    fn is_retryable(&self, status_code: u16, error: Option<&Self::Error>) -> bool {
        // Default implementation: don't retry
        let _ = (status_code, error);
        false
    }
}

/// Default error structure implementation for OpenAI-compatible providers
pub struct DefaultOpenAICompatibleErrorStructure;

impl ProviderErrorStructure for DefaultOpenAICompatibleErrorStructure {
    type Error = OpenAICompatibleErrorData;

    fn parse_error(&self, body: &str) -> Result<Self::Error, serde_json::Error> {
        serde_json::from_str(body)
    }

    fn error_to_message(&self, error: &Self::Error) -> String {
        error.error.message.clone()
    }
}

impl OpenAICompatibleErrorData {
    /// Create a new error data structure
    pub fn new(message: String) -> Self {
        Self {
            error: OpenAICompatibleErrorDetails {
                message,
                error_type: None,
                param: None,
                code: None,
            },
        }
    }

    /// Create an error with all fields
    pub fn with_details(
        message: String,
        error_type: Option<String>,
        param: Option<Value>,
        code: Option<ErrorCode>,
    ) -> Self {
        Self {
            error: OpenAICompatibleErrorDetails {
                message,
                error_type,
                param,
                code,
            },
        }
    }
}

impl OpenAICompatibleErrorDetails {
    /// Create a new error
    pub fn new(message: String) -> Self {
        Self {
            message,
            error_type: None,
            param: None,
            code: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_error_data_new() {
        let error = OpenAICompatibleErrorData::new("Test error".to_string());
        assert_eq!(error.error.message, "Test error");
        assert_eq!(error.error.error_type, None);
        assert_eq!(error.error.param, None);
        assert_eq!(error.error.code, None);
    }

    #[test]
    fn test_error_serialization() {
        let error = OpenAICompatibleErrorData::new("Test error".to_string());
        let json = serde_json::to_value(&error).unwrap();

        assert_eq!(json["error"]["message"], "Test error");
        assert!(!json["error"].as_object().unwrap().contains_key("type"));
    }

    #[test]
    fn test_error_deserialization_minimal() {
        let json = r#"{
            "error": {
                "message": "Invalid request"
            }
        }"#;

        let error: OpenAICompatibleErrorData = serde_json::from_str(json).unwrap();
        assert_eq!(error.error.message, "Invalid request");
        assert_eq!(error.error.error_type, None);
    }

    #[test]
    fn test_error_deserialization_full() {
        let json = r#"{
            "error": {
                "message": "Invalid request",
                "type": "invalid_request_error",
                "param": "temperature",
                "code": "invalid_value"
            }
        }"#;

        let error: OpenAICompatibleErrorData = serde_json::from_str(json).unwrap();
        assert_eq!(error.error.message, "Invalid request");
        assert_eq!(
            error.error.error_type,
            Some("invalid_request_error".to_string())
        );
        assert_eq!(error.error.param, Some(json!("temperature")));
        match error.error.code {
            Some(ErrorCode::String(s)) => assert_eq!(s, "invalid_value"),
            _ => panic!("Expected string error code"),
        }
    }

    #[test]
    fn test_error_code_string() {
        let json = r#"{
            "error": {
                "message": "Error",
                "code": "test_code"
            }
        }"#;

        let error: OpenAICompatibleErrorData = serde_json::from_str(json).unwrap();
        match error.error.code {
            Some(ErrorCode::String(s)) => assert_eq!(s, "test_code"),
            _ => panic!("Expected string error code"),
        }
    }

    #[test]
    fn test_error_code_number() {
        let json = r#"{
            "error": {
                "message": "Error",
                "code": 429
            }
        }"#;

        let error: OpenAICompatibleErrorData = serde_json::from_str(json).unwrap();
        match error.error.code {
            Some(ErrorCode::Number(n)) => assert_eq!(n, 429),
            _ => panic!("Expected numeric error code"),
        }
    }

    #[test]
    fn test_default_error_structure() {
        let structure = DefaultOpenAICompatibleErrorStructure;

        let json = r#"{
            "error": {
                "message": "Test error message"
            }
        }"#;

        let error = structure.parse_error(json).unwrap();
        let message = structure.error_to_message(&error);

        assert_eq!(message, "Test error message");
    }

    #[test]
    fn test_is_retryable_default() {
        let structure = DefaultOpenAICompatibleErrorStructure;
        let error = OpenAICompatibleErrorData::new("Error".to_string());

        assert!(!structure.is_retryable(500, Some(&error)));
        assert!(!structure.is_retryable(429, None));
    }

    #[test]
    fn test_error_with_details() {
        let error = OpenAICompatibleErrorData::with_details(
            "Test error".to_string(),
            Some("test_type".to_string()),
            Some(json!({"key": "value"})),
            Some(ErrorCode::String("test_code".to_string())),
        );

        assert_eq!(error.error.message, "Test error");
        assert_eq!(error.error.error_type, Some("test_type".to_string()));
        assert_eq!(error.error.param, Some(json!({"key": "value"})));
    }

    #[test]
    fn test_error_param_any_type() {
        let json = r#"{
            "error": {
                "message": "Error",
                "param": ["array", "of", "values"]
            }
        }"#;

        let error: OpenAICompatibleErrorData = serde_json::from_str(json).unwrap();
        assert_eq!(error.error.param, Some(json!(["array", "of", "values"])));
    }
}
