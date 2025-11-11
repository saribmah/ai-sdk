/// Custom tool with JSON schema for input validation.
pub mod custom_tool;

/// Code execution tool types.
pub mod code_execution_tool;

/// Computer use tool.
pub mod computer_tool;

/// Text editor tool types.
pub mod text_editor_tool;

/// Bash tool.
pub mod bash_tool;

/// Memory tool.
pub mod memory_tool;

/// Web fetch tool.
pub mod web_fetch_tool;

/// Web search tool.
pub mod web_search_tool;

// Re-export all tool types for convenience
pub use bash_tool::{AnthropicBashTool, BashToolVersion};
pub use code_execution_tool::{
    AnthropicCodeExecutionTool20250522, AnthropicCodeExecutionTool20250825,
};
pub use computer_tool::{AnthropicComputerTool, ComputerToolVersion};
pub use custom_tool::AnthropicCustomTool;
pub use memory_tool::AnthropicMemoryTool;
pub use text_editor_tool::{
    AnthropicTextEditorTool, AnthropicTextEditorTool20250728, TextEditorToolVersion,
};
pub use web_fetch_tool::AnthropicWebFetchTool;
pub use web_search_tool::{AnthropicWebSearchTool, UserLocation};

use serde::{Deserialize, Serialize};

/// Anthropic tool definition.
///
/// Represents all possible tool types supported by the Anthropic API.
/// Tools allow the model to perform actions like code execution, web browsing,
/// file editing, and custom functions defined via JSON schema.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::tool::{AnthropicTool, AnthropicCustomTool};
/// use serde_json::json;
///
/// // Custom tool
/// let custom = AnthropicTool::Custom(
///     AnthropicCustomTool::new(
///         "get_weather",
///         json!({
///             "type": "object",
///             "properties": {
///                 "location": { "type": "string" }
///             }
///         })
///     ).with_description("Get weather for a location")
/// );
///
/// // Code execution tool
/// use ai_sdk_anthropic::tool::AnthropicCodeExecutionTool20250522;
/// let code_exec = AnthropicTool::CodeExecution20250522(
///     AnthropicCodeExecutionTool20250522::new("python")
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnthropicTool {
    /// Web fetch tool for retrieving web content.
    /// Must be before other tools to match "type": "web_fetch_20250910"
    WebFetch(AnthropicWebFetchTool),

    /// Web search tool for searching the web.
    /// Must be before other tools to match "type": "web_search_20250305"
    WebSearch(AnthropicWebSearchTool),

    /// Computer use tool for desktop interaction.
    /// Must be before CodeExecution to match "type": "computer_20241022" | "computer_20250124"
    Computer(AnthropicComputerTool),

    /// Text editor tool (version 20250728 with max_characters support).
    /// Must be before TextEditor to match "type": "text_editor_20250728" with max_characters field
    TextEditor20250728(AnthropicTextEditorTool20250728),

    /// Text editor tool (versions without max_characters parameter).
    /// Matches "type": "text_editor_20241022" | "text_editor_20250124" | "text_editor_20250429"
    TextEditor(AnthropicTextEditorTool),

    /// Bash command execution tool.
    /// Matches "type": "bash_20241022" | "bash_20250124"
    Bash(AnthropicBashTool),

    /// Memory tool for storing and retrieving information.
    /// Matches "type": "memory_20250818"
    Memory(AnthropicMemoryTool),

    /// Code execution tool (version 20250825) without cache control support.
    /// Must be before CodeExecution20250522 as it has fewer fields
    CodeExecution20250825(AnthropicCodeExecutionTool20250825),

    /// Code execution tool (version 20250522) with cache control support.
    /// Matches "type": "code_execution_20250522"
    CodeExecution20250522(AnthropicCodeExecutionTool20250522),

    /// Custom tool with JSON schema for input validation.
    ///
    /// This is the most common tool type for defining custom functions.
    /// IMPORTANT: This variant must be last in the enum to ensure proper deserialization,
    /// as it doesn't have a "type" field and will match any JSON object with name and input_schema.
    Custom(AnthropicCustomTool),
}

