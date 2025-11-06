use super::{
    Tool, ToolCall, ToolError, ToolExecuteOptions, ToolExecutionEvent, ToolOutput, ToolResult,
    ToolSet, execute_tool,
};
use crate::prompt::message::Message;
use futures_util::StreamExt;
use serde_json::Value;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

/// Callback function for preliminary tool results during streaming.
pub type OnPreliminaryToolResult = Arc<dyn Fn(ToolResult) + Send + Sync>;

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
/// ```ignore
/// use ai_sdk_core::tool::execute_tool_call;
///
/// let output = execute_tool_call(
///     tool_call,
///     &tools,
///     messages,
///     Some(abort_signal),
///     None,
///     None,
/// ).await;
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
    if tool.execute.is_none() {
        return None;
    }

    // Create tool call options
    let mut options = ToolExecuteOptions::new(&tool_call_id, messages);

    if let Some(signal) = abort_signal {
        options = options.with_abort_signal(signal);
    }

    if let Some(context) = experimental_context {
        options = options.with_experimental_context(context);
    }

    // Execute the tool using the execute_tool function
    let execute_fn = tool.execute.as_ref().unwrap();
    let mut event_stream = Box::pin(execute_tool(execute_fn, input.clone(), options));

    let mut final_output: Option<Value> = None;

    // Process the event stream
    while let Some(event) = event_stream.next().await {
        match event {
            ToolExecutionEvent::Preliminary { output } => {
                // Store the preliminary output
                final_output = Some(output.clone());

                // Call the preliminary result callback if provided
                if let Some(callback) = &on_preliminary_tool_result {
                    let result = ToolResult::new(
                        tool_call_id.clone(),
                        tool_name.clone(),
                        input.clone(),
                        output,
                    )
                    .with_preliminary(true);

                    callback(result);
                }
            }
            ToolExecutionEvent::Final { output } => {
                // Store the final output
                final_output = Some(output);
            }
            ToolExecutionEvent::Error { error } => {
                // Return tool error
                let tool_error = ToolError::new(
                    tool_call_id,
                    tool_name,
                    input,
                    error,
                );

                return Some(ToolOutput::Error(tool_error));
            }
        }
    }

    // Return the final result if we have one
    final_output.map(|output| {
        let result = ToolResult::new(
            tool_call_id,
            tool_name,
            input,
            output,
        );

        ToolOutput::Result(result)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool::definition::{Tool, ToolExecutionOutput};
    use serde_json::json;

    #[tokio::test]
    async fn test_execute_tool_call_success() {
        let mut tools = ToolSet::new();

        let tool = Tool::function(json!({"type": "object"}))
            .with_description("Test tool")
            .with_execute(Box::new(|input: Value, _options| {
                ToolExecutionOutput::Single(Box::pin(async move {
                    Ok(json!({"result": input}))
                }))
            }));

        tools.insert("test_tool".to_string(), tool);

        let tool_call = ToolCall::new(
            "call_123",
            "test_tool",
            json!({"city": "SF"}),
        );

        let result = execute_tool_call(
            tool_call,
            &tools,
            vec![],
            None,
            None,
            None,
        )
        .await;

        assert!(result.is_some());
        let output = result.unwrap();
        assert!(output.is_result());

        if let ToolOutput::Result(res) = output {
            assert_eq!(res.tool_call_id, "call_123");
            assert_eq!(res.tool_name, "test_tool");
            assert_eq!(res.output, json!({"result": {"city": "SF"}}));
        }
    }

    #[tokio::test]
    async fn test_execute_tool_call_error() {
        let mut tools = ToolSet::new();

        let tool = Tool::function(json!({"type": "object"}))
            .with_execute(Box::new(|_input: Value, _options| {
                ToolExecutionOutput::Single(Box::pin(async move {
                    Err(json!({"error": "Something went wrong"}))
                }))
            }));

        tools.insert("test_tool".to_string(), tool);

        let tool_call = ToolCall::new("call_123", "test_tool", json!({}));

        let result = execute_tool_call(
            tool_call,
            &tools,
            vec![],
            None,
            None,
            None,
        )
        .await;

        assert!(result.is_some());
        let output = result.unwrap();
        assert!(output.is_error());

        if let ToolOutput::Error(err) = output {
            assert_eq!(err.tool_call_id, "call_123");
            assert_eq!(err.tool_name, "test_tool");
            assert_eq!(err.error, json!({"error": "Something went wrong"}));
        }
    }

    #[tokio::test]
    async fn test_execute_tool_call_tool_not_found() {
        let tools = ToolSet::new();

        let tool_call = ToolCall::new("call_123", "nonexistent_tool", json!({}));

        let result = execute_tool_call(
            tool_call,
            &tools,
            vec![],
            None,
            None,
            None,
        )
        .await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_execute_tool_call_no_execute_function() {
        let mut tools = ToolSet::new();

        let tool = Tool::function(json!({"type": "object"}));

        tools.insert("test_tool".to_string(), tool);

        let tool_call = ToolCall::new("call_123", "test_tool", json!({}));

        let result = execute_tool_call(
            tool_call,
            &tools,
            vec![],
            None,
            None,
            None,
        )
        .await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_execute_tool_call_with_streaming() {
        let mut tools = ToolSet::new();

        let tool = Tool::function(json!({"type": "object"}))
            .with_execute(Box::new(|_input: Value, _options| {
                ToolExecutionOutput::Streaming(Box::pin(async_stream::stream! {
                    yield Ok(json!({"step": 1}));
                    yield Ok(json!({"step": 2}));
                    yield Ok(json!({"step": 3}));
                }))
            }));

        tools.insert("test_tool".to_string(), tool);

        let tool_call = ToolCall::new("call_123", "test_tool", json!({}));

        // Track preliminary results
        let preliminary_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let preliminary_count_clone = preliminary_count.clone();

        let callback = Arc::new(move |result: ToolResult| {
            if result.preliminary == Some(true) {
                preliminary_count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        });

        let result = execute_tool_call(
            tool_call,
            &tools,
            vec![],
            None,
            None,
            Some(callback),
        )
        .await;

        assert!(result.is_some());

        // Should have received 3 preliminary results
        assert_eq!(preliminary_count.load(std::sync::atomic::Ordering::SeqCst), 3);

        // Final result should be the last output
        if let Some(ToolOutput::Result(res)) = result {
            assert_eq!(res.output, json!({"step": 3}));
            assert_eq!(res.preliminary, None); // Final result is not preliminary
        }
    }
}
