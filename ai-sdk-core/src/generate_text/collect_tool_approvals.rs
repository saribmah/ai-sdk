use crate::generate_text::tool_call::{StaticToolCall, TypedToolCall};
use crate::prompt::message::Message;
use crate::prompt::message::{AssistantContent, ToolContentPart};
use crate::prompt::message::tool::{ToolApprovalRequest, ToolApprovalResponse};
use serde_json::Value;
use std::collections::HashMap;

/// A collected tool approval containing the approval request, response, and associated tool call.
///
/// This struct groups together all the information related to a single tool approval:
/// - The original approval request from the assistant
/// - The user's approval response
/// - The tool call that the approval was for
#[derive(Debug, Clone, PartialEq)]
pub struct CollectedToolApproval {
    /// The tool approval request from the assistant message.
    pub approval_request: ToolApprovalRequest,

    /// The user's approval response from the tool message.
    pub approval_response: ToolApprovalResponse,

    /// The tool call that this approval is for.
    /// This is a TypedToolCall to match the TypeScript implementation.
    pub tool_call: TypedToolCall<Value>,
}

impl CollectedToolApproval {
    /// Creates a new collected tool approval.
    pub fn new(
        approval_request: ToolApprovalRequest,
        approval_response: ToolApprovalResponse,
        tool_call: TypedToolCall<Value>,
    ) -> Self {
        Self {
            approval_request,
            approval_response,
            tool_call,
        }
    }
}

/// Result of collecting tool approvals from messages.
#[derive(Debug, Clone, PartialEq)]
pub struct CollectedToolApprovals {
    /// Tool approvals that were granted (approved = true).
    pub approved_tool_approvals: Vec<CollectedToolApproval>,

    /// Tool approvals that were denied (approved = false).
    pub denied_tool_approvals: Vec<CollectedToolApproval>,
}

impl CollectedToolApprovals {
    /// Creates a new empty collected tool approvals result.
    pub fn empty() -> Self {
        Self {
            approved_tool_approvals: Vec::new(),
            denied_tool_approvals: Vec::new(),
        }
    }

    /// Returns true if there are no collected approvals (neither approved nor denied).
    pub fn is_empty(&self) -> bool {
        self.approved_tool_approvals.is_empty() && self.denied_tool_approvals.is_empty()
    }

    /// Returns the total count of all approvals (approved + denied).
    pub fn total_count(&self) -> usize {
        self.approved_tool_approvals.len() + self.denied_tool_approvals.len()
    }
}

