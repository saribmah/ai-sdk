use super::ProviderError;

/// Builder for constructing InvalidResponseData error with a fluent interface
///
/// # Examples
///
/// ```
/// use ai_sdk_provider::error::{InvalidResponseDataErrorBuilder, ProviderError};
///
/// let error = InvalidResponseDataErrorBuilder::new(r#"{"unexpected": "format"}"#)
///     .message("Expected 'data' field but got 'unexpected'")
///     .build();
///
/// match error {
///     ProviderError::InvalidResponseData { data, message } => {
///         assert_eq!(data, r#"{"unexpected": "format"}"#);
///         assert_eq!(message, "Expected 'data' field but got 'unexpected'");
///     }
///     _ => panic!("Expected InvalidResponseData error"),
/// }
/// ```
#[derive(Debug, Default)]
pub struct InvalidResponseDataErrorBuilder {
    data: String,
    message: Option<String>,
}

impl InvalidResponseDataErrorBuilder {
    /// Create a new InvalidResponseDataErrorBuilder with required fields
    ///
    /// If no custom message is set, a default message will be generated
    /// from the data.
    pub fn new(data: impl Into<String>) -> Self {
        Self {
            data: data.into(),
            message: None,
        }
    }

    /// Set a custom error message
    ///
    /// If not set, defaults to "Invalid response data: {data}"
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Build the ProviderError
    pub fn build(self) -> ProviderError {
        let message = self
            .message
            .unwrap_or_else(|| format!("Invalid response data: {}", self.data));

        ProviderError::InvalidResponseData {
            data: self.data,
            message,
        }
    }
}

impl ProviderError {
    /// Create a new InvalidResponseData error with default message
    ///
    /// The default message will be "Invalid response data: {data}"
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_provider::error::ProviderError;
    ///
    /// let error = ProviderError::invalid_response_data(r#"{"error": "invalid"}"#);
    ///
    /// match error {
    ///     ProviderError::InvalidResponseData { data, message } => {
    ///         assert_eq!(data, r#"{"error": "invalid"}"#);
    ///         assert!(message.contains("Invalid response data"));
    ///     }
    ///     _ => panic!("Expected InvalidResponseData error"),
    /// }
    /// ```
    pub fn invalid_response_data(data: impl Into<String>) -> Self {
        let data_str = data.into();
        Self::InvalidResponseData {
            message: format!("Invalid response data: {}", data_str),
            data: data_str,
        }
    }

