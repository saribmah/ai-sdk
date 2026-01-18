use serde::{Deserialize, Serialize};

use crate::prompt::message::cache_control::AnthropicCacheControl;

/// Text editor tool (versions without max_characters).
///
/// This tool allows the model to edit text files. These versions do not support
/// the max_characters parameter.
///
/// # Example
///
/// ```
/// use llm_kit_anthropic::prompt::tool::text_editor_tool::{AnthropicTextEditorTool, TextEditorToolVersion};
///
/// let tool = AnthropicTextEditorTool::new(
///     "editor",
///     TextEditorToolVersion::TextEditor20250124
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicTextEditorTool {
    /// The name of the tool
    pub name: String,

    /// The version/type of the text editor tool
    #[serde(rename = "type")]
    pub tool_type: TextEditorToolVersion,

    /// Optional cache control configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}

/// Text editor tool versions (without max_characters support).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextEditorToolVersion {
    /// Version from October 2024
    #[serde(rename = "text_editor_20241022")]
    TextEditor20241022,

    /// Version from January 2025
    #[serde(rename = "text_editor_20250124")]
    TextEditor20250124,

    /// Version from April 2025
    #[serde(rename = "text_editor_20250429")]
    TextEditor20250429,
}

impl AnthropicTextEditorTool {
    /// Creates a new text editor tool.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the tool
    /// * `version` - The version of the text editor tool
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::tool::text_editor_tool::{AnthropicTextEditorTool, TextEditorToolVersion};
    ///
    /// let tool = AnthropicTextEditorTool::new(
    ///     "editor",
    ///     TextEditorToolVersion::TextEditor20250124
    /// );
    /// ```
    pub fn new(name: impl Into<String>, tool_type: TextEditorToolVersion) -> Self {
        Self {
            name: name.into(),
            tool_type,
            cache_control: None,
        }
    }

    /// Creates a new text editor tool with the October 2024 version.
    pub fn new_20241022(name: impl Into<String>) -> Self {
        Self::new(name, TextEditorToolVersion::TextEditor20241022)
    }

    /// Creates a new text editor tool with the January 2025 version.
    pub fn new_20250124(name: impl Into<String>) -> Self {
        Self::new(name, TextEditorToolVersion::TextEditor20250124)
    }

    /// Creates a new text editor tool with the April 2025 version.
    pub fn new_20250429(name: impl Into<String>) -> Self {
        Self::new(name, TextEditorToolVersion::TextEditor20250429)
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
    pub fn with_version(mut self, version: TextEditorToolVersion) -> Self {
        self.tool_type = version;
        self
    }
}

/// Text editor tool (version 20250728 with max_characters support).
///
/// This is a newer version of the text editor tool that supports the max_characters
/// parameter to limit the size of files that can be edited.
///
/// # Example
///
/// ```
/// use llm_kit_anthropic::prompt::tool::text_editor_tool::AnthropicTextEditorTool20250728;
///
/// let tool = AnthropicTextEditorTool20250728::new("editor")
///     .with_max_characters(10000);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicTextEditorTool20250728 {
    /// The name of the tool
    pub name: String,

    /// The type of tool (always "text_editor_20250728")
    #[serde(rename = "type")]
    pub tool_type: String,

    /// Optional maximum number of characters allowed in a file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_characters: Option<u32>,

