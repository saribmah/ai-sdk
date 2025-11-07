//! Tool call parsing and validation.
//!
//! This module provides functionality to parse and validate tool calls from LLM providers.
//! It includes JSON Schema validation to ensure tool inputs conform to their defined schemas,
//! providing similar error reporting to the TypeScript AI SDK.
//!
//! # Features
//!
//! - Parses JSON tool call inputs from provider responses
//! - Validates tool inputs against JSON Schemas using the `jsonschema` crate
//! - Supports both function tools and dynamic tools
//! - Provides detailed error messages for validation failures
//! - Handles provider-executed dynamic tools
//!
//! # Schema Validation
//!
//! Tool inputs are validated against JSON Schemas (Draft 7) that define:
//! - Required and optional properties
//! - Type constraints (string, number, object, array, etc.)
//! - Format constraints (email, date, etc.)
//! - Value constraints (min/max, enum, pattern, etc.)
//! - Nested object and array validation
//!
//! When validation fails, detailed error messages indicate which constraints were violated.

use super::{Tool, ToolCall};
use crate::error::AISDKError;
use ai_sdk_provider::language_model::content::tool_call::LanguageModelToolCall;
use serde_json::Value;
use std::collections::HashMap;

/// Validates tool input against a JSON Schema.
///
/// # Arguments
///
/// * `tool_name` - The name of the tool (for error messages)
/// * `input` - The parsed input to validate
/// * `schema` - The JSON Schema to validate against
///
/// # Returns
///
/// Returns `Ok(())` if validation succeeds.
///
/// # Errors
///
/// Returns an error if:
/// - The schema itself is invalid
/// - The input does not conform to the schema
fn validate_tool_input(tool_name: &str, input: &Value, schema: &Value) -> Result<(), AISDKError> {
    // Compile the JSON Schema
    let compiled_schema = jsonschema::validator_for(schema).map_err(|e| {
        AISDKError::invalid_tool_input(
            tool_name,
            input.to_string(),
            format!("Invalid tool schema: {}", e),
        )
    })?;

    // Validate the input against the schema
    // The is_valid() method is faster for just checking validation
    if !compiled_schema.is_valid(input) {
        // If invalid, collect detailed error messages
        let error_messages: Vec<String> = compiled_schema
            .iter_errors(input)
            .map(|e| format!("{} at {}", e, e.instance_path))
            .collect();

        return Err(AISDKError::invalid_tool_input(
            tool_name,
            input.to_string(),
            format!("Input does not match schema: {}", error_messages.join("; ")),
        ));
    }

    Ok(())
}

/// Parses a provider-executed dynamic tool call.
///
/// Provider-executed dynamic tools are tools that were executed by the provider
/// and are not part of the user's tool set. They have dynamic schemas.
///
/// # Arguments
///
/// * `tool_call` - The tool call from the provider to parse
///
/// # Returns
///
/// Returns a `ToolCall` with the parsed input.
///
/// # Errors
///
/// Returns an error if the input cannot be parsed as JSON.
pub fn parse_provider_executed_dynamic_tool_call(
    tool_call: &LanguageModelToolCall,
) -> Result<ToolCall, AISDKError> {
    // Empty input is treated as an empty object
    let input = if tool_call.input.trim().is_empty() {
        Value::Object(serde_json::Map::new())
    } else {
        serde_json::from_str(&tool_call.input).map_err(|e| {
            AISDKError::invalid_tool_input(
                &tool_call.tool_name,
                &tool_call.input,
                format!("Failed to parse JSON: {}", e),
            )
        })?
    };

    let mut call = ToolCall::new(&tool_call.tool_call_id, &tool_call.tool_name, input)
        .with_provider_executed(true);

    if let Some(metadata) = tool_call.provider_metadata.clone() {
        call = call.with_provider_metadata(metadata);
    }

    Ok(call)
}

