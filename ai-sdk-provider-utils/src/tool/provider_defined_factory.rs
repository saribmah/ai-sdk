use super::{NeedsApproval, Tool, ToolExecuteFunction, ToolNeedsApprovalFunction, ToolType};
use crate::tool::callbacks::{
    OnInputAvailableCallback, OnInputDeltaCallback, OnInputStartCallback, ToModelOutputFunction,
};
use serde_json::Value;
use std::collections::HashMap;

/// Options for creating a provider-defined tool.
///
/// This struct contains all the optional configuration that can be provided
/// when creating a provider-defined tool instance.
pub struct ProviderDefinedToolOptions {
    /// Description of what the tool does
    pub description: Option<String>,

    /// JSON Schema for the output (optional)
    pub output_schema: Option<Value>,

    /// Execute function for the tool
    pub execute: Option<ToolExecuteFunction<Value, Value>>,

    /// Whether the tool needs approval
    pub needs_approval: NeedsApproval,

    /// Function to determine if approval is needed
    pub needs_approval_function: Option<ToolNeedsApprovalFunction<Value>>,

    /// Convert tool output to model output format
    pub to_model_output: Option<ToModelOutputFunction<Value>>,

    /// Callback when tool input streaming starts
    pub on_input_start: Option<OnInputStartCallback>,

    /// Callback for each input delta during streaming
    pub on_input_delta: Option<OnInputDeltaCallback>,

    /// Callback when complete tool input is available
    pub on_input_available: Option<OnInputAvailableCallback<Value>>,

    /// Additional provider-specific arguments
    pub args: HashMap<String, Value>,
}

impl ProviderDefinedToolOptions {
    /// Creates a new `ProviderDefinedToolOptions` with default values.
    pub fn new() -> Self {
        Self {
            description: None,
            output_schema: None,
            execute: None,
            needs_approval: NeedsApproval::No,
            needs_approval_function: None,
            to_model_output: None,
            on_input_start: None,
            on_input_delta: None,
            on_input_available: None,
            args: HashMap::new(),
        }
    }

    /// Sets the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the output schema.
    pub fn with_output_schema(mut self, schema: Value) -> Self {
        self.output_schema = Some(schema);
        self
    }

    /// Sets the execute function.
    pub fn with_execute(mut self, execute: ToolExecuteFunction<Value, Value>) -> Self {
        self.execute = Some(execute);
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
        self.needs_approval = NeedsApproval::Function(func.clone());
        self.needs_approval_function = Some(func);
        self
    }

    /// Sets the to_model_output conversion function.
    pub fn with_to_model_output(mut self, func: ToModelOutputFunction<Value>) -> Self {
        self.to_model_output = Some(func);
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

    /// Adds a provider-specific argument.
    pub fn with_arg(mut self, key: impl Into<String>, value: Value) -> Self {
        self.args.insert(key.into(), value);
        self
    }

    /// Sets all provider-specific arguments.
    pub fn with_args(mut self, args: HashMap<String, Value>) -> Self {
        self.args = args;
        self
    }
}

impl Default for ProviderDefinedToolOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Factory for creating provider-defined tools.
///
/// This factory holds the core configuration for a provider-defined tool
/// (id, name, and input schema) and provides a method to create tool instances
/// with specific options.
///
/// # Example
///
/// ```rust
/// use llm_kit_provider_utils::tool::{ProviderDefinedToolFactory, ProviderDefinedToolOptions};
/// use serde_json::json;
///
/// // Create a factory for a code execution tool
/// let factory = ProviderDefinedToolFactory::new(
///     "anthropic.code_execution_20250522",
///     "code_execution",
///     json!({
///         "type": "object",
///         "properties": {
///             "code": {
///                 "type": "string",
///                 "description": "The Python code to execute"
///             }
///         },
///         "required": ["code"]
///     })
/// );
///
/// // Create a tool instance with options
/// let tool = factory.create(
///     ProviderDefinedToolOptions::new()
///         .with_description("Execute Python code in a sandbox")
/// );
///
/// assert_eq!(tool.tool_type, llm_kit_provider_utils::tool::ToolType::ProviderDefined {
///     id: "anthropic.code_execution_20250522".to_string(),
///     name: "code_execution".to_string(),
///     args: std::collections::HashMap::new(),
/// });
/// ```
#[derive(Clone)]
pub struct ProviderDefinedToolFactory {
    /// The ID of the tool. Should follow the format `<provider-name>.<unique-tool-name>`.
    pub id: String,

