//! # AI SDK Provider Utilities
//!
//! Shared utilities and types for AI SDK providers.
//!
//! This crate provides reusable components that are used by both provider implementations
//! and the core SDK. It includes message types, tool definitions, and other common utilities
//! that facilitate communication between the SDK and providers.
//!
//! ## Overview
//!
//! The provider utilities layer serves as a shared foundation for:
//! - **Message Types**: Standardized message formats (user, assistant, system, tool)
//! - **Content Parts**: Rich content types (text, images, files, reasoning, tool calls)
//! - **Tool System**: Tool definitions, execution, and approval workflows
//! - **Data Types**: Binary data handling (base64, bytes)
//!
//! ## Architecture
//!
//! This crate sits between `ai-sdk-provider` and `ai-sdk-core`, providing shared types
//! that both layers depend on:
//!
//! ```text
//! ai-sdk-core (high-level APIs)
//!      ↓
//! ai-sdk-provider-utils (shared types) ← You are here
//!      ↓
//! ai-sdk-provider (provider interface)
//!      ↓
//! ai-sdk-openai-compatible (implementation)
//! ```
//!
//! ## Message System
//!
//! The message system provides a unified way to represent conversations:
//!
//! ### Message Types
//!
//! ```rust
//! use ai_sdk_provider_utils::message::{Message, UserMessage, AssistantMessage, SystemMessage};
//!
//! // System message
//! let system = SystemMessage::new("You are a helpful assistant.");
//!
//! // User message
//! let user = UserMessage::new("What is the capital of France?");
//!
//! // Assistant message
//! let assistant = AssistantMessage::new("The capital of France is Paris.");
//!
//! // Messages can be converted to the unified Message type
//! let messages: Vec<Message> = vec![
//!     system.into(),
//!     user.into(),
//!     assistant.into(),
//! ];
//! ```
//!
//! ### Content Parts
//!
//! Messages can contain rich content with multiple parts:
//!
//! ```rust
//! use ai_sdk_provider_utils::message::{UserMessage, UserContentPart};
//! use ai_sdk_provider_utils::message::content_parts::{TextPart, ImagePart};
//! # use ai_sdk_provider_utils::message::DataContent;
//!
//! let message = UserMessage::with_parts(vec![
//!     UserContentPart::Text(TextPart::new("What's in this image?")),
//!     UserContentPart::Image(ImagePart::from_data(DataContent::base64("..."))),
//! ]);
//! ```
//!
//! ## Tool System
//!
//! The tool system enables language models to perform actions through function calling:
//!
//! ### Dynamic Tools
//!
//! ```rust,no_run
//! use ai_sdk_provider_utils::tool::{Tool, ToolExecutionOutput};
//! use serde_json::json;
//! use std::sync::Arc;
//!
//! let weather_tool = Tool::function(json!({
//!     "type": "object",
//!     "properties": {
//!         "location": {
//!             "type": "string",
//!             "description": "City name"
//!         }
//!     },
//!     "required": ["location"]
//! }))
//! .with_description("Get current weather for a location")
//! .with_execute(Arc::new(|input, _opts| {
//!     ToolExecutionOutput::Single(Box::pin(async move {
//!         Ok(json!({"temperature": 72, "condition": "sunny"}))
//!     }))
//! }));
//! ```
//!
//! ### Tool Types
//!
//! The crate supports three tool types:
//!
//! - **Function Tools**: User-defined tools with custom execution logic
//! - **Dynamic Tools**: Tools defined at runtime (e.g., MCP tools)
//! - **Provider-Defined Tools**: Tools with provider-specific implementations
//!
//! ### Tool Approval
//!
//! Tools can require user approval before execution:
//!
//! ```rust,no_run
//! # use ai_sdk_provider_utils::tool::Tool;
//! # use serde_json::json;
//! # let schema = json!({"type": "object"});
//! let delete_tool = Tool::function(schema)
//!     .with_description("Delete a file")
//!     .with_needs_approval(true);
//! ```
//!
//! ## Data Content
//!
//! Binary data (images, files, audio) can be represented as base64 or raw bytes:
//!
//! ```rust
//! use ai_sdk_provider_utils::message::DataContent;
//!
//! // From base64
//! let data = DataContent::base64("iVBORw0KGgo...");
//!
//! // From bytes
//! let data = DataContent::bytes(vec![0x89, 0x50, 0x4E, 0x47]);
//!
//! // Convert between formats
//! let base64_string = data.to_base64();
//! let bytes = data.to_bytes().unwrap();
//! ```
//!
//! ## Module Organization
//!
//! - [`message`]: Message types and content parts for conversations
//! - [`tool`]: Tool definitions, execution, and approval workflows
//!
//! ## Re-exports
//!
//! Common types are re-exported at the crate root for convenience:
//!
//! ```rust
//! use ai_sdk_provider_utils::{
//!     // Messages
//!     Message, UserMessage, AssistantMessage, SystemMessage, ToolMessage,
//!     // Content parts
//!     TextPart, ImagePart, FilePart, ReasoningPart, ToolCallPart, ToolResultPart,
//!     // Data
//!     DataContent,
//!     // Tools
//!     Tool, ToolCall, ToolResult, ToolError, ToolOutput,
//! };
//! # fn main() {}
//! ```
//!
//! ## Design Principles
//!
//! 1. **Provider Agnostic**: Types work across all providers
//! 2. **Type Safety**: Strong typing with compile-time guarantees
//! 3. **Cloneable**: All types are cloneable for ergonomic usage
//! 4. **Serializable**: Full serde support for all types
//! 5. **Extensible**: Provider-specific metadata via `SharedProviderOptions`
//!
//! ## See Also
//!
//! - `ai-sdk-core`: High-level builder APIs that use these types
//! - `ai-sdk-provider`: Provider interface that these types support

#![warn(missing_docs)]

/// Message types and content parts for conversations.
///
/// This module provides standardized message formats used in prompts and conversations,
/// including user messages, assistant messages, system messages, and tool messages.
/// Each message can contain rich content with multiple parts (text, images, files, etc.).
pub mod message;

/// Tool system for function calling (dynamic and type-safe).
///
/// This module provides the tool definition and execution system, including:
/// - Tool creation (function, dynamic, provider-defined)
/// - Tool execution (single-value and streaming)
/// - Tool approval workflows
/// - Tool callbacks and options
pub mod tool;

// Re-export commonly used types for convenience
pub use message::content_parts::{
    FileId, FilePart, FileSource, ImagePart, ImageSource, ReasoningPart, TextPart, ToolCallPart,
    ToolResultContentPart, ToolResultOutput, ToolResultPart,
};
pub use message::{
    AssistantContent, AssistantContentPart, AssistantMessage, DataContent, Message, SystemMessage,
    ToolContent, ToolContentPart, ToolMessage, UserContent, UserContentPart, UserMessage,
};
pub use tool::{
    NeedsApproval, OnPreliminaryToolResult, ProviderDefinedToolFactory,
    ProviderDefinedToolFactoryWithOutput, ProviderDefinedToolOptions, Tool, ToolApprovalRequest,
    ToolApprovalRequestOutput, ToolApprovalResponse, ToolCall, ToolError, ToolExecuteFunction,
    ToolExecuteOptions, ToolExecutionOutput, ToolNeedsApprovalFunction, ToolOutput, ToolResult,
    ToolType,
};
