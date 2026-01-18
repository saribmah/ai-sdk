use llm_kit_provider::language_model::call_warning::LanguageModelCallWarning;
use llm_kit_provider::language_model::tool::LanguageModelTool;
use llm_kit_provider::language_model::tool_choice::LanguageModelToolChoice;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Hugging Face Responses API tool format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuggingFaceResponsesTool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub parameters: Value,
}

/// Hugging Face Responses API tool choice format.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HuggingFaceResponsesToolChoice {
    /// Auto mode - model decides whether to call tools
    Auto(String), // "auto"
    /// Required mode - model must call a tool
    Required(String), // "required"
    /// Specific function
    Function {
        #[serde(rename = "type")]
        choice_type: String,
        function: HuggingFaceResponsesFunctionChoice,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuggingFaceResponsesFunctionChoice {
    pub name: String,
}

/// Result of tool preparation.
pub struct PrepareToolsResult {
    pub tools: Option<Vec<HuggingFaceResponsesTool>>,
    pub tool_choice: Option<HuggingFaceResponsesToolChoice>,
    pub tool_warnings: Vec<LanguageModelCallWarning>,
}

/// Prepares tools for the Hugging Face Responses API.
pub fn prepare_responses_tools(
    tools: Option<Vec<LanguageModelTool>>,
    tool_choice: Option<LanguageModelToolChoice>,
) -> PrepareToolsResult {
    let mut tool_warnings = Vec::new();

    // If tools is empty, treat as None
    let tools = tools.and_then(|t| if t.is_empty() { None } else { Some(t) });

    // If no tools, return early
    let Some(tools) = tools else {
        return PrepareToolsResult {
            tools: None,
            tool_choice: None,
            tool_warnings,
        };
    };

    // Convert tools to Hugging Face format
    let mut hf_tools = Vec::new();

    for tool in tools {
        match tool {
            LanguageModelTool::Function(func_tool) => {
                hf_tools.push(HuggingFaceResponsesTool {
                    tool_type: "function".to_string(),
                    name: func_tool.name,
                    description: func_tool.description,
                    parameters: func_tool.input_schema,
                });
            }
            LanguageModelTool::ProviderDefined(tool) => {
                tool_warnings.push(LanguageModelCallWarning::UnsupportedTool {
                    tool: LanguageModelTool::ProviderDefined(tool),
                    details: Some(
                        "Provider-defined tools are not supported by Hugging Face Responses API"
                            .to_string(),
                    ),
                });
            }
        }
    }

    // Convert tool choice to Hugging Face format
    let hf_tool_choice = tool_choice.and_then(|choice| match choice {
        LanguageModelToolChoice::Auto => {
            Some(HuggingFaceResponsesToolChoice::Auto("auto".to_string()))
        }
        LanguageModelToolChoice::Required => Some(HuggingFaceResponsesToolChoice::Required(
            "required".to_string(),
        )),
        LanguageModelToolChoice::None => {
            // Not supported, ignore
            None
        }
        LanguageModelToolChoice::Tool { name } => Some(HuggingFaceResponsesToolChoice::Function {
            choice_type: "function".to_string(),
            function: HuggingFaceResponsesFunctionChoice { name },
        }),
    });

    PrepareToolsResult {
        tools: Some(hf_tools),
        tool_choice: hf_tool_choice,
        tool_warnings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_prepare_tools_empty() {
        let result = prepare_responses_tools(None, None);
        assert!(result.tools.is_none());
        assert!(result.tool_choice.is_none());
        assert!(result.tool_warnings.is_empty());
    }

    #[test]
    fn test_prepare_tools_function() {
        use llm_kit_provider::language_model::tool::function_tool::LanguageModelFunctionTool;

        let tools = vec![LanguageModelTool::Function(
            LanguageModelFunctionTool::new("get_weather", json!({ "type": "object" }))
                .with_description("Get weather"),
        )];

        let result = prepare_responses_tools(Some(tools), None);
        let hf_tools = result.tools.unwrap();
        assert_eq!(hf_tools.len(), 1);
        assert_eq!(hf_tools[0].name, "get_weather");
        assert_eq!(hf_tools[0].tool_type, "function");
    }

    #[test]
    fn test_prepare_tool_choice_auto() {
        use llm_kit_provider::language_model::tool::function_tool::LanguageModelFunctionTool;

        let result = prepare_responses_tools(
            Some(vec![LanguageModelTool::Function(
                LanguageModelFunctionTool::new("test", json!({})),
            )]),
            Some(LanguageModelToolChoice::Auto),
        );

        assert!(result.tool_choice.is_some());
        match result.tool_choice.unwrap() {
            HuggingFaceResponsesToolChoice::Auto(s) => assert_eq!(s, "auto"),
            _ => panic!("Expected Auto"),
        }
    }

    #[test]
    fn test_prepare_tool_choice_required() {
        use llm_kit_provider::language_model::tool::function_tool::LanguageModelFunctionTool;

        let result = prepare_responses_tools(
            Some(vec![LanguageModelTool::Function(
                LanguageModelFunctionTool::new("test", json!({})),
            )]),
            Some(LanguageModelToolChoice::Required),
        );

        assert!(result.tool_choice.is_some());
        match result.tool_choice.unwrap() {
            HuggingFaceResponsesToolChoice::Required(s) => assert_eq!(s, "required"),
            _ => panic!("Expected Required"),
        }
    }

    #[test]
    fn test_prepare_tool_choice_specific() {
        use llm_kit_provider::language_model::tool::function_tool::LanguageModelFunctionTool;

        let result = prepare_responses_tools(
            Some(vec![LanguageModelTool::Function(
                LanguageModelFunctionTool::new("test", json!({})),
            )]),
            Some(LanguageModelToolChoice::Tool {
                name: "test".to_string(),
            }),
        );

        assert!(result.tool_choice.is_some());
        match result.tool_choice.unwrap() {
            HuggingFaceResponsesToolChoice::Function {
                choice_type,
                function,
            } => {
                assert_eq!(choice_type, "function");
                assert_eq!(function.name, "test");
            }
            _ => panic!("Expected Function"),
        }
    }

    #[test]
    fn test_prepare_tool_choice_none() {
        use llm_kit_provider::language_model::tool::function_tool::LanguageModelFunctionTool;

        let result = prepare_responses_tools(
            Some(vec![LanguageModelTool::Function(
                LanguageModelFunctionTool::new("test", json!({})),
            )]),
            Some(LanguageModelToolChoice::None),
        );

        // None is not supported, should be ignored
        assert!(result.tool_choice.is_none());
    }
}