/// Collects tool approvals from the message history.
///
/// If the last message is a tool message, this function collects all tool approvals
/// from that message and categorizes them into approved and denied approvals.
///
/// The function:
/// 1. Checks if the last message is a tool message - if not, returns empty results
/// 2. Gathers all tool calls from assistant messages (indexed by tool call ID)
/// 3. Gathers all approval requests from assistant messages (indexed by approval ID)
/// 4. Gathers all tool results from the last tool message
/// 5. For each approval response in the last tool message:
///    - Finds the corresponding approval request and tool call
///    - Skips if there's already a tool result for that tool call
///    - Categorizes into approved or denied based on the approval response
///
/// # Arguments
///
/// * `messages` - The message history to collect approvals from
///
/// # Returns
///
/// A `CollectedToolApprovals` struct containing:
/// - `approved_tool_approvals`: Approvals that were granted
/// - `denied_tool_approvals`: Approvals that were denied
///
/// # Example
///
/// ```rust
/// use ai_sdk_core::generate_text::collect_tool_approvals;
/// use ai_sdk_core::message::ModelMessage;
/// use ai_sdk_core::message::model::{AssistantMessage, ToolMessage, AssistantContent, AssistantContentPart, ToolContentPart};
/// use ai_sdk_core::message::content_parts::ToolCallPart;
/// use ai_sdk_core::message::tool::{ToolApprovalRequest, ToolApprovalResponse};
/// use serde_json::json;
///
/// // Create a message history with an approval request
/// let messages = vec![
///     ModelMessage::Assistant(AssistantMessage::with_parts(vec![
///         AssistantContentPart::ToolCall(ToolCallPart::new(
///             "call_123",
///             "delete_file",
///             json!({"path": "/important/file.txt"}),
///         )),
///         AssistantContentPart::ToolApprovalRequest(
///             ToolApprovalRequest::new("approval_456", "call_123")
///         ),
///     ])),
///     ModelMessage::Tool(ToolMessage::new(vec![
///         ToolContentPart::ApprovalResponse(
///             ToolApprovalResponse::granted("approval_456")
///         ),
///     ])),
/// ];
///
/// // Collect the approvals
/// let result = collect_tool_approvals(&messages);
///
/// // The approval should be in the approved list
/// assert_eq!(result.approved_tool_approvals.len(), 1);
/// assert_eq!(result.denied_tool_approvals.len(), 0);
///
/// // The tool call is available as a TypedToolCall
/// let approval = &result.approved_tool_approvals[0];
/// assert_eq!(approval.approval_response.approved, true);
/// ```
pub fn collect_tool_approvals(messages: &[Message]) -> CollectedToolApprovals {
    // Check if the last message is a tool message
    let last_message = match messages.last() {
        Some(Message::Tool(tool_msg)) => tool_msg,
        _ => return CollectedToolApprovals::empty(),
    };

    // Gather tool calls from assistant messages and create lookup by tool_call_id
    // Convert ToolCallPart to TypedToolCall<Value> (as StaticToolCall since we have the input as Value)
    let mut tool_calls_by_id: HashMap<String, TypedToolCall<Value>> = HashMap::new();
    for message in messages {
        if let Message::Assistant(assistant_msg) = message {
            if let AssistantContent::Parts(parts) = &assistant_msg.content {
                for part in parts {
                    if let crate::prompt::message::assistant::AssistantContentPart::ToolCall(
                        tool_call,
                    ) = part
                    {
                        // Convert ToolCallPart to TypedToolCall<Value>
                        let typed_tool_call = TypedToolCall::Static(
                            StaticToolCall::new(
                                tool_call.tool_call_id.clone(),
                                tool_call.tool_name.clone(),
                                tool_call.input.clone(),
                            )
                            .with_provider_executed(tool_call.provider_executed.unwrap_or(false)),
                        );
                        tool_calls_by_id.insert(tool_call.tool_call_id.clone(), typed_tool_call);
                    }
                }
            }
        }
    }

    // Gather approval requests from assistant messages and create lookup by approval_id
    let mut approval_requests_by_id: HashMap<String, ToolApprovalRequest> = HashMap::new();
    for message in messages {
        if let Message::Assistant(assistant_msg) = message {
            if let AssistantContent::Parts(parts) = &assistant_msg.content {
                for part in parts {
                    if let crate::prompt::message::assistant::AssistantContentPart::ToolApprovalRequest(
                        approval_req,
                    ) = part
                    {
                        approval_requests_by_id
                            .insert(approval_req.approval_id.clone(), approval_req.clone());
                    }
                }
            }
        }
    }

    // Gather tool results from the last tool message and create a set of tool_call_ids
    let mut tool_result_ids: HashMap<String, ()> = HashMap::new();
    for part in &last_message.content {
        if let ToolContentPart::ToolResult(tool_result) = part {
            tool_result_ids.insert(tool_result.tool_call_id.clone(), ());
        }
    }

    // Collect approval responses from the last tool message
    let mut approved_tool_approvals: Vec<CollectedToolApproval> = Vec::new();
    let mut denied_tool_approvals: Vec<CollectedToolApproval> = Vec::new();

    for part in &last_message.content {
        if let ToolContentPart::ApprovalResponse(approval_response) = part {
            // Find the corresponding approval request
            let approval_request = match approval_requests_by_id.get(&approval_response.approval_id)
            {
                Some(req) => req,
                None => continue, // Skip if no matching approval request
            };

            // Skip if there's already a tool result for this tool call
            if tool_result_ids.contains_key(&approval_request.tool_call_id) {
                continue;
            }

            // Find the corresponding tool call
            let tool_call = match tool_calls_by_id.get(&approval_request.tool_call_id) {
                Some(call) => call,
                None => continue, // Skip if no matching tool call
            };

            // Create the collected approval
            let collected = CollectedToolApproval::new(
                approval_request.clone(),
                approval_response.clone(),
                tool_call.clone(),
            );

            // Categorize based on whether it was approved or denied
            if approval_response.approved {
                approved_tool_approvals.push(collected);
            } else {
                denied_tool_approvals.push(collected);
            }
        }
    }

    CollectedToolApprovals {
        approved_tool_approvals,
        denied_tool_approvals,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::content_parts::{ToolCallPart, ToolResultOutput, ToolResultPart};
    use crate::prompt::message::{
        AssistantContentPart, AssistantMessage, ToolMessage,
    };
    use serde_json::json;

    #[test]
    fn test_collect_tool_approvals_empty_messages() {
        let messages: Vec<Message> = vec![];
        let result = collect_tool_approvals(&messages);

        assert!(result.is_empty());
        assert_eq!(result.approved_tool_approvals.len(), 0);
        assert_eq!(result.denied_tool_approvals.len(), 0);
    }

    #[test]
    fn test_collect_tool_approvals_non_tool_last_message() {
        let messages = vec![Message::Assistant(AssistantMessage::new("Hello"))];

        let result = collect_tool_approvals(&messages);

        assert!(result.is_empty());
    }

    #[test]
    fn test_collect_tool_approvals_approved() {
        let messages = vec![
            Message::Assistant(AssistantMessage::with_parts(vec![
                AssistantContentPart::ToolCall(ToolCallPart::new(
                    "call_123",
                    "delete_file",
                    json!({"path": "/file.txt"}),
                )),
                AssistantContentPart::ToolApprovalRequest(ToolApprovalRequest::new(
                    "approval_456",
                    "call_123",
                )),
            ])),
            Message::Tool(ToolMessage::new(vec![
                ToolContentPart::ApprovalResponse(ToolApprovalResponse::granted("approval_456")),
            ])),
        ];

        let result = collect_tool_approvals(&messages);

        assert_eq!(result.approved_tool_approvals.len(), 1);
        assert_eq!(result.denied_tool_approvals.len(), 0);

        let approval = &result.approved_tool_approvals[0];
        assert_eq!(approval.approval_request.approval_id, "approval_456");
        assert_eq!(approval.approval_response.approved, true);

        // Verify the tool call (it should be a StaticToolCall)
        match &approval.tool_call {
            TypedToolCall::Static(static_call) => {
                assert_eq!(static_call.tool_call_id, "call_123");
                assert_eq!(static_call.tool_name, "delete_file");
            }
            _ => panic!("Expected StaticToolCall"),
        }
    }

    #[test]
    fn test_collect_tool_approvals_denied() {
        let messages = vec![
            Message::Assistant(AssistantMessage::with_parts(vec![
                AssistantContentPart::ToolCall(ToolCallPart::new(
                    "call_123",
                    "delete_database",
                    json!({"name": "prod"}),
                )),
                AssistantContentPart::ToolApprovalRequest(ToolApprovalRequest::new(
                    "approval_456",
                    "call_123",
                )),
            ])),
            Message::Tool(ToolMessage::new(vec![
                ToolContentPart::ApprovalResponse(
                    ToolApprovalResponse::denied("approval_456").with_reason("Too dangerous"),
                ),
            ])),
        ];

        let result = collect_tool_approvals(&messages);

        assert_eq!(result.approved_tool_approvals.len(), 0);
        assert_eq!(result.denied_tool_approvals.len(), 1);

        let denial = &result.denied_tool_approvals[0];
        assert_eq!(denial.approval_request.approval_id, "approval_456");
        assert_eq!(denial.approval_response.approved, false);

        // Verify the tool call
        match &denial.tool_call {
            TypedToolCall::Static(static_call) => {
                assert_eq!(static_call.tool_call_id, "call_123");
                assert_eq!(static_call.tool_name, "delete_database");
            }
            _ => panic!("Expected StaticToolCall"),
        }
    }

    #[test]
    fn test_collect_tool_approvals_mixed() {
        let messages = vec![
            Message::Assistant(AssistantMessage::with_parts(vec![
                AssistantContentPart::ToolCall(ToolCallPart::new(
                    "call_1",
                    "read_file",
                    json!({"path": "/data.txt"}),
                )),
                AssistantContentPart::ToolApprovalRequest(ToolApprovalRequest::new(
                    "approval_1",
                    "call_1",
                )),
                AssistantContentPart::ToolCall(ToolCallPart::new(
                    "call_2",
                    "delete_file",
                    json!({"path": "/important.txt"}),
                )),
                AssistantContentPart::ToolApprovalRequest(ToolApprovalRequest::new(
                    "approval_2",
                    "call_2",
                )),
            ])),
            Message::Tool(ToolMessage::new(vec![
                ToolContentPart::ApprovalResponse(ToolApprovalResponse::granted("approval_1")),
                ToolContentPart::ApprovalResponse(ToolApprovalResponse::denied("approval_2")),
            ])),
        ];

        let result = collect_tool_approvals(&messages);

        assert_eq!(result.approved_tool_approvals.len(), 1);
        assert_eq!(result.denied_tool_approvals.len(), 1);
        assert_eq!(result.total_count(), 2);
    }

    #[test]
    fn test_collect_tool_approvals_skips_with_existing_result() {
        let messages = vec![
            Message::Assistant(AssistantMessage::with_parts(vec![
                AssistantContentPart::ToolCall(ToolCallPart::new(
                    "call_123",
                    "some_tool",
                    json!({}),
                )),
                AssistantContentPart::ToolApprovalRequest(ToolApprovalRequest::new(
                    "approval_456",
                    "call_123",
                )),
            ])),
            Message::Tool(ToolMessage::new(vec![
                ToolContentPart::ToolResult(ToolResultPart::new(
                    "call_123",
                    "some_tool",
                    ToolResultOutput::text("Already executed"),
                )),
                ToolContentPart::ApprovalResponse(ToolApprovalResponse::granted("approval_456")),
            ])),
        ];

        let result = collect_tool_approvals(&messages);

        // Should be empty because there's already a tool result for this call
        assert!(result.is_empty());
    }

    #[test]
    fn test_collect_tool_approvals_multiple_messages() {
        let messages = vec![
            Message::Assistant(AssistantMessage::with_parts(vec![
                AssistantContentPart::ToolCall(ToolCallPart::new("call_1", "tool_a", json!({}))),
                AssistantContentPart::ToolApprovalRequest(ToolApprovalRequest::new(
                    "approval_1",
                    "call_1",
                )),
            ])),
            Message::Tool(ToolMessage::new(vec![
                ToolContentPart::ApprovalResponse(ToolApprovalResponse::granted("approval_1")),
            ])),
            Message::Assistant(AssistantMessage::with_parts(vec![
                AssistantContentPart::ToolCall(ToolCallPart::new("call_2", "tool_b", json!({}))),
                AssistantContentPart::ToolApprovalRequest(ToolApprovalRequest::new(
                    "approval_2",
                    "call_2",
                )),
            ])),
            Message::Tool(ToolMessage::new(vec![
                ToolContentPart::ApprovalResponse(ToolApprovalResponse::denied("approval_2")),
            ])),
        ];

        let result = collect_tool_approvals(&messages);

        // Only the last tool message is checked, so only approval_2 should be collected
        assert_eq!(result.approved_tool_approvals.len(), 0);
        assert_eq!(result.denied_tool_approvals.len(), 1);
        assert_eq!(
            result.denied_tool_approvals[0].approval_request.approval_id,
            "approval_2"
        );
    }

    #[test]
    fn test_collected_tool_approvals_empty() {
        let result = CollectedToolApprovals::empty();
        assert!(result.is_empty());
        assert_eq!(result.total_count(), 0);
    }

    #[test]
    fn test_collected_tool_approval_new() {
        let approval_request = ToolApprovalRequest::new("approval_123", "call_456");
        let approval_response = ToolApprovalResponse::granted("approval_123");
        let tool_call =
            TypedToolCall::Static(StaticToolCall::new("call_456", "test_tool", json!({})));

        let collected = CollectedToolApproval::new(
            approval_request.clone(),
            approval_response.clone(),
            tool_call.clone(),
        );

        assert_eq!(collected.approval_request, approval_request);
        assert_eq!(collected.approval_response, approval_response);
        assert_eq!(collected.tool_call, tool_call);
    }
}
