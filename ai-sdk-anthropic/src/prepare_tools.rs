//! Tool preparation utilities for converting SDK tools to Anthropic API format.
//!
//! This module provides functions to prepare tools for use with the Anthropic API,
//! including converting from SDK provider-defined tools to Anthropic-specific formats
//! and collecting required beta feature flags.

use crate::prompt::tool::{AnthropicTool, AnthropicToolChoice};
use ai_sdk_provider::language_model::tool::LanguageModelTool;
use ai_sdk_provider::language_model::tool_choice::LanguageModelToolChoice;
use ai_sdk_provider_utils::tool::{Tool, ToolType};
use std::collections::HashSet;

/// Result of preparing tools for the Anthropic API.
#[derive(Debug, Clone)]
pub struct PreparedTools {
    /// The tools in Anthropic API format, or None if no tools
    pub tools: Option<Vec<AnthropicTool>>,

    /// The tool choice strategy, or None if not specified
    pub tool_choice: Option<AnthropicToolChoice>,

    /// Set of beta feature flags required for the tools
    pub betas: HashSet<String>,

    /// Warnings about unsupported tools or features
    pub warnings: Vec<ToolWarning>,
}

/// Warning about an unsupported tool or feature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolWarning {
    /// A tool is not supported
    UnsupportedTool {
        /// The tool ID or name
        tool_id: String,
    },

    /// A feature is not supported
    UnsupportedFeature {
        /// Description of the unsupported feature
        feature: String,
    },
}

