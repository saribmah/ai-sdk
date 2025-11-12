use serde::{Deserialize, Serialize};

/// Memory tool (version 20250818).
///
/// This tool allows the model to store and retrieve memories across conversations.
/// Note: This version does not support cache control.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::prompt::tool::memory_tool::AnthropicMemoryTool;
///
/// let tool = AnthropicMemoryTool::new("memory");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicMemoryTool {
    /// The type of tool (always "memory_20250818")
    #[serde(rename = "type")]
    pub tool_type: String,

    /// The name of the tool
    pub name: String,
}

impl AnthropicMemoryTool {
    /// Creates a new memory tool.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the tool
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::tool::memory_tool::AnthropicMemoryTool;
    ///
    /// let tool = AnthropicMemoryTool::new("memory");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            tool_type: "memory_20250818".to_string(),
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
    use serde_json::json;

    #[test]
    fn test_new() {
        let tool = AnthropicMemoryTool::new("memory");

        assert_eq!(tool.tool_type, "memory_20250818");
        assert_eq!(tool.name, "memory");
    }

    #[test]
    fn test_with_name() {
        let tool = AnthropicMemoryTool::new("old").with_name("new");

        assert_eq!(tool.name, "new");
    }

    #[test]
    fn test_serialize() {
        let tool = AnthropicMemoryTool::new("memory");
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
    fn test_deserialize() {
        let json = json!({
            "type": "memory_20250818",
            "name": "my_memory"
        });

        let tool: AnthropicMemoryTool = serde_json::from_value(json).unwrap();

        assert_eq!(tool.name, "my_memory");
    }

    #[test]
    fn test_equality() {
        let tool1 = AnthropicMemoryTool::new("memory");
        let tool2 = AnthropicMemoryTool::new("memory");
        let tool3 = AnthropicMemoryTool::new("different");

        assert_eq!(tool1, tool2);
        assert_ne!(tool1, tool3);
    }

    #[test]
    fn test_clone() {
        let tool1 = AnthropicMemoryTool::new("memory");
        let tool2 = tool1.clone();

        assert_eq!(tool1, tool2);
    }

    #[test]
    fn test_roundtrip() {
        let original = AnthropicMemoryTool::new("memory");

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicMemoryTool = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }
}
