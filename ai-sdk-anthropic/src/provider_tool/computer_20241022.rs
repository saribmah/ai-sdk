//! Anthropic computer tool (version 20241022).
//!
//! This module provides a factory for creating computer control tools that allow
//! the model to interact with a computer's display, keyboard, and mouse.
//!
//! # Example
//!
//! ```
//! use llm_kit_anthropic::provider_tool::computer_20241022;
//! use llm_kit_provider_utils::ProviderDefinedToolOptions;
//! use serde_json::json;
//!
//! // Create a computer tool with display configuration
//! let tool = computer_20241022(1920, 1080, None);
//!
//! // Create with display number (for X11)
//! let tool_x11 = computer_20241022(1920, 1080, Some(0));
//! ```

use llm_kit_provider_utils::tool::{ProviderDefinedToolFactory, ProviderDefinedToolOptions, Tool};
use serde_json::json;

/// Creates a computer control tool (version 20241022).
///
/// This tool allows the model to interact with a computer's display, keyboard, and mouse.
/// It supports various actions including keyboard input, mouse control, and screenshots.
///
/// # Tool ID
/// `anthropic.computer_20241022`
///
/// # Actions
///
/// - `key` - Press a key or key-combination (supports xdotool syntax)
/// - `type` - Type a string of text
/// - `mouse_move` - Move cursor to coordinates
/// - `left_click` - Click left mouse button
/// - `left_click_drag` - Click and drag to coordinates
/// - `right_click` - Click right mouse button
/// - `middle_click` - Click middle mouse button
/// - `double_click` - Double-click left mouse button
/// - `screenshot` - Take a screenshot
/// - `cursor_position` - Get current cursor position
///
/// # Arguments
///
/// * `display_width_px` - The width of the display in pixels
/// * `display_height_px` - The height of the display in pixels
/// * `display_number` - Optional display number (for X11 environments)
/// * `options` - Optional additional configuration for the tool instance
///
/// # Returns
///
/// A configured `Tool` instance ready to be used with the Anthropic API.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```
/// use llm_kit_anthropic::provider_tool::computer_20241022;
///
/// let tool = computer_20241022(1920, 1080, None);
/// ```
///
/// ## With Display Number (X11)
///
/// ```
/// use llm_kit_anthropic::provider_tool::computer_20241022;
///
/// let tool = computer_20241022(1920, 1080, Some(0));
/// ```
///
/// ## With Custom Options
///
/// ```
/// use llm_kit_anthropic::provider_tool::computer_20241022;
/// use llm_kit_provider_utils::ProviderDefinedToolOptions;
///
/// let tool = computer_20241022(
///     1920,
///     1080,
///     None,
/// );
/// ```
pub fn computer_20241022(
    display_width_px: u32,
    display_height_px: u32,
    display_number: Option<u32>,
) -> Tool {
    let factory = ProviderDefinedToolFactory::new(
        "anthropic.computer_20241022",
        "computer",
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": [
                        "key",
                        "type",
                        "mouse_move",
                        "left_click",
                        "left_click_drag",
                        "right_click",
                        "middle_click",
                        "double_click",
                        "screenshot",
                        "cursor_position"
                    ],
                    "description": "The action to perform."
                },
                "coordinate": {
                    "type": "array",
                    "items": {
                        "type": "integer"
                    },
                    "description": "(x, y) coordinates. Required for mouse_move and left_click_drag."
                },
                "text": {
                    "type": "string",
                    "description": "Required for type and key actions."
                }
            },
            "required": ["action"]
        }),
    );

    let mut options = ProviderDefinedToolOptions::new()
        .with_arg("display_width_px", json!(display_width_px))
        .with_arg("display_height_px", json!(display_height_px));

    if let Some(number) = display_number {
        options = options.with_arg("display_number", json!(number));
    }

    factory.create(options)
}

#[cfg(test)]
mod tests {
    use super::*;
    use llm_kit_provider_utils::ToolType;

    #[test]
    fn test_computer_20241022_basic() {
        let tool = computer_20241022(1920, 1080, None);

        assert!(tool.is_provider_defined());

        if let ToolType::ProviderDefined { id, name, args } = &tool.tool_type {
            assert_eq!(id, "anthropic.computer_20241022");
            assert_eq!(name, "computer");
            assert_eq!(args.get("display_width_px"), Some(&json!(1920)));
            assert_eq!(args.get("display_height_px"), Some(&json!(1080)));
            assert!(args.get("display_number").is_none());
        } else {
            panic!("Expected ProviderDefined tool type");
        }

        assert!(!tool.input_schema.is_null());
        assert!(tool.output_schema.is_none());
    }