/// Prepares tools from LanguageModelTool for use with the Anthropic API.
///
/// This is the main entry point for tool preparation, taking LanguageModelTool instances
/// (which include names for function tools) and converting them to Anthropic format.
///
/// # Arguments
///
/// * `tools` - Optional slice of LanguageModelTool to convert
/// * `tool_choice` - Optional tool choice strategy
/// * `disable_parallel_tool_use` - Whether to disable parallel tool use
///
/// # Returns
///
/// Returns a `PreparedTools` struct containing the converted tools, tool choice,
/// beta flags, and any warnings.
pub fn prepare_language_model_tools(
    tools: Option<&[LanguageModelTool]>,
    tool_choice: Option<&LanguageModelToolChoice>,
    disable_parallel_tool_use: bool,
) -> PreparedTools {
    // When the tools array is empty, treat it as None
    let tools = match tools {
        Some(slice) if !slice.is_empty() => Some(slice),
        _ => None,
    };

    let mut warnings = Vec::new();
    let mut betas = HashSet::new();

    let tools = match tools {
        None => None,
        Some(tools_slice) => {
            let mut anthropic_tools = Vec::new();

            for tool in tools_slice {
                match tool {
                    LanguageModelTool::Function(function_tool) => {
                        // Custom function tool - has a name field
                        let mut custom_tool = AnthropicTool::custom(
                            function_tool.name.clone(),
                            function_tool.input_schema.clone(),
                        );

                        // Add description if present
                        if let Some(ref description) = function_tool.description
                            && let AnthropicTool::Custom(ref mut t) = custom_tool
                        {
                            t.description = Some(description.clone());
                        }

                        // Add cache control if present in provider options
                        if let Some(ref provider_opts) = function_tool.provider_options
                            && let Some(anthropic_opts) = provider_opts.get("anthropic")
                            && let Some(cache_control) = anthropic_opts.get("cacheControl")
                            && let AnthropicTool::Custom(ref mut t) = custom_tool
                            && let Ok(cc) = serde_json::from_value(cache_control.clone())
                        {
                            t.cache_control = Some(cc);
                        }

                        anthropic_tools.push(custom_tool);
                    }

                    LanguageModelTool::ProviderDefined(provider_tool) => {
                        // Handle provider-defined tools
                        match provider_tool.id.as_str() {
                            "anthropic.code_execution_20250522" => {
                                betas.insert("code-execution-2025-05-22".to_string());
                                anthropic_tools.push(AnthropicTool::code_execution_20250522(
                                    &provider_tool.name,
                                ));
                            }

                            "anthropic.code_execution_20250825" => {
                                betas.insert("code-execution-2025-08-25".to_string());
                                anthropic_tools.push(AnthropicTool::code_execution_20250825(
                                    &provider_tool.name,
                                ));
                            }

                            "anthropic.computer_20250124" => {
                                betas.insert("computer-use-2025-01-24".to_string());
                                let display_width = provider_tool
                                    .args
                                    .get("displayWidthPx")
                                    .and_then(|v| v.as_u64())
                                    .unwrap_or(1024)
                                    as u32;
                                let display_height = provider_tool
                                    .args
                                    .get("displayHeightPx")
                                    .and_then(|v| v.as_u64())
                                    .unwrap_or(768)
                                    as u32;
                                let display_number = provider_tool
                                    .args
                                    .get("displayNumber")
                                    .and_then(|v| v.as_u64())
                                    .unwrap_or(0)
                                    as u32;

                                anthropic_tools.push(AnthropicTool::computer(
                                    &provider_tool.name,
                                    crate::prompt::tool::ComputerToolVersion::Computer20250124,
                                    display_width,
                                    display_height,
                                    display_number,
                                ));
                            }

                            "anthropic.computer_20241022" => {
                                betas.insert("computer-use-2024-10-22".to_string());
                                let display_width = provider_tool
                                    .args
                                    .get("displayWidthPx")
                                    .and_then(|v| v.as_u64())
                                    .unwrap_or(1024)
                                    as u32;
                                let display_height = provider_tool
                                    .args
                                    .get("displayHeightPx")
                                    .and_then(|v| v.as_u64())
                                    .unwrap_or(768)
                                    as u32;
                                let display_number = provider_tool
                                    .args
                                    .get("displayNumber")
                                    .and_then(|v| v.as_u64())
                                    .unwrap_or(0)
                                    as u32;

                                anthropic_tools.push(AnthropicTool::computer(
                                    &provider_tool.name,
                                    crate::prompt::tool::ComputerToolVersion::Computer20241022,
                                    display_width,
                                    display_height,
                                    display_number,
                                ));
                            }

                            "anthropic.text_editor_20250124" => {
                                betas.insert("computer-use-2025-01-24".to_string());
                                anthropic_tools.push(AnthropicTool::text_editor(
                                    &provider_tool.name,
                                    crate::prompt::tool::TextEditorToolVersion::TextEditor20250124,
                                ));
                            }

                            "anthropic.text_editor_20241022" => {
                                betas.insert("computer-use-2024-10-22".to_string());
                                anthropic_tools.push(AnthropicTool::text_editor(
                                    &provider_tool.name,
                                    crate::prompt::tool::TextEditorToolVersion::TextEditor20241022,
                                ));
                            }

                            "anthropic.text_editor_20250429" => {
                                betas.insert("computer-use-2025-01-24".to_string());
                                anthropic_tools.push(AnthropicTool::text_editor(
                                    &provider_tool.name,
                                    crate::prompt::tool::TextEditorToolVersion::TextEditor20250429,
                                ));
                            }

                            "anthropic.text_editor_20250728" => {
                                betas.insert("computer-use-2025-01-24".to_string());
                                let max_characters = provider_tool
                                    .args
                                    .get("maxCharacters")
                                    .and_then(|v| v.as_u64())
                                    .map(|v| v as u32);

                                let mut tool =
                                    AnthropicTool::text_editor_20250728(&provider_tool.name);
                                if let AnthropicTool::TextEditor20250728(ref mut t) = tool {
                                    t.max_characters = max_characters;
                                }
                                anthropic_tools.push(tool);
                            }

                            "anthropic.bash_20250124" => {
                                betas.insert("computer-use-2025-01-24".to_string());
                                anthropic_tools.push(AnthropicTool::bash(
                                    &provider_tool.name,
                                    crate::prompt::tool::BashToolVersion::Bash20250124,
                                ));
                            }

                            "anthropic.bash_20241022" => {
                                betas.insert("computer-use-2024-10-22".to_string());
                                anthropic_tools.push(AnthropicTool::bash(
                                    &provider_tool.name,
                                    crate::prompt::tool::BashToolVersion::Bash20241022,
                                ));
                            }

                            "anthropic.memory_20250818" => {
                                betas.insert("context-management-2025-06-27".to_string());
                                anthropic_tools.push(AnthropicTool::memory(&provider_tool.name));
                            }

                            "anthropic.web_fetch_20250910" => {
                                betas.insert("web-fetch-2025-09-10".to_string());
                                let mut tool = AnthropicTool::web_fetch(&provider_tool.name);

                                // Set optional parameters from args
                                if let AnthropicTool::WebFetch(ref mut t) = tool {
                                    if let Some(max_uses) =
                                        provider_tool.args.get("maxUses").and_then(|v| v.as_u64())
                                    {
                                        t.max_uses = Some(max_uses as u32);
                                    }
                                    if let Some(allowed) = provider_tool
                                        .args
                                        .get("allowedDomains")
                                        .and_then(|v| v.as_array())
                                    {
                                        t.allowed_domains = Some(
                                            allowed
                                                .iter()
                                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                                .collect(),
                                        );
                                    }
                                    if let Some(blocked) = provider_tool
                                        .args
                                        .get("blockedDomains")
                                        .and_then(|v| v.as_array())
                                    {
                                        t.blocked_domains = Some(
                                            blocked
                                                .iter()
                                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                                .collect(),
                                        );
                                    }
                                    if let Some(citations) = provider_tool
                                        .args
                                        .get("citations")
                                        .and_then(|v| v.as_object())
                                        && let Some(enabled) =
                                            citations.get("enabled").and_then(|v| v.as_bool())
                                    {
                                        use crate::prompt::message::content::document::DocumentCitations;
                                        t.citations = Some(DocumentCitations::new(enabled));
                                    }
                                    if let Some(max_tokens) = provider_tool
                                        .args
                                        .get("maxContentTokens")
                                        .and_then(|v| v.as_u64())
                                    {
                                        t.max_content_tokens = Some(max_tokens as u32);
                                    }
                                }

                                anthropic_tools.push(tool);
                            }

                            "anthropic.web_search_20250305" => {
                                betas.insert("web-search-2025-03-05".to_string());
                                let mut tool = AnthropicTool::web_search(&provider_tool.name);

                                // Set optional parameters from args
                                if let AnthropicTool::WebSearch(ref mut t) = tool {
                                    if let Some(max_uses) =
                                        provider_tool.args.get("maxUses").and_then(|v| v.as_u64())
                                    {
                                        t.max_uses = Some(max_uses as u32);
                                    }
                                    if let Some(allowed) = provider_tool
                                        .args
                                        .get("allowedDomains")
                                        .and_then(|v| v.as_array())
                                    {
                                        t.allowed_domains = Some(
                                            allowed
                                                .iter()
                                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                                .collect(),
                                        );
                                    }
                                    if let Some(blocked) = provider_tool
                                        .args
                                        .get("blockedDomains")
                                        .and_then(|v| v.as_array())
                                    {
                                        t.blocked_domains = Some(
                                            blocked
                                                .iter()
                                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                                .collect(),
                                        );
                                    }
                                    if let Some(location) = provider_tool
                                        .args
                                        .get("userLocation")
                                        .and_then(|v| v.as_object())
                                    {
                                        use crate::prompt::tool::UserLocation;
                                        let user_location = UserLocation {
                                            location_type: "approximate".to_string(),
                                            city: location
                                                .get("city")
                                                .and_then(|v| v.as_str())
                                                .map(|s| s.to_string()),
                                            region: location
                                                .get("region")
                                                .and_then(|v| v.as_str())
                                                .map(|s| s.to_string()),
                                            country: location
                                                .get("country")
                                                .and_then(|v| v.as_str())
                                                .map(|s| s.to_string()),
                                            timezone: location
                                                .get("timezone")
                                                .and_then(|v| v.as_str())
                                                .map(|s| s.to_string()),
                                        };
                                        t.user_location = Some(user_location);
                                    }
                                }

                                anthropic_tools.push(tool);
                            }

                            _ => {
                                warnings.push(ToolWarning::UnsupportedTool {
                                    tool_id: provider_tool.id.clone(),
                                });
                            }
                        }
                    }
                }
            }

            Some(anthropic_tools)
        }
    };

    // Handle tool choice
    let tool_choice = convert_tool_choice(tool_choice, disable_parallel_tool_use);

    PreparedTools {
        tools,
        tool_choice,
        betas,
        warnings,
    }
}

