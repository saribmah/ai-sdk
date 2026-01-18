//! Anthropic computer tool (version 20250124).
//!
//! This module provides a factory for creating computer control tools that allow
//! the model to interact with a computer's display, keyboard, and mouse.
//! This version includes additional actions like hold_key, triple_click, scroll, and wait.
//!
//! # Example
//!
//! ```
//! use llm_kit_anthropic::provider_tool::computer_20250124;
//!
//! // Create a computer tool with display configuration
//! let tool = computer_20250124(1920, 1080, None);
//!
//! // Create with display number (for X11)
//! let tool_x11 = computer_20250124(1920, 1080, Some(0));
//! ```

use llm_kit_provider_utils::tool::{ProviderDefinedToolFactory, ProviderDefinedToolOptions, Tool};
use serde_json::json;

/// Creates a computer control tool (version 20250124).
///
/// This tool allows the model to interact with a computer's display, keyboard, and mouse.
/// It supports various actions including keyboard input, mouse control, scrolling, and screenshots.
///
/// # Tool ID
/// `anthropic.computer_20250124`
///
/// # Actions
///
/// - `key` - Press a key or key-combination (supports xdotool syntax)
/// - `hold_key` - Hold down a key for a specified duration
/// - `type` - Type a string of text
/// - `cursor_position` - Get current cursor position
/// - `mouse_move` - Move cursor to coordinates
/// - `left_mouse_down` - Press left mouse button
/// - `left_mouse_up` - Release left mouse button
/// - `left_click` - Click left mouse button at coordinates
/// - `left_click_drag` - Click and drag from start to end coordinates
/// - `right_click` - Click right mouse button at coordinates
/// - `middle_click` - Click middle mouse button at coordinates
/// - `double_click` - Double-click left mouse button at coordinates
/// - `triple_click` - Triple-click left mouse button at coordinates
/// - `scroll` - Scroll in a direction by amount
/// - `wait` - Wait for a specified duration
/// - `screenshot` - Take a screenshot
///
/// # Arguments
///
/// * `display_width_px` - The width of the display in pixels
/// * `display_height_px` - The height of the display in pixels
/// * `display_number` - Optional display number (for X11 environments)
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
/// use llm_kit_anthropic::provider_tool::computer_20250124;
///
/// let tool = computer_20250124(1920, 1080, None);
/// ```
///
/// ## With Display Number (X11)
///
/// ```
/// use llm_kit_anthropic::provider_tool::computer_20250124;
///
/// let tool = computer_20250124(1920, 1080, Some(0));
/// ```
pub fn computer_20250124(
    display_width_px: u32,
    display_height_px: u32,
    display_number: Option<u32>,
) -> Tool {
    let factory = ProviderDefinedToolFactory::new(
        "anthropic.computer_20250124",
        "computer",
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": [
                        "key",
                        "hold_key",
                        "type",
                        "cursor_position",
                        "mouse_move",
                        "left_mouse_down",
                        "left_mouse_up",
                        "left_click",
                        "left_click_drag",
                        "right_click",
                        "middle_click",
                        "double_click",
                        "triple_click",
                        "scroll",
                        "wait",
                        "screenshot"
                    ],
                    "description": "The action to perform."
                },
                "coordinate": {
                    "type": "array",
                    "items": {
                        "type": "integer"
                    },
                    "minItems": 2,
                    "maxItems": 2,
                    "description": "(x, y) coordinates. Required for mouse_move, click actions, and scroll."
                },
                "duration": {
                    "type": "number",
                    "description": "Duration in seconds. Required for hold_key and wait actions."
                },
                "scroll_amount": {
                    "type": "number",
                    "description": "Number of scroll wheel clicks. Required for scroll action."
                },
                "scroll_direction": {
                    "type": "string",
                    "enum": ["up", "down", "left", "right"],
                    "description": "Scroll direction. Required for scroll action."
                },
                "start_coordinate": {
                    "type": "array",
                    "items": {
                        "type": "integer"
                    },
                    "minItems": 2,
                    "maxItems": 2,
                    "description": "Starting (x, y) coordinates. Required for left_click_drag."
                },
                "text": {
                    "type": "string",
                    "description": "Text input. Required for type, key, and hold_key actions. Can be used with clicks to hold keys."
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
    fn test_computer_20250124_basic() {
        let tool = computer_20250124(1920, 1080, None);

        assert!(tool.is_provider_defined());

        if let ToolType::ProviderDefined { id, name, args } = &tool.tool_type {
            assert_eq!(id, "anthropic.computer_20250124");
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
    fn test_computer_20250124_with_display_number() {
        let tool = computer_20250124(1920, 1080, Some(1));

        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            assert_eq!(args.get("display_width_px"), Some(&json!(1920)));
            assert_eq!(args.get("display_height_px"), Some(&json!(1080)));
            assert_eq!(args.get("display_number"), Some(&json!(1)));
        } else {
            panic!("Expected ProviderDefined tool type");
        }
    }

    #[test]
    fn test_computer_20250124_input_schema() {
        let tool = computer_20250124(1920, 1080, None);

        let schema = &tool.input_schema;
        assert_eq!(schema["type"], "object");

        let properties = &schema["properties"];
        assert!(properties.is_object());

        // Check action property
        let action = &properties["action"];
        assert_eq!(action["type"], "string");
        assert!(action.get("enum").is_some());

        let enum_values = action["enum"].as_array().unwrap();
        assert_eq!(enum_values.len(), 16);

        // Check coordinate property
        let coordinate = &properties["coordinate"];
        assert_eq!(coordinate["type"], "array");
        assert_eq!(coordinate["items"]["type"], "integer");
        assert_eq!(coordinate["minItems"], 2);
        assert_eq!(coordinate["maxItems"], 2);

        // Check duration property
        let duration = &properties["duration"];
        assert_eq!(duration["type"], "number");

        // Check scroll properties
        let scroll_amount = &properties["scroll_amount"];
        assert_eq!(scroll_amount["type"], "number");

        let scroll_direction = &properties["scroll_direction"];
        assert_eq!(scroll_direction["type"], "string");
        let scroll_enum = scroll_direction["enum"].as_array().unwrap();
        assert_eq!(scroll_enum.len(), 4);

        // Check start_coordinate property
        let start_coordinate = &properties["start_coordinate"];
        assert_eq!(start_coordinate["type"], "array");
        assert_eq!(start_coordinate["minItems"], 2);
        assert_eq!(start_coordinate["maxItems"], 2);

        // Check text property
        let text = &properties["text"];
        assert_eq!(text["type"], "string");

        // Check required fields
        let required = &schema["required"];
        assert!(required.is_array());
        assert!(required.as_array().unwrap().contains(&json!("action")));
    }

    #[test]
    fn test_computer_20250124_all_actions() {
        let tool = computer_20250124(1920, 1080, None);
        let schema = &tool.input_schema;
        let action_enum = schema["properties"]["action"]["enum"].as_array().unwrap();

        let expected_actions = vec![
            "key",
            "hold_key",
            "type",
            "cursor_position",
            "mouse_move",
            "left_mouse_down",
            "left_mouse_up",
            "left_click",
            "left_click_drag",
            "right_click",
            "middle_click",
            "double_click",
            "triple_click",
            "scroll",
            "wait",
            "screenshot",
        ];

        assert_eq!(action_enum.len(), expected_actions.len());

        for action in expected_actions {
            assert!(
                action_enum.contains(&json!(action)),
                "Missing action: {}",
                action
            );
        }
    }

    #[test]
    fn test_computer_20250124_new_actions() {
        let tool = computer_20250124(1920, 1080, None);
        let schema = &tool.input_schema;
        let action_enum = schema["properties"]["action"]["enum"].as_array().unwrap();

        // Check new actions compared to 20241022
        let new_actions = vec![
            "hold_key",
            "left_mouse_down",
            "left_mouse_up",
            "triple_click",
            "scroll",
            "wait",
        ];

        for action in new_actions {
            assert!(
                action_enum.contains(&json!(action)),
                "Missing new action: {}",
                action
            );
        }
    }

    #[test]
    fn test_computer_20250124_scroll_direction_enum() {
        let tool = computer_20250124(1920, 1080, None);
        let schema = &tool.input_schema;
        let scroll_direction = &schema["properties"]["scroll_direction"];
        let enum_values = scroll_direction["enum"].as_array().unwrap();

        assert_eq!(enum_values.len(), 4);
        assert!(enum_values.contains(&json!("up")));
        assert!(enum_values.contains(&json!("down")));
        assert!(enum_values.contains(&json!("left")));
        assert!(enum_values.contains(&json!("right")));
    }

    #[test]
    fn test_computer_20250124_different_resolutions() {
        let tool_hd = computer_20250124(1920, 1080, None);
        let tool_4k = computer_20250124(3840, 2160, None);

        if let ToolType::ProviderDefined { args, .. } = &tool_hd.tool_type {
            assert_eq!(args.get("display_width_px"), Some(&json!(1920)));
            assert_eq!(args.get("display_height_px"), Some(&json!(1080)));
        }

        if let ToolType::ProviderDefined { args, .. } = &tool_4k.tool_type {
            assert_eq!(args.get("display_width_px"), Some(&json!(3840)));
            assert_eq!(args.get("display_height_px"), Some(&json!(2160)));
        }
    }

    #[test]
    fn test_computer_20250124_clone() {
        let tool1 = computer_20250124(1920, 1080, Some(0));
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

    #[tokio::test]
    async fn test_computer_20250124_with_execute() {
        use llm_kit_provider_utils::{ToolExecuteOptions, ToolExecutionOutput};
        use std::sync::Arc;

        let factory = ProviderDefinedToolFactory::new(
            "anthropic.computer_20250124",
            "computer",
            json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["screenshot", "wait"]
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
}
