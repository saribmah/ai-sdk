use llm_kit_provider::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A tool call representing an invocation of a tool by the language model.
///
/// This contains all information about a tool call, including its ID, name,
/// input parameters, and metadata about how it was executed.
///
/// # Example
///
/// ```
/// use llm_kit_provider_utils::tool::ToolCall;
/// use serde_json::json;
///
/// let tool_call = ToolCall::new(
///     "call_123",
///     "get_weather",
///     json!({"city": "San Francisco"}),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolCall {
    /// Type discriminator (always "tool-call").
    #[serde(rename = "type")]
    pub call_type: String,

    /// The ID of this tool call.
    pub tool_call_id: String,

    /// The name of the tool being called.
    pub tool_name: String,

    /// The input parameters for the tool.
    pub input: Value,

    /// Whether the provider executed this tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,

    /// Provider-specific metadata for this tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,

    /// True if this is caused by an unparsable tool call or a tool that does not exist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid: Option<bool>,

    /// The error that caused the tool call to be invalid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

impl ToolCall {
    /// Creates a new tool call.
    ///
    /// # Arguments
    ///
    /// * `tool_call_id` - The ID of this tool call
    /// * `tool_name` - The name of the tool being called
    /// * `input` - The input parameters for the tool
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_provider_utils::tool::ToolCall;
    /// use serde_json::json;
    ///
    /// let tool_call = ToolCall::new(
    ///     "call_abc123",
    ///     "calculate_sum",
    ///     json!({"a": 5, "b": 3}),
    /// );
    /// ```
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
            invalid: None,
            error: None,
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

    /// Sets the provider metadata for this tool call.
    ///
    /// # Arguments
    ///
    /// * `metadata` - Provider-specific metadata
    pub fn with_provider_metadata(mut self, metadata: SharedProviderMetadata) -> Self {
        self.provider_metadata = Some(metadata);
        self
    }

    /// Marks this tool call as invalid.
    ///
    /// # Arguments
    ///
    /// * `invalid` - True if the tool call is invalid
    pub fn with_invalid(mut self, invalid: bool) -> Self {
        self.invalid = Some(invalid);
        self
    }

    /// Sets the error that caused this tool call to be invalid.
    ///
    /// # Arguments
    ///
    /// * `error` - The error value
    pub fn with_error(mut self, error: impl Into<Value>) -> Self {
        self.error = Some(error.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_tool_call_new() {
        let call = ToolCall::new("call_123", "get_weather", json!({"city": "SF"}));

        assert_eq!(call.call_type, "tool-call");
        assert_eq!(call.tool_call_id, "call_123");
        assert_eq!(call.tool_name, "get_weather");
        assert_eq!(call.input, json!({"city": "SF"}));
        assert_eq!(call.provider_executed, None);
        assert_eq!(call.provider_metadata, None);
        assert_eq!(call.invalid, None);
        assert_eq!(call.error, None);
    }

    #[test]
    fn test_tool_call_with_provider_executed() {
        let call = ToolCall::new("call_123", "tool", json!({})).with_provider_executed(true);

        assert_eq!(call.provider_executed, Some(true));
    }

    #[test]
    fn test_tool_call_with_provider_metadata() {
        use std::collections::HashMap;
        let mut metadata = SharedProviderMetadata::new();
        let mut inner = HashMap::new();
        inner.insert("key".to_string(), json!("value"));
        metadata.insert("provider".to_string(), inner);

        let call =
            ToolCall::new("call_123", "tool", json!({})).with_provider_metadata(metadata.clone());

        assert_eq!(call.provider_metadata, Some(metadata));
    }

    #[test]
    fn test_tool_call_with_invalid() {
        let call = ToolCall::new("call_456", "tool", json!({})).with_invalid(true);

        assert_eq!(call.invalid, Some(true));
    }

    #[test]
    fn test_tool_call_with_error() {
        let call = ToolCall::new("call_456", "tool", json!({}))
            .with_error(json!({"message": "Tool not found"}));

        assert_eq!(call.error, Some(json!({"message": "Tool not found"})));
    }

    #[test]
    fn test_tool_call_invalid_with_error() {
        let call = ToolCall::new("call_789", "nonexistent_tool", json!({}))
            .with_invalid(true)
            .with_error(json!({"message": "Tool does not exist"}));

        assert_eq!(call.invalid, Some(true));
        assert_eq!(call.error, Some(json!({"message": "Tool does not exist"})));
    }

    #[test]
    fn test_tool_call_serialization() {
        let call = ToolCall::new("call_123", "test_tool", json!({"key": "value"}))
            .with_provider_executed(true);

        let serialized = serde_json::to_value(&call).unwrap();

        assert_eq!(serialized["type"], "tool-call");
        assert_eq!(serialized["toolCallId"], "call_123");
        assert_eq!(serialized["toolName"], "test_tool");
        assert_eq!(serialized["input"], json!({"key": "value"}));
        assert_eq!(serialized["providerExecuted"], true);
    }

    #[test]
    fn test_tool_call_clone() {
        let call = ToolCall::new("call_123", "test_tool", json!({}));

        let cloned = call.clone();
        assert_eq!(call, cloned);
    }
}
