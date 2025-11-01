use crate::generate_text::tool_set::ToolSet;
use crate::prompt::message::tool::definition::Tool;
use ai_sdk_provider::{
    language_model::tool_choice::ToolChoice,
    language_model::{
        tool::Tool as ProviderTool, tool::function_tool::FunctionTool,
        tool::provider_defined_tool::ProviderDefinedTool,
    },
};
use serde_json::Value;

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
/// A tuple of (Option<Vec<ProviderTool>>, Option<ToolChoice>)
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::{ToolSet, prepare_tools_and_tool_choice};
/// use ai_sdk_core::message::tool::definition::Tool;
///
/// let mut tools = ToolSet::new();
/// tools.insert("my_tool".to_string(), Tool::function(...));
///
/// let (provider_tools, tool_choice) = prepare_tools_and_tool_choice(Some(&tools), None);
/// ```
pub fn prepare_tools_and_tool_choice(
    tools: Option<&ToolSet>,
    tool_choice: Option<ToolChoice>,
) -> (Option<Vec<ProviderTool>>, Option<ToolChoice>) {
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
    let prepared_tool_choice = tool_choice.or(Some(ToolChoice::Auto));

    (Some(language_model_tools), prepared_tool_choice)
}

/// Convert a core Tool to a provider Tool.
///
/// # Arguments
///
/// * `name` - The name of the tool (from the ToolSet key)
/// * `core_tool` - The tool definition
fn convert_tool_to_provider(name: String, core_tool: &Tool<Value, Value>) -> ProviderTool {
    use crate::prompt::message::tool::definition::ToolType;

    match &core_tool.tool_type {
        ToolType::Function => {
            let mut function_tool = FunctionTool::new(name, core_tool.input_schema.clone());

            if let Some(desc) = &core_tool.description {
                function_tool = function_tool.with_description(desc.clone());
            }
            if let Some(opts) = &core_tool.provider_options {
                function_tool = function_tool.with_provider_options(opts.clone());
            }

            ProviderTool::Function(function_tool)
        }
        ToolType::Dynamic => {
            // Dynamic tools are treated as function tools in the provider
            let mut function_tool = FunctionTool::new(name, core_tool.input_schema.clone());

            if let Some(desc) = &core_tool.description {
                function_tool = function_tool.with_description(desc.clone());
            }
            if let Some(opts) = &core_tool.provider_options {
                function_tool = function_tool.with_provider_options(opts.clone());
            }

            ProviderTool::Function(function_tool)
        }
        ToolType::ProviderDefined { id, name: _, args } => {
            // For provider-defined tools, use the HashMap key as the name
            let provider_tool = ProviderDefinedTool::new(id.clone(), name, args.clone());
            ProviderTool::ProviderDefined(provider_tool)
        }
    }
}
