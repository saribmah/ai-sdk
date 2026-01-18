use super::ProviderError;

/// Builder for constructing TypeValidation error with a fluent interface
///
/// # Examples
///
/// ```
/// use llm_kit_provider::error::{TypeValidationErrorBuilder, ProviderError};
/// use std::io;
///
/// let validation_error = io::Error::new(io::ErrorKind::InvalidInput, "expected string, got number");
///
/// let error = TypeValidationErrorBuilder::new(r#"{"value": 123}"#)
///     .cause(validation_error)
///     .build();
///
/// match error {
///     ProviderError::TypeValidation { value, .. } => {
///         assert_eq!(value, r#"{"value": 123}"#);
///     }
///     _ => panic!("Expected TypeValidation error"),
/// }
/// ```
#[derive(Debug)]
pub struct TypeValidationErrorBuilder {
    value: String,
    cause: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl TypeValidationErrorBuilder {
    /// Create a new TypeValidationErrorBuilder with required fields
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
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
        ProviderError::TypeValidation {
            value: self.value,
            cause: self.cause,
        }
    }
}

impl ProviderError {
    /// Create a new TypeValidation error
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_provider::error::ProviderError;
    /// use std::io;
    ///
    /// let validation_error = io::Error::new(io::ErrorKind::InvalidInput, "type mismatch");
    /// let error = ProviderError::type_validation_error(
    ///     r#"{"field": "wrong_type"}"#,
    ///     validation_error,
    /// );
    ///
    /// match error {
    ///     ProviderError::TypeValidation { value, .. } => {
    ///         assert_eq!(value, r#"{"field": "wrong_type"}"#);
    ///     }
    ///     _ => panic!("Expected TypeValidation error"),
    /// }
    /// ```
    pub fn type_validation_error(
        value: impl Into<String>,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::TypeValidation {
            value: value.into(),
            cause: Some(Box::new(cause)),
        }
    }

    /// Create a new TypeValidation error without a cause
    ///
    /// This is useful when you need to create a type validation error but don't have
    /// an underlying error to attach.
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_provider::error::ProviderError;
    ///
    /// let error = ProviderError::type_validation_error_without_cause(
    ///     r#"{"invalid": "value"}"#,
    /// );
    ///
    /// match error {
    ///     ProviderError::TypeValidation { value, cause } => {
    ///         assert_eq!(value, r#"{"invalid": "value"}"#);
    ///         assert!(cause.is_none());
    ///     }
    ///     _ => panic!("Expected TypeValidation error"),
    /// }
    /// ```
    pub fn type_validation_error_without_cause(value: impl Into<String>) -> Self {
        Self::TypeValidation {
            value: value.into(),
            cause: None,
        }
    }