    /// Optional cache control configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}

impl AnthropicTextEditorTool20250728 {
    /// Creates a new text editor tool (version 20250728).
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the tool
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::tool::text_editor_tool::AnthropicTextEditorTool20250728;
    ///
    /// let tool = AnthropicTextEditorTool20250728::new("editor");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            tool_type: "text_editor_20250728".to_string(),
            max_characters: None,
            cache_control: None,
        }
    }

    /// Sets the maximum number of characters allowed.
    ///
    /// # Arguments
    ///
    /// * `max_characters` - Maximum number of characters
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::tool::text_editor_tool::AnthropicTextEditorTool20250728;
    ///
    /// let tool = AnthropicTextEditorTool20250728::new("editor")
    ///     .with_max_characters(50000);
    /// ```
    pub fn with_max_characters(mut self, max_characters: u32) -> Self {
        self.max_characters = Some(max_characters);
        self
    }

    /// Removes the max_characters limit.
    pub fn without_max_characters(mut self) -> Self {
        self.max_characters = None;
        self
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    use serde_json::json;

    // Tests for AnthropicTextEditorTool
    #[test]
    fn test_new() {
        let tool =
            AnthropicTextEditorTool::new("editor", TextEditorToolVersion::TextEditor20250124);

        assert_eq!(tool.name, "editor");
        assert_eq!(tool.tool_type, TextEditorToolVersion::TextEditor20250124);
        assert!(tool.cache_control.is_none());
    }

    #[test]
    fn test_new_20241022() {
        let tool = AnthropicTextEditorTool::new_20241022("editor");
        assert_eq!(tool.tool_type, TextEditorToolVersion::TextEditor20241022);
    }

    #[test]
    fn test_new_20250124() {
        let tool = AnthropicTextEditorTool::new_20250124("editor");
        assert_eq!(tool.tool_type, TextEditorToolVersion::TextEditor20250124);
    }

    #[test]
    fn test_new_20250429() {
        let tool = AnthropicTextEditorTool::new_20250429("editor");
        assert_eq!(tool.tool_type, TextEditorToolVersion::TextEditor20250429);
    }

    #[test]
    fn test_with_cache_control() {
        let cache = AnthropicCacheControl::new(CacheTTL::OneHour);
        let tool =
            AnthropicTextEditorTool::new_20250124("editor").with_cache_control(cache.clone());

        assert_eq!(tool.cache_control, Some(cache));
    }

    #[test]
    fn test_without_cache_control() {
        let tool = AnthropicTextEditorTool::new_20250124("editor")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
            .without_cache_control();

        assert!(tool.cache_control.is_none());
    }

    #[test]
    fn test_with_name() {
        let tool = AnthropicTextEditorTool::new_20250124("old").with_name("new");

        assert_eq!(tool.name, "new");
    }

    #[test]
    fn test_with_version() {
        let tool = AnthropicTextEditorTool::new_20250124("editor")
            .with_version(TextEditorToolVersion::TextEditor20250429);

        assert_eq!(tool.tool_type, TextEditorToolVersion::TextEditor20250429);
    }

    #[test]
    fn test_serialize_20241022() {
        let tool = AnthropicTextEditorTool::new_20241022("editor");
        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "name": "editor",
                "type": "text_editor_20241022"
            })
        );
    }

    #[test]
    fn test_serialize_20250124() {
        let tool = AnthropicTextEditorTool::new_20250124("editor");
        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "name": "editor",
                "type": "text_editor_20250124"
            })
        );
    }

    #[test]
    fn test_serialize_20250429() {
        let tool = AnthropicTextEditorTool::new_20250429("editor");
        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "name": "editor",
                "type": "text_editor_20250429"
            })
        );
    }

    #[test]
    fn test_serialize_with_cache() {
        let tool = AnthropicTextEditorTool::new_20250124("editor")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "name": "editor",
                "type": "text_editor_20250124",
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
            "name": "my_editor",
            "type": "text_editor_20250124",
            "cache_control": {
                "type": "ephemeral",
                "ttl": "5m"
            }
        });

        let tool: AnthropicTextEditorTool = serde_json::from_value(json).unwrap();

        assert_eq!(tool.name, "my_editor");
        assert_eq!(tool.tool_type, TextEditorToolVersion::TextEditor20250124);
        assert_eq!(
            tool.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
        );
    }

    #[test]
    fn test_equality() {
        let tool1 = AnthropicTextEditorTool::new_20250124("editor");
        let tool2 = AnthropicTextEditorTool::new_20250124("editor");
        let tool3 = AnthropicTextEditorTool::new_20250124("different");

        assert_eq!(tool1, tool2);
        assert_ne!(tool1, tool3);
    }

    #[test]
    fn test_clone() {
        let tool1 = AnthropicTextEditorTool::new_20250124("editor")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let tool2 = tool1.clone();

        assert_eq!(tool1, tool2);
    }

    #[test]
    fn test_roundtrip() {
        let original = AnthropicTextEditorTool::new_20250124("editor")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicTextEditorTool = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    // Tests for AnthropicTextEditorTool20250728
    #[test]
    fn test_20250728_new() {
        let tool = AnthropicTextEditorTool20250728::new("editor");

        assert_eq!(tool.name, "editor");
        assert_eq!(tool.tool_type, "text_editor_20250728");
        assert!(tool.max_characters.is_none());
        assert!(tool.cache_control.is_none());
    }

    #[test]
    fn test_20250728_with_max_characters() {
        let tool = AnthropicTextEditorTool20250728::new("editor").with_max_characters(10000);

        assert_eq!(tool.max_characters, Some(10000));
    }

    #[test]
    fn test_20250728_without_max_characters() {
        let tool = AnthropicTextEditorTool20250728::new("editor")
            .with_max_characters(10000)
            .without_max_characters();

        assert!(tool.max_characters.is_none());
    }

    #[test]
    fn test_20250728_with_cache_control() {
        let cache = AnthropicCacheControl::new(CacheTTL::OneHour);
        let tool = AnthropicTextEditorTool20250728::new("editor").with_cache_control(cache.clone());

        assert_eq!(tool.cache_control, Some(cache));
    }

    #[test]
    fn test_20250728_without_cache_control() {
        let tool = AnthropicTextEditorTool20250728::new("editor")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
            .without_cache_control();

        assert!(tool.cache_control.is_none());
    }

    #[test]
    fn test_20250728_with_name() {
        let tool = AnthropicTextEditorTool20250728::new("old").with_name("new");

        assert_eq!(tool.name, "new");
    }

    #[test]
    fn test_20250728_serialize_minimal() {
        let tool = AnthropicTextEditorTool20250728::new("editor");
        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "name": "editor",
                "type": "text_editor_20250728"
            })
        );
    }

    #[test]
    fn test_20250728_serialize_with_max_characters() {
        let tool = AnthropicTextEditorTool20250728::new("editor").with_max_characters(50000);
        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "name": "editor",
                "type": "text_editor_20250728",
                "max_characters": 50000
            })
        );
    }

    #[test]
    fn test_20250728_serialize_full() {
        let tool = AnthropicTextEditorTool20250728::new("editor")
            .with_max_characters(100000)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "name": "editor",
                "type": "text_editor_20250728",
                "max_characters": 100000,
                "cache_control": {
                    "type": "ephemeral",
                    "ttl": "1h"
                }
            })
        );
    }

    #[test]
    fn test_20250728_deserialize() {
        let json = json!({
            "name": "my_editor",
            "type": "text_editor_20250728",
            "max_characters": 75000,
            "cache_control": {
                "type": "ephemeral",
                "ttl": "5m"
            }
        });

        let tool: AnthropicTextEditorTool20250728 = serde_json::from_value(json).unwrap();

        assert_eq!(tool.name, "my_editor");
        assert_eq!(tool.max_characters, Some(75000));
        assert_eq!(
            tool.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
        );
    }

    #[test]
    fn test_20250728_equality() {
        let tool1 = AnthropicTextEditorTool20250728::new("editor");
        let tool2 = AnthropicTextEditorTool20250728::new("editor");
        let tool3 = AnthropicTextEditorTool20250728::new("different");

        assert_eq!(tool1, tool2);
        assert_ne!(tool1, tool3);
    }

    #[test]
    fn test_20250728_clone() {
        let tool1 = AnthropicTextEditorTool20250728::new("editor")
            .with_max_characters(10000)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let tool2 = tool1.clone();

        assert_eq!(tool1, tool2);
    }

    #[test]
    fn test_20250728_roundtrip() {
        let original = AnthropicTextEditorTool20250728::new("editor")
            .with_max_characters(50000)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicTextEditorTool20250728 = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }
}
