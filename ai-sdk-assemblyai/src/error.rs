use ai_sdk_provider::error::ProviderError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// AssemblyAI-specific error types.
#[derive(Error, Debug)]
pub enum AssemblyAIError {
    /// API error from AssemblyAI
    #[error("AssemblyAI API error: {message} (code: {code})")]
    ApiError { message: String, code: i32 },

    /// Network error
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    /// JSON parsing error
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Transcription failed
    #[error("Transcription failed: {0}")]
    TranscriptionFailed(String),

    /// Transcription was aborted
    #[error("Transcription was aborted")]
    Aborted,

    /// Invalid response from API
    #[error("Invalid API response: {0}")]
    InvalidResponse(String),

    /// Upload failed
    #[error("Upload failed: {0}")]
    UploadFailed(String),
}

/// AssemblyAI API error response structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyAIErrorResponse {
    pub error: AssemblyAIErrorDetail,
}

/// AssemblyAI API error detail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyAIErrorDetail {
    pub message: String,
    pub code: i32,
}

impl From<AssemblyAIError> for ProviderError {
    fn from(error: AssemblyAIError) -> Self {
        match error {
            AssemblyAIError::ApiError { message, code } => ProviderError::api_call_error(
                format!("AssemblyAI API error (code {}): {}", code, message),
                "https://api.assemblyai.com",
                "",
            ),
            AssemblyAIError::NetworkError(e) => ProviderError::api_call_error(
                format!("Network error: {}", e),
                "https://api.assemblyai.com",
                "",
            ),
            AssemblyAIError::JsonError(e) => ProviderError::api_call_error(
                format!("JSON parsing error: {}", e),
                "https://api.assemblyai.com",
                "",
            ),
            AssemblyAIError::TranscriptionFailed(msg) => ProviderError::api_call_error(
                format!("Transcription failed: {}", msg),
                "https://api.assemblyai.com",
                "",
            ),
            AssemblyAIError::Aborted => {
                ProviderError::api_call_error("Request aborted", "https://api.assemblyai.com", "")
            }
            AssemblyAIError::InvalidResponse(msg) => ProviderError::api_call_error(
                format!("Invalid response: {}", msg),
                "https://api.assemblyai.com",
                "",
            ),
            AssemblyAIError::UploadFailed(msg) => ProviderError::api_call_error(
                format!("Upload failed: {}", msg),
                "https://api.assemblyai.com",
                "",
            ),
        }
    }
}

/// Parse an AssemblyAI error response from JSON.
pub fn parse_error_response(body: &str) -> Result<AssemblyAIError, serde_json::Error> {
    let error_response: AssemblyAIErrorResponse = serde_json::from_str(body)?;
    Ok(AssemblyAIError::ApiError {
        message: error_response.error.message,
        code: error_response.error.code,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_response() {
        let json = r#"{"error": {"message": "Invalid API key", "code": 401}}"#;
        let error = parse_error_response(json).unwrap();

        match error {
            AssemblyAIError::ApiError { message, code } => {
                assert_eq!(message, "Invalid API key");
                assert_eq!(code, 401);
            }
            _ => panic!("Expected ApiError"),
        }
    }

    #[test]
    fn test_error_to_provider_error() {
        let error = AssemblyAIError::ApiError {
            message: "Test error".to_string(),
            code: 500,
        };

        let provider_error: ProviderError = error.into();
        assert!(provider_error.to_string().contains("Test error"));
    }
}