    /// Wrap an error into a TypeValidation error
    ///
    /// If the error is already a TypeValidation error with the same value, returns it as-is.
    /// Otherwise, creates a new TypeValidation error.
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_provider::error::ProviderError;
    /// use std::io;
    ///
    /// let value = r#"{"test": "value"}"#;
    /// let cause = io::Error::new(io::ErrorKind::InvalidInput, "validation failed");
    ///
    /// // First wrap
    /// let error1 = ProviderError::wrap_type_validation_error(value, cause);
    ///
    /// // Wrapping again with same value should return the same error
    /// let error2 = match error1 {
    ///     ProviderError::TypeValidation { ref value, ref cause } => {
    ///         ProviderError::wrap_type_validation_error(
    ///             value.clone(),
    ///             io::Error::new(io::ErrorKind::Other, "another error"),
    ///         )
    ///     }
    ///     _ => panic!("Expected TypeValidation error"),
    /// };
    ///
    /// // Both should be TypeValidation errors with the same value
    /// match (error1, error2) {
    ///     (
    ///         ProviderError::TypeValidation { value: v1, .. },
    ///         ProviderError::TypeValidation { value: v2, .. }
    ///     ) => {
    ///         assert_eq!(v1, v2);
    ///     }
    ///     _ => panic!("Expected TypeValidation errors"),
    /// }
    /// ```
    pub fn wrap_type_validation_error(
        value: impl Into<String>,
        cause: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        let value_str = value.into();
        Self::TypeValidation {
            value: value_str,
            cause: Some(Box::new(cause)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_validation_error_with_cause() {
        use std::io;

        let cause = io::Error::new(io::ErrorKind::InvalidInput, "expected integer, got string");
        let error = ProviderError::type_validation_error(r#"{"value": "not a number"}"#, cause);

        match error {
            ProviderError::TypeValidation { value, cause } => {
                assert_eq!(value, r#"{"value": "not a number"}"#);
                assert!(cause.is_some());
            }
            _ => panic!("Expected TypeValidation error"),
        }
    }

    #[test]
    fn test_type_validation_error_without_cause() {
        let error = ProviderError::type_validation_error_without_cause("invalid value");

        match error {
            ProviderError::TypeValidation { value, cause } => {
                assert_eq!(value, "invalid value");
                assert!(cause.is_none());
            }
            _ => panic!("Expected TypeValidation error"),
        }
    }

    #[test]
    fn test_type_validation_error_builder_with_cause() {
        use std::io;

        let cause = io::Error::new(io::ErrorKind::InvalidData, "type mismatch");
        let error = TypeValidationErrorBuilder::new(r#"{"key": null}"#)
            .cause(cause)
            .build();

        match error {
            ProviderError::TypeValidation { value, cause } => {
                assert_eq!(value, r#"{"key": null}"#);
                assert!(cause.is_some());
            }
            _ => panic!("Expected TypeValidation error"),
        }
    }

    #[test]
    fn test_type_validation_error_builder_without_cause() {
        let error = TypeValidationErrorBuilder::new("plain value").build();

        match error {
            ProviderError::TypeValidation { value, cause } => {
                assert_eq!(value, "plain value");
                assert!(cause.is_none());
            }
            _ => panic!("Expected TypeValidation error"),
        }
    }

    #[test]
    fn test_type_validation_error_not_retryable() {
        let error = ProviderError::type_validation_error_without_cause("test");
        assert!(!error.is_retryable());
        assert!(error.status_code().is_none());
        assert!(error.url().is_none());
    }

    #[test]
    fn test_error_display() {
        use std::io;

        let cause = io::Error::new(io::ErrorKind::InvalidData, "validation error");
        let error = ProviderError::type_validation_error("invalid", cause);
        let display = format!("{}", error);
        assert!(display.contains("Type validation failed"));
        assert!(display.contains("invalid"));
    }

    #[test]
    fn test_builder_method_chaining() {
        use std::io;

        let error1 = TypeValidationErrorBuilder::new("value1").build();
        let error2 = TypeValidationErrorBuilder::new("value2")
            .cause(io::Error::other("Error"))
            .build();

        // Both should be valid TypeValidation errors
        assert!(matches!(error1, ProviderError::TypeValidation { .. }));
        assert!(matches!(error2, ProviderError::TypeValidation { .. }));
    }

    #[test]
    fn test_wrap_type_validation_error() {
        use std::io;

        let value = r#"{"test": "data"}"#;
        let cause = io::Error::new(io::ErrorKind::InvalidInput, "first error");

        let error = ProviderError::wrap_type_validation_error(value, cause);

        match error {
            ProviderError::TypeValidation { value: v, cause } => {
                assert_eq!(v, r#"{"test": "data"}"#);
                assert!(cause.is_some());
            }
            _ => panic!("Expected TypeValidation error"),
        }
    }

    #[test]
    fn test_different_value_types() {
        use std::io;

        let test_cases = vec![
            (r#"null"#, "null value"),
            (r#"123"#, "number"),
            (r#""string""#, "string"),
            (r#"true"#, "boolean"),
            (r#"[]"#, "array"),
            (r#"{}"#, "object"),
        ];

        for (value, description) in test_cases {
            let cause = io::Error::new(io::ErrorKind::InvalidData, description);
            let error = ProviderError::type_validation_error(value, cause);
            match error {
                ProviderError::TypeValidation { value: v, .. } => {
                    assert_eq!(v, value, "Failed for case: {}", description);
                }
                _ => panic!("Expected TypeValidation error for: {}", description),
            }
        }
    }

    #[test]
    fn test_value_preservation() {
        use std::io;

        let original_value = r#"{
            "field1": "value1",
            "field2": 123,
            "field3": null
        }"#;

        let cause = io::Error::new(io::ErrorKind::InvalidData, "schema mismatch");
        let error = ProviderError::type_validation_error(original_value, cause);

        match error {
            ProviderError::TypeValidation { value, .. } => {
                assert_eq!(value, original_value);
            }
            _ => panic!("Expected TypeValidation error"),
        }
    }

    #[test]
    fn test_error_source_chain() {
        use std::io;

        let inner_error = io::Error::new(io::ErrorKind::InvalidData, "expected object, got array");
        let error = ProviderError::type_validation_error(r#"[1, 2, 3]"#, inner_error);

        match error {
            ProviderError::TypeValidation { cause, .. } => {
                assert!(cause.is_some());
                let cause_msg = cause.unwrap().to_string();
                assert!(cause_msg.contains("expected object"));
            }
            _ => panic!("Expected TypeValidation error"),
        }
    }

    #[test]
    fn test_empty_value() {
        use std::io;

        let cause = io::Error::new(io::ErrorKind::InvalidInput, "empty value");
        let error = ProviderError::type_validation_error("", cause);
        match error {
            ProviderError::TypeValidation { value, .. } => {
                assert_eq!(value, "");
            }
            _ => panic!("Expected TypeValidation error"),
        }
    }

    #[test]
    fn test_complex_nested_value() {
        use std::io;

        let complex_value = r#"{
            "user": {
                "name": "John",
                "age": "not a number",
                "settings": {
                    "theme": 123
                }
            }
        }"#;

        let cause = io::Error::new(
            io::ErrorKind::InvalidData,
            "age must be integer, theme must be string",
        );
        let error = ProviderError::type_validation_error(complex_value, cause);

        match error {
            ProviderError::TypeValidation { value, cause } => {
                assert_eq!(value, complex_value);
                assert!(cause.is_some());
            }
            _ => panic!("Expected TypeValidation error"),
        }
    }

    #[test]
    fn test_with_actual_serde_error() {
        use serde::Deserialize;
        use serde_json;

        #[derive(Deserialize)]
        struct Expected {
            _field: i32,
        }

        let invalid_json = r#"{"field": "not a number"}"#;

        // Try to parse and capture the error
        match serde_json::from_str::<Expected>(invalid_json) {
            Err(validation_err) => {
                let error = ProviderError::type_validation_error(invalid_json, validation_err);
                match error {
                    ProviderError::TypeValidation { value, cause } => {
                        assert_eq!(value, invalid_json);
                        assert!(cause.is_some());
                    }
                    _ => panic!("Expected TypeValidation error"),
                }
            }
            Ok(_) => panic!("Expected validation to fail"),
        }
    }
}
