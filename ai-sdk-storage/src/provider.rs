//! Storage provider trait and implementation.
//!
//! This module defines the main [`StorageProvider`] trait that all storage
//! implementations must implement.

use crate::conversation::ConversationStorage;
use async_trait::async_trait;
use std::sync::Arc;

/// Main storage provider trait that all storage implementations must implement.
///
/// The provider pattern allows for pluggable storage backends (filesystem, MongoDB,
/// PostgreSQL, etc.) while maintaining a consistent interface.
///
/// # Example
///
/// ```rust,no_run
/// use ai_sdk_storage::{StorageProvider, ConversationStorage};
/// use async_trait::async_trait;
/// use std::sync::Arc;
///
/// struct MyStorageProvider {
///     // Implementation fields
/// }
///
/// #[async_trait]
/// impl StorageProvider for MyStorageProvider {
///     fn conversation_storage(&self) -> Arc<dyn ConversationStorage> {
///         // Return your conversation storage implementation
///         # unimplemented!()
///     }
/// }
/// ```
#[async_trait]
pub trait StorageProvider: Send + Sync {
    /// Returns the specification version this provider implements.
    ///
    /// The default implementation returns `"v1"`. Implementations can override
    /// this to indicate support for different specification versions.
    fn specification_version(&self) -> &str {
        "v1"
    }

    /// Returns the conversation storage handler.
    ///
    /// This method provides access to conversation-related storage operations
    /// including storing messages, creating sessions, and retrieving conversation history.
    fn conversation_storage(&self) -> Arc<dyn ConversationStorage>;

    // Future: Add embedding_storage() and artifact_storage() methods
    // fn embedding_storage(&self) -> Arc<dyn EmbeddingStorage>;
    // fn artifact_storage(&self) -> Arc<dyn ArtifactStorage>;
}
