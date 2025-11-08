mod callbacks;
pub mod generate_text_result;
pub mod generated_file;
mod prepare_step;
mod response_message;
mod retries;
mod step_result;
mod stop_condition;
pub mod to_response_messages;

use crate::output::{Output, ReasoningOutput, SourceOutput, TextOutput};
pub use callbacks::{FinishEvent, OnFinish, OnStepFinish};
pub use generate_text_result::{GenerateTextResult, ResponseMetadata};
pub use generated_file::{GeneratedFile, GeneratedFileWithType};
pub use prepare_step::{PrepareStep, PrepareStepOptions, PrepareStepResult};
pub use response_message::ResponseMessage;
pub use retries::{RetryConfig, prepare_retries};
pub use step_result::{RequestMetadata, StepResponseMetadata, StepResult};
pub use stop_condition::{
    HasToolCall, StepCountIs, StopCondition, has_tool_call, is_stop_condition_met, step_count_is,
};
pub use to_response_messages::to_response_messages;

use crate::error::AISDKError;
use crate::prompt::message::Message;
use crate::prompt::{
    Prompt,
    call_settings::{CallSettings, prepare_call_settings},
    convert_to_language_model_prompt::convert_to_language_model_prompt,
    standardize::{StandardizedPrompt, validate_and_standardize},
};
use crate::tool::{
    ToolCall, ToolError, ToolOutput, ToolResult, ToolSet, execute_tool_call, parse_tool_call,
    prepare_tools_and_tool_choice,
};
use ai_sdk_provider::{
    language_model::tool_choice::LanguageModelToolChoice,
    language_model::{
        LanguageModel, call_options::LanguageModelCallOptions, usage::LanguageModelUsage,
    },
    shared::provider_options::SharedProviderOptions,
};
use std::sync::Arc;
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
    tool_calls: &[&ToolCall],
    tools: &ToolSet,
    messages: &[Message],
    abort_signal: Option<CancellationToken>,
) -> Vec<ToolOutput> {
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
/// and executed tool outputs, and combines them into a single array of `Output` items.
///
/// # Arguments
///
/// * `content` - The content array from the language model response
/// * `tool_calls` - Parsed tool calls that were found in the response
/// * `tool_outputs` - Tool outputs from client-executed tools
///
/// # Returns
///
/// A vector of `Output` items representing all content including:
/// - Text, reasoning, source, and file parts (passed through)
/// - Tool calls (mapped from provider ToolCall to TypedToolCall)
/// - Provider-executed tool results (converted to TypedToolResult or TypedToolError)
/// - Client-executed tool outputs (appended at the end)
///
/// # Example
///
/// ```ignore
/// let content_parts = as_output(
///     response.content,
///     tool_calls,
///     tool_outputs,
/// );
/// ```
pub fn as_output(
    content: Vec<ai_sdk_provider::language_model::content::LanguageModelContent>,
    tool_calls: Vec<ToolCall>,
    tool_outputs: Vec<ToolOutput>,
) -> Vec<Output> {
    use ai_sdk_provider::language_model::content::LanguageModelContent;

    let mut result = Vec::new();

    // Map over content parts
    for part in content {
        match part {
            // Convert provider Text to TextOutput
            LanguageModelContent::Text(text) => {
                result.push(Output::Text(TextOutput::new(text.text)));
            }
            // Convert provider Reasoning to ReasoningOutput
            LanguageModelContent::Reasoning(reasoning) => {
                result.push(Output::Reasoning(ReasoningOutput::new(reasoning.text)));
            }
            // Convert provider Source to SourceOutput
            LanguageModelContent::Source(source) => {
                result.push(Output::Source(SourceOutput::new(source)));
            }
            // Convert provider File to GeneratedFile
            LanguageModelContent::File(file) => {
                use ai_sdk_provider::language_model::content::file::FileData;

                let generated_file = match &file.data {
                    FileData::Base64(base64) => {
                        GeneratedFile::from_base64(base64.clone(), file.media_type.clone())
                    }
                    FileData::Binary(bytes) => {
                        GeneratedFile::from_bytes(bytes.clone(), file.media_type.clone())
                    }
                };

                result.push(Output::File(generated_file));
            }

            // Convert provider ToolCall to TypedToolCall
            LanguageModelContent::ToolCall(provider_tool_call) => {
                // Find the matching TypedToolCall
                if let Some(typed_call) = tool_calls
                    .iter()
                    .find(|tc| tc.tool_call_id == provider_tool_call.tool_call_id)
                    .cloned()
                {
                    result.push(Output::ToolCall(typed_call));
                }
            }

            // Convert provider ToolResult to TypedToolResult or TypedToolError
            LanguageModelContent::ToolResult(provider_result) => {
                // Find the matching tool call to get the input
                let matching_call = tool_calls
                    .iter()
                    .find(|tc| tc.tool_call_id == provider_result.tool_call_id);

                if let Some(tool_call) = matching_call {
                    // Check if this is an error result
                    if provider_result.is_error == Some(true) {
                        // Create ToolError
                        let error = ToolError::new(
                            provider_result.tool_call_id.clone(),
                            provider_result.tool_name.clone(),
                            tool_call.input.clone(),
                            provider_result.result.clone(),
                        )
                        .with_provider_executed(true);

                        result.push(Output::ToolError(error));
                    } else {
                        // Create ToolResult
                        let tool_result = ToolResult::new(
                            provider_result.tool_call_id.clone(),
                            provider_result.tool_name.clone(),
                            tool_call.input.clone(),
                            provider_result.result.clone(),
                        )
                        .with_provider_executed(true);

                        result.push(Output::ToolResult(tool_result));
                    }
                }
            }
        }
    }

    // Append client-executed tool outputs
    for output in tool_outputs {
        match output {
            ToolOutput::Result(tool_result) => {
                result.push(Output::ToolResult(tool_result));
            }
            ToolOutput::Error(tool_error) => {
                result.push(Output::ToolError(tool_error));
            }
        }
    }

    result
}

/// Builder for generating text using a language model with fluent API.
///
/// This builder provides a chainable interface for configuring text generation.
///
/// # Examples
///
/// ```ignore
/// use ai_sdk_core::{GenerateText, step_count_is};
/// use ai_sdk_core::prompt::Prompt;
/// use std::sync::Arc;
///
/// let result = GenerateText::new(Arc::new(model), Prompt::text("Tell me a joke"))
///     .temperature(0.7)
///     .max_output_tokens(100)
///     .tools(my_tools)
///     .stop_when(vec![Box::new(step_count_is(1))])
///     .execute()
///     .await?;
/// ```
pub struct GenerateText {
    model: Arc<dyn LanguageModel>,
    prompt: Prompt,
    settings: CallSettings,
    tools: Option<ToolSet>,
    tool_choice: Option<LanguageModelToolChoice>,
    provider_options: Option<SharedProviderOptions>,
    stop_when: Option<Vec<Box<dyn StopCondition>>>,
    prepare_step: Option<Box<dyn PrepareStep>>,
    on_step_finish: Option<Box<dyn OnStepFinish>>,
    on_finish: Option<Box<dyn OnFinish>>,
}

impl GenerateText {
    /// Creates a new builder with the required model and prompt.
    pub fn new(model: Arc<dyn LanguageModel>, prompt: Prompt) -> Self {
        Self {
            model,
            prompt,
            settings: CallSettings::default(),
            tools: None,
            tool_choice: None,
            provider_options: None,
            stop_when: None,
            prepare_step: None,
            on_step_finish: None,
            on_finish: None,
        }
    }

    /// Sets the complete call settings.
    pub fn settings(mut self, settings: CallSettings) -> Self {
        self.settings = settings;
        self
    }

    /// Sets the temperature for generation.
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.settings = self.settings.with_temperature(temperature);
        self
    }

    /// Sets the maximum output tokens.
    pub fn max_output_tokens(mut self, max_tokens: u32) -> Self {
        self.settings = self.settings.with_max_output_tokens(max_tokens);
        self
    }

    /// Sets the top_p sampling parameter.
    pub fn top_p(mut self, top_p: f64) -> Self {
        self.settings = self.settings.with_top_p(top_p);
        self
    }

    /// Sets the top_k sampling parameter.
    pub fn top_k(mut self, top_k: u32) -> Self {
        self.settings = self.settings.with_top_k(top_k);
        self
    }

    /// Sets the presence penalty.
    pub fn presence_penalty(mut self, penalty: f64) -> Self {
        self.settings = self.settings.with_presence_penalty(penalty);
        self
    }

    /// Sets the frequency penalty.
    pub fn frequency_penalty(mut self, penalty: f64) -> Self {
        self.settings = self.settings.with_frequency_penalty(penalty);
        self
    }

    /// Sets the random seed for deterministic generation.
    pub fn seed(mut self, seed: u32) -> Self {
        self.settings = self.settings.with_seed(seed);
        self
    }

    /// Sets the stop sequences.
    pub fn stop_sequences(mut self, sequences: Vec<String>) -> Self {
        self.settings = self.settings.with_stop_sequences(sequences);
        self
    }

    /// Sets the maximum number of retries.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.settings = self.settings.with_max_retries(max_retries);
        self
    }

    /// Sets custom headers for the request.
    pub fn headers(mut self, headers: std::collections::HashMap<String, String>) -> Self {
        self.settings = self.settings.with_headers(headers);
        self
    }

    /// Sets the abort signal for cancellation.
    pub fn abort_signal(mut self, signal: CancellationToken) -> Self {
        self.settings = self.settings.with_abort_signal(signal);
        self
    }

    /// Sets the tools available for the model to use.
    pub fn tools(mut self, tools: ToolSet) -> Self {
        self.tools = Some(tools);
        self
    }

    /// Sets the tool choice strategy.
    pub fn tool_choice(mut self, choice: LanguageModelToolChoice) -> Self {
        self.tool_choice = Some(choice);
        self
    }

    /// Sets provider-specific options.
    pub fn provider_options(mut self, options: SharedProviderOptions) -> Self {
        self.provider_options = Some(options);
        self
    }

    /// Sets stop conditions for multi-step generation.
    pub fn stop_when(mut self, conditions: Vec<Box<dyn StopCondition>>) -> Self {
        self.stop_when = Some(conditions);
        self
    }

    /// Sets the prepare step callback.
    pub fn prepare_step(mut self, callback: Box<dyn PrepareStep>) -> Self {
        self.prepare_step = Some(callback);
        self
    }

    /// Sets the on_step_finish callback.
    pub fn on_step_finish(mut self, callback: Box<dyn OnStepFinish>) -> Self {
        self.on_step_finish = Some(callback);
        self
    }

    /// Sets the on_finish callback.
    pub fn on_finish(mut self, callback: Box<dyn OnFinish>) -> Self {
        self.on_finish = Some(callback);
        self
    }

    /// Executes the text generation with the configured settings.
    pub async fn execute(self) -> Result<GenerateTextResult, AISDKError> {
        // Prepare stop conditions - default to step_count_is(1)
        let stop_conditions = self
            .stop_when
            .unwrap_or_else(|| vec![Box::new(step_count_is(1))]);

        // Prepare retries
        let retry_config = prepare_retries(
            self.settings.max_retries,
            self.settings.abort_signal.clone(),
        )?;

        // Prepare and validate call settings
        let prepared_settings = prepare_call_settings(&self.settings)?;

        // Initialize response messages array for multi-step generation
        let initial_prompt = validate_and_standardize(self.prompt)?;

        // Store the initial messages before the loop
        let initial_messages = initial_prompt.messages.clone();

        // Validate and standardize the prompt
        let mut response_messages: Vec<ResponseMessage> = Vec::new();

        // Initialize steps array for tracking all generation steps
        let mut steps: Vec<StepResult> = Vec::new();

        // Prepare tools and tool choice (done once before the loop)
        let (provider_tools, prepared_tool_choice) =
            prepare_tools_and_tool_choice(self.tools.as_ref(), self.tool_choice);

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
            let prepare_step_result = if let Some(ref prepare_fn) = self.prepare_step {
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
            if let Some(ref headers) = self.settings.headers {
                call_options = call_options.with_headers(headers.clone());
            }
            // Clone abort signal so we can use it later for tool execution
            let abort_signal_for_tools = self.settings.abort_signal.clone();
            if let Some(signal) = self.settings.abort_signal.clone() {
                call_options = call_options.with_abort_signal(signal);
            }

            // Add provider options
            if let Some(ref opts) = self.provider_options {
                call_options = call_options.with_provider_options(opts.clone());
            }

            // Step 8: Call model.do_generate with retry logic
            let response = retry_config
                .execute(|| {
                    let call_options_clone = call_options.clone();
                    let model_clone = Arc::clone(&self.model);
                    async move {
                        model_clone
                            .do_generate(call_options_clone)
                            .await
                            .map_err(|e| {
                                let error_string = e.to_string();
                                // Check if error contains retry hint and create appropriate error type
                                if let Some(retry_after) =
                                    retries::extract_retry_delay_from_error(&error_string)
                                {
                                    AISDKError::retryable_error_with_delay(
                                        error_string,
                                        retry_after,
                                    )
                                } else {
                                    AISDKError::model_error(error_string)
                                }
                            })
                    }
                })
                .await?;

            // Step 9: Parse tool calls from the response
            use ai_sdk_provider::language_model::content::LanguageModelContent;

            let step_tool_calls: Vec<ToolCall> = if let Some(tool_set) = self.tools.as_ref() {
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
            let client_tool_calls: Vec<&ToolCall> = step_tool_calls
                .iter()
                .filter(|tool_call| tool_call.provider_executed != Some(true))
                .collect();

            // Execute client tool calls and collect outputs
            let client_tool_outputs = if let Some(tool_set) = self.tools.as_ref() {
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

            // Create step content using as_output(clone step_tool_calls since we borrowed it above)
            let step_content = as_output(
                response.content.clone(),
                step_tool_calls.clone(),
                client_tool_outputs,
            );

            // Append to messages for potential next step
            let step_response_messages =
                to_response_messages(step_content.clone(), self.tools.as_ref());
            for msg in step_response_messages {
                response_messages.push(msg);
            }

            // Create and push the current step result (using step_content, NOT response.content)
            let current_step_result = StepResult::new(
                step_content.clone(), // ← Use Output, not provider Content
                response.finish_reason.clone(),
                response.usage,
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
            if let Some(ref callback) = self.on_step_finish {
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
        let total_usage = steps
            .iter()
            .fold(LanguageModelUsage::default(), |acc, step| {
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

        // Return the GenerateTextResult
        Ok(GenerateTextResult::from_steps(steps, total_usage))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_sdk_provider::language_model::{
        LanguageModelGenerateResponse, LanguageModelStreamResponse,
        call_options::LanguageModelCallOptions,
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
        let model = Arc::new(MockLanguageModel::new());
        let prompt = Prompt::text("Tell me a joke");

        // This should validate settings and call do_generate
        // The mock returns an error, but that's expected
        let result = GenerateText::new(model, prompt).execute().await;
        assert!(result.is_err());
        // Check that it's a model error (from do_generate), not a validation error
        match result {
            Err(AISDKError::ModelError { .. }) => (), // Expected
            _ => panic!("Expected ModelError from mock do_generate"),
        }
    }

    #[tokio::test]
    async fn test_generate_text_builder_basic() {
        let model = Arc::new(MockLanguageModel::new());
        let prompt = Prompt::text("Tell me a joke");

        // Test the builder pattern
        let result = GenerateText::new(model, prompt).execute().await;
        assert!(result.is_err());
        match result {
            Err(AISDKError::ModelError { .. }) => (), // Expected
            _ => panic!("Expected ModelError from mock do_generate"),
        }
    }

    #[tokio::test]
    async fn test_generate_text_with_settings() {
        let model = Arc::new(MockLanguageModel::new());
        let prompt =
            Prompt::text("What is the weather?").with_system("You are a helpful assistant");

        // This should validate settings and call do_generate
        // The mock returns an error, but that's expected
        let result = GenerateText::new(model, prompt)
            .temperature(0.7)
            .max_output_tokens(100)
            .execute()
            .await;
        assert!(result.is_err());
        match result {
            Err(AISDKError::ModelError { .. }) => (), // Expected
            _ => panic!("Expected ModelError from mock do_generate"),
        }
    }

    #[tokio::test]
    async fn test_generate_text_builder_with_settings() {
        let model = Arc::new(MockLanguageModel::new());
        let prompt =
            Prompt::text("What is the weather?").with_system("You are a helpful assistant");

        // Test the builder pattern with chained settings
        let result = GenerateText::new(model, prompt)
            .temperature(0.7)
            .max_output_tokens(100)
            .execute()
            .await;
        assert!(result.is_err());
        match result {
            Err(AISDKError::ModelError { .. }) => (), // Expected
            _ => panic!("Expected ModelError from mock do_generate"),
        }
    }

    #[tokio::test]
    async fn test_generate_text_with_tool_choice() {
        let model = Arc::new(MockLanguageModel::new());
        let prompt = Prompt::text("Use a tool to check the weather");

        // This should validate settings and call do_generate
        // The mock returns an error, but that's expected
        let result = GenerateText::new(model, prompt)
            .tool_choice(LanguageModelToolChoice::Auto)
            .execute()
            .await;
        assert!(result.is_err());
        match result {
            Err(AISDKError::ModelError { .. }) => (), // Expected
            _ => panic!("Expected ModelError from mock do_generate"),
        }
    }

    #[tokio::test]
    async fn test_generate_text_with_provider_options() {
        let model = Arc::new(MockLanguageModel::new());
        let prompt = Prompt::text("Hello, world!");
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
        let result = GenerateText::new(model, prompt)
            .provider_options(provider_options)
            .execute()
            .await;
        assert!(result.is_err());
        match result {
            Err(AISDKError::ModelError { .. }) => (), // Expected
            _ => panic!("Expected ModelError from mock do_generate"),
        }
    }

    #[tokio::test]
    async fn test_generate_text_with_invalid_temperature() {
        let model = Arc::new(MockLanguageModel::new());
        let prompt = Prompt::text("Hello");

        // This should fail validation
        let result = GenerateText::new(model, prompt)
            .temperature(f64::NAN)
            .execute()
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
        let model = Arc::new(MockLanguageModel::new());
        let prompt = Prompt::text("Hello");

        // This should fail validation
        let result = GenerateText::new(model, prompt)
            .max_output_tokens(0)
            .execute()
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
        let model = Arc::new(MockLanguageModel::new());
        let prompt = Prompt::messages(vec![]);

        // This should fail validation (empty messages)
        let result = GenerateText::new(model, prompt).execute().await;
        assert!(result.is_err());
        match result {
            Err(AISDKError::InvalidPrompt { message }) => {
                assert_eq!(message, "messages must not be empty");
            }
            _ => panic!("Expected InvalidPrompt error for empty messages"),
        }
    }

    #[test]
    fn test_as_output_converts_file() {
        use ai_sdk_provider::language_model::content::LanguageModelContent;
        use ai_sdk_provider::language_model::content::file::LanguageModelFile;

        // Create a provider File content
        let provider_file = LanguageModelFile::from_base64("text/plain", "SGVsbG8gV29ybGQh");
        let content = vec![LanguageModelContent::File(provider_file)];

        // Convert to Output
        let output = as_output(content, vec![], vec![]);

        // Verify the conversion
        assert_eq!(output.len(), 1);
        match &output[0] {
            Output::File(file) => {
                assert_eq!(file.media_type, "text/plain");
                assert_eq!(file.base64(), "SGVsbG8gV29ybGQh");
                assert_eq!(file.bytes(), b"Hello World!");
            }
            _ => panic!("Expected Output::File variant"),
        }
    }

    #[test]
    fn test_as_output_converts_file_from_bytes() {
        use ai_sdk_provider::language_model::content::LanguageModelContent;
        use ai_sdk_provider::language_model::content::file::LanguageModelFile;

        // Create a provider File content from bytes
        let provider_file =
            LanguageModelFile::from_binary("image/png", vec![0x89, 0x50, 0x4E, 0x47]);
        let content = vec![LanguageModelContent::File(provider_file)];

        // Convert to Output
        let output = as_output(content, vec![], vec![]);

        // Verify the conversion
        assert_eq!(output.len(), 1);
        match &output[0] {
            Output::File(file) => {
                assert_eq!(file.media_type, "image/png");
                assert_eq!(file.bytes(), &[0x89, 0x50, 0x4E, 0x47]);
            }
            _ => panic!("Expected Output::File variant"),
        }
    }
}
