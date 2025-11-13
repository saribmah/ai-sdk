use serde::Deserialize;

/// Error data from the Hugging Face API.
#[derive(Debug, Deserialize, Clone)]
pub struct HuggingFaceErrorData {
    /// Error details.
    pub error: HuggingFaceErrorDetail,
}

/// Details of an error from the Hugging Face API.
#[derive(Debug, Deserialize, Clone)]
pub struct HuggingFaceErrorDetail {
    /// Error message.
    pub message: String,

    /// Error type (optional).
    #[serde(rename = "type")]
    pub error_type: Option<String>,

    /// Error code (optional).
    pub code: Option<String>,
}

impl HuggingFaceErrorData {
    /// Returns the error message.
    pub fn message(&self) -> &str {
        &self.error.message
    }

    /// Returns the error type if available.
    pub fn error_type(&self) -> Option<&str> {
        self.error.error_type.as_deref()
    }

    /// Returns the error code if available.
    pub fn code(&self) -> Option<&str> {
        self.error.code.as_deref()
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
                "type": "authentication_error",
                "code": "invalid_key"
            }
        }"#;

        let error: HuggingFaceErrorData = serde_json::from_str(json).unwrap();
        assert_eq!(error.message(), "Invalid API key");
        assert_eq!(error.error_type(), Some("authentication_error"));
        assert_eq!(error.code(), Some("invalid_key"));
    }

    #[test]
    fn test_error_deserialization_minimal() {
        let json = r#"{
            "error": {
                "message": "Something went wrong"
            }
        }"#;

        let error: HuggingFaceErrorData = serde_json::from_str(json).unwrap();
        assert_eq!(error.message(), "Something went wrong");
        assert_eq!(error.error_type(), None);
        assert_eq!(error.code(), None);
    }
}
