//! Anthropic provider-defined tools.
//!
//! This module provides a convenient namespace for accessing all Anthropic provider-defined tools.
//! Each tool is re-exported from the `provider_tool` module with comprehensive documentation.
//!
//! # Example
//!
//! ```
//! use ai_sdk_anthropic::anthropic_tools;
//!
//! // Create tools using the anthropic_tools namespace
//! let bash = anthropic_tools::bash_20250124(None);
//! let computer = anthropic_tools::computer_20250124(1920, 1080, None);
//! let search = anthropic_tools::web_search_20250305()
//!     .user_location("San Francisco", Some("CA"), Some("US"), None)
//!     .build();
//! ```

pub use crate::provider_tool::{
    bash_20241022, bash_20250124, code_execution_20250522, code_execution_20250825,
    computer_20241022, computer_20250124, memory_20250818, text_editor_20241022,
    text_editor_20250124, text_editor_20250429, text_editor_20250728, web_fetch_20250910,
    web_search_20250305,
};

use crate::provider_tool::{
    text_editor_20250728::TextEditor20250728Builder, web_fetch_20250910::WebFetch20250910Builder,
    web_search_20250305::WebSearch20250305Builder,
};
use ai_sdk_provider_utils::tool::provider_defined_factory::ProviderDefinedToolOptions;

/// Static tools namespace for provider access.
pub struct AnthropicTools;

impl AnthropicTools {
    pub fn bash_20241022(
        &self,
        options: Option<ProviderDefinedToolOptions>,
    ) -> ai_sdk_provider_utils::Tool {
        bash_20241022(options)
    }

    pub fn bash_20250124(
        &self,
        options: Option<ProviderDefinedToolOptions>,
    ) -> ai_sdk_provider_utils::Tool {
        bash_20250124(options)
    }

    pub fn code_execution_20250522(
        &self,
        options: Option<ProviderDefinedToolOptions>,
    ) -> ai_sdk_provider_utils::Tool {
        code_execution_20250522(options)
    }

    pub fn code_execution_20250825(
        &self,
        options: Option<ProviderDefinedToolOptions>,
    ) -> ai_sdk_provider_utils::Tool {
        code_execution_20250825(options)
    }

    pub fn computer_20241022(
        &self,
        display_width_px: u32,
        display_height_px: u32,
        display_number: Option<u32>,
    ) -> ai_sdk_provider_utils::Tool {
        computer_20241022(display_width_px, display_height_px, display_number)
    }

    pub fn computer_20250124(
        &self,
        display_width_px: u32,
        display_height_px: u32,
        display_number: Option<u32>,
    ) -> ai_sdk_provider_utils::Tool {
        computer_20250124(display_width_px, display_height_px, display_number)
    }

    pub fn memory_20250818(
        &self,
        options: Option<ProviderDefinedToolOptions>,
    ) -> ai_sdk_provider_utils::Tool {
        memory_20250818(options)
    }

    pub fn text_editor_20241022(
        &self,
        options: Option<ProviderDefinedToolOptions>,
    ) -> ai_sdk_provider_utils::Tool {
        text_editor_20241022(options)
    }

    pub fn text_editor_20250124(
        &self,
        options: Option<ProviderDefinedToolOptions>,
    ) -> ai_sdk_provider_utils::Tool {
        text_editor_20250124(options)
    }

    #[allow(deprecated)]
    pub fn text_editor_20250429(
        &self,
        options: Option<ProviderDefinedToolOptions>,
    ) -> ai_sdk_provider_utils::Tool {
        text_editor_20250429(options)
    }

    pub fn text_editor_20250728(&self) -> TextEditor20250728Builder {
        text_editor_20250728()
    }

    pub fn web_fetch_20250910(&self) -> WebFetch20250910Builder {
        web_fetch_20250910()
    }

    pub fn web_search_20250305(&self) -> WebSearch20250305Builder {
        web_search_20250305()
    }
}

/// Static instance of tools for provider access.
pub static TOOLS: AnthropicTools = AnthropicTools;

