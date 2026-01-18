/// Collection of tool approval requests.
pub mod collect_tool_approvals;
/// Execute tool call function
pub mod execute_tool_call;
/// Helper to check if tool approval is needed.
pub mod is_approval_needed;
/// Parsing of tool calls from provider responses.
pub mod parse_tool_call;
/// Preparation of tools and tool choice for provider calls.
pub mod prepare_tools;
/// Tool call repair functions for fixing malformed inputs.
pub mod repair_function;
/// Tool set collection type.
pub mod tool_set;
/// Type-safe tool trait with compile-time schema validation.
pub mod type_safe;

pub use collect_tool_approvals::{
    CollectedToolApproval, CollectedToolApprovals, collect_tool_approvals,
};
pub use execute_tool_call::execute_tool_call;
pub use is_approval_needed::is_approval_needed;
pub use parse_tool_call::{parse_provider_executed_dynamic_tool_call, parse_tool_call};
pub use prepare_tools::prepare_tools_and_tool_choice;
pub use repair_function::{ToolCallRepairFunction, ToolCallRepairOptions, no_repair};
pub use tool_set::ToolSet;
pub use type_safe::TypeSafeTool;