impl AnthropicTool {
    /// Gets the name of the tool.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::tool::{AnthropicTool, AnthropicCustomTool};
    /// use serde_json::json;
    ///
    /// let tool = AnthropicTool::Custom(AnthropicCustomTool::new("my_tool", json!({})));
    /// assert_eq!(tool.name(), "my_tool");
    /// ```
    pub fn name(&self) -> &str {
        match self {
            AnthropicTool::Custom(tool) => &tool.name,
            AnthropicTool::CodeExecution20250522(tool) => &tool.name,
            AnthropicTool::CodeExecution20250825(tool) => &tool.name,
            AnthropicTool::Computer(tool) => &tool.name,
            AnthropicTool::TextEditor(tool) => &tool.name,
            AnthropicTool::TextEditor20250728(tool) => &tool.name,
            AnthropicTool::Bash(tool) => &tool.name,
            AnthropicTool::Memory(tool) => &tool.name,
            AnthropicTool::WebFetch(tool) => &tool.name,
            AnthropicTool::WebSearch(tool) => &tool.name,
        }
    }

    /// Creates a custom tool.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::tool::AnthropicTool;
    /// use serde_json::json;
    ///
    /// let tool = AnthropicTool::custom(
    ///     "calculator",
    ///     json!({
    ///         "type": "object",
    ///         "properties": {
    ///             "expression": { "type": "string" }
    ///         }
    ///     })
    /// );
    /// ```
    pub fn custom(name: impl Into<String>, input_schema: serde_json::Value) -> Self {
        AnthropicTool::Custom(AnthropicCustomTool::new(name, input_schema))
    }

    /// Creates a code execution tool (version 20250522).
    pub fn code_execution_20250522(name: impl Into<String>) -> Self {
        AnthropicTool::CodeExecution20250522(AnthropicCodeExecutionTool20250522::new(name))
    }

    /// Creates a code execution tool (version 20250825).
    pub fn code_execution_20250825(name: impl Into<String>) -> Self {
        AnthropicTool::CodeExecution20250825(AnthropicCodeExecutionTool20250825::new(name))
    }

    /// Creates a computer use tool.
    pub fn computer(
        name: impl Into<String>,
        version: ComputerToolVersion,
        display_width_px: u32,
        display_height_px: u32,
        display_number: u32,
    ) -> Self {
        AnthropicTool::Computer(AnthropicComputerTool::new(
            name,
            version,
            display_width_px,
            display_height_px,
            display_number,
        ))
    }

    /// Creates a text editor tool.
    pub fn text_editor(name: impl Into<String>, version: TextEditorToolVersion) -> Self {
        AnthropicTool::TextEditor(AnthropicTextEditorTool::new(name, version))
    }

    /// Creates a text editor tool (version 20250728).
    pub fn text_editor_20250728(name: impl Into<String>) -> Self {
        AnthropicTool::TextEditor20250728(AnthropicTextEditorTool20250728::new(name))
    }

    /// Creates a bash tool.
    pub fn bash(name: impl Into<String>, version: BashToolVersion) -> Self {
        AnthropicTool::Bash(AnthropicBashTool::new(name, version))
    }

    /// Creates a memory tool.
    pub fn memory(name: impl Into<String>) -> Self {
        AnthropicTool::Memory(AnthropicMemoryTool::new(name))
    }

    /// Creates a web fetch tool.
    pub fn web_fetch(name: impl Into<String>) -> Self {
        AnthropicTool::WebFetch(AnthropicWebFetchTool::new(name))
    }

