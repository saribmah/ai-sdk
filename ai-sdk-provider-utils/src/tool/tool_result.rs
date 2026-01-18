use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A tool result representing the output from a successful tool execution.
///
/// This contains the input that was provided to the tool and the output
/// that was generated, along with metadata about the execution.
///
/// # Example
///
/// ```
/// use llm_kit_provider_utils::tool::ToolResult;
/// use serde_json::json;
///
/// let result = ToolResult::new(
///     "call_123",
///     "get_weather",
///     json!({"city": "San Francisco"}),
///     json!({"temperature": 72, "conditions": "sunny"}),
/// );
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ToolResult {
    /// The type discriminator (always "tool-result")
    #[serde(rename = "type")]
    pub result_type: String,

    /// The ID of the tool call this result corresponds to
    pub tool_call_id: String,

    /// The name of the tool
    pub tool_name: String,

    /// The input that was provided to the tool
    pub input: Value,

    /// The output from the tool execution
    pub output: Value,

    /// Whether the provider executed this tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,

    /// Whether this is a preliminary result (for streaming tools)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preliminary: Option<bool>,
}

impl ToolResult {
    /// Creates a new tool result.
    ///
    /// # Arguments
    ///
    /// * `tool_call_id` - The ID of the tool call this result corresponds to
    /// * `tool_name` - The name of the tool that was executed
    /// * `input` - The input that was provided to the tool
    /// * `output` - The output from the tool execution
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_provider_utils::tool::ToolResult;
    /// use serde_json::json;
    ///
    /// let result = ToolResult::new(
    ///     "call_abc123",
    ///     "calculate_sum",
    ///     json!({"a": 5, "b": 3}),
    ///     json!({"sum": 8}),
    /// );
    /// ```
    pub fn new(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        input: Value,
        output: Value,
    ) -> Self {
        Self {
            result_type: "tool-result".to_string(),
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            input,
            output,
            provider_executed: None,
            preliminary: None,
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

    /// Sets whether this is a preliminary result.
    ///
    /// Preliminary results are used for streaming tools that emit intermediate outputs
    /// before the final result.
    ///
    /// # Arguments
    ///
    /// * `preliminary` - True if this is a preliminary result, false for final results
    pub fn with_preliminary(mut self, preliminary: bool) -> Self {
        self.preliminary = Some(preliminary);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_tool_result_new() {
        let result = ToolResult::new(
            "call_123",
            "get_weather",
            json!({"city": "SF"}),
            json!({"temp": 72}),
        );

        assert_eq!(result.result_type, "tool-result");
        assert_eq!(result.tool_call_id, "call_123");
        assert_eq!(result.tool_name, "get_weather");
        assert_eq!(result.input, json!({"city": "SF"}));
        assert_eq!(result.output, json!({"temp": 72}));
        assert_eq!(result.provider_executed, None);
        assert_eq!(result.preliminary, None);
    }

    #[test]
    fn test_tool_result_with_provider_executed() {
        let result =
            ToolResult::new("call_123", "tool", json!({}), json!({})).with_provider_executed(true);

        assert_eq!(result.provider_executed, Some(true));
    }

    #[test]
    fn test_tool_result_with_preliminary() {
        let result =
            ToolResult::new("call_123", "tool", json!({}), json!({})).with_preliminary(true);

        assert_eq!(result.preliminary, Some(true));
    }

    #[test]
    fn test_tool_result_with_options() {
        let result = ToolResult::new("call_456", "tool", json!({}), json!({}))
            .with_provider_executed(false)
            .with_preliminary(true);

        assert_eq!(result.provider_executed, Some(false));
        assert_eq!(result.preliminary, Some(true));
    }

    #[test]
    fn test_tool_result_serialization() {
        let result = ToolResult::new(
            "call_123",
            "get_weather",
            json!({"city": "SF"}),
            json!({"temp": 72}),
        )
        .with_provider_executed(true)
        .with_preliminary(false);

        let serialized = serde_json::to_string(&result).unwrap();
        let deserialized: ToolResult = serde_json::from_str(&serialized).unwrap();

        assert_eq!(result, deserialized);
    }

    #[test]
    fn test_json_field_names() {
        let result = ToolResult::new("call_123", "tool", json!({}), json!({}));

        let json = serde_json::to_value(&result).unwrap();

        // Check that fields are serialized with camelCase
        assert_eq!(json["type"], "tool-result");
        assert_eq!(json["toolCallId"], "call_123");
        assert_eq!(json["toolName"], "tool");
        assert!(json.get("provider_executed").is_none()); // None fields should be omitted
        assert!(json.get("providerExecuted").is_none()); // None fields should be omitted
    }
}
