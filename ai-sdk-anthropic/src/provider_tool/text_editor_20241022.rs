//! Anthropic text editor tool (version 20241022).
//!
//! This module provides a factory for creating text editor tools that allow
//! the model to view and edit files with commands like view, create, str_replace,
//! insert, and undo_edit.
//!
//! # Example
//!
//! ```
//! use llm_kit_anthropic::provider_tool::text_editor_20241022;
//!
//! // Create a text editor tool with default options
//! let tool = text_editor_20241022(None);
//! ```

use llm_kit_provider_utils::tool::{ProviderDefinedToolFactory, ProviderDefinedToolOptions, Tool};
use serde_json::json;

/// Creates a text editor tool (version 20241022).
///
/// This tool allows the model to view and edit files using various commands.
/// The tool is named `str_replace_editor` in the Anthropic API.
///
/// # Tool ID
/// `anthropic.text_editor_20241022`
///
/// # Tool Name
/// `str_replace_editor`
///
/// # Commands
///
/// - `view` - View file or directory contents (with optional line range)
/// - `create` - Create a new file with specified content
/// - `str_replace` - Replace a string in a file with a new string
/// - `insert` - Insert text after a specific line
/// - `undo_edit` - Undo the last edit operation
///
/// # Arguments
///
/// * `options` - Optional configuration for the tool instance. If `None`, uses default options.
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
/// use llm_kit_anthropic::provider_tool::text_editor_20241022;
///
/// let tool = text_editor_20241022(None);
/// ```
///
/// ## With Description
///
/// ```
/// use llm_kit_anthropic::provider_tool::text_editor_20241022;
/// use llm_kit_provider_utils::ProviderDefinedToolOptions;
///
/// let tool = text_editor_20241022(Some(
///     ProviderDefinedToolOptions::new()
///         .with_description("Edit and view files")
/// ));
/// ```
///
/// ## With Approval Requirement
///
/// ```
/// use llm_kit_anthropic::provider_tool::text_editor_20241022;
/// use llm_kit_provider_utils::ProviderDefinedToolOptions;
///
/// let tool = text_editor_20241022(Some(
///     ProviderDefinedToolOptions::new()
///         .with_description("Edit files (requires approval)")
///         .with_needs_approval(true)
/// ));
/// ```
pub fn text_editor_20241022(options: Option<ProviderDefinedToolOptions>) -> Tool {
    let factory = ProviderDefinedToolFactory::new(
        "anthropic.text_editor_20241022",
        "str_replace_editor",
        json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "enum": ["view", "create", "str_replace", "insert", "undo_edit"],
                    "description": "The commands to run. Allowed options are: view, create, str_replace, insert, undo_edit."
                },
                "path": {
                    "type": "string",
                    "description": "Absolute path to file or directory, e.g. /repo/file.py or /repo."
                },
                "file_text": {
                    "type": "string",
                    "description": "Required parameter of create command, with the content of the file to be created."
                },
                "insert_line": {
                    "type": "integer",
                    "description": "Required parameter of insert command. The new_str will be inserted AFTER the line insert_line of path."
                },
                "new_str": {
                    "type": "string",
                    "description": "Optional parameter of str_replace command containing the new string (if not given, no string will be added). Required parameter of insert command containing the string to insert."
                },
                "old_str": {
                    "type": "string",
                    "description": "Required parameter of str_replace command containing the string in path to replace."
                },
                "view_range": {
                    "type": "array",
                    "items": {
                        "type": "integer"
                    },
                    "description": "Optional parameter of view command when path points to a file. If none is given, the full file is shown. If provided, the file will be shown in the indicated line number range, e.g. [11, 12] will show lines 11 and 12. Indexing at 1 to start. Setting [start_line, -1] shows all lines from start_line to the end of the file."
                }
            },
            "required": ["command", "path"]
        }),
    );

    factory.create(options.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use llm_kit_provider_utils::ToolType;

    #[test]
    fn test_text_editor_20241022_default() {
        let tool = text_editor_20241022(None);

        assert!(tool.is_provider_defined());

        if let ToolType::ProviderDefined { id, name, args } = &tool.tool_type {
            assert_eq!(id, "anthropic.text_editor_20241022");
            assert_eq!(name, "str_replace_editor");
            assert!(args.is_empty());
        } else {
            panic!("Expected ProviderDefined tool type");
        }

        assert!(!tool.input_schema.is_null());
        assert!(tool.output_schema.is_none());
        assert!(tool.description.is_none());
        assert!(tool.execute.is_none());
    }

    #[test]
    fn test_text_editor_20241022_with_description() {
        let tool = text_editor_20241022(Some(
            ProviderDefinedToolOptions::new().with_description("Edit and view files"),
        ));

        assert_eq!(tool.description, Some("Edit and view files".to_string()));
    }

    #[test]
    fn test_text_editor_20241022_with_approval() {
        let tool = text_editor_20241022(Some(
            ProviderDefinedToolOptions::new().with_needs_approval(true),
        ));

        assert!(matches!(
            tool.needs_approval,
            llm_kit_provider_utils::NeedsApproval::Yes
        ));
    }

    #[test]
    fn test_text_editor_20241022_input_schema() {
        let tool = text_editor_20241022(None);

        let schema = &tool.input_schema;
        assert_eq!(schema["type"], "object");

        let properties = &schema["properties"];
        assert!(properties.is_object());

        // Check command property
        let command = &properties["command"];
        assert_eq!(command["type"], "string");
        assert!(command.get("enum").is_some());

        let enum_values = command["enum"].as_array().unwrap();
        assert_eq!(enum_values.len(), 5);
        assert!(enum_values.contains(&json!("view")));
        assert!(enum_values.contains(&json!("create")));
        assert!(enum_values.contains(&json!("str_replace")));
        assert!(enum_values.contains(&json!("insert")));
        assert!(enum_values.contains(&json!("undo_edit")));

        // Check path property
        let path = &properties["path"];
        assert_eq!(path["type"], "string");

        // Check file_text property
        let file_text = &properties["file_text"];
        assert_eq!(file_text["type"], "string");

        // Check insert_line property
        let insert_line = &properties["insert_line"];
        assert_eq!(insert_line["type"], "integer");

        // Check new_str property
        let new_str = &properties["new_str"];
        assert_eq!(new_str["type"], "string");

        // Check old_str property
        let old_str = &properties["old_str"];
        assert_eq!(old_str["type"], "string");

        // Check view_range property
        let view_range = &properties["view_range"];
        assert_eq!(view_range["type"], "array");
        assert_eq!(view_range["items"]["type"], "integer");

        // Check required fields
        let required = &schema["required"];
        assert!(required.is_array());
        let required_array = required.as_array().unwrap();
        assert!(required_array.contains(&json!("command")));
        assert!(required_array.contains(&json!("path")));
        assert_eq!(required_array.len(), 2);
    }

    #[test]
    fn test_text_editor_20241022_all_commands() {
        let tool = text_editor_20241022(None);
        let schema = &tool.input_schema;
        let command_enum = schema["properties"]["command"]["enum"].as_array().unwrap();

        let expected_commands = vec!["view", "create", "str_replace", "insert", "undo_edit"];

        assert_eq!(command_enum.len(), expected_commands.len());

        for command in expected_commands {
            assert!(
                command_enum.contains(&json!(command)),
                "Missing command: {}",
                command
            );
        }
    }

    #[test]
    fn test_text_editor_20241022_clone() {
        let tool1 = text_editor_20241022(Some(
            ProviderDefinedToolOptions::new().with_description("Text editor tool"),
        ));
        let tool2 = tool1.clone();

        assert_eq!(tool1.description, tool2.description);
        assert_eq!(tool1.input_schema, tool2.input_schema);

        if let (
            ToolType::ProviderDefined {
                id: id1,
                name: name1,
                ..
            },
            ToolType::ProviderDefined {
                id: id2,
                name: name2,
                ..
            },
        ) = (&tool1.tool_type, &tool2.tool_type)
        {
            assert_eq!(id1, id2);
            assert_eq!(name1, name2);
        }
    }

    #[tokio::test]
    async fn test_text_editor_20241022_with_execute() {
        use llm_kit_provider_utils::{ToolExecuteOptions, ToolExecutionOutput};
        use std::sync::Arc;

        let tool = text_editor_20241022(Some(ProviderDefinedToolOptions::new().with_execute(
            Arc::new(|input, _opts| {
                ToolExecutionOutput::Single(Box::pin(async move {
                    Ok(json!({
                        "command": input["command"],
                        "path": input["path"],
                        "success": true
                    }))
                }))
            }),
        )));

        assert!(tool.execute.is_some());

        let options = ToolExecuteOptions::new("test_call", vec![]);
        let result = tool
            .execute_tool(
                json!({"command": "view", "path": "/test.txt"}),
                options,
                None::<fn(serde_json::Value)>,
            )
            .await;

        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_ok());

        let output = result.unwrap();
        assert_eq!(output["command"], "view");
        assert_eq!(output["path"], "/test.txt");
    }

    #[test]
    fn test_text_editor_20241022_with_args() {
        let tool = text_editor_20241022(Some(
            ProviderDefinedToolOptions::new()
                .with_arg("max_file_size", json!(1048576))
                .with_arg("allowed_extensions", json!(vec![".txt", ".py", ".rs"])),
        ));

        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            assert_eq!(args.get("max_file_size"), Some(&json!(1048576)));
            assert_eq!(
                args.get("allowed_extensions"),
                Some(&json!(vec![".txt", ".py", ".rs"]))
            );
        } else {
            panic!("Expected ProviderDefined tool type");
        }
    }

    #[test]
    fn test_text_editor_20241022_tool_name() {
        let tool = text_editor_20241022(None);

        if let ToolType::ProviderDefined { name, .. } = &tool.tool_type {
            assert_eq!(name, "str_replace_editor");
        }
    }
}
