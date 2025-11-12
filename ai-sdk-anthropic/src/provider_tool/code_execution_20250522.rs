//! Anthropic code execution tool (version 20250522).
//!
//! This module provides a factory for creating code execution tools that allow
//! the model to execute Python code in a sandboxed environment.
//!
//! # Example
//!
//! ```
//! use ai_sdk_anthropic::provider_tool::code_execution_20250522;
//! use ai_sdk_provider_utils::ProviderDefinedToolOptions;
//!
//! // Create a code execution tool with default options
//! let tool = code_execution_20250522(None);
//!
//! // Create with custom options
//! let tool_with_approval = code_execution_20250522(Some(
//!     ProviderDefinedToolOptions::new()
//!         .with_description("Execute Python code")
//!         .with_needs_approval(true)
//! ));
//! ```

use ai_sdk_provider_utils::tool::{
    ProviderDefinedToolFactoryWithOutput, ProviderDefinedToolOptions, Tool,
};
use serde_json::json;

/// Creates a factory for the code_execution_20250522 tool.
fn create_code_execution_20250522_factory() -> ProviderDefinedToolFactoryWithOutput {
    ProviderDefinedToolFactoryWithOutput::new(
        "anthropic.code_execution_20250522",
        "code_execution",
        // Input schema
        json!({
            "type": "object",
            "properties": {
                "code": {
                    "type": "string",
                    "description": "The Python code to execute."
                }
            },
            "required": ["code"]
        }),
        // Output schema
        json!({
            "type": "object",
            "properties": {
                "type": {
                    "type": "string",
                    "const": "code_execution_result"
                },
                "stdout": {
                    "type": "string",
                    "description": "Standard output from the code execution"
                },
                "stderr": {
                    "type": "string",
                    "description": "Standard error from the code execution"
                },
                "return_code": {
                    "type": "number",
                    "description": "Exit code from the code execution"
                }
            },
            "required": ["type", "stdout", "stderr", "return_code"]
        }),
    )
}

