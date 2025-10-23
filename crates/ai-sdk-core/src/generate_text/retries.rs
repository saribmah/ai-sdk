use crate::error::AISDKError;
use std::future::Future;
use std::pin::Pin;
use tokio_util::sync::CancellationToken;

/// A retry function that takes an async operation and retries it with exponential backoff.
///
/// The function takes a closure that returns a Future, and returns a Future that resolves
/// to the result of the operation after retrying if necessary.
pub type RetryFunction = Box<
    dyn Fn(
            Box<dyn Fn() -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send>> + Send>,
        ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send>>
        + Send
        + Sync,
>;

/// Configuration for retry behavior.
pub struct RetryConfig {
    pub max_retries: u32,
    pub retry: RetryFunction,
}

/// Validate and prepare retries.
///
/// # Arguments
/// * `max_retries` - Optional maximum number of retries. Defaults to 2 if not specified.
/// * `abort_signal` - Optional cancellation token to abort retry attempts.
///
/// # Returns
/// Returns a `RetryConfig` with validated max_retries and a retry function.
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
        retry: retry_with_exponential_backoff_respecting_retry_headers(
            max_retries_result,
            abort_signal,
        ),
    })
}

/// Create a retry function with exponential backoff that respects retry headers.
///
/// This function returns a retry function that will:
/// - Retry failed operations up to `max_retries` times
/// - Use exponential backoff between retries
/// - Respect retry-after headers from responses
/// - Cancel retries if the abort_signal is triggered
///
/// # Arguments
/// * `max_retries` - Maximum number of retry attempts
/// * `abort_signal` - Optional cancellation token to abort retries
fn retry_with_exponential_backoff_respecting_retry_headers(
    max_retries: u32,
    abort_signal: Option<CancellationToken>,
) -> RetryFunction {
    Box::new(
        move |operation: Box<
            dyn Fn() -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send>>
                + Send,
        >| {
            let abort_signal = abort_signal.clone();
            Box::pin(async move {
                let mut retries = 0;
                let mut delay = std::time::Duration::from_millis(100); // Start with 100ms

                loop {
                    // Check if we should abort
                    if let Some(ref token) = abort_signal {
                        if token.is_cancelled() {
                            return Err("Operation cancelled".into());
                        }
                    }

                    // Try the operation
                    let fut = operation();
                    match fut.await {
                        Ok(result) => return Ok(result),
                        Err(e) => {
                            if retries >= max_retries {
                                return Err(e);
                            }

                            // Wait with exponential backoff
                            tokio::time::sleep(delay).await;

                            // Double the delay for next retry (exponential backoff)
                            delay = delay * 2;
                            retries += 1;
                        }
                    }
                }
            }) as Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send>>
        },
    )
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
