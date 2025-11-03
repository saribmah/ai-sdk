use ai_sdk_provider::language_model::{
    call_warning::LanguageModelCallWarning, tool::LanguageModelTool,
    tool_choice::LanguageModelToolChoice,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// OpenAI-compatible tool definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAICompatibleTool {
    /// The type of tool (always "function")
    #[serde(rename = "type")]
    pub tool_type: String,

    /// Function details
    pub function: OpenAICompatibleFunction,
}

/// OpenAI-compatible function definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAICompatibleFunction {
    /// The name of the function
    pub name: String,

    /// The description of the function
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The parameters schema for the function
    pub parameters: Value,
}

/// OpenAI-compatible tool choice
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAICompatibleToolChoice {
    /// Auto mode - let the model decide
    Auto(String),

    /// None mode - don't use tools
    None(String),

    /// Required mode - must use a tool
    Required(String),

    /// Specific tool choice
    Tool {
        #[serde(rename = "type")]
        tool_type: String,
        function: ToolFunction,
    },
}

/// Tool function name for specific tool choice
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolFunction {
    pub name: String,
}

/// Result of preparing tools for OpenAI-compatible API
#[derive(Debug, Clone, PartialEq)]
pub struct PrepareToolsResult {
    /// The prepared tools in OpenAI format
    pub tools: Option<Vec<OpenAICompatibleTool>>,

    /// The tool choice setting
    pub tool_choice: Option<OpenAICompatibleToolChoice>,

    /// Warnings generated during preparation
    pub tool_warnings: Vec<LanguageModelCallWarning>,
}

