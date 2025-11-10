//! Storage configuration and error handling for conversation persistence.
//!
//! This module provides configuration types for controlling how storage errors are handled,
//! retry behavior for transient failures, and telemetry hooks for monitoring storage operations.

use std::sync::Arc;
use std::time::Duration;

#[cfg(feature = "storage")]
use ai_sdk_storage::StorageError;

/// Defines how storage errors should be handled during generation/streaming.
///
/// # Examples
///
/// ```
/// use ai_sdk_core::storage_config::StorageErrorBehavior;
///
/// // Log and continue (default behavior)
/// let behavior = StorageErrorBehavior::LogWarning;
///
/// // Fail fast on any storage error
/// let behavior = StorageErrorBehavior::ReturnError;
///
/// // Retry transient errors, then log
/// let behavior = StorageErrorBehavior::RetryThenLog {
///     max_retries: 3,
///     initial_delay: std::time::Duration::from_millis(100),
///     max_delay: std::time::Duration::from_secs(5),
///     backoff_multiplier: 2.0,
/// };
/// ```
#[derive(Debug, Clone, Default)]
pub enum StorageErrorBehavior {
    /// Log a warning and continue (non-blocking).
    ///
    /// This is the default behavior. Storage failures are logged but do not prevent
    /// the generation/streaming operation from completing successfully.
    #[default]
    LogWarning,

    /// Return the error to the caller (blocking).
    ///
    /// Storage failures will cause the entire operation to fail and return an error.
    /// Use this when storage is critical and you want to ensure all data is persisted.
    ReturnError,

    /// Retry transient errors with exponential backoff, then log as warning.
    ///
    /// This attempts to retry transient errors (I/O errors, provider errors) with
    /// exponential backoff. If all retries are exhausted, the error is logged as a warning
    /// and the operation continues.
    RetryThenLog {
        /// Maximum number of retry attempts (default: 3)
        max_retries: u32,
        /// Initial delay between retries (default: 100ms)
        initial_delay: Duration,
        /// Maximum delay between retries (default: 5s)
        max_delay: Duration,
        /// Backoff multiplier for exponential backoff (default: 2.0)
        backoff_multiplier: f64,
    },
}

impl StorageErrorBehavior {
    /// Creates a `RetryThenLog` behavior with default retry parameters.
    ///
    /// Default parameters:
    /// - `max_retries`: 3
    /// - `initial_delay`: 100ms
    /// - `max_delay`: 5s
    /// - `backoff_multiplier`: 2.0
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::storage_config::StorageErrorBehavior;
    ///
    /// let behavior = StorageErrorBehavior::default_retry();
    /// ```
    pub fn default_retry() -> Self {
        StorageErrorBehavior::RetryThenLog {
            max_retries: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 2.0,
        }
    }
}

/// Callback for storage operation events (success and failure).
///
/// This trait allows implementing custom telemetry, metrics, or logging for storage operations.
///
/// # Examples
///
/// ```
/// use ai_sdk_core::storage_config::StorageTelemetry;
///
/// #[derive(Clone)]
/// struct MyTelemetry;
///
/// impl StorageTelemetry for MyTelemetry {
///     fn on_operation_start(&self, operation: &str, resource_id: &str) {
///         println!("Starting: {} on {}", operation, resource_id);
///     }
///
///     fn on_operation_success(&self, operation: &str, resource_id: &str, duration_ms: u64) {
///         println!("Success: {} on {} took {}ms", operation, resource_id, duration_ms);
///     }
///
///     fn on_operation_error(&self, operation: &str, resource_id: &str, error: &str, attempt: u32) {
///         eprintln!("Error: {} on {} (attempt {}): {}", operation, resource_id, attempt, error);
///     }
/// }
/// ```
pub trait StorageTelemetry: Send + Sync {
    /// Called when a storage operation starts.
    ///
    /// # Arguments
    ///
    /// * `operation` - The type of operation (e.g., "store_session", "store_user_message")
    /// * `resource_id` - The ID of the resource being operated on (session_id, message_id, etc.)
    fn on_operation_start(&self, operation: &str, resource_id: &str);

    /// Called when a storage operation succeeds.
    ///
    /// # Arguments
    ///
    /// * `operation` - The type of operation
    /// * `resource_id` - The ID of the resource
    /// * `duration_ms` - Duration of the operation in milliseconds
    fn on_operation_success(&self, operation: &str, resource_id: &str, duration_ms: u64);

