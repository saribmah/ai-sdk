use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A tool execution error representing a failed tool invocation.
///
/// This contains information about the tool call that failed, the input
/// that was provided, and the error that occurred.
///
/// # Example
///
/// ```
/// use ai_sdk_provider_utils::tool::ToolError;
/// use serde_json::json;
///
/// let error = ToolError::new(
///     "call_123",
///     "get_weather",
///     json!({"city": "SF"}),
///     "Failed to fetch weather data",
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolError {
    /// Type discriminator (always "tool-error").
    #[serde(rename = "type")]
    pub error_type: String,

    /// The ID of the tool call that resulted in this error.
    pub tool_call_id: String,

    /// The name of the tool that was called.
    pub tool_name: String,

    /// The input that was provided to the tool.
    pub input: Value,

    /// The error that occurred during execution.
    pub error: Value,

    /// Whether the provider executed this tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,
}

impl ToolError {
    /// Creates a new tool error.
    ///
    /// # Arguments
    ///
    /// * `tool_call_id` - The ID of the tool call
    /// * `tool_name` - The name of the tool
    /// * `input` - The input that was provided to the tool
    /// * `error` - The error value (can be string, object, etc.)
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_provider_utils::tool::ToolError;
    /// use serde_json::json;
    ///
    /// let error = ToolError::new(
    ///     "call_abc123",
    ///     "calculate_sum",
    ///     json!({"a": 5, "b": 3}),
    ///     "Division by zero",
    /// );
    /// ```
    pub fn new(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        input: Value,
        error: impl Into<Value>,
    ) -> Self {
        Self {
            error_type: "tool-error".to_string(),
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            input,
            error: error.into(),
            provider_executed: None,
        }
    }

    /// Sets whether the provider executed this tool.
    ///
    /// # Arguments
    ///
    /// * `executed` - True if the provider executed the tool, false otherwise
    pub fn with_provider_executed(mut self, executed: bool) -> Self {
        self.provider_executed = Some(executed);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_tool_error_new() {
        let error = ToolError::new(
            "call_123",
            "get_weather",
            json!({"city": "SF"}),
            "Network timeout",
        );

        assert_eq!(error.error_type, "tool-error");
        assert_eq!(error.tool_call_id, "call_123");
        assert_eq!(error.tool_name, "get_weather");
        assert_eq!(error.input, json!({"city": "SF"}));
        assert_eq!(error.error, json!("Network timeout"));
        assert_eq!(error.provider_executed, None);
    }

    #[test]
    fn test_tool_error_with_provider_executed() {
        let error =
            ToolError::new("call_123", "tool", json!({}), "Error").with_provider_executed(true);

        assert_eq!(error.provider_executed, Some(true));
    }

    #[test]
    fn test_tool_error_serialization() {
        let error = ToolError::new(
            "call_123",
            "test_tool",
            json!({"key": "value"}),
            "Test error",
        )
        .with_provider_executed(true);

        let serialized = serde_json::to_value(&error).unwrap();

        assert_eq!(serialized["type"], "tool-error");
        assert_eq!(serialized["toolCallId"], "call_123");
        assert_eq!(serialized["toolName"], "test_tool");
        assert_eq!(serialized["input"], json!({"key": "value"}));
        assert_eq!(serialized["error"], "Test error");
        assert_eq!(serialized["providerExecuted"], true);
    }

    #[test]
    fn test_tool_error_clone() {
        let error = ToolError::new("call_123", "test_tool", json!({}), "Error message");

        let cloned = error.clone();
        assert_eq!(error, cloned);
    }

    #[test]
    fn test_tool_error_deserialization() {
        let error = ToolError::new(
            "call_456",
            "unknown_tool",
            json!({"param": "value"}),
            "Execution failed",
        )
        .with_provider_executed(false);

        let serialized = serde_json::to_string(&error).unwrap();
        let deserialized: ToolError = serde_json::from_str(&serialized).unwrap();

        assert_eq!(error, deserialized);
    }
}
