mod callbacks;
pub mod collect_tool_approvals;
pub mod content_part;
pub mod execute_tool_call;
pub mod generate_text_result;
pub mod generated_file;
pub mod is_approval_needed;
mod parse_tool_call;
mod prepare_step;
mod prepare_tools;
pub mod reasoning_output;
mod response_message;
mod retries;
pub mod source_output;
mod step_result;
mod stop_condition;
pub mod text_output;
pub mod to_response_messages;
pub mod tool_approval_request_output;
pub mod tool_call;
pub mod tool_call_repair_function;
pub mod tool_error;
pub mod tool_output;
pub mod tool_result;
pub mod tool_set;

pub use callbacks::{FinishEvent, OnFinish, OnStepFinish};
pub use collect_tool_approvals::{
    CollectedToolApproval, CollectedToolApprovals, collect_tool_approvals,
};
pub use content_part::ContentPart;
pub use execute_tool_call::{OnPreliminaryToolResult, execute_tool_call};
pub use generate_text_result::{GenerateTextResult, ResponseMetadata};
pub use generated_file::{GeneratedFile, GeneratedFileWithType};
pub use is_approval_needed::is_approval_needed;
pub use parse_tool_call::{parse_provider_executed_dynamic_tool_call, parse_tool_call};
pub use prepare_step::{PrepareStep, PrepareStepOptions, PrepareStepResult};
pub use prepare_tools::prepare_tools_and_tool_choice;
pub use reasoning_output::ReasoningOutput;
pub use response_message::ResponseMessage;
pub use retries::{RetryConfig, prepare_retries};
pub use source_output::SourceOutput;
pub use step_result::{RequestMetadata, StepResponseMetadata, StepResult};
pub use stop_condition::{
    HasToolCall, StepCountIs, StopCondition, has_tool_call, is_stop_condition_met, step_count_is,
};
pub use text_output::TextOutput;
pub use to_response_messages::to_response_messages;
pub use tool_approval_request_output::ToolApprovalRequestOutput;
pub use tool_call::{DynamicToolCall, StaticToolCall, TypedToolCall};
pub use tool_call_repair_function::{ToolCallRepairFunction, ToolCallRepairOptions, no_repair};
pub use tool_error::{DynamicToolError, StaticToolError, TypedToolError};
pub use tool_output::ToolOutput;
pub use tool_result::{DynamicToolResult, StaticToolResult, TypedToolResult};
pub use tool_set::ToolSet;

use crate::error::AISDKError;
use crate::prompt::message::Message;
use crate::prompt::{
    Prompt,
    call_settings::{CallSettings, prepare_call_settings},
    convert_to_language_model_prompt::convert_to_language_model_prompt,
    standardize::{StandardizedPrompt, validate_and_standardize},
};
use ai_sdk_provider::{
    language_model::tool_choice::LanguageModelToolChoice,
    language_model::{LanguageModel, call_options::LanguageModelCallOptions, usage::LanguageModelUsage},
    shared::provider_options::SharedProviderOptions,
};
use serde_json::Value;
use tokio_util::sync::CancellationToken;

/// Executes tool calls and returns the outputs.
///
/// This function takes a list of typed tool calls that need to be executed on the client side
/// (not provider-executed), looks up each tool in the tool set, and executes them.
///
/// # Arguments
///
/// * `tool_calls` - References to the typed tool calls to execute
/// * `tools` - The tool set containing tool definitions
/// * `messages` - The conversation messages for context (passed to each tool)
/// * `abort_signal` - Optional cancellation token for aborting tool execution
///
/// # Returns
///
/// A vector of tool outputs (results or errors). Tools that cannot be executed are skipped.
///
/// # Example
///
/// ```ignore
/// let outputs = execute_tools(
///     &tool_calls,
///     tool_set,
///     &messages,
///     Some(abort_signal),
/// ).await;
/// ```
async fn execute_tools(
    tool_calls: &[&TypedToolCall<Value>],
    tools: &ToolSet,
    messages: &[Message],
    abort_signal: Option<CancellationToken>,
) -> Vec<ToolOutput<Value, Value>> {
    let mut outputs = Vec::new();

    for &tool_call in tool_calls {
        // Execute the tool call with conversation context
        if let Some(output) = execute_tool_call(
            tool_call.clone(),
            tools,
            messages.to_vec(),
            abort_signal.clone(),
            None,
            None,
        )
        .await
        {
            outputs.push(output);
        }
    }

    outputs
}