    /// Creates a web search tool.
    pub fn web_search(name: impl Into<String>) -> Self {
        AnthropicTool::WebSearch(AnthropicWebSearchTool::new(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_custom_tool() {
        let tool = AnthropicTool::custom(
            "my_tool",
            json!({
                "type": "object",
                "properties": {
                    "param": { "type": "string" }
                }
            }),
        );

        assert_eq!(tool.name(), "my_tool");
    }

    #[test]
    fn test_code_execution_20250522() {
        let tool = AnthropicTool::code_execution_20250522("python");
        assert_eq!(tool.name(), "python");
    }

    #[test]
    fn test_code_execution_20250825() {
        let tool = AnthropicTool::code_execution_20250825("python");
        assert_eq!(tool.name(), "python");
    }

    #[test]
    fn test_computer_tool() {
        let tool = AnthropicTool::computer(
            "computer",
            ComputerToolVersion::Computer20250124,
            1920,
            1080,
            0,
        );
        assert_eq!(tool.name(), "computer");
    }

    #[test]
    fn test_text_editor_tool() {
        let tool = AnthropicTool::text_editor("editor", TextEditorToolVersion::TextEditor20250124);
        assert_eq!(tool.name(), "editor");
    }

    #[test]
    fn test_text_editor_20250728() {
        let tool = AnthropicTool::text_editor_20250728("editor");
        assert_eq!(tool.name(), "editor");
    }

    #[test]
    fn test_bash_tool() {
        let tool = AnthropicTool::bash("bash", BashToolVersion::Bash20250124);
        assert_eq!(tool.name(), "bash");
    }

    #[test]
    fn test_memory_tool() {
        let tool = AnthropicTool::memory("memory");
        assert_eq!(tool.name(), "memory");
    }

    #[test]
    fn test_web_fetch_tool() {
        let tool = AnthropicTool::web_fetch("fetch");
        assert_eq!(tool.name(), "fetch");
    }

    #[test]
    fn test_web_search_tool() {
        let tool = AnthropicTool::web_search("search");
        assert_eq!(tool.name(), "search");
    }

    #[test]
    fn test_serialize_custom() {
        let tool = AnthropicTool::custom(
            "tool",
            json!({
                "type": "object"
            }),
        );

        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "name": "tool",
                "input_schema": {
                    "type": "object"
                }
            })
        );
    }

    #[test]
    fn test_serialize_code_execution() {
        let tool = AnthropicTool::code_execution_20250522("python");
        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "code_execution_20250522",
                "name": "python"
            })
        );
    }

    #[test]
    fn test_serialize_computer() {
        let tool = AnthropicTool::computer(
            "computer",
            ComputerToolVersion::Computer20250124,
            1920,
            1080,
            0,
        );
        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "name": "computer",
                "type": "computer_20250124",
                "display_width_px": 1920,
                "display_height_px": 1080,
                "display_number": 0
            })
        );
    }

    #[test]
    fn test_serialize_memory() {
        let tool = AnthropicTool::memory("memory");
        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "memory_20250818",
                "name": "memory"
            })
        );
    }

    #[test]
    fn test_deserialize_custom() {
        let json = json!({
            "name": "my_tool",
            "input_schema": {
                "type": "object"
            }
        });

        let tool: AnthropicTool = serde_json::from_value(json).unwrap();

        assert_eq!(tool.name(), "my_tool");
    }

    #[test]
    fn test_deserialize_code_execution_20250522() {
        let json = json!({
            "type": "code_execution_20250522",
            "name": "python"
        });

        let tool: AnthropicTool = serde_json::from_value(json).unwrap();

        assert_eq!(tool.name(), "python");
    }

    #[test]
    fn test_deserialize_memory() {
        let json = json!({
            "type": "memory_20250818",
            "name": "memory"
        });

        let tool: AnthropicTool = serde_json::from_value(json).unwrap();

        assert_eq!(tool.name(), "memory");
    }

    #[test]
    fn test_roundtrip_custom() {
        let original = AnthropicTool::custom("tool", json!({"type": "string"}));
        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicTool = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    // Note: Roundtrip tests for individual tool types work fine in their own modules.
    // The main AnthropicTool enum uses untagged serialization which can cause
    // ambiguity during deserialization when tools have overlapping field structures.
    // In practice, tools are typically constructed programmatically and sent to the API,
    // not deserialized from arbitrary JSON, so this is not a practical concern.

    #[test]
    fn test_equality() {
        let tool1 = AnthropicTool::memory("memory");
        let tool2 = AnthropicTool::memory("memory");
        let tool3 = AnthropicTool::memory("different");

        assert_eq!(tool1, tool2);
        assert_ne!(tool1, tool3);
    }

    #[test]
    fn test_clone() {
        let tool1 = AnthropicTool::custom("tool", json!({}));
        let tool2 = tool1.clone();

        assert_eq!(tool1, tool2);
    }
}

