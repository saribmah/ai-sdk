//! Anthropic memory tool (version 20250818).
//!
//! This module provides a factory for creating memory tools that allow
//! the model to store and retrieve information across conversations.
//! Supports file operations like view, create, str_replace, insert, delete, and rename.
//!
//! # Example
//!
//! ```
//! use ai_sdk_anthropic::provider_tool::memory_20250818;
//!
//! // Create a memory tool with default options
//! let tool = memory_20250818(None);
//! ```

use ai_sdk_provider_utils::tool::{ProviderDefinedToolFactory, ProviderDefinedToolOptions, Tool};
use serde_json::json;

/// Creates a memory tool (version 20250818).
///
/// This tool allows the model to store and retrieve information in a persistent memory system.
/// It supports various file operations through a command-based interface.
///
/// # Tool ID
/// `anthropic.memory_20250818`
///
/// # Commands
///
/// - `view` - View the contents of a file (with optional line range)
/// - `create` - Create a new file with specified content
/// - `str_replace` - Replace a string in a file
/// - `insert` - Insert text at a specific line
/// - `delete` - Delete a file
/// - `rename` - Rename a file
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
/// use ai_sdk_anthropic::provider_tool::memory_20250818;
///
/// let tool = memory_20250818(None);
/// ```
///
/// ## With Description
///
/// ```
/// use ai_sdk_anthropic::provider_tool::memory_20250818;
/// use ai_sdk_provider_utils::ProviderDefinedToolOptions;
///
/// let tool = memory_20250818(Some(
///     ProviderDefinedToolOptions::new()
///         .with_description("Store and retrieve information")
/// ));
/// ```
///
/// ## With Approval Requirement
///
/// ```
/// use ai_sdk_anthropic::provider_tool::memory_20250818;
/// use ai_sdk_provider_utils::ProviderDefinedToolOptions;
///
/// let tool = memory_20250818(Some(
///     ProviderDefinedToolOptions::new()
///         .with_description("Memory operations (requires approval)")
///         .with_needs_approval(true)
/// ));
/// ```
pub fn memory_20250818(options: Option<ProviderDefinedToolOptions>) -> Tool {
    let factory = ProviderDefinedToolFactory::new(
        "anthropic.memory_20250818",
        "memory",
        json!({
            "oneOf": [
                {
                    "type": "object",
                    "properties": {
                        "command": {
                            "type": "string",
                            "const": "view"
                        },
                        "path": {
                            "type": "string",
                            "description": "The path to the file to view."
                        },
                        "view_range": {
                            "type": "array",
                            "items": {
                                "type": "number"
                            },
                            "minItems": 2,
                            "maxItems": 2,
                            "description": "Optional line range [start, end] to view."
                        }
                    },
                    "required": ["command", "path"]
                },
                {
                    "type": "object",
                    "properties": {
                        "command": {
                            "type": "string",
                            "const": "create"
                        },
                        "path": {
                            "type": "string",
                            "description": "The path to the file to create."
                        },
                        "file_text": {
                            "type": "string",
                            "description": "The content of the file to create."
                        }
                    },
                    "required": ["command", "path", "file_text"]
                },
                {
                    "type": "object",
                    "properties": {
                        "command": {
                            "type": "string",
                            "const": "str_replace"
                        },
                        "path": {
                            "type": "string",
                            "description": "The path to the file to edit."
                        },
                        "old_str": {
                            "type": "string",
                            "description": "The string to replace."
                        },
                        "new_str": {
                            "type": "string",
                            "description": "The new string to replace with."
                        }
                    },
                    "required": ["command", "path", "old_str", "new_str"]
                },
                {
                    "type": "object",
                    "properties": {
                        "command": {
                            "type": "string",
                            "const": "insert"
                        },
                        "path": {
                            "type": "string",
                            "description": "The path to the file to edit."
                        },
                        "insert_line": {
                            "type": "number",
                            "description": "The line number to insert at."
                        },
                        "insert_text": {
                            "type": "string",
                            "description": "The text to insert."
                        }
                    },
                    "required": ["command", "path", "insert_line", "insert_text"]
                },
                {
                    "type": "object",
                    "properties": {
                        "command": {
                            "type": "string",
                            "const": "delete"
                        },
                        "path": {
                            "type": "string",
                            "description": "The path to the file to delete."
                        }
                    },
                    "required": ["command", "path"]
                },
                {
                    "type": "object",
                    "properties": {
                        "command": {
                            "type": "string",
                            "const": "rename"
                        },
                        "old_path": {
                            "type": "string",
                            "description": "The current path of the file."
                        },
                        "new_path": {
                            "type": "string",
                            "description": "The new path for the file."
                        }
                    },
                    "required": ["command", "old_path", "new_path"]
                }
            ]
        }),
    );

    factory.create(options.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_sdk_provider_utils::ToolType;

    #[test]
    fn test_memory_20250818_default() {
        let tool = memory_20250818(None);

        assert!(tool.is_provider_defined());

        if let ToolType::ProviderDefined { id, name, args } = &tool.tool_type {
            assert_eq!(id, "anthropic.memory_20250818");
            assert_eq!(name, "memory");
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
    fn test_memory_20250818_with_description() {
        let tool = memory_20250818(Some(
            ProviderDefinedToolOptions::new().with_description("Store and retrieve information"),
        ));

        assert_eq!(
            tool.description,
            Some("Store and retrieve information".to_string())
        );
    }

    #[test]
    fn test_memory_20250818_with_approval() {
        let tool = memory_20250818(Some(
            ProviderDefinedToolOptions::new().with_needs_approval(true),
        ));

        assert!(matches!(
            tool.needs_approval,
            ai_sdk_provider_utils::NeedsApproval::Yes
        ));
    }

    #[test]
    fn test_memory_20250818_input_schema() {
        let tool = memory_20250818(None);

        let schema = &tool.input_schema;

        // Check that it has oneOf for discriminated union
        assert!(schema.get("oneOf").is_some());
        let one_of = schema["oneOf"].as_array().unwrap();

        // Should have 6 command variants
        assert_eq!(one_of.len(), 6);

        // Check each command variant
        assert_eq!(one_of[0]["properties"]["command"]["const"], "view");
        assert_eq!(one_of[1]["properties"]["command"]["const"], "create");
        assert_eq!(one_of[2]["properties"]["command"]["const"], "str_replace");
        assert_eq!(one_of[3]["properties"]["command"]["const"], "insert");
        assert_eq!(one_of[4]["properties"]["command"]["const"], "delete");
        assert_eq!(one_of[5]["properties"]["command"]["const"], "rename");
    }

    #[test]
    fn test_memory_20250818_view_command_schema() {
        let tool = memory_20250818(None);
        let schema = &tool.input_schema;
        let one_of = schema["oneOf"].as_array().unwrap();

        let view_cmd = &one_of[0];
        let properties = &view_cmd["properties"];

        assert_eq!(properties["command"]["const"], "view");
        assert!(properties.get("path").is_some());
        assert!(properties.get("view_range").is_some());

        let view_range = &properties["view_range"];
        assert_eq!(view_range["type"], "array");
        assert_eq!(view_range["minItems"], 2);
        assert_eq!(view_range["maxItems"], 2);

        let required = view_cmd["required"].as_array().unwrap();
        assert!(required.contains(&json!("command")));
        assert!(required.contains(&json!("path")));
        assert!(!required.contains(&json!("view_range"))); // Optional
    }

    #[test]
    fn test_memory_20250818_create_command_schema() {
        let tool = memory_20250818(None);
        let schema = &tool.input_schema;
        let one_of = schema["oneOf"].as_array().unwrap();

        let create_cmd = &one_of[1];
        let properties = &create_cmd["properties"];

        assert_eq!(properties["command"]["const"], "create");
        assert!(properties.get("path").is_some());
        assert!(properties.get("file_text").is_some());

        let required = create_cmd["required"].as_array().unwrap();
        assert!(required.contains(&json!("command")));
        assert!(required.contains(&json!("path")));
        assert!(required.contains(&json!("file_text")));
    }

    #[test]
    fn test_memory_20250818_str_replace_command_schema() {
        let tool = memory_20250818(None);
        let schema = &tool.input_schema;
        let one_of = schema["oneOf"].as_array().unwrap();

        let replace_cmd = &one_of[2];
        let properties = &replace_cmd["properties"];

        assert_eq!(properties["command"]["const"], "str_replace");
        assert!(properties.get("path").is_some());
        assert!(properties.get("old_str").is_some());
        assert!(properties.get("new_str").is_some());

        let required = replace_cmd["required"].as_array().unwrap();
        assert_eq!(required.len(), 4);
    }

    #[test]
    fn test_memory_20250818_insert_command_schema() {
        let tool = memory_20250818(None);
        let schema = &tool.input_schema;
        let one_of = schema["oneOf"].as_array().unwrap();

        let insert_cmd = &one_of[3];
        let properties = &insert_cmd["properties"];

        assert_eq!(properties["command"]["const"], "insert");
        assert!(properties.get("path").is_some());
        assert!(properties.get("insert_line").is_some());
        assert!(properties.get("insert_text").is_some());

        assert_eq!(properties["insert_line"]["type"], "number");
    }

    #[test]
    fn test_memory_20250818_delete_command_schema() {
        let tool = memory_20250818(None);
        let schema = &tool.input_schema;
        let one_of = schema["oneOf"].as_array().unwrap();

        let delete_cmd = &one_of[4];
        let properties = &delete_cmd["properties"];

        assert_eq!(properties["command"]["const"], "delete");
        assert!(properties.get("path").is_some());

        let required = delete_cmd["required"].as_array().unwrap();
        assert_eq!(required.len(), 2);
    }

    #[test]
    fn test_memory_20250818_rename_command_schema() {
        let tool = memory_20250818(None);
        let schema = &tool.input_schema;
        let one_of = schema["oneOf"].as_array().unwrap();

        let rename_cmd = &one_of[5];
        let properties = &rename_cmd["properties"];

        assert_eq!(properties["command"]["const"], "rename");
        assert!(properties.get("old_path").is_some());
        assert!(properties.get("new_path").is_some());

        let required = rename_cmd["required"].as_array().unwrap();
        assert!(required.contains(&json!("command")));
        assert!(required.contains(&json!("old_path")));
        assert!(required.contains(&json!("new_path")));
    }

    #[test]
    fn test_memory_20250818_clone() {
        let tool1 = memory_20250818(Some(
            ProviderDefinedToolOptions::new().with_description("Memory tool"),
        ));
        let tool2 = tool1.clone();

        assert_eq!(tool1.description, tool2.description);
        assert_eq!(tool1.input_schema, tool2.input_schema);

        if let (
            ToolType::ProviderDefined { id: id1, .. },
            ToolType::ProviderDefined { id: id2, .. },
        ) = (&tool1.tool_type, &tool2.tool_type)
        {
            assert_eq!(id1, id2);
        }
    }

    #[tokio::test]
    async fn test_memory_20250818_with_execute() {
        use ai_sdk_provider_utils::{ToolExecuteOptions, ToolExecutionOutput};
        use std::sync::Arc;

        let tool = memory_20250818(Some(ProviderDefinedToolOptions::new().with_execute(
            Arc::new(|input, _opts| {
                ToolExecutionOutput::Single(Box::pin(async move {
                    Ok(json!({
                        "command": input["command"],
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
    }

    #[test]
    fn test_memory_20250818_with_args() {
        let tool = memory_20250818(Some(
            ProviderDefinedToolOptions::new()
                .with_arg("max_file_size", json!(1048576))
                .with_arg("allowed_paths", json!(["/"])),
        ));

        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            assert_eq!(args.get("max_file_size"), Some(&json!(1048576)));
            assert_eq!(args.get("allowed_paths"), Some(&json!(vec!["/"])));
        } else {
            panic!("Expected ProviderDefined tool type");
        }
    }
}