    /// Called when a storage operation fails (including retry attempts).
    ///
    /// # Arguments
    ///
    /// * `operation` - The type of operation
    /// * `resource_id` - The ID of the resource
    /// * `error` - The error message
    /// * `attempt` - The attempt number (1-indexed)
    fn on_operation_error(&self, operation: &str, resource_id: &str, error: &str, attempt: u32);
}

/// Configuration for storage error handling and telemetry.
///
/// This type is used to configure how storage operations handle errors, retries,
/// and telemetry/monitoring.
///
/// # Examples
///
/// ```
/// use ai_sdk_core::storage_config::{StorageConfig, StorageErrorBehavior};
///
/// // Default configuration (log warnings, no telemetry)
/// let config = StorageConfig::default();
///
/// // Custom configuration with retries
/// let config = StorageConfig::new()
///     .with_error_behavior(StorageErrorBehavior::default_retry());
///
/// // With custom error behavior
/// let config = StorageConfig::new()
///     .with_error_behavior(StorageErrorBehavior::ReturnError);
/// ```
#[derive(Clone, Default)]
pub struct StorageConfig {
    /// How to handle storage errors
    pub error_behavior: StorageErrorBehavior,
    /// Optional telemetry callback for monitoring storage operations
    pub telemetry: Option<Arc<dyn StorageTelemetry>>,
}

impl StorageConfig {
    /// Creates a new `StorageConfig` with default settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::storage_config::StorageConfig;
    ///
    /// let config = StorageConfig::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the error handling behavior.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::storage_config::{StorageConfig, StorageErrorBehavior};
    ///
    /// let config = StorageConfig::new()
    ///     .with_error_behavior(StorageErrorBehavior::ReturnError);
    /// ```
    pub fn with_error_behavior(mut self, behavior: StorageErrorBehavior) -> Self {
        self.error_behavior = behavior;
        self
    }

    /// Sets the telemetry callback.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_core::storage_config::StorageConfig;
    /// use std::sync::Arc;
    /// # use ai_sdk_core::storage_config::StorageTelemetry;
    /// #
    /// # #[derive(Clone)]
    /// # struct MyTelemetry;
    /// # impl StorageTelemetry for MyTelemetry {
    /// #     fn on_operation_start(&self, _: &str, _: &str) {}
    /// #     fn on_operation_success(&self, _: &str, _: &str, _: u64) {}
    /// #     fn on_operation_error(&self, _: &str, _: &str, _: &str, _: u32) {}
    /// # }
    ///
    /// let telemetry = Arc::new(MyTelemetry);
    /// let config = StorageConfig::new()
    ///     .with_telemetry(telemetry);
    /// ```
    pub fn with_telemetry(mut self, telemetry: Arc<dyn StorageTelemetry>) -> Self {
        self.telemetry = Some(telemetry);
        self
    }
}

