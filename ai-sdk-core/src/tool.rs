/// Tool approval request types.
pub mod approval_request;
/// Tool approval response types.
pub mod approval_response;
/// Collection of tool approval requests.
pub mod collect_tool_approvals;
/// Tool definition and execution logic.
pub mod definition;
/// Helper to check if tool approval is needed.
pub mod is_approval_needed;
/// Options for tool execution.
pub mod options;
/// Parsing of tool calls from provider responses.
pub mod parse_tool_call;
/// Preparation of tools and tool choice for provider calls.
pub mod prepare_tools;
/// Output type for tool approval requests.
pub mod tool_approval_request_output;
/// Tool call type representing a function invocation.
pub mod tool_call;
/// Tool call repair functions for fixing malformed inputs.
pub mod tool_call_repair_function;
/// Tool error type for failed tool executions.
pub mod tool_error;
/// Tool output type (either result or error).
pub mod tool_output;
/// Tool result type for successful tool executions.
pub mod tool_result;
/// Tool set collection type.
pub mod tool_set;
/// Type-safe tool trait with compile-time schema validation.
pub mod type_safe;

pub use approval_request::ToolApprovalRequest;
pub use approval_response::ToolApprovalResponse;
pub use collect_tool_approvals::{
    CollectedToolApproval, CollectedToolApprovals, collect_tool_approvals,
};
pub use definition::{
    NeedsApproval, OnPreliminaryToolResult, Tool, ToolExecuteFunction, ToolExecutionOutput,
    ToolNeedsApprovalFunction, ToolType, execute_tool_call,
};
pub use is_approval_needed::is_approval_needed;
pub use options::ToolExecuteOptions;
pub use parse_tool_call::{parse_provider_executed_dynamic_tool_call, parse_tool_call};
pub use prepare_tools::prepare_tools_and_tool_choice;
pub use tool_approval_request_output::ToolApprovalRequestOutput;
pub use tool_call::ToolCall;
pub use tool_call_repair_function::{ToolCallRepairFunction, ToolCallRepairOptions, no_repair};
pub use tool_error::ToolError;
pub use tool_output::ToolOutput;
pub use tool_result::ToolResult;
pub use tool_set::ToolSet;
pub use type_safe::TypeSafeTool;
