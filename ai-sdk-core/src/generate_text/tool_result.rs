use serde::{Deserialize, Serialize};

/// A tool result for a statically-typed tool.
///
/// This represents the result of executing a tool with known input and output types.
/// Each tool in a ToolSet can have its own specific input and output types.
///
/// # Type Parameters
///
/// * `INPUT` - The input type for the tool
/// * `OUTPUT` - The output type from the tool
///
/// # Example
///
/// ```
/// use ai_sdk_core::StaticToolResult;
/// use serde_json::json;
///
/// let result = StaticToolResult::new(
///     "call_123",
///     "get_weather",
///     json!({"city": "San Francisco"}),
///     json!({"temperature": 72, "conditions": "sunny"}),
/// );
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StaticToolResult<INPUT, OUTPUT> {
    /// The type discriminator (always "tool-result")
    #[serde(rename = "type")]
    pub result_type: String,

    /// The ID of the tool call this result corresponds to
    #[serde(rename = "toolCallId")]
    pub tool_call_id: String,

    /// The name of the tool
    #[serde(rename = "toolName")]
    pub tool_name: String,

    /// The input that was provided to the tool
    pub input: INPUT,

    /// The output from the tool execution
    pub output: OUTPUT,

    /// Whether the provider executed this tool
    #[serde(rename = "providerExecuted", skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,

    /// Whether this is a dynamic tool (false or None for static tools)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,

    /// Whether this is a preliminary result (for streaming tools)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preliminary: Option<bool>,
}

