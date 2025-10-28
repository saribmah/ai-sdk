use crate::error::AISDKError;
use crate::message::tool::definition::Tool;
use ai_sdk_provider::language_model::tool_call::ToolCall;
use ai_sdk_provider::shared::provider_metadata::ProviderMetadata;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// A tool call that has been parsed and validated.
///
/// This represents a tool call with parsed input, unlike the provider's ToolCall
/// which contains unparsed input as a string.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedToolCall {
    /// Type is always "tool-call".
    #[serde(rename = "type")]
    pub call_type: String,

    /// The ID of the tool call.
    pub tool_call_id: String,

    /// The name of the tool that was called.
    pub tool_name: String,

    /// The parsed input to the tool.
    pub input: Value,

    /// Whether the tool was executed by the provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,

    /// Provider-specific metadata for this tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<ProviderMetadata>,

    /// Whether this is a dynamic tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,
}

impl ParsedToolCall {
    /// Creates a new parsed tool call.
    pub fn new(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        input: Value,
    ) -> Self {
        Self {
            call_type: "tool-call".to_string(),
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            input,
            provider_executed: None,
            provider_metadata: None,
            dynamic: None,
        }
    }

    /// Creates a parsed tool call with all options.
    pub fn with_options(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        input: Value,
        provider_executed: Option<bool>,
        provider_metadata: Option<ProviderMetadata>,
        dynamic: Option<bool>,
    ) -> Self {
        Self {
            call_type: "tool-call".to_string(),
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            input,
            provider_executed,
            provider_metadata,
            dynamic,
        }
    }
}

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
fn validate_tool_input(
    tool_name: &str,
    input: &Value,
    schema: &Value,
) -> Result<(), AISDKError> {
    // Compile the JSON Schema
    let compiled_schema = jsonschema::validator_for(schema)
        .map_err(|e| {
            AISDKError::invalid_tool_input(
                tool_name,
                &input.to_string(),
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
            &input.to_string(),
            format!(
                "Input does not match schema: {}",
                error_messages.join("; ")
            ),
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
/// Returns a `ParsedToolCall` with the parsed input and dynamic flag set to true.
///
/// # Errors
///
/// Returns an error if the input cannot be parsed as JSON.
pub fn parse_provider_executed_dynamic_tool_call(
    tool_call: &ToolCall,
) -> Result<ParsedToolCall, AISDKError> {
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

    Ok(ParsedToolCall::with_options(
        &tool_call.tool_call_id,
        &tool_call.tool_name,
        input,
        Some(true), // provider_executed
        tool_call.provider_metadata.clone(),
        Some(true), // dynamic
    ))
}

/// Parses a tool call from the provider against a set of available tools.
///
/// This function:
/// 1. Looks up the tool in the tool set
/// 2. Parses and validates the input against the tool's schema
/// 3. Returns a parsed tool call with the validated input
///
/// # Arguments
///
/// * `tool_call` - The tool call from the provider to parse
/// * `tools` - The set of available tools
///
/// # Returns
///
/// Returns a `ParsedToolCall` with the parsed and validated input.
///
/// # Errors
///
/// Returns an error if:
/// - The tool is not found in the tool set
/// - The input cannot be parsed as JSON
/// - The input does not validate against the tool's schema (future implementation)
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::generate_text::parse_tool_call::parse_tool_call;
/// use ai_sdk_provider::language_model::tool_call::ToolCall;
/// use std::collections::HashMap;
///
/// let tool_call = ToolCall::new("call_123", "get_weather", r#"{"city": "SF"}"#);
/// let tools = HashMap::new();
/// let parsed = parse_tool_call(&tool_call, &tools)?;
/// ```
pub fn parse_tool_call(
    tool_call: &ToolCall,
    tools: &HashMap<String, Tool<Value, Value>>,
) -> Result<ParsedToolCall, AISDKError> {
    let tool_name = &tool_call.tool_name;

    // Look up the tool in the tool set
    let tool = tools.get(tool_name);

    if tool.is_none() {
        // Provider-executed dynamic tools are not part of our list of tools
        if tool_call.provider_executed == Some(true) {
            // Check if there's a 'dynamic' field or assume it's dynamic for provider-executed tools
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

    // Return different structures based on whether the tool is dynamic
    let is_dynamic = tool.is_dynamic();

    Ok(ParsedToolCall::with_options(
        &tool_call.tool_call_id,
        tool_name,
        input,
        tool_call.provider_executed,
        tool_call.provider_metadata.clone(),
        if is_dynamic { Some(true) } else { None },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::tool::definition::Tool;
    use serde_json::json;

    #[test]
    fn test_parsed_tool_call_new() {
        let parsed = ParsedToolCall::new("call_123", "tool_name", json!({"key": "value"}));

        assert_eq!(parsed.call_type, "tool-call");
        assert_eq!(parsed.tool_call_id, "call_123");
        assert_eq!(parsed.tool_name, "tool_name");
        assert_eq!(parsed.input, json!({"key": "value"}));
        assert_eq!(parsed.provider_executed, None);
        assert_eq!(parsed.provider_metadata, None);
        assert_eq!(parsed.dynamic, None);
    }

    #[test]
    fn test_parsed_tool_call_with_options() {
        let parsed = ParsedToolCall::with_options(
            "call_123",
            "tool_name",
            json!({"key": "value"}),
            Some(true),
            None,
            Some(true),
        );

        assert_eq!(parsed.provider_executed, Some(true));
        assert_eq!(parsed.dynamic, Some(true));
    }

    #[test]
    fn test_parse_provider_executed_dynamic_tool_call_with_input() {
        let tool_call = ToolCall::new("call_123", "dynamic_tool", r#"{"city": "SF"}"#);

        let result = parse_provider_executed_dynamic_tool_call(&tool_call);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.tool_call_id, "call_123");
        assert_eq!(parsed.tool_name, "dynamic_tool");
        assert_eq!(parsed.input, json!({"city": "SF"}));
        assert_eq!(parsed.provider_executed, Some(true));
        assert_eq!(parsed.dynamic, Some(true));
    }

    #[test]
    fn test_parse_provider_executed_dynamic_tool_call_empty_input() {
        let tool_call = ToolCall::new("call_123", "dynamic_tool", "");

        let result = parse_provider_executed_dynamic_tool_call(&tool_call);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.input, json!({}));
    }

    #[test]
    fn test_parse_provider_executed_dynamic_tool_call_invalid_json() {
        let tool_call = ToolCall::new("call_123", "dynamic_tool", "invalid json");

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

        let tool_call = ToolCall::new("call_123", "get_weather", r#"{"city": "SF"}"#);

        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.tool_call_id, "call_123");
        assert_eq!(parsed.tool_name, "get_weather");
        assert_eq!(parsed.input, json!({"city": "SF"}));
        assert_eq!(parsed.dynamic, None); // Function tools are not dynamic
    }

    #[test]
    fn test_parse_tool_call_dynamic_tool() {
        let mut tools = HashMap::new();
        let schema = json!({"type": "object"});
        tools.insert("dynamic_tool".to_string(), Tool::dynamic(schema));

        let tool_call = ToolCall::new("call_123", "dynamic_tool", r#"{"data": "test"}"#);

        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.tool_name, "dynamic_tool");
        assert_eq!(parsed.input, json!({"data": "test"}));
        assert_eq!(parsed.dynamic, Some(true)); // Dynamic tools have dynamic flag
    }

    #[test]
    fn test_parse_tool_call_empty_input() {
        let mut tools = HashMap::new();
        let schema = json!({"type": "object"});
        tools.insert("no_args_tool".to_string(), Tool::function(schema));

        let tool_call = ToolCall::new("call_123", "no_args_tool", "");

        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.input, json!({})); // Empty string becomes empty object
    }

    #[test]
    fn test_parse_tool_call_tool_not_found() {
        let tools = HashMap::new();
        let tool_call = ToolCall::new("call_123", "unknown_tool", "{}");

        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_err());

        match result {
            Err(AISDKError::NoSuchTool { tool_name, .. }) => {
                assert_eq!(tool_name, "unknown_tool");
            }
            _ => panic!("Expected NoSuchTool error"),
        }
    }

    #[test]
    fn test_parse_tool_call_invalid_json() {
        let mut tools = HashMap::new();
        let schema = json!({"type": "object"});
        tools.insert("test_tool".to_string(), Tool::function(schema));

        let tool_call = ToolCall::new("call_123", "test_tool", "not valid json");

        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_err());

        match result {
            Err(AISDKError::InvalidToolInput {
                tool_name,
                tool_input,
                ..
            }) => {
                assert_eq!(tool_name, "test_tool");
                assert_eq!(tool_input, "not valid json");
            }
            _ => panic!("Expected InvalidToolInput error"),
        }
    }

    #[test]
    fn test_parse_tool_call_provider_executed_not_in_toolset() {
        let tools = HashMap::new();
        let tool_call = ToolCall::with_options(
            "call_123",
            "provider_tool",
            r#"{"key": "value"}"#,
            Some(true), // provider_executed
            None,
        );

        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.tool_name, "provider_tool");
        assert_eq!(parsed.provider_executed, Some(true));
        assert_eq!(parsed.dynamic, Some(true)); // Treated as dynamic
    }

    // Schema validation tests

    #[test]
    fn test_parse_tool_call_with_valid_schema() {
        let mut tools = HashMap::new();
        let schema = json!({
            "type": "object",
            "properties": {
                "city": {
                    "type": "string",
                    "description": "The city name"
                },
                "units": {
                    "type": "string",
                    "enum": ["celsius", "fahrenheit"]
                }
            },
            "required": ["city"]
        });
        tools.insert("get_weather".to_string(), Tool::function(schema));

        let tool_call = ToolCall::new(
            "call_123",
            "get_weather",
            r#"{"city": "San Francisco", "units": "celsius"}"#,
        );

        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.input["city"], "San Francisco");
        assert_eq!(parsed.input["units"], "celsius");
    }

    #[test]
    fn test_parse_tool_call_missing_required_field() {
        let mut tools = HashMap::new();
        let schema = json!({
            "type": "object",
            "properties": {
                "city": {
                    "type": "string"
                }
            },
            "required": ["city"]
        });
        tools.insert("get_weather".to_string(), Tool::function(schema));

        let tool_call = ToolCall::new("call_123", "get_weather", r#"{"units": "celsius"}"#);

        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_err());

        match result {
            Err(AISDKError::InvalidToolInput {
                tool_name,
                message,
                ..
            }) => {
                assert_eq!(tool_name, "get_weather");
                assert!(message.contains("does not match schema"));
                assert!(message.contains("required"));
            }
            _ => panic!("Expected InvalidToolInput error"),
        }
    }

    #[test]
    fn test_parse_tool_call_wrong_type() {
        let mut tools = HashMap::new();
        let schema = json!({
            "type": "object",
            "properties": {
                "count": {
                    "type": "number"
                }
            }
        });
        tools.insert("count_tool".to_string(), Tool::function(schema));

        let tool_call = ToolCall::new("call_123", "count_tool", r#"{"count": "not a number"}"#);

        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_err());

        match result {
            Err(AISDKError::InvalidToolInput {
                tool_name,
                message,
                ..
            }) => {
                assert_eq!(tool_name, "count_tool");
                assert!(message.contains("does not match schema"));
            }
            _ => panic!("Expected InvalidToolInput error"),
        }
    }

    #[test]
    fn test_parse_tool_call_invalid_enum_value() {
        let mut tools = HashMap::new();
        let schema = json!({
            "type": "object",
            "properties": {
                "status": {
                    "type": "string",
                    "enum": ["active", "inactive", "pending"]
                }
            }
        });
        tools.insert("update_status".to_string(), Tool::function(schema));

        let tool_call = ToolCall::new("call_123", "update_status", r#"{"status": "deleted"}"#);

        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_err());

        match result {
            Err(AISDKError::InvalidToolInput {
                tool_name,
                message,
                ..
            }) => {
                assert_eq!(tool_name, "update_status");
                assert!(message.contains("does not match schema"));
            }
            _ => panic!("Expected InvalidToolInput error"),
        }
    }

    #[test]
    fn test_parse_tool_call_additional_properties() {
        let mut tools = HashMap::new();
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string"
                }
            },
            "additionalProperties": false
        });
        tools.insert("strict_tool".to_string(), Tool::function(schema));

        let tool_call =
            ToolCall::new("call_123", "strict_tool", r#"{"name": "test", "extra": "field"}"#);

        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_err());

        match result {
            Err(AISDKError::InvalidToolInput {
                tool_name,
                message,
                ..
            }) => {
                assert_eq!(tool_name, "strict_tool");
                assert!(message.contains("does not match schema"));
            }
            _ => panic!("Expected InvalidToolInput error"),
        }
    }

    #[test]
    fn test_parse_tool_call_nested_object_validation() {
        let mut tools = HashMap::new();
        let schema = json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "object",
                    "properties": {
                        "city": {"type": "string"},
                        "country": {"type": "string"}
                    },
                    "required": ["city", "country"]
                }
            },
            "required": ["location"]
        });
        tools.insert("get_info".to_string(), Tool::function(schema));

        // Valid nested object
        let tool_call = ToolCall::new(
            "call_123",
            "get_info",
            r#"{"location": {"city": "Paris", "country": "France"}}"#,
        );
        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_ok());

        // Invalid nested object (missing required field)
        let tool_call = ToolCall::new("call_124", "get_info", r#"{"location": {"city": "Paris"}}"#);
        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_tool_call_array_validation() {
        let mut tools = HashMap::new();
        let schema = json!({
            "type": "object",
            "properties": {
                "tags": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    },
                    "minItems": 1,
                    "maxItems": 5
                }
            }
        });
        tools.insert("tag_tool".to_string(), Tool::function(schema));

        // Valid array
        let tool_call = ToolCall::new("call_123", "tag_tool", r#"{"tags": ["tag1", "tag2"]}"#);
        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_ok());

        // Invalid array (wrong item type)
        let tool_call = ToolCall::new("call_124", "tag_tool", r#"{"tags": ["tag1", 123]}"#);
        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_err());

        // Invalid array (too many items)
        let tool_call = ToolCall::new(
            "call_125",
            "tag_tool",
            r#"{"tags": ["tag1", "tag2", "tag3", "tag4", "tag5", "tag6"]}"#,
        );
        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_tool_call_string_constraints() {
        let mut tools = HashMap::new();
        let schema = json!({
            "type": "object",
            "properties": {
                "email": {
                    "type": "string",
                    "format": "email"
                },
                "username": {
                    "type": "string",
                    "minLength": 3,
                    "maxLength": 20
                }
            }
        });
        tools.insert("user_tool".to_string(), Tool::function(schema));

        // Valid input
        let tool_call = ToolCall::new(
            "call_123",
            "user_tool",
            r#"{"email": "user@example.com", "username": "john_doe"}"#,
        );
        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_ok());

        // Invalid username (too short)
        let tool_call = ToolCall::new(
            "call_124",
            "user_tool",
            r#"{"email": "user@example.com", "username": "ab"}"#,
        );
        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_tool_call_number_constraints() {
        let mut tools = HashMap::new();
        let schema = json!({
            "type": "object",
            "properties": {
                "age": {
                    "type": "integer",
                    "minimum": 0,
                    "maximum": 150
                },
                "temperature": {
                    "type": "number",
                    "minimum": -273.15
                }
            }
        });
        tools.insert("data_tool".to_string(), Tool::function(schema));

        // Valid input
        let tool_call =
            ToolCall::new("call_123", "data_tool", r#"{"age": 25, "temperature": 20.5}"#);
        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_ok());

        // Invalid age (negative)
        let tool_call =
            ToolCall::new("call_124", "data_tool", r#"{"age": -5, "temperature": 20.5}"#);
        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_err());

        // Invalid temperature (below absolute zero)
        let tool_call = ToolCall::new(
            "call_125",
            "data_tool",
            r#"{"age": 25, "temperature": -300}"#,
        );
        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_tool_call_empty_schema_allows_anything() {
        let mut tools = HashMap::new();
        // Empty schema (or true) allows any valid JSON
        let schema = json!({});
        tools.insert("flexible_tool".to_string(), Tool::function(schema));

        let tool_call = ToolCall::new(
            "call_123",
            "flexible_tool",
            r#"{"any": "value", "is": "allowed", "count": 42}"#,
        );
        let result = parse_tool_call(&tool_call, &tools);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_tool_input_function() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            },
            "required": ["name"]
        });

        // Valid input
        let input = json!({"name": "test"});
        let result = validate_tool_input("test_tool", &input, &schema);
        assert!(result.is_ok());

        // Invalid input (missing required field)
        let input = json!({"other": "field"});
        let result = validate_tool_input("test_tool", &input, &schema);
        assert!(result.is_err());

        match result {
            Err(AISDKError::InvalidToolInput { message, .. }) => {
                assert!(message.contains("does not match schema"));
            }
            _ => panic!("Expected InvalidToolInput error"),
        }
    }
}
