use std::collections::HashMap;

use super::ProviderError;

/// Builder for constructing APICallError with a fluent interface
///
/// # Examples
///
/// ```
/// use ai_sdk_provider::error::{APICallErrorBuilder, ProviderError};
/// use std::collections::HashMap;
///
/// let mut headers = HashMap::new();
/// headers.insert("content-type".to_string(), "application/json".to_string());
///
/// let error = APICallErrorBuilder::new(
///     "Request failed with status 429",
///     "https://api.openai.com/v1/chat/completions",
///     r#"{"model": "gpt-4", "messages": []}"#,
/// )
/// .status_code(429)
/// .response_headers(headers)
/// .response_body(r#"{"error": {"message": "Rate limit exceeded"}}"#)
/// .data(r#"{"retry_after": 60}"#)
/// .build();
///
/// // Check if retryable (automatically determined from status code)
/// assert!(error.is_retryable());
/// assert_eq!(error.status_code(), Some(429));
/// ```
#[derive(Debug, Default)]
pub struct APICallErrorBuilder {
    message: String,
    url: String,
    request_body_values: String,
    status_code: Option<u16>,
    response_headers: Option<HashMap<String, String>>,
    response_body: Option<String>,
    is_retryable: Option<bool>,
    data: Option<String>,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl APICallErrorBuilder {
    /// Create a new APICallErrorBuilder with required fields
    pub fn new(
        message: impl Into<String>,
        url: impl Into<String>,
        request_body_values: impl Into<String>,
    ) -> Self {
        Self {
            message: message.into(),
            url: url.into(),
            request_body_values: request_body_values.into(),
            ..Default::default()
        }
    }

    /// Set the HTTP status code
    pub fn status_code(mut self, status_code: u16) -> Self {
        self.status_code = Some(status_code);
        self
    }

    /// Set the response headers
    pub fn response_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.response_headers = Some(headers);
        self
    }

    /// Set the response body
    pub fn response_body(mut self, body: impl Into<String>) -> Self {
        self.response_body = Some(body.into());
        self
    }

    /// Set whether the error is retryable
    pub fn is_retryable(mut self, retryable: bool) -> Self {
        self.is_retryable = Some(retryable);
        self
    }

    /// Set additional data
    pub fn data(mut self, data: impl Into<String>) -> Self {
        self.data = Some(data.into());
        self
    }

    /// Set the source error
    pub fn source(mut self, source: impl std::error::Error + Send + Sync + 'static) -> Self {
        self.source = Some(Box::new(source));
        self
    }

    /// Build the ProviderError
    pub fn build(self) -> ProviderError {
        ProviderError::api_call_error_with_details(
            self.message,
            self.url,
            self.request_body_values,
            self.status_code,
            self.response_headers,
            self.response_body,
            self.is_retryable,
            self.data,
            self.source,
        )
    }
}

