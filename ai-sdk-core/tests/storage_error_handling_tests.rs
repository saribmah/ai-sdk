//! Integration tests for storage error handling.
//!
//! These tests verify that the storage error handling configuration works correctly
//! with GenerateText, StreamText, and Agent.

#![cfg(feature = "storage")]

use ai_sdk_core::GenerateText;
use ai_sdk_core::prompt::Prompt;
use ai_sdk_core::storage_config::{
    StorageConfig, StorageErrorBehavior, StorageTelemetry, retry_with_backoff,
};
use ai_sdk_core::{Agent, AgentCallParameters, AgentInterface, AgentSettings};
use ai_sdk_provider::language_model::call_options::LanguageModelCallOptions;
use ai_sdk_provider::language_model::usage::LanguageModelUsage;
use ai_sdk_provider::{LanguageModel, language_model::*};
use ai_sdk_storage::*;
use async_trait::async_trait;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

// Mock Storage implementation that can fail on demand
#[derive(Clone)]
struct MockStorage {
    fail_count: Arc<AtomicU32>,
    call_count: Arc<AtomicU32>,
}

impl MockStorage {
    fn new(fail_count: u32) -> Self {
        Self {
            fail_count: Arc::new(AtomicU32::new(fail_count)),
            call_count: Arc::new(AtomicU32::new(0)),
        }
    }

    fn should_fail(&self) -> bool {
        self.call_count.fetch_add(1, Ordering::SeqCst);
        let remaining = self.fail_count.load(Ordering::SeqCst);
        if remaining > 0 {
            self.fail_count.fetch_sub(1, Ordering::SeqCst);
            true
        } else {
            false
        }
    }
}

#[async_trait]
impl Storage for MockStorage {
    fn generate_session_id(&self) -> String {
        "test_session".to_string()
    }

    fn generate_message_id(&self) -> String {
        "test_message".to_string()
    }

    fn generate_part_id(&self) -> String {
        "test_part".to_string()
    }

    async fn store_session(&self, _session: &Session) -> Result<(), StorageError> {
        if self.should_fail() {
            Err(StorageError::IoError("Mock I/O error".to_string()))
        } else {
            Ok(())
        }
    }

    async fn get_session(&self, _session_id: &str) -> Result<Session, StorageError> {
        Err(StorageError::NotFound("Session not found".to_string()))
    }

    async fn list_sessions(&self, _limit: Option<usize>) -> Result<Vec<Session>, StorageError> {
        Ok(vec![])
    }

    async fn delete_session(&self, _session_id: &str) -> Result<(), StorageError> {
        Ok(())
    }

    async fn store_user_message(
        &self,
        _message: &UserMessage,
        _parts: &[MessagePart],
    ) -> Result<(), StorageError> {
        if self.should_fail() {
            Err(StorageError::IoError("Mock I/O error".to_string()))
        } else {
            Ok(())
        }
    }

    async fn store_assistant_message(
        &self,
        _message: &AssistantMessage,
        _parts: &[MessagePart],
    ) -> Result<(), StorageError> {
        if self.should_fail() {
            Err(StorageError::IoError("Mock I/O error".to_string()))
        } else {
            Ok(())
        }
    }

    async fn get_message(
        &self,
        _session_id: &str,
        _message_id: &str,
    ) -> Result<(MessageRole, Vec<MessagePart>), StorageError> {
        Err(StorageError::NotFound("Message not found".to_string()))
    }

    async fn list_messages(
        &self,
        _session_id: &str,
        _limit: Option<usize>,
    ) -> Result<Vec<String>, StorageError> {
        Ok(vec![])
    }
}

// Mock telemetry to track calls
#[derive(Clone)]
struct MockTelemetry {
    start_count: Arc<AtomicU32>,
    success_count: Arc<AtomicU32>,
    error_count: Arc<AtomicU32>,
}

impl MockTelemetry {
    fn new() -> Self {
        Self {
            start_count: Arc::new(AtomicU32::new(0)),
            success_count: Arc::new(AtomicU32::new(0)),
            error_count: Arc::new(AtomicU32::new(0)),
        }
    }

    fn get_success_count(&self) -> u32 {
        self.success_count.load(Ordering::SeqCst)
    }