    /// The name of the tool that the user must use in the tool set.
    pub name: String,

    /// JSON Schema describing the tool's input parameters.
    pub input_schema: Value,
}

impl ProviderDefinedToolFactory {
    /// Creates a new provider-defined tool factory.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier for the tool (format: `provider.tool_name`)
    /// * `name` - The name users will use to reference this tool
    /// * `input_schema` - JSON Schema describing the input parameters
    ///
    /// # Example
    ///
    /// ```rust
    /// use llm_kit_provider_utils::tool::ProviderDefinedToolFactory;
    /// use serde_json::json;
    ///
    /// let factory = ProviderDefinedToolFactory::new(
    ///     "anthropic.bash_20241022",
    ///     "bash",
    ///     json!({
    ///         "type": "object",
    ///         "properties": {
    ///             "command": { "type": "string" }
    ///         }
    ///     })
    /// );
    /// ```
    pub fn new(id: impl Into<String>, name: impl Into<String>, input_schema: Value) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            input_schema,
        }
    }

    /// Creates a provider-defined tool with the given options.
    ///
    /// # Arguments
    ///
    /// * `options` - Configuration options for the tool instance
    ///
    /// # Example
    ///
    /// ```rust
    /// use llm_kit_provider_utils::tool::{ProviderDefinedToolFactory, ProviderDefinedToolOptions, ToolExecutionOutput};
    /// use serde_json::json;
    /// use std::sync::Arc;
    ///
    /// let factory = ProviderDefinedToolFactory::new(
    ///     "provider.calculator",
    ///     "calc",
    ///     json!({ "type": "object" })
    /// );
    ///
    /// let tool = factory.create(
    ///     ProviderDefinedToolOptions::new()
    ///         .with_description("Perform calculations")
    ///         .with_execute(Arc::new(|input, _opts| {
    ///             ToolExecutionOutput::Single(Box::pin(async move {
    ///                 Ok(json!({"result": 42}))
    ///             }))
    ///         }))
    /// );
    /// ```
    pub fn create(&self, options: ProviderDefinedToolOptions) -> Tool {
        let needs_approval = if let Some(func) = options.needs_approval_function {
            NeedsApproval::Function(func)
        } else {
            options.needs_approval
        };

        Tool {
            description: options.description,
            provider_options: None,
            input_schema: self.input_schema.clone(),
            output_schema: options.output_schema,
            needs_approval,
            tool_type: ToolType::ProviderDefined {
                id: self.id.clone(),
                name: self.name.clone(),
                args: options.args,
            },
            execute: options.execute,
            on_input_start: options.on_input_start,
            on_input_delta: options.on_input_delta,
            on_input_available: options.on_input_available,
            to_model_output: options.to_model_output,
        }
    }

    /// Creates a provider-defined tool with default options.
    ///
    /// This is a convenience method equivalent to calling `create(ProviderDefinedToolOptions::new())`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use llm_kit_provider_utils::tool::ProviderDefinedToolFactory;
    /// use serde_json::json;
    ///
    /// let factory = ProviderDefinedToolFactory::new(
    ///     "provider.tool",
    ///     "tool",
    ///     json!({ "type": "object" })
    /// );
    ///
    /// let tool = factory.create_default();
    /// ```
    pub fn create_default(&self) -> Tool {
        self.create(ProviderDefinedToolOptions::new())
    }
}

