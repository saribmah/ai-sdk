use crate::error::AISDKError;
use std::time::Duration;

/// Builder for [`AISDKError::RetryableError`].
///
/// # Examples
///
/// ```
/// use llm_kit_core::error::{AISDKError, RetryableErrorBuilder};
/// use std::time::Duration;
///
/// let error = RetryableErrorBuilder::new("Rate limit exceeded")
///     .retry_after(Duration::from_secs(60))
///     .build();
///
/// match error {
///     AISDKError::RetryableError { message, retry_after } => {
///         assert_eq!(message, "Rate limit exceeded");
///         assert_eq!(retry_after, Some(Duration::from_secs(60)));
///     }
///     _ => panic!("Expected RetryableError"),
/// }
/// ```
#[derive(Debug, Clone)]
pub struct RetryableErrorBuilder {
    message: String,
    retry_after: Option<Duration>,
}

impl RetryableErrorBuilder {
    /// Creates a new builder for a retryable error.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of the error
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::RetryableErrorBuilder;
    ///
    /// let builder = RetryableErrorBuilder::new("Service temporarily unavailable");
    /// ```
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            retry_after: None,
        }
    }

    /// Sets the retry delay hint from the provider.
    ///
    /// This is typically based on information from the Retry-After header or similar.
    ///
    /// # Arguments
    ///
    /// * `retry_after` - Duration to wait before retrying
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::RetryableErrorBuilder;
    /// use std::time::Duration;
    ///
    /// let builder = RetryableErrorBuilder::new("Rate limited")
    ///     .retry_after(Duration::from_secs(30));
    /// ```
    pub fn retry_after(mut self, retry_after: Duration) -> Self {
        self.retry_after = Some(retry_after);
        self
    }

    /// Builds the [`AISDKError::RetryableError`] error.
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::RetryableErrorBuilder;
    /// use std::time::Duration;
    ///
    /// let error = RetryableErrorBuilder::new("Temporary error")
    ///     .retry_after(Duration::from_secs(10))
    ///     .build();
    /// ```
    pub fn build(self) -> AISDKError {
        AISDKError::RetryableError {
            message: self.message,
            retry_after: self.retry_after,
        }
    }
}

impl AISDKError {
    /// Creates a new retryable error without a retry delay hint.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of the error
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::AISDKError;
    ///
    /// let error = AISDKError::retryable_error("Network timeout");
    ///
    /// match error {
    ///     AISDKError::RetryableError { message, retry_after } => {
    ///         assert_eq!(message, "Network timeout");
    ///         assert_eq!(retry_after, None);
    ///     }
    ///     _ => panic!("Expected RetryableError"),
    /// }
    /// ```
    pub fn retryable_error(message: impl Into<String>) -> Self {
        Self::RetryableError {
            message: message.into(),
            retry_after: None,
        }
    }

    /// Creates a new retryable error with a retry delay hint.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of the error
    /// * `retry_after` - Duration to wait before retrying
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::AISDKError;
    /// use std::time::Duration;
    ///
    /// let error = AISDKError::retryable_error_with_delay(
    ///     "Rate limit exceeded",
    ///     Duration::from_secs(60)
    /// );
    ///
    /// match error {
    ///     AISDKError::RetryableError { message, retry_after } => {
    ///         assert_eq!(message, "Rate limit exceeded");
    ///         assert_eq!(retry_after, Some(Duration::from_secs(60)));
    ///     }
    ///     _ => panic!("Expected RetryableError"),
    /// }
    /// ```
    pub fn retryable_error_with_delay(message: impl Into<String>, retry_after: Duration) -> Self {
        Self::RetryableError {
            message: message.into(),
            retry_after: Some(retry_after),
        }
    }

    /// Creates a builder for a retryable error.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of the error
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::AISDKError;
    /// use std::time::Duration;
    ///
    /// let error = AISDKError::retryable_error_builder("Service unavailable")
    ///     .retry_after(Duration::from_secs(5))
    ///     .build();
    /// ```
    pub fn retryable_error_builder(message: impl Into<String>) -> RetryableErrorBuilder {
        RetryableErrorBuilder::new(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retryable_error_simple() {
        let error = AISDKError::retryable_error("Network timeout");

        match error {
            AISDKError::RetryableError {
                message,
                retry_after,
            } => {
                assert_eq!(message, "Network timeout");
                assert_eq!(retry_after, None);
            }
            _ => panic!("Expected RetryableError"),
        }
    }

    #[test]
    fn test_retryable_error_with_delay() {
        let error = AISDKError::retryable_error_with_delay("Rate limited", Duration::from_secs(60));

        match error {
            AISDKError::RetryableError {
                message,
                retry_after,
            } => {
                assert_eq!(message, "Rate limited");
                assert_eq!(retry_after, Some(Duration::from_secs(60)));
            }
            _ => panic!("Expected RetryableError"),
        }
    }

    #[test]
    fn test_retryable_error_builder() {
        let error = RetryableErrorBuilder::new("Service unavailable")
            .retry_after(Duration::from_secs(30))
            .build();

        match error {
            AISDKError::RetryableError {
                message,
                retry_after,
            } => {
                assert_eq!(message, "Service unavailable");
                assert_eq!(retry_after, Some(Duration::from_secs(30)));
            }
            _ => panic!("Expected RetryableError"),
        }
    }

    #[test]
    fn test_retryable_error_builder_via_error() {
        let error = AISDKError::retryable_error_builder("Temporary failure")
            .retry_after(Duration::from_secs(10))
            .build();

        match error {
            AISDKError::RetryableError {
                message,
                retry_after,
            } => {
                assert_eq!(message, "Temporary failure");
                assert_eq!(retry_after, Some(Duration::from_secs(10)));
            }
            _ => panic!("Expected RetryableError"),
        }
    }

    #[test]
    fn test_retryable_error_builder_no_delay() {
        let error = RetryableErrorBuilder::new("Connection lost").build();

        match error {
            AISDKError::RetryableError {
                message,
                retry_after,
            } => {
                assert_eq!(message, "Connection lost");
                assert_eq!(retry_after, None);
            }
            _ => panic!("Expected RetryableError"),
        }
    }

    #[test]
    fn test_retryable_error_display() {
        let error = AISDKError::retryable_error("Service temporarily unavailable");
        let display = format!("{}", error);
        assert!(display.contains("Retryable error"));
        assert!(display.contains("Service temporarily unavailable"));
    }

    #[test]
    fn test_retryable_error_with_various_durations() {
        // Test with milliseconds
        let error1 = AISDKError::retryable_error_with_delay("Test 1", Duration::from_millis(500));
        match error1 {
            AISDKError::RetryableError { retry_after, .. } => {
                assert_eq!(retry_after, Some(Duration::from_millis(500)));
            }
            _ => panic!("Expected RetryableError"),
        }

        // Test with seconds
        let error2 = AISDKError::retryable_error_with_delay("Test 2", Duration::from_secs(120));
        match error2 {
            AISDKError::RetryableError { retry_after, .. } => {
                assert_eq!(retry_after, Some(Duration::from_secs(120)));
            }
            _ => panic!("Expected RetryableError"),
        }

        // Test with minutes (represented as seconds)
        let error3 = AISDKError::retryable_error_with_delay("Test 3", Duration::from_secs(5 * 60));
        match error3 {
            AISDKError::RetryableError { retry_after, .. } => {
                assert_eq!(retry_after, Some(Duration::from_secs(300)));
            }
            _ => panic!("Expected RetryableError"),
        }
    }
}
