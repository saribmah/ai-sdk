/// Tool approval request types.
pub mod approval_request;
/// Output type for tool approval requests.
pub mod approval_request_output;
/// Tool approval response types.
pub mod approval_response;
/// Tool definition and execution logic.
pub mod callbacks;
/// Options for tool execution.
pub mod execute_options;
/// Factory for creating provider-defined tools.
pub mod provider_defined_factory;
/// Tool call type representing a function invocation.
pub mod tool_call;
/// Tool error type for failed tool executions.
pub mod tool_error;
/// Tool output type (either result or error).
pub mod tool_output;
/// Tool result type for successful tool executions.
pub mod tool_result;

use crate::tool::callbacks::{
    OnInputAvailableCallback, OnInputDeltaCallback, OnInputStartCallback, ToModelOutputFunction,
};
pub use approval_request::ToolApprovalRequest;
pub use approval_request_output::ToolApprovalRequestOutput;
pub use approval_response::ToolApprovalResponse;
pub use callbacks::{
    NeedsApproval, OnPreliminaryToolResult, ToolExecuteFunction, ToolExecutionOutput,
    ToolNeedsApprovalFunction, ToolType,
};
pub use execute_options::ToolExecuteOptions;
use llm_kit_provider::shared::provider_options::SharedProviderOptions;
pub use provider_defined_factory::{
    ProviderDefinedToolFactory, ProviderDefinedToolFactoryWithOutput, ProviderDefinedToolOptions,
};
use serde_json::Value;
use std::collections::HashMap;
pub use tool_call::ToolCall;
pub use tool_error::ToolError;
pub use tool_output::ToolOutput;
pub use tool_result::ToolResult;

/// A tool that can be called by a language model.
///
/// Tools enable language models to perform actions beyond text generation, such as:
/// - Retrieving information from external APIs
/// - Performing calculations
/// - Accessing databases
/// - Executing arbitrary code
/// - Interacting with external systems
///
/// # Tool Definition
///
/// A `Tool` consists of:
/// - **Input Schema**: JSON Schema describing the parameters the model should provide
/// - **Description**: Human-readable explanation helping the model decide when to use the tool
/// - **Execute Function**: Async function that performs the tool's action
/// - **Approval Settings**: Optional approval workflow before execution
///
/// # Tool Types
///
/// - **Function**: User-defined tools with custom execution logic
/// - **Dynamic**: Tools defined at runtime (e.g., from MCP servers)
/// - **Provider-Defined**: Tools with provider-specific schemas and implementations
///
/// # Cloneability
///
/// Tools are cloneable because all function pointers are wrapped in `Arc`. This allows
/// tools to be shared across multiple agents or included in multiple tool sets.
///
/// # Examples
///
/// ## Basic Function Tool
///
/// ```no_run
/// use llm_kit_provider_utils::tool::{Tool, ToolExecutionOutput};
/// use serde_json::json;
/// use std::sync::Arc;
///
/// let weather_tool = Tool::function(json!({
///     "type": "object",
///     "properties": {
///         "location": {
///             "type": "string",
///             "description": "City name"
///         }
///     },
///     "required": ["location"]
/// }))
/// .with_description("Get current weather for a location")
/// .with_execute(Arc::new(|input, _opts| {
///     ToolExecutionOutput::Single(Box::pin(async move {
///         // Parse input and fetch weather
///         Ok(json!({"temperature": 72, "condition": "sunny"}))
///     }))
/// }));
/// ```
///
/// ## Tool with Approval
///
/// ```no_run
/// # use llm_kit_provider_utils::tool::{Tool, ToolExecutionOutput};
/// # use serde_json::json;
/// # use std::sync::Arc;
/// # let schema = json!({"type": "object"});
/// # let execute_fn = Arc::new(|input, _opts| {
/// #     ToolExecutionOutput::Single(Box::pin(async move { Ok(json!({})) }))
/// # });
/// let delete_tool = Tool::function(schema)
///     .with_description("Delete a file")
///     .with_needs_approval(true)
///     .with_execute(execute_fn);
/// ```
///
/// ## Type-Safe Alternative
///
/// For compile-time type safety, use the `TypeSafeTool` trait instead:
///
/// ```ignore
/// # use serde::{Deserialize, Serialize};
/// # use schemars::JsonSchema;
/// # use llm_kit_provider_utils::tool::TypeSafeTool;
/// # use serde_json::Value;
/// # struct WeatherTool;
/// # #[derive(Serialize, Deserialize, JsonSchema)]
/// # struct WeatherOutput { temp: f64 }
/// #[derive(Serialize, Deserialize, JsonSchema)]
/// struct WeatherInput {
///     location: String,
/// }
///
/// impl TypeSafeTool for WeatherTool {
///     type Input = WeatherInput;
///     type Output = WeatherOutput;
/// #     fn name(&self) -> &str { "weather" }
/// #     fn description(&self) -> &str { "Get weather" }
/// #     fn execute(
/// #         &self,
/// #         _input: Self::Input,
/// #     ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Output, Value>> + Send>> {
/// #         Box::pin(async move { Ok(WeatherOutput { temp: 72.0 }) })
/// #     }
///     // ...
/// }
/// ```
///
#[derive(Clone)]
pub struct Tool {
    /// Description of what the tool does.
    ///
    /// This helps the language model decide when to use the tool. Should be
    /// clear and specific about the tool's purpose and when it should be called.
    pub description: Option<String>,

