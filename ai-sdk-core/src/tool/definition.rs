use super::options::ToolExecuteOptions;
use super::{ToolCall, ToolError, ToolOutput, ToolResult, ToolSet};
use crate::prompt::message::Message;
use crate::prompt::message::content_parts::ToolResultOutput;
use ai_sdk_provider::shared::provider_options::SharedProviderOptions;
use futures_util::Stream;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

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
pub type ToolExecuteFunction<INPUT, OUTPUT> =
    Box<dyn Fn(INPUT, ToolExecuteOptions) -> ToolExecutionOutput<OUTPUT> + Send + Sync>;

/// Function that determines if a tool needs approval before execution.
pub type ToolNeedsApprovalFunction<INPUT> = Box<
    dyn Fn(INPUT, ToolExecuteOptions) -> Pin<Box<dyn Future<Output = bool> + Send>> + Send + Sync,
>;

/// Callback that is called when tool input streaming starts.
pub type OnInputStartCallback =
    Box<dyn Fn(ToolExecuteOptions) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Callback that is called when a tool input delta is available.
pub type OnInputDeltaCallback = Box<
    dyn Fn(String, ToolExecuteOptions) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync,
>;

/// Callback that is called when tool input is available.
pub type OnInputAvailableCallback<INPUT> = Box<
    dyn Fn(INPUT, ToolExecuteOptions) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync,
>;

/// Function that converts tool output to model output format.
pub type ToModelOutputFunction<OUTPUT> = Box<dyn Fn(OUTPUT) -> ToolResultOutput + Send + Sync>;

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

/// A tool contains the description and the schema of the input that the tool expects.
/// This enables the language model to generate the input.
///
/// The tool can also contain an optional execute function for the actual execution of the tool.
///
/// For type-safe tools with compile-time guarantees, use the `TypeSafeTool` trait instead.
pub struct Tool {
    /// An optional description of what the tool does.
    /// Will be used by the language model to decide whether to use the tool.
    pub description: Option<String>,

    /// Additional provider-specific metadata.
    pub provider_options: Option<SharedProviderOptions>,

    /// The schema of the input that the tool expects.
    /// Use JSON Schema to describe the input structure.
    pub input_schema: Value,

    /// The schema of the output that the tool produces (optional).
    pub output_schema: Option<Value>,

    /// Whether the tool needs approval before it can be executed.
    pub needs_approval: NeedsApproval,

    /// The type of tool.
    pub tool_type: ToolType,

    /// An async function that is called with the arguments from the tool call and produces a result.
    pub execute: Option<ToolExecuteFunction<Value, Value>>,

    /// Optional function that is called when the argument streaming starts.
    pub on_input_start: Option<OnInputStartCallback>,

    /// Optional function that is called when an argument streaming delta is available.
    pub on_input_delta: Option<OnInputDeltaCallback>,

    /// Optional function that is called when a tool call can be started.
    pub on_input_available: Option<OnInputAvailableCallback<Value>>,

    /// Optional conversion function that maps the tool result to an output that can be used by the language model.
    pub to_model_output: Option<ToModelOutputFunction<Value>>,
}

/// Whether a tool needs approval before execution.
pub enum NeedsApproval {
    /// Tool does not need approval.
    No,

    /// Tool always needs approval.
    Yes,

    /// Tool needs approval based on a function.
    Function(ToolNeedsApprovalFunction<Value>),
}

impl Tool {
    /// Creates a new function tool with the given input schema.
    pub fn function(input_schema: Value) -> Self {
        Self {
            description: None,
            provider_options: None,
            input_schema,
            output_schema: None,
            needs_approval: NeedsApproval::No,
            tool_type: ToolType::Function,
            execute: None,
            on_input_start: None,
            on_input_delta: None,
            on_input_available: None,
            to_model_output: None,
        }
    }

    /// Creates a new dynamic tool with the given input schema.
    pub fn dynamic(input_schema: Value) -> Self {
        Self {
            description: None,
            provider_options: None,
            input_schema,
            output_schema: None,
            needs_approval: NeedsApproval::No,
            tool_type: ToolType::Dynamic,
            execute: None,
            on_input_start: None,
            on_input_delta: None,
            on_input_available: None,
            to_model_output: None,
        }
    }

    /// Creates a new provider-defined tool.
    pub fn provider_defined(
        id: impl Into<String>,
        name: impl Into<String>,
        args: HashMap<String, Value>,
        input_schema: Value,
    ) -> Self {
        Self {
            description: None,
            provider_options: None,
            input_schema,
            output_schema: None,
            needs_approval: NeedsApproval::No,
            tool_type: ToolType::ProviderDefined {
                id: id.into(),
                name: name.into(),
                args,
            },
            execute: None,
            on_input_start: None,
            on_input_delta: None,
            on_input_available: None,
            to_model_output: None,
        }
    }

