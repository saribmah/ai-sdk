use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A tool execution error for a statically-typed tool.
///
/// This represents an error that occurred during the execution of a tool
/// with a known input type at compile time.
///
/// # Type Parameters
///
/// * `INPUT` - The input type for the tool
///
/// # Example
///
/// ```
/// use ai_sdk_core::StaticToolError;
/// use serde_json::json;
///
/// let error = StaticToolError::new(
///     "call_123",
///     "get_weather",
///     json!({"city": "SF"}),
///     "Failed to fetch weather data",
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaticToolError<INPUT> {
    /// Type discriminator (always "tool-error").
    #[serde(rename = "type")]
    pub error_type: String,

    /// The ID of the tool call that resulted in this error.
    pub tool_call_id: String,

    /// The name of the tool that was called.
    pub tool_name: String,

    /// The input that was provided to the tool.
    pub input: INPUT,

    /// The error that occurred during execution.
    pub error: Value,

    /// Whether the provider executed this tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,

    /// Whether this is a dynamic tool (false or None for static tools).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,
}

impl<INPUT> StaticToolError<INPUT> {
    /// Creates a new static tool error.
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
    /// use ai_sdk_core::StaticToolError;
    /// use serde_json::json;
    ///
    /// let error = StaticToolError::new(
    ///     "call_abc123",
    ///     "calculate_sum",
    ///     json!({"a": 5, "b": 3}),
    ///     "Division by zero",
    /// );
    /// ```
    pub fn new(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        input: INPUT,
        error: impl Into<Value>,
    ) -> Self {
        Self {
            error_type: "tool-error".to_string(),
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            input,
            error: error.into(),
            provider_executed: None,
            dynamic: None,
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

/// A tool execution error for a dynamically-typed tool.
///
/// This represents an error that occurred during the execution of a tool
/// where the input type is not known at compile time.
///
/// # Example
///
/// ```
/// use ai_sdk_core::DynamicToolError;
/// use serde_json::json;
///
/// let error = DynamicToolError::new(
///     "call_456",
///     "unknown_tool",
///     json!({"param": "value"}),
///     "Tool execution failed",
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicToolError {
    /// Type discriminator (always "tool-error").
    #[serde(rename = "type")]
    pub error_type: String,

    /// The ID of the tool call that resulted in this error.
    pub tool_call_id: String,

    /// The name of the tool that was called.
    pub tool_name: String,

    /// The input that was provided to the tool (unknown type).
    pub input: Value,

    /// The error that occurred during execution.
    pub error: Value,

    /// Whether the provider executed this tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,

    /// Whether this is a dynamic tool (always true).
    pub dynamic: bool,
}

impl DynamicToolError {
    /// Creates a new dynamic tool error.
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
    /// use ai_sdk_core::DynamicToolError;
    /// use serde_json::json;
    ///
    /// let error = DynamicToolError::new(
    ///     "call_xyz789",
    ///     "fetch_data",
    ///     json!({"url": "https://api.example.com"}),
    ///     "Network timeout",
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
            dynamic: true,
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

/// A typed tool error that can be either static or dynamic.
///
/// This enum represents the union of `StaticToolError` and `DynamicToolError`,
/// similar to TypeScript's union type `StaticToolError<TOOLS> | DynamicToolError`.
///
/// # Type Parameters
///
/// * `INPUT` - The input type for static tool errors
///
/// # Example
///
/// ```
/// use ai_sdk_core::{TypedToolError, StaticToolError, DynamicToolError};
/// use serde_json::{json, Value};
///
/// // Create a static error
/// let static_error: TypedToolError<Value> = TypedToolError::Static(
///     StaticToolError::new(
///         "call_1",
///         "add",
///         json!({"a": 1, "b": 2}),
///         "Invalid input",
///     )
/// );
///
/// // Create a dynamic error
/// let dynamic_error: TypedToolError<Value> = TypedToolError::Dynamic(
///     DynamicToolError::new(
///         "call_2",
///         "unknown",
///         json!({}),
///         "Unknown error",
///     )
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TypedToolError<INPUT = Value> {
    /// A statically-typed tool error with known input type.
    Static(StaticToolError<INPUT>),

    /// A dynamically-typed tool error with unknown input type.
    Dynamic(DynamicToolError),
}

impl<INPUT> From<StaticToolError<INPUT>> for TypedToolError<INPUT> {
    fn from(error: StaticToolError<INPUT>) -> Self {
        TypedToolError::Static(error)
    }
}

impl<INPUT> From<DynamicToolError> for TypedToolError<INPUT> {
    fn from(error: DynamicToolError) -> Self {
        TypedToolError::Dynamic(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_static_tool_error_new() {
        let error = StaticToolError::new(
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
        assert_eq!(error.dynamic, None);
    }

    #[test]
    fn test_static_tool_error_with_provider_executed() {
        let error = StaticToolError::new(
            "call_123",
            "tool",
            json!({}),
            "Error",
        )
        .with_provider_executed(true);

        assert_eq!(error.provider_executed, Some(true));
    }

    #[test]
    fn test_dynamic_tool_error_new() {
        let error = DynamicToolError::new(
            "call_456",
            "unknown_tool",
            json!({"param": "value"}),
            "Tool execution failed",
        );

        assert_eq!(error.error_type, "tool-error");
        assert_eq!(error.tool_call_id, "call_456");
        assert_eq!(error.tool_name, "unknown_tool");
        assert_eq!(error.input, json!({"param": "value"}));
        assert_eq!(error.error, json!("Tool execution failed"));
        assert_eq!(error.provider_executed, None);
        assert_eq!(error.dynamic, true);
    }

    #[test]
    fn test_dynamic_tool_error_with_provider_executed() {
        let error = DynamicToolError::new(
            "call_456",
            "tool",
            json!({}),
            "Error",
        )
        .with_provider_executed(false);

        assert_eq!(error.provider_executed, Some(false));
    }

    #[test]
    fn test_typed_tool_error_static() {
        let static_error = StaticToolError::new(
            "call_1",
            "add",
            json!({"a": 1}),
            "Invalid input",
        );

        let typed: TypedToolError<Value> = TypedToolError::Static(static_error.clone());

        match typed {
            TypedToolError::Static(e) => assert_eq!(e, static_error),
            TypedToolError::Dynamic(_) => panic!("Expected Static variant"),
        }
    }

    #[test]
    fn test_typed_tool_error_dynamic() {
        let dynamic_error = DynamicToolError::new(
            "call_2",
            "unknown",
            json!({}),
            "Unknown error",
        );

        let typed: TypedToolError<Value> = TypedToolError::Dynamic(dynamic_error.clone());

        match typed {
            TypedToolError::Static(_) => panic!("Expected Dynamic variant"),
            TypedToolError::Dynamic(e) => assert_eq!(e, dynamic_error),
        }
    }

    #[test]
    fn test_static_tool_error_serialization() {
        let error = StaticToolError::new(
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
    fn test_dynamic_tool_error_serialization() {
        let error = DynamicToolError::new(
            "call_456",
            "unknown_tool",
            json!({"param": "value"}),
            "Execution failed",
        )
        .with_provider_executed(false);

        let serialized = serde_json::to_value(&error).unwrap();

        assert_eq!(serialized["type"], "tool-error");
        assert_eq!(serialized["toolCallId"], "call_456");
        assert_eq!(serialized["toolName"], "unknown_tool");
        assert_eq!(serialized["input"], json!({"param": "value"}));
        assert_eq!(serialized["error"], "Execution failed");
        assert_eq!(serialized["providerExecuted"], false);
        assert_eq!(serialized["dynamic"], true);
    }

    #[test]
    fn test_typed_tool_error_serialization() {
        let typed: TypedToolError<Value> = TypedToolError::Static(
            StaticToolError::new(
                "call_1",
                "add",
                json!({"a": 1, "b": 2}),
                "Invalid operation",
            ),
        );

        let serialized = serde_json::to_string(&typed).unwrap();
        let deserialized: TypedToolError<Value> =
            serde_json::from_str(&serialized).unwrap();

        assert_eq!(typed, deserialized);
    }

    #[test]
    fn test_static_tool_error_clone() {
        let error = StaticToolError::new(
            "call_123",
            "test_tool",
            json!({}),
            "Error message",
        );

        let cloned = error.clone();
        assert_eq!(error, cloned);
    }

    #[test]
    fn test_dynamic_tool_error_clone() {
        let error = DynamicToolError::new(
            "call_456",
            "test_tool",
            json!({}),
            "Error message",
        );

        let cloned = error.clone();
        assert_eq!(error, cloned);
    }

    #[test]
    fn test_from_static_tool_error() {
        let static_error = StaticToolError::new(
            "call_1",
            "tool",
            json!({}),
            "Error",
        );

        let typed: TypedToolError<Value> = static_error.clone().into();

        match typed {
            TypedToolError::Static(e) => assert_eq!(e, static_error),
            _ => panic!("Expected Static variant"),
        }
    }

    #[test]
    fn test_from_dynamic_tool_error() {
        let dynamic_error = DynamicToolError::new(
            "call_2",
            "tool",
            json!({}),
            "Error",
        );

        let typed: TypedToolError<Value> = dynamic_error.clone().into();

        match typed {
            TypedToolError::Dynamic(e) => assert_eq!(e, dynamic_error),
            _ => panic!("Expected Dynamic variant"),
        }
    }
}