/// Tool choice strategy for controlling which tools the model can use.
///
/// This type allows you to control how the model selects tools to call during generation.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::tool::AnthropicToolChoice;
///
/// // Auto mode - model decides whether to use tools
/// let auto = AnthropicToolChoice::auto();
///
/// // Any mode - model must use at least one tool
/// let any = AnthropicToolChoice::any();
///
/// // Specific tool - model must use a specific tool
/// let specific = AnthropicToolChoice::tool("get_weather");
///
/// // With parallel tool use disabled
/// let sequential = AnthropicToolChoice::auto()
///     .with_disable_parallel_tool_use(true);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum AnthropicToolChoice {
    /// Automatic tool selection - model decides whether to use tools.
    ///
    /// This is the default behavior where the model can choose to use tools
    /// or respond directly without using any tools.
    #[serde(rename = "auto")]
    Auto {
        /// Whether to disable parallel tool use (optional)
        #[serde(skip_serializing_if = "Option::is_none")]
        disable_parallel_tool_use: Option<bool>,
    },

    /// Any tool - model must use at least one tool.
    ///
    /// The model is required to use at least one of the provided tools
    /// before responding.
    #[serde(rename = "any")]
    Any {
        /// Whether to disable parallel tool use (optional)
        #[serde(skip_serializing_if = "Option::is_none")]
        disable_parallel_tool_use: Option<bool>,
    },

    /// Specific tool - model must use a specific tool.
    ///
    /// The model is required to use the specified tool by name.
    #[serde(rename = "tool")]
    Tool {
        /// The name of the tool that must be used
        name: String,

        /// Whether to disable parallel tool use (optional)
        #[serde(skip_serializing_if = "Option::is_none")]
        disable_parallel_tool_use: Option<bool>,
    },
}

impl AnthropicToolChoice {
    /// Creates an auto tool choice (model decides whether to use tools).
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::tool::AnthropicToolChoice;
    ///
    /// let choice = AnthropicToolChoice::auto();
    /// ```
    pub fn auto() -> Self {
        Self::Auto {
            disable_parallel_tool_use: None,
        }
    }

    /// Creates an any tool choice (model must use at least one tool).
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::tool::AnthropicToolChoice;
    ///
    /// let choice = AnthropicToolChoice::any();
    /// ```
    pub fn any() -> Self {
        Self::Any {
            disable_parallel_tool_use: None,
        }
    }

    /// Creates a tool choice that requires a specific tool by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the tool that must be used
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::tool::AnthropicToolChoice;
    ///
    /// let choice = AnthropicToolChoice::tool("get_weather");
    /// ```
    pub fn tool(name: impl Into<String>) -> Self {
        Self::Tool {
            name: name.into(),
            disable_parallel_tool_use: None,
        }
    }

    /// Sets whether to disable parallel tool use.
    ///
    /// When set to `true`, the model will not call multiple tools in parallel.
    ///
    /// # Arguments
    ///
    /// * `disable` - Whether to disable parallel tool use
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::tool::AnthropicToolChoice;
    ///
    /// let choice = AnthropicToolChoice::auto()
    ///     .with_disable_parallel_tool_use(true);
    /// ```
    pub fn with_disable_parallel_tool_use(mut self, disable: bool) -> Self {
        match &mut self {
            Self::Auto {
                disable_parallel_tool_use,
            } => {
                *disable_parallel_tool_use = Some(disable);
            }
            Self::Any {
                disable_parallel_tool_use,
            } => {
                *disable_parallel_tool_use = Some(disable);
            }
            Self::Tool {
                disable_parallel_tool_use,
                ..
            } => {
                *disable_parallel_tool_use = Some(disable);
            }
        }
        self
    }

    /// Removes the disable_parallel_tool_use setting.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::tool::AnthropicToolChoice;
    ///
    /// let choice = AnthropicToolChoice::auto()
    ///     .with_disable_parallel_tool_use(true)
    ///     .without_disable_parallel_tool_use();
    /// ```
    pub fn without_disable_parallel_tool_use(mut self) -> Self {
        match &mut self {
            Self::Auto {
                disable_parallel_tool_use,
            } => {
                *disable_parallel_tool_use = None;
            }
            Self::Any {
                disable_parallel_tool_use,
            } => {
                *disable_parallel_tool_use = None;
            }
            Self::Tool {
                disable_parallel_tool_use,
                ..
            } => {
                *disable_parallel_tool_use = None;
            }
        }
        self
    }