/// Converts SDK tool choice to Anthropic tool choice format.
fn convert_tool_choice(
    tool_choice: Option<&LanguageModelToolChoice>,
    disable_parallel_tool_use: bool,
) -> Option<AnthropicToolChoice> {
    match tool_choice {
        None => {
            // If no tool choice specified but parallel use is disabled, return auto with flag
            if disable_parallel_tool_use {
                Some(AnthropicToolChoice::auto().with_disable_parallel_tool_use(true))
            } else {
                None
            }
        }
        Some(LanguageModelToolChoice::Auto) => {
            if disable_parallel_tool_use {
                Some(AnthropicToolChoice::auto().with_disable_parallel_tool_use(true))
            } else {
                Some(AnthropicToolChoice::auto())
            }
        }
        Some(LanguageModelToolChoice::None) => {
            // Anthropic doesn't have a "none" tool choice - we just don't send tools
            // This should be handled at a higher level by not passing tools
            None
        }
        Some(LanguageModelToolChoice::Required) => {
            if disable_parallel_tool_use {
                Some(AnthropicToolChoice::any().with_disable_parallel_tool_use(true))
            } else {
                Some(AnthropicToolChoice::any())
            }
        }
        Some(LanguageModelToolChoice::Tool { name }) => {
            if disable_parallel_tool_use {
                Some(AnthropicToolChoice::tool(name).with_disable_parallel_tool_use(true))
            } else {
                Some(AnthropicToolChoice::tool(name))
            }
        }
    }
}

