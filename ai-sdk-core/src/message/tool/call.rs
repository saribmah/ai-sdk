use serde::{Deserialize, Serialize};

/// Typed tool call that is returned by generateText and streamText.
///
/// It contains the tool call ID, the tool name, and the tool arguments.
///
/// # Type Parameters
///
/// * `I` - The input type for the tool arguments. This should be a type that implements
///   `Serialize` and `Deserialize` to match the tool's input schema.
///
/// # Examples
///
/// ```
/// use ai_sdk_core::message::ToolCall;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Serialize, Deserialize)]
/// struct WeatherInput {
///     city: String,
///     units: String,
/// }
///
/// let tool_call = ToolCall::new(
///     "call_123",
///     "get_weather",
///     WeatherInput {
///         city: "San Francisco".to_string(),
///         units: "celsius".to_string(),
///     }
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolCall<I> {
    /// ID of the tool call. This ID is used to match the tool call with the tool result.
    #[serde(rename = "toolCallId")]
    pub tool_call_id: String,

    /// Name of the tool that is being called.
    #[serde(rename = "toolName")]
    pub tool_name: String,

    /// Arguments of the tool call.
    ///
    /// This is a JSON-serializable object that matches the tool's input schema.
    pub input: I,

    /// Whether the tool call will be executed by the provider.
    ///
    /// If this flag is not set or is false, the tool call will be executed by the client.
    #[serde(rename = "providerExecuted", skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,

    /// Whether the tool is dynamic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,
}

impl<I> ToolCall<I> {
    /// Creates a new tool call.
    pub fn new(tool_call_id: impl Into<String>, tool_name: impl Into<String>, input: I) -> Self {
        Self {
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            input,
            provider_executed: None,
            dynamic: None,
        }
    }

    /// Sets whether the tool call will be executed by the provider.
    pub fn with_provider_executed(mut self, executed: bool) -> Self {
        self.provider_executed = Some(executed);
        self
    }

    /// Sets whether the tool is dynamic.
    pub fn with_dynamic(mut self, dynamic: bool) -> Self {
        self.dynamic = Some(dynamic);
        self
    }

    /// Returns true if the tool will be executed by the provider.
    pub fn is_provider_executed(&self) -> bool {
        self.provider_executed.unwrap_or(false)
    }

    /// Returns true if the tool is dynamic.
    pub fn is_dynamic(&self) -> bool {
        self.dynamic.unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct TestInput {
        city: String,
        units: String,
    }

    #[test]
    fn test_tool_call_new() {
        let input = TestInput {
            city: "San Francisco".to_string(),
            units: "celsius".to_string(),
        };

        let tool_call = ToolCall::new("call_123", "get_weather", input.clone());

        assert_eq!(tool_call.tool_call_id, "call_123");
        assert_eq!(tool_call.tool_name, "get_weather");
        assert_eq!(tool_call.input, input);
        assert!(tool_call.provider_executed.is_none());
        assert!(tool_call.dynamic.is_none());
    }

    #[test]
    fn test_tool_call_with_provider_executed() {
        let input = TestInput {
            city: "New York".to_string(),
            units: "fahrenheit".to_string(),
        };

        let tool_call =
            ToolCall::new("call_456", "get_weather", input).with_provider_executed(true);

        assert_eq!(tool_call.provider_executed, Some(true));
        assert!(tool_call.is_provider_executed());
    }

    #[test]
    fn test_tool_call_with_dynamic() {
        let input = TestInput {
            city: "London".to_string(),
            units: "celsius".to_string(),
        };

        let tool_call = ToolCall::new("call_789", "get_weather", input).with_dynamic(true);

        assert_eq!(tool_call.dynamic, Some(true));
        assert!(tool_call.is_dynamic());
    }

    #[test]
    fn test_tool_call_is_provider_executed_default() {
        let input = TestInput {
            city: "Paris".to_string(),
            units: "celsius".to_string(),
        };

        let tool_call = ToolCall::new("call_000", "get_weather", input);

        assert!(!tool_call.is_provider_executed());
    }

    #[test]
    fn test_tool_call_is_dynamic_default() {
        let input = TestInput {
            city: "Tokyo".to_string(),
            units: "celsius".to_string(),
        };

        let tool_call = ToolCall::new("call_111", "get_weather", input);

        assert!(!tool_call.is_dynamic());
    }

    #[test]
    fn test_tool_call_serialization() {
        let input = json!({
            "city": "Berlin",
            "units": "celsius"
        });

        let tool_call = ToolCall::new("call_222", "get_weather", input);

        let serialized = serde_json::to_value(&tool_call).unwrap();

        assert_eq!(serialized["toolCallId"], "call_222");
        assert_eq!(serialized["toolName"], "get_weather");
        assert_eq!(serialized["input"]["city"], "Berlin");
        assert!(serialized.get("providerExecuted").is_none());
        assert!(serialized.get("dynamic").is_none());
    }

    #[test]
    fn test_tool_call_deserialization() {
        let json = json!({
            "toolCallId": "call_333",
            "toolName": "get_weather",
            "input": {
                "city": "Sydney",
                "units": "celsius"
            },
            "providerExecuted": true,
            "dynamic": false
        });

        let tool_call: ToolCall<serde_json::Value> = serde_json::from_value(json).unwrap();

        assert_eq!(tool_call.tool_call_id, "call_333");
        assert_eq!(tool_call.tool_name, "get_weather");
        assert_eq!(tool_call.provider_executed, Some(true));
        assert_eq!(tool_call.dynamic, Some(false));
    }

    #[test]
    fn test_tool_call_clone() {
        let input = TestInput {
            city: "Rome".to_string(),
            units: "celsius".to_string(),
        };

        let tool_call = ToolCall::new("call_444", "get_weather", input);
        let cloned = tool_call.clone();

        assert_eq!(tool_call, cloned);
    }
}