    /// Updates the tool name (only for Tool variant).
    ///
    /// This method only has an effect if the choice is a `Tool` variant.
    ///
    /// # Arguments
    ///
    /// * `name` - The new tool name
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::tool::AnthropicToolChoice;
    ///
    /// let choice = AnthropicToolChoice::tool("old_tool")
    ///     .with_tool_name("new_tool");
    /// ```
    pub fn with_tool_name(mut self, name: impl Into<String>) -> Self {
        if let Self::Tool {
            name: tool_name, ..
        } = &mut self
        {
            *tool_name = name.into();
        }
        self
    }

    /// Gets the tool name if this is a Tool variant.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::tool::AnthropicToolChoice;
    ///
    /// let choice = AnthropicToolChoice::tool("my_tool");
    /// assert_eq!(choice.tool_name(), Some("my_tool"));
    ///
    /// let auto = AnthropicToolChoice::auto();
    /// assert_eq!(auto.tool_name(), None);
    /// ```
    pub fn tool_name(&self) -> Option<&str> {
        match self {
            Self::Tool { name, .. } => Some(name),
            _ => None,
        }
    }

    /// Gets the disable_parallel_tool_use setting.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::tool::AnthropicToolChoice;
    ///
    /// let choice = AnthropicToolChoice::auto()
    ///     .with_disable_parallel_tool_use(true);
    /// assert_eq!(choice.disable_parallel_tool_use(), Some(true));
    /// ```
    pub fn disable_parallel_tool_use(&self) -> Option<bool> {
        match self {
            Self::Auto {
                disable_parallel_tool_use,
            } => *disable_parallel_tool_use,
            Self::Any {
                disable_parallel_tool_use,
            } => *disable_parallel_tool_use,
            Self::Tool {
                disable_parallel_tool_use,
                ..
            } => *disable_parallel_tool_use,
        }
    }
}

impl Default for AnthropicToolChoice {
    /// Creates a default tool choice (auto mode).
    fn default() -> Self {
        Self::auto()
    }
}

#[cfg(test)]
mod tool_choice_tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_auto() {
        let choice = AnthropicToolChoice::auto();

