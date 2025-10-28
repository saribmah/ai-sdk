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
        let mut delay = std::time::Duration::from_millis(100); // Start with 100ms

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
                        return Err(AISDKError::model_error(e.to_string()));
                    }

                    // Wait with exponential backoff
                    tokio::time::sleep(delay).await;

                    // Double the delay for next retry (exponential backoff)
                    delay = delay * 2;
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
}