    /// Create a new InvalidResponseData error with a custom message
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_provider::error::ProviderError;
    ///
    /// let error = ProviderError::invalid_response_data_with_message(
    ///     r#"{"unexpected": "structure"}"#,
    ///     "Expected 'choices' array but got 'unexpected' field",
    /// );
    ///
    /// match error {
    ///     ProviderError::InvalidResponseData { data, message } => {
    ///         assert_eq!(data, r#"{"unexpected": "structure"}"#);
    ///         assert_eq!(message, "Expected 'choices' array but got 'unexpected' field");
    ///     }
    ///     _ => panic!("Expected InvalidResponseData error"),
    /// }
    /// ```
    pub fn invalid_response_data_with_message(
        data: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::InvalidResponseData {
            data: data.into(),
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_response_data_default() {
        let error = ProviderError::invalid_response_data(r#"{"invalid": true}"#);

        match error {
            ProviderError::InvalidResponseData { data, message } => {
                assert_eq!(data, r#"{"invalid": true}"#);
                assert!(message.contains("Invalid response data"));
                assert!(message.contains(r#"{"invalid": true}"#));
            }
            _ => panic!("Expected InvalidResponseData error"),
        }
    }

    #[test]
    fn test_invalid_response_data_with_message() {
        let error = ProviderError::invalid_response_data_with_message(
            r#"{"error": "missing_field"}"#,
            "Response is missing required 'choices' field",
        );

        match error {
            ProviderError::InvalidResponseData { data, message } => {
                assert_eq!(data, r#"{"error": "missing_field"}"#);
                assert_eq!(message, "Response is missing required 'choices' field");
            }
            _ => panic!("Expected InvalidResponseData error"),
        }
    }

    #[test]
    fn test_invalid_response_data_builder_default() {
        let error = InvalidResponseDataErrorBuilder::new(r#"{"test": "data"}"#).build();

        match error {
            ProviderError::InvalidResponseData { data, message } => {
                assert_eq!(data, r#"{"test": "data"}"#);
                assert!(message.contains("Invalid response data"));
            }
            _ => panic!("Expected InvalidResponseData error"),
        }
    }

    #[test]
    fn test_invalid_response_data_builder_with_message() {
        let error = InvalidResponseDataErrorBuilder::new(r#"null"#)
            .message("Expected object but got null")
            .build();

        match error {
            ProviderError::InvalidResponseData { data, message } => {
                assert_eq!(data, "null");
                assert_eq!(message, "Expected object but got null");
            }
            _ => panic!("Expected InvalidResponseData error"),
        }
    }

    #[test]
    fn test_invalid_response_data_not_retryable() {
        let error = ProviderError::invalid_response_data("test");
        assert!(!error.is_retryable());
        assert!(error.status_code().is_none());
        assert!(error.url().is_none());
    }

    #[test]
    fn test_error_display() {
        let error = ProviderError::invalid_response_data(r#"{"bad": "data"}"#);
        let display = format!("{}", error);
        assert!(display.contains("Invalid response data"));
    }

    #[test]
    fn test_builder_method_chaining() {
        let error1 = InvalidResponseDataErrorBuilder::new("data1").build();
        let error2 = InvalidResponseDataErrorBuilder::new("data2")
            .message("Custom message")
            .build();

        // Both should be valid InvalidResponseData errors
        assert!(matches!(error1, ProviderError::InvalidResponseData { .. }));
        assert!(matches!(error2, ProviderError::InvalidResponseData { .. }));
    }

    #[test]
    fn test_different_data_formats() {
        let test_cases = vec![
            (r#"null"#, "null value"),
            (r#"[]"#, "empty array"),
            (r#"{}"#, "empty object"),
            (r#"{"incomplete"}"#, "malformed JSON"),
            ("plain text", "non-JSON response"),
        ];

        for (data, description) in test_cases {
            let error = ProviderError::invalid_response_data(data);
            match error {
                ProviderError::InvalidResponseData { data: d, .. } => {
                    assert_eq!(d, data, "Failed for case: {}", description);
                }
                _ => panic!("Expected InvalidResponseData error for: {}", description),
            }
        }
    }

    #[test]
    fn test_data_preservation() {
        let original_data = r#"{
            "model": "gpt-4",
            "choices": [
                {"message": {"role": "assistant", "content": "Hello"}}
            ]
        }"#;

        let error = ProviderError::invalid_response_data(original_data);

        match error {
            ProviderError::InvalidResponseData { data, .. } => {
                assert_eq!(data, original_data);
            }
            _ => panic!("Expected InvalidResponseData error"),
        }
    }

    #[test]
    fn test_message_variations() {
        let messages = vec![
            "Missing 'id' field in response",
            "Expected array but got object",
            "Response schema validation failed",
            "Unknown response format",
        ];

        for msg in messages {
            let error = ProviderError::invalid_response_data_with_message("data", msg);
            if let ProviderError::InvalidResponseData { message, .. } = error {
                assert_eq!(message, msg);
            } else {
                panic!("Expected InvalidResponseData error");
            }
        }
    }

    #[test]
    fn test_empty_data() {
        let error = ProviderError::invalid_response_data("");
        match error {
            ProviderError::InvalidResponseData { data, .. } => {
                assert_eq!(data, "");
            }
            _ => panic!("Expected InvalidResponseData error"),
        }
    }
}
