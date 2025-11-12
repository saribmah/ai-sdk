//! Provider-defined tools for Anthropic.
//!
//! This module provides factory functions for creating Anthropic's provider-defined tools
//! using the provider-defined tool factory pattern from `ai-sdk-provider-utils`.
//!
//! Each tool in this module follows the pattern:
//! - A factory function that creates a `Tool` instance
//! - Input schema matching Anthropic's API requirements
//! - Optional configuration via `ProviderDefinedToolOptions`
//!
//! # Available Tools
//!
//! - [`bash_20241022()`] - Execute bash commands in a sandboxed environment (version 20241022)
//! - [`bash_20250124()`] - Execute bash commands in a sandboxed environment (version 20250124)
//! - [`code_execution_20250522()`] - Execute Python code in a sandboxed environment (version 20250522)
//! - [`code_execution_20250825()`] - Execute code and edit files (bash + text editor) (version 20250825)
//! - [`computer_20241022()`] - Control computer display, keyboard, and mouse (version 20241022)
//! - [`computer_20250124()`] - Control computer with enhanced actions (version 20250124)
//! - [`memory_20250818()`] - Store and retrieve information across conversations (version 20250818)
//! - [`text_editor_20241022()`] - View and edit files with text editor commands (version 20241022)
//! - [`text_editor_20250124()`] - View and edit files with text editor commands (version 20250124)
//! - [`text_editor_20250429()`] - View and edit files without undo_edit (version 20250429)
//! - [`text_editor_20250728()`] - View and edit files with maxCharacters support (version 20250728)
//! - [`web_fetch_20250910()`] - Fetch content from URLs with optional citations (version 20250910)
//! - [`web_search_20250305()`] - Search the web with citations always enabled (version 20250305)
//!
//! # Example
//!
//! ```
//! use ai_sdk_anthropic::provider_tool::bash_20241022;
//! use ai_sdk_provider_utils::ProviderDefinedToolOptions;
//!
//! // Create a bash tool with default options
//! let tool = bash_20241022(None);
//!
//! // Create a bash tool with approval requirement
//! let tool_with_approval = bash_20241022(Some(
//!     ProviderDefinedToolOptions::new()
//!         .with_description("Execute bash commands")
//!         .with_needs_approval(true)
//! ));
//! ```

/// Bash tool (version 20241022).
pub mod bash_20241022;
/// Bash tool (version 20250124).
pub mod bash_20250124;
/// Code execution tool (version 20250522).
pub mod code_execution_20250522;
/// Code execution tool (version 20250825).
pub mod code_execution_20250825;
/// Computer control tool (version 20241022).
pub mod computer_20241022;
/// Computer control tool (version 20250124).
pub mod computer_20250124;
/// Memory tool (version 20250818).
pub mod memory_20250818;
/// Text editor tool (version 20241022).
pub mod text_editor_20241022;
/// Text editor tool (version 20250124).
pub mod text_editor_20250124;
/// Text editor tool (version 20250429).
pub mod text_editor_20250429;
/// Text editor tool (version 20250728).
pub mod text_editor_20250728;
/// Web fetch tool (version 20250910).
pub mod web_fetch_20250910;
/// Web search tool (version 20250305).
pub mod web_search_20250305;

// Re-export the tool functions for convenience
pub use bash_20241022::bash_20241022;
pub use bash_20250124::bash_20250124;
pub use code_execution_20250522::code_execution_20250522;
pub use code_execution_20250825::code_execution_20250825;
pub use computer_20241022::computer_20241022;
pub use computer_20250124::computer_20250124;
pub use memory_20250818::memory_20250818;
pub use text_editor_20241022::text_editor_20241022;
pub use text_editor_20250124::text_editor_20250124;
pub use text_editor_20250429::text_editor_20250429;
pub use text_editor_20250728::text_editor_20250728;
pub use web_fetch_20250910::web_fetch_20250910;
pub use web_search_20250305::web_search_20250305;
