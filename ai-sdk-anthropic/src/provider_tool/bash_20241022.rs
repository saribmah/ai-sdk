//! Anthropic bash tool (version 20241022).
//!
//! This module provides a factory for creating bash execution tools that allow
//! the model to run bash commands in a sandboxed environment.
//!
//! # Example
//!
//! ```
//! use ai_sdk_anthropic::provider_tool::bash_20241022;
//! use ai_sdk_provider_utils::ProviderDefinedToolOptions;
//!
//! // Create a bash tool with default options
//! let tool = bash_20241022(None);
//!
//! // Create a bash tool with custom options
//! let tool_with_approval = bash_20241022(Some(
//!     ProviderDefinedToolOptions::new()
//!         .with_description("Execute bash commands")
//!         .with_needs_approval(true)
//! ));
//! ```

use ai_sdk_provider_utils::tool::{ProviderDefinedToolFactory, ProviderDefinedToolOptions, Tool};
use serde_json::json;

/// Creates a factory for the bash_20241022 tool.
///
/// This is an internal function that creates the factory with the proper
/// input schema. The factory can be used to create multiple tool instances.
fn create_bash_20241022_factory() -> ProviderDefinedToolFactory {
    ProviderDefinedToolFactory::new(
        "anthropic.bash_20241022",
        "bash",
        json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "The bash command to run. Required unless the tool is being restarted."
                },
                "restart": {
                    "type": "boolean",
                    "description": "Specifying true will restart this tool. Otherwise, leave this unspecified."
                }
            }
        }),
    )
}

