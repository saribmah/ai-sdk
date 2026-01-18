//! OpenAI Chat Tool Preparation
//!
//! Converts SDK tools to OpenAI tool format.

use crate::chat::openai_chat_api::{
    FunctionDefinition, FunctionName, OpenAIChatFunctionTool, OpenAIChatToolChoice,
};
use llm_kit_provider::language_model::call_warning::LanguageModelCallWarning;
use llm_kit_provider::language_model::tool::LanguageModelTool;
use llm_kit_provider::language_model::tool_choice::LanguageModelToolChoice;

/// Prepare tools for OpenAI API call.
///
/// Converts SDK tools and tool choice to OpenAI format.
///
/// # Arguments
///
/// * `tools` - Optional list of tools
/// * `tool_choice` - Optional tool choice strategy
/// * `structured_outputs` - Whether structured outputs are enabled
/// * `strict_json_schema` - Whether strict JSON schema validation is enabled
///
/// # Returns
///
/// Tuple of (OpenAI tools, OpenAI tool choice, warnings)
pub fn prepare_chat_tools(
    tools: Option<&[LanguageModelTool]>,
    tool_choice: Option<&LanguageModelToolChoice>,
    structured_outputs: bool,
    strict_json_schema: bool,
) -> (
    Option<Vec<OpenAIChatFunctionTool>>,
    Option<OpenAIChatToolChoice>,
    Vec<LanguageModelCallWarning>,
) {
    let mut tool_warnings = Vec::new();

    // When the tools array is empty, change it to None to prevent errors
    let tools = match tools {
        Some(t) if !t.is_empty() => Some(t),
        _ => None,
    };

    if tools.is_none() {
        return (None, None, tool_warnings);
    }

    let tools = tools.unwrap();
    let mut openai_tools = Vec::new();

    for tool in tools {
        match tool {
            LanguageModelTool::Function(function_tool) => {
                openai_tools.push(OpenAIChatFunctionTool {
                    tool_type: "function".to_string(),
                    function: FunctionDefinition {
                        name: function_tool.name.clone(),
                        description: function_tool.description.clone(),
                        parameters: function_tool.input_schema.clone(),
                        strict: if structured_outputs {
                            Some(strict_json_schema)
                        } else {
                            None
                        },
                    },
                });
            }
            LanguageModelTool::ProviderDefined(provider_tool) => {
                tool_warnings.push(LanguageModelCallWarning::unsupported_tool(
                    LanguageModelTool::ProviderDefined(provider_tool.clone()),
                ));
            }
        }
    }

    let openai_tool_choice = match tool_choice {
        None => None,
        Some(LanguageModelToolChoice::Auto) => Some(OpenAIChatToolChoice::Auto("auto".to_string())),
        Some(LanguageModelToolChoice::None) => Some(OpenAIChatToolChoice::None("none".to_string())),
        Some(LanguageModelToolChoice::Required) => {
            Some(OpenAIChatToolChoice::Required("required".to_string()))
        }
        Some(LanguageModelToolChoice::Tool { name }) => Some(OpenAIChatToolChoice::Function {
            tool_type: "function".to_string(),
            function: FunctionName { name: name.clone() },
        }),
    };

    (Some(openai_tools), openai_tool_choice, tool_warnings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use llm_kit_provider::language_model::tool::function_tool::LanguageModelFunctionTool;
    use serde_json::json;

    #[test]
    fn test_no_tools() {
        let (tools, choice, warnings) = prepare_chat_tools(None, None, true, false);
        assert!(tools.is_none());
        assert!(choice.is_none());
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_empty_tools() {
        let (tools, choice, warnings) = prepare_chat_tools(Some(&[]), None, true, false);
        assert!(tools.is_none());
        assert!(choice.is_none());
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_function_tool() {
        let function_tool = LanguageModelTool::Function(LanguageModelFunctionTool::with_options(
            "get_weather",
            json!({
                "type": "object",
                "properties": {
                    "city": { "type": "string" }
                }
            }),
            Some("Get the weather".to_string()),
            None,
        ));

        let (tools, _choice, warnings) =
            prepare_chat_tools(Some(&[function_tool]), None, true, true);

        assert_eq!(tools.as_ref().unwrap().len(), 1);
        assert_eq!(tools.as_ref().unwrap()[0].function.name, "get_weather");
        assert_eq!(tools.as_ref().unwrap()[0].function.strict, Some(true));
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_tool_choice_auto() {
        let function_tool = LanguageModelTool::Function(LanguageModelFunctionTool::new(
            "test_tool",
            json!({ "type": "object" }),
        ));
        let (_, choice, _) = prepare_chat_tools(
            Some(&[function_tool]),
            Some(&LanguageModelToolChoice::Auto),
            true,
            false,
        );
        assert!(matches!(choice, Some(OpenAIChatToolChoice::Auto(_))));
    }

    #[test]
    fn test_tool_choice_required() {
        let function_tool = LanguageModelTool::Function(LanguageModelFunctionTool::new(
            "test_tool",
            json!({ "type": "object" }),
        ));
        let (_, choice, _) = prepare_chat_tools(
            Some(&[function_tool]),
            Some(&LanguageModelToolChoice::Required),
            true,
            false,
        );
        assert!(matches!(choice, Some(OpenAIChatToolChoice::Required(_))));
    }

    #[test]
    fn test_tool_choice_specific() {
        let function_tool = LanguageModelTool::Function(LanguageModelFunctionTool::new(
            "get_weather",
            json!({ "type": "object" }),
        ));
        let tool_choice = LanguageModelToolChoice::Tool {
            name: "get_weather".to_string(),
        };
        let (_, choice, _) =
            prepare_chat_tools(Some(&[function_tool]), Some(&tool_choice), true, false);

        if let Some(OpenAIChatToolChoice::Function { function, .. }) = choice {
            assert_eq!(function.name, "get_weather");
        } else {
            panic!("Expected Function tool choice");
        }
    }
}
