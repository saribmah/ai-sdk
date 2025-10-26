use super::tool_error::TypedToolError;
use super::tool_result::TypedToolResult;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Output from a tool execution, which can be either a successful result or an error.
///
/// This is a union type that represents the outcome of executing a tool.
///
/// # Example
///
/// ```
/// use ai_sdk_core::generate_text::tool_output::ToolOutput;
/// use ai_sdk_core::generate_text::tool_result::StaticToolResult;
/// use ai_sdk_core::generate_text::tool_error::StaticToolError;
/// use serde_json::{json, Value};
///
/// // Success case
/// let result = StaticToolResult::new(
///     "call_123",
///     "get_weather",
///     json!({"city": "SF"}),
///     json!({"temperature": 72}),
/// );
/// let output = ToolOutput::Result(result.into());
///
/// // Error case
/// let error = StaticToolError::new(
///     "call_456",
///     "get_weather",
///     json!({"city": "Unknown"}),
///     "City not found",
/// );
/// let output: ToolOutput<Value, Value> = ToolOutput::Error(error.into());
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolOutput<INPUT = Value, OUTPUT = Value> {
    /// A successful tool execution result.
    Result(TypedToolResult<INPUT, OUTPUT>),

    /// A tool execution error.
    Error(TypedToolError<INPUT>),
}

impl<INPUT, OUTPUT> ToolOutput<INPUT, OUTPUT> {
    /// Creates a new `ToolOutput` from a successful result.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use ai_sdk_core::generate_text::tool_output::ToolOutput;
    ///
    /// let output = ToolOutput::from_result(result);
    /// ```
    pub fn from_result(result: TypedToolResult<INPUT, OUTPUT>) -> Self {
        ToolOutput::Result(result)
    }

    /// Creates a new `ToolOutput` from an error.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use ai_sdk_core::generate_text::tool_output::ToolOutput;
    ///
    /// let output = ToolOutput::from_error(error);
    /// ```
    pub fn from_error(error: TypedToolError<INPUT>) -> Self {
        ToolOutput::Error(error)
    }

    /// Returns `true` if this is a successful result.
    pub fn is_result(&self) -> bool {
        matches!(self, ToolOutput::Result(_))
    }

    /// Returns `true` if this is an error.
    pub fn is_error(&self) -> bool {
        matches!(self, ToolOutput::Error(_))
    }

    /// Returns a reference to the result if this is a successful result.
    pub fn as_result(&self) -> Option<&TypedToolResult<INPUT, OUTPUT>> {
        match self {
            ToolOutput::Result(result) => Some(result),
            _ => None,
        }
    }

    /// Returns a reference to the error if this is an error.
    pub fn as_error(&self) -> Option<&TypedToolError<INPUT>> {
        match self {
            ToolOutput::Error(error) => Some(error),
            _ => None,
        }
    }

    /// Consumes the `ToolOutput` and returns the result if this is a successful result.
    pub fn into_result(self) -> Option<TypedToolResult<INPUT, OUTPUT>> {
        match self {
            ToolOutput::Result(result) => Some(result),
            _ => None,
        }
    }

    /// Consumes the `ToolOutput` and returns the error if this is an error.
    pub fn into_error(self) -> Option<TypedToolError<INPUT>> {
        match self {
            ToolOutput::Error(error) => Some(error),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_text::tool_error::StaticToolError;
    use crate::generate_text::tool_result::StaticToolResult;
    use serde_json::json;

    #[test]
    fn test_tool_output_from_result() {
        let result = StaticToolResult::new(
            "call_123",
            "get_weather",
            json!({"city": "SF"}),
            json!({"temperature": 72}),
        );

        let output = ToolOutput::from_result(result.into());

        assert!(output.is_result());
        assert!(!output.is_error());
        assert!(output.as_result().is_some());
        assert!(output.as_error().is_none());
    }

    #[test]
    fn test_tool_output_from_error() {
        let error = StaticToolError::new(
            "call_123",
            "get_weather",
            json!({"city": "Unknown"}),
            "City not found",
        );

        let output: ToolOutput<Value, Value> = ToolOutput::from_error(error.into());

        assert!(!output.is_result());
        assert!(output.is_error());
        assert!(output.as_result().is_none());
        assert!(output.as_error().is_some());
    }

    #[test]
    fn test_tool_output_result_variant() {
        let result = StaticToolResult::new(
            "call_123",
            "test_tool",
            json!({}),
            json!({"status": "ok"}),
        );

        let output = ToolOutput::Result(result.into());

        match output {
            ToolOutput::Result(_) => {} // Expected
            _ => panic!("Expected Result variant"),
        }
    }

    #[test]
    fn test_tool_output_error_variant() {
        let error = StaticToolError::new(
            "call_123",
            "test_tool",
            json!({}),
            "Test error",
        );

        let output: ToolOutput<Value, Value> = ToolOutput::Error(error.into());

        match output {
            ToolOutput::Error(_) => {} // Expected
            _ => panic!("Expected Error variant"),
        }
    }

    #[test]
    fn test_tool_output_into_result() {
        let result = StaticToolResult::new(
            "call_123",
            "test_tool",
            json!({}),
            json!({"data": "value"}),
        );

        let output = ToolOutput::from_result(result.clone().into());
        let extracted = output.into_result();

        assert!(extracted.is_some());
    }

    #[test]
    fn test_tool_output_into_error() {
        let error = StaticToolError::new(
            "call_123",
            "test_tool",
            json!({}),
            "Error message",
        );

        let output: ToolOutput<Value, Value> = ToolOutput::from_error(error.clone().into());
        let extracted = output.into_error();

        assert!(extracted.is_some());
    }

    #[test]
    fn test_tool_output_serialization_result() {
        let result = StaticToolResult::new(
            "call_123",
            "test_tool",
            json!({"key": "value"}),
            json!({"result": "success"}),
        );

        let output: ToolOutput = ToolOutput::from_result(result.into());
        let serialized = serde_json::to_value(&output).unwrap();

        assert_eq!(serialized["type"], "tool-result");
        assert_eq!(serialized["toolCallId"], "call_123");
    }

    #[test]
    fn test_tool_output_serialization_error() {
        let error = StaticToolError::new(
            "call_456",
            "test_tool",
            json!({"key": "value"}),
            "Something failed",
        );

        let output: ToolOutput = ToolOutput::from_error(error.into());
        let serialized = serde_json::to_value(&output).unwrap();

        assert_eq!(serialized["type"], "tool-error");
        assert_eq!(serialized["toolCallId"], "call_456");
        assert_eq!(serialized["error"], "Something failed");
    }

    #[test]
    fn test_tool_output_clone() {
        let result = StaticToolResult::new(
            "call_123",
            "test_tool",
            json!({}),
            json!({}),
        );

        let output = ToolOutput::from_result(result.into());
        let cloned = output.clone();

        assert_eq!(output, cloned);
    }
}