/// Creates a code execution tool (version 20250522).
///
/// This tool allows the model to execute Python code in a sandboxed environment.
/// The tool returns structured output including stdout, stderr, and return code.
///
/// # Tool ID
/// `anthropic.code_execution_20250522`
///
/// # Input Schema
///
/// ```json
/// {
///   "type": "object",
///   "properties": {
///     "code": {
///       "type": "string",
///       "description": "The Python code to execute."
///     }
///   },
///   "required": ["code"]
/// }
/// ```
///
/// # Output Schema
///
/// ```json
/// {
///   "type": "object",
///   "properties": {
///     "type": {
///       "type": "string",
///       "const": "code_execution_result"
///     },
///     "stdout": {
///       "type": "string",
///       "description": "Standard output from the code execution"
///     },
///     "stderr": {
///       "type": "string",
///       "description": "Standard error from the code execution"
///     },
///     "return_code": {
///       "type": "number",
///       "description": "Exit code from the code execution"
///     }
///   },
///   "required": ["type", "stdout", "stderr", "return_code"]
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
/// use ai_sdk_anthropic::provider_tool::code_execution_20250522;
///
/// // Create with default options
/// let tool = code_execution_20250522(None);
/// ```
///
/// ## With Description
///
/// ```
/// use ai_sdk_anthropic::provider_tool::code_execution_20250522;
/// use ai_sdk_provider_utils::ProviderDefinedToolOptions;
///
/// let tool = code_execution_20250522(Some(
///     ProviderDefinedToolOptions::new()
///         .with_description("Execute Python code in a sandbox")
/// ));
/// ```
///
/// ## With Approval Requirement
///
/// ```
/// use ai_sdk_anthropic::provider_tool::code_execution_20250522;
/// use ai_sdk_provider_utils::ProviderDefinedToolOptions;
///
/// let tool = code_execution_20250522(Some(
///     ProviderDefinedToolOptions::new()
///         .with_description("Execute Python code (requires approval)")
///         .with_needs_approval(true)
/// ));
/// ```
pub fn code_execution_20250522(options: Option<ProviderDefinedToolOptions>) -> Tool {
    let factory = create_code_execution_20250522_factory();
    factory.create(options.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_sdk_provider_utils::ToolType;

    #[test]
    fn test_code_execution_20250522_default() {
        let tool = code_execution_20250522(None);

        assert!(tool.is_provider_defined());

        if let ToolType::ProviderDefined { id, name, args } = &tool.tool_type {
            assert_eq!(id, "anthropic.code_execution_20250522");
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
    fn test_code_execution_20250522_with_description() {
        let tool = code_execution_20250522(Some(
            ProviderDefinedToolOptions::new().with_description("Execute Python code"),
        ));

        assert_eq!(tool.description, Some("Execute Python code".to_string()));
    }

    #[test]
    fn test_code_execution_20250522_with_approval() {
        let tool = code_execution_20250522(Some(
            ProviderDefinedToolOptions::new().with_needs_approval(true),
        ));

        assert!(matches!(
            tool.needs_approval,
            ai_sdk_provider_utils::NeedsApproval::Yes
        ));
    }

    #[test]
    fn test_code_execution_20250522_input_schema() {
        let tool = code_execution_20250522(None);

        let schema = &tool.input_schema;
        assert_eq!(schema["type"], "object");

        let properties = &schema["properties"];
        assert!(properties.is_object());
        assert!(properties.get("code").is_some());

        let code = &properties["code"];
        assert_eq!(code["type"], "string");
        assert!(code.get("description").is_some());

        let required = &schema["required"];
        assert!(required.is_array());
        assert!(required.as_array().unwrap().contains(&json!("code")));
    }

    #[test]
    fn test_code_execution_20250522_output_schema() {
        let tool = code_execution_20250522(None);

        let output_schema = tool
            .output_schema
            .as_ref()
            .expect("Should have output schema");
        assert_eq!(output_schema["type"], "object");

        let properties = &output_schema["properties"];
        assert!(properties.is_object());

        // Check all required output properties
        assert!(properties.get("type").is_some());
        assert!(properties.get("stdout").is_some());
        assert!(properties.get("stderr").is_some());
        assert!(properties.get("return_code").is_some());

        // Verify type is a const
        let type_prop = &properties["type"];
        assert_eq!(type_prop["const"], "code_execution_result");

        // Verify required fields
        let required = &output_schema["required"];
        assert!(required.is_array());
        let required_array = required.as_array().unwrap();
        assert!(required_array.contains(&json!("type")));
        assert!(required_array.contains(&json!("stdout")));
        assert!(required_array.contains(&json!("stderr")));
        assert!(required_array.contains(&json!("return_code")));
    }

    #[test]
    fn test_code_execution_20250522_factory_reuse() {
        let factory = create_code_execution_20250522_factory();

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
    fn test_code_execution_20250522_clone() {
        let tool1 = code_execution_20250522(Some(
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
    async fn test_code_execution_20250522_with_execute() {
        use ai_sdk_provider_utils::{ToolExecuteOptions, ToolExecutionOutput};
        use std::sync::Arc;

        let tool = code_execution_20250522(Some(ProviderDefinedToolOptions::new().with_execute(
            Arc::new(|input, _opts| {
                ToolExecutionOutput::Single(Box::pin(async move {
                    Ok(json!({
                        "type": "code_execution_result",
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
                json!({"code": "print('hello')"}),
                options,
                None::<fn(serde_json::Value)>,
            )
            .await;

        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_ok());

        let output = result.unwrap();
        assert_eq!(output["type"], "code_execution_result");
        assert_eq!(output["return_code"], 0);
    }

    #[test]
    fn test_code_execution_20250522_with_args() {
        let tool = code_execution_20250522(Some(
            ProviderDefinedToolOptions::new()
                .with_arg("timeout", json!(30))
                .with_arg("max_output_size", json!(1024)),
        ));

        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            assert_eq!(args.get("timeout"), Some(&json!(30)));
            assert_eq!(args.get("max_output_size"), Some(&json!(1024)));
        } else {
            panic!("Expected ProviderDefined tool type");
        }
    }
}
