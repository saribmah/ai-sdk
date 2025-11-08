use crate::prompt::message::content_parts::tool_result::ToolResultOutput;
use crate::tool::definition::Tool;
use serde_json::Value;

/// Error mode for tool output.
///
/// Determines how tool execution errors should be formatted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorMode {
    /// No error mode - output is not an error
    None,
    /// Error output formatted as text
    Text,
    /// Error output formatted as JSON
    Json,
}

/// Creates a formatted tool model output from raw tool output.
///
/// This function takes the raw output from a tool execution and formats it
/// according to the error mode and tool configuration.
///
/// # Arguments
///
/// * `output` - The raw output value from the tool
/// * `tool` - Optional reference to the tool definition
/// * `error_mode` - How to format the output (none, text, or json error)
///
/// # Returns
///
/// A `ToolResultOutput` formatted according to the error mode and output type
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::prompt::create_tool_model_output::{create_tool_model_output, ErrorMode};
/// use serde_json::json;
///
/// // Regular output
/// let output = json!("Hello, world!");
/// let result = create_tool_model_output(output, None, ErrorMode::None);
/// // Returns ToolResultOutput::Text { value: "Hello, world!" }
///
/// // Error output
/// let error = json!({"error": "Something went wrong"});
/// let result = create_tool_model_output(error, None, ErrorMode::Text);
/// // Returns ToolResultOutput::ErrorText { value: "..." }
/// ```
pub fn create_tool_model_output(
    output: Value,
    tool: Option<&Tool>,
    error_mode: ErrorMode,
) -> ToolResultOutput {
    match error_mode {
        ErrorMode::Text => ToolResultOutput::ErrorText {
            value: get_error_message(&output),
            provider_options: None,
        },
        ErrorMode::Json => ToolResultOutput::ErrorJson {
            value: to_json_value(output),
            provider_options: None,
        },
        ErrorMode::None => {
            // Check if tool has a to_model_output method
            if let Some(tool_def) = tool
                && let Some(to_model_output_fn) = &tool_def.to_model_output
            {
                // Since the function expects OUTPUT type, we need to deserialize from Value
                // For now, we'll skip this and fall through to default behavior
                // This would require proper type handling in a real implementation
                let _ = to_model_output_fn;
            }

            // Default behavior: string -> text, otherwise -> json
            if let Some(string_value) = output.as_str() {
                ToolResultOutput::Text {
                    value: string_value.to_string(),
                    provider_options: None,
                }
            } else {
                ToolResultOutput::Json {
                    value: to_json_value(output),
                    provider_options: None,
                }
            }
        }
    }
}

/// Extracts an error message from a JSON value.
///
/// Tries to extract a meaningful error message by checking for common
/// error field names like "message" or "error", or converting to string.
fn get_error_message(value: &Value) -> String {
    // If it's already a string, return it
    if let Some(string) = value.as_str() {
        return string.to_string();
    }

    // If it's an object, try to get "message" or "error" field
    if let Some(obj) = value.as_object() {
        if let Some(msg) = obj.get("message")
            && let Some(msg_str) = msg.as_str()
        {
            return msg_str.to_string();
        }
        if let Some(err) = obj.get("error")
            && let Some(err_str) = err.as_str()
        {
            return err_str.to_string();
        }
    }

    // Otherwise, convert to JSON string
    serde_json::to_string(value).unwrap_or_else(|_| "Unknown error".to_string())
}

/// Converts a value to a JSON value.
fn to_json_value(value: Value) -> Value {
    // In Rust, we don't have undefined, so we just return the value as-is
    // Null values in serde_json::Value are already represented as Value::Null
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_tool_model_output_error_text() {
        let output = json!({"message": "Something went wrong"});
        let result = create_tool_model_output(output, None, ErrorMode::Text);

        match result {
            ToolResultOutput::ErrorText {
                value,
                provider_options,
            } => {
                assert_eq!(value, "Something went wrong");
                assert!(provider_options.is_none());
            }
            _ => panic!("Expected ErrorText variant"),
        }
    }

    #[test]
    fn test_create_tool_model_output_error_json() {
        let output = json!({"error": "failed"});
        let result = create_tool_model_output(output, None, ErrorMode::Json);

        match result {
            ToolResultOutput::ErrorJson {
                value,
                provider_options,
            } => {
                assert_eq!(value["error"], "failed");
                assert!(provider_options.is_none());
            }
            _ => panic!("Expected ErrorJson variant"),
        }
    }

    #[test]
    fn test_create_tool_model_output_text() {
        let output = json!("Hello, world!");
        let result = create_tool_model_output(output, None, ErrorMode::None);

        match result {
            ToolResultOutput::Text {
                value,
                provider_options,
            } => {
                assert_eq!(value, "Hello, world!");
                assert!(provider_options.is_none());
            }
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_create_tool_model_output_json() {
        let output = json!({"result": "success", "count": 42});
        let result = create_tool_model_output(output, None, ErrorMode::None);

        match result {
            ToolResultOutput::Json {
                value,
                provider_options,
            } => {
                assert_eq!(value["result"], "success");
                assert_eq!(value["count"], 42);
                assert!(provider_options.is_none());
            }
            _ => panic!("Expected Json variant"),
        }
    }

    #[test]
    fn test_get_error_message_from_string() {
        let value = json!("Error occurred");
        assert_eq!(get_error_message(&value), "Error occurred");
    }

    #[test]
    fn test_get_error_message_from_message_field() {
        let value = json!({"message": "Something failed"});
        assert_eq!(get_error_message(&value), "Something failed");
    }

    #[test]
    fn test_get_error_message_from_error_field() {
        let value = json!({"error": "Operation failed"});
        assert_eq!(get_error_message(&value), "Operation failed");
    }

    #[test]
    fn test_get_error_message_from_complex_object() {
        let value = json!({"code": 500, "details": "Internal error"});
        let message = get_error_message(&value);
        assert!(message.contains("500"));
        assert!(message.contains("Internal error"));
    }

    #[test]
    fn test_error_mode_equality() {
        assert_eq!(ErrorMode::None, ErrorMode::None);
        assert_eq!(ErrorMode::Text, ErrorMode::Text);
        assert_eq!(ErrorMode::Json, ErrorMode::Json);
        assert_ne!(ErrorMode::None, ErrorMode::Text);
    }

    #[test]
    fn test_create_tool_model_output_with_number() {
        let output = json!(42);
        let result = create_tool_model_output(output, None, ErrorMode::None);

        match result {
            ToolResultOutput::Json { value, .. } => {
                assert_eq!(value, 42);
            }
            _ => panic!("Expected Json variant"),
        }
    }

    #[test]
    fn test_create_tool_model_output_with_array() {
        let output = json!([1, 2, 3]);
        let result = create_tool_model_output(output, None, ErrorMode::None);

        match result {
            ToolResultOutput::Json { value, .. } => {
                assert!(value.is_array());
                assert_eq!(value.as_array().unwrap().len(), 3);
            }
            _ => panic!("Expected Json variant"),
        }
    }
}
