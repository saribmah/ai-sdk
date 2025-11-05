use super::options::ToolExecuteOptions;
use crate::prompt::message::content_parts::ToolResultOutput;
use ai_sdk_provider::shared::provider_options::SharedProviderOptions;
use futures_util::Stream;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

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
pub type ToolNeedsApprovalFunction<INPUT> =
    Box<dyn Fn(INPUT, ToolExecuteOptions) -> Pin<Box<dyn Future<Output = bool> + Send>> + Send + Sync>;

/// Callback that is called when tool input streaming starts.
pub type OnInputStartCallback =
    Box<dyn Fn(ToolExecuteOptions) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Callback that is called when a tool input delta is available.
pub type OnInputDeltaCallback =
    Box<dyn Fn(String, ToolExecuteOptions) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Callback that is called when tool input is available.
pub type OnInputAvailableCallback<INPUT> =
    Box<dyn Fn(INPUT, ToolExecuteOptions) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

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
pub struct Tool<INPUT = Value, OUTPUT = Value>
where
    INPUT: Serialize + for<'de> Deserialize<'de> + Send + 'static,
    OUTPUT: Serialize + for<'de> Deserialize<'de> + Send + 'static,
{
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
    pub needs_approval: NeedsApproval<INPUT>,

    /// The type of tool.
    pub tool_type: ToolType,

    /// An async function that is called with the arguments from the tool call and produces a result.
    pub execute: Option<ToolExecuteFunction<INPUT, OUTPUT>>,

    /// Optional function that is called when the argument streaming starts.
    pub on_input_start: Option<OnInputStartCallback>,

    /// Optional function that is called when an argument streaming delta is available.
    pub on_input_delta: Option<OnInputDeltaCallback>,

    /// Optional function that is called when a tool call can be started.
    pub on_input_available: Option<OnInputAvailableCallback<INPUT>>,

    /// Optional conversion function that maps the tool result to an output that can be used by the language model.
    pub to_model_output: Option<ToModelOutputFunction<OUTPUT>>,
}

/// Whether a tool needs approval before execution.
pub enum NeedsApproval<INPUT>
where
    INPUT: Send + 'static,
{
    /// Tool does not need approval.
    No,

    /// Tool always needs approval.
    Yes,

    /// Tool needs approval based on a function.
    Function(ToolNeedsApprovalFunction<INPUT>),
}

impl<INPUT, OUTPUT> Tool<INPUT, OUTPUT>
where
    INPUT: Serialize + for<'de> Deserialize<'de> + Send + 'static,
    OUTPUT: Serialize + for<'de> Deserialize<'de> + Send + 'static,
{
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
    pub fn with_needs_approval_function(mut self, func: ToolNeedsApprovalFunction<INPUT>) -> Self {
        self.needs_approval = NeedsApproval::Function(func);
        self
    }

    /// Sets the execute function.
    pub fn with_execute(mut self, func: ToolExecuteFunction<INPUT, OUTPUT>) -> Self {
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
    pub fn with_on_input_available(mut self, callback: OnInputAvailableCallback<INPUT>) -> Self {
        self.on_input_available = Some(callback);
        self
    }

    /// Sets the to_model_output conversion function.
    pub fn with_to_model_output(mut self, func: ToModelOutputFunction<OUTPUT>) -> Self {
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
    pub async fn check_needs_approval(&self, input: INPUT, options: ToolExecuteOptions) -> bool {
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
    pub async fn execute_tool(
        &self,
        input: INPUT,
        options: ToolExecuteOptions,
    ) -> Option<Result<OUTPUT, Value>>
    where
        OUTPUT: Clone,
    {
        use futures_util::StreamExt;

        if let Some(execute) = &self.execute {
            let result = execute(input, options);
            match result {
                ToolExecutionOutput::Single(future) => Some(future.await),
                ToolExecutionOutput::Streaming(mut stream) => {
                    let mut last_output: Option<Result<OUTPUT, Value>> = None;
                    while let Some(output) = stream.next().await {
                        // If we encounter an error in the stream, return it immediately
                        if output.is_err() {
                            return Some(output);
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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
        let result = tool.execute_tool(json!({"city": "SF"}), options).await;

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
        let result = tool.execute_tool(json!({}), options).await;

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

        let options = ToolExecuteOptions::new("call_123", vec![]);
        let result = tool.execute_tool(json!({}), options).await;

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
        let result = tool.execute_tool(json!({}), options).await;

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
        let result = tool.execute_tool(json!({}), options).await;

        // Should return the error immediately
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), json!("Error in stream"));
    }
}
