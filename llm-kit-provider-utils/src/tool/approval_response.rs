use serde::{Deserialize, Serialize};

/// Tool approval response prompt part.
///
/// This represents the user's response to a tool approval request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolApprovalResponse {
    /// ID of the tool approval.
    #[serde(rename = "approvalId")]
    pub approval_id: String,

    /// Flag indicating whether the approval was granted or denied.
    pub approved: bool,

    /// Optional reason for the approval or denial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl ToolApprovalResponse {
    /// Creates a new tool approval response.
    pub fn new(approval_id: impl Into<String>, approved: bool) -> Self {
        Self {
            approval_id: approval_id.into(),
            approved,
            reason: None,
        }
    }

    /// Creates a tool approval response that grants approval.
    pub fn granted(approval_id: impl Into<String>) -> Self {
        Self::new(approval_id, true)
    }

    /// Creates a tool approval response that denies approval.
    pub fn denied(approval_id: impl Into<String>) -> Self {
        Self::new(approval_id, false)
    }

    /// Sets the reason for the approval or denial.
    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_approval_response_new() {
        let response = ToolApprovalResponse::new("approval_123", true);

        assert_eq!(response.approval_id, "approval_123");
        assert!(response.approved);
        assert!(response.reason.is_none());
    }

    #[test]
    fn test_tool_approval_response_granted() {
        let response = ToolApprovalResponse::granted("approval_123");

        assert_eq!(response.approval_id, "approval_123");
        assert!(response.approved);
    }

    #[test]
    fn test_tool_approval_response_denied() {
        let response = ToolApprovalResponse::denied("approval_123");

        assert_eq!(response.approval_id, "approval_123");
        assert!(!response.approved);
    }

    #[test]
    fn test_tool_approval_response_with_reason() {
        let response =
            ToolApprovalResponse::granted("approval_123").with_reason("User confirmed action");

        assert_eq!(response.reason, Some("User confirmed action".to_string()));
    }

    #[test]
    fn test_tool_approval_response_denied_with_reason() {
        let response =
            ToolApprovalResponse::denied("approval_123").with_reason("Insufficient permissions");

        assert!(!response.approved);
        assert_eq!(
            response.reason,
            Some("Insufficient permissions".to_string())
        );
    }

    #[test]
    fn test_tool_approval_response_clone() {
        let response = ToolApprovalResponse::granted("approval_123").with_reason("Approved");
        let cloned = response.clone();

        assert_eq!(response, cloned);
    }
}
