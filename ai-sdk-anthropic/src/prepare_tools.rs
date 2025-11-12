//! Tool preparation utilities for converting SDK tools to Anthropic API format.
//!
//! This module provides functions to prepare tools for use with the Anthropic API,
//! including converting from SDK provider-defined tools to Anthropic-specific formats
//! and collecting required beta feature flags.

use crate::prompt::tool::{AnthropicTool, AnthropicToolChoice};
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

/// Prepares tools for use with the Anthropic API.
///
/// This function converts SDK provider-defined tools to Anthropic API format,
/// collects required beta flags, and handles tool choice conversion.
///
/// # Arguments
///
/// * `tools` - Optional slice of SDK tools to convert
/// * `tool_choice` - Optional tool choice strategy (not fully implemented yet)
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
    _tool_choice: Option<&str>, // Placeholder for future implementation
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
                        // Custom function tool
                        anthropic_tools.push(AnthropicTool::custom(
                            // Tool name is derived from the tool_type for function tools
                            // In the SDK, function tools don't have a name field directly,
                            // so we'll use a placeholder or skip for now
                            // This would need to be enhanced based on actual SDK structure
                            "function_tool".to_string(),
                            tool.input_schema.clone(),
                        ));
                    }

                    ToolType::ProviderDefined { id, name: _, args } => {
                        match id.as_str() {
                            "anthropic.code_execution_20250522" => {
                                betas.insert("code-execution-2025-05-22".to_string());
                                anthropic_tools
                                    .push(AnthropicTool::code_execution_20250522("code_execution"));
                            }

                            "anthropic.code_execution_20250825" => {
                                betas.insert("code-execution-2025-08-25".to_string());
                                anthropic_tools
                                    .push(AnthropicTool::code_execution_20250825("code_execution"));
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
                                    "computer",
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
                                    "computer",
                                    crate::prompt::tool::ComputerToolVersion::Computer20241022,
                                    display_width,
                                    display_height,
                                    display_number,
                                ));
                            }

                            "anthropic.text_editor_20250124" => {
                                betas.insert("computer-use-2025-01-24".to_string());
                                anthropic_tools.push(AnthropicTool::text_editor(
                                    "str_replace_editor",
                                    crate::prompt::tool::TextEditorToolVersion::TextEditor20250124,
                                ));
                            }

                            "anthropic.text_editor_20241022" => {
                                betas.insert("computer-use-2024-10-22".to_string());
                                anthropic_tools.push(AnthropicTool::text_editor(
                                    "str_replace_editor",
                                    crate::prompt::tool::TextEditorToolVersion::TextEditor20241022,
                                ));
                            }

                            "anthropic.text_editor_20250429" => {
                                betas.insert("computer-use-2025-01-24".to_string());
                                anthropic_tools.push(AnthropicTool::text_editor(
                                    "str_replace_based_edit_tool",
                                    crate::prompt::tool::TextEditorToolVersion::TextEditor20250429,
                                ));
                            }

                            "anthropic.text_editor_20250728" => {
                                betas.insert("computer-use-2025-01-24".to_string());
                                let max_characters = args
                                    .get("maxCharacters")
                                    .and_then(|v| v.as_u64())
                                    .map(|v| v as u32);

                                let mut tool = AnthropicTool::text_editor_20250728(
                                    "str_replace_based_edit_tool",
                                );
                                if let AnthropicTool::TextEditor20250728(ref mut t) = tool {
                                    t.max_characters = max_characters;
                                }
                                anthropic_tools.push(tool);
                            }

                            "anthropic.bash_20250124" => {
                                betas.insert("computer-use-2025-01-24".to_string());
                                anthropic_tools.push(AnthropicTool::bash(
                                    "bash",
                                    crate::prompt::tool::BashToolVersion::Bash20250124,
                                ));
                            }

                            "anthropic.bash_20241022" => {
                                betas.insert("computer-use-2024-10-22".to_string());
                                anthropic_tools.push(AnthropicTool::bash(
                                    "bash",
                                    crate::prompt::tool::BashToolVersion::Bash20241022,
                                ));
                            }

                            "anthropic.memory_20250818" => {
                                betas.insert("context-management-2025-06-27".to_string());
                                anthropic_tools.push(AnthropicTool::memory("memory"));
                            }

                            "anthropic.web_fetch_20250910" => {
                                betas.insert("web-fetch-2025-09-10".to_string());
                                let mut tool = AnthropicTool::web_fetch("web_fetch");

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
                                let mut tool = AnthropicTool::web_search("web_search");

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

    // Handle tool choice
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