/// Retry execution with exponential backoff for transient errors.
///
/// This function attempts to execute the provided operation, retrying on transient errors
/// with exponential backoff according to the configured parameters.
///
/// # Arguments
///
/// * `operation_name` - Name of the operation for telemetry
/// * `resource_id` - ID of the resource for telemetry
/// * `max_retries` - Maximum number of retry attempts
/// * `initial_delay` - Initial delay between retries
/// * `max_delay` - Maximum delay between retries
/// * `backoff_multiplier` - Multiplier for exponential backoff
/// * `telemetry` - Optional telemetry callback
/// * `operation` - The async operation to execute
///
/// # Returns
///
/// The result of the operation, or the last error if all retries are exhausted.
#[cfg(feature = "storage")]
#[allow(clippy::too_many_arguments)]
pub async fn retry_with_backoff<F, Fut, T>(
    operation_name: &str,
    resource_id: &str,
    max_retries: u32,
    initial_delay: Duration,
    max_delay: Duration,
    backoff_multiplier: f64,
    telemetry: &Option<Arc<dyn StorageTelemetry>>,
    mut operation: F,
) -> Result<T, StorageError>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, StorageError>>,
{
    let start_time = std::time::Instant::now();

    if let Some(tel) = telemetry {
        tel.on_operation_start(operation_name, resource_id);
    }

    let mut current_delay = initial_delay;
    let mut last_error = None;

    for attempt in 1..=max_retries + 1 {
        match operation().await {
            Ok(result) => {
                if let Some(tel) = telemetry {
                    tel.on_operation_success(
                        operation_name,
                        resource_id,
                        start_time.elapsed().as_millis() as u64,
                    );
                }
                return Ok(result);
            }
            Err(e) => {
                if let Some(tel) = telemetry {
                    tel.on_operation_error(operation_name, resource_id, &e.to_string(), attempt);
                }

                // Check if error is transient and we have retries remaining
                if e.is_transient() && attempt <= max_retries {
                    last_error = Some(e);
                    tokio::time::sleep(current_delay).await;
                    // Calculate next delay with exponential backoff
                    current_delay = std::cmp::min(
                        Duration::from_millis(
                            (current_delay.as_millis() as f64 * backoff_multiplier) as u64,
                        ),
                        max_delay,
                    );
                } else {
                    // Non-transient error or no retries remaining
                    return Err(e);
                }
            }
        }
    }

    // All retries exhausted
    Err(last_error.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_error_behavior_default() {
        let behavior = StorageErrorBehavior::default();
        assert!(matches!(behavior, StorageErrorBehavior::LogWarning));
    }

    #[test]
    fn test_storage_error_behavior_default_retry() {
        let behavior = StorageErrorBehavior::default_retry();
        if let StorageErrorBehavior::RetryThenLog {
            max_retries,
            initial_delay,
            max_delay,
            backoff_multiplier,
        } = behavior
        {
            assert_eq!(max_retries, 3);
            assert_eq!(initial_delay, Duration::from_millis(100));
            assert_eq!(max_delay, Duration::from_secs(5));
            assert_eq!(backoff_multiplier, 2.0);
        } else {
            panic!("Expected RetryThenLog variant");
        }
    }

    #[test]
    fn test_storage_config_default() {
        let config = StorageConfig::default();
        assert!(matches!(
            config.error_behavior,
            StorageErrorBehavior::LogWarning
        ));
        assert!(config.telemetry.is_none());
    }

    #[test]
    fn test_storage_config_builder() {
        let config = StorageConfig::new().with_error_behavior(StorageErrorBehavior::ReturnError);

        assert!(matches!(
            config.error_behavior,
            StorageErrorBehavior::ReturnError
        ));
    }

    #[cfg(feature = "storage")]
    #[tokio::test]
    async fn test_retry_with_backoff_success_first_try() {
        use std::sync::atomic::{AtomicU32, Ordering};
        let call_count = AtomicU32::new(0);
        let result = retry_with_backoff(
            "test_op",
            "test_id",
            3,
            Duration::from_millis(10),
            Duration::from_millis(100),
            2.0,
            &None,
            || async {
                call_count.fetch_add(1, Ordering::SeqCst);
                Ok::<_, StorageError>(42)
            },
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(call_count.load(Ordering::SeqCst), 1);
    }

    #[cfg(feature = "storage")]
    #[tokio::test]
    async fn test_retry_with_backoff_success_after_retry() {
        use std::sync::atomic::{AtomicU32, Ordering};
        let call_count = AtomicU32::new(0);
        let result = retry_with_backoff(
            "test_op",
            "test_id",
            3,
            Duration::from_millis(10),
            Duration::from_millis(100),
            2.0,
            &None,
            || async {
                let count = call_count.fetch_add(1, Ordering::SeqCst) + 1;
                if count < 3 {
                    Err(StorageError::IoError("transient".to_string()))
                } else {
                    Ok(42)
                }
            },
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(call_count.load(Ordering::SeqCst), 3);
    }

    #[cfg(feature = "storage")]
    #[tokio::test]
    async fn test_retry_with_backoff_failure_exhausted() {
        use std::sync::atomic::{AtomicU32, Ordering};
        let call_count = AtomicU32::new(0);
        let result = retry_with_backoff(
            "test_op",
            "test_id",
            2,
            Duration::from_millis(10),
            Duration::from_millis(100),
            2.0,
            &None,
            || async {
                call_count.fetch_add(1, Ordering::SeqCst);
                Err::<i32, _>(StorageError::IoError("persistent".to_string()))
            },
        )
        .await;

        assert!(result.is_err());
        assert_eq!(call_count.load(Ordering::SeqCst), 3); // Initial + 2 retries
    }

    #[cfg(feature = "storage")]
    #[tokio::test]
    async fn test_retry_with_backoff_non_transient_error() {
        use std::sync::atomic::{AtomicU32, Ordering};
        let call_count = AtomicU32::new(0);
        let result = retry_with_backoff(
            "test_op",
            "test_id",
            3,
            Duration::from_millis(10),
            Duration::from_millis(100),
            2.0,
            &None,
            || async {
                call_count.fetch_add(1, Ordering::SeqCst);
                Err::<i32, _>(StorageError::NotFound("resource".to_string()))
            },
        )
        .await;

        assert!(result.is_err());
        assert_eq!(call_count.load(Ordering::SeqCst), 1); // No retries for non-transient errors
    }
}
