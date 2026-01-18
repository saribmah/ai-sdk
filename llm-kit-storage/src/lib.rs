//! # AI SDK Storage
//!
//! Storage trait and types for the AI SDK Rust project.
//!
//! This crate provides the storage abstraction for persisting AI-related data including:
//! - **Session management**: Create, retrieve, and delete conversation sessions
//! - **Message storage**: Store user and assistant messages with full metadata
//! - **Part storage**: Hierarchical storage of message parts (text, tools, files, etc.)
//!
//! ## Architecture
//!
//! The storage system follows the provider pattern similar to `llm-kit-provider`:
//!
//! 1. **Trait Layer** (`llm-kit-storage`): Defines the [`Storage`] trait - NO implementations
//! 2. **Implementation Layer**: Concrete implementations in separate crates:
//!    - `llm-kit-storage-filesystem`: Filesystem-based storage
//!    - `llm-kit-storage-mongodb`: MongoDB storage (future)
//!    - `llm-kit-storage-postgresql`: PostgreSQL storage (future)
//!
//! ## Hierarchy
//!
//! ```text
//! Session
//!   └── Message (User/Assistant)
//!       └── Part (Text/Tool/File/Reasoning/etc.)
//! ```
//!
//! ## Usage Example
//!
//! ```rust,no_run
//! use llm_kit_storage::{Storage, Session, SessionMetadata};
//! // use llm_kit_storage_filesystem::FilesystemStorage;
//! // use std::sync::Arc;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create storage provider (example - actual implementation will be in separate crate)
//! // let storage = Arc::new(FilesystemStorage::new("./ai-storage")?);
//! // storage.initialize().await?;
//!
//! // Generate a new session
//! // let session_id = storage.generate_session_id();
//! // let session = Session::new(session_id.clone())
//! //     .with_title("My Conversation".to_string());
//!
//! // Store the session
//! // storage.store_session(&session).await?;
//!
//! // Use with GenerateText
//! // let result = GenerateText::new(model, prompt)
//! //     .with_storage(storage.clone())
//! //     .with_session_id(session_id)
//! //     .execute()
//! //     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Type System
//!
//! The storage system uses strongly-typed parts for all message content:
//!
//! - [`TextPart`]: Plain text content
//! - [`ImagePart`]: Image content with dimensions
//! - [`FilePart`]: File attachments
//! - [`ReasoningPart`]: Model reasoning/thinking content
//! - [`ToolCallPart`]: Tool invocation requests
//! - [`ToolResultPart`]: Tool execution results
//! - [`SourcePart`]: Source references and citations
//!
//! All parts are unified under the [`MessagePart`] enum with tagged serialization.

#![warn(missing_docs)]

/// Error types for storage operations.
pub mod error;
/// Message types (User, Assistant, System).
pub mod message;
/// Message part types (Text, Tool, File, etc.).
pub mod part;
/// Session types and metadata.
pub mod session;
/// Storage trait definition.
pub mod storage;

// Re-export commonly used types
pub use error::StorageError;
pub use message::{AssistantMessage, MessageMetadata, MessageRole, UsageStats, UserMessage};
pub use part::{
    FileData, FilePart, ImageData, ImageDimensions, ImagePart, MessagePart, ReasoningPart,
    SourcePart, TextPart, ToolCallPart, ToolResultData, ToolResultPart,
};
pub use session::{Session, SessionMetadata};
pub use storage::Storage;