    fn get_error_count(&self) -> u32 {
        self.error_count.load(Ordering::SeqCst)
    }
}

impl StorageTelemetry for MockTelemetry {
    fn on_operation_start(&self, _operation: &str, _resource_id: &str) {
        self.start_count.fetch_add(1, Ordering::SeqCst);
    }

    fn on_operation_success(&self, _operation: &str, _resource_id: &str, _duration_ms: u64) {
        self.success_count.fetch_add(1, Ordering::SeqCst);
    }

    fn on_operation_error(
        &self,
        _operation: &str,
        _resource_id: &str,
        _error: &str,
        _attempt: u32,
    ) {
        self.error_count.fetch_add(1, Ordering::SeqCst);
    }
}

// Mock Language Model for testing
struct MockLanguageModel;

#[async_trait]
impl LanguageModel for MockLanguageModel {
    fn provider(&self) -> &str {
        "mock"
    }

    fn model_id(&self) -> &str {
        "mock-model-id"
    }

    async fn supported_urls(&self) -> std::collections::HashMap<String, Vec<regex::Regex>> {
        std::collections::HashMap::new()
    }

    async fn do_generate(
        &self,
        _options: LanguageModelCallOptions,
    ) -> Result<LanguageModelGenerateResponse, Box<dyn std::error::Error>> {
        use ai_sdk_provider::language_model::content::text::LanguageModelText;
        use ai_sdk_provider::language_model::content::*;
        use ai_sdk_provider::language_model::finish_reason::LanguageModelFinishReason;

        Ok(LanguageModelGenerateResponse {
            content: vec![LanguageModelContent::Text(LanguageModelText::new(
                "Mock response",
            ))],
            finish_reason: LanguageModelFinishReason::Stop,
            usage: LanguageModelUsage::default(),
            provider_metadata: None,
            request: None,
            response: None,
            warnings: vec![],
        })
    }

    async fn do_stream(
        &self,
        _options: LanguageModelCallOptions,
    ) -> Result<LanguageModelStreamResponse, Box<dyn std::error::Error>> {
        unimplemented!("Streaming not used in these tests")
    }
}