/// Prepares tools for use with the Anthropic API (legacy function using provider-utils Tool).
///
/// This function converts SDK provider-defined tools to Anthropic API format,
/// collects required beta flags, and handles tool choice conversion.
///
/// **Note:** This function is used internally and has limitations with function tool names.
/// Use `prepare_language_model_tools()` for the full conversion from LanguageModelTool.
///
/// # Arguments
///
/// * `tools` - Optional slice of SDK tools to convert
/// * `_tool_choice` - Optional tool choice strategy (deprecated parameter)
/// * `disable_parallel_tool_use` - Whether to disable parallel tool use
///
/// # Returns
///
/// Returns a `PreparedTools` struct containing the converted tools, tool choice,
/// beta flags, and any warnings.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::prepare_tools::prepare_tools;
/// use ai_sdk_anthropic::provider_tool::bash_20250124;
///
/// let tool = bash_20250124(None);
/// let prepared = prepare_tools(Some(&[tool]), None, false);
///
/// assert!(prepared.betas.contains("computer-use-2025-01-24"));
/// assert!(prepared.tools.is_some());
/// ```
pub fn prepare_tools(
    tools: Option<&[Tool]>,
    _tool_choice: Option<&str>, // Deprecated parameter
    disable_parallel_tool_use: bool,
) -> PreparedTools {
    // When the tools array is empty, treat it as None
    let tools = match tools {
        Some(slice) if !slice.is_empty() => Some(slice),
        _ => None,
    };

    let mut warnings = Vec::new();
    let mut betas = HashSet::new();

    let tools = match tools {
        None => None,
        Some(tools_slice) => {
            let mut anthropic_tools = Vec::new();

            for tool in tools_slice {
                match &tool.tool_type {
                    ToolType::Function => {
                        // Function tools from this path don't have names - this is a limitation
                        // Recommend using prepare_language_model_tools() instead
                        warnings.push(ToolWarning::UnsupportedFeature {
                            feature: "Function tools without names - use prepare_language_model_tools() instead".to_string(),
                        });
                    }

                    ToolType::ProviderDefined { id, name, args } => {
                        match id.as_str() {
                            "anthropic.code_execution_20250522" => {
                                betas.insert("code-execution-2025-05-22".to_string());
                                anthropic_tools.push(AnthropicTool::code_execution_20250522(name));
                            }

                            "anthropic.code_execution_20250825" => {
                                betas.insert("code-execution-2025-08-25".to_string());
                                anthropic_tools.push(AnthropicTool::code_execution_20250825(name));
                            }

                            "anthropic.computer_20250124" => {
                                betas.insert("computer-use-2025-01-24".to_string());
                                let display_width =
                                    args.get("displayWidthPx")
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(1024) as u32;
                                let display_height =
                                    args.get("displayHeightPx")
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(768) as u32;
                                let display_number =
                                    args.get("displayNumber")
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(0) as u32;

                                anthropic_tools.push(AnthropicTool::computer(
                                    name,
                                    crate::prompt::tool::ComputerToolVersion::Computer20250124,
                                    display_width,
                                    display_height,
                                    display_number,
                                ));
                            }

                            "anthropic.computer_20241022" => {
                                betas.insert("computer-use-2024-10-22".to_string());
                                let display_width =
                                    args.get("displayWidthPx")
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(1024) as u32;
                                let display_height =
                                    args.get("displayHeightPx")
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(768) as u32;
                                let display_number =
                                    args.get("displayNumber")
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(0) as u32;

                                anthropic_tools.push(AnthropicTool::computer(
                                    name,
                                    crate::prompt::tool::ComputerToolVersion::Computer20241022,
                                    display_width,
                                    display_height,
                                    display_number,
                                ));
                            }

                            "anthropic.text_editor_20250124" => {
                                betas.insert("computer-use-2025-01-24".to_string());
                                anthropic_tools.push(AnthropicTool::text_editor(
                                    name,
                                    crate::prompt::tool::TextEditorToolVersion::TextEditor20250124,
                                ));
                            }

                            "anthropic.text_editor_20241022" => {
                                betas.insert("computer-use-2024-10-22".to_string());
                                anthropic_tools.push(AnthropicTool::text_editor(
                                    name,
                                    crate::prompt::tool::TextEditorToolVersion::TextEditor20241022,
                                ));
                            }

                            "anthropic.text_editor_20250429" => {
                                betas.insert("computer-use-2025-01-24".to_string());
                                anthropic_tools.push(AnthropicTool::text_editor(
                                    name,
                                    crate::prompt::tool::TextEditorToolVersion::TextEditor20250429,
                                ));
                            }

                            "anthropic.text_editor_20250728" => {
                                betas.insert("computer-use-2025-01-24".to_string());
                                let max_characters = args
                                    .get("maxCharacters")
                                    .and_then(|v| v.as_u64())
                                    .map(|v| v as u32);

                                let mut tool = AnthropicTool::text_editor_20250728(name);
                                if let AnthropicTool::TextEditor20250728(ref mut t) = tool {
                                    t.max_characters = max_characters;
                                }
                                anthropic_tools.push(tool);
                            }

                            "anthropic.bash_20250124" => {
                                betas.insert("computer-use-2025-01-24".to_string());
                                anthropic_tools.push(AnthropicTool::bash(
                                    name,
                                    crate::prompt::tool::BashToolVersion::Bash20250124,
                                ));
                            }

                            "anthropic.bash_20241022" => {
                                betas.insert("computer-use-2024-10-22".to_string());
                                anthropic_tools.push(AnthropicTool::bash(
                                    name,
                                    crate::prompt::tool::BashToolVersion::Bash20241022,
                                ));
                            }

                            "anthropic.memory_20250818" => {
                                betas.insert("context-management-2025-06-27".to_string());
                                anthropic_tools.push(AnthropicTool::memory(name));
                            }

                            "anthropic.web_fetch_20250910" => {
                                betas.insert("web-fetch-2025-09-10".to_string());
                                let mut tool = AnthropicTool::web_fetch(name);

                                // Set optional parameters from args
                                if let AnthropicTool::WebFetch(ref mut t) = tool {
                                    if let Some(max_uses) =
                                        args.get("maxUses").and_then(|v| v.as_u64())
                                    {
                                        t.max_uses = Some(max_uses as u32);
                                    }
                                    if let Some(allowed) =
                                        args.get("allowedDomains").and_then(|v| v.as_array())
                                    {
                                        t.allowed_domains = Some(
                                            allowed
                                                .iter()
                                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                                .collect(),
                                        );
                                    }
                                    if let Some(blocked) =
                                        args.get("blockedDomains").and_then(|v| v.as_array())
                                    {
                                        t.blocked_domains = Some(
                                            blocked
                                                .iter()
                                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                                .collect(),
                                        );
                                    }
                                    if let Some(citations) =
                                        args.get("citations").and_then(|v| v.as_object())
                                        && let Some(enabled) =
                                            citations.get("enabled").and_then(|v| v.as_bool())
                                    {
                                        use crate::prompt::message::content::document::DocumentCitations;
                                        t.citations = Some(DocumentCitations::new(enabled));
                                    }
                                    if let Some(max_tokens) =
                                        args.get("maxContentTokens").and_then(|v| v.as_u64())
                                    {
                                        t.max_content_tokens = Some(max_tokens as u32);
                                    }
                                }

                                anthropic_tools.push(tool);
                            }

                            "anthropic.web_search_20250305" => {
                                betas.insert("web-search-2025-03-05".to_string());
                                let mut tool = AnthropicTool::web_search(name);

                                // Set optional parameters from args
                                if let AnthropicTool::WebSearch(ref mut t) = tool {
                                    if let Some(max_uses) =
                                        args.get("maxUses").and_then(|v| v.as_u64())
                                    {
                                        t.max_uses = Some(max_uses as u32);
                                    }
                                    if let Some(allowed) =
                                        args.get("allowedDomains").and_then(|v| v.as_array())
                                    {
                                        t.allowed_domains = Some(
                                            allowed
                                                .iter()
                                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                                .collect(),
                                        );
                                    }
                                    if let Some(blocked) =
                                        args.get("blockedDomains").and_then(|v| v.as_array())
                                    {
                                        t.blocked_domains = Some(
                                            blocked
                                                .iter()
                                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                                .collect(),
                                        );
                                    }
                                    if let Some(location) =
                                        args.get("userLocation").and_then(|v| v.as_object())
                                    {
                                        use crate::prompt::tool::UserLocation;
                                        let user_location = UserLocation {
                                            location_type: "approximate".to_string(),
                                            city: location
                                                .get("city")
                                                .and_then(|v| v.as_str())
                                                .map(|s| s.to_string()),
                                            region: location
                                                .get("region")
                                                .and_then(|v| v.as_str())
                                                .map(|s| s.to_string()),
                                            country: location
                                                .get("country")
                                                .and_then(|v| v.as_str())
                                                .map(|s| s.to_string()),
                                            timezone: location
                                                .get("timezone")
                                                .and_then(|v| v.as_str())
                                                .map(|s| s.to_string()),
                                        };
                                        t.user_location = Some(user_location);
                                    }
                                }

                                anthropic_tools.push(tool);
                            }

                            _ => {
                                warnings.push(ToolWarning::UnsupportedTool {
                                    tool_id: id.clone(),
                                });
                            }
                        }
                    }

                    ToolType::Dynamic => {
                        warnings.push(ToolWarning::UnsupportedFeature {
                            feature: "Dynamic tools are not yet supported".to_string(),
                        });
                    }
                }
            }

            Some(anthropic_tools)
        }
    };

    // Handle tool choice - simple version for legacy function
    let tool_choice = if disable_parallel_tool_use {
        Some(AnthropicToolChoice::auto().with_disable_parallel_tool_use(true))
    } else {
        None
    };

    PreparedTools {
        tools,
        tool_choice,
        betas,
        warnings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_sdk_provider::language_model::tool::function_tool::LanguageModelFunctionTool;
    use ai_sdk_provider::language_model::tool::provider_defined_tool::LanguageModelProviderDefinedTool;
    use serde_json::json;
    use std::collections::HashMap;

    // Tests for prepare_language_model_tools (new function)

    #[test]
    fn test_prepare_language_model_tools_empty() {
        let prepared = prepare_language_model_tools(None, None, false);

        assert!(prepared.tools.is_none());
        assert!(prepared.tool_choice.is_none());
        assert!(prepared.betas.is_empty());
        assert!(prepared.warnings.is_empty());
    }

    #[test]
    fn test_prepare_language_model_tools_function_tool() {
        let function_tool = LanguageModelFunctionTool::new(
            "get_weather",
            json!({
                "type": "object",
                "properties": {
                    "location": {"type": "string"}
                },
                "required": ["location"]
            }),
        )
        .with_description("Get the weather for a location");

        let tools = vec![LanguageModelTool::Function(function_tool)];
        let prepared = prepare_language_model_tools(Some(&tools), None, false);

        assert!(prepared.tools.is_some());
        let anthropic_tools = prepared.tools.unwrap();
        assert_eq!(anthropic_tools.len(), 1);
        assert_eq!(anthropic_tools[0].name(), "get_weather");
    }

    #[test]
    fn test_prepare_language_model_tools_provider_defined_bash() {
        let provider_tool = LanguageModelProviderDefinedTool::new(
            "anthropic.bash_20250124",
            "bash",
            HashMap::new(),
        );

        let tools = vec![LanguageModelTool::ProviderDefined(provider_tool)];
        let prepared = prepare_language_model_tools(Some(&tools), None, false);

        assert!(prepared.betas.contains("computer-use-2025-01-24"));
        assert!(prepared.tools.is_some());
        let anthropic_tools = prepared.tools.unwrap();
        assert_eq!(anthropic_tools.len(), 1);
        assert_eq!(anthropic_tools[0].name(), "bash");
    }

    #[test]
    fn test_prepare_language_model_tools_mixed() {
        let function_tool = LanguageModelFunctionTool::new("calculator", json!({"type": "object"}));

        let provider_tool = LanguageModelProviderDefinedTool::new(
            "anthropic.memory_20250818",
            "memory",
            HashMap::new(),
        );

        let tools = vec![
            LanguageModelTool::Function(function_tool),
            LanguageModelTool::ProviderDefined(provider_tool),
        ];

        let prepared = prepare_language_model_tools(Some(&tools), None, false);

        assert!(prepared.betas.contains("context-management-2025-06-27"));
        assert!(prepared.tools.is_some());
        let anthropic_tools = prepared.tools.unwrap();
        assert_eq!(anthropic_tools.len(), 2);
    }

    #[test]
    fn test_convert_tool_choice_auto() {
        let choice = convert_tool_choice(Some(&LanguageModelToolChoice::Auto), false);
        assert!(choice.is_some());
        assert!(matches!(choice.unwrap(), AnthropicToolChoice::Auto { .. }));
    }

    #[test]
    fn test_convert_tool_choice_required() {
        let choice = convert_tool_choice(Some(&LanguageModelToolChoice::Required), false);
        assert!(choice.is_some());
        assert!(matches!(choice.unwrap(), AnthropicToolChoice::Any { .. }));
    }

    #[test]
    fn test_convert_tool_choice_specific_tool() {
        let choice = convert_tool_choice(
            Some(&LanguageModelToolChoice::Tool {
                name: "get_weather".to_string(),
            }),
            false,
        );
        assert!(choice.is_some());
        let anthropic_choice = choice.unwrap();
        assert_eq!(anthropic_choice.tool_name(), Some("get_weather"));
    }

    #[test]
    fn test_convert_tool_choice_none() {
        let choice = convert_tool_choice(Some(&LanguageModelToolChoice::None), false);
        assert!(choice.is_none());
    }

    #[test]
    fn test_convert_tool_choice_with_disable_parallel() {
        let choice = convert_tool_choice(Some(&LanguageModelToolChoice::Auto), true);
        assert!(choice.is_some());
        let anthropic_choice = choice.unwrap();
        assert_eq!(anthropic_choice.disable_parallel_tool_use(), Some(true));
    }

    // Tests for legacy prepare_tools function

    #[test]
    fn test_prepare_tools_empty() {
        let prepared = prepare_tools(None, None, false);

        assert!(prepared.tools.is_none());
        assert!(prepared.tool_choice.is_none());
        assert!(prepared.betas.is_empty());
        assert!(prepared.warnings.is_empty());
    }

    #[test]
    fn test_prepare_tools_empty_slice() {
        let tools: Vec<Tool> = vec![];
        let prepared = prepare_tools(Some(&tools), None, false);

        assert!(prepared.tools.is_none());
        assert!(prepared.betas.is_empty());
    }

    #[test]
    fn test_prepare_tools_with_disable_parallel() {
        let prepared = prepare_tools(None, None, true);

        assert!(prepared.tool_choice.is_some());
        let choice = prepared.tool_choice.unwrap();
        assert_eq!(choice.disable_parallel_tool_use(), Some(true));
    }

    #[test]
    fn test_prepare_tools_bash_20250124() {
        use crate::provider_tool::bash_20250124;

        let tool = bash_20250124(None);
        let prepared = prepare_tools(Some(&[tool]), None, false);

        assert!(prepared.betas.contains("computer-use-2025-01-24"));
        assert!(prepared.tools.is_some());
        let tools = prepared.tools.unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name(), "bash");
    }

    #[test]
    fn test_prepare_tools_multiple() {
        use crate::provider_tool::{bash_20250124, memory_20250818};

        let bash = bash_20250124(None);
        let memory = memory_20250818(None);

        let prepared = prepare_tools(Some(&[bash, memory]), None, false);

        assert!(prepared.betas.contains("computer-use-2025-01-24"));
        assert!(prepared.betas.contains("context-management-2025-06-27"));
        assert!(prepared.tools.is_some());
        let tools = prepared.tools.unwrap();
        assert_eq!(tools.len(), 2);
    }

    #[test]
    fn test_prepare_tools_computer() {
        use crate::provider_tool::computer_20250124;

        let tool = computer_20250124(1920, 1080, Some(1));

        let prepared = prepare_tools(Some(&[tool]), None, false);

        assert!(prepared.betas.contains("computer-use-2025-01-24"));
        assert!(prepared.tools.is_some());
    }

    #[test]
    fn test_prepare_tools_web_search_with_location() {
        use crate::provider_tool::web_search_20250305;

        let tool = web_search_20250305()
            .user_location(
                "San Francisco",
                Some("CA"),
                Some("US"),
                Some("America/Los_Angeles"),
            )
            .max_uses(5)
            .build();

        let prepared = prepare_tools(Some(&[tool]), None, false);

        assert!(prepared.betas.contains("web-search-2025-03-05"));
        assert!(prepared.tools.is_some());
    }

    #[test]
    fn test_prepare_tools_web_fetch_with_domains() {
        use crate::provider_tool::web_fetch_20250910;

        let tool = web_fetch_20250910()
            .allowed_domains(vec!["example.com".to_string()])
            .citations(true)
            .build();

        let prepared = prepare_tools(Some(&[tool]), None, false);

        assert!(prepared.betas.contains("web-fetch-2025-09-10"));
        assert!(prepared.tools.is_some());
    }
}