/// Prepares tools and tool choice for OpenAI-compatible API calls.
///
/// # Arguments
///
/// * `tools` - Optional vector of tools to prepare
/// * `tool_choice` - Optional tool choice setting
///
/// # Returns
///
/// A `PrepareToolsResult` containing the prepared tools, tool choice, and any warnings
///
/// # Behavior
///
/// - Empty tools array is converted to None
/// - Provider-defined tools are filtered out and generate warnings
/// - Function tools are converted to OpenAI format
/// - Tool choice is converted to OpenAI format (auto/none/required/specific tool)
pub fn prepare_tools(
    tools: Option<Vec<LanguageModelTool>>,
    tool_choice: Option<LanguageModelToolChoice>,
) -> PrepareToolsResult {
    // When the tools array is empty, change it to None to prevent errors
    let tools = tools.and_then(|t| if t.is_empty() { None } else { Some(t) });

    let mut tool_warnings: Vec<LanguageModelCallWarning> = Vec::new();

    if tools.is_none() {
        return PrepareToolsResult {
            tools: None,
            tool_choice: None,
            tool_warnings,
        };
    }

    let tools = tools.unwrap();
    let mut openai_compat_tools: Vec<OpenAICompatibleTool> = Vec::new();

    for tool in tools {
        match tool {
            LanguageModelTool::ProviderDefined(_) => {
                tool_warnings.push(LanguageModelCallWarning::unsupported_tool(tool));
            }
            LanguageModelTool::Function(function_tool) => {
                openai_compat_tools.push(OpenAICompatibleTool {
                    tool_type: "function".to_string(),
                    function: OpenAICompatibleFunction {
                        name: function_tool.name,
                        description: function_tool.description,
                        parameters: function_tool.input_schema,
                    },
                });
            }
        }
    }

    if tool_choice.is_none() {
        return PrepareToolsResult {
            tools: Some(openai_compat_tools),
            tool_choice: None,
            tool_warnings,
        };
    }

    let tool_choice = tool_choice.unwrap();

    let openai_tool_choice = match tool_choice {
        LanguageModelToolChoice::Auto => Some(OpenAICompatibleToolChoice::Auto("auto".to_string())),
        LanguageModelToolChoice::None => Some(OpenAICompatibleToolChoice::None("none".to_string())),
        LanguageModelToolChoice::Required => {
            Some(OpenAICompatibleToolChoice::Required("required".to_string()))
        }
        LanguageModelToolChoice::Tool { name } => Some(OpenAICompatibleToolChoice::Tool {
            tool_type: "function".to_string(),
            function: ToolFunction { name },
        }),
    };

    PrepareToolsResult {
        tools: Some(openai_compat_tools),
        tool_choice: openai_tool_choice,
        tool_warnings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_sdk_provider::language_model::tool::function_tool::LanguageModelFunctionTool;
    use ai_sdk_provider::language_model::tool::provider_defined_tool::LanguageModelProviderDefinedTool;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn test_prepare_tools_none() {
        let result = prepare_tools(None, None);

        assert_eq!(result.tools, None);
        assert_eq!(result.tool_choice, None);
        assert_eq!(result.tool_warnings.len(), 0);
    }

    #[test]
    fn test_prepare_tools_empty_array() {
        let result = prepare_tools(Some(vec![]), None);

        assert_eq!(result.tools, None);
        assert_eq!(result.tool_choice, None);
        assert_eq!(result.tool_warnings.len(), 0);
    }

    #[test]
    fn test_prepare_tools_single_function() {
        let tool = LanguageModelTool::Function(LanguageModelFunctionTool::new(
            "get_weather",
            json!({
                "type": "object",
                "properties": {
                    "city": {"type": "string"}
                }
            }),
        ));

        let result = prepare_tools(Some(vec![tool]), None);

        assert!(result.tools.is_some());
        let tools = result.tools.unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].tool_type, "function");
        assert_eq!(tools[0].function.name, "get_weather");
        assert_eq!(result.tool_choice, None);
        assert_eq!(result.tool_warnings.len(), 0);
    }

    #[test]
    fn test_prepare_tools_with_description() {
        let tool = LanguageModelTool::Function(
            LanguageModelFunctionTool::new("get_weather", json!({"type": "object"}))
                .with_description("Get the current weather"),
        );

        let result = prepare_tools(Some(vec![tool]), None);

        let tools = result.tools.unwrap();
        assert_eq!(
            tools[0].function.description,
            Some("Get the current weather".to_string())
        );
    }

    #[test]
    fn test_prepare_tools_provider_defined_warning() {
        let provider_tool = LanguageModelTool::ProviderDefined(
            LanguageModelProviderDefinedTool::new("tool-1", "custom_tool", HashMap::new()),
        );

        let result = prepare_tools(Some(vec![provider_tool]), None);

        assert!(result.tools.is_some());
        assert_eq!(result.tools.unwrap().len(), 0); // Provider-defined tools filtered out
        assert_eq!(result.tool_warnings.len(), 1);
        assert!(matches!(
            result.tool_warnings[0],
            LanguageModelCallWarning::UnsupportedTool { .. }
        ));
    }

    #[test]
    fn test_prepare_tools_mixed() {
        let function_tool =
            LanguageModelTool::Function(LanguageModelFunctionTool::new("fn1", json!({})));
        let provider_tool = LanguageModelTool::ProviderDefined(
            LanguageModelProviderDefinedTool::new("tool-1", "custom_tool", HashMap::new()),
        );

        let result = prepare_tools(Some(vec![function_tool, provider_tool]), None);

        let tools = result.tools.unwrap();
        assert_eq!(tools.len(), 1); // Only function tool included
        assert_eq!(tools[0].function.name, "fn1");
        assert_eq!(result.tool_warnings.len(), 1);
    }

    #[test]
    fn test_prepare_tools_choice_auto() {
        let tool = LanguageModelTool::Function(LanguageModelFunctionTool::new("fn1", json!({})));

        let result = prepare_tools(Some(vec![tool]), Some(LanguageModelToolChoice::Auto));

        assert_eq!(
            result.tool_choice,
            Some(OpenAICompatibleToolChoice::Auto("auto".to_string()))
        );
    }

    #[test]
    fn test_prepare_tools_choice_none() {
        let tool = LanguageModelTool::Function(LanguageModelFunctionTool::new("fn1", json!({})));

        let result = prepare_tools(Some(vec![tool]), Some(LanguageModelToolChoice::None));

        assert_eq!(
            result.tool_choice,
            Some(OpenAICompatibleToolChoice::None("none".to_string()))
        );
    }

    #[test]
    fn test_prepare_tools_choice_required() {
        let tool = LanguageModelTool::Function(LanguageModelFunctionTool::new("fn1", json!({})));

        let result = prepare_tools(Some(vec![tool]), Some(LanguageModelToolChoice::Required));

        assert_eq!(
            result.tool_choice,
            Some(OpenAICompatibleToolChoice::Required("required".to_string()))
        );
    }

    #[test]
    fn test_prepare_tools_choice_specific_tool() {
        let tool =
            LanguageModelTool::Function(LanguageModelFunctionTool::new("get_weather", json!({})));

        let result = prepare_tools(
            Some(vec![tool]),
            Some(LanguageModelToolChoice::Tool {
                name: "get_weather".to_string(),
            }),
        );

        assert!(matches!(
            result.tool_choice,
            Some(OpenAICompatibleToolChoice::Tool { .. })
        ));

        if let Some(OpenAICompatibleToolChoice::Tool {
            tool_type,
            function,
        }) = result.tool_choice
        {
            assert_eq!(tool_type, "function");
            assert_eq!(function.name, "get_weather");
        }
    }

    #[test]
    fn test_prepare_tools_multiple_functions() {
        let tools = vec![
            LanguageModelTool::Function(LanguageModelFunctionTool::new("fn1", json!({}))),
            LanguageModelTool::Function(LanguageModelFunctionTool::new("fn2", json!({}))),
            LanguageModelTool::Function(LanguageModelFunctionTool::new("fn3", json!({}))),
        ];

        let result = prepare_tools(Some(tools), None);

        let result_tools = result.tools.unwrap();
        assert_eq!(result_tools.len(), 3);
        assert_eq!(result_tools[0].function.name, "fn1");
        assert_eq!(result_tools[1].function.name, "fn2");
        assert_eq!(result_tools[2].function.name, "fn3");
    }
}
