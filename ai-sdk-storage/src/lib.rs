//! # AI SDK Storage
//!
//! Storage interfaces and traits for the AI SDK Rust project.
//!
//! This crate provides a unified storage abstraction for persisting AI-related data including:
//! - **Conversation storage**: Messages, sessions, and conversation history
//! - **Embedding storage**: Vector embeddings with similarity search (future)
//! - **Artifact storage**: Images, speech, transcripts, and other generated files (future)
//!
//! ## Architecture
//!
//! The storage system follows a provider pattern similar to the main AI SDK:
//!
//! 1. **Provider Layer**: The [`StorageProvider`] trait defines the top-level interface
//! 2. **Storage Traits**: Specialized traits for different storage types:
//!    - [`ConversationStorage`] for message and session management
//! 3. **Implementation Layer**: Concrete implementations (filesystem, databases, etc.)
//!
//! ## Usage Example
//!
//! ```rust,no_run
//! use ai_sdk_storage::{StorageProvider, ConversationStorage, ConversationSession};
//! use std::sync::Arc;
//!
//! # async fn example(storage: Arc<dyn StorageProvider>) -> Result<(), Box<dyn std::error::Error>> {
//! // Create a new conversation session
//! let session = ConversationSession {
//!     id: "session-123".to_string(),
//!     title: Some("My Conversation".to_string()),
//!     metadata: Default::default(),
//!     created_at: chrono::Utc::now(),
//!     updated_at: chrono::Utc::now(),
//! };
//!
//! // Store the session
//! let session_id = storage.conversation_storage()
//!     .create_session(session)
//!     .await?;
//!
//! // Retrieve messages for the session
//! let messages = storage.conversation_storage()
//!     .get_messages(&session_id, Some(50))
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Integration with AI SDK Core
//!
//! Storage can be optionally enabled on builders:
//!
//! ```rust,ignore
//! use ai_sdk_core::{GenerateText, Prompt};
//! use ai_sdk_storage::StorageProvider;
//! use std::sync::Arc;
//!
//! let result = GenerateText::new(model, Prompt::text("Hello"))
//!     .with_storage(storage)
//!     .with_session_id("session-123".to_string())
//!     .execute()
//!     .await?;
//! ```

// Public modules
pub mod conversation;
pub mod error;
pub mod provider;
pub mod utils;

// Re-export key types for convenience
pub use conversation::{
    ConversationSession, ConversationStorage, MessageMetadata, MessageRole, SessionMetadata,
    StoredMessage, ToolCallRecord, UsageStats,
};
pub use error::StorageError;
pub use provider::StorageProvider;