/// The bash tool (version 20241022) enables Claude to execute shell commands in a persistent bash session,
/// allowing system operations, script execution, and command-line automation.
///
/// Image results are supported.
///
/// **Tool name must be `bash`.**
///
/// # Beta Flag
/// Requires: `computer-use-2024-10-22`
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::anthropic_tools;
///
/// let tool = anthropic_tools::bash_20241022(None);
/// ```
///
/// See [`bash_20241022`] for more details.
pub use bash_20241022 as bash_20241022_tool;

/// The bash tool (version 20250124) enables Claude to execute shell commands in a persistent bash session,
/// allowing system operations, script execution, and command-line automation.
///
/// Image results are supported.
///
/// **Tool name must be `bash`.**
///
/// # Beta Flag
/// Requires: `computer-use-2025-01-24`
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::anthropic_tools;
///
/// let tool = anthropic_tools::bash_20250124(None);
/// ```
///
/// See [`bash_20250124`] for more details.
pub use bash_20250124 as bash_20250124_tool;

/// Claude can analyze data, create visualizations, perform complex calculations,
/// run system commands, create and edit files, and process uploaded files directly within
/// the API conversation.
///
/// The code execution tool allows Claude to run Python code in a secure, sandboxed environment.
///
/// **Tool name must be `code_execution`.**
///
/// # Beta Flag
/// Requires: `code-execution-2025-05-22`
///
/// # Features
/// - Python code execution
/// - Data analysis
/// - Visualization creation
/// - File processing
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::anthropic_tools;
///
/// let tool = anthropic_tools::code_execution_20250522(None);
/// ```
///
/// See [`code_execution_20250522`] for more details.
pub use code_execution_20250522 as code_execution_20250522_tool;

/// Claude can analyze data, create visualizations, perform complex calculations,
/// run system commands, create and edit files, and process uploaded files directly within
/// the API conversation.
///
/// The code execution tool allows Claude to run both Python and Bash commands and manipulate files,
/// including writing code, in a secure, sandboxed environment.
///
/// This is the latest version with enhanced Bash support and file operations.
///
/// **Tool name must be `code_execution`.**
///
/// # Beta Flag
/// Requires: `code-execution-2025-08-25`
///
/// # Features
/// - Python and Bash execution
/// - Data analysis
/// - Visualization creation
/// - File manipulation
/// - Text editor integration
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::anthropic_tools;
///
/// let tool = anthropic_tools::code_execution_20250825(None);
/// ```
///
/// See [`code_execution_20250825`] for more details.
pub use code_execution_20250825 as code_execution_20250825_tool;

/// Claude can interact with computer environments through the computer use tool (version 20241022),
/// which provides screenshot capabilities and mouse/keyboard control for autonomous desktop interaction.
///
/// Image results are supported.
///
/// **Tool name must be `computer`.**
///
/// # Beta Flag
/// Requires: `computer-use-2024-10-22`
///
/// # Parameters
///
/// * `display_width_px` - The width of the display being controlled by the model in pixels
/// * `display_height_px` - The height of the display being controlled by the model in pixels
/// * `display_number` - The display number to control (only relevant for X11 environments)
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::anthropic_tools;
///
/// let tool = anthropic_tools::computer_20241022(1920, 1080, None);
/// ```
///
/// See [`computer_20241022`] for more details.
pub use computer_20241022 as computer_20241022_tool;

/// Claude can interact with computer environments through the computer use tool (version 20250124),
/// which provides screenshot capabilities and mouse/keyboard control for autonomous desktop interaction.
///
/// Image results are supported.
///
/// **Tool name must be `computer`.**
///
/// # Beta Flag
/// Requires: `computer-use-2025-01-24`
///
/// # Parameters
///
/// * `display_width_px` - The width of the display being controlled by the model in pixels
/// * `display_height_px` - The height of the display being controlled by the model in pixels
/// * `display_number` - The display number to control (only relevant for X11 environments)
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::anthropic_tools;
///
/// let tool = anthropic_tools::computer_20250124(1920, 1080, Some(0));
/// ```
///
/// See [`computer_20250124`] for more details.
pub use computer_20250124 as computer_20250124_tool;

