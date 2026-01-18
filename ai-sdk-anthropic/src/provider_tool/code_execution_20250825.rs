//! Anthropic code execution tool (version 20250825).
//!
//! This module provides a factory for creating code execution tools that support
//! both bash command execution and text editor operations in a sandboxed environment.
//!
//! # Example
//!
//! ```
//! use llm_kit_anthropic::provider_tool::code_execution_20250825;
//! use llm_kit_provider_utils::ProviderDefinedToolOptions;
//!
//! // Create a code execution tool with default options
//! let tool = code_execution_20250825(None);
//!
//! // Create with custom options
//! let tool_with_approval = code_execution_20250825(Some(
//!     ProviderDefinedToolOptions::new()
//!         .with_description("Execute code and edit files")
//!         .with_needs_approval(true)
//! ));
//! ```

use llm_kit_provider_utils::tool::{
    ProviderDefinedToolFactoryWithOutput, ProviderDefinedToolOptions, Tool,
};
use serde_json::json;

/// Creates a factory for the code_execution_20250825 tool.
fn create_code_execution_20250825_factory() -> ProviderDefinedToolFactoryWithOutput {
    ProviderDefinedToolFactoryWithOutput::new(
        "anthropic.code_execution_20250825",
        "code_execution",
        // Input schema - discriminated union of bash and text_editor commands
        json!({
            "oneOf": [
                {
                    "type": "object",
                    "properties": {
                        "type": {
                            "type": "string",
                            "const": "bash_code_execution"
                        },
                        "command": {
                            "type": "string",
                            "description": "Shell command to execute."
                        }
                    },
                    "required": ["type", "command"]
                },
                {
                    "type": "object",
                    "properties": {
                        "type": {
                            "type": "string",
                            "const": "text_editor_code_execution"
                        },
                        "command": {
                            "type": "string",
                            "const": "view"
                        },
                        "path": {
                            "type": "string",
                            "description": "The path to the file to view."
                        }
                    },
                    "required": ["type", "command", "path"]
                },
                {
                    "type": "object",
                    "properties": {
                        "type": {
                            "type": "string",
                            "const": "text_editor_code_execution"
                        },
                        "command": {
                            "type": "string",
                            "const": "create"
                        },
                        "path": {
                            "type": "string",
                            "description": "The path to the file to edit."
                        },
                        "file_text": {
                            "type": ["string", "null"],
                            "description": "The text of the file to edit."
                        }
                    },
                    "required": ["type", "command", "path"]
                },
                {
                    "type": "object",
                    "properties": {
                        "type": {
                            "type": "string",
                            "const": "text_editor_code_execution"
                        },
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
                            "description": "The new string to replace the old string with."
                        }
                    },
                    "required": ["type", "command", "path", "old_str", "new_str"]
                }
            ]
        }),
        // Output schema - discriminated union of result types
        json!({
            "oneOf": [
                {
                    "type": "object",
                    "properties": {
                        "type": {
                            "type": "string",
                            "const": "bash_code_execution_result"
                        },
                        "content": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "type": {
                                        "type": "string",
                                        "const": "bash_code_execution_output"
                                    },
                                    "file_id": {
                                        "type": "string"
                                    }
                                },
                                "required": ["type", "file_id"]
                            },
                            "description": "Output file Id list"
                        },
                        "stdout": {
                            "type": "string",
                            "description": "Output from successful execution"
                        },
                        "stderr": {
                            "type": "string",
                            "description": "Error messages if execution fails"
                        },
                        "return_code": {
                            "type": "number",
                            "description": "0 for success, non-zero for failure"
                        }
                    },
                    "required": ["type", "content", "stdout", "stderr", "return_code"]
                },
                {
                    "type": "object",
                    "properties": {
                        "type": {
                            "type": "string",
                            "const": "bash_code_execution_tool_result_error"
                        },
                        "error_code": {
                            "type": "string",
                            "description": "Available options: invalid_tool_input, unavailable, too_many_requests, execution_time_exceeded, output_file_too_large."
                        }
                    },
                    "required": ["type", "error_code"]
                },
                {
                    "type": "object",
                    "properties": {
                        "type": {
                            "type": "string",
                            "const": "text_editor_code_execution_tool_result_error"
                        },
                        "error_code": {
                            "type": "string",
                            "description": "Available options: invalid_tool_input, unavailable, too_many_requests, execution_time_exceeded, file_not_found."
                        }
                    },
                    "required": ["type", "error_code"]
                },
                {
                    "type": "object",
                    "properties": {
                        "type": {
                            "type": "string",
                            "const": "text_editor_code_execution_view_result"
                        },
                        "content": {
                            "type": "string"
                        },
                        "file_type": {
                            "type": "string",
                            "description": "The type of the file. Available options: text, image, pdf."
                        },
                        "num_lines": {
                            "type": ["number", "null"]
                        },
                        "start_line": {
                            "type": ["number", "null"]
                        },
                        "total_lines": {
                            "type": ["number", "null"]
                        }
                    },
                    "required": ["type", "content", "file_type", "num_lines", "start_line", "total_lines"]
                },
                {
                    "type": "object",
                    "properties": {
                        "type": {
                            "type": "string",
                            "const": "text_editor_code_execution_create_result"
                        },
                        "is_file_update": {
                            "type": "boolean"
                        }
                    },
                    "required": ["type", "is_file_update"]
                },
                {
                    "type": "object",
                    "properties": {
                        "type": {
                            "type": "string",
                            "const": "text_editor_code_execution_str_replace_result"
                        },
                        "lines": {
                            "type": ["array", "null"],
                            "items": {
                                "type": "string"
                            }
                        },
                        "new_lines": {
                            "type": ["number", "null"]
                        },
                        "new_start": {
                            "type": ["number", "null"]
                        },
                        "old_lines": {
                            "type": ["number", "null"]
                        },
                        "old_start": {
                            "type": ["number", "null"]
                        }
                    },
                    "required": ["type", "lines", "new_lines", "new_start", "old_lines", "old_start"]
                }
            ]
        }),
    )
}