#[tokio::test]
async fn test_storage_error_behavior_log_warning() {
    // Storage that always fails
    let storage = Arc::new(MockStorage::new(10)) as Arc<dyn Storage>;
    let model = Arc::new(MockLanguageModel) as Arc<dyn LanguageModel>;

    // LogWarning should not fail the operation
    let result = GenerateText::new(model, Prompt::text("test"))
        .with_storage(storage.clone())
        .with_session_id("test_session")
        .without_history()
        .storage_error_behavior(StorageErrorBehavior::LogWarning)
        .execute()
        .await;

    // Operation should succeed despite storage failures
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_storage_error_behavior_return_error() {
    // Storage that always fails
    let storage = Arc::new(MockStorage::new(10)) as Arc<dyn Storage>;
    let model = Arc::new(MockLanguageModel) as Arc<dyn LanguageModel>;

    // ReturnError should fail the operation
    let result = GenerateText::new(model, Prompt::text("test"))
        .with_storage(storage.clone())
        .with_session_id("test_session")
        .without_history()
        .storage_error_behavior(StorageErrorBehavior::ReturnError)
        .execute()
        .await;

    // Operation should fail due to storage error
    assert!(result.is_err());
}

#[tokio::test]
async fn test_storage_error_behavior_retry_then_log_success() {
    // Storage that fails 2 times then succeeds
    let storage = Arc::new(MockStorage::new(2)) as Arc<dyn Storage>;
    let model = Arc::new(MockLanguageModel) as Arc<dyn LanguageModel>;
    let telemetry = Arc::new(MockTelemetry::new());

    let config = StorageConfig::new()
        .with_error_behavior(StorageErrorBehavior::RetryThenLog {
            max_retries: 3,
            initial_delay: Duration::from_millis(10),
            max_delay: Duration::from_millis(100),
            backoff_multiplier: 2.0,
        })
        .with_telemetry(telemetry.clone() as Arc<dyn StorageTelemetry>);

    let result = GenerateText::new(model, Prompt::text("test"))
        .with_storage(storage.clone())
        .with_session_id("test_session")
        .without_history()
        .storage_config(config)
        .execute()
        .await;

    // Operation should succeed after retries
    assert!(result.is_ok());

    // Telemetry should show retries occurred
    assert!(telemetry.get_error_count() > 0);
    assert!(telemetry.get_success_count() > 0);
}

#[tokio::test]
async fn test_storage_error_behavior_retry_exhausted() {
    // Storage that always fails
    let storage = Arc::new(MockStorage::new(10)) as Arc<dyn Storage>;
    let model = Arc::new(MockLanguageModel) as Arc<dyn LanguageModel>;

    // Retry should be exhausted and logged as warning
    let result = GenerateText::new(model, Prompt::text("test"))
        .with_storage(storage.clone())
        .with_session_id("test_session")
        .without_history()
        .storage_error_behavior(StorageErrorBehavior::RetryThenLog {
            max_retries: 2,
            initial_delay: Duration::from_millis(10),
            max_delay: Duration::from_millis(100),
            backoff_multiplier: 2.0,
        })
        .execute()
        .await;

    // Operation should succeed (storage errors logged as warnings)
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_agent_storage_error_config() -> Result<(), Box<dyn std::error::Error>> {
    // Storage that fails once then succeeds
    let storage = Arc::new(MockStorage::new(1)) as Arc<dyn Storage>;
    let model = Arc::new(MockLanguageModel) as Arc<dyn LanguageModel>;

    let settings = AgentSettings::new(model)
        .with_storage(storage.clone())
        .with_session_id("test_session")
        .storage_error_behavior(StorageErrorBehavior::RetryThenLog {
            max_retries: 2,
            initial_delay: Duration::from_millis(10),
            max_delay: Duration::from_millis(100),
            backoff_multiplier: 2.0,
        });

    let agent = Agent::new(settings);

    let result = agent
        .generate(AgentCallParameters::from_text("test"))?
        .without_history()
        .execute()
        .await;

    // Operation should succeed after retry
    assert!(result.is_ok());
    Ok(())
}

#[tokio::test]
async fn test_storage_config_default() {
    let config = StorageConfig::default();
    assert!(matches!(
        config.error_behavior,
        StorageErrorBehavior::LogWarning
    ));
    assert!(config.telemetry.is_none());
}

#[tokio::test]
async fn test_storage_config_builder() {
    let telemetry = Arc::new(MockTelemetry::new());
    let config = StorageConfig::new()
        .with_error_behavior(StorageErrorBehavior::ReturnError)
        .with_telemetry(telemetry.clone() as Arc<dyn StorageTelemetry>);

    assert!(matches!(
        config.error_behavior,
        StorageErrorBehavior::ReturnError
    ));
    assert!(config.telemetry.is_some());
}

#[tokio::test]
async fn test_retry_with_backoff_transient_vs_permanent() {
    use ai_sdk_storage::StorageError;

    let call_count = Arc::new(AtomicU32::new(0));
    let call_count_clone = call_count.clone();

    // Transient error should be retried
    let result = retry_with_backoff(
        "test_op",
        "test_id",
        2,
        Duration::from_millis(10),
        Duration::from_millis(100),
        2.0,
        &None,
        || {
            let count = call_count_clone.clone();
            async move {
                count.fetch_add(1, Ordering::SeqCst);
                Err::<(), _>(StorageError::IoError("transient".to_string()))
            }
        },
    )
    .await;

    assert!(result.is_err());
    assert_eq!(call_count.load(Ordering::SeqCst), 3); // Initial + 2 retries

    // Permanent error should NOT be retried
    let call_count2 = Arc::new(AtomicU32::new(0));
    let call_count2_clone = call_count2.clone();

    let result2 = retry_with_backoff(
        "test_op",
        "test_id",
        2,
        Duration::from_millis(10),
        Duration::from_millis(100),
        2.0,
        &None,
        || {
            let count = call_count2_clone.clone();
            async move {
                count.fetch_add(1, Ordering::SeqCst);
                Err::<(), _>(StorageError::NotFound("permanent".to_string()))
            }
        },
    )
    .await;

    assert!(result2.is_err());
    assert_eq!(call_count2.load(Ordering::SeqCst), 1); // No retries for permanent errors
}
