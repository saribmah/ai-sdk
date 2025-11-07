use crate::error::AISDKError;
use std::future::Future;
use tokio_util::sync::CancellationToken;

/// Configuration for retry behavior.
pub struct RetryConfig {
    pub max_retries: u32,
    pub abort_signal: Option<CancellationToken>,
}

impl RetryConfig {
    /// Execute an async operation with retry logic.
    ///
    /// This method will retry failed operations up to `max_retries` times with
    /// exponential backoff, respecting retry hints from `AISDKError::RetryableError`
    /// and abort signals.
    ///
    /// When an error is `AISDKError::RetryableError` with a `retry_after` hint,
    /// that delay will be used instead of the default exponential backoff.
    ///
    /// # Arguments
    ///
    /// * `operation` - An async function that returns `Result<T, AISDKError>`
    ///
    /// # Returns
    ///
    /// Returns the successful result or the final error after all retries are exhausted.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let config = RetryConfig {
    ///     max_retries: 3,
    ///     abort_signal: None,
    /// };
    ///
    /// let result = config.execute(|| async {
    ///     // Your operation that returns Result<T, AISDKError>
    ///     execute_tool(tool_call).await
    /// }).await?;
    /// ```
    pub async fn execute<F, Fut, T>(&self, operation: F) -> Result<T, AISDKError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, AISDKError>>,
    {
        let mut retries = 0;
        let mut default_delay = std::time::Duration::from_millis(100); // Start with 100ms

        loop {
            // Check if we should abort
            if let Some(ref token) = self.abort_signal
                && token.is_cancelled()
            {
                return Err(AISDKError::model_error("Operation cancelled".to_string()));
            }

            // Try the operation
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if retries >= self.max_retries {
                        return Err(e);
                    }

                    // Extract retry delay hint from RetryableError, or use exponential backoff
                    let delay = match &e {
                        AISDKError::RetryableError { retry_after, .. } => {
                            retry_after.unwrap_or(default_delay)
                        }
                        _ => default_delay,
                    };

                    // Wait with the appropriate delay
                    tokio::time::sleep(delay).await;

                    // Double the default delay for next retry (exponential backoff)
                    default_delay *= 2;
                    retries += 1;
                }
            }
        }
    }

    /// Execute an async operation with retry logic, converting boxed errors to AISDKError.
    ///
    /// This method will retry failed operations up to `max_retries` times with
    /// exponential backoff, respecting retry headers and abort signals.
    ///
    /// When an error contains a retry delay hint (e.g., from a Retry-After header),
    /// that delay will be used instead of the default exponential backoff.
    ///
    /// **Note**: This method is mainly for backward compatibility. For new code,
    /// prefer using [`execute`](Self::execute) which works with `AISDKError` directly
    /// and provides better type safety with structured retry hints.
    ///
    /// This variant is specifically designed for operations that return
    /// `Result<T, Box<dyn std::error::Error>>`. It parses retry hints from error
    /// strings, whereas `execute` can use structured `RetryableError` variants.
    ///
    /// # See Also
    ///
    /// * [`execute`](Self::execute) - Recommended for new code using `Result<T, AISDKError>`
    pub async fn execute_with_boxed_error<F, Fut, T>(&self, operation: F) -> Result<T, AISDKError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, Box<dyn std::error::Error>>>,
    {
        let mut retries = 0;
        let mut default_delay = std::time::Duration::from_millis(100); // Start with 100ms

        loop {
            // Check if we should abort
            if let Some(ref token) = self.abort_signal
                && token.is_cancelled()
            {
                return Err(AISDKError::model_error("Operation cancelled".to_string()));
            }

            // Try the operation
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if retries >= self.max_retries {
                        // Check if the error string contains a retry hint
                        // This is a fallback for errors that don't use AISDKError::RetryableError
                        return Err(AISDKError::model_error(e.to_string()));
                    }

                    // Try to extract retry delay from error message
                    // If error contains "retry after X seconds", parse it
                    let delay =
                        if let Some(retry_delay) = extract_retry_delay_from_error(&e.to_string()) {
                            retry_delay
                        } else {
                            default_delay
                        };

                    // Wait with the appropriate delay
                    tokio::time::sleep(delay).await;

                    // Double the default delay for next retry (exponential backoff)
                    default_delay *= 2;
                    retries += 1;
                }
            }
        }
    }
}

