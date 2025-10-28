use crate::error::AISDKError;
use std::future::Future;
use tokio_util::sync::CancellationToken;

/// Configuration for retry behavior.
pub struct RetryConfig {
    pub max_retries: u32,
    pub abort_signal: Option<CancellationToken>,
}

impl RetryConfig {
    /// Execute an async operation with retry logic, converting boxed errors to AISDKError.
    ///
    /// This method will retry failed operations up to `max_retries` times with
    /// exponential backoff, respecting retry headers and abort signals.
    ///
    /// When an error contains a retry delay hint (e.g., from a Retry-After header),
    /// that delay will be used instead of the default exponential backoff.
    ///
    /// This variant is specifically designed for operations that return
    /// `Result<T, Box<dyn std::error::Error>>`, such as the `do_generate` method.
    pub async fn execute_with_boxed_error<F, Fut, T>(
        &self,
        operation: F,
    ) -> Result<T, AISDKError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, Box<dyn std::error::Error>>>,
    {
        let mut retries = 0;
        let mut default_delay = std::time::Duration::from_millis(100); // Start with 100ms

        loop {
            // Check if we should abort
            if let Some(ref token) = self.abort_signal {
                if token.is_cancelled() {
                    return Err(AISDKError::model_error("Operation cancelled".to_string()));
                }
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
                    let delay = if let Some(retry_delay) = extract_retry_delay_from_error(&e.to_string()) {
                        retry_delay
                    } else {
                        default_delay
                    };

                    // Wait with the appropriate delay
                    tokio::time::sleep(delay).await;

                    // Double the default delay for next retry (exponential backoff)
                    default_delay = default_delay * 2;
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
fn extract_retry_delay_from_error(error_message: &str) -> Option<std::time::Duration> {
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
}