impl<INPUT, OUTPUT> StaticToolResult<INPUT, OUTPUT> {
    /// Creates a new static tool result.
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
    /// use ai_sdk_core::StaticToolResult;
    /// use serde_json::json;
    ///
    /// let result = StaticToolResult::new(
    ///     "call_abc123",
    ///     "calculate_sum",
    ///     json!({"a": 5, "b": 3}),
    ///     json!({"sum": 8}),
    /// );
    /// ```
    pub fn new(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        input: INPUT,
        output: OUTPUT,
    ) -> Self {
        Self {
            result_type: "tool-result".to_string(),
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            input,
            output,
            provider_executed: None,
            dynamic: None,
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

/// A tool result for a dynamically-typed tool.
///
/// This represents the result of executing a tool where the input/output types
/// are not known at compile time. Both input and output use `serde_json::Value`
/// for maximum flexibility.
///
/// # Example
///
/// ```
/// use ai_sdk_core::DynamicToolResult;
/// use serde_json::json;
///
/// let result = DynamicToolResult::new(
///     "call_456",
///     "unknown_tool",
///     json!({"param": "value"}),
///     json!({"result": "success"}),
/// );
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DynamicToolResult {
    /// The type discriminator (always "tool-result")
    #[serde(rename = "type")]
    pub result_type: String,

    /// The ID of the tool call this result corresponds to
    #[serde(rename = "toolCallId")]
    pub tool_call_id: String,

    /// The name of the tool
    #[serde(rename = "toolName")]
    pub tool_name: String,

    /// The input that was provided to the tool (unknown type)
    pub input: serde_json::Value,

    /// The output from the tool execution (unknown type)
    pub output: serde_json::Value,

    /// Whether the provider executed this tool
    #[serde(rename = "providerExecuted", skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,

    /// Whether this is a dynamic tool (always true)
    pub dynamic: bool,

    /// Whether this is a preliminary result (for streaming tools)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preliminary: Option<bool>,
}

impl DynamicToolResult {
    /// Creates a new dynamic tool result.
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
    /// use ai_sdk_core::DynamicToolResult;
    /// use serde_json::json;
    ///
    /// let result = DynamicToolResult::new(
    ///     "call_xyz789",
    ///     "fetch_data",
    ///     json!({"url": "https://api.example.com"}),
    ///     json!({"data": [1, 2, 3]}),
    /// );
    /// ```
    pub fn new(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        input: serde_json::Value,
        output: serde_json::Value,
    ) -> Self {
        Self {
            result_type: "tool-result".to_string(),
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            input,
            output,
            provider_executed: None,
            dynamic: true,
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

/// A typed tool result that can be either static or dynamic.
///
/// This enum represents the union of `StaticToolResult` and `DynamicToolResult`,
/// similar to TypeScript's union type `StaticToolResult<TOOLS> | DynamicToolResult`.
///
/// Use this when you need to handle both statically-typed and dynamically-typed
/// tool results in a uniform way.
///
/// # Type Parameters
///
/// * `INPUT` - The input type for static tool results
/// * `OUTPUT` - The output type for static tool results
///
/// # Example
///
/// ```
/// use ai_sdk_core::{TypedToolResult, StaticToolResult, DynamicToolResult};
/// use serde_json::{json, Value};
///
/// // Create a static result
/// let static_result: TypedToolResult<Value, Value> = TypedToolResult::Static(
///     StaticToolResult::new(
///         "call_1",
///         "add",
///         json!({"a": 1, "b": 2}),
///         json!({"sum": 3}),
///     )
/// );
///
/// // Create a dynamic result
/// let dynamic_result: TypedToolResult<Value, Value> = TypedToolResult::Dynamic(
///     DynamicToolResult::new(
///         "call_2",
///         "unknown",
///         json!({}),
///         json!({}),
///     )
/// );
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TypedToolResult<INPUT, OUTPUT> {
    /// A statically-typed tool result with known input/output types
    Static(StaticToolResult<INPUT, OUTPUT>),

    /// A dynamically-typed tool result with unknown input/output types
    Dynamic(DynamicToolResult),
}

impl<INPUT, OUTPUT> From<StaticToolResult<INPUT, OUTPUT>> for TypedToolResult<INPUT, OUTPUT> {
    fn from(result: StaticToolResult<INPUT, OUTPUT>) -> Self {
        TypedToolResult::Static(result)
    }
}

impl<INPUT, OUTPUT> From<DynamicToolResult> for TypedToolResult<INPUT, OUTPUT> {
    fn from(result: DynamicToolResult) -> Self {
        TypedToolResult::Dynamic(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    #[test]
    fn test_static_tool_result_new() {
        let result = StaticToolResult::new(
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
        assert_eq!(result.dynamic, None);
        assert_eq!(result.preliminary, None);
    }

    #[test]
    fn test_static_tool_result_with_provider_executed() {
        let result = StaticToolResult::new(
            "call_123",
            "tool",
            json!({}),
            json!({}),
        )
        .with_provider_executed(true);

        assert_eq!(result.provider_executed, Some(true));
    }

    #[test]
    fn test_static_tool_result_with_preliminary() {
        let result = StaticToolResult::new(
            "call_123",
            "tool",
            json!({}),
            json!({}),
        )
        .with_preliminary(true);

        assert_eq!(result.preliminary, Some(true));
    }

    #[test]
    fn test_dynamic_tool_result_new() {
        let result = DynamicToolResult::new(
            "call_456",
            "unknown_tool",
            json!({"param": "value"}),
            json!({"result": "success"}),
        );

        assert_eq!(result.result_type, "tool-result");
        assert_eq!(result.tool_call_id, "call_456");
        assert_eq!(result.tool_name, "unknown_tool");
        assert_eq!(result.input, json!({"param": "value"}));
        assert_eq!(result.output, json!({"result": "success"}));
        assert_eq!(result.provider_executed, None);
        assert_eq!(result.dynamic, true);
        assert_eq!(result.preliminary, None);
    }

    #[test]
    fn test_dynamic_tool_result_with_options() {
        let result = DynamicToolResult::new(
            "call_456",
            "tool",
            json!({}),
            json!({}),
        )
        .with_provider_executed(false)
        .with_preliminary(true);

        assert_eq!(result.provider_executed, Some(false));
        assert_eq!(result.preliminary, Some(true));
    }

    #[test]
    fn test_typed_tool_result_static() {
        let static_result = StaticToolResult::new(
            "call_1",
            "add",
            json!({"a": 1}),
            json!({"sum": 1}),
        );

        let typed: TypedToolResult<Value, Value> = TypedToolResult::Static(static_result.clone());

        match typed {
            TypedToolResult::Static(r) => assert_eq!(r, static_result),
            TypedToolResult::Dynamic(_) => panic!("Expected Static variant"),
        }
    }

    #[test]
    fn test_typed_tool_result_dynamic() {
        let dynamic_result = DynamicToolResult::new(
            "call_2",
            "unknown",
            json!({}),
            json!({}),
        );

        let typed: TypedToolResult<Value, Value> = TypedToolResult::Dynamic(dynamic_result.clone());

        match typed {
            TypedToolResult::Static(_) => panic!("Expected Dynamic variant"),
            TypedToolResult::Dynamic(r) => assert_eq!(r, dynamic_result),
        }
    }

    #[test]
    fn test_static_tool_result_serialization() {
        let result = StaticToolResult::new(
            "call_123",
            "get_weather",
            json!({"city": "SF"}),
            json!({"temp": 72}),
        )
        .with_provider_executed(true)
        .with_preliminary(false);

        let serialized = serde_json::to_string(&result).unwrap();
        let deserialized: StaticToolResult<Value, Value> =
            serde_json::from_str(&serialized).unwrap();

        assert_eq!(result, deserialized);
    }

    #[test]
    fn test_dynamic_tool_result_serialization() {
        let result = DynamicToolResult::new(
            "call_456",
            "unknown_tool",
            json!({"param": "value"}),
            json!({"result": "success"}),
        )
        .with_provider_executed(false);

        let serialized = serde_json::to_string(&result).unwrap();
        let deserialized: DynamicToolResult = serde_json::from_str(&serialized).unwrap();

        assert_eq!(result, deserialized);
    }

    #[test]
    fn test_typed_tool_result_serialization() {
        let typed: TypedToolResult<Value, Value> = TypedToolResult::Static(
            StaticToolResult::new(
                "call_1",
                "add",
                json!({"a": 1, "b": 2}),
                json!({"sum": 3}),
            ),
        );

        let serialized = serde_json::to_string(&typed).unwrap();
        let deserialized: TypedToolResult<Value, Value> =
            serde_json::from_str(&serialized).unwrap();

        assert_eq!(typed, deserialized);
    }

    #[test]
    fn test_json_field_names() {
        let result = StaticToolResult::new(
            "call_123",
            "tool",
            json!({}),
            json!({}),
        );

        let json = serde_json::to_value(&result).unwrap();

        // Check that fields are serialized with camelCase
        assert_eq!(json["type"], "tool-result");
        assert_eq!(json["toolCallId"], "call_123");
        assert_eq!(json["toolName"], "tool");
        assert!(json.get("provider_executed").is_none()); // None fields should be omitted
        assert!(json.get("providerExecuted").is_none()); // None fields should be omitted
    }
}