/// Validate and prepare retries.
///
/// # Arguments
/// * `max_retries` - Optional maximum number of retries. Defaults to 2 if not specified.
/// * `abort_signal` - Optional cancellation token to abort retry attempts.
///
/// # Returns
/// Returns a `RetryConfig` with validated max_retries and abort signal.
///
/// # Errors
/// Returns an error if max_retries is negative.
pub fn prepare_retries(
    max_retries: Option<u32>,
    abort_signal: Option<CancellationToken>,
) -> Result<RetryConfig, AISDKError> {
    // Validate max_retries (Rust's u32 ensures it's non-negative and an integer)
    let max_retries_result = max_retries.unwrap_or(2);

    Ok(RetryConfig {
        max_retries: max_retries_result,
        abort_signal,
    })
}

/// Extract retry delay from error message.
///
/// This function attempts to parse retry delay hints embedded in error messages.
/// It looks for patterns like "retry after X seconds" or "retry-after: X".
///
/// # Arguments
///
/// * `error_message` - The error message string to parse
///
/// # Returns
///
/// `Some(Duration)` if a retry delay was found, `None` otherwise.
pub(crate) fn extract_retry_delay_from_error(error_message: &str) -> Option<std::time::Duration> {
    // Check for "retry after X seconds" pattern (case-insensitive)
    let lower = error_message.to_lowercase();

    // Try to find "retry after X" or "retry-after: X"
    if let Some(idx) = lower.find("retry") {
        let after_retry = &lower[idx..];

        // Look for numbers after "retry after" or "retry-after"
        for word in after_retry.split_whitespace() {
            // Remove common punctuation
            let cleaned = word.trim_matches(|c: char| !c.is_ascii_digit());
            if let Ok(seconds) = cleaned.parse::<u64>() {
                // Found a number that looks like retry delay in seconds
                // Cap at reasonable maximum (1 hour)
                let capped_seconds = seconds.min(3600);
                return Some(std::time::Duration::from_secs(capped_seconds));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_retries_default() {
        let config = prepare_retries(None, None).unwrap();
        assert_eq!(config.max_retries, 2);
    }

    #[test]
    fn test_prepare_retries_custom() {
        let config = prepare_retries(Some(5), None).unwrap();
        assert_eq!(config.max_retries, 5);
    }

    #[test]
    fn test_prepare_retries_zero() {
        let config = prepare_retries(Some(0), None).unwrap();
        assert_eq!(config.max_retries, 0);
    }

    #[test]
    fn test_extract_retry_delay_simple() {
        let delay = extract_retry_delay_from_error("Rate limit exceeded. Retry after 60 seconds");
        assert_eq!(delay, Some(std::time::Duration::from_secs(60)));
    }

    #[test]
    fn test_extract_retry_delay_with_punctuation() {
        let delay = extract_retry_delay_from_error("Error: retry-after: 30");
        assert_eq!(delay, Some(std::time::Duration::from_secs(30)));
    }

    #[test]
    fn test_extract_retry_delay_case_insensitive() {
        let delay = extract_retry_delay_from_error("RETRY AFTER 45 SECONDS");
        assert_eq!(delay, Some(std::time::Duration::from_secs(45)));
    }

    #[test]
    fn test_extract_retry_delay_caps_at_max() {
        let delay = extract_retry_delay_from_error("Retry after 7200 seconds");
        assert_eq!(delay, Some(std::time::Duration::from_secs(3600))); // Capped at 1 hour
    }

    #[test]
    fn test_extract_retry_delay_no_match() {
        let delay = extract_retry_delay_from_error("Generic error message");
        assert_eq!(delay, None);
    }

    #[test]
    fn test_extract_retry_delay_no_number() {
        let delay = extract_retry_delay_from_error("Retry after a while");
        assert_eq!(delay, None);
    }

    #[tokio::test]
    async fn test_execute_success_on_first_try() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicU32, Ordering};

        let config = RetryConfig {
            max_retries: 3,
            abort_signal: None,
        };

        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();

        let result = config
            .execute(move || {
                let attempts = attempts_clone.clone();
                async move {
                    attempts.fetch_add(1, Ordering::SeqCst);
                    Ok::<i32, AISDKError>(42)
                }
            })
            .await;

        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempts.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_execute_with_retryable_error_hint() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicU32, Ordering};
        use std::time::Duration;

        let config = RetryConfig {
            max_retries: 2,
            abort_signal: None,
        };

        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();
        let start = std::time::Instant::now();

        let result = config
            .execute(move || {
                let attempts = attempts_clone.clone();
                async move {
                    let count = attempts.fetch_add(1, Ordering::SeqCst) + 1;
                    if count < 2 {
                        // Return retryable error with 200ms delay hint
                        Err(AISDKError::retryable_error_with_delay(
                            "Rate limited",
                            Duration::from_millis(200),
                        ))
                    } else {
                        Ok::<i32, AISDKError>(100)
                    }
                }
            })
            .await;

        let elapsed = start.elapsed();

        assert_eq!(result.unwrap(), 100);
        assert_eq!(attempts.load(Ordering::SeqCst), 2);
        // Should have waited at least 200ms for the retry hint
        assert!(elapsed >= Duration::from_millis(200));
        // But not too long (shouldn't use exponential backoff)
        assert!(elapsed < Duration::from_millis(500));
    }

    #[tokio::test]
    async fn test_execute_with_exponential_backoff() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicU32, Ordering};

        let config = RetryConfig {
            max_retries: 3,
            abort_signal: None,
        };

        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();
        let start = std::time::Instant::now();

        let result = config
            .execute(move || {
                let attempts = attempts_clone.clone();
                async move {
                    let count = attempts.fetch_add(1, Ordering::SeqCst) + 1;
                    if count < 3 {
                        // Return regular error (no retry hint)
                        Err(AISDKError::model_error("Generic error"))
                    } else {
                        Ok::<i32, AISDKError>(200)
                    }
                }
            })
            .await;

        let elapsed = start.elapsed();

        assert_eq!(result.unwrap(), 200);
        assert_eq!(attempts.load(Ordering::SeqCst), 3);
        // Should have used exponential backoff: 100ms + 200ms = 300ms minimum
        assert!(elapsed >= std::time::Duration::from_millis(300));
    }

    #[tokio::test]
    async fn test_execute_max_retries_exhausted() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicU32, Ordering};

        let config = RetryConfig {
            max_retries: 2,
            abort_signal: None,
        };

        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();

        let result = config
            .execute(move || {
                let attempts = attempts_clone.clone();
                async move {
                    attempts.fetch_add(1, Ordering::SeqCst);
                    Err::<i32, AISDKError>(AISDKError::model_error("Always fails"))
                }
            })
            .await;

        assert!(result.is_err());
        assert_eq!(attempts.load(Ordering::SeqCst), 3); // Initial attempt + 2 retries

        if let Err(AISDKError::ModelError { message }) = result {
            assert_eq!(message, "Always fails");
        } else {
            panic!("Expected ModelError");
        }
    }

    #[tokio::test]
    async fn test_execute_with_cancellation() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicU32, Ordering};

        let token = CancellationToken::new();
        let config = RetryConfig {
            max_retries: 5,
            abort_signal: Some(token.clone()),
        };

        let token_clone = token.clone();
        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();

        let result = config
            .execute(move || {
                let token = token_clone.clone();
                let attempts = attempts_clone.clone();
                async move {
                    let count = attempts.fetch_add(1, Ordering::SeqCst) + 1;
                    if count == 2 {
                        token.cancel();
                    }
                    Err::<i32, AISDKError>(AISDKError::model_error("Error"))
                }
            })
            .await;

        assert!(result.is_err());
        if let Err(AISDKError::ModelError { message }) = result {
            assert_eq!(message, "Operation cancelled");
        } else {
            panic!("Expected cancellation error");
        }
    }

    #[tokio::test]
    async fn test_execute_retryable_without_hint() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicU32, Ordering};

        let config = RetryConfig {
            max_retries: 2,
            abort_signal: None,
        };

        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();
        let start = std::time::Instant::now();

        let result = config
            .execute(move || {
                let attempts = attempts_clone.clone();
                async move {
                    let count = attempts.fetch_add(1, Ordering::SeqCst) + 1;
                    if count < 2 {
                        // Retryable error without delay hint - should use exponential backoff
                        Err(AISDKError::retryable_error("Need to retry"))
                    } else {
                        Ok::<i32, AISDKError>(300)
                    }
                }
            })
            .await;

        let elapsed = start.elapsed();

        assert_eq!(result.unwrap(), 300);
        assert_eq!(attempts.load(Ordering::SeqCst), 2);
        // Should use default exponential backoff (100ms minimum)
        assert!(elapsed >= std::time::Duration::from_millis(100));
    }
}