    #[test]
    fn test_computer_20241022_with_display_number() {
        let tool = computer_20241022(1920, 1080, Some(1));

        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            assert_eq!(args.get("display_width_px"), Some(&json!(1920)));
            assert_eq!(args.get("display_height_px"), Some(&json!(1080)));
            assert_eq!(args.get("display_number"), Some(&json!(1)));
        } else {
            panic!("Expected ProviderDefined tool type");
        }
    }

    #[test]
    fn test_computer_20241022_input_schema() {
        let tool = computer_20241022(1920, 1080, None);

        let schema = &tool.input_schema;
        assert_eq!(schema["type"], "object");

        let properties = &schema["properties"];
        assert!(properties.is_object());

        // Check action property
        let action = &properties["action"];
        assert_eq!(action["type"], "string");
        assert!(action.get("enum").is_some());

        let enum_values = action["enum"].as_array().unwrap();
        assert_eq!(enum_values.len(), 10);
        assert!(enum_values.contains(&json!("key")));
        assert!(enum_values.contains(&json!("type")));
        assert!(enum_values.contains(&json!("mouse_move")));
        assert!(enum_values.contains(&json!("screenshot")));

        // Check coordinate property
        let coordinate = &properties["coordinate"];
        assert_eq!(coordinate["type"], "array");
        assert_eq!(coordinate["items"]["type"], "integer");

        // Check text property
        let text = &properties["text"];
        assert_eq!(text["type"], "string");

        // Check required fields
        let required = &schema["required"];
        assert!(required.is_array());
        assert!(required.as_array().unwrap().contains(&json!("action")));
    }

    #[test]
    fn test_computer_20241022_different_resolutions() {
        let tool_hd = computer_20241022(1920, 1080, None);
        let tool_4k = computer_20241022(3840, 2160, None);
        let tool_mobile = computer_20241022(1080, 1920, None);

        if let ToolType::ProviderDefined { args, .. } = &tool_hd.tool_type {
            assert_eq!(args.get("display_width_px"), Some(&json!(1920)));
            assert_eq!(args.get("display_height_px"), Some(&json!(1080)));
        }

        if let ToolType::ProviderDefined { args, .. } = &tool_4k.tool_type {
            assert_eq!(args.get("display_width_px"), Some(&json!(3840)));
            assert_eq!(args.get("display_height_px"), Some(&json!(2160)));
        }

        if let ToolType::ProviderDefined { args, .. } = &tool_mobile.tool_type {
            assert_eq!(args.get("display_width_px"), Some(&json!(1080)));
            assert_eq!(args.get("display_height_px"), Some(&json!(1920)));
        }
    }

    #[test]
    fn test_computer_20241022_clone() {
        let tool1 = computer_20241022(1920, 1080, Some(0));
        let tool2 = tool1.clone();

        assert_eq!(tool1.input_schema, tool2.input_schema);

        if let (
            ToolType::ProviderDefined {
                id: id1,
                args: args1,
                ..
            },
            ToolType::ProviderDefined {
                id: id2,
                args: args2,
                ..
            },
        ) = (&tool1.tool_type, &tool2.tool_type)
        {
            assert_eq!(id1, id2);
            assert_eq!(args1, args2);
        }
    }

    #[test]
    fn test_computer_20241022_all_actions() {
        let tool = computer_20241022(1920, 1080, None);
        let schema = &tool.input_schema;
        let action_enum = schema["properties"]["action"]["enum"].as_array().unwrap();

        let expected_actions = vec![
            "key",
            "type",
            "mouse_move",
            "left_click",
            "left_click_drag",
            "right_click",
            "middle_click",
            "double_click",
            "screenshot",
            "cursor_position",
        ];

        for action in expected_actions {
            assert!(
                action_enum.contains(&json!(action)),
                "Missing action: {}",
                action
            );
        }
    }

    #[tokio::test]
    async fn test_computer_20241022_with_execute() {
        use llm_kit_provider_utils::{ToolExecuteOptions, ToolExecutionOutput};
        use std::sync::Arc;

        let factory = ProviderDefinedToolFactory::new(
            "anthropic.computer_20241022",
            "computer",
            json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["screenshot"]
                    }
                },
                "required": ["action"]
            }),
        );

        let tool = factory.create(
            ProviderDefinedToolOptions::new()
                .with_arg("display_width_px", json!(1920))
                .with_arg("display_height_px", json!(1080))
                .with_execute(Arc::new(|input, _opts| {
                    ToolExecutionOutput::Single(Box::pin(async move {
                        Ok(json!({
                            "action": input["action"],
                            "success": true
                        }))
                    }))
                })),
        );

        assert!(tool.execute.is_some());

        let options = ToolExecuteOptions::new("test_call", vec![]);
        let result = tool
            .execute_tool(
                json!({"action": "screenshot"}),
                options,
                None::<fn(serde_json::Value)>,
            )
            .await;

        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_ok());
    }

    #[test]
    fn test_computer_20241022_zero_display_number() {
        let tool = computer_20241022(1920, 1080, Some(0));

        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            assert_eq!(args.get("display_number"), Some(&json!(0)));
        }
    }
}
