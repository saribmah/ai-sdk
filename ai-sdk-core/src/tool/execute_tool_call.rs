use super::ToolSet;
use llm_kit_provider_utils::message::Message;
use llm_kit_provider_utils::tool::{
    OnPreliminaryToolResult, ToolCall, ToolError, ToolExecuteOptions, ToolOutput, ToolResult,
};
use serde_json::Value;
use tokio_util::sync::CancellationToken;

/// Executes a tool call and returns the output or error.
///
/// This function takes a tool call, looks up the tool in the tool set,
/// executes it with the provided options, and returns the result or error.
///
/// For streaming tools, it handles preliminary results through the callback.
///
/// # Arguments
///
/// * `tool_call` - The tool call to execute
/// * `tools` - The tool set containing tool definitions
/// * `messages` - The conversation messages for context
/// * `abort_signal` - Optional cancellation token to abort execution
/// * `experimental_context` - Optional experimental context data
/// * `on_preliminary_tool_result` - Optional callback for preliminary streaming results
///
/// # Returns
///
/// Returns `Some(ToolOutput)` if the tool was executed (successfully or with error),
/// or `None` if the tool has no execute function.
///
/// # Example
///
/// ```no_run
/// use llm_kit_core::tool::{execute_tool_call, ToolSet};
/// use llm_kit_provider_utils::tool::ToolCall;
/// use llm_kit_provider_utils::message::Message;
/// use tokio_util::sync::CancellationToken;
///
/// # async fn example(tool_call: ToolCall, tools: ToolSet, messages: Vec<Message>, abort_signal: CancellationToken) {
/// let output = execute_tool_call(
///     tool_call,
///     &tools,
///     messages,
///     Some(abort_signal),
///     None,
///     None,
/// ).await;
/// # }
/// ```
pub async fn execute_tool_call(
    tool_call: ToolCall,
    tools: &ToolSet,
    messages: Vec<Message>,
    abort_signal: Option<CancellationToken>,
    experimental_context: Option<Value>,
    on_preliminary_tool_result: Option<OnPreliminaryToolResult>,
) -> Option<ToolOutput> {
    // Extract tool call information
    let tool_call_id = tool_call.tool_call_id.clone();
    let tool_name = tool_call.tool_name.clone();
    let input = tool_call.input.clone();

    // Look up the tool
    let tool = match tools.get(&tool_name) {
        Some(tool) => tool,
        None => return None,
    };

    // Check if tool has an execute function
    tool.execute.as_ref()?;

    // Create tool call options
    let mut options = ToolExecuteOptions::new(&tool_call_id, messages);

    if let Some(signal) = abort_signal {
        options = options.with_abort_signal(signal);
    }

    if let Some(context) = experimental_context {
        options = options.with_experimental_context(context);
    }

    // Execute the tool using the Tool::execute_tool method
    let result = if let Some(callback) = on_preliminary_tool_result.as_ref() {
        let callback = callback.clone();
        let tool_call_id_clone = tool_call_id.clone();
        let tool_name_clone = tool_name.clone();
        let input_clone = input.clone();

        let preliminary_callback = move |output: Value| {
            let result = ToolResult::new(
                tool_call_id_clone.clone(),
                tool_name_clone.clone(),
                input_clone.clone(),
                output,
            )
            .with_preliminary(true);

            callback(result);
        };

        tool.execute_tool(input.clone(), options, Some(preliminary_callback))
            .await
    } else {
        tool.execute_tool(input.clone(), options, None::<fn(Value)>)
            .await
    };

    // Convert the result to ToolOutput
    match result {
        Some(Ok(output)) => {
            let result = ToolResult::new(tool_call_id, tool_name, input, output);
            Some(ToolOutput::Result(result))
        }
        Some(Err(error)) => {
            let tool_error = ToolError::new(tool_call_id, tool_name, input, error);
            Some(ToolOutput::Error(tool_error))
        }
        None => None,
    }
}
