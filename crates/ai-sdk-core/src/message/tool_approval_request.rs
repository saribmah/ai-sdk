/// Tool approval request prompt part.
///
/// This represents a request for user approval before executing a tool call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolApprovalRequest {
    /// ID of the tool approval.
    pub approval_id: String,

    /// ID of the tool call that the approval request is for.
    pub tool_call_id: String,
}

impl ToolApprovalRequest {
    /// Creates a new tool approval request.
    pub fn new(approval_id: impl Into<String>, tool_call_id: impl Into<String>) -> Self {
        Self {
            approval_id: approval_id.into(),
            tool_call_id: tool_call_id.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_approval_request_new() {
        let request = ToolApprovalRequest::new("approval_123", "call_456");

        assert_eq!(request.approval_id, "approval_123");
        assert_eq!(request.tool_call_id, "call_456");
    }

    #[test]
    fn test_tool_approval_request_clone() {
        let request = ToolApprovalRequest::new("approval_123", "call_456");
        let cloned = request.clone();

        assert_eq!(request, cloned);
    }
}