/// Creates a code execution tool (version 20250825).
///
/// This tool supports both bash command execution and text editor operations
/// (view, create, str_replace) in a sandboxed environment.
///
/// # Tool ID
/// `anthropic.code_execution_20250825`
///
/// # Input Types
///
/// ## Bash Code Execution
/// ```json
/// {
///   "type": "bash_code_execution",
///   "command": "echo 'hello'"
/// }
/// ```
///
/// ## Text Editor - View
/// ```json
/// {
///   "type": "text_editor_code_execution",
///   "command": "view",
///   "path": "/path/to/file.txt"
/// }
/// ```
///
/// ## Text Editor - Create
/// ```json
/// {
///   "type": "text_editor_code_execution",
///   "command": "create",
///   "path": "/path/to/file.txt",
///   "file_text": "content"
/// }
/// ```
///
/// ## Text Editor - String Replace
/// ```json
/// {
///   "type": "text_editor_code_execution",
///   "command": "str_replace",
///   "path": "/path/to/file.txt",
///   "old_str": "old",
///   "new_str": "new"
/// }
/// ```
///
/// # Output Types
///
/// The tool can return various result types depending on the operation:
/// - `bash_code_execution_result`
/// - `bash_code_execution_tool_result_error`
/// - `text_editor_code_execution_tool_result_error`
/// - `text_editor_code_execution_view_result`
/// - `text_editor_code_execution_create_result`
/// - `text_editor_code_execution_str_replace_result`
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
/// use llm_kit_anthropic::provider_tool::code_execution_20250825;
///
/// let tool = code_execution_20250825(None);
/// ```
///
/// ## With Description
///
/// ```
/// use llm_kit_anthropic::provider_tool::code_execution_20250825;
/// use llm_kit_provider_utils::ProviderDefinedToolOptions;
///
/// let tool = code_execution_20250825(Some(
///     ProviderDefinedToolOptions::new()
///         .with_description("Execute code and edit files")
/// ));
/// ```
///
/// ## With Approval Requirement
///
/// ```
/// use llm_kit_anthropic::provider_tool::code_execution_20250825;
/// use llm_kit_provider_utils::ProviderDefinedToolOptions;
///
/// let tool = code_execution_20250825(Some(
///     ProviderDefinedToolOptions::new()
///         .with_description("Execute code (requires approval)")
///         .with_needs_approval(true)
/// ));
/// ```
pub fn code_execution_20250825(options: Option<ProviderDefinedToolOptions>) -> Tool {
    let factory = create_code_execution_20250825_factory();
    factory.create(options.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use llm_kit_provider_utils::ToolType;

    #[test]
    fn test_code_execution_20250825_default() {
        let tool = code_execution_20250825(None);

        assert!(tool.is_provider_defined());

        if let ToolType::ProviderDefined { id, name, args } = &tool.tool_type {
            assert_eq!(id, "anthropic.code_execution_20250825");
            assert_eq!(name, "code_execution");
            assert!(args.is_empty());
        } else {
            panic!("Expected ProviderDefined tool type");
        }

        assert!(!tool.input_schema.is_null());
        assert!(tool.output_schema.is_some());
        assert!(tool.description.is_none());
        assert!(tool.execute.is_none());
    }

    #[test]
    fn test_code_execution_20250825_with_description() {
        let tool = code_execution_20250825(Some(
            ProviderDefinedToolOptions::new().with_description("Execute code and edit files"),
        ));

        assert_eq!(
            tool.description,
            Some("Execute code and edit files".to_string())
        );
    }

    #[test]
    fn test_code_execution_20250825_with_approval() {
        let tool = code_execution_20250825(Some(
            ProviderDefinedToolOptions::new().with_needs_approval(true),
        ));

        assert!(matches!(
            tool.needs_approval,
            llm_kit_provider_utils::NeedsApproval::Yes
        ));
    }

    #[test]
    fn test_code_execution_20250825_input_schema() {
        let tool = code_execution_20250825(None);

        let schema = &tool.input_schema;

        // Check that it has oneOf for discriminated union
        assert!(schema.get("oneOf").is_some());
        let one_of = schema["oneOf"].as_array().unwrap();

        // Should have 4 variants: bash, view, create, str_replace
        assert_eq!(one_of.len(), 4);

        // Check bash variant
        let bash_variant = &one_of[0];
        assert_eq!(
            bash_variant["properties"]["type"]["const"],
            "bash_code_execution"
        );

        // Check text_editor view variant
        let view_variant = &one_of[1];
        assert_eq!(
            view_variant["properties"]["type"]["const"],
            "text_editor_code_execution"
        );
        assert_eq!(view_variant["properties"]["command"]["const"], "view");

        // Check text_editor create variant
        let create_variant = &one_of[2];
        assert_eq!(create_variant["properties"]["command"]["const"], "create");

        // Check text_editor str_replace variant
        let replace_variant = &one_of[3];
        assert_eq!(
            replace_variant["properties"]["command"]["const"],
            "str_replace"
        );
    }

    #[test]
    fn test_code_execution_20250825_output_schema() {
        let tool = code_execution_20250825(None);

        let output_schema = tool
            .output_schema
            .as_ref()
            .expect("Should have output schema");

        // Check that it has oneOf for discriminated union
        assert!(output_schema.get("oneOf").is_some());
        let one_of = output_schema["oneOf"].as_array().unwrap();

        // Should have 6 result variants
        assert_eq!(one_of.len(), 6);

        // Check result type constants
        assert_eq!(
            one_of[0]["properties"]["type"]["const"],
            "bash_code_execution_result"
        );
        assert_eq!(
            one_of[1]["properties"]["type"]["const"],
            "bash_code_execution_tool_result_error"
        );
        assert_eq!(
            one_of[2]["properties"]["type"]["const"],
            "text_editor_code_execution_tool_result_error"
        );
        assert_eq!(
            one_of[3]["properties"]["type"]["const"],
            "text_editor_code_execution_view_result"
        );
        assert_eq!(
            one_of[4]["properties"]["type"]["const"],
            "text_editor_code_execution_create_result"
        );
        assert_eq!(
            one_of[5]["properties"]["type"]["const"],
            "text_editor_code_execution_str_replace_result"
        );
    }

    #[test]
    fn test_code_execution_20250825_bash_result_schema() {
        let tool = code_execution_20250825(None);
        let output_schema = tool.output_schema.as_ref().unwrap();
        let one_of = output_schema["oneOf"].as_array().unwrap();

        let bash_result = &one_of[0];
        let properties = &bash_result["properties"];

        // Check all required properties
        assert!(properties.get("content").is_some());
        assert!(properties.get("stdout").is_some());
        assert!(properties.get("stderr").is_some());
        assert!(properties.get("return_code").is_some());

        // Verify content is an array
        assert_eq!(properties["content"]["type"], "array");
    }

    #[test]
    fn test_code_execution_20250825_factory_reuse() {
        let factory = create_code_execution_20250825_factory();

        let tool1 = factory.create(ProviderDefinedToolOptions::new().with_description("Tool 1"));
        let tool2 = factory.create(ProviderDefinedToolOptions::new().with_description("Tool 2"));

        assert_eq!(tool1.description, Some("Tool 1".to_string()));
        assert_eq!(tool2.description, Some("Tool 2".to_string()));

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

    #[test]
    fn test_code_execution_20250825_clone() {
        let tool1 = code_execution_20250825(Some(
            ProviderDefinedToolOptions::new().with_description("Code execution tool"),
        ));
        let tool2 = tool1.clone();

        assert_eq!(tool1.description, tool2.description);
        assert_eq!(tool1.input_schema, tool2.input_schema);
        assert_eq!(tool1.output_schema, tool2.output_schema);

        if let (
            ToolType::ProviderDefined { id: id1, .. },
            ToolType::ProviderDefined { id: id2, .. },
        ) = (&tool1.tool_type, &tool2.tool_type)
        {
            assert_eq!(id1, id2);
        }
    }

    #[tokio::test]
    async fn test_code_execution_20250825_with_execute() {
        use llm_kit_provider_utils::{ToolExecuteOptions, ToolExecutionOutput};
        use std::sync::Arc;

        let tool = code_execution_20250825(Some(ProviderDefinedToolOptions::new().with_execute(
            Arc::new(|input, _opts| {
                ToolExecutionOutput::Single(Box::pin(async move {
                    Ok(json!({
                        "type": "bash_code_execution_result",
                        "content": [],
                        "stdout": format!("Executed: {}", input),
                        "stderr": "",
                        "return_code": 0
                    }))
                }))
            }),
        )));

        assert!(tool.execute.is_some());

        let options = ToolExecuteOptions::new("test_call", vec![]);
        let result = tool
            .execute_tool(
                json!({"type": "bash_code_execution", "command": "echo hello"}),
                options,
                None::<fn(serde_json::Value)>,
            )
            .await;

        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_ok());

        let output = result.unwrap();
        assert_eq!(output["type"], "bash_code_execution_result");
        assert_eq!(output["return_code"], 0);
    }

    #[test]
    fn test_code_execution_20250825_with_args() {
        let tool = code_execution_20250825(Some(
            ProviderDefinedToolOptions::new()
                .with_arg("timeout", json!(60))
                .with_arg("max_output_size", json!(2048)),
        ));

        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            assert_eq!(args.get("timeout"), Some(&json!(60)));
            assert_eq!(args.get("max_output_size"), Some(&json!(2048)));
        } else {
            panic!("Expected ProviderDefined tool type");
        }
    }
}
