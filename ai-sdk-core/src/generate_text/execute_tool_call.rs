use crate::generate_text::tool_call::TypedToolCall;
use crate::generate_text::tool_error::{DynamicToolError, StaticToolError, TypedToolError};
use crate::generate_text::tool_output::ToolOutput;
use crate::generate_text::tool_result::{DynamicToolResult, StaticToolResult, TypedToolResult};
use crate::generate_text::ToolSet;
use crate::message::tool::definition::{Tool, ToolExecutionOutput};
use crate::message::tool::options::ToolCallOptions;
use crate::message::ModelMessage;
use futures_util::StreamExt;
use serde_json::Value;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

/// Callback function for preliminary tool results during streaming.
pub type OnPreliminaryToolResult =
    Arc<dyn Fn(TypedToolResult<Value, Value>) + Send + Sync>;

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

    // Execute the tool
    let execute_fn = tool.execute.as_ref().unwrap();
    let execution_output = execute_fn(input.clone(), options);

    // Handle the execution output (single or streaming)
    match execution_output {
        ToolExecutionOutput::Single(future) => {
            // Execute single output tool
            match execute_single_tool(
                future,
                tool_call_id,
                tool_name,
                input,
                is_dynamic,
            )
            .await
            {
                Ok(output) => Some(output),
                Err(err_output) => Some(err_output),
            }
        }
        ToolExecutionOutput::Streaming(stream) => {
            // Execute streaming tool with preliminary result handling
            match execute_streaming_tool(
                stream,
                tool_call_id,
                tool_name,
                input,
                is_dynamic,
                on_preliminary_tool_result,
            )
            .await
            {
                Ok(output) => Some(output),
                Err(err_output) => Some(err_output),
            }
        }
    }
}

/// Executes a single (non-streaming) tool and returns the output.
async fn execute_single_tool(
    future: std::pin::Pin<Box<dyn std::future::Future<Output = Value> + Send>>,
    tool_call_id: String,
    tool_name: String,
    input: Value,
    is_dynamic: bool,
) -> Result<ToolOutput<Value, Value>, ToolOutput<Value, Value>> {
    match tokio::task::spawn(async move { future.await }).await {
        Ok(output) => {
            // Create successful result
            if is_dynamic {
                let result = DynamicToolResult::new(&tool_call_id, &tool_name, input, output);
                Ok(ToolOutput::Result(TypedToolResult::Dynamic(result)))
            } else {
                let result = StaticToolResult::new(&tool_call_id, &tool_name, input, output);
                Ok(ToolOutput::Result(TypedToolResult::Static(result)))
            }
        }
        Err(error) => {
            // Create error result
            let error_value = Value::String(error.to_string());
            if is_dynamic {
                let err = DynamicToolError::new(&tool_call_id, &tool_name, input, error_value);
                Err(ToolOutput::Error(TypedToolError::Dynamic(err)))
            } else {
                let err = StaticToolError::new(&tool_call_id, &tool_name, input, error_value);
                Err(ToolOutput::Error(TypedToolError::Static(err)))
            }
        }
    }
}

/// Executes a streaming tool and handles preliminary results.
async fn execute_streaming_tool(
    mut stream: std::pin::Pin<Box<dyn futures_util::Stream<Item = Value> + Send>>,
    tool_call_id: String,
    tool_name: String,
    input: Value,
    is_dynamic: bool,
    on_preliminary_tool_result: Option<OnPreliminaryToolResult>,
) -> Result<ToolOutput<Value, Value>, ToolOutput<Value, Value>> {
    let mut final_output: Option<Value> = None;

    // Process stream
    while let Some(output) = stream.next().await {
        // If this is not the last item, it's a preliminary result
        if on_preliminary_tool_result.is_some() {
            // Create preliminary result
            if is_dynamic {
                let preliminary = DynamicToolResult::new(
                    &tool_call_id,
                    &tool_name,
                    input.clone(),
                    output.clone(),
                )
                .with_preliminary(true);

                if let Some(ref callback) = on_preliminary_tool_result {
                    callback(TypedToolResult::Dynamic(preliminary));
                }
            } else {
                let preliminary = StaticToolResult::new(
                    &tool_call_id,
                    &tool_name,
                    input.clone(),
                    output.clone(),
                )
                .with_preliminary(true);

                if let Some(ref callback) = on_preliminary_tool_result {
                    callback(TypedToolResult::Static(preliminary));
                }
            }
        }

        final_output = Some(output);
    }

    // Return final result
    if let Some(output) = final_output {
        if is_dynamic {
            let result = DynamicToolResult::new(&tool_call_id, &tool_name, input, output);
            Ok(ToolOutput::Result(TypedToolResult::Dynamic(result)))
        } else {
            let result = StaticToolResult::new(&tool_call_id, &tool_name, input, output);
            Ok(ToolOutput::Result(TypedToolResult::Static(result)))
        }
    } else {
        // Stream was empty - treat as error
        let error_value = Value::String("Tool execution produced no output".to_string());
        if is_dynamic {
            let err = DynamicToolError::new(&tool_call_id, &tool_name, input, error_value);
            Err(ToolOutput::Error(TypedToolError::Dynamic(err)))
        } else {
            let err = StaticToolError::new(&tool_call_id, &tool_name, input, error_value);
            Err(ToolOutput::Error(TypedToolError::Static(err)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_text::tool_call::StaticToolCall;
    use crate::message::tool::definition::ToolExecutionOutput;
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
                json!({"temperature": 72, "city": input["city"]})
            }))
        }));

        let mut tools = ToolSet::new();
        tools.insert("get_weather".to_string(), tool);

        let tool_call = TypedToolCall::Static(StaticToolCall::new(
            "call_123",
            "get_weather",
            json!({"city": "SF"}),
        ));

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
    async fn test_execute_tool_call_tool_not_found() {
        let tools = ToolSet::new();

        let tool_call = TypedToolCall::Static(StaticToolCall::new(
            "call_123",
            "nonexistent_tool",
            json!({"city": "SF"}),
        ));

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
                json!({"step": 1}),
                json!({"step": 2}),
                json!({"step": 3}),
            ];
            ToolExecutionOutput::Streaming(Box::pin(stream::iter(values)))
        }));

        let mut tools = ToolSet::new();
        tools.insert("streaming_tool".to_string(), tool);

        let tool_call = TypedToolCall::Static(StaticToolCall::new(
            "call_123",
            "streaming_tool",
            json!({}),
        ));

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
        let output = result.unwrap();
        assert!(output.is_result());

        // Check preliminary results were called
        let prelim = preliminary_results.lock().unwrap();
        assert_eq!(prelim.len(), 3);
    }
}