/// Factory for creating provider-defined tools with a fixed output schema.
///
/// This is a specialized version of `ProviderDefinedToolFactory` that includes
/// a predefined output schema, useful when the output format is known and fixed.
///
/// # Example
///
/// ```rust
/// use llm_kit_provider_utils::tool::{ProviderDefinedToolFactoryWithOutput, ProviderDefinedToolOptions};
/// use serde_json::json;
///
/// let factory = ProviderDefinedToolFactoryWithOutput::new(
///     "anthropic.code_execution_20250522",
///     "code_execution",
///     json!({
///         "type": "object",
///         "properties": {
///             "code": { "type": "string" }
///         }
///     }),
///     json!({
///         "type": "object",
///         "properties": {
///             "stdout": { "type": "string" },
///             "stderr": { "type": "string" },
///             "return_code": { "type": "number" }
///         }
///     })
/// );
///
/// let tool = factory.create(ProviderDefinedToolOptions::new());
/// assert!(tool.output_schema.is_some());
/// ```
#[derive(Clone)]
pub struct ProviderDefinedToolFactoryWithOutput {
    /// The ID of the tool. Should follow the format `<provider-name>.<unique-tool-name>`.
    pub id: String,

    /// The name of the tool that the user must use in the tool set.
    pub name: String,

    /// JSON Schema describing the tool's input parameters.
    pub input_schema: Value,

    /// JSON Schema describing the tool's output format.
    pub output_schema: Value,
}