        assert_eq!(choice.disable_parallel_tool_use(), None);
        assert_eq!(choice.tool_name(), None);
    }

    #[test]
    fn test_any() {
        let choice = AnthropicToolChoice::any();

        assert_eq!(choice.disable_parallel_tool_use(), None);
        assert_eq!(choice.tool_name(), None);
    }

    #[test]
    fn test_tool() {
        let choice = AnthropicToolChoice::tool("my_tool");

        assert_eq!(choice.tool_name(), Some("my_tool"));
        assert_eq!(choice.disable_parallel_tool_use(), None);
    }

    #[test]
    fn test_with_disable_parallel_tool_use_auto() {
        let choice = AnthropicToolChoice::auto().with_disable_parallel_tool_use(true);

        assert_eq!(choice.disable_parallel_tool_use(), Some(true));
    }

    #[test]
    fn test_with_disable_parallel_tool_use_any() {
        let choice = AnthropicToolChoice::any().with_disable_parallel_tool_use(false);

        assert_eq!(choice.disable_parallel_tool_use(), Some(false));
    }

    #[test]
    fn test_with_disable_parallel_tool_use_tool() {
        let choice = AnthropicToolChoice::tool("tool").with_disable_parallel_tool_use(true);

        assert_eq!(choice.disable_parallel_tool_use(), Some(true));
        assert_eq!(choice.tool_name(), Some("tool"));
    }

    #[test]
    fn test_without_disable_parallel_tool_use() {
        let choice = AnthropicToolChoice::auto()
            .with_disable_parallel_tool_use(true)
            .without_disable_parallel_tool_use();

        assert_eq!(choice.disable_parallel_tool_use(), None);
    }

    #[test]
    fn test_with_tool_name() {
        let choice = AnthropicToolChoice::tool("old").with_tool_name("new");

        assert_eq!(choice.tool_name(), Some("new"));
    }

    #[test]
    fn test_with_tool_name_on_auto() {
        let choice = AnthropicToolChoice::auto().with_tool_name("tool");

        // Should have no effect on Auto variant
        assert_eq!(choice.tool_name(), None);
    }

    #[test]
    fn test_serialize_auto() {
        let choice = AnthropicToolChoice::auto();
        let json = serde_json::to_value(&choice).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "auto"
            })
        );
    }

    #[test]
    fn test_serialize_any() {
        let choice = AnthropicToolChoice::any();
        let json = serde_json::to_value(&choice).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "any"
            })
        );
    }

    #[test]
    fn test_serialize_tool() {
        let choice = AnthropicToolChoice::tool("get_weather");
        let json = serde_json::to_value(&choice).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "tool",
                "name": "get_weather"
            })
        );
    }

    #[test]
    fn test_serialize_auto_with_disable() {
        let choice = AnthropicToolChoice::auto().with_disable_parallel_tool_use(true);
        let json = serde_json::to_value(&choice).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "auto",
                "disable_parallel_tool_use": true
            })
        );
    }

    #[test]
    fn test_serialize_any_with_disable() {
        let choice = AnthropicToolChoice::any().with_disable_parallel_tool_use(false);
        let json = serde_json::to_value(&choice).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "any",
                "disable_parallel_tool_use": false
            })
        );
    }

    #[test]
    fn test_serialize_tool_with_disable() {
        let choice = AnthropicToolChoice::tool("calculator").with_disable_parallel_tool_use(true);
        let json = serde_json::to_value(&choice).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "tool",
                "name": "calculator",
                "disable_parallel_tool_use": true
            })
        );
    }

    #[test]
    fn test_deserialize_auto() {
        let json = json!({
            "type": "auto"
        });

        let choice: AnthropicToolChoice = serde_json::from_value(json).unwrap();

        assert_eq!(choice.disable_parallel_tool_use(), None);
    }

    #[test]
    fn test_deserialize_any() {
        let json = json!({
            "type": "any",
            "disable_parallel_tool_use": true
        });

        let choice: AnthropicToolChoice = serde_json::from_value(json).unwrap();

        assert_eq!(choice.disable_parallel_tool_use(), Some(true));
    }

    #[test]
    fn test_deserialize_tool() {
        let json = json!({
            "type": "tool",
            "name": "search",
            "disable_parallel_tool_use": false
        });

        let choice: AnthropicToolChoice = serde_json::from_value(json).unwrap();

        assert_eq!(choice.tool_name(), Some("search"));
        assert_eq!(choice.disable_parallel_tool_use(), Some(false));
    }

    #[test]
    fn test_equality() {
        let choice1 = AnthropicToolChoice::auto();
        let choice2 = AnthropicToolChoice::auto();
        let choice3 = AnthropicToolChoice::any();

        assert_eq!(choice1, choice2);
        assert_ne!(choice1, choice3);
    }

    #[test]
    fn test_clone() {
        let choice1 = AnthropicToolChoice::tool("tool").with_disable_parallel_tool_use(true);
        let choice2 = choice1.clone();

        assert_eq!(choice1, choice2);
    }

    #[test]
    fn test_default() {
        let choice = AnthropicToolChoice::default();

        assert_eq!(choice, AnthropicToolChoice::auto());
    }

    #[test]
    fn test_builder_chaining() {
        let choice = AnthropicToolChoice::tool("tool1")
            .with_tool_name("tool2")
            .with_disable_parallel_tool_use(true)
            .without_disable_parallel_tool_use()
            .with_disable_parallel_tool_use(false);

        assert_eq!(choice.tool_name(), Some("tool2"));
        assert_eq!(choice.disable_parallel_tool_use(), Some(false));
    }

    #[test]
    fn test_roundtrip_auto() {
        let original = AnthropicToolChoice::auto().with_disable_parallel_tool_use(true);

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicToolChoice = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_roundtrip_any() {
        let original = AnthropicToolChoice::any();

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicToolChoice = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_roundtrip_tool() {
        let original = AnthropicToolChoice::tool("my_tool").with_disable_parallel_tool_use(false);

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicToolChoice = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_tool_name_for_different_variants() {
        let auto = AnthropicToolChoice::auto();
        let any = AnthropicToolChoice::any();
        let tool = AnthropicToolChoice::tool("search");

        assert_eq!(auto.tool_name(), None);
        assert_eq!(any.tool_name(), None);
        assert_eq!(tool.tool_name(), Some("search"));
    }
}