    /// Sets the description of the tool.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the provider options.
    pub fn with_provider_options(mut self, options: SharedProviderOptions) -> Self {
        self.provider_options = Some(options);
        self
    }

    /// Sets the output schema.
    pub fn with_output_schema(mut self, schema: Value) -> Self {
        self.output_schema = Some(schema);
        self
    }

    /// Sets whether the tool needs approval.
    pub fn with_needs_approval(mut self, needs_approval: bool) -> Self {
        self.needs_approval = if needs_approval {
            NeedsApproval::Yes
        } else {
            NeedsApproval::No
        };
        self
    }

    /// Sets the needs approval function.
    pub fn with_needs_approval_function(mut self, func: ToolNeedsApprovalFunction<Value>) -> Self {
        self.needs_approval = NeedsApproval::Function(func);
        self
    }

    /// Sets the execute function.
    pub fn with_execute(mut self, func: ToolExecuteFunction<Value, Value>) -> Self {
        self.execute = Some(func);
        self
    }

    /// Sets the on_input_start callback.
    pub fn with_on_input_start(mut self, callback: OnInputStartCallback) -> Self {
        self.on_input_start = Some(callback);
        self
    }

    /// Sets the on_input_delta callback.
    pub fn with_on_input_delta(mut self, callback: OnInputDeltaCallback) -> Self {
        self.on_input_delta = Some(callback);
        self
    }

    /// Sets the on_input_available callback.
    pub fn with_on_input_available(mut self, callback: OnInputAvailableCallback<Value>) -> Self {
        self.on_input_available = Some(callback);
        self
    }

    /// Sets the to_model_output conversion function.
    pub fn with_to_model_output(mut self, func: ToModelOutputFunction<Value>) -> Self {
        self.to_model_output = Some(func);
        self
    }

    /// Returns true if this tool is a function tool.
    pub fn is_function(&self) -> bool {
        matches!(self.tool_type, ToolType::Function)
    }

    /// Returns true if this tool is a dynamic tool.
    pub fn is_dynamic(&self) -> bool {
        matches!(self.tool_type, ToolType::Dynamic)
    }

    /// Returns true if this tool is a provider-defined tool.
    pub fn is_provider_defined(&self) -> bool {
        matches!(self.tool_type, ToolType::ProviderDefined { .. })
    }

    /// Checks if this tool needs approval for the given input.
    pub async fn check_needs_approval(&self, input: Value, options: ToolExecuteOptions) -> bool {
        match &self.needs_approval {
            NeedsApproval::No => false,
            NeedsApproval::Yes => true,
            NeedsApproval::Function(func) => func(input, options).await,
        }
    }

