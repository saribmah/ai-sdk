use super::ToolResult;
use super::execute_options::ToolExecuteOptions;
use crate::message::content_parts::ToolResultOutput;
use futures_util::Stream;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// The output from a tool execution, which can be either a single value or a stream of values.
///
/// Tools can return `Result<OUTPUT, Value>` to surface errors. When `Err(error_value)` is returned,
/// the error will be converted to a `ToolError` and returned to the model.
pub enum ToolExecutionOutput<OUTPUT>
where
    OUTPUT: Send + 'static,
{
    /// A single output value (non-streaming).
    /// Returns `Ok(output)` on success or `Err(error)` on failure.
    Single(Pin<Box<dyn Future<Output = Result<OUTPUT, Value>> + Send>>),

    /// A stream of output values (streaming).
    /// Each item is `Ok(output)` on success or `Err(error)` on failure.
    /// The first error terminates the stream and is returned as a `ToolError`.
    Streaming(Pin<Box<dyn Stream<Item = Result<OUTPUT, Value>> + Send>>),
}

/// Function that executes a tool with the given input and options.
///
/// Returns either a Future that resolves to a single output, or a Stream of outputs.
/// Wrapped in Arc to allow cloning.
pub type ToolExecuteFunction<INPUT, OUTPUT> =
    Arc<dyn Fn(INPUT, ToolExecuteOptions) -> ToolExecutionOutput<OUTPUT> + Send + Sync>;

/// Function that determines if a tool needs approval before execution.
/// Wrapped in Arc to allow cloning.
pub type ToolNeedsApprovalFunction<INPUT> = Arc<
    dyn Fn(INPUT, ToolExecuteOptions) -> Pin<Box<dyn Future<Output = bool> + Send>> + Send + Sync,
>;

/// Callback that is called when tool input streaming starts.
/// Wrapped in Arc to allow cloning.
pub type OnInputStartCallback =
    Arc<dyn Fn(ToolExecuteOptions) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Callback that is called when a tool input delta is available.
/// Wrapped in Arc to allow cloning.
pub type OnInputDeltaCallback = Arc<
    dyn Fn(String, ToolExecuteOptions) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync,
>;

/// Callback that is called when tool input is available.
/// Wrapped in Arc to allow cloning.
pub type OnInputAvailableCallback<INPUT> = Arc<
    dyn Fn(INPUT, ToolExecuteOptions) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync,
>;

/// Function that converts tool output to model output format.
/// Wrapped in Arc to allow cloning.
pub type ToModelOutputFunction<OUTPUT> = Arc<dyn Fn(OUTPUT) -> ToolResultOutput + Send + Sync>;

/// Type of tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToolType {
    /// User-defined function tool.
    #[serde(rename = "function")]
    Function,

    /// Tool that is defined at runtime (e.g. an MCP tool).
    Dynamic,

    /// Tool with provider-defined input and output schemas.
    ProviderDefined {
        /// The ID of the tool. Should follow the format `<provider-name>.<unique-tool-name>`.
        id: String,

        /// The name of the tool that the user must use in the tool set.
        name: String,

        /// The arguments for configuring the tool.
        args: HashMap<String, Value>,
    },
}

/// Whether a tool needs approval before execution.
#[derive(Clone)]
pub enum NeedsApproval {
    /// Tool does not need approval.
    No,

    /// Tool always needs approval.
    Yes,

    /// Tool needs approval based on a function.
    Function(ToolNeedsApprovalFunction<Value>),
}

/// Callback function for preliminary tool results during streaming.
pub type OnPreliminaryToolResult = Arc<dyn Fn(ToolResult) + Send + Sync>;