/// Parses a tool call from the provider against a set of available tools.
///
/// This function:
/// 1. Looks up the tool in the tool set
/// 2. Parses the input as JSON
/// 3. Validates the input against the tool's JSON Schema
/// 4. Returns a tool call with the validated input
///
/// # Arguments
///
/// * `tool_call` - The tool call from the provider to parse
/// * `tools` - The set of available tools
///
/// # Returns
///
/// Returns a `ToolCall` with the parsed and validated input.
///
/// # Errors
///
/// Returns an error if:
/// - The tool is not found in the tool set
/// - The input cannot be parsed as JSON
/// - The input does not validate against the tool's JSON Schema
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::tool::parse_tool_call;
/// use ai_sdk_provider::language_model::content::tool_call::LanguageModelToolCall;
/// use std::collections::HashMap;
///
/// let tool_call = LanguageModelToolCall::new("call_123", "get_weather", r#"{"city": "SF"}"#);
/// let tools = HashMap::new();
/// let parsed = parse_tool_call(&tool_call, &tools)?;
/// ```
pub fn parse_tool_call(
    tool_call: &LanguageModelToolCall,
    tools: &HashMap<String, Tool>,
) -> Result<ToolCall, AISDKError> {
    let tool_name = &tool_call.tool_name;

    // Look up the tool in the tool set
    let tool = tools.get(tool_name);

    if tool.is_none() {
        // Provider-executed dynamic tools are not part of our list of tools
        if tool_call.provider_executed == Some(true) {
            return parse_provider_executed_dynamic_tool_call(tool_call);
        }

        return Err(AISDKError::no_such_tool(
            tool_name,
            tools.keys().cloned().collect(),
        ));
    }

    let tool = tool.unwrap();

    // Parse the input
    // Empty input is treated as an empty object (many LLMs generate empty strings for no-arg calls)
    let input = if tool_call.input.trim().is_empty() {
        Value::Object(serde_json::Map::new())
    } else {
        serde_json::from_str(&tool_call.input).map_err(|e| {
            AISDKError::invalid_tool_input(
                tool_name,
                &tool_call.input,
                format!("Failed to parse JSON: {}", e),
            )
        })?
    };

    // Validate the input against the tool's input schema
    validate_tool_input(tool_name, &input, &tool.input_schema)?;

    // Create the tool call
    let mut call = ToolCall::new(&tool_call.tool_call_id, tool_name, input);

    if let Some(executed) = tool_call.provider_executed {
        call = call.with_provider_executed(executed);
    }

    if let Some(metadata) = tool_call.provider_metadata.clone() {
        call = call.with_provider_metadata(metadata);
    }

    Ok(call)
}

#[cfg(test)]
mod tests {
    use super::super::Tool;
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_provider_executed_dynamic_tool_call_with_input() {
        let tool_call = LanguageModelToolCall::new("call_123", "dynamic_tool", r#"{"city": "SF"}"#);

        let result = parse_provider_executed_dynamic_tool_call(&tool_call);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.tool_call_id, "call_123");
        assert_eq!(parsed.tool_name, "dynamic_tool");
        assert_eq!(parsed.input, json!({"city": "SF"}));
        assert_eq!(parsed.provider_executed, Some(true));
    }