    /// Executes the tool with the given input and options.
    ///
    /// For streaming tools, this will consume the entire stream and return the final output.
    /// For non-streaming tools, this will await and return the single output.
    ///
    /// Returns None if the tool has no execute function.
    /// Returns Some(Err(error)) if the tool execution fails.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to pass to the tool
    /// * `options` - Additional options for the tool execution
    /// * `on_preliminary_output` - Optional callback for preliminary streaming results
    ///
    /// # Returns
    ///
    /// Returns `Some(Ok(output))` if the tool executed successfully,
    /// `Some(Err(error))` if the tool execution failed,
    /// or `None` if the tool has no execute function.
    pub async fn execute_tool<F>(
        &self,
        input: Value,
        options: ToolExecuteOptions,
        on_preliminary_output: Option<F>,
    ) -> Option<Result<Value, Value>>
    where
        F: Fn(Value) + Send + Sync,
    {
        use futures_util::StreamExt;

        if let Some(execute) = &self.execute {
            let result = execute(input, options);
            match result {
                ToolExecutionOutput::Single(future) => Some(future.await),
                ToolExecutionOutput::Streaming(mut stream) => {
                    let mut last_output: Option<Result<Value, Value>> = None;
                    while let Some(output) = stream.next().await {
                        // If we encounter an error in the stream, return it immediately
                        if output.is_err() {
                            return Some(output);
                        }

                        // Call the preliminary output callback if provided
                        if let (Some(callback), Ok(value)) =
                            (on_preliminary_output.as_ref(), &output)
                        {
                            callback(value.clone());
                        }

                        last_output = Some(output);
                    }
                    last_output
                }
            }
        } else {
            None
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tool::ToolSet;
    use serde_json::json;
    use std::sync::Arc;

    #[test]
    fn test_tool_function() {
        let schema = json!({
            "type": "object",
            "properties": {
                "city": {"type": "string"}
            }
        });

        let tool: Tool = Tool::function(schema.clone());

        assert!(tool.is_function());
        assert!(!tool.is_dynamic());
        assert!(!tool.is_provider_defined());
        assert!(tool.description.is_none());
        assert_eq!(tool.input_schema, schema);
    }

    #[test]
    fn test_tool_dynamic() {
        let schema = json!({"type": "object"});

        let tool: Tool = Tool::dynamic(schema.clone());

        assert!(!tool.is_function());
        assert!(tool.is_dynamic());
        assert!(!tool.is_provider_defined());
    }

    #[test]
    fn test_tool_provider_defined() {
        let schema = json!({"type": "object"});
        let mut args = HashMap::new();
        args.insert("key".to_string(), json!("value"));

        let tool: Tool = Tool::provider_defined("provider.tool", "tool_name", args.clone(), schema);

        assert!(!tool.is_function());
        assert!(!tool.is_dynamic());
        assert!(tool.is_provider_defined());

        if let ToolType::ProviderDefined {
            id,
            name,
            args: tool_args,
        } = &tool.tool_type
        {
            assert_eq!(id, "provider.tool");
            assert_eq!(name, "tool_name");
            assert_eq!(tool_args, &args);
        } else {
            panic!("Expected ProviderDefined");
        }
    }

    #[test]
    fn test_tool_with_description() {
        let schema = json!({"type": "object"});

        let tool: Tool = Tool::function(schema).with_description("A helpful tool");

        assert_eq!(tool.description, Some("A helpful tool".to_string()));
    }

    #[test]
    fn test_tool_with_output_schema() {
        let input_schema = json!({"type": "object"});
        let output_schema = json!({"type": "string"});

        let tool: Tool = Tool::function(input_schema).with_output_schema(output_schema.clone());

        assert_eq!(tool.output_schema, Some(output_schema));
    }

    #[test]
    fn test_tool_with_needs_approval() {
        let schema = json!({"type": "object"});

        let tool: Tool = Tool::function(schema).with_needs_approval(true);

        assert!(matches!(tool.needs_approval, NeedsApproval::Yes));
    }

    #[test]
    fn test_tool_with_provider_options() {
        let schema = json!({"type": "object"});
        let options = SharedProviderOptions::new();

        let tool: Tool = Tool::function(schema).with_provider_options(options.clone());

        assert_eq!(tool.provider_options, Some(options));
    }

    #[tokio::test]
    async fn test_tool_check_needs_approval_no() {
        let schema = json!({"type": "object"});
        let tool: Tool = Tool::function(schema);

        let options = ToolExecuteOptions::new("call_123", vec![]);
        let needs_approval = tool.check_needs_approval(json!({}), options).await;

        assert!(!needs_approval);
    }

    #[tokio::test]
    async fn test_tool_check_needs_approval_yes() {
        let schema = json!({"type": "object"});
        let tool: Tool = Tool::function(schema).with_needs_approval(true);

        let options = ToolExecuteOptions::new("call_123", vec![]);
        let needs_approval = tool.check_needs_approval(json!({}), options).await;

        assert!(needs_approval);
    }

    #[tokio::test]
    async fn test_tool_execute_with_function() {
        let schema = json!({"type": "object"});

        let tool: Tool = Tool::function(schema).with_execute(Box::new(|input: Value, _options| {
            ToolExecutionOutput::Single(Box::pin(async move { Ok(json!({"result": input})) }))
        }));

        let options = ToolExecuteOptions::new("call_123", vec![]);
        let result = tool
            .execute_tool(json!({"city": "SF"}), options, None::<fn(Value)>)
            .await;

        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), json!({"result": {"city": "SF"}}));
    }

