use serde::{Deserialize, Serialize};

use crate::prompt::message::cache_control::AnthropicCacheControl;

/// Code execution tool (version 20250522).
///
/// This tool allows the model to execute Python code in a sandboxed environment.
/// The 20250522 version supports cache control.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::prompt::tool::code_execution_tool::AnthropicCodeExecutionTool20250522;
///
/// let tool = AnthropicCodeExecutionTool20250522::new("code_exec");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicCodeExecutionTool20250522 {
    /// The type of tool (always "code_execution_20250522")
    #[serde(rename = "type")]
    pub tool_type: String,

    /// The name of the tool
    pub name: String,

    /// Optional cache control configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}

impl AnthropicCodeExecutionTool20250522 {
    /// Creates a new code execution tool (version 20250522).
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the tool
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::tool::code_execution_tool::AnthropicCodeExecutionTool20250522;
    ///
    /// let tool = AnthropicCodeExecutionTool20250522::new("python_executor");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            tool_type: "code_execution_20250522".to_string(),
            name: name.into(),
            cache_control: None,
        }
    }

    /// Sets the cache control for this tool.
    pub fn with_cache_control(mut self, cache_control: AnthropicCacheControl) -> Self {
        self.cache_control = Some(cache_control);
        self
    }

    /// Removes cache control from this tool.
    pub fn without_cache_control(mut self) -> Self {
        self.cache_control = None;
        self
    }

    /// Updates the name of this tool.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
}

/// Code execution tool (version 20250825).
///
/// This is a newer version of the code execution tool that does not support
/// cache control. It allows the model to execute Python code in a sandboxed environment.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::prompt::tool::code_execution_tool::AnthropicCodeExecutionTool20250825;
///
/// let tool = AnthropicCodeExecutionTool20250825::new("code_exec");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicCodeExecutionTool20250825 {
    /// The type of tool (always "code_execution_20250825")
    #[serde(rename = "type")]
    pub tool_type: String,

    /// The name of the tool
    pub name: String,
}

impl AnthropicCodeExecutionTool20250825 {
    /// Creates a new code execution tool (version 20250825).
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the tool
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::tool::code_execution_tool::AnthropicCodeExecutionTool20250825;
    ///
    /// let tool = AnthropicCodeExecutionTool20250825::new("python_executor");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            tool_type: "code_execution_20250825".to_string(),
            name: name.into(),
        }
    }

    /// Updates the name of this tool.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    use serde_json::json;

    // Tests for AnthropicCodeExecutionTool20250522
    #[test]
    fn test_20250522_new() {
        let tool = AnthropicCodeExecutionTool20250522::new("code_exec");

        assert_eq!(tool.tool_type, "code_execution_20250522");
        assert_eq!(tool.name, "code_exec");
        assert!(tool.cache_control.is_none());
    }

    #[test]
    fn test_20250522_with_cache_control() {
        let cache = AnthropicCacheControl::new(CacheTTL::OneHour);
        let tool =
            AnthropicCodeExecutionTool20250522::new("exec").with_cache_control(cache.clone());

        assert_eq!(tool.cache_control, Some(cache));
    }

    #[test]
    fn test_20250522_without_cache_control() {
        let tool = AnthropicCodeExecutionTool20250522::new("exec")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
            .without_cache_control();

        assert!(tool.cache_control.is_none());
    }

    #[test]
    fn test_20250522_with_name() {
        let tool = AnthropicCodeExecutionTool20250522::new("old").with_name("new");

        assert_eq!(tool.name, "new");
    }

    #[test]
    fn test_20250522_serialize_minimal() {
        let tool = AnthropicCodeExecutionTool20250522::new("exec");
        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "code_execution_20250522",
                "name": "exec"
            })
        );
    }

    #[test]
    fn test_20250522_serialize_with_cache() {
        let tool = AnthropicCodeExecutionTool20250522::new("exec")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "code_execution_20250522",
                "name": "exec",
                "cache_control": {
                    "type": "ephemeral",
                    "ttl": "1h"
                }
            })
        );
    }

    #[test]
    fn test_20250522_deserialize() {
        let json = json!({
            "type": "code_execution_20250522",
            "name": "my_exec",
            "cache_control": {
                "type": "ephemeral",
                "ttl": "5m"
            }
        });

        let tool: AnthropicCodeExecutionTool20250522 = serde_json::from_value(json).unwrap();

        assert_eq!(tool.name, "my_exec");
        assert_eq!(
            tool.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
        );
    }

    #[test]
    fn test_20250522_equality() {
        let tool1 = AnthropicCodeExecutionTool20250522::new("exec");
        let tool2 = AnthropicCodeExecutionTool20250522::new("exec");
        let tool3 = AnthropicCodeExecutionTool20250522::new("different");

        assert_eq!(tool1, tool2);
        assert_ne!(tool1, tool3);
    }

    #[test]
    fn test_20250522_clone() {
        let tool1 = AnthropicCodeExecutionTool20250522::new("exec")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let tool2 = tool1.clone();

        assert_eq!(tool1, tool2);
    }

    #[test]
    fn test_20250522_roundtrip() {
        let original = AnthropicCodeExecutionTool20250522::new("exec")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicCodeExecutionTool20250522 =
            serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    // Tests for AnthropicCodeExecutionTool20250825
    #[test]
    fn test_20250825_new() {
        let tool = AnthropicCodeExecutionTool20250825::new("code_exec");

        assert_eq!(tool.tool_type, "code_execution_20250825");
        assert_eq!(tool.name, "code_exec");
    }

    #[test]
    fn test_20250825_with_name() {
        let tool = AnthropicCodeExecutionTool20250825::new("old").with_name("new");

        assert_eq!(tool.name, "new");
    }

    #[test]
    fn test_20250825_serialize() {
        let tool = AnthropicCodeExecutionTool20250825::new("exec");
        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "code_execution_20250825",
                "name": "exec"
            })
        );
    }

    #[test]
    fn test_20250825_deserialize() {
        let json = json!({
            "type": "code_execution_20250825",
            "name": "my_exec"
        });

        let tool: AnthropicCodeExecutionTool20250825 = serde_json::from_value(json).unwrap();

        assert_eq!(tool.name, "my_exec");
    }

    #[test]
    fn test_20250825_equality() {
        let tool1 = AnthropicCodeExecutionTool20250825::new("exec");
        let tool2 = AnthropicCodeExecutionTool20250825::new("exec");
        let tool3 = AnthropicCodeExecutionTool20250825::new("different");

        assert_eq!(tool1, tool2);
        assert_ne!(tool1, tool3);
    }

    #[test]
    fn test_20250825_clone() {
        let tool1 = AnthropicCodeExecutionTool20250825::new("exec");
        let tool2 = tool1.clone();

        assert_eq!(tool1, tool2);
    }

    #[test]
    fn test_20250825_roundtrip() {
        let original = AnthropicCodeExecutionTool20250825::new("exec");

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicCodeExecutionTool20250825 =
            serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }
}
