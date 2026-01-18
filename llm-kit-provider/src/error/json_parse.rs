use super::ProviderError;

/// Builder for constructing JSONParse error with a fluent interface
///
/// # Examples
///
/// ```
/// use llm_kit_provider::error::{JSONParseErrorBuilder, ProviderError};
/// use std::io;
///
/// let parse_error = io::Error::new(io::ErrorKind::InvalidData, "unexpected character");
///
/// let error = JSONParseErrorBuilder::new(r#"{"invalid": json}"#)
///     .cause(parse_error)
///     .build();
///
/// match error {
///     ProviderError::JSONParse { text, .. } => {
///         assert_eq!(text, r#"{"invalid": json}"#);
///     }
///     _ => panic!("Expected JSONParse error"),
/// }
/// ```
#[derive(Debug)]
pub struct JSONParseErrorBuilder {
    text: String,
    cause: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl JSONParseErrorBuilder {
    /// Create a new JSONParseErrorBuilder with required fields
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            cause: None,
        }
    }

    /// Set the cause error
    pub fn cause(mut self, cause: impl std::error::Error + Send + Sync + 'static) -> Self {
        self.cause = Some(Box::new(cause));
        self
    }

    /// Build the ProviderError
    pub fn build(self) -> ProviderError {
        ProviderError::JSONParse {
            text: self.text,
            cause: self.cause,
        }
    }
}

impl ProviderError {
    /// Create a new JSONParse error
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_provider::error::ProviderError;
    /// use std::io;
    ///
    /// let parse_error = io::Error::new(io::ErrorKind::InvalidData, "expected value at line 1");
    /// let error = ProviderError::json_parse_error(
    ///     r#"{"incomplete": }"#,
    ///     parse_error,
    /// );
    ///
    /// match error {
    ///     ProviderError::JSONParse { text, .. } => {
    ///         assert_eq!(text, r#"{"incomplete": }"#);
    ///     }
    ///     _ => panic!("Expected JSONParse error"),
    /// }
    /// ```
    pub fn json_parse_error(
        text: impl Into<String>,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::JSONParse {
            text: text.into(),
            cause: Some(Box::new(cause)),
        }
    }