/// Converts language model content, tool calls, and tool outputs into a unified content array.
///
/// This function takes the raw content from a language model response along with parsed tool calls
/// and executed tool outputs, and combines them into a single array of `ContentPart` items.
///
/// # Arguments
///
/// * `content` - The content array from the language model response
/// * `tool_calls` - Parsed tool calls that were found in the response
/// * `tool_outputs` - Tool outputs from client-executed tools
///
/// # Returns
///
/// A vector of `ContentPart` items representing all content including:
/// - Text, reasoning, source, and file parts (passed through)
/// - Tool calls (mapped from provider ToolCall to TypedToolCall)
/// - Provider-executed tool results (converted to TypedToolResult or TypedToolError)
/// - Client-executed tool outputs (appended at the end)
///
/// # Example
///
/// ```ignore
/// let content_parts = as_content(
///     response.content,
///     tool_calls,
///     tool_outputs,
/// );
/// ```
pub fn as_content(
    content: Vec<ai_sdk_provider::language_model::content::LanguageModelContent>,
    tool_calls: Vec<TypedToolCall<Value>>,
    tool_outputs: Vec<ToolOutput<Value, Value>>,
) -> Vec<ContentPart<Value, Value>> {
    use ai_sdk_provider::language_model::content::LanguageModelContent;

    let mut result = Vec::new();

    // Map over content parts
    for part in content {
        match part {
            // Convert provider Text to TextOutput
            LanguageModelContent::Text(text) => {
                result.push(ContentPart::Text(TextOutput::new(text.text)));
            }
            // Convert provider Reasoning to ReasoningOutput
            LanguageModelContent::Reasoning(reasoning) => {
                result.push(ContentPart::Reasoning(ReasoningOutput::new(reasoning.text)));
            }
            // Convert provider Source to SourceOutput
            LanguageModelContent::Source(source) => {
                result.push(ContentPart::Source(SourceOutput::new(source)));
            }
            // Skip File parts as they're not in the TypeScript ContentPart
            LanguageModelContent::File(_) => {
                // File parts are not included in ContentPart
            }

            // Convert provider ToolCall to TypedToolCall
            LanguageModelContent::ToolCall(provider_tool_call) => {
                // Find the matching TypedToolCall
                if let Some(typed_call) = tool_calls
                    .iter()
                    .find(|tc| {
                        let tc_id = match tc {
                            TypedToolCall::Static(c) => &c.tool_call_id,
                            TypedToolCall::Dynamic(c) => &c.tool_call_id,
                        };
                        tc_id == &provider_tool_call.tool_call_id
                    })
                    .cloned()
                {
                    result.push(ContentPart::ToolCall(typed_call));
                }
            }

            // Convert provider ToolResult to TypedToolResult or TypedToolError
            LanguageModelContent::ToolResult(provider_result) => {
                // Find the matching tool call to get the input
                let matching_call = tool_calls.iter().find(|tc| {
                    let tc_id = match tc {
                        TypedToolCall::Static(c) => &c.tool_call_id,
                        TypedToolCall::Dynamic(c) => &c.tool_call_id,
                    };
                    tc_id == &provider_result.tool_call_id
                });

                if let Some(tool_call) = matching_call {
                    let (input, is_dynamic) = match tool_call {
                        TypedToolCall::Static(c) => (c.input.clone(), false),
                        TypedToolCall::Dynamic(c) => (c.input.clone(), true),
                    };

                    // Check if this is an error result
                    if provider_result.is_error == Some(true) {
                        // Create TypedToolError
                        let error = if is_dynamic {
                            TypedToolError::Dynamic(
                                DynamicToolError::new(
                                    provider_result.tool_call_id.clone(),
                                    provider_result.tool_name.clone(),
                                    input,
                                    provider_result.result.clone(),
                                )
                                .with_provider_executed(true),
                            )
                        } else {
                            TypedToolError::Static(
                                StaticToolError::new(
                                    provider_result.tool_call_id.clone(),
                                    provider_result.tool_name.clone(),
                                    input,
                                    provider_result.result.clone(),
                                )
                                .with_provider_executed(true),
                            )
                        };

                        result.push(ContentPart::ToolError(error));
                    } else {
                        // Create TypedToolResult
                        let tool_result = if is_dynamic {
                            TypedToolResult::Dynamic(
                                DynamicToolResult::new(
                                    provider_result.tool_call_id.clone(),
                                    provider_result.tool_name.clone(),
                                    input,
                                    provider_result.result.clone(),
                                )
                                .with_provider_executed(true),
                            )
                        } else {
                            TypedToolResult::Static(
                                StaticToolResult::new(
                                    provider_result.tool_call_id.clone(),
                                    provider_result.tool_name.clone(),
                                    input,
                                    provider_result.result.clone(),
                                )
                                .with_provider_executed(true),
                            )
                        };

                        result.push(ContentPart::ToolResult(tool_result));
                    }
                }
            }
        }
    }

    // Append client-executed tool outputs
    for output in tool_outputs {
        match output {
            ToolOutput::Result(tool_result) => {
                result.push(ContentPart::ToolResult(tool_result));
            }
            ToolOutput::Error(tool_error) => {
                result.push(ContentPart::ToolError(tool_error));
            }
        }
    }

    result
}

