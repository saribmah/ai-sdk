use crate::generate_text::ToolSet;
use crate::generate_text::tool_call::TypedToolCall;
use crate::generate_text::tool_error::{DynamicToolError, StaticToolError, TypedToolError};
use crate::generate_text::tool_output::ToolOutput;
use crate::generate_text::tool_result::{DynamicToolResult, StaticToolResult, TypedToolResult};
use crate::prompt::message::ModelMessage;
use crate::prompt::message::tool::definition::Tool;
use crate::prompt::message::tool::execute::{ToolExecutionEvent, execute_tool};
use crate::prompt::message::tool::options::ToolCallOptions;
use futures_util::StreamExt;
use serde_json::Value;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

/// Callback function for preliminary tool results during streaming.
pub type OnPreliminaryToolResult = Arc<dyn Fn(TypedToolResult<Value, Value>) + Send + Sync>;

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
/// use ai_sdk_core::generate_text::execute_tool_call::execute_tool_call;
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
    tool_call: TypedToolCall<Value>,
    tools: &ToolSet,
    messages: Vec<ModelMessage>,
    abort_signal: Option<CancellationToken>,
    experimental_context: Option<Value>,
    on_preliminary_tool_result: Option<OnPreliminaryToolResult>,
) -> Option<ToolOutput<Value, Value>> {
    // Extract tool call information
    let (tool_call_id, tool_name, input, is_dynamic) = match &tool_call {
        TypedToolCall::Static(call) => (
            call.tool_call_id.clone(),
            call.tool_name.clone(),
            call.input.clone(),
            false,
        ),
        TypedToolCall::Dynamic(call) => (
            call.tool_call_id.clone(),
            call.tool_name.clone(),
            call.input.clone(),
            true,
        ),
    };

    // Look up the tool
    let tool: &Tool<Value, Value> = match tools.get(&tool_name) {
        Some(tool) => tool,
        None => return None,
    };

    // Check if tool has an execute function
    if tool.execute.is_none() {
        return None;
    }

    // Create tool call options
    let mut options = ToolCallOptions::new(&tool_call_id, messages);

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

    // Process events from the stream
    while let Some(event) = event_stream.next().await {
        match event {
            ToolExecutionEvent::Preliminary { output } => {
                // Handle preliminary results via callback
                if let Some(ref callback) = on_preliminary_tool_result {
                    let preliminary_result = if is_dynamic {
                        TypedToolResult::Dynamic(
                            DynamicToolResult::new(
                                &tool_call_id,
                                &tool_name,
                                input.clone(),
                                output.clone(),
                            )
                            .with_preliminary(true),
                        )
                    } else {
                        TypedToolResult::Static(
                            StaticToolResult::new(
                                &tool_call_id,
                                &tool_name,
                                input.clone(),
                                output.clone(),
                            )
                            .with_preliminary(true),
                        )
                    };
                    callback(preliminary_result);
                }
                final_output = Some(output);
            }
            ToolExecutionEvent::Final { output } => {
                final_output = Some(output);
            }
            ToolExecutionEvent::Error { error } => {
                // Return error immediately
                let tool_error = if is_dynamic {
                    TypedToolError::Dynamic(DynamicToolError::new(
                        &tool_call_id,
                        &tool_name,
                        input,
                        error,
                    ))
                } else {
                    TypedToolError::Static(StaticToolError::new(
                        &tool_call_id,
                        &tool_name,
                        input,
                        error,
                    ))
                };
                return Some(ToolOutput::Error(tool_error));
            }
        }
    }

    // Return final result
    final_output.map(|output| {
        let result = if is_dynamic {
            TypedToolResult::Dynamic(DynamicToolResult::new(
                &tool_call_id,
                &tool_name,
                input,
                output,
            ))
        } else {
            TypedToolResult::Static(StaticToolResult::new(
                &tool_call_id,
                &tool_name,
                input,
                output,
            ))
        };
        ToolOutput::Result(result)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_text::tool_call::StaticToolCall;
    use crate::prompt::message::tool::definition::ToolExecutionOutput;
    use serde_json::json;
    use std::pin::Pin;
    use std::sync::Mutex;

    #[tokio::test]
    async fn test_execute_tool_call_single_success() {
        // Create a tool that returns a single value
        let tool = Tool::function(json!({
            "type": "object",
            "properties": {
                "city": {"type": "string"}
            }
        }))
        .with_execute(Box::new(|input: Value, _options| {
            ToolExecutionOutput::Single(Box::pin(async move {
                Ok(json!({"temperature": 72, "city": input["city"]}))
            }))
        }));

        let mut tools = ToolSet::new();
        tools.insert("get_weather".to_string(), tool);

        let tool_call = TypedToolCall::Static(StaticToolCall::new(
            "call_123",
            "get_weather",
            json!({"city": "SF"}),
        ));

        let result = execute_tool_call(tool_call, &tools, vec![], None, None, None).await;

        assert!(result.is_some());
        let output = result.unwrap();
        assert!(output.is_result());
    }

    #[tokio::test]
    async fn test_execute_tool_call_no_execute_function() {
        // Create a tool without execute function
        let tool = Tool::function(json!({
            "type": "object",
            "properties": {
                "city": {"type": "string"}
            }
        }));

        let mut tools = ToolSet::new();
        tools.insert("get_weather".to_string(), tool);

        let tool_call = TypedToolCall::Static(StaticToolCall::new(
            "call_123",
            "get_weather",
            json!({"city": "SF"}),
        ));

        let result = execute_tool_call(tool_call, &tools, vec![], None, None, None).await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_execute_tool_call_tool_not_found() {
        let tools = ToolSet::new();

        let tool_call = TypedToolCall::Static(StaticToolCall::new(
            "call_123",
            "nonexistent_tool",
            json!({"city": "SF"}),
        ));

        let result = execute_tool_call(tool_call, &tools, vec![], None, None, None).await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_execute_tool_call_streaming_with_preliminary() {
        use futures_util::stream;

        // Track preliminary results
        let preliminary_results = Arc::new(Mutex::new(Vec::new()));
        let preliminary_clone = preliminary_results.clone();

        let callback: OnPreliminaryToolResult = Arc::new(move |result| {
            preliminary_clone.lock().unwrap().push(result);
        });

        // Create a streaming tool
        let tool = Tool::function(json!({
            "type": "object",
            "properties": {}
        }))
        .with_execute(Box::new(|_input: Value, _options| {
            let values = vec![
                Ok(json!({"step": 1})),
                Ok(json!({"step": 2})),
                Ok(json!({"step": 3})),
            ];
            ToolExecutionOutput::Streaming(Box::pin(stream::iter(values)))
        }));

        let mut tools = ToolSet::new();
        tools.insert("streaming_tool".to_string(), tool);

        let tool_call =
            TypedToolCall::Static(StaticToolCall::new("call_123", "streaming_tool", json!({})));

        let result = execute_tool_call(tool_call, &tools, vec![], None, None, Some(callback)).await;

        assert!(result.is_some());
        let output = result.unwrap();
        assert!(output.is_result());

        // Check preliminary results were called
        let prelim = preliminary_results.lock().unwrap();
        assert_eq!(prelim.len(), 3);
    }

    #[tokio::test]
    async fn test_execute_tool_call_single_error() {
        // Create a tool that returns an error
        let tool = Tool::function(json!({
            "type": "object",
            "properties": {
                "value": {"type": "number"}
            }
        }))
        .with_execute(Box::new(|input: Value, _options| {
            ToolExecutionOutput::Single(Box::pin(async move {
                if input["value"].as_i64() == Some(0) {
                    Err(json!("Division by zero"))
                } else {
                    Ok(json!({"result": 100 / input["value"].as_i64().unwrap()}))
                }
            }))
        }));

        let mut tools = ToolSet::new();
        tools.insert("divide".to_string(), tool);

        let tool_call = TypedToolCall::Static(StaticToolCall::new(
            "call_123",
            "divide",
            json!({"value": 0}),
        ));

        let result = execute_tool_call(tool_call, &tools, vec![], None, None, None).await;

        assert!(result.is_some());
        let output = result.unwrap();
        assert!(output.is_error());

        if let Some(error) = output.as_error() {
            match error {
                TypedToolError::Static(err) => {
                    assert_eq!(err.tool_call_id, "call_123");
                    assert_eq!(err.tool_name, "divide");
                    assert_eq!(err.error, json!("Division by zero"));
                }
                _ => panic!("Expected Static error"),
            }
        }
    }

    #[tokio::test]
    async fn test_execute_tool_call_streaming_error() {
        use futures_util::stream;

        // Create a streaming tool that returns an error mid-stream
        let tool = Tool::function(json!({
            "type": "object",
            "properties": {}
        }))
        .with_execute(Box::new(|_input: Value, _options| {
            let values = vec![
                Ok(json!({"step": 1})),
                Err(json!("Unexpected error")),
                Ok(json!({"step": 3})), // This should not be reached
            ];
            ToolExecutionOutput::Streaming(Box::pin(stream::iter(values)))
        }));

        let mut tools = ToolSet::new();
        tools.insert("streaming_error".to_string(), tool);

        let tool_call = TypedToolCall::Static(StaticToolCall::new(
            "call_456",
            "streaming_error",
            json!({}),
        ));

        let result = execute_tool_call(tool_call, &tools, vec![], None, None, None).await;

        assert!(result.is_some());
        let output = result.unwrap();
        assert!(output.is_error());

        if let Some(error) = output.as_error() {
            match error {
                TypedToolError::Static(err) => {
                    assert_eq!(err.tool_call_id, "call_456");
                    assert_eq!(err.tool_name, "streaming_error");
                    assert_eq!(err.error, json!("Unexpected error"));
                }
                _ => panic!("Expected Static error"),
            }
        }
    }
}