impl ProviderError {
    /// Check if an error is retryable
    ///
    /// Returns `true` only for `APICallError` variants that are marked as retryable.
    /// All other error types return `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_provider::error::ProviderError;
    ///
    /// let error = ProviderError::api_call_error_with_details(
    ///     "Server error",
    ///     "https://api.example.com",
    ///     "{}",
    ///     Some(500),
    ///     None,
    ///     None,
    ///     None,
    ///     None,
    ///     None,
    /// );
    ///
    /// assert!(error.is_retryable());
    ///
    /// let model_error = ProviderError::model_error("Some error");
    /// assert!(!model_error.is_retryable());
    /// ```
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::APICallError { is_retryable, .. } => *is_retryable,
            _ => false,
        }
    }

    /// Get the status code if this is an API call error
    ///
    /// Returns `Some(status_code)` for `APICallError` variants, `None` for all others.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_provider::error::ProviderError;
    ///
    /// let error = ProviderError::api_call_error_with_details(
    ///     "Not found",
    ///     "https://api.example.com",
    ///     "{}",
    ///     Some(404),
    ///     None,
    ///     None,
    ///     None,
    ///     None,
    ///     None,
    /// );
    ///
    /// assert_eq!(error.status_code(), Some(404));
    ///
    /// let model_error = ProviderError::model_error("Some error");
    /// assert_eq!(model_error.status_code(), None);
    /// ```
    pub fn status_code(&self) -> Option<u16> {
        match self {
            Self::APICallError { status_code, .. } => *status_code,
            _ => None,
        }
    }

    /// Get the response body if this is an API call error
    ///
    /// Returns `Some(&str)` for `APICallError` variants with a response body, `None` for all others.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_provider::error::ProviderError;
    ///
    /// let error = ProviderError::api_call_error_with_details(
    ///     "Error",
    ///     "https://api.example.com",
    ///     "{}",
    ///     Some(400),
    ///     None,
    ///     Some("Invalid request".to_string()),
    ///     None,
    ///     None,
    ///     None,
    /// );
    ///
    /// assert_eq!(error.response_body(), Some("Invalid request"));
    ///
    /// let model_error = ProviderError::model_error("Some error");
    /// assert_eq!(model_error.response_body(), None);
    /// ```
    pub fn response_body(&self) -> Option<&str> {
        match self {
            Self::APICallError { response_body, .. } => response_body.as_deref(),
            _ => None,
        }
    }

    /// Get the URL if this is an API call error
    ///
    /// Returns `Some(&str)` for `APICallError` variants, `None` for all others.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_provider::error::ProviderError;
    ///
    /// let error = ProviderError::api_call_error(
    ///     "Failed",
    ///     "https://api.example.com/v1/chat",
    ///     "{}",
    /// );
    ///
    /// assert_eq!(error.url(), Some("https://api.example.com/v1/chat"));
    ///
    /// let model_error = ProviderError::model_error("Some error");
    /// assert_eq!(model_error.url(), None);
    /// ```
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::APICallError { url, .. } => Some(url.as_str()),
            _ => None,
        }
    }

    /// Create a new APICallError with basic information
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_provider::error::ProviderError;
    ///
    /// let error = ProviderError::api_call_error(
    ///     "Request failed",
    ///     "https://api.openai.com/v1/chat/completions",
    ///     r#"{"model": "gpt-4"}"#,
    /// );
    ///
    /// assert_eq!(error.url(), Some("https://api.openai.com/v1/chat/completions"));
    /// assert!(!error.is_retryable());
    /// ```
    pub fn api_call_error(
        message: impl Into<String>,
        url: impl Into<String>,
        request_body_values: impl Into<String>,
    ) -> Self {
        Self::APICallError {
            message: message.into(),
            url: url.into(),
            request_body_values: request_body_values.into(),
            status_code: None,
            response_headers: None,
            response_body: None,
            is_retryable: false,
            data: None,
            source: None,
        }
    }

    /// Create a new APICallError with full details
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_provider::error::ProviderError;
    ///
    /// let error = ProviderError::api_call_error_with_details(
    ///     "Internal server error",
    ///     "https://api.openai.com/v1/chat/completions",
    ///     r#"{"model": "gpt-4"}"#,
    ///     Some(500),
    ///     None,
    ///     Some("Server is temporarily unavailable".to_string()),
    ///     None, // Auto-determine retryability (will be true for 500)
    ///     None,
    ///     None,
    /// );
    ///
    /// assert!(error.is_retryable());
    /// assert_eq!(error.status_code(), Some(500));
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn api_call_error_with_details(
        message: impl Into<String>,
        url: impl Into<String>,
        request_body_values: impl Into<String>,
        status_code: Option<u16>,
        response_headers: Option<HashMap<String, String>>,
        response_body: Option<String>,
        is_retryable: Option<bool>,
        data: Option<String>,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    ) -> Self {
        // Auto-determine retryability based on status code if not explicitly provided
        let is_retryable = is_retryable.unwrap_or_else(|| {
            status_code.is_some_and(|code| {
                code == 408 || // request timeout
                code == 409 || // conflict
                code == 429 || // too many requests
                code >= 500 // server error
            })
        });

        Self::APICallError {
            message: message.into(),
            url: url.into(),
            request_body_values: request_body_values.into(),
            status_code,
            response_headers,
            response_body,
            is_retryable,
            data,
            source,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_call_error_basic() {
        let error = ProviderError::api_call_error(
            "Request failed",
            "https://api.example.com/v1/chat",
            r#"{"model": "gpt-4"}"#,
        );

        assert!(matches!(error, ProviderError::APICallError { .. }));
        assert_eq!(error.url(), Some("https://api.example.com/v1/chat"));
        assert!(!error.is_retryable());
    }

    #[test]
    fn test_api_call_error_with_details() {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());

        let error = ProviderError::api_call_error_with_details(
            "Too many requests",
            "https://api.example.com/v1/chat",
            r#"{"model": "gpt-4"}"#,
            Some(429),
            Some(headers),
            Some(r#"{"error": "rate limit exceeded"}"#.to_string()),
            None, // Let it auto-determine retryability
            Some(r#"{"retry_after": 60}"#.to_string()),
            None,
        );

        assert!(matches!(error, ProviderError::APICallError { .. }));
        assert_eq!(error.status_code(), Some(429));
        assert!(error.is_retryable()); // 429 should be retryable
        assert_eq!(
            error.response_body(),
            Some(r#"{"error": "rate limit exceeded"}"#)
        );
    }

    #[test]
    fn test_api_call_error_builder() {
        let mut headers = HashMap::new();
        headers.insert("x-request-id".to_string(), "abc123".to_string());

        let error = APICallErrorBuilder::new(
            "Internal server error",
            "https://api.example.com/v1/chat",
            r#"{"model": "gpt-4", "messages": []}"#,
        )
        .status_code(500)
        .response_headers(headers)
        .response_body(r#"{"error": "internal error"}"#)
        .data(r#"{"request_id": "abc123"}"#)
        .build();

        assert!(matches!(error, ProviderError::APICallError { .. }));
        assert_eq!(error.status_code(), Some(500));
        assert!(error.is_retryable()); // 500 should be retryable
    }

    #[test]
    fn test_retryable_status_codes() {
        // Test 408 - Request Timeout
        let error = ProviderError::api_call_error_with_details(
            "Timeout",
            "https://api.example.com",
            "{}",
            Some(408),
            None,
            None,
            None,
            None,
            None,
        );
        assert!(error.is_retryable());

        // Test 409 - Conflict
        let error = ProviderError::api_call_error_with_details(
            "Conflict",
            "https://api.example.com",
            "{}",
            Some(409),
            None,
            None,
            None,
            None,
            None,
        );
        assert!(error.is_retryable());

        // Test 429 - Too Many Requests
        let error = ProviderError::api_call_error_with_details(
            "Rate limited",
            "https://api.example.com",
            "{}",
            Some(429),
            None,
            None,
            None,
            None,
            None,
        );
        assert!(error.is_retryable());

        // Test 500 - Internal Server Error
        let error = ProviderError::api_call_error_with_details(
            "Server error",
            "https://api.example.com",
            "{}",
            Some(500),
            None,
            None,
            None,
            None,
            None,
        );
        assert!(error.is_retryable());

        // Test 503 - Service Unavailable
        let error = ProviderError::api_call_error_with_details(
            "Service unavailable",
            "https://api.example.com",
            "{}",
            Some(503),
            None,
            None,
            None,
            None,
            None,
        );
        assert!(error.is_retryable());
    }

    #[test]
    fn test_non_retryable_status_codes() {
        // Test 400 - Bad Request
        let error = ProviderError::api_call_error_with_details(
            "Bad request",
            "https://api.example.com",
            "{}",
            Some(400),
            None,
            None,
            None,
            None,
            None,
        );
        assert!(!error.is_retryable());

        // Test 401 - Unauthorized
        let error = ProviderError::api_call_error_with_details(
            "Unauthorized",
            "https://api.example.com",
            "{}",
            Some(401),
            None,
            None,
            None,
            None,
            None,
        );
        assert!(!error.is_retryable());

        // Test 404 - Not Found
        let error = ProviderError::api_call_error_with_details(
            "Not found",
            "https://api.example.com",
            "{}",
            Some(404),
            None,
            None,
            None,
            None,
            None,
        );
        assert!(!error.is_retryable());
    }

    #[test]
    fn test_explicit_retryable_override() {
        // Override default retryability for a 500 error
        let error = ProviderError::api_call_error_with_details(
            "Server error",
            "https://api.example.com",
            "{}",
            Some(500),
            None,
            None,
            Some(false), // Explicitly mark as not retryable
            None,
            None,
        );
        assert!(!error.is_retryable());

        // Override default for a 400 error
        let error = ProviderError::api_call_error_with_details(
            "Bad request",
            "https://api.example.com",
            "{}",
            Some(400),
            None,
            None,
            Some(true), // Explicitly mark as retryable
            None,
            None,
        );
        assert!(error.is_retryable());
    }

    #[test]
    fn test_error_display() {
        let error = ProviderError::api_call_error("Test error", "https://api.example.com", "{}");
        let display = format!("{}", error);
        assert!(display.contains("API call failed"));
        assert!(display.contains("Test error"));
    }

    #[test]
    fn test_helper_methods_on_api_call_error() {
        let error = ProviderError::api_call_error_with_details(
            "Error",
            "https://api.example.com",
            "{}",
            Some(429),
            None,
            Some("rate limited".to_string()),
            None,
            None,
            None,
        );

        assert!(error.is_retryable());
        assert_eq!(error.status_code(), Some(429));
        assert_eq!(error.response_body(), Some("rate limited"));
        assert_eq!(error.url(), Some("https://api.example.com"));
    }

    #[test]
    fn test_helper_methods_on_other_errors() {
        let model_error = ProviderError::model_error("test");
        assert!(!model_error.is_retryable());
        assert_eq!(model_error.status_code(), None);
        assert_eq!(model_error.response_body(), None);
        assert_eq!(model_error.url(), None);

        let no_such_model = ProviderError::no_such_model("gpt-4", "openai");
        assert!(!no_such_model.is_retryable());
        assert_eq!(no_such_model.status_code(), None);
        assert_eq!(no_such_model.response_body(), None);
        assert_eq!(no_such_model.url(), None);
    }
}