/// Generate text using a language model.
///
/// This is the main user-facing function for text generation in the AI SDK.
/// It takes a prompt, model, settings, and optionally tools to generate text.
///
/// # Arguments
///
/// * `model` - The language model to use for generation
/// * `prompt` - The prompt to send to the model. Can be a simple string or structured messages.
/// * `settings` - Configuration settings for the generation (temperature, max tokens, etc.)
/// * `tools` - Optional tool set (HashMap of tool names to tools). The model needs to support calling tools.
/// * `tool_choice` - Optional tool choice strategy. Default: 'auto'.
/// * `provider_options` - Optional provider-specific options.
/// * `stop_when` - Optional stop condition(s) for multi-step generation. Can be a single condition
///   or multiple conditions. Any condition being met will stop generation. Default: `step_count_is(1)`.
/// * `prepare_step` - Optional function to customize settings for each step in multi-step generation.
/// * `on_step_finish` - Optional callback called after each step (LLM call) completes.
/// * `on_finish` - Optional callback called when all steps are finished and response is complete.
///
/// # Returns
///
/// Returns `Result<LanguageModelGenerateResponse, AISDKError>` - The generated response or validation error
///
/// # Errors
///
/// Returns `AISDKError::InvalidArgument` if any settings are invalid (e.g., non-finite temperature).
///
/// # Examples
///
/// ```ignore
/// use ai_sdk_core::{generate_text, step_count_is};
/// use ai_sdk_core::prompt::{Prompt, call_settings::CallSettings};
///
/// let prompt = Prompt::text("Tell me a joke");
/// let settings = CallSettings::default();
/// let response = generate_text(
///     model,
///     prompt,
///     settings,
///     None,
///     None,
///     None,
///     Some(vec![Box::new(step_count_is(1))]),
///     None,
///     None,
///     None,
/// ).await?;
/// println!("Response: {:?}", response.content);
/// ```
pub async fn generate_text(
    model: &dyn LanguageModel,
    prompt: Prompt,
    settings: CallSettings,
    tools: Option<ToolSet>,
    tool_choice: Option<LanguageModelToolChoice>,
    provider_options: Option<SharedProviderOptions>,
    stop_when: Option<Vec<Box<dyn StopCondition>>>,
    prepare_step: Option<Box<dyn PrepareStep>>,
    on_step_finish: Option<Box<dyn OnStepFinish>>,
    on_finish: Option<Box<dyn OnFinish>>,
) -> Result<GenerateTextResult<Value, Value>, AISDKError> {
    // Prepare stop conditions - default to step_count_is(1)
    let stop_conditions = stop_when.unwrap_or_else(|| vec![Box::new(step_count_is(1))]);

    // Prepare retries
    let retry_config = prepare_retries(settings.max_retries, settings.abort_signal.clone())?;

    // Prepare and validate call settings
    let prepared_settings = prepare_call_settings(&settings)?;

    // Initialize response messages array for multi-step generation
    let initial_prompt = validate_and_standardize(prompt)?;

    // Store the initial messages before the loop
    let initial_messages = initial_prompt.messages.clone();

    // Validate and standardize the prompt
    let mut response_messages: Vec<ResponseMessage> = Vec::new();

    // Initialize steps array for tracking all generation steps
    let mut steps: Vec<StepResult> = Vec::new();

    // Prepare tools and tool choice (done once before the loop)
    let (provider_tools, prepared_tool_choice) =
        prepare_tools_and_tool_choice(tools.as_ref(), tool_choice);

    // Do-while loop for multi-step generation
    loop {
        // Step 5: Create step input messages by combining initial messages with accumulated response messages
        let mut step_input_messages = initial_messages.clone();
        // Convert response messages to model messages and append to step_input_messages
        for response_msg in &response_messages {
            let model_msg = match response_msg {
                ResponseMessage::Assistant(msg) => Message::Assistant(msg.clone()),
                ResponseMessage::Tool(msg) => Message::Tool(msg.clone()),
            };
            step_input_messages.push(model_msg);
        }

        // Call prepare_step callback to allow step customization
        let prepare_step_result = if let Some(ref prepare_fn) = prepare_step {
            prepare_fn
                .prepare(&PrepareStepOptions {
                    steps: &steps,
                    step_number: steps.len(),
                    messages: &step_input_messages,
                })
                .await
        } else {
            None
        };

        // Apply prepare_step overrides
        let step_system = prepare_step_result
            .as_ref()
            .and_then(|r| r.system.clone())
            .or_else(|| initial_prompt.system.clone());

        let step_messages = prepare_step_result
            .as_ref()
            .and_then(|r| r.messages.clone())
            .unwrap_or_else(|| step_input_messages.clone());

        let step_tool_choice = prepare_step_result
            .as_ref()
            .and_then(|r| r.tool_choice.clone())
            .or(prepared_tool_choice.clone());

        let step_active_tools = prepare_step_result
            .as_ref()
            .and_then(|r| r.active_tools.clone());

        // Step 6: Convert to language model format (provider messages)
        let messages = convert_to_language_model_prompt(StandardizedPrompt {
            messages: step_messages,
            system: step_system,
        })?;

        // Step 7: Build CallOptions
        let mut call_options = LanguageModelCallOptions::new(messages);
        // Add prepared settings
        if let Some(max_tokens) = prepared_settings.max_output_tokens {
            call_options = call_options.with_max_output_tokens(max_tokens);
        }
        if let Some(temp) = prepared_settings.temperature {
            call_options = call_options.with_temperature(temp);
        }
        if let Some(top_p) = prepared_settings.top_p {
            call_options = call_options.with_top_p(top_p);
        }
        if let Some(top_k) = prepared_settings.top_k {
            call_options = call_options.with_top_k(top_k);
        }
        if let Some(penalty) = prepared_settings.presence_penalty {
            call_options = call_options.with_presence_penalty(penalty);
        }
        if let Some(penalty) = prepared_settings.frequency_penalty {
            call_options = call_options.with_frequency_penalty(penalty);
        }
        if let Some(ref sequences) = prepared_settings.stop_sequences {
            call_options = call_options.with_stop_sequences(sequences.clone());
        }
        if let Some(seed) = prepared_settings.seed {
            call_options = call_options.with_seed(seed);
        }

        // Add tools and tool choice
        // Filter tools based on active_tools if provided by prepare_step
        let step_provider_tools = if let Some(ref active_tool_names) = step_active_tools {
            provider_tools.as_ref().map(|tools_vec| {
                tools_vec
                    .iter()
                    .filter(|tool| {
                        // Check if this tool is in the active_tools list
                        active_tool_names.iter().any(|name| {
                            // Match against the tool name from the provider tool
                            match tool {
                                ai_sdk_provider::language_model::tool::LanguageModelTool::Function(f) => {
                                    f.name == *name
                                }
                                ai_sdk_provider::language_model::tool::LanguageModelTool::ProviderDefined(p) => {
                                    p.name == *name
                                }
                            }
                        })
                    })
                    .cloned()
                    .collect()
            })
        } else {
            provider_tools.clone()
        };

        if let Some(ref tools_vec) = step_provider_tools {
            call_options = call_options.with_tools(tools_vec.clone());
        }
        if let Some(ref choice) = step_tool_choice {
            call_options = call_options.with_tool_choice(choice.clone());
        }

        // Add headers and abort signal from settings
        if let Some(ref headers) = settings.headers {
            call_options = call_options.with_headers(headers.clone());
        }
        // Clone abort signal so we can use it later for tool execution
        let abort_signal_for_tools = settings.abort_signal.clone();
        if let Some(signal) = settings.abort_signal.clone() {
            call_options = call_options.with_abort_signal(signal);
        }

        // Add provider options
        if let Some(ref opts) = provider_options {
            call_options = call_options.with_provider_options(opts.clone());
        }

        // Step 8: Call model.do_generate with retry logic
        let response = retry_config
            .execute(|| {
                let call_options_clone = call_options.clone();
                async move {
                    model.do_generate(call_options_clone).await.map_err(|e| {
                        let error_string = e.to_string();
                        // Check if error contains retry hint and create appropriate error type
                        if let Some(retry_after) =
                            retries::extract_retry_delay_from_error(&error_string)
                        {
                            AISDKError::retryable_error_with_delay(error_string, retry_after)
                        } else {
                            AISDKError::model_error(error_string)
                        }
                    })
                }
            })
            .await?;

        // Step 9: Parse tool calls from the response
        use ai_sdk_provider::language_model::content::LanguageModelContent;

        let step_tool_calls: Vec<TypedToolCall<Value>> = if let Some(tool_set) = tools.as_ref() {
            response
                .content
                .iter()
                .filter_map(|part| {
                    if let LanguageModelContent::ToolCall(tool_call) = part {
                        Some(tool_call)
                    } else {
                        None
                    }
                })
                .map(|tool_call| {
                    // Parse each tool call against the tool set
                    parse_tool_call(tool_call, tool_set)
                })
                .collect::<Result<Vec<_>, _>>()?
        } else {
            // No tools provided, so no tool calls to parse
            Vec::new()
        };

        // Step 10: Filter and execute client tool calls
        // Note: In the TypeScript implementation, invalid tool calls are tracked separately.
        // In our Rust implementation, we fail fast on parsing errors, so all parsed tool calls are valid.

        // Filter client tool calls (those not executed by the provider)
        let client_tool_calls: Vec<&TypedToolCall<Value>> = step_tool_calls
            .iter()
            .filter(|tool_call| {
                let provider_executed = match tool_call {
                    TypedToolCall::Static(c) => c.provider_executed,
                    TypedToolCall::Dynamic(c) => c.provider_executed,
                };
                provider_executed != Some(true)
            })
            .collect();

        // Execute client tool calls and collect outputs
        let client_tool_outputs = if let Some(tool_set) = tools.as_ref() {
            execute_tools(
                &client_tool_calls,
                tool_set,
                &step_input_messages,
                abort_signal_for_tools,
            )
            .await
        } else {
            Vec::new()
        };

        // Store the count before moving client_tool_outputs
        let client_tool_outputs_count = client_tool_outputs.len();

        // Create step content using as_content (clone step_tool_calls since we borrowed it above)
        let step_content = as_content(
            response.content.clone(),
            step_tool_calls.clone(),
            client_tool_outputs,
        );

        // Append to messages for potential next step
        let step_response_messages = to_response_messages(step_content.clone(), tools.as_ref());
        for msg in step_response_messages {
            response_messages.push(ResponseMessage::from(msg));
        }

        // Create and push the current step result (using step_content, NOT response.content)
        let current_step_result = StepResult::new(
            step_content.clone(), // ← Use ContentPart, not provider Content
            response.finish_reason.clone(),
            response.usage.clone(),
            if response.warnings.is_empty() {
                None
            } else {
                Some(response.warnings.clone())
            },
            RequestMetadata {
                body: response.request.as_ref().and_then(|r| r.body.clone()),
            },
            response
                .response
                .clone()
                .map(|r| r.into())
                .unwrap_or_default(),
            response.provider_metadata.clone(),
        );

        steps.push(current_step_result.clone());

        // Call on_step_finish callback
        if let Some(ref callback) = on_step_finish {
            callback.call(current_step_result).await;
        }

        // Check loop termination conditions (do-while loop pattern)
        // Continue if:
        // - There are client tool calls AND
        // - All tool calls have outputs AND
        // - Stop conditions are not met
        let should_continue = !client_tool_calls.is_empty()
            && client_tool_outputs_count == client_tool_calls.len()
            && !is_stop_condition_met(&stop_conditions, &steps).await;

        if !should_continue {
            break;
        }
    }

    // Calculate total usage by summing all steps
    let total_usage = steps.iter().fold(LanguageModelUsage::default(), |acc, step| {
        let input_tokens = acc.input_tokens + step.usage.input_tokens;
        let output_tokens = acc.output_tokens + step.usage.output_tokens;
        let total_tokens = acc.total_tokens + step.usage.total_tokens;
        let reasoning_tokens = acc.reasoning_tokens + step.usage.reasoning_tokens;
        let cached_input_tokens = acc.cached_input_tokens + step.usage.cached_input_tokens;

        LanguageModelUsage {
            input_tokens,
            output_tokens,
            total_tokens,
            reasoning_tokens,
            cached_input_tokens,
        }
    });

    // Create the resolved output (placeholder for now, as there's no output specification yet)
    let resolved_output = Value::Null;

    // Call on_finish callback before returning
    if let Some(ref callback) = on_finish {
        // Get the final step
        if let Some(final_step) = steps.last() {
            let finish_event = FinishEvent::new(final_step, steps.clone());
            callback.call(finish_event).await;
        }
    }

    // Debug logging: Show all steps
    eprintln!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("🔍 DEBUG: Generation Steps Summary");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("Total steps: {}", steps.len());
    eprintln!(
        "Total usage: {} input, {} output, {} total tokens",
        total_usage.input_tokens, total_usage.output_tokens, total_usage.total_tokens
    );

    for (i, step) in steps.iter().enumerate() {
        eprintln!("\n📍 Step {} of {}:", i + 1, steps.len());
        eprintln!("  Finish reason: {:?}", step.finish_reason);
        eprintln!(
            "  Usage: {} in, {} out, {} total",
            step.usage.input_tokens, step.usage.output_tokens, step.usage.total_tokens
        );

        // Show text content
        let text = step.text();
        if !text.is_empty() {
            eprintln!(
                "  Text: {}",
                if text.len() > 100 {
                    format!("{}...", &text[..100])
                } else {
                    text
                }
            );
        }

        // Show tool calls
        let tool_calls = step.tool_calls();
        if !tool_calls.is_empty() {
            eprintln!("  Tool calls ({}):", tool_calls.len());
            for tc in tool_calls {
                use super::TypedToolCall;
                match tc {
                    TypedToolCall::Static(call) => {
                        eprintln!("    - {} (id: {})", call.tool_name, call.tool_call_id);
                        eprintln!("      Input: {:?}", call.input);
                        if let Some(executed) = call.provider_executed {
                            eprintln!("      Provider executed: {}", executed);
                        }
                    }
                    TypedToolCall::Dynamic(call) => {
                        eprintln!(
                            "    - {} (id: {}) [dynamic]",
                            call.tool_name, call.tool_call_id
                        );
                        eprintln!("      Input: {}", call.input);
                        if let Some(executed) = call.provider_executed {
                            eprintln!("      Provider executed: {}", executed);
                        }
                    }
                }
            }
        }

        // Show tool results
        let tool_results = step.tool_results();
        if !tool_results.is_empty() {
            eprintln!("  Tool results ({}):", tool_results.len());
            for tr in tool_results {
                use super::TypedToolResult;
                match tr {
                    TypedToolResult::Static(result) => {
                        eprintln!(
                            "    - {} (call_id: {})",
                            result.tool_name, result.tool_call_id
                        );
                        eprintln!("      Output: {:?}", result.output);
                        if let Some(executed) = result.provider_executed {
                            eprintln!("      Provider executed: {}", executed);
                        }
                    }
                    TypedToolResult::Dynamic(result) => {
                        eprintln!(
                            "    - {} (call_id: {}) [dynamic]",
                            result.tool_name, result.tool_call_id
                        );
                        eprintln!("      Output: {}", result.output);
                        if let Some(executed) = result.provider_executed {
                            eprintln!("      Provider executed: {}", executed);
                        }
                    }
                }
            }
        }

        // Show content types
        eprintln!("  Content parts: {}", step.content.len());
        for (j, content) in step.content.iter().enumerate() {
            let content_type = match content {
                ContentPart::Text(_) => "Text",
                ContentPart::ToolCall(_) => "ToolCall",
                ContentPart::ToolResult(_) => "ToolResult",
                ContentPart::ToolError(_) => "ToolError",
                ContentPart::Reasoning(_) => "Reasoning",
                ContentPart::Source(_) => "Source",
            };
            eprintln!("    [{}] {}", j, content_type);
        }
    }

    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Return the GenerateTextResult
    Ok(GenerateTextResult::from_steps(
        steps,
        total_usage,
        resolved_output,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_sdk_provider::language_model::{
        LanguageModelGenerateResponse, LanguageModelStreamResponse, call_options::LanguageModelCallOptions,
    };
    use async_trait::async_trait;
    use regex::Regex;
    use serde_json::Value;
    use std::collections::HashMap;

    // Mock LanguageModel for testing
    struct MockLanguageModel {
        provider_name: String,
        model_name: String,
    }

    impl MockLanguageModel {
        fn new() -> Self {
            Self {
                provider_name: "test-provider".to_string(),
                model_name: "test-model".to_string(),
            }
        }
    }

    #[async_trait]
    impl LanguageModel for MockLanguageModel {
        fn provider(&self) -> &str {
            &self.provider_name
        }

        fn model_id(&self) -> &str {
            &self.model_name
        }

        async fn supported_urls(&self) -> HashMap<String, Vec<Regex>> {
            HashMap::new()
        }

        async fn do_generate(
            &self,
            _options: LanguageModelCallOptions,
        ) -> Result<LanguageModelGenerateResponse, Box<dyn std::error::Error>> {
            // Return a basic mock response
            // For now, we just return a mock error to indicate that this is a test mock
            Err("Mock implementation - tests should not actually call do_generate".into())
        }

        async fn do_stream(
            &self,
            _options: LanguageModelCallOptions,
        ) -> Result<LanguageModelStreamResponse, Box<dyn std::error::Error>> {
            unimplemented!("Mock implementation")
        }
    }

    #[tokio::test]
    async fn test_generate_text_basic() {
        let model = MockLanguageModel::new();
        let prompt = Prompt::text("Tell me a joke");
        let settings = CallSettings::default();

        // This should validate settings and call do_generate
        // The mock returns an error, but that's expected
        let result = generate_text(
            &model, prompt, settings, None, None, None, None, None, None, None,
        )
        .await;
        assert!(result.is_err());
        // Check that it's a model error (from do_generate), not a validation error
        match result {
            Err(AISDKError::ModelError { .. }) => (), // Expected
            _ => panic!("Expected ModelError from mock do_generate"),
        }
    }

    #[tokio::test]
    async fn test_generate_text_with_settings() {
        let model = MockLanguageModel::new();
        let prompt =
            Prompt::text("What is the weather?").with_system("You are a helpful assistant");
        let settings = CallSettings::default()
            .with_temperature(0.7)
            .with_max_output_tokens(100);

        // This should validate settings and call do_generate
        // The mock returns an error, but that's expected
        let result = generate_text(
            &model, prompt, settings, None, None, None, None, None, None, None,
        )
        .await;
        assert!(result.is_err());
        match result {
            Err(AISDKError::ModelError { .. }) => (), // Expected
            _ => panic!("Expected ModelError from mock do_generate"),
        }
    }

    #[tokio::test]
    async fn test_generate_text_with_tool_choice() {
        let model = MockLanguageModel::new();
        let prompt = Prompt::text("Use a tool to check the weather");
        let settings = CallSettings::default();
        let tool_choice = Some(LanguageModelToolChoice::Auto);

        // This should validate settings and call do_generate
        // The mock returns an error, but that's expected
        let result = generate_text(
            &model,
            prompt,
            settings,
            None,
            tool_choice,
            None,
            None,
            None,
            None,
            None,
        )
        .await;
        assert!(result.is_err());
        match result {
            Err(AISDKError::ModelError { .. }) => (), // Expected
            _ => panic!("Expected ModelError from mock do_generate"),
        }
    }

    #[tokio::test]
    async fn test_generate_text_with_provider_options() {
        let model = MockLanguageModel::new();
        let prompt = Prompt::text("Hello, world!");
        let settings = CallSettings::default();
        let mut provider_options = SharedProviderOptions::new();
        provider_options.insert(
            "custom".to_string(),
            [("key".to_string(), Value::String("value".to_string()))]
                .iter()
                .cloned()
                .collect(),
        );

        // This should validate settings and call do_generate
        // The mock returns an error, but that's expected
        let result = generate_text(
            &model,
            prompt,
            settings,
            None,
            None,
            Some(provider_options),
            None,
            None,
            None,
            None,
        )
        .await;
        assert!(result.is_err());
        match result {
            Err(AISDKError::ModelError { .. }) => (), // Expected
            _ => panic!("Expected ModelError from mock do_generate"),
        }
    }

    #[tokio::test]
    async fn test_generate_text_with_invalid_temperature() {
        let model = MockLanguageModel::new();
        let prompt = Prompt::text("Hello");
        let settings = CallSettings::default().with_temperature(f64::NAN);

        // This should fail validation
        let result = generate_text(
            &model, prompt, settings, None, None, None, None, None, None, None,
        )
        .await;
        assert!(result.is_err());
        match result {
            Err(AISDKError::InvalidArgument { parameter, .. }) => {
                assert_eq!(parameter, "temperature");
            }
            _ => panic!("Expected InvalidArgument error for temperature"),
        }
    }

    #[tokio::test]
    async fn test_generate_text_with_invalid_max_tokens() {
        let model = MockLanguageModel::new();
        let prompt = Prompt::text("Hello");
        let settings = CallSettings::default().with_max_output_tokens(0);

        // This should fail validation
        let result = generate_text(
            &model, prompt, settings, None, None, None, None, None, None, None,
        )
        .await;
        assert!(result.is_err());
        match result {
            Err(AISDKError::InvalidArgument { parameter, .. }) => {
                assert_eq!(parameter, "maxOutputTokens");
            }
            _ => panic!("Expected InvalidArgument error for maxOutputTokens"),
        }
    }

    #[tokio::test]
    async fn test_generate_text_with_empty_messages() {
        let model = MockLanguageModel::new();
        let prompt = Prompt::messages(vec![]);
        let settings = CallSettings::default();

        // This should fail validation (empty messages)
        let result = generate_text(
            &model, prompt, settings, None, None, None, None, None, None, None,
        )
        .await;
        assert!(result.is_err());
        match result {
            Err(AISDKError::InvalidPrompt { message }) => {
                assert_eq!(message, "messages must not be empty");
            }
            _ => panic!("Expected InvalidPrompt error for empty messages"),
        }
    }
}