    #[test]
    fn test_parse_provider_executed_dynamic_tool_call_empty_input() {
        let tool_call = LanguageModelToolCall::new("call_123", "dynamic_tool", "");

        let result = parse_provider_executed_dynamic_tool_call(&tool_call);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.input, json!({}));
    }

    #[test]
    fn test_parse_provider_executed_dynamic_tool_call_invalid_json() {
        let tool_call = LanguageModelToolCall::new("call_123", "dynamic_tool", "invalid json");

        let result = parse_provider_executed_dynamic_tool_call(&tool_call);
        assert!(result.is_err());

        match result {
            Err(AISDKError::InvalidToolInput { tool_name, .. }) => {
                assert_eq!(tool_name, "dynamic_tool");
            }
            _ => panic!("Expected InvalidToolInput error"),
        }
    }

    #[test]
    fn test_parse_tool_call_function_tool() {
        let mut tools = HashMap::new();
        let schema = json!({
            "type": "object",
            "properties": {
                "city": {"type": "string"}
            }
        });
        tools.insert("get_weather".to_string(), Tool::function(schema));

        let tool_call = LanguageModelToolCall::new("call_123", "get_weather", r#"{"city": "SF"}"#);

        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.tool_call_id, "call_123");
        assert_eq!(parsed.tool_name, "get_weather");
        assert_eq!(parsed.input, json!({"city": "SF"}));
    }

    #[test]
    fn test_parse_tool_call_dynamic_tool() {
        let mut tools = HashMap::new();
        let schema = json!({
            "type": "object",
            "properties": {
                "param": {"type": "string"}
            }
        });
        tools.insert("dynamic_tool".to_string(), Tool::dynamic(schema));

        let tool_call =
            LanguageModelToolCall::new("call_456", "dynamic_tool", r#"{"param": "value"}"#);

        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.tool_call_id, "call_456");
        assert_eq!(parsed.tool_name, "dynamic_tool");
        assert_eq!(parsed.input, json!({"param": "value"}));
    }

    #[test]
    fn test_parse_tool_call_empty_input() {
        let mut tools = HashMap::new();
        let schema = json!({
            "type": "object"
        });
        tools.insert("no_args_tool".to_string(), Tool::function(schema));

        let tool_call = LanguageModelToolCall::new("call_789", "no_args_tool", "");

        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.input, json!({}));
    }

    #[test]
    fn test_parse_tool_call_tool_not_found() {
        let tools = HashMap::new();

        let tool_call =
            LanguageModelToolCall::new("call_123", "nonexistent_tool", r#"{"city": "SF"}"#);

        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_err());

        match result {
            Err(AISDKError::NoSuchTool { tool_name, .. }) => {
                assert_eq!(tool_name, "nonexistent_tool");
            }
            _ => panic!("Expected NoSuchTool error"),
        }
    }

    #[test]
    fn test_parse_tool_call_invalid_json() {
        let mut tools = HashMap::new();
        let schema = json!({
            "type": "object",
            "properties": {
                "city": {"type": "string"}
            }
        });
        tools.insert("get_weather".to_string(), Tool::function(schema));

        let tool_call = LanguageModelToolCall::new("call_123", "get_weather", "invalid json");

        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_err());

        match result {
            Err(AISDKError::InvalidToolInput { .. }) => {}
            _ => panic!("Expected InvalidToolInput error"),
        }
    }

    #[test]
    fn test_parse_tool_call_schema_validation_failure() {
        let mut tools = HashMap::new();
        let schema = json!({
            "type": "object",
            "properties": {
                "city": {"type": "string"}
            },
            "required": ["city"]
        });
        tools.insert("get_weather".to_string(), Tool::function(schema));

        // Missing required field
        let tool_call = LanguageModelToolCall::new("call_123", "get_weather", r#"{}"#);

        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_err());

        match result {
            Err(AISDKError::InvalidToolInput { tool_name, .. }) => {
                assert_eq!(tool_name, "get_weather");
            }
            _ => panic!("Expected InvalidToolInput error for schema validation failure"),
        }
    }

    #[test]
    fn test_parse_tool_call_with_provider_metadata() {
        use ai_sdk_provider::shared::provider_metadata::SharedProviderMetadata;

        let mut tools = HashMap::new();
        let schema = json!({"type": "object"});
        tools.insert("test_tool".to_string(), Tool::function(schema));

        let mut metadata = SharedProviderMetadata::new();
        let mut inner = HashMap::new();
        inner.insert("key".to_string(), json!("value"));
        metadata.insert("provider".to_string(), inner);

        let tool_call = LanguageModelToolCall::with_options(
            "call_123",
            "test_tool",
            "{}",
            None,
            Some(metadata.clone()),
        );

        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.provider_metadata, Some(metadata));
    }

    #[test]
    fn test_parse_provider_executed_tool_not_in_set() {
        let tools = HashMap::new();

        let tool_call = LanguageModelToolCall::with_options(
            "call_123",
            "external_tool",
            "{}",
            Some(true),
            None,
        );

        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.tool_name, "external_tool");
        assert_eq!(parsed.provider_executed, Some(true));
    }
}
