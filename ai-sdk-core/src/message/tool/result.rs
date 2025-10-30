use serde::{Deserialize, Serialize};

/// Typed tool result that is returned by `generateText` and `streamText`.
///
/// It contains the tool call ID, the tool name, the tool arguments, and the tool result.
///
/// # Type Parameters
///
/// * `I` - The input type for the tool arguments. This should match the tool's input schema.
/// * `O` - The output type for the tool result. This is the result of the tool's execution.
///
/// # Examples
///
/// ```
/// use ai_sdk_core::message::ToolResult;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Serialize, Deserialize)]
/// struct WeatherInput {
///     city: String,
///     units: String,
/// }
///
/// #[derive(Debug, Serialize, Deserialize)]
/// struct WeatherOutput {
///     temperature: f64,
///     condition: String,
/// }
///
/// let tool_result = ToolResult::new(
///     "call_123",
///     "get_weather",
///     WeatherInput {
///         city: "San Francisco".to_string(),
///         units: "celsius".to_string(),
///     },
///     WeatherOutput {
///         temperature: 18.5,
///         condition: "Partly cloudy".to_string(),
///     }
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolResult<I, O> {
    /// ID of the tool call. This ID is used to match the tool call with the tool result.
    #[serde(rename = "toolCallId")]
    pub tool_call_id: String,

    /// Name of the tool that was called.
    #[serde(rename = "toolName")]
    pub tool_name: String,

    /// Arguments of the tool call.
    ///
    /// This is a JSON-serializable object that matches the tool's input schema.
    pub input: I,

    /// Result of the tool call. This is the result of the tool's execution.
    pub output: O,

    /// Whether the tool result has been executed by the provider.
    #[serde(rename = "providerExecuted", skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,

    /// Whether the tool is dynamic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,
}

impl<I, O> ToolResult<I, O> {
    /// Creates a new tool result.
    pub fn new(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        input: I,
        output: O,
    ) -> Self {
        Self {
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            input,
            output,
            provider_executed: None,
            dynamic: None,
        }
    }

    /// Sets whether the tool result has been executed by the provider.
    pub fn with_provider_executed(mut self, executed: bool) -> Self {
        self.provider_executed = Some(executed);
        self
    }

    /// Sets whether the tool is dynamic.
    pub fn with_dynamic(mut self, dynamic: bool) -> Self {
        self.dynamic = Some(dynamic);
        self
    }

    /// Returns true if the tool was executed by the provider.
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

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct TestOutput {
        temperature: f64,
        condition: String,
    }

    #[test]
    fn test_tool_result_new() {
        let input = TestInput {
            city: "San Francisco".to_string(),
            units: "celsius".to_string(),
        };

        let output = TestOutput {
            temperature: 18.5,
            condition: "Partly cloudy".to_string(),
        };

        let tool_result = ToolResult::new("call_123", "get_weather", input.clone(), output.clone());

        assert_eq!(tool_result.tool_call_id, "call_123");
        assert_eq!(tool_result.tool_name, "get_weather");
        assert_eq!(tool_result.input, input);
        assert_eq!(tool_result.output, output);
        assert!(tool_result.provider_executed.is_none());
        assert!(tool_result.dynamic.is_none());
    }

    #[test]
    fn test_tool_result_with_provider_executed() {
        let input = TestInput {
            city: "New York".to_string(),
            units: "fahrenheit".to_string(),
        };

        let output = TestOutput {
            temperature: 72.0,
            condition: "Sunny".to_string(),
        };

        let tool_result =
            ToolResult::new("call_456", "get_weather", input, output).with_provider_executed(true);

        assert_eq!(tool_result.provider_executed, Some(true));
        assert!(tool_result.is_provider_executed());
    }

    #[test]
    fn test_tool_result_with_dynamic() {
        let input = TestInput {
            city: "London".to_string(),
            units: "celsius".to_string(),
        };

        let output = TestOutput {
            temperature: 15.0,
            condition: "Rainy".to_string(),
        };

        let tool_result =
            ToolResult::new("call_789", "get_weather", input, output).with_dynamic(true);

        assert_eq!(tool_result.dynamic, Some(true));
        assert!(tool_result.is_dynamic());
    }

    #[test]
    fn test_tool_result_is_provider_executed_default() {
        let input = TestInput {
            city: "Paris".to_string(),
            units: "celsius".to_string(),
        };

        let output = TestOutput {
            temperature: 20.0,
            condition: "Clear".to_string(),
        };

        let tool_result = ToolResult::new("call_000", "get_weather", input, output);

        assert!(!tool_result.is_provider_executed());
    }

    #[test]
    fn test_tool_result_is_dynamic_default() {
        let input = TestInput {
            city: "Tokyo".to_string(),
            units: "celsius".to_string(),
        };

        let output = TestOutput {
            temperature: 22.0,
            condition: "Foggy".to_string(),
        };

        let tool_result = ToolResult::new("call_111", "get_weather", input, output);

        assert!(!tool_result.is_dynamic());
    }

    #[test]
    fn test_tool_result_serialization() {
        let input = json!({
            "city": "Berlin",
            "units": "celsius"
        });

        let output = json!({
            "temperature": 16.0,
            "condition": "Cloudy"
        });

        let tool_result = ToolResult::new("call_222", "get_weather", input, output);

        let serialized = serde_json::to_value(&tool_result).unwrap();

        assert_eq!(serialized["toolCallId"], "call_222");
        assert_eq!(serialized["toolName"], "get_weather");
        assert_eq!(serialized["input"]["city"], "Berlin");
        assert_eq!(serialized["output"]["temperature"], 16.0);
        assert!(serialized.get("providerExecuted").is_none());
        assert!(serialized.get("dynamic").is_none());
    }

    #[test]
    fn test_tool_result_deserialization() {
        let json = json!({
            "toolCallId": "call_333",
            "toolName": "get_weather",
            "input": {
                "city": "Sydney",
                "units": "celsius"
            },
            "output": {
                "temperature": 25.0,
                "condition": "Sunny"
            },
            "providerExecuted": true,
            "dynamic": false
        });

        let tool_result: ToolResult<serde_json::Value, serde_json::Value> =
            serde_json::from_value(json).unwrap();

        assert_eq!(tool_result.tool_call_id, "call_333");
        assert_eq!(tool_result.tool_name, "get_weather");
        assert_eq!(tool_result.input["city"], "Sydney");
        assert_eq!(tool_result.output["temperature"], 25.0);
        assert_eq!(tool_result.provider_executed, Some(true));
        assert_eq!(tool_result.dynamic, Some(false));
    }

    #[test]
    fn test_tool_result_clone() {
        let input = TestInput {
            city: "Rome".to_string(),
            units: "celsius".to_string(),
        };

        let output = TestOutput {
            temperature: 28.0,
            condition: "Hot".to_string(),
        };

        let tool_result = ToolResult::new("call_444", "get_weather", input, output);
        let cloned = tool_result.clone();

        assert_eq!(tool_result, cloned);
    }

    #[test]
    fn test_tool_result_with_all_flags() {
        let input = TestInput {
            city: "Madrid".to_string(),
            units: "celsius".to_string(),
        };

        let output = TestOutput {
            temperature: 30.0,
            condition: "Very hot".to_string(),
        };

        let tool_result = ToolResult::new("call_555", "get_weather", input, output)
            .with_provider_executed(true)
            .with_dynamic(true);

        assert!(tool_result.is_provider_executed());
        assert!(tool_result.is_dynamic());
        assert_eq!(tool_result.provider_executed, Some(true));
        assert_eq!(tool_result.dynamic, Some(true));
    }
}