    #[tokio::test]
    async fn test_tool_execute_without_function() {
        let schema = json!({"type": "object"});
        let tool: Tool = Tool::function(schema);

        let options = ToolExecuteOptions::new("call_123", vec![]);
        let result = tool
            .execute_tool(json!({}), options, None::<fn(Value)>)
            .await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_tool_execute_with_streaming() {
        let schema = json!({"type": "object"});

        let tool: Tool =
            Tool::function(schema).with_execute(Box::new(|_input: Value, _options| {
                ToolExecutionOutput::Streaming(Box::pin(async_stream::stream! {
                    yield Ok(json!({"step": 1}));
                    yield Ok(json!({"step": 2}));
                    yield Ok(json!({"step": 3}));
                }))
            }));

        // Track preliminary results
        let preliminary_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let preliminary_count_clone = preliminary_count.clone();

        let callback = move |_output: Value| {
            preliminary_count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        };

        let options = ToolExecuteOptions::new("call_123", vec![]);
        let result = tool.execute_tool(json!({}), options, Some(&callback)).await;

        // Should have received 3 preliminary results
        assert_eq!(
            preliminary_count.load(std::sync::atomic::Ordering::SeqCst),
            3
        );

        // Should return the last output
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), json!({"step": 3}));
    }

    #[tokio::test]
    async fn test_tool_execute_with_error() {
        let schema = json!({"type": "object"});

        let tool: Tool =
            Tool::function(schema).with_execute(Box::new(|_input: Value, _options| {
                ToolExecutionOutput::Single(Box::pin(async move {
                    Err(json!({"error": "Something went wrong"}))
                }))
            }));

        let options = ToolExecuteOptions::new("call_123", vec![]);
        let result = tool
            .execute_tool(json!({}), options, None::<fn(Value)>)
            .await;

        // Should return an error
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            json!({"error": "Something went wrong"})
        );
    }

    #[tokio::test]
    async fn test_tool_execute_streaming_with_error() {
        let schema = json!({"type": "object"});

        let tool: Tool =
            Tool::function(schema).with_execute(Box::new(|_input: Value, _options| {
                ToolExecutionOutput::Streaming(Box::pin(async_stream::stream! {
                    yield Ok(json!({"step": 1}));
                    yield Err(json!("Error in stream"));
                    yield Ok(json!({"step": 3})); // This should not be reached
                }))
            }));

        let options = ToolExecuteOptions::new("call_123", vec![]);
        let result = tool
            .execute_tool(json!({}), options, None::<fn(Value)>)
            .await;

        // Should return the error immediately
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), json!("Error in stream"));
    }

    // Tests for execute_tool_call function

    #[tokio::test]
    async fn test_execute_tool_call_success() {
        let mut tools = ToolSet::new();

        let tool = Tool::function(json!({"type": "object"}))
            .with_description("Test tool")
            .with_execute(Box::new(|input: Value, _options| {
                ToolExecutionOutput::Single(Box::pin(async move { Ok(json!({"result": input})) }))
            }));

        tools.insert("test_tool".to_string(), tool);

        let tool_call = ToolCall::new("call_123", "test_tool", json!({"city": "SF"}));

        let result = execute_tool_call(tool_call, &tools, vec![], None, None, None).await;

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

        let tool = Tool::function(json!({"type": "object"})).with_execute(Box::new(
            |_input: Value, _options| {
                ToolExecutionOutput::Single(Box::pin(async move {
                    Err(json!({"error": "Something went wrong"}))
                }))
            },
        ));

        tools.insert("test_tool".to_string(), tool);

        let tool_call = ToolCall::new("call_123", "test_tool", json!({}));

        let result = execute_tool_call(tool_call, &tools, vec![], None, None, None).await;

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

        let result = execute_tool_call(tool_call, &tools, vec![], None, None, None).await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_execute_tool_call_no_execute_function() {
        let mut tools = ToolSet::new();

        let tool = Tool::function(json!({"type": "object"}));

        tools.insert("test_tool".to_string(), tool);

        let tool_call = ToolCall::new("call_123", "test_tool", json!({}));

        let result = execute_tool_call(tool_call, &tools, vec![], None, None, None).await;

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_execute_tool_call_with_streaming() {
        let mut tools = ToolSet::new();

        let tool = Tool::function(json!({"type": "object"})).with_execute(Box::new(
            |_input: Value, _options| {
                ToolExecutionOutput::Streaming(Box::pin(async_stream::stream! {
                    yield Ok(json!({"step": 1}));
                    yield Ok(json!({"step": 2}));
                    yield Ok(json!({"step": 3}));
                }))
            },
        ));

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

        let result = execute_tool_call(tool_call, &tools, vec![], None, None, Some(callback)).await;

        assert!(result.is_some());

        // Should have received 3 preliminary results
        assert_eq!(
            preliminary_count.load(std::sync::atomic::Ordering::SeqCst),
            3
        );

        // Final result should be the last output
        if let Some(ToolOutput::Result(res)) = result {
            assert_eq!(res.output, json!({"step": 3}));
            assert_eq!(res.preliminary, None); // Final result is not preliminary
        }
    }
}
