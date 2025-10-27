use ai_sdk_provider::shared::provider_metadata::ProviderMetadata;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A tool call for a statically-typed tool.
///
/// This represents a tool call where the input type is known at compile time.
///
/// # Type Parameters
///
/// * `INPUT` - The input type for the tool
///
/// # Example
///
/// ```
/// use ai_sdk_core::StaticToolCall;
/// use serde_json::json;
///
/// let tool_call = StaticToolCall::new(
///     "call_123",
///     "get_weather",
///     json!({"city": "SF"}),
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StaticToolCall<INPUT> {
    /// Type discriminator (always "tool-call").
    #[serde(rename = "type")]
    pub call_type: String,

    /// The ID of this tool call.
    pub tool_call_id: String,

    /// The name of the tool being called.
    pub tool_name: String,

    /// The input parameters for the tool.
    pub input: INPUT,

    /// Whether the provider executed this tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,

    /// Provider-specific metadata for this tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<ProviderMetadata>,

    /// Whether this is a dynamic tool (false or None for static tools).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,

    /// Whether this is an invalid tool call (false or None for valid static tools).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid: Option<bool>,
}

impl<INPUT> StaticToolCall<INPUT> {
    /// Creates a new static tool call.
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
    /// use ai_sdk_core::StaticToolCall;
    /// use serde_json::json;
    ///
    /// let tool_call = StaticToolCall::new(
    ///     "call_abc123",
    ///     "calculate_sum",
    ///     json!({"a": 5, "b": 3}),
    /// );
    /// ```
    pub fn new(
        tool_call_id: impl Into<String>,
        tool_name: impl Into<String>,
        input: INPUT,
    ) -> Self {
        Self {
            call_type: "tool-call".to_string(),
            tool_call_id: tool_call_id.into(),
            tool_name: tool_name.into(),
            input,
            provider_executed: None,
            provider_metadata: None,
            dynamic: None,
            invalid: None,
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
    pub fn with_provider_metadata(mut self, metadata: ProviderMetadata) -> Self {
        self.provider_metadata = Some(metadata);
        self
    }
}

/// A tool call for a dynamically-typed tool.
///
/// This represents a tool call where the input type is not known at compile time.
/// It can also represent invalid tool calls (unparsable or non-existent tools).
///
/// # Example
///
/// ```
/// use ai_sdk_core::DynamicToolCall;
/// use serde_json::json;
///
/// // Valid dynamic tool call
/// let tool_call = DynamicToolCall::new(
///     "call_456",
///     "unknown_tool",
///     json!({"param": "value"}),
/// );
///
/// // Invalid tool call
/// let invalid_call = DynamicToolCall::new(
///     "call_789",
///     "nonexistent_tool",
///     json!({}),
/// )
/// .with_invalid(true)
/// .with_error(json!({"message": "Tool not found"}));
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicToolCall {
    /// Type discriminator (always "tool-call").
    #[serde(rename = "type")]
    pub call_type: String,

    /// The ID of this tool call.
    pub tool_call_id: String,

    /// The name of the tool being called.
    pub tool_name: String,

    /// The input parameters for the tool (unknown type).
    pub input: Value,

    /// Whether the provider executed this tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,

    /// Provider-specific metadata for this tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<ProviderMetadata>,

    /// Whether this is a dynamic tool (always true).
    pub dynamic: bool,

    /// True if this is caused by an unparsable tool call or a tool that does not exist.
    ///
    /// # Note
    ///
    /// Added into DynamicToolCall to avoid breaking changes.
    /// TODO AI SDK 6: separate into a new InvalidToolCall type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid: Option<bool>,

    /// The error that caused the tool call to be invalid.
    ///
    /// # Note
    ///
    /// TODO AI SDK 6: separate into a new InvalidToolCall type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

impl DynamicToolCall {
    /// Creates a new dynamic tool call.
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
    /// use ai_sdk_core::DynamicToolCall;
    /// use serde_json::json;
    ///
    /// let tool_call = DynamicToolCall::new(
    ///     "call_xyz789",
    ///     "fetch_data",
    ///     json!({"url": "https://api.example.com"}),
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
            dynamic: true,
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
    pub fn with_provider_metadata(mut self, metadata: ProviderMetadata) -> Self {
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

/// A typed tool call that can be either static or dynamic.
///
/// This enum represents the union of `StaticToolCall` and `DynamicToolCall`,
/// similar to TypeScript's union type `StaticToolCall<TOOLS> | DynamicToolCall`.
///
/// # Type Parameters
///
/// * `INPUT` - The input type for static tool calls
///
/// # Example
///
/// ```
/// use ai_sdk_core::{TypedToolCall, StaticToolCall, DynamicToolCall};
/// use serde_json::{json, Value};
///
/// // Create a static tool call
/// let static_call: TypedToolCall<Value> = TypedToolCall::Static(
///     StaticToolCall::new(
///         "call_1",
///         "add",
///         json!({"a": 1, "b": 2}),
///     )
/// );
///
/// // Create a dynamic tool call
/// let dynamic_call: TypedToolCall<Value> = TypedToolCall::Dynamic(
///     DynamicToolCall::new(
///         "call_2",
///         "unknown",
///         json!({}),
///     )
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TypedToolCall<INPUT = Value> {
    /// A statically-typed tool call with known input type.
    Static(StaticToolCall<INPUT>),

    /// A dynamically-typed tool call with unknown input type.
    Dynamic(DynamicToolCall),
}

impl<INPUT> From<StaticToolCall<INPUT>> for TypedToolCall<INPUT> {
    fn from(call: StaticToolCall<INPUT>) -> Self {
        TypedToolCall::Static(call)
    }
}

impl<INPUT> From<DynamicToolCall> for TypedToolCall<INPUT> {
    fn from(call: DynamicToolCall) -> Self {
        TypedToolCall::Dynamic(call)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_static_tool_call_new() {
        let call = StaticToolCall::new(
            "call_123",
            "get_weather",
            json!({"city": "SF"}),
        );

        assert_eq!(call.call_type, "tool-call");
        assert_eq!(call.tool_call_id, "call_123");
        assert_eq!(call.tool_name, "get_weather");
        assert_eq!(call.input, json!({"city": "SF"}));
        assert_eq!(call.provider_executed, None);
        assert_eq!(call.provider_metadata, None);
        assert_eq!(call.dynamic, None);
        assert_eq!(call.invalid, None);
    }

    #[test]
    fn test_static_tool_call_with_provider_executed() {
        let call = StaticToolCall::new(
            "call_123",
            "tool",
            json!({}),
        )
        .with_provider_executed(true);

        assert_eq!(call.provider_executed, Some(true));
    }

    #[test]
    fn test_static_tool_call_with_provider_metadata() {
        use std::collections::HashMap;
        let mut metadata = ProviderMetadata::new();
        let mut inner = HashMap::new();
        inner.insert("key".to_string(), json!("value"));
        metadata.insert("provider".to_string(), inner);

        let call = StaticToolCall::new(
            "call_123",
            "tool",
            json!({}),
        )
        .with_provider_metadata(metadata.clone());

        assert_eq!(call.provider_metadata, Some(metadata));
    }

    #[test]
    fn test_dynamic_tool_call_new() {
        let call = DynamicToolCall::new(
            "call_456",
            "unknown_tool",
            json!({"param": "value"}),
        );

        assert_eq!(call.call_type, "tool-call");
        assert_eq!(call.tool_call_id, "call_456");
        assert_eq!(call.tool_name, "unknown_tool");
        assert_eq!(call.input, json!({"param": "value"}));
        assert_eq!(call.provider_executed, None);
        assert_eq!(call.provider_metadata, None);
        assert_eq!(call.dynamic, true);
        assert_eq!(call.invalid, None);
        assert_eq!(call.error, None);
    }

    #[test]
    fn test_dynamic_tool_call_with_invalid() {
        let call = DynamicToolCall::new(
            "call_456",
            "tool",
            json!({}),
        )
        .with_invalid(true);

        assert_eq!(call.invalid, Some(true));
    }

    #[test]
    fn test_dynamic_tool_call_with_error() {
        let call = DynamicToolCall::new(
            "call_456",
            "tool",
            json!({}),
        )
        .with_error(json!({"message": "Tool not found"}));

        assert_eq!(call.error, Some(json!({"message": "Tool not found"})));
    }

    #[test]
    fn test_dynamic_tool_call_invalid_with_error() {
        let call = DynamicToolCall::new(
            "call_789",
            "nonexistent_tool",
            json!({}),
        )
        .with_invalid(true)
        .with_error(json!({"message": "Tool does not exist"}));

        assert_eq!(call.invalid, Some(true));
        assert_eq!(call.error, Some(json!({"message": "Tool does not exist"})));
    }

    #[test]
    fn test_typed_tool_call_static() {
        let static_call = StaticToolCall::new(
            "call_1",
            "add",
            json!({"a": 1}),
        );

        let typed: TypedToolCall<Value> = TypedToolCall::Static(static_call.clone());

        match typed {
            TypedToolCall::Static(c) => assert_eq!(c, static_call),
            TypedToolCall::Dynamic(_) => panic!("Expected Static variant"),
        }
    }

    #[test]
    fn test_typed_tool_call_dynamic() {
        let dynamic_call = DynamicToolCall::new(
            "call_2",
            "unknown",
            json!({}),
        );

        let typed: TypedToolCall<Value> = TypedToolCall::Dynamic(dynamic_call.clone());

        match typed {
            TypedToolCall::Static(_) => panic!("Expected Dynamic variant"),
            TypedToolCall::Dynamic(c) => assert_eq!(c, dynamic_call),
        }
    }

    #[test]
    fn test_static_tool_call_serialization() {
        let call = StaticToolCall::new(
            "call_123",
            "test_tool",
            json!({"key": "value"}),
        )
        .with_provider_executed(true);

        let serialized = serde_json::to_value(&call).unwrap();

        assert_eq!(serialized["type"], "tool-call");
        assert_eq!(serialized["toolCallId"], "call_123");
        assert_eq!(serialized["toolName"], "test_tool");
        assert_eq!(serialized["input"], json!({"key": "value"}));
        assert_eq!(serialized["providerExecuted"], true);
    }

    #[test]
    fn test_dynamic_tool_call_serialization() {
        let call = DynamicToolCall::new(
            "call_456",
            "unknown_tool",
            json!({"param": "value"}),
        )
        .with_provider_executed(false)
        .with_invalid(true)
        .with_error(json!({"message": "Parse error"}));

        let serialized = serde_json::to_value(&call).unwrap();

        assert_eq!(serialized["type"], "tool-call");
        assert_eq!(serialized["toolCallId"], "call_456");
        assert_eq!(serialized["toolName"], "unknown_tool");
        assert_eq!(serialized["input"], json!({"param": "value"}));
        assert_eq!(serialized["providerExecuted"], false);
        assert_eq!(serialized["dynamic"], true);
        assert_eq!(serialized["invalid"], true);
        assert_eq!(serialized["error"], json!({"message": "Parse error"}));
    }

    #[test]
    fn test_typed_tool_call_serialization() {
        let typed: TypedToolCall<Value> = TypedToolCall::Static(
            StaticToolCall::new(
                "call_1",
                "add",
                json!({"a": 1, "b": 2}),
            ),
        );

        let serialized = serde_json::to_string(&typed).unwrap();
        let deserialized: TypedToolCall<Value> =
            serde_json::from_str(&serialized).unwrap();

        assert_eq!(typed, deserialized);
    }

    #[test]
    fn test_static_tool_call_clone() {
        let call = StaticToolCall::new(
            "call_123",
            "test_tool",
            json!({}),
        );

        let cloned = call.clone();
        assert_eq!(call, cloned);
    }

    #[test]
    fn test_dynamic_tool_call_clone() {
        let call = DynamicToolCall::new(
            "call_456",
            "test_tool",
            json!({}),
        );

        let cloned = call.clone();
        assert_eq!(call, cloned);
    }

    #[test]
    fn test_from_static_tool_call() {
        let static_call = StaticToolCall::new(
            "call_1",
            "tool",
            json!({}),
        );

        let typed: TypedToolCall<Value> = static_call.clone().into();

        match typed {
            TypedToolCall::Static(c) => assert_eq!(c, static_call),
            _ => panic!("Expected Static variant"),
        }
    }

    #[test]
    fn test_from_dynamic_tool_call() {
        let dynamic_call = DynamicToolCall::new(
            "call_2",
            "tool",
            json!({}),
        );

        let typed: TypedToolCall<Value> = dynamic_call.clone().into();

        match typed {
            TypedToolCall::Dynamic(c) => assert_eq!(c, dynamic_call),
            _ => panic!("Expected Dynamic variant"),
        }
    }
}