impl ProviderDefinedToolFactoryWithOutput {
    /// Creates a new provider-defined tool factory with output schema.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier for the tool (format: `provider.tool_name`)
    /// * `name` - The name users will use to reference this tool
    /// * `input_schema` - JSON Schema describing the input parameters
    /// * `output_schema` - JSON Schema describing the output format
    ///
    /// # Example
    ///
    /// ```rust
    /// use llm_kit_provider_utils::tool::ProviderDefinedToolFactoryWithOutput;
    /// use serde_json::json;
    ///
    /// let factory = ProviderDefinedToolFactoryWithOutput::new(
    ///     "anthropic.bash_20241022",
    ///     "bash",
    ///     json!({
    ///         "type": "object",
    ///         "properties": {
    ///             "command": { "type": "string" }
    ///         }
    ///     }),
    ///     json!({
    ///         "type": "object",
    ///         "properties": {
    ///             "output": { "type": "string" },
    ///             "exit_code": { "type": "number" }
    ///         }
    ///     })
    /// );
    /// ```
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        input_schema: Value,
        output_schema: Value,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            input_schema,
            output_schema,
        }
    }

    /// Creates a provider-defined tool with the given options.
    ///
    /// Note: If `options.output_schema` is set, it will override the factory's output schema.
    ///
    /// # Arguments
    ///
    /// * `options` - Configuration options for the tool instance
    ///
    /// # Example
    ///
    /// ```rust
    /// use llm_kit_provider_utils::tool::{ProviderDefinedToolFactoryWithOutput, ProviderDefinedToolOptions};
    /// use serde_json::json;
    ///
    /// let factory = ProviderDefinedToolFactoryWithOutput::new(
    ///     "provider.tool",
    ///     "tool",
    ///     json!({ "type": "object" }),
    ///     json!({ "type": "string" })
    /// );
    ///
    /// let tool = factory.create(
    ///     ProviderDefinedToolOptions::new()
    ///         .with_description("A useful tool")
    /// );
    /// ```
    pub fn create(&self, mut options: ProviderDefinedToolOptions) -> Tool {
        // Use the factory's output schema if options doesn't override it
        if options.output_schema.is_none() {
            options.output_schema = Some(self.output_schema.clone());
        }

        let needs_approval = if let Some(func) = options.needs_approval_function {
            NeedsApproval::Function(func)
        } else {
            options.needs_approval
        };

        Tool {
            description: options.description,
            provider_options: None,
            input_schema: self.input_schema.clone(),
            output_schema: options.output_schema,
            needs_approval,
            tool_type: ToolType::ProviderDefined {
                id: self.id.clone(),
                name: self.name.clone(),
                args: options.args,
            },
            execute: options.execute,
            on_input_start: options.on_input_start,
            on_input_delta: options.on_input_delta,
            on_input_available: options.on_input_available,
            to_model_output: options.to_model_output,
        }
    }

    /// Creates a provider-defined tool with default options.
    ///
    /// This is a convenience method equivalent to calling `create(ProviderDefinedToolOptions::new())`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use llm_kit_provider_utils::tool::ProviderDefinedToolFactoryWithOutput;
    /// use serde_json::json;
    ///
    /// let factory = ProviderDefinedToolFactoryWithOutput::new(
    ///     "provider.tool",
    ///     "tool",
    ///     json!({ "type": "object" }),
    ///     json!({ "type": "string" })
    /// );
    ///
    /// let tool = factory.create_default();
    /// ```
    pub fn create_default(&self) -> Tool {
        self.create(ProviderDefinedToolOptions::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::sync::Arc;

    #[test]
    fn test_provider_defined_tool_options_new() {
        let options = ProviderDefinedToolOptions::new();

        assert!(options.description.is_none());
        assert!(options.output_schema.is_none());
        assert!(options.execute.is_none());
        assert!(matches!(options.needs_approval, NeedsApproval::No));
        assert!(options.args.is_empty());
    }

    #[test]
    fn test_provider_defined_tool_options_builder() {
        let mut args = HashMap::new();
        args.insert("key".to_string(), json!("value"));

        let options = ProviderDefinedToolOptions::new()
            .with_description("Test tool")
            .with_output_schema(json!({"type": "string"}))
            .with_needs_approval(true)
            .with_arg("key", json!("value"));

        assert_eq!(options.description, Some("Test tool".to_string()));
        assert_eq!(options.output_schema, Some(json!({"type": "string"})));
        assert!(matches!(options.needs_approval, NeedsApproval::Yes));
        assert_eq!(options.args.get("key"), Some(&json!("value")));
    }

    #[test]
    fn test_provider_defined_tool_factory_new() {
        let factory = ProviderDefinedToolFactory::new(
            "provider.tool",
            "tool_name",
            json!({"type": "object"}),
        );

        assert_eq!(factory.id, "provider.tool");
        assert_eq!(factory.name, "tool_name");
        assert_eq!(factory.input_schema, json!({"type": "object"}));
    }

    #[test]
    fn test_provider_defined_tool_factory_create_default() {
        let factory =
            ProviderDefinedToolFactory::new("anthropic.bash", "bash", json!({"type": "object"}));

        let tool = factory.create_default();

        assert!(tool.description.is_none());
        assert_eq!(tool.input_schema, json!({"type": "object"}));
        assert!(tool.output_schema.is_none());
        assert!(matches!(tool.needs_approval, NeedsApproval::No));
        assert!(tool.execute.is_none());

        if let ToolType::ProviderDefined { id, name, args } = &tool.tool_type {
            assert_eq!(id, "anthropic.bash");
            assert_eq!(name, "bash");
            assert!(args.is_empty());
        } else {
            panic!("Expected ProviderDefined tool type");
        }
    }

    #[test]
    fn test_provider_defined_tool_factory_create_with_options() {
        let factory = ProviderDefinedToolFactory::new(
            "provider.calc",
            "calculator",
            json!({
                "type": "object",
                "properties": {
                    "expression": {"type": "string"}
                }
            }),
        );

        let tool = factory.create(
            ProviderDefinedToolOptions::new()
                .with_description("Calculate expressions")
                .with_output_schema(json!({"type": "number"}))
                .with_needs_approval(true)
                .with_arg("precision", json!(2)),
        );

        assert_eq!(tool.description, Some("Calculate expressions".to_string()));
        assert_eq!(tool.output_schema, Some(json!({"type": "number"})));
        assert!(matches!(tool.needs_approval, NeedsApproval::Yes));

        if let ToolType::ProviderDefined { id, name, args } = &tool.tool_type {
            assert_eq!(id, "provider.calc");
            assert_eq!(name, "calculator");
            assert_eq!(args.get("precision"), Some(&json!(2)));
        } else {
            panic!("Expected ProviderDefined tool type");
        }
    }

    #[tokio::test]
    async fn test_provider_defined_tool_factory_with_execute() {
        use crate::tool::ToolExecuteOptions;
        use crate::tool::ToolExecutionOutput;

        let factory =
            ProviderDefinedToolFactory::new("provider.echo", "echo", json!({"type": "object"}));

        let tool = factory.create(ProviderDefinedToolOptions::new().with_execute(Arc::new(
            |input, _opts| ToolExecutionOutput::Single(Box::pin(async move { Ok(input) })),
        )));

        let options = ToolExecuteOptions::new("call_123", vec![]);
        let result = tool
            .execute_tool(json!({"test": "data"}), options, None::<fn(Value)>)
            .await;

        assert!(result.is_some());
        assert_eq!(result.unwrap().unwrap(), json!({"test": "data"}));
    }

    #[test]
    fn test_provider_defined_tool_factory_with_output_new() {
        let factory = ProviderDefinedToolFactoryWithOutput::new(
            "provider.tool",
            "tool_name",
            json!({"type": "object"}),
            json!({"type": "string"}),
        );

        assert_eq!(factory.id, "provider.tool");
        assert_eq!(factory.name, "tool_name");
        assert_eq!(factory.input_schema, json!({"type": "object"}));
        assert_eq!(factory.output_schema, json!({"type": "string"}));
    }

    #[test]
    fn test_provider_defined_tool_factory_with_output_create_default() {
        let factory = ProviderDefinedToolFactoryWithOutput::new(
            "anthropic.code_exec",
            "code_execution",
            json!({
                "type": "object",
                "properties": {
                    "code": {"type": "string"}
                }
            }),
            json!({
                "type": "object",
                "properties": {
                    "stdout": {"type": "string"},
                    "stderr": {"type": "string"}
                }
            }),
        );

        let tool = factory.create_default();

        assert!(tool.description.is_none());
        assert!(tool.output_schema.is_some());
        assert_eq!(
            tool.output_schema.unwrap(),
            json!({
                "type": "object",
                "properties": {
                    "stdout": {"type": "string"},
                    "stderr": {"type": "string"}
                }
            })
        );

        if let ToolType::ProviderDefined { id, name, args } = &tool.tool_type {
            assert_eq!(id, "anthropic.code_exec");
            assert_eq!(name, "code_execution");
            assert!(args.is_empty());
        } else {
            panic!("Expected ProviderDefined tool type");
        }
    }

    #[test]
    fn test_provider_defined_tool_factory_with_output_override() {
        let factory = ProviderDefinedToolFactoryWithOutput::new(
            "provider.tool",
            "tool",
            json!({"type": "object"}),
            json!({"type": "string"}),
        );

        // Override the output schema
        let tool = factory.create(
            ProviderDefinedToolOptions::new().with_output_schema(json!({"type": "number"})),
        );

        assert_eq!(tool.output_schema, Some(json!({"type": "number"})));
    }

    #[test]
    fn test_provider_defined_tool_factory_clone() {
        let factory =
            ProviderDefinedToolFactory::new("provider.tool", "tool", json!({"type": "object"}));

        let cloned = factory.clone();

        assert_eq!(factory.id, cloned.id);
        assert_eq!(factory.name, cloned.name);
        assert_eq!(factory.input_schema, cloned.input_schema);
    }

    #[test]
    fn test_provider_defined_tool_factory_with_output_clone() {
        let factory = ProviderDefinedToolFactoryWithOutput::new(
            "provider.tool",
            "tool",
            json!({"type": "object"}),
            json!({"type": "string"}),
        );

        let cloned = factory.clone();

        assert_eq!(factory.id, cloned.id);
        assert_eq!(factory.name, cloned.name);
        assert_eq!(factory.input_schema, cloned.input_schema);
        assert_eq!(factory.output_schema, cloned.output_schema);
    }

    #[test]
    fn test_options_default() {
        let options1 = ProviderDefinedToolOptions::default();
        let options2 = ProviderDefinedToolOptions::new();

        assert_eq!(options1.args.len(), options2.args.len());
        assert!(matches!(options1.needs_approval, NeedsApproval::No));
    }
}