    /// Provider-specific options and metadata.
    ///
    /// Some providers support custom tool configurations. Use this to pass
    /// provider-specific settings.
    pub provider_options: Option<SharedProviderOptions>,

    /// JSON Schema describing the tool's input parameters.
    ///
    /// The model uses this schema to generate valid input arguments.
    /// Should be a valid JSON Schema object with properties and types.
    pub input_schema: Value,

    /// JSON Schema describing the tool's output (optional).
    ///
    /// Documents what the tool returns. Not used by most models, but helpful
    /// for documentation and validation.
    pub output_schema: Option<Value>,

    /// Whether the tool requires approval before execution.
    ///
    /// - `NeedsApproval::No`: Execute immediately
    /// - `NeedsApproval::Yes`: Always require approval
    /// - `NeedsApproval::Function(f)`: Conditional approval based on input
    pub needs_approval: NeedsApproval,

    /// The type of tool (function, dynamic, or provider-defined).
    pub tool_type: ToolType,

    /// Async function that executes the tool with the given input.
    ///
    /// Returns either a single value or a stream of values. Wrapped in `Arc`
    /// to allow cloning. If `None`, the tool is for schema-only purposes.
    pub execute: Option<ToolExecuteFunction<Value, Value>>,

    /// Callback invoked when tool input streaming starts.
    ///
    /// For tools that support streaming input, this is called before any deltas.
    pub on_input_start: Option<OnInputStartCallback>,

    /// Callback invoked for each input delta during streaming.
    ///
    /// For tools that support streaming input, this is called with each chunk.
    pub on_input_delta: Option<OnInputDeltaCallback>,

    /// Callback invoked when complete tool input is available.
    ///
    /// Called once the model has provided all input parameters, before execution.
    pub on_input_available: Option<OnInputAvailableCallback<Value>>,

    /// Function that converts tool output to model-compatible format.
    ///
    /// Use this to transform the tool's output before sending it back to the model.
    pub to_model_output: Option<ToModelOutputFunction<Value>>,
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

#[cfg(test)]
mod tests {
    use super::*;
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

        let tool: Tool = Tool::function(schema).with_execute(Arc::new(|input: Value, _options| {
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
            Tool::function(schema).with_execute(Arc::new(|_input: Value, _options| {
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
            Tool::function(schema).with_execute(Arc::new(|_input: Value, _options| {
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
            Tool::function(schema).with_execute(Arc::new(|_input: Value, _options| {
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
}
