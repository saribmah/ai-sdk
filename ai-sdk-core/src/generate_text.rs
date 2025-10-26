mod retries;
mod prepare_tools;
mod tool_result;
mod step_result;
mod stop_condition;
mod prepare_step;
mod callbacks;
mod response_message;
mod parse_tool_call;

pub use retries::{prepare_retries, RetryConfig, RetryFunction};
pub use prepare_tools::{prepare_tools_and_tool_choice, ToolSet};
pub use tool_result::{DynamicToolResult, StaticToolResult, TypedToolResult};
pub use step_result::{RequestMetadata, StepResponseMetadata, StepResult};
pub use stop_condition::{
    has_tool_call, is_stop_condition_met, step_count_is, HasToolCall, StepCountIs, StopCondition,
};
pub use prepare_step::{PrepareStep, PrepareStepOptions, PrepareStepResult};
pub use callbacks::{FinishEvent, OnFinish, OnStepFinish};
pub use response_message::ResponseMessage;
pub use parse_tool_call::{
    parse_provider_executed_dynamic_tool_call, parse_tool_call, ParsedToolCall,
};

use crate::error::AISDKError;
use crate::prompt::{
    call_settings::{prepare_call_settings, CallSettings},
    convert_to_language_model_prompt::convert_to_language_model_prompt,
    standardize::validate_and_standardize,
    Prompt,
};
use ai_sdk_provider::{
    language_model::{call_options::CallOptions, LanguageModel},
    language_model::tool_choice::ToolChoice,
    shared::provider_options::ProviderOptions,
};

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
    tool_choice: Option<ToolChoice>,
    provider_options: Option<ProviderOptions>,
    stop_when: Option<Vec<Box<dyn StopCondition>>>,
    prepare_step: Option<Box<dyn PrepareStep>>,
    on_step_finish: Option<Box<dyn OnStepFinish>>,
    on_finish: Option<Box<dyn OnFinish>>,
) -> Result<ai_sdk_provider::language_model::LanguageModelGenerateResponse, AISDKError> {
    // Prepare stop conditions - default to step_count_is(1)
    let _stop_conditions = stop_when.unwrap_or_else(|| vec![Box::new(step_count_is(1))]);
    let _prepare_step = prepare_step;
    let _on_step_finish = on_step_finish;
    let _on_finish = on_finish;

    // Initialize response messages array for multi-step generation
    let _response_messages: Vec<ResponseMessage> = Vec::new();

    // Note: Multi-step generation logic with stop conditions, step preparation,
    // and callbacks will be implemented in a future update.
    // For now, the function performs a single generation step.

    // Step 1: Prepare retries
    let _retry_config = prepare_retries(settings.max_retries, settings.abort_signal.clone())?;

    // Step 2: Prepare and validate call settings
    let prepared_settings = prepare_call_settings(&settings)?;

    // Step 3: Validate and standardize the prompt
    let initial_prompt = validate_and_standardize(prompt)?;

    // Step 4: Convert to language model format (provider messages)
    let messages = convert_to_language_model_prompt(initial_prompt.clone())?;

    // Step 5: Prepare tools and tool choice
    let (provider_tools, prepared_tool_choice) = prepare_tools_and_tool_choice(tools.as_ref(), tool_choice);

    // Step 6: Build CallOptions
    let mut call_options = CallOptions::new(messages);

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
    if let Some(sequences) = prepared_settings.stop_sequences {
        call_options = call_options.with_stop_sequences(sequences);
    }
    if let Some(seed) = prepared_settings.seed {
        call_options = call_options.with_seed(seed);
    }

    // Add tools and tool choice
    if let Some(tools) = provider_tools {
        call_options = call_options.with_tools(tools);
    }
    if let Some(choice) = prepared_tool_choice {
        call_options = call_options.with_tool_choice(choice);
    }

    // Add headers and abort signal from settings
    if let Some(headers) = settings.headers {
        call_options = call_options.with_headers(headers);
    }
    if let Some(signal) = settings.abort_signal {
        call_options = call_options.with_abort_signal(signal);
    }

    // Add provider options
    if let Some(opts) = provider_options {
        call_options = call_options.with_provider_options(opts);
    }

    // Step 7: Call model.do_generate
    let response = model.do_generate(call_options).await
        .map_err(|e| AISDKError::model_error(e.to_string()))?;

    // Step 8: Parse tool calls from the response
    use ai_sdk_provider::language_model::content::Content;

    let _step_tool_calls: Vec<ParsedToolCall> = if let Some(tool_set) = tools.as_ref() {
        response
            .content
            .iter()
            .filter_map(|part| {
                if let Content::ToolCall(tool_call) = part {
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

    // Return the response
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_sdk_provider::language_model::{
        call_options::CallOptions, LanguageModelGenerateResponse, LanguageModelStreamResponse,
    };
    use async_trait::async_trait;
    use std::collections::HashMap;
    use regex::Regex;
    use serde_json::Value;

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
            _options: CallOptions,
        ) -> Result<LanguageModelGenerateResponse, Box<dyn std::error::Error>> {
            // Return a basic mock response
            // For now, we just return a mock error to indicate that this is a test mock
            Err("Mock implementation - tests should not actually call do_generate".into())
        }

        async fn do_stream(
            &self,
            _options: CallOptions,
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
        let result = generate_text(&model, prompt, settings, None, None, None, None, None, None, None).await;
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
        let result = generate_text(&model, prompt, settings, None, None, None, None, None, None, None).await;
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
        let tool_choice = Some(ToolChoice::Auto);

        // This should validate settings and call do_generate
        // The mock returns an error, but that's expected
        let result = generate_text(&model, prompt, settings, None, tool_choice, None, None, None, None, None).await;
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
        let mut provider_options = ProviderOptions::new();
        provider_options.insert(
            "custom".to_string(),
            [("key".to_string(), Value::String("value".to_string()))]
                .iter()
                .cloned()
                .collect(),
        );

        // This should validate settings and call do_generate
        // The mock returns an error, but that's expected
        let result = generate_text(&model, prompt, settings, None, None, Some(provider_options), None, None, None, None).await;
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
        let result = generate_text(&model, prompt, settings, None, None, None, None, None, None, None).await;
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
        let result = generate_text(&model, prompt, settings, None, None, None, None, None, None, None).await;
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
        let result = generate_text(&model, prompt, settings, None, None, None, None, None, None, None).await;
        assert!(result.is_err());
        match result {
            Err(AISDKError::InvalidPrompt { message }) => {
                assert_eq!(message, "messages must not be empty");
            }
            _ => panic!("Expected InvalidPrompt error for empty messages"),
        }
    }
}