/// Creates a bash execution tool (version 20241022).
///
/// This tool allows the model to execute bash commands in a sandboxed environment.
/// It supports running commands and restarting the tool session.
///
/// # Tool ID
/// `anthropic.bash_20241022`
///
/// # Input Schema
///
/// ```json
/// {
///   "type": "object",
///   "properties": {
///     "command": {
///       "type": "string",
///       "description": "The bash command to run. Required unless the tool is being restarted."
///     },
///     "restart": {
///       "type": "boolean",
///       "description": "Specifying true will restart this tool. Otherwise, leave this unspecified."
///     }
///   }
/// }
/// ```
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
/// use ai_sdk_anthropic::provider_tool::bash_20241022;
///
/// // Create with default options
/// let tool = bash_20241022(None);
/// ```
///
/// ## With Description
///
/// ```
/// use ai_sdk_anthropic::provider_tool::bash_20241022;
/// use ai_sdk_provider_utils::ProviderDefinedToolOptions;
///
/// let tool = bash_20241022(Some(
///     ProviderDefinedToolOptions::new()
///         .with_description("Execute bash commands in a sandbox")
/// ));
/// ```
///
/// ## With Approval Requirement
///
/// ```
/// use ai_sdk_anthropic::provider_tool::bash_20241022;
/// use ai_sdk_provider_utils::ProviderDefinedToolOptions;
///
/// let tool = bash_20241022(Some(
///     ProviderDefinedToolOptions::new()
///         .with_description("Execute bash commands (requires approval)")
///         .with_needs_approval(true)
/// ));
/// ```
///
/// ## With Custom Execute Function
///
/// ```no_run
/// use ai_sdk_anthropic::provider_tool::bash_20241022;
/// use ai_sdk_provider_utils::{ProviderDefinedToolOptions, ToolExecutionOutput};
/// use serde_json::json;
/// use std::sync::Arc;
///
/// let tool = bash_20241022(Some(
///     ProviderDefinedToolOptions::new()
///         .with_execute(Arc::new(|input, _opts| {
///             ToolExecutionOutput::Single(Box::pin(async move {
///                 // Custom execution logic
///                 let command = input.get("command")
///                     .and_then(|v| v.as_str())
///                     .unwrap_or("");
///                 
///                 // Execute command and return result
///                 Ok(json!({
///                     "output": "Command output",
///                     "exit_code": 0
///                 }))
///             }))
///         }))
/// ));
/// ```
pub fn bash_20241022(options: Option<ProviderDefinedToolOptions>) -> Tool {
    let factory = create_bash_20241022_factory();
    factory.create(options.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_sdk_provider_utils::ToolType;

    #[test]
    fn test_bash_20241022_default() {
        let tool = bash_20241022(None);

        // Check tool type
        assert!(tool.is_provider_defined());

        // Check tool ID and name
        if let ToolType::ProviderDefined { id, name, args } = &tool.tool_type {
            assert_eq!(id, "anthropic.bash_20241022");
            assert_eq!(name, "bash");
            assert!(args.is_empty());
        } else {
            panic!("Expected ProviderDefined tool type");
        }

        // Check schemas
        assert!(!tool.input_schema.is_null());
        assert!(tool.output_schema.is_none());

        // Check default options
        assert!(tool.description.is_none());
        assert!(tool.execute.is_none());
    }

    #[test]
    fn test_bash_20241022_with_description() {
        let tool = bash_20241022(Some(
            ProviderDefinedToolOptions::new().with_description("Execute bash commands"),
        ));

        assert_eq!(tool.description, Some("Execute bash commands".to_string()));
    }

    #[test]
    fn test_bash_20241022_with_approval() {
        let tool = bash_20241022(Some(
            ProviderDefinedToolOptions::new().with_needs_approval(true),
        ));

        assert!(matches!(
            tool.needs_approval,
            ai_sdk_provider_utils::NeedsApproval::Yes
        ));
    }

    #[test]
    fn test_bash_20241022_input_schema() {
        let tool = bash_20241022(None);

        // Verify the input schema structure
        let schema = &tool.input_schema;
        assert_eq!(schema["type"], "object");

        let properties = &schema["properties"];
        assert!(properties.is_object());
        assert!(properties.get("command").is_some());
        assert!(properties.get("restart").is_some());

        // Check command property
        let command = &properties["command"];
        assert_eq!(command["type"], "string");
        assert!(command.get("description").is_some());

        // Check restart property
        let restart = &properties["restart"];
        assert_eq!(restart["type"], "boolean");
        assert!(restart.get("description").is_some());
    }

    #[test]
    fn test_bash_20241022_factory_reuse() {
        let factory = create_bash_20241022_factory();

        // Create multiple tools from the same factory
        let tool1 = factory.create(ProviderDefinedToolOptions::new().with_description("Tool 1"));
        let tool2 = factory.create(ProviderDefinedToolOptions::new().with_description("Tool 2"));

        assert_eq!(tool1.description, Some("Tool 1".to_string()));
        assert_eq!(tool2.description, Some("Tool 2".to_string()));

        // Both should have the same ID and name
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
    fn test_bash_20241022_clone() {
        let tool1 = bash_20241022(Some(
            ProviderDefinedToolOptions::new().with_description("Bash tool"),
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
    async fn test_bash_20241022_with_execute() {
        use ai_sdk_provider_utils::{ToolExecuteOptions, ToolExecutionOutput};
        use std::sync::Arc;

        let tool = bash_20241022(Some(ProviderDefinedToolOptions::new().with_execute(
            Arc::new(|input, _opts| {
                ToolExecutionOutput::Single(Box::pin(async move {
                    Ok(json!({
                        "output": format!("Executed: {}", input),
                        "exit_code": 0
                    }))
                }))
            }),
        )));

        assert!(tool.execute.is_some());

        // Test execution
        let options = ToolExecuteOptions::new("test_call", vec![]);
        let result = tool
            .execute_tool(
                json!({"command": "echo hello"}),
                options,
                None::<fn(serde_json::Value)>,
            )
            .await;

        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_ok());
    }

    #[test]
    fn test_bash_20241022_with_args() {
        let tool = bash_20241022(Some(
            ProviderDefinedToolOptions::new()
                .with_arg("timeout", json!(30))
                .with_arg("shell", json!("/bin/bash")),
        ));

        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            assert_eq!(args.get("timeout"), Some(&json!(30)));
            assert_eq!(args.get("shell"), Some(&json!("/bin/bash")));
        } else {
            panic!("Expected ProviderDefined tool type");
        }
    }
}
