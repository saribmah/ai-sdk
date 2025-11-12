use serde::{Deserialize, Serialize};

use crate::prompt::message::cache_control::AnthropicCacheControl;

/// Bash tool versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BashToolVersion {
    /// Version from October 2024
    #[serde(rename = "bash_20241022")]
    Bash20241022,

    /// Version from January 2025
    #[serde(rename = "bash_20250124")]
    Bash20250124,
}

/// Bash tool.
///
/// This tool allows the model to execute bash commands in a sandboxed environment.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::prompt::tool::bash_tool::{AnthropicBashTool, BashToolVersion};
///
/// let tool = AnthropicBashTool::new("bash", BashToolVersion::Bash20250124);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicBashTool {
    /// The name of the tool
    pub name: String,

    /// The version/type of the bash tool
    #[serde(rename = "type")]
    pub tool_type: BashToolVersion,

    /// Optional cache control configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}

impl AnthropicBashTool {
    /// Creates a new bash tool.
    pub fn new(name: impl Into<String>, tool_type: BashToolVersion) -> Self {
        Self {
            name: name.into(),
            tool_type,
            cache_control: None,
        }
    }

    /// Creates a new bash tool with the October 2024 version.
    pub fn new_20241022(name: impl Into<String>) -> Self {
        Self::new(name, BashToolVersion::Bash20241022)
    }

    /// Creates a new bash tool with the January 2025 version.
    pub fn new_20250124(name: impl Into<String>) -> Self {
        Self::new(name, BashToolVersion::Bash20250124)
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

    /// Updates the tool version.
    pub fn with_version(mut self, version: BashToolVersion) -> Self {
        self.tool_type = version;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    use serde_json::json;

    #[test]
    fn test_new() {
        let tool = AnthropicBashTool::new("bash", BashToolVersion::Bash20250124);

        assert_eq!(tool.name, "bash");
        assert_eq!(tool.tool_type, BashToolVersion::Bash20250124);
        assert!(tool.cache_control.is_none());
    }

    #[test]
    fn test_new_20241022() {
        let tool = AnthropicBashTool::new_20241022("bash");
        assert_eq!(tool.tool_type, BashToolVersion::Bash20241022);
    }

    #[test]
    fn test_new_20250124() {
        let tool = AnthropicBashTool::new_20250124("bash");
        assert_eq!(tool.tool_type, BashToolVersion::Bash20250124);
    }

    #[test]
    fn test_serialize() {
        let tool = AnthropicBashTool::new_20250124("bash");
        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "name": "bash",
                "type": "bash_20250124"
            })
        );
    }

    #[test]
    fn test_serialize_with_cache() {
        let tool = AnthropicBashTool::new_20250124("bash")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "name": "bash",
                "type": "bash_20250124",
                "cache_control": {
                    "type": "ephemeral",
                    "ttl": "1h"
                }
            })
        );
    }

    #[test]
    fn test_deserialize() {
        let json = json!({
            "name": "my_bash",
            "type": "bash_20241022",
            "cache_control": {
                "type": "ephemeral",
                "ttl": "5m"
            }
        });

        let tool: AnthropicBashTool = serde_json::from_value(json).unwrap();

        assert_eq!(tool.name, "my_bash");
        assert_eq!(tool.tool_type, BashToolVersion::Bash20241022);
        assert_eq!(
            tool.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
        );
    }

    #[test]
    fn test_roundtrip() {
        let original = AnthropicBashTool::new_20250124("bash")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicBashTool = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }
}