/// The memory tool enables Claude to store and retrieve information across conversations
/// through a memory file directory. Claude can create, read, update, and delete files
/// that persist between sessions, allowing it to build knowledge over time without keeping
/// everything in the context window.
///
/// The memory tool operates client-side—you control where and how the data is stored
/// through your own infrastructure.
///
/// **Tool name must be `memory`.**
///
/// # Beta Flag
/// Requires: `context-management-2025-06-27`
///
/// # Supported Models
/// - Claude Sonnet 4.5
/// - Claude Sonnet 4
/// - Claude Opus 4.1
/// - Claude Opus 4
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::anthropic_tools;
///
/// let tool = anthropic_tools::memory_20250818(None);
/// ```
///
/// See [`memory_20250818`] for more details.
pub use memory_20250818 as memory_20250818_tool;

/// Claude can use an Anthropic-defined text editor tool to view and modify text files,
/// helping you debug, fix, and improve your code or other text documents. This allows Claude
/// to directly interact with your files, providing hands-on assistance rather than just suggesting changes.
///
/// **Tool name must be `str_replace_editor`.**
///
/// # Beta Flag
/// Requires: `computer-use-2024-10-22`
///
/// # Supported Models
/// - Claude Sonnet 3.5
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::anthropic_tools;
///
/// let tool = anthropic_tools::text_editor_20241022(None);
/// ```
///
/// See [`text_editor_20241022`] for more details.
pub use text_editor_20241022 as text_editor_20241022_tool;

/// Claude can use an Anthropic-defined text editor tool to view and modify text files,
/// helping you debug, fix, and improve your code or other text documents. This allows Claude
/// to directly interact with your files, providing hands-on assistance rather than just suggesting changes.
///
/// **Tool name must be `str_replace_editor`.**
///
/// # Beta Flag
/// Requires: `computer-use-2025-01-24`
///
/// # Supported Models
/// - Claude Sonnet 3.7
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::anthropic_tools;
///
/// let tool = anthropic_tools::text_editor_20250124(None);
/// ```
///
/// See [`text_editor_20250124`] for more details.
pub use text_editor_20250124 as text_editor_20250124_tool;

/// Claude can use an Anthropic-defined text editor tool to view and modify text files,
/// helping you debug, fix, and improve your code or other text documents. This allows Claude
/// to directly interact with your files, providing hands-on assistance rather than just suggesting changes.
///
/// **Note:** This version does not support the "undo_edit" command.
///
/// **Tool name must be `str_replace_based_edit_tool`.**
///
/// # Beta Flag
/// Requires: `computer-use-2025-01-24`
///
/// # Deprecation Warning
/// **Deprecated:** Use [`text_editor_20250728`] instead.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::anthropic_tools;
///
/// let tool = anthropic_tools::text_editor_20250429(None);
/// ```
///
/// See [`text_editor_20250429`] for more details.
#[deprecated(
    since = "0.1.0",
    note = "Use text_editor_20250728 instead for max_characters support"
)]
pub use text_editor_20250429 as text_editor_20250429_tool;

/// Claude can use an Anthropic-defined text editor tool to view and modify text files,
/// helping you debug, fix, and improve your code or other text documents. This allows Claude
/// to directly interact with your files, providing hands-on assistance rather than just suggesting changes.
///
/// **Note:** This version does not support the "undo_edit" command and adds optional max_characters parameter.
///
/// **Tool name must be `str_replace_based_edit_tool`.**
///
/// # Beta Flag
/// Requires: `computer-use-2025-01-24`
///
/// # Supported Models
/// - Claude Sonnet 4
/// - Claude Opus 4
/// - Claude Opus 4.1
///
/// # Parameters
///
/// * `max_characters` - Optional maximum number of characters to view in the file
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::anthropic_tools;
///
/// let tool = anthropic_tools::text_editor_20250728()
///     .max_characters(10000)
///     .build();
/// ```
///
/// See [`text_editor_20250728`] for more details.
pub use text_editor_20250728 as text_editor_20250728_tool;

