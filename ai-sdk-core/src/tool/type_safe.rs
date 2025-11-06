use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use super::definition::{Tool, ToolExecuteFunction, ToolExecutionOutput};
use super::options::ToolExecuteOptions;

/// A trait for defining type-safe tools with compile-time checked input and output types.
///
/// This trait allows you to define tools with strongly-typed inputs and outputs,
/// providing compile-time guarantees about the data flowing through your tools.
///
/// # Type Parameters
///
/// * `Input` - The input type for the tool (must be serializable and have JSON schema)
/// * `Output` - The output type from the tool (must be serializable)
///
/// # Example
///
/// ```
/// use ai_sdk_core::tool::TypeSafeTool;
/// use serde::{Deserialize, Serialize};
/// use schemars::JsonSchema;
/// use async_trait::async_trait;
///
/// #[derive(Debug, Serialize, Deserialize, JsonSchema)]
/// struct WeatherInput {
///     city: String,
/// }
///
/// #[derive(Debug, Serialize, Deserialize)]
/// struct WeatherOutput {
///     temperature: f64,
///     conditions: String,
/// }
///
/// struct WeatherTool;
///
/// #[async_trait]
/// impl TypeSafeTool for WeatherTool {
///     type Input = WeatherInput;
///     type Output = WeatherOutput;
///
///     fn name(&self) -> &str {
///         "get_weather"
///     }
///
///     fn description(&self) -> &str {
///         "Get the current weather for a city"
///     }
///
///     async fn execute(&self, input: Self::Input) -> Result<Self::Output, String> {
///         Ok(WeatherOutput {
///             temperature: 72.0,
///             conditions: format!("Sunny in {}", input.city),
///         })
///     }
/// }
/// ```
#[async_trait]
pub trait TypeSafeTool: Send + Sync {
    /// The input type for this tool.
    /// Must implement Serialize, Deserialize, and JsonSchema for automatic schema generation.
    type Input: Serialize + for<'de> Deserialize<'de> + JsonSchema + Send + 'static;

    /// The output type from this tool.
    /// Must implement Serialize and Deserialize for JSON conversion.
    type Output: Serialize + for<'de> Deserialize<'de> + Send + 'static;

    /// Returns the name of the tool.
    fn name(&self) -> &str;

    /// Returns a description of what the tool does.
    fn description(&self) -> &str;

    /// Executes the tool with the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The typed input for the tool
    ///
    /// # Returns
    ///
    /// A Result containing either the typed output or an error message
    async fn execute(&self, input: Self::Input) -> Result<Self::Output, String>;

    /// Converts this type-safe tool into an untyped Tool that can be used with the SDK.
    ///
    /// This method automatically:
    /// - Generates a JSON schema from the Input type
    /// - Wraps the execute function to handle serialization/deserialization
    /// - Provides proper error handling
    fn into_tool(self) -> Tool
    where
        Self: Sized + 'static,
    {
        // Generate JSON schema from the Input type
        let schema = schemars::schema_for!(Self::Input);
        let schema_value =
            serde_json::to_value(schema.schema).expect("Failed to convert schema to JSON value");

        let tool_arc = Arc::new(self);
        let description = tool_arc.description().to_string();

        // Create the execute function that wraps the typed execute
        let execute_fn: ToolExecuteFunction<Value, Value> =
            Box::new(move |input: Value, _options| {
                let tool = tool_arc.clone();

                ToolExecutionOutput::Single(Box::pin(async move {
                    // Deserialize the input to the typed Input
                    let typed_input: Self::Input = serde_json::from_value(input).map_err(|e| {
                        serde_json::json!({
                            "error": "Failed to parse input",
                            "details": e.to_string()
                        })
                    })?;

                    // Execute the tool
                    let output = tool.execute(typed_input).await.map_err(|e| {
                        serde_json::json!({
                            "error": "Tool execution failed",
                            "details": e
                        })
                    })?;

                    // Serialize the output back to JSON
                    serde_json::to_value(output).map_err(|e| {
                        serde_json::json!({
                            "error": "Failed to serialize output",
                            "details": e.to_string()
                        })
                    })
                }))
            });

        Tool::function(schema_value)
            .with_description(description)
            .with_execute(execute_fn)
    }
}

