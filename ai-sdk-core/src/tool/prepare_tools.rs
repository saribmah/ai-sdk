use crate::tool::tool_set::ToolSet;
use ai_sdk_provider::{
    language_model::tool::{
        LanguageModelTool, function_tool::LanguageModelFunctionTool,
        provider_defined_tool::LanguageModelProviderDefinedTool,
    },
    language_model::tool_choice::LanguageModelToolChoice,
};
use ai_sdk_provider_utils::tool::Tool;

/// Prepares tools and tool choice for the language model.
///
/// Converts a ToolSet (HashMap of tool names to tools) into provider tools
/// and prepares the tool choice strategy.
///
/// # Arguments
///
/// * `tools` - Optional reference to tool set (HashMap of tool names to tools)
/// * `tool_choice` - Optional tool choice strategy
///
/// # Returns
///
/// A tuple of (`Option<Vec<LanguageModelTool>>`, `Option<ToolChoice>`)
///
/// # Example
///
/// ```no_run
/// use ai_sdk_core::tool::{ToolSet, prepare_tools_and_tool_choice};
/// use ai_sdk_provider_utils::tool::{Tool, ToolExecutionOutput};
/// use serde_json::json;
/// use std::sync::Arc;
///
/// let mut tools = ToolSet::new();
/// let tool = Tool::function(json!({"type": "object"}))
///     .with_execute(Arc::new(|_input, _opts| {
///         ToolExecutionOutput::Single(Box::pin(async move { Ok(json!({})) }))
///     }));
/// tools.insert("my_tool".to_string(), tool);
///
/// let (provider_tools, tool_choice) = prepare_tools_and_tool_choice(Some(&tools), None);
/// ```
pub fn prepare_tools_and_tool_choice(
    tools: Option<&ToolSet>,
    tool_choice: Option<LanguageModelToolChoice>,
) -> (
    Option<Vec<LanguageModelTool>>,
    Option<LanguageModelToolChoice>,
) {
    // If no tools provided, return None for both
    if tools.is_none() || tools.map(|t| t.is_empty()).unwrap_or(true) {
        return (None, None);
    }

    let tools = tools.unwrap();
    let mut language_model_tools = Vec::new();

    // Convert each tool in the toolset to a provider tool
    for (name, tool) in tools {
        let provider_tool = convert_tool_to_provider(name.clone(), tool);
        language_model_tools.push(provider_tool);
    }

    // Prepare tool choice - if not specified, default to auto
    let prepared_tool_choice = tool_choice.or(Some(LanguageModelToolChoice::Auto));

    (Some(language_model_tools), prepared_tool_choice)
}

/// Convert a core Tool to a provider Tool.
///
/// # Arguments
///
/// * `name` - The name of the tool (from the ToolSet key)
/// * `core_tool` - The tool definition
fn convert_tool_to_provider(name: String, core_tool: &Tool) -> LanguageModelTool {
    use ai_sdk_provider_utils::tool::ToolType;

    match &core_tool.tool_type {
        ToolType::Function => {
            let mut function_tool =
                LanguageModelFunctionTool::new(name, core_tool.input_schema.clone());

            if let Some(desc) = &core_tool.description {
                function_tool = function_tool.with_description(desc.clone());
            }
            if let Some(opts) = &core_tool.provider_options {
                function_tool = function_tool.with_provider_options(opts.clone());
            }

            LanguageModelTool::Function(function_tool)
        }
        ToolType::Dynamic => {
            // Dynamic tools are treated as function tools in the provider
            let mut function_tool =
                LanguageModelFunctionTool::new(name, core_tool.input_schema.clone());

            if let Some(desc) = &core_tool.description {
                function_tool = function_tool.with_description(desc.clone());
            }
            if let Some(opts) = &core_tool.provider_options {
                function_tool = function_tool.with_provider_options(opts.clone());
            }

            LanguageModelTool::Function(function_tool)
        }
        ToolType::ProviderDefined { id, name: _, args } => {
            // For provider-defined tools, use the HashMap key as the name
            let provider_tool =
                LanguageModelProviderDefinedTool::new(id.clone(), name, args.clone());
            LanguageModelTool::ProviderDefined(provider_tool)
        }
    }
}
