use ai_sdk_provider::language_model::call_options::LanguageModelCallOptions;
use ai_sdk_provider::language_model::call_warning::LanguageModelCallWarning;
use serde_json::{Value, json};
use std::collections::HashSet;

use crate::convert_to_message_prompt::convert_to_message_prompt;
use crate::get_cache_control::CacheControlValidator;
use crate::options::{AnthropicProviderOptions, ThinkingType};

use super::model_limits::get_max_output_tokens_for_model;

/// Result of building request arguments
pub struct BuildArgsResult {
    pub args: Value,
    pub warnings: Vec<LanguageModelCallWarning>,
    pub betas: HashSet<String>,
    pub uses_json_response_tool: bool,
}

/// Build request arguments for the Anthropic Messages API
pub async fn build_request_args(
    model_id: &str,
    options: &LanguageModelCallOptions,
    stream: bool,
) -> Result<BuildArgsResult, Box<dyn std::error::Error>> {
    let mut warnings = Vec::new();

    // Handle unsupported settings
    if options.frequency_penalty.is_some() {
        warnings.push(LanguageModelCallWarning::UnsupportedSetting {
            setting: "frequencyPenalty".to_string(),
            details: None,
        });
    }

    if options.presence_penalty.is_some() {
        warnings.push(LanguageModelCallWarning::UnsupportedSetting {
            setting: "presencePenalty".to_string(),
            details: None,
        });
    }

    if options.seed.is_some() {
        warnings.push(LanguageModelCallWarning::UnsupportedSetting {
            setting: "seed".to_string(),
            details: None,
        });
    }

    // Parse provider options
    let anthropic_options: Option<AnthropicProviderOptions> =
        if let Some(provider_options) = &options.provider_options {
            if let Some(anthropic_opts) = provider_options.get("anthropic") {
                serde_json::from_value(json!(anthropic_opts)).ok()
            } else {
                None
            }
        } else {
            None
        };

    // TODO: Handle response format properly once LanguageModelResponseFormat is defined
    // For now, we'll just set this to None
    let json_response_tool: Option<Value> = None;

    // let json_response_tool = match &options.response_format {
    //     Some(format) if format.format_type == "json" => {
    //         if format.schema.is_none() {
    //             warnings.push(LanguageModelCallWarning::UnsupportedSetting {
    //                 setting: "responseFormat".to_string(),
    //                 details: Some(
    //                     "JSON response format requires a schema. The response format is ignored."
    //                         .to_string(),
    //                 ),
    //             });
    //             None
    //         } else {
    //             // Create a JSON response tool
    //             Some(json!({
    //                 "type": "function",
    //                 "name": "json",
    //                 "description": "Respond with a JSON object.",
    //                 "parameters": format.schema.as_ref().unwrap()
    //             }))
    //         }
    //     }
    //     _ => None,
    // };

    // Convert prompt to Anthropic format
    let send_reasoning = anthropic_options
        .as_ref()
        .and_then(|opts| opts.send_reasoning)
        .unwrap_or(true);

    let mut convert_warnings = Vec::new();
    let _cache_control_validator = CacheControlValidator::new();
    let _prompt_result = convert_to_message_prompt(
        options.prompt.clone(),
        send_reasoning,
        &mut convert_warnings,
        Some(CacheControlValidator::new()),
    )?;
    warnings.extend(convert_warnings);

    let mut betas: HashSet<String> = HashSet::new();
    // TODO: Extract betas from prompt_result once available
    // betas.extend(prompt_result.betas);
    // warnings.extend(prompt_result.warnings);

    // Handle thinking mode
    let is_thinking = anthropic_options
        .as_ref()
        .and_then(|opts| opts.thinking.as_ref())
        .map(|t| t.thinking_type == ThinkingType::Enabled)
        .unwrap_or(false);

    let thinking_budget = anthropic_options
        .as_ref()
        .and_then(|opts| opts.thinking.as_ref())
        .and_then(|t| t.budget_tokens);

    // Get max output tokens for the model
    let max_tokens_result = get_max_output_tokens_for_model(model_id);
    let mut max_tokens = options
        .max_output_tokens
        .map(|t| t as u64)
        .unwrap_or(max_tokens_result.max_output_tokens);

    // Build base arguments
    let mut args = json!({
        "model": model_id,
        "messages": _prompt_result.prompt.messages,
        "max_tokens": max_tokens,
    });

    // Add system message if present
    if let Some(system) = _prompt_result.prompt.system {
        args["system"] = json!(system);
    }

    // Add optional parameters
    if let Some(temperature) = options.temperature {
        args["temperature"] = json!(temperature);
    }
    if let Some(top_k) = options.top_k {
        args["top_k"] = json!(top_k);
    }
    if let Some(top_p) = options.top_p {
        args["top_p"] = json!(top_p);
    }
    if let Some(stop_sequences) = &options.stop_sequences {
        args["stop_sequences"] = json!(stop_sequences);
    }

    // Handle thinking mode settings
    if is_thinking {
        if thinking_budget.is_none() {
            return Err("Thinking mode requires a budget_tokens value".into());
        }

        args["thinking"] = json!({
            "type": "enabled",
            "budget_tokens": thinking_budget.unwrap()
        });

        // Remove unsupported parameters in thinking mode
        if options.temperature.is_some() {
            args.as_object_mut().unwrap().remove("temperature");
            warnings.push(LanguageModelCallWarning::UnsupportedSetting {
                setting: "temperature".to_string(),
                details: Some("temperature is not supported when thinking is enabled".to_string()),
            });
        }

        if options.top_k.is_some() {
            args.as_object_mut().unwrap().remove("top_k");
            warnings.push(LanguageModelCallWarning::UnsupportedSetting {
                setting: "topK".to_string(),
                details: Some("topK is not supported when thinking is enabled".to_string()),
            });
        }

        if options.top_p.is_some() {
            args.as_object_mut().unwrap().remove("top_p");
            warnings.push(LanguageModelCallWarning::UnsupportedSetting {
                setting: "topP".to_string(),
                details: Some("topP is not supported when thinking is enabled".to_string()),
            });
        }

        // Adjust max tokens for thinking budget
        max_tokens += thinking_budget.unwrap();
        args["max_tokens"] = json!(max_tokens);
    }

    // Limit max tokens for known models
    if max_tokens_result.known_model && max_tokens > max_tokens_result.max_output_tokens {
        if options.max_output_tokens.is_some() {
            warnings.push(LanguageModelCallWarning::UnsupportedSetting {
                setting: "maxOutputTokens".to_string(),
                details: Some(format!(
                    "{} (maxOutputTokens + thinkingBudget) is greater than {} {} max output tokens. The max output tokens have been limited to {}.",
                    max_tokens, model_id, max_tokens_result.max_output_tokens, max_tokens_result.max_output_tokens
                )),
            });
        }
        args["max_tokens"] = json!(max_tokens_result.max_output_tokens);
    }

    // Add MCP servers if present
    if let Some(ref anthropic_opts) = anthropic_options {
        if let Some(ref mcp_servers) = anthropic_opts.mcp_servers
            && !mcp_servers.is_empty()
        {
            betas.insert("mcp-client-2025-04-04".to_string());

            args["mcp_servers"] = json!(
                mcp_servers
                    .iter()
                    .map(|server| {
                        json!({
                            "type": server.server_type,
                            "name": server.name,
                            "url": server.url,
                            "authorization_token": server.authorization_token,
                            "tool_configuration": server.tool_configuration.as_ref().map(|tc| {
                                json!({
                                    "allowed_tools": tc.allowed_tools,
                                    "enabled": tc.enabled,
                                })
                            }),
                        })
                    })
                    .collect::<Vec<_>>()
            );
        }

        // Add container configuration
        if let Some(ref container) = anthropic_opts.container
            && let Some(ref skills) = container.skills
            && !skills.is_empty()
        {
            betas.insert("code-execution-2025-08-25".to_string());
            betas.insert("skills-2025-10-02".to_string());
            betas.insert("files-api-2025-04-14".to_string());

            args["container"] = json!({
                "id": container.id,
                "skills": skills.iter().map(|skill| {
                    json!({
                        "type": skill.skill_type,
                        "skill_id": skill.skill_id,
                        "version": skill.version,
                    })
                }).collect::<Vec<_>>(),
            });

            // TODO: Check if code_execution tool is present
            // This requires accessing tool IDs from LanguageModelTool
            // For now, we'll add a warning
            warnings.push(LanguageModelCallWarning::Other {
                message: "Ensure code execution tool is enabled when using skills".to_string(),
            });
        }
    }

    // Enable fine-grained tool streaming when streaming
    if stream {
        let tool_streaming = anthropic_options
            .as_ref()
            .and_then(|opts| opts.tool_streaming)
            .unwrap_or(true);

        if tool_streaming {
            betas.insert("fine-grained-tool-streaming-2025-05-14".to_string());
        }
    }

    // Prepare tools using the new prepare_language_model_tools function
    let uses_json_response_tool = json_response_tool.is_some();

    let disable_parallel_tool_use = anthropic_options
        .as_ref()
        .and_then(|opts| opts.disable_parallel_tool_use)
        .unwrap_or(false);

    // Prepare tools if present
    if let Some(ref tools) = options.tools {
        let prepared = crate::prepare_tools::prepare_language_model_tools(
            Some(tools.as_slice()),
            options.tool_choice.as_ref(),
            disable_parallel_tool_use,
        );

        // Convert tool warnings to call warnings
        warnings.extend(prepared.warnings.iter().map(|w| match w {
            crate::prepare_tools::ToolWarning::UnsupportedTool { tool_id } => {
                LanguageModelCallWarning::UnsupportedSetting {
                    setting: "tools".to_string(),
                    details: Some(format!("Unsupported tool: {}", tool_id)),
                }
            }
            crate::prepare_tools::ToolWarning::UnsupportedFeature { feature } => {
                LanguageModelCallWarning::UnsupportedSetting {
                    setting: "tools".to_string(),
                    details: Some(format!("Unsupported feature: {}", feature)),
                }
            }
        }));

        // Add beta flags from tool preparation
        betas.extend(prepared.betas);

        // Add tools to args if present
        if let Some(tools) = prepared.tools {
            args["tools"] = serde_json::to_value(tools)?;
        }

        // Add tool choice if present
        if let Some(tool_choice) = prepared.tool_choice {
            args["tool_choice"] = serde_json::to_value(tool_choice)?;
        }
    }

    // Add stream parameter if streaming
    if stream {
        args["stream"] = json!(true);
    }

    // Extract cache control warnings
    // TODO: Convert CacheControlWarning to LanguageModelCallWarning
    // For now, we'll skip this
    // warnings.extend(cache_control_validator.get_warnings());

    Ok(BuildArgsResult {
        args,
        warnings,
        betas,
        uses_json_response_tool,
    })
}