/// Helper macro to quickly create a type-safe tool from a closure.
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::create_tool;
///
/// let weather_tool = create_tool!(
///     name: "get_weather",
///     description: "Get weather for a city",
///     input: WeatherInput,
///     output: WeatherOutput,
///     execute: |input: WeatherInput| async move {
///         Ok(WeatherOutput {
///             temperature: 72.0,
///             conditions: format!("Sunny in {}", input.city),
///         })
///     }
/// );
/// ```
#[macro_export]
macro_rules! create_tool {
    (
        name: $name:expr,
        description: $desc:expr,
        input: $input_ty:ty,
        output: $output_ty:ty,
        execute: $exec:expr
    ) => {{
        struct GeneratedTool<F> {
            name: String,
            description: String,
            execute_fn: F,
        }

        #[async_trait::async_trait]
        impl<F, Fut> $crate::tool::TypeSafeTool for GeneratedTool<F>
        where
            F: Fn($input_ty) -> Fut + Send + Sync,
            Fut: std::future::Future<Output = Result<$output_ty, String>> + Send,
        {
            type Input = $input_ty;
            type Output = $output_ty;

            fn name(&self) -> &str {
                &self.name
            }

            fn description(&self) -> &str {
                &self.description
            }

            async fn execute(&self, input: Self::Input) -> Result<Self::Output, String> {
                (self.execute_fn)(input).await
            }
        }

        GeneratedTool {
            name: $name.to_string(),
            description: $desc.to_string(),
            execute_fn: $exec,
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
    struct TestInput {
        value: String,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestOutput {
        result: String,
    }

    struct TestTool;

    #[async_trait]
    impl TypeSafeTool for TestTool {
        type Input = TestInput;
        type Output = TestOutput;

        fn name(&self) -> &str {
            "test_tool"
        }

        fn description(&self) -> &str {
            "A test tool"
        }

        async fn execute(&self, input: Self::Input) -> Result<Self::Output, String> {
            Ok(TestOutput {
                result: format!("Processed: {}", input.value),
            })
        }
    }

    #[tokio::test]
    async fn test_type_safe_tool_execute() {
        let tool = TestTool;
        let input = TestInput {
            value: "hello".to_string(),
        };

        let result = tool.execute(input).await.unwrap();

        assert_eq!(
            result,
            TestOutput {
                result: "Processed: hello".to_string()
            }
        );
    }

    #[tokio::test]
    async fn test_type_safe_tool_into_tool() {
        let tool = TestTool;
        let untyped_tool = tool.into_tool();

        // Verify the tool has the correct properties
        assert!(untyped_tool.description.is_some());
        assert_eq!(untyped_tool.description.unwrap(), "A test tool");
        assert!(untyped_tool.execute.is_some());
    }

    #[tokio::test]
    async fn test_tool_execute_with_json() {
        let tool = TestTool;
        let untyped_tool = tool.into_tool();

        let input = serde_json::json!({
            "value": "test"
        });

        let result = untyped_tool
            .execute_tool(input, ToolExecuteOptions::new("call_123", vec![]), None::<fn(Value)>)
            .await
            .unwrap()
            .unwrap();

        let output: TestOutput = serde_json::from_value(result).unwrap();
        assert_eq!(output.result, "Processed: test");
    }

    #[tokio::test]
    async fn test_tool_execute_with_invalid_input() {
        let tool = TestTool;
        let untyped_tool = tool.into_tool();

        let input = serde_json::json!({
            "wrong_field": "test"
        });

        let result = untyped_tool
            .execute_tool(input, ToolExecuteOptions::new("call_123", vec![]), None::<fn(Value)>)
            .await
            .unwrap();

        assert!(result.is_err());
    }
}
