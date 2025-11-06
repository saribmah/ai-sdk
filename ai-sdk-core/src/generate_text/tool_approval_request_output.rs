use crate::tool::TypedToolCall;
use serde::{Deserialize, Serialize};

/// Output part that indicates that a tool approval request has been made.
///
/// The tool approval request can be approved or denied in the next tool message.
///
/// # Type Parameters
///
/// * `INPUT` - The input type for the tool call
///
/// # Example
///
/// ```
/// use ai_sdk_core::generate_text::{ToolApprovalRequestOutput, TypedToolCall, StaticToolCall};
/// use serde_json::json;
///
/// let tool_call = TypedToolCall::Static(StaticToolCall::new(
///     "call_123",
///     "dangerous_operation",
///     json!({"action": "delete"}),
/// ));
///
/// let approval_request = ToolApprovalRequestOutput::new(
///     "approval_456",
///     tool_call,
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ToolApprovalRequestOutput<INPUT> {
    /// Type discriminator (always "tool-approval-request").
    #[serde(rename = "type")]
    pub output_type: String,

    /// ID of the tool approval request.
    pub approval_id: String,

    /// Tool call that the approval request is for.
    pub tool_call: TypedToolCall<INPUT>,
}

impl<INPUT> ToolApprovalRequestOutput<INPUT> {
    /// Creates a new tool approval request output.
    ///
    /// # Arguments
    ///
    /// * `approval_id` - The ID of the approval request
    /// * `tool_call` - The tool call that needs approval
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_core::generate_text::{ToolApprovalRequestOutput, TypedToolCall, StaticToolCall};
    /// use serde_json::json;
    ///
    /// let tool_call = TypedToolCall::Static(StaticToolCall::new(
    ///     "call_123",
    ///     "delete_file",
    ///     json!({"filename": "important.txt"}),
    /// ));
    ///
    /// let approval_request = ToolApprovalRequestOutput::new(
    ///     "approval_789",
    ///     tool_call,
    /// );
    ///
    /// assert_eq!(approval_request.approval_id, "approval_789");
    /// ```
    pub fn new(approval_id: impl Into<String>, tool_call: TypedToolCall<INPUT>) -> Self {
        Self {
            output_type: "tool-approval-request".to_string(),
            approval_id: approval_id.into(),
            tool_call,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool::{DynamicToolCall, StaticToolCall, TypedToolCall};
    use serde_json::{Value, json};

    #[test]
    fn test_new_with_static_tool_call() {
        let tool_call = TypedToolCall::Static(StaticToolCall::new(
            "call_123",
            "get_weather",
            json!({"city": "SF"}),
        ));

        let approval_request = ToolApprovalRequestOutput::new("approval_456", tool_call.clone());

        assert_eq!(approval_request.output_type, "tool-approval-request");
        assert_eq!(approval_request.approval_id, "approval_456");
        assert_eq!(approval_request.tool_call, tool_call);
    }

    #[test]
    fn test_new_with_dynamic_tool_call() {
        let tool_call: TypedToolCall<Value> = TypedToolCall::Dynamic(DynamicToolCall::new(
            "call_789",
            "unknown_tool",
            json!({"param": "value"}),
        ));

        let approval_request = ToolApprovalRequestOutput::new("approval_abc", tool_call.clone());

        assert_eq!(approval_request.output_type, "tool-approval-request");
        assert_eq!(approval_request.approval_id, "approval_abc");
        assert_eq!(approval_request.tool_call, tool_call);
    }

    #[test]
    fn test_clone() {
        let tool_call = TypedToolCall::Static(StaticToolCall::new(
            "call_123",
            "delete_file",
            json!({"filename": "test.txt"}),
        ));

        let approval_request = ToolApprovalRequestOutput::new("approval_1", tool_call);
        let cloned = approval_request.clone();

        assert_eq!(approval_request, cloned);
    }

    #[test]
    fn test_serialization() {
        let tool_call = TypedToolCall::Static(StaticToolCall::new(
            "call_123",
            "dangerous_operation",
            json!({"action": "delete"}),
        ));

        let approval_request = ToolApprovalRequestOutput::new("approval_xyz", tool_call);
        let serialized = serde_json::to_value(&approval_request).unwrap();

        assert_eq!(serialized["type"], "tool-approval-request");
        assert_eq!(serialized["approvalId"], "approval_xyz");
        assert!(serialized["toolCall"].is_object());
    }

    #[test]
    fn test_deserialization() {
        let json_data = json!({
            "type": "tool-approval-request",
            "approvalId": "approval_123",
            "toolCall": {
                "type": "tool-call",
                "toolCallId": "call_456",
                "toolName": "test_tool",
                "input": {"key": "value"}
            }
        });

        let approval_request: ToolApprovalRequestOutput<Value> =
            serde_json::from_value(json_data).unwrap();

        assert_eq!(approval_request.output_type, "tool-approval-request");
        assert_eq!(approval_request.approval_id, "approval_123");
    }

    #[test]
    fn test_equality() {
        let tool_call1 =
            TypedToolCall::Static(StaticToolCall::new("call_123", "test", json!({"a": 1})));
        let tool_call2 =
            TypedToolCall::Static(StaticToolCall::new("call_123", "test", json!({"a": 1})));

        let request1 = ToolApprovalRequestOutput::new("approval_1", tool_call1);
        let request2 = ToolApprovalRequestOutput::new("approval_1", tool_call2);

        assert_eq!(request1, request2);
    }

    #[test]
    fn test_inequality_different_approval_id() {
        let tool_call1 =
            TypedToolCall::Static(StaticToolCall::new("call_123", "test", json!({"a": 1})));
        let tool_call2 =
            TypedToolCall::Static(StaticToolCall::new("call_123", "test", json!({"a": 1})));

        let request1 = ToolApprovalRequestOutput::new("approval_1", tool_call1);
        let request2 = ToolApprovalRequestOutput::new("approval_2", tool_call2);

        assert_ne!(request1, request2);
    }

    #[test]
    fn test_inequality_different_tool_call() {
        let tool_call1 =
            TypedToolCall::Static(StaticToolCall::new("call_123", "test", json!({"a": 1})));
        let tool_call2 =
            TypedToolCall::Static(StaticToolCall::new("call_456", "test", json!({"a": 1})));

        let request1 = ToolApprovalRequestOutput::new("approval_1", tool_call1);
        let request2 = ToolApprovalRequestOutput::new("approval_1", tool_call2);

        assert_ne!(request1, request2);
    }
}
