use serde::{Deserialize, Serialize};

use crate::prompt::message::cache_control::AnthropicCacheControl;

/// Computer use tool versions.
///
/// Represents the different versions of the computer use tool available.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComputerToolVersion {
    /// Version from October 2024
    #[serde(rename = "computer_20241022")]
    Computer20241022,

    /// Version from January 2025
    #[serde(rename = "computer_20250124")]
    Computer20250124,
}

/// Computer use tool.
///
/// This tool allows the model to interact with a computer desktop environment,
/// including mouse and keyboard control, screenshots, and more.
///
/// # Example
///
/// ```
/// use llm_kit_anthropic::prompt::tool::computer_tool::{AnthropicComputerTool, ComputerToolVersion};
///
/// let tool = AnthropicComputerTool::new(
///     "computer",
///     ComputerToolVersion::Computer20250124,
///     1920,
///     1080,
///     0
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicComputerTool {
    /// The name of the tool
    pub name: String,

    /// The version/type of the computer tool
    #[serde(rename = "type")]
    pub tool_type: ComputerToolVersion,

    /// Display width in pixels
    pub display_width_px: u32,

    /// Display height in pixels
    pub display_height_px: u32,

    /// Display number (for multi-monitor setups)
    pub display_number: u32,

    /// Optional cache control configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}

impl AnthropicComputerTool {
    /// Creates a new computer use tool.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the tool
    /// * `version` - The version of the computer tool
    /// * `display_width_px` - Display width in pixels
    /// * `display_height_px` - Display height in pixels
    /// * `display_number` - Display number (for multi-monitor setups)
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::tool::computer_tool::{AnthropicComputerTool, ComputerToolVersion};
    ///
    /// let tool = AnthropicComputerTool::new(
    ///     "computer",
    ///     ComputerToolVersion::Computer20250124,
    ///     1920,
    ///     1080,
    ///     0
    /// );
    /// ```
    pub fn new(
        name: impl Into<String>,
        tool_type: ComputerToolVersion,
        display_width_px: u32,
        display_height_px: u32,
        display_number: u32,
    ) -> Self {
        Self {
            name: name.into(),
            tool_type,
            display_width_px,
            display_height_px,
            display_number,
            cache_control: None,
        }
    }

    /// Creates a new computer use tool with the October 2024 version.
    pub fn new_20241022(
        name: impl Into<String>,
        display_width_px: u32,
        display_height_px: u32,
        display_number: u32,
    ) -> Self {
        Self::new(
            name,
            ComputerToolVersion::Computer20241022,
            display_width_px,
            display_height_px,
            display_number,
        )
    }

    /// Creates a new computer use tool with the January 2025 version.
    pub fn new_20250124(
        name: impl Into<String>,
        display_width_px: u32,
        display_height_px: u32,
        display_number: u32,
    ) -> Self {
        Self::new(
            name,
            ComputerToolVersion::Computer20250124,
            display_width_px,
            display_height_px,
            display_number,
        )
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
    pub fn with_version(mut self, version: ComputerToolVersion) -> Self {
        self.tool_type = version;
        self
    }

    /// Updates the display width.
    pub fn with_display_width(mut self, width: u32) -> Self {
        self.display_width_px = width;
        self
    }

    /// Updates the display height.
    pub fn with_display_height(mut self, height: u32) -> Self {
        self.display_height_px = height;
        self
    }

    /// Updates the display number.
    pub fn with_display_number(mut self, number: u32) -> Self {
        self.display_number = number;
        self
    }

    /// Updates display dimensions (width and height).
    pub fn with_display_dimensions(mut self, width: u32, height: u32) -> Self {
        self.display_width_px = width;
        self.display_height_px = height;
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
        let tool = AnthropicComputerTool::new(
            "computer",
            ComputerToolVersion::Computer20250124,
            1920,
            1080,
            0,
        );

        assert_eq!(tool.name, "computer");
        assert_eq!(tool.tool_type, ComputerToolVersion::Computer20250124);
        assert_eq!(tool.display_width_px, 1920);
        assert_eq!(tool.display_height_px, 1080);
        assert_eq!(tool.display_number, 0);
        assert!(tool.cache_control.is_none());
    }

    #[test]
    fn test_new_20241022() {
        let tool = AnthropicComputerTool::new_20241022("computer", 1920, 1080, 0);

        assert_eq!(tool.tool_type, ComputerToolVersion::Computer20241022);
    }

    #[test]
    fn test_new_20250124() {
        let tool = AnthropicComputerTool::new_20250124("computer", 1920, 1080, 0);

        assert_eq!(tool.tool_type, ComputerToolVersion::Computer20250124);
    }

    #[test]
    fn test_with_cache_control() {
        let cache = AnthropicCacheControl::new(CacheTTL::OneHour);
        let tool = AnthropicComputerTool::new_20250124("computer", 1920, 1080, 0)
            .with_cache_control(cache.clone());

        assert_eq!(tool.cache_control, Some(cache));
    }

    #[test]
    fn test_without_cache_control() {
        let tool = AnthropicComputerTool::new_20250124("computer", 1920, 1080, 0)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
            .without_cache_control();

        assert!(tool.cache_control.is_none());
    }

    #[test]
    fn test_with_name() {
        let tool = AnthropicComputerTool::new_20250124("old", 1920, 1080, 0).with_name("new");

        assert_eq!(tool.name, "new");
    }

    #[test]
    fn test_with_version() {
        let tool = AnthropicComputerTool::new_20250124("computer", 1920, 1080, 0)
            .with_version(ComputerToolVersion::Computer20241022);

        assert_eq!(tool.tool_type, ComputerToolVersion::Computer20241022);
    }

    #[test]
    fn test_with_display_width() {
        let tool =
            AnthropicComputerTool::new_20250124("computer", 1920, 1080, 0).with_display_width(2560);

        assert_eq!(tool.display_width_px, 2560);
    }

    #[test]
    fn test_with_display_height() {
        let tool = AnthropicComputerTool::new_20250124("computer", 1920, 1080, 0)
            .with_display_height(1440);

        assert_eq!(tool.display_height_px, 1440);
    }

    #[test]
    fn test_with_display_number() {
        let tool =
            AnthropicComputerTool::new_20250124("computer", 1920, 1080, 0).with_display_number(1);

        assert_eq!(tool.display_number, 1);
    }

    #[test]
    fn test_with_display_dimensions() {
        let tool = AnthropicComputerTool::new_20250124("computer", 1920, 1080, 0)
            .with_display_dimensions(3840, 2160);

        assert_eq!(tool.display_width_px, 3840);
        assert_eq!(tool.display_height_px, 2160);
    }

    #[test]
    fn test_serialize_20241022() {
        let tool = AnthropicComputerTool::new_20241022("computer", 1920, 1080, 0);
        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "name": "computer",
                "type": "computer_20241022",
                "display_width_px": 1920,
                "display_height_px": 1080,
                "display_number": 0
            })
        );
    }

    #[test]
    fn test_serialize_20250124() {
        let tool = AnthropicComputerTool::new_20250124("computer", 1920, 1080, 0);
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
    fn test_serialize_with_cache() {
        let tool = AnthropicComputerTool::new_20250124("computer", 1920, 1080, 0)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "name": "computer",
                "type": "computer_20250124",
                "display_width_px": 1920,
                "display_height_px": 1080,
                "display_number": 0,
                "cache_control": {
                    "type": "ephemeral",
                    "ttl": "1h"
                }
            })
        );
    }

    #[test]
    fn test_deserialize_20241022() {
        let json = json!({
            "name": "my_computer",
            "type": "computer_20241022",
            "display_width_px": 2560,
            "display_height_px": 1440,
            "display_number": 1,
            "cache_control": {
                "type": "ephemeral",
                "ttl": "5m"
            }
        });

        let tool: AnthropicComputerTool = serde_json::from_value(json).unwrap();

        assert_eq!(tool.name, "my_computer");
        assert_eq!(tool.tool_type, ComputerToolVersion::Computer20241022);
        assert_eq!(tool.display_width_px, 2560);
        assert_eq!(tool.display_height_px, 1440);
        assert_eq!(tool.display_number, 1);
        assert_eq!(
            tool.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
        );
    }

    #[test]
    fn test_deserialize_20250124() {
        let json = json!({
            "name": "computer",
            "type": "computer_20250124",
            "display_width_px": 1920,
            "display_height_px": 1080,
            "display_number": 0
        });

        let tool: AnthropicComputerTool = serde_json::from_value(json).unwrap();

        assert_eq!(tool.tool_type, ComputerToolVersion::Computer20250124);
    }

    #[test]
    fn test_equality() {
        let tool1 = AnthropicComputerTool::new_20250124("computer", 1920, 1080, 0);
        let tool2 = AnthropicComputerTool::new_20250124("computer", 1920, 1080, 0);
        let tool3 = AnthropicComputerTool::new_20250124("different", 1920, 1080, 0);

        assert_eq!(tool1, tool2);
        assert_ne!(tool1, tool3);
    }

    #[test]
    fn test_clone() {
        let tool1 = AnthropicComputerTool::new_20250124("computer", 1920, 1080, 0)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let tool2 = tool1.clone();

        assert_eq!(tool1, tool2);
    }

    #[test]
    fn test_builder_chaining() {
        let tool = AnthropicComputerTool::new_20241022("old", 800, 600, 0)
            .with_name("new")
            .with_version(ComputerToolVersion::Computer20250124)
            .with_display_dimensions(1920, 1080)
            .with_display_number(1)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
            .without_cache_control()
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        assert_eq!(tool.name, "new");
        assert_eq!(tool.tool_type, ComputerToolVersion::Computer20250124);
        assert_eq!(tool.display_width_px, 1920);
        assert_eq!(tool.display_height_px, 1080);
        assert_eq!(tool.display_number, 1);
        assert_eq!(
            tool.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::OneHour))
        );
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original = AnthropicComputerTool::new_20250124("computer", 1920, 1080, 0)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicComputerTool = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_multi_monitor() {
        let tool = AnthropicComputerTool::new_20250124("computer", 1920, 1080, 2);

        assert_eq!(tool.display_number, 2);
    }

    #[test]
    fn test_4k_display() {
        let tool = AnthropicComputerTool::new_20250124("computer", 3840, 2160, 0);

        assert_eq!(tool.display_width_px, 3840);
        assert_eq!(tool.display_height_px, 2160);
    }
}
