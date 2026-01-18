//! Anthropic text editor tool (version 20250728).
//!
//! This module provides a factory for creating text editor tools that allow
//! the model to view and edit files with commands like view, create, str_replace,
//! and insert. Note: `undo_edit` is NOT supported in Claude 4 models (this version).
//!
//! This version adds support for the `maxCharacters` argument to control truncation
//! when viewing large files.
//!
//! # Example
//!
//! ```
//! use llm_kit_anthropic::provider_tool::text_editor_20250728;
//!
//! // Create a text editor tool with default options
//! let builder = text_editor_20250728();
//! let tool = builder.build();
//!
//! // Create with maxCharacters limit
//! let tool = text_editor_20250728()
//!     .max_characters(10000)
//!     .build();
//! ```

use llm_kit_provider_utils::tool::{ProviderDefinedToolFactory, ProviderDefinedToolOptions, Tool};
use serde_json::json;

/// Builder for creating a text editor tool (version 20250728).
///
/// This builder allows configuring the `maxCharacters` argument for controlling
/// truncation when viewing large files.
#[derive(Clone, Debug)]
pub struct TextEditor20250728Builder {
    max_characters: Option<i64>,
    description: Option<String>,
    needs_approval: bool,
}

impl TextEditor20250728Builder {
    /// Creates a new builder with default settings.
    fn new() -> Self {
        Self {
            max_characters: None,
            description: None,
            needs_approval: false,
        }
    }

    /// Sets the maximum number of characters to display when viewing files.
    ///
    /// This parameter controls truncation for large files. Only compatible with
    /// text_editor_20250728 and later versions.
    ///
    /// # Arguments
    ///
    /// * `max_characters` - Maximum number of characters to display
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::provider_tool::text_editor_20250728;
    ///
    /// let tool = text_editor_20250728()
    ///     .max_characters(10000)
    ///     .build();
    /// ```
    pub fn max_characters(mut self, max_characters: i64) -> Self {
        self.max_characters = Some(max_characters);
        self
    }

    /// Sets the description for the tool.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::provider_tool::text_editor_20250728;
    ///
    /// let tool = text_editor_20250728()
    ///     .with_description("Edit and view files with truncation support")
    ///     .build();
    /// ```
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets whether the tool requires approval before execution.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::provider_tool::text_editor_20250728;
    ///
    /// let tool = text_editor_20250728()
    ///     .with_needs_approval(true)
    ///     .build();
    /// ```
    pub fn with_needs_approval(mut self, needs_approval: bool) -> Self {
        self.needs_approval = needs_approval;
        self
    }

