use super::{ToolError, ToolResult};
use serde::{Deserialize, Serialize};

/// Output from a tool execution, which can be either a successful result or an error.
///
/// This is a union type that represents the outcome of executing a tool.
///
/// # Example
///
/// ```
/// use llm_kit_provider_utils::tool::{ToolOutput, ToolResult, ToolError};
/// use serde_json::json;
///
/// // Success case
/// let result = ToolResult::new(
///     "call_123",
///     "get_weather",
///     json!({"city": "SF"}),
///     json!({"temperature": 72}),
/// );
/// let output = ToolOutput::Result(result);
///
/// // Error case
/// let error = ToolError::new(
///     "call_456",
///     "get_weather",
///     json!({"city": "Unknown"}),
///     "City not found",
/// );
/// let output = ToolOutput::Error(error);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolOutput {
    /// A successful tool execution result.
    Result(ToolResult),

    /// A tool execution error.
    Error(ToolError),
}

impl ToolOutput {
    /// Creates a new `ToolOutput` from a successful result.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_provider_utils::tool::{ToolOutput, ToolResult};
    /// use serde_json::json;
    ///
    /// let result = ToolResult::new("call_123", "tool", json!({}), json!({}));
    /// let output = ToolOutput::from_result(result);
    /// ```
    pub fn from_result(result: ToolResult) -> Self {
        ToolOutput::Result(result)
    }

    /// Creates a new `ToolOutput` from an error.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_provider_utils::tool::{ToolOutput, ToolError};
    /// use serde_json::json;
    ///
    /// let error = ToolError::new("call_123", "tool", json!({}), "Failed");
    /// let output = ToolOutput::from_error(error);
    /// ```
    pub fn from_error(error: ToolError) -> Self {
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
    pub fn as_result(&self) -> Option<&ToolResult> {
        match self {
            ToolOutput::Result(result) => Some(result),
            _ => None,
        }
    }

    /// Returns a reference to the error if this is an error.
    pub fn as_error(&self) -> Option<&ToolError> {
        match self {
            ToolOutput::Error(error) => Some(error),
            _ => None,
        }
    }

    /// Consumes the `ToolOutput` and returns the result if this is a successful result.
    pub fn into_result(self) -> Option<ToolResult> {
        match self {
            ToolOutput::Result(result) => Some(result),
            _ => None,
        }
    }

    /// Consumes the `ToolOutput` and returns the error if this is an error.
    pub fn into_error(self) -> Option<ToolError> {
        match self {
            ToolOutput::Error(error) => Some(error),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_tool_output_from_result() {
        let result = ToolResult::new(
            "call_123",
            "get_weather",
            json!({"city": "SF"}),
            json!({"temperature": 72}),
        );

        let output = ToolOutput::from_result(result);

        assert!(output.is_result());
        assert!(!output.is_error());
        assert!(output.as_result().is_some());
        assert!(output.as_error().is_none());
    }

    #[test]
    fn test_tool_output_from_error() {
        let error = ToolError::new(
            "call_123",
            "get_weather",
            json!({"city": "Unknown"}),
            "City not found",
        );

        let output = ToolOutput::from_error(error);

        assert!(!output.is_result());
        assert!(output.is_error());
        assert!(output.as_result().is_none());
        assert!(output.as_error().is_some());
    }

    #[test]
    fn test_tool_output_result_variant() {
        let result = ToolResult::new("call_123", "test_tool", json!({}), json!({"status": "ok"}));

        let output = ToolOutput::Result(result);

        match output {
            ToolOutput::Result(_) => {} // Expected
            _ => panic!("Expected Result variant"),
        }
    }

    #[test]
    fn test_tool_output_error_variant() {
        let error = ToolError::new("call_123", "test_tool", json!({}), "Test error");

        let output = ToolOutput::Error(error);

        match output {
            ToolOutput::Error(_) => {} // Expected
            _ => panic!("Expected Error variant"),
        }
    }

    #[test]
    fn test_tool_output_into_result() {
        let result = ToolResult::new("call_123", "test_tool", json!({}), json!({"data": "value"}));

        let output = ToolOutput::from_result(result.clone());
        let extracted = output.into_result();

        assert!(extracted.is_some());
    }

    #[test]
    fn test_tool_output_into_error() {
        let error = ToolError::new("call_123", "test_tool", json!({}), "Error message");

        let output = ToolOutput::from_error(error.clone());
        let extracted = output.into_error();

        assert!(extracted.is_some());
    }

    #[test]
    fn test_tool_output_serialization_result() {
        let result = ToolResult::new(
            "call_123",
            "test_tool",
            json!({"key": "value"}),
            json!({"result": "success"}),
        );

        let output = ToolOutput::from_result(result);
        let serialized = serde_json::to_value(&output).unwrap();

        assert_eq!(serialized["type"], "tool-result");
        assert_eq!(serialized["toolCallId"], "call_123");
    }

    #[test]
    fn test_tool_output_serialization_error() {
        let error = ToolError::new(
            "call_456",
            "test_tool",
            json!({"key": "value"}),
            "Something failed",
        );

        let output = ToolOutput::from_error(error);
        let serialized = serde_json::to_value(&output).unwrap();

        assert_eq!(serialized["type"], "tool-error");
        assert_eq!(serialized["toolCallId"], "call_456");
        assert_eq!(serialized["error"], "Something failed");
    }

    #[test]
    fn test_tool_output_clone() {
        let result = ToolResult::new("call_123", "test_tool", json!({}), json!({}));

        let output = ToolOutput::from_result(result);
        let cloned = output.clone();

        assert_eq!(output, cloned);
    }
}