    /// Create a new JSONParse error without a cause
    ///
    /// This is useful when you need to create a JSON parse error but don't have
    /// an underlying error to attach.
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_provider::error::ProviderError;
    ///
    /// let error = ProviderError::json_parse_error_without_cause(
    ///     r#"not valid json"#,
    /// );
    ///
    /// match error {
    ///     ProviderError::JSONParse { text, cause } => {
    ///         assert_eq!(text, "not valid json");
    ///         assert!(cause.is_none());
    ///     }
    ///     _ => panic!("Expected JSONParse error"),
    /// }
    /// ```
    pub fn json_parse_error_without_cause(text: impl Into<String>) -> Self {
        Self::JSONParse {
            text: text.into(),
            cause: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_parse_error_with_cause() {
        use std::io;

        let cause = io::Error::new(io::ErrorKind::InvalidData, "expected `:` at line 1");
        let error = ProviderError::json_parse_error(r#"{"key" "value"}"#, cause);

        match error {
            ProviderError::JSONParse { text, cause } => {
                assert_eq!(text, r#"{"key" "value"}"#);
                assert!(cause.is_some());
            }
            _ => panic!("Expected JSONParse error"),
        }
    }

    #[test]
    fn test_json_parse_error_without_cause() {
        let error = ProviderError::json_parse_error_without_cause("invalid json");

        match error {
            ProviderError::JSONParse { text, cause } => {
                assert_eq!(text, "invalid json");
                assert!(cause.is_none());
            }
            _ => panic!("Expected JSONParse error"),
        }
    }

    #[test]
    fn test_json_parse_error_builder_with_cause() {
        use std::io;

        let cause = io::Error::new(io::ErrorKind::InvalidData, "trailing comma");
        let error = JSONParseErrorBuilder::new(r#"{"key": "value",}"#)
            .cause(cause)
            .build();

        match error {
            ProviderError::JSONParse { text, cause } => {
                assert_eq!(text, r#"{"key": "value",}"#);
                assert!(cause.is_some());
            }
            _ => panic!("Expected JSONParse error"),
        }
    }

    #[test]
    fn test_json_parse_error_builder_without_cause() {
        let error = JSONParseErrorBuilder::new("plain text").build();

        match error {
            ProviderError::JSONParse { text, cause } => {
                assert_eq!(text, "plain text");
                assert!(cause.is_none());
            }
            _ => panic!("Expected JSONParse error"),
        }
    }

    #[test]
    fn test_json_parse_error_not_retryable() {
        let error = ProviderError::json_parse_error_without_cause("test");
        assert!(!error.is_retryable());
        assert!(error.status_code().is_none());
        assert!(error.url().is_none());
    }

    #[test]
    fn test_error_display() {
        use std::io;

        let cause = io::Error::new(io::ErrorKind::InvalidData, "parse error");
        let error = ProviderError::json_parse_error(r#"bad json"#, cause);
        let display = format!("{}", error);
        assert!(display.contains("JSON parsing failed"));
        assert!(display.contains("bad json"));
    }

    #[test]
    fn test_builder_method_chaining() {
        use std::io;

        let error1 = JSONParseErrorBuilder::new("text1").build();
        let error2 = JSONParseErrorBuilder::new("text2")
            .cause(io::Error::other("Error"))
            .build();

        // Both should be valid JSONParse errors
        assert!(matches!(error1, ProviderError::JSONParse { .. }));
        assert!(matches!(error2, ProviderError::JSONParse { .. }));
    }

    #[test]
    fn test_different_invalid_json_formats() {
        let test_cases = vec![
            (r#"{"key": }"#, "missing value"),
            (r#"{"key" "value"}"#, "missing colon"),
            (r#"{"key": "value",}"#, "trailing comma"),
            (r#"{key: "value"}"#, "unquoted key"),
            (r#"{'key': 'value'}"#, "single quotes"),
            ("not json at all", "plain text"),
        ];

        for (text, description) in test_cases {
            let error = ProviderError::json_parse_error_without_cause(text);
            match error {
                ProviderError::JSONParse { text: t, .. } => {
                    assert_eq!(t, text, "Failed for case: {}", description);
                }
                _ => panic!("Expected JSONParse error for: {}", description),
            }
        }
    }

    #[test]
    fn test_text_preservation() {
        let original_text = r#"{
            "model": "gpt-4",
            "invalid": ,
            "choices": []
        }"#;

        let error = ProviderError::json_parse_error_without_cause(original_text);

        match error {
            ProviderError::JSONParse { text, .. } => {
                assert_eq!(text, original_text);
            }
            _ => panic!("Expected JSONParse error"),
        }
    }

    #[test]
    fn test_error_source_chain() {
        use std::io;

        let inner_error = io::Error::new(io::ErrorKind::InvalidData, "EOF while parsing object");
        let error = ProviderError::json_parse_error(r#"{"incomplete""#, inner_error);

        match error {
            ProviderError::JSONParse { cause, .. } => {
                assert!(cause.is_some());
                let cause_msg = cause.unwrap().to_string();
                assert!(cause_msg.contains("EOF while parsing"));
            }
            _ => panic!("Expected JSONParse error"),
        }
    }

    #[test]
    fn test_empty_text() {
        let error = ProviderError::json_parse_error_without_cause("");
        match error {
            ProviderError::JSONParse { text, .. } => {
                assert_eq!(text, "");
            }
            _ => panic!("Expected JSONParse error"),
        }
    }

    #[test]
    fn test_very_long_text() {
        let long_text = "x".repeat(10000);
        let error = ProviderError::json_parse_error_without_cause(&long_text);
        match error {
            ProviderError::JSONParse { text, .. } => {
                assert_eq!(text, long_text);
            }
            _ => panic!("Expected JSONParse error"),
        }
    }

    #[test]
    fn test_with_actual_serde_error() {
        use serde_json;

        let invalid_json = r#"{"key": undefined}"#;

        // Try to parse and capture the error
        match serde_json::from_str::<serde_json::Value>(invalid_json) {
            Err(parse_err) => {
                let error = ProviderError::json_parse_error(invalid_json, parse_err);
                match error {
                    ProviderError::JSONParse { text, cause } => {
                        assert_eq!(text, invalid_json);
                        assert!(cause.is_some());
                    }
                    _ => panic!("Expected JSONParse error"),
                }
            }
            Ok(_) => panic!("Expected JSON parsing to fail"),
        }
    }
}