    /// Builds the configured tool.
    ///
    /// # Returns
    ///
    /// A configured `Tool` instance ready to be used with the Anthropic API.
    pub fn build(self) -> Tool {
        let factory = ProviderDefinedToolFactory::new(
            "anthropic.text_editor_20250728",
            "str_replace_based_edit_tool",
            json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "enum": ["view", "create", "str_replace", "insert"],
                        "description": "The commands to run. Allowed options are: view, create, str_replace, insert. Note: undo_edit is not supported in Claude 4 models."
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

        // Build options with configured settings
        let mut opts = ProviderDefinedToolOptions::new();

        if let Some(desc) = self.description {
            opts = opts.with_description(desc);
        }

        if self.needs_approval {
            opts = opts.with_needs_approval(true);
        }

        // Add maxCharacters to args if specified
        if let Some(max_chars) = self.max_characters {
            opts = opts.with_arg("maxCharacters", json!(max_chars));
        }

        factory.create(opts)
    }
}

/// Creates a builder for a text editor tool (version 20250728).
///
/// This tool allows the model to view and edit files using various commands.
/// The tool is named `str_replace_based_edit_tool` in the Anthropic API.
///
/// **Note:** This version does NOT support the `undo_edit` command, as it is
/// intended for Claude 4 models where undo_edit is not available.
///
/// This version adds support for the `maxCharacters` argument to control truncation
/// when viewing large files.
///
/// # Tool ID
/// `anthropic.text_editor_20250728`
///
/// # Tool Name
/// `str_replace_based_edit_tool`
///
/// # Commands
///
/// - `view` - View file or directory contents (with optional line range)
/// - `create` - Create a new file with specified content
/// - `str_replace` - Replace a string in a file with a new string
/// - `insert` - Insert text after a specific line
///
/// # Returns
///
/// A builder that can be configured before calling `.build()` to create the tool.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```
/// use llm_kit_anthropic::provider_tool::text_editor_20250728;
///
/// let tool = text_editor_20250728().build();
/// ```
///
/// ## With Max Characters
///
/// ```
/// use llm_kit_anthropic::provider_tool::text_editor_20250728;
///
/// let tool = text_editor_20250728()
///     .max_characters(10000)
///     .build();
/// ```
///
/// ## With Description and Approval
///
/// ```
/// use llm_kit_anthropic::provider_tool::text_editor_20250728;
///
/// let tool = text_editor_20250728()
///     .with_description("Edit files with truncation support")
///     .with_needs_approval(true)
///     .max_characters(5000)
///     .build();
/// ```
pub fn text_editor_20250728() -> TextEditor20250728Builder {
    TextEditor20250728Builder::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use llm_kit_provider_utils::ToolType;

    #[test]
    fn test_text_editor_20250728_default() {
        let tool = text_editor_20250728().build();

        assert!(tool.is_provider_defined());

        if let ToolType::ProviderDefined { id, name, args } = &tool.tool_type {
            assert_eq!(id, "anthropic.text_editor_20250728");
            assert_eq!(name, "str_replace_based_edit_tool");
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
    fn test_text_editor_20250728_with_max_characters() {
        let tool = text_editor_20250728().max_characters(10000).build();

        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            assert_eq!(args.get("maxCharacters"), Some(&json!(10000)));
        } else {
            panic!("Expected ProviderDefined tool type");
        }
    }

    #[test]
    fn test_text_editor_20250728_with_description() {
        let tool = text_editor_20250728()
            .with_description("Edit and view files")
            .build();

        assert_eq!(tool.description, Some("Edit and view files".to_string()));
    }

    #[test]
    fn test_text_editor_20250728_with_approval() {
        let tool = text_editor_20250728().with_needs_approval(true).build();

        assert!(matches!(
            tool.needs_approval,
            llm_kit_provider_utils::NeedsApproval::Yes
        ));
    }

    #[test]
    fn test_text_editor_20250728_input_schema() {
        let tool = text_editor_20250728().build();

        let schema = &tool.input_schema;
        assert_eq!(schema["type"], "object");

        let properties = &schema["properties"];
        assert!(properties.is_object());

        // Check command property
        let command = &properties["command"];
        assert_eq!(command["type"], "string");
        assert!(command.get("enum").is_some());

        let enum_values = command["enum"].as_array().unwrap();
        assert_eq!(enum_values.len(), 4);
        assert!(enum_values.contains(&json!("view")));
        assert!(enum_values.contains(&json!("create")));
        assert!(enum_values.contains(&json!("str_replace")));
        assert!(enum_values.contains(&json!("insert")));
        // undo_edit should NOT be present
        assert!(!enum_values.contains(&json!("undo_edit")));

        // Check path property
        let path = &properties["path"];
        assert_eq!(path["type"], "string");

        // Check required fields
        let required = &schema["required"];
        assert!(required.is_array());
        let required_array = required.as_array().unwrap();
        assert!(required_array.contains(&json!("command")));
        assert!(required_array.contains(&json!("path")));
        assert_eq!(required_array.len(), 2);
    }

    #[test]
    fn test_text_editor_20250728_all_commands() {
        let tool = text_editor_20250728().build();
        let schema = &tool.input_schema;
        let command_enum = schema["properties"]["command"]["enum"].as_array().unwrap();

        let expected_commands = vec!["view", "create", "str_replace", "insert"];

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
    fn test_text_editor_20250728_no_undo_edit() {
        let tool = text_editor_20250728().build();
        let schema = &tool.input_schema;
        let command_enum = schema["properties"]["command"]["enum"].as_array().unwrap();

        // Ensure undo_edit is NOT in the command list
        assert!(!command_enum.contains(&json!("undo_edit")));
    }

    #[test]
    fn test_text_editor_20250728_clone() {
        let tool1 = text_editor_20250728()
            .with_description("Text editor tool")
            .build();
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

    #[test]
    fn test_text_editor_20250728_builder_chaining() {
        let tool = text_editor_20250728()
            .max_characters(5000)
            .with_description("Text editor with truncation")
            .with_needs_approval(true)
            .build();

        // Check maxCharacters arg
        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            assert_eq!(args.get("maxCharacters"), Some(&json!(5000)));
        } else {
            panic!("Expected ProviderDefined tool type");
        }

        // Check description
        assert_eq!(
            tool.description,
            Some("Text editor with truncation".to_string())
        );

        // Check approval
        assert!(matches!(
            tool.needs_approval,
            llm_kit_provider_utils::NeedsApproval::Yes
        ));
    }

    #[test]
    fn test_text_editor_20250728_tool_name() {
        let tool = text_editor_20250728().build();

        if let ToolType::ProviderDefined { name, .. } = &tool.tool_type {
            assert_eq!(name, "str_replace_based_edit_tool");
        }
    }

    #[test]
    fn test_text_editor_20250728_builder_reuse() {
        let builder = text_editor_20250728().max_characters(8000);

        // Clone the builder and create two different tools
        let tool1 = builder.clone().with_description("Tool 1").build();
        let tool2 = builder.with_description("Tool 2").build();

        // Both should have maxCharacters
        if let ToolType::ProviderDefined { args, .. } = &tool1.tool_type {
            assert_eq!(args.get("maxCharacters"), Some(&json!(8000)));
        }
        if let ToolType::ProviderDefined { args, .. } = &tool2.tool_type {
            assert_eq!(args.get("maxCharacters"), Some(&json!(8000)));
        }

        // But different descriptions
        assert_eq!(tool1.description, Some("Tool 1".to_string()));
        assert_eq!(tool2.description, Some("Tool 2".to_string()));
    }
}
