use llm_kit_provider::error::ProviderError;
use serde::Deserialize;

/// xAI API error response structure
#[derive(Debug, Deserialize)]
pub struct XaiErrorData {
    pub error: XaiErrorDetails,
}

/// xAI API error details
#[derive(Debug, Deserialize)]
pub struct XaiErrorDetails {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub param: Option<serde_json::Value>,
    pub code: Option<serde_json::Value>,
}

impl XaiErrorData {
    /// Convert xAI error to provider error
    pub fn to_provider_error(&self, status_code: u16, url: &str) -> ProviderError {
        let message = &self.error.message;
        let request_body = "{}"; // We don't have the original request body here

        match status_code {
            400 => ProviderError::api_call_error(
                format!("Bad request: {}", message),
                url,
                request_body,
            ),
            401 => ProviderError::api_call_error(
                format!("Unauthorized: {}", message),
                url,
                request_body,
            ),
            404 => {
                ProviderError::api_call_error(format!("Not found: {}", message), url, request_body)
            }
            429 => ProviderError::api_call_error(
                format!("Rate limit exceeded: {}", message),
                url,
                request_body,
            ),
            500..=599 => ProviderError::api_call_error(
                format!("Server error: {}", message),
                url,
                request_body,
            ),
            _ => {
                ProviderError::api_call_error(format!("API error: {}", message), url, request_body)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_deserialization() {
        let json = r#"{
            "error": {
                "message": "Invalid API key",
                "type": "invalid_request_error",
                "param": null,
                "code": "invalid_api_key"
            }
        }"#;

        let error: XaiErrorData = serde_json::from_str(json).unwrap();
        assert_eq!(error.error.message, "Invalid API key");
        assert_eq!(
            error.error.error_type.as_deref(),
            Some("invalid_request_error")
        );
    }

    #[test]
    fn test_error_conversion() {
        let error_data = XaiErrorData {
            error: XaiErrorDetails {
                message: "Rate limit exceeded".to_string(),
                error_type: Some("rate_limit_error".to_string()),
                param: None,
                code: Some(serde_json::Value::String("rate_limit".to_string())),
            },
        };

        let provider_error =
            error_data.to_provider_error(429, "https://api.x.ai/v1/chat/completions");
        // Just verify it doesn't panic - the error type is ApiCallError
        assert!(provider_error.to_string().contains("Rate limit exceeded"));
    }
}