/// Creates a web fetch tool that gives Claude direct access to real-time web content.
///
/// **Tool name must be `web_fetch`.**
///
/// # Beta Flag
/// Requires: `web-fetch-2025-09-10`
///
/// # Parameters
///
/// * `max_uses` - The max_uses parameter limits the number of web fetches performed
/// * `allowed_domains` - Only fetch from these domains
/// * `blocked_domains` - Never fetch from these domains
/// * `citations` - Unlike web search where citations are always enabled, citations are optional for web fetch.
///   Set citations to true to enable Claude to cite specific passages from fetched documents.
/// * `max_content_tokens` - The max_content_tokens parameter limits the amount of content that will be included in the context.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::anthropic_tools;
///
/// let tool = anthropic_tools::web_fetch_20250910()
///     .allowed_domains(vec!["example.com".to_string()])
///     .citations(true)
///     .max_uses(10)
///     .build();
/// ```
///
/// See [`web_fetch_20250910`] for more details.
pub use web_fetch_20250910 as web_fetch_20250910_tool;

/// Creates a web search tool that gives Claude direct access to real-time web content.
///
/// **Tool name must be `web_search`.**
///
/// # Beta Flag
/// Requires: `web-search-2025-03-05`
///
/// # Parameters
///
/// * `max_uses` - Maximum number of web searches Claude can perform during the conversation
/// * `allowed_domains` - Optional list of domains that Claude is allowed to search
/// * `blocked_domains` - Optional list of domains that Claude should avoid when searching
/// * `user_location` - Optional user location information to provide geographically relevant search results
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::anthropic_tools;
///
/// let tool = anthropic_tools::web_search_20250305()
///     .user_location("San Francisco", Some("CA"), Some("US"), Some("America/Los_Angeles"))
///     .max_uses(5)
///     .build();
/// ```
///
/// See [`web_search_20250305`] for more details.
pub use web_search_20250305 as web_search_20250305_tool;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bash_20241022() {
        let tool = bash_20241022(None);
        assert!(tool.is_provider_defined());
    }

    #[test]
    fn test_bash_20250124() {
        let tool = bash_20250124(None);
        assert!(tool.is_provider_defined());
    }

    #[test]
    fn test_code_execution_20250522() {
        let tool = code_execution_20250522(None);
        assert!(tool.is_provider_defined());
    }

    #[test]
    fn test_code_execution_20250825() {
        let tool = code_execution_20250825(None);
        assert!(tool.is_provider_defined());
    }

    #[test]
    fn test_computer_20241022() {
        let tool = computer_20241022(1920, 1080, None);
        assert!(tool.is_provider_defined());
    }

    #[test]
    fn test_computer_20250124() {
        let tool = computer_20250124(1920, 1080, Some(0));
        assert!(tool.is_provider_defined());
    }

    #[test]
    fn test_memory_20250818() {
        let tool = memory_20250818(None);
        assert!(tool.is_provider_defined());
    }

    #[test]
    fn test_text_editor_20241022() {
        let tool = text_editor_20241022(None);
        assert!(tool.is_provider_defined());
    }

    #[test]
    fn test_text_editor_20250124() {
        let tool = text_editor_20250124(None);
        assert!(tool.is_provider_defined());
    }

    #[test]
    #[allow(deprecated)]
    fn test_text_editor_20250429() {
        let tool = text_editor_20250429(None);
        assert!(tool.is_provider_defined());
    }

    #[test]
    fn test_text_editor_20250728() {
        let tool = text_editor_20250728().build();
        assert!(tool.is_provider_defined());
    }

    #[test]
    fn test_web_fetch_20250910() {
        let tool = web_fetch_20250910().build();
        assert!(tool.is_provider_defined());
    }

    #[test]
    fn test_web_search_20250305() {
        let tool = web_search_20250305().build();
        assert!(tool.is_provider_defined());
    }

    #[test]
    fn test_aliased_bash_20241022() {
        let tool = bash_20241022_tool(None);
        assert!(tool.is_provider_defined());
    }

    #[test]
    fn test_aliased_web_search() {
        let tool = web_search_20250305_tool().build();
        assert!(tool.is_provider_defined());
    }
}
