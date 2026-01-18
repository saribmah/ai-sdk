use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

/// ElevenLabs API error response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElevenLabsErrorData {
    pub error: ElevenLabsError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElevenLabsError {
    pub message: String,
    pub code: i32,
}

impl fmt::Display for ElevenLabsErrorData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ElevenLabs API error (code {}): {}",
            self.error.code, self.error.message
        )
    }
}

impl Error for ElevenLabsErrorData {}

/// Parse ElevenLabs error from response body.
pub fn parse_elevenlabs_error(body: &str) -> Option<String> {
    serde_json::from_str::<ElevenLabsErrorData>(body)
        .ok()
        .map(|error_data| error_data.error.message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_elevenlabs_error() {
        let body = r#"{"error":{"message":"Invalid API key","code":401}}"#;
        let error = parse_elevenlabs_error(body);
        assert_eq!(error, Some("Invalid API key".to_string()));
    }

    #[test]
    fn test_parse_elevenlabs_error_invalid() {
        let body = r#"{"invalid":"json"}"#;
        let error = parse_elevenlabs_error(body);
        assert_eq!(error, None);
    }
}
