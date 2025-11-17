use serde::{Deserialize, Serialize};

/// Error data returned by the Cerebras API.
///
/// This structure matches the error format returned by the Cerebras API
/// when requests fail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CerebrasErrorData {
    /// Error message describing what went wrong
    pub message: String,

    /// Type of error (e.g., "invalid_request_error", "authentication_error")
    #[serde(rename = "type")]
    pub error_type: String,

    /// Parameter that caused the error (if applicable)
    pub param: String,

    /// Error code for programmatic error handling
    pub code: String,
}

impl std::fmt::Display for CerebrasErrorData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cerebras API error ({}): {} [code: {}, param: {}]",
            self.error_type, self.message, self.code, self.param
        )
    }
}

impl std::error::Error for CerebrasErrorData {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_data_display() {
        let error = CerebrasErrorData {
            message: "Invalid API key".to_string(),
            error_type: "authentication_error".to_string(),
            param: "api_key".to_string(),
            code: "invalid_api_key".to_string(),
        };

        let display = format!("{}", error);
        assert!(display.contains("Cerebras API error"));
        assert!(display.contains("Invalid API key"));
        assert!(display.contains("authentication_error"));
    }

    #[test]
    fn test_error_data_serialize() {
        let error = CerebrasErrorData {
            message: "Test error".to_string(),
            error_type: "test_error".to_string(),
            param: "test_param".to_string(),
            code: "test_code".to_string(),
        };

        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("Test error"));
        assert!(json.contains("test_error"));
    }

    #[test]
    fn test_error_data_deserialize() {
        let json = r#"{
            "message": "Test error",
            "type": "test_error",
            "param": "test_param",
            "code": "test_code"
        }"#;

        let error: CerebrasErrorData = serde_json::from_str(json).unwrap();
        assert_eq!(error.message, "Test error");
        assert_eq!(error.error_type, "test_error");
        assert_eq!(error.param, "test_param");
        assert_eq!(error.code, "test_code");
    }
}
