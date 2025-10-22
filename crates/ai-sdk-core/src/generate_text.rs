mod retries;

pub use retries::{prepare_retries, RetryConfig, RetryFunction};

use crate::error::AISDKError;
use crate::message::tool::definition::Tool;
use crate::prompt::{
    call_settings::{prepare_call_settings, CallSettings},
    convert_to_language_model_prompt::convert_to_language_model_prompt,
    standardize::validate_and_standardize,
    Prompt,
};
use ai_sdk_provider::{
    language_model::LanguageModel, language_model::tool_choice::ToolChoice,
    shared::provider_options::ProviderOptions,
};
use serde_json::Value;

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
/// * `tools` - Optional tools that the model can call. The model needs to support calling tools.
/// * `tool_choice` - Optional tool choice strategy. Default: 'auto'.
/// * `provider_options` - Optional provider-specific options.
///
/// # Returns
///
/// Returns `Result<(), AISDKError>` - Success or validation error
///
/// # Errors
///
/// Returns `AISDKError::InvalidArgument` if any settings are invalid (e.g., non-finite temperature).
///
/// # Examples
///
/// ```ignore
/// use ai_sdk_core::generate_text;
/// use ai_sdk_core::prompt::{Prompt, call_settings::CallSettings};
///
/// let prompt = Prompt::text("Tell me a joke");
/// let settings = CallSettings::default();
/// generate_text(model, prompt, settings, None, None, None)?;
/// ```
pub fn generate_text(
    model: &dyn LanguageModel,
    prompt: Prompt,
    settings: CallSettings,
    tools: Option<Vec<Tool<Value, Value>>>,
    tool_choice: Option<ToolChoice>,
    provider_options: Option<ProviderOptions>,
) -> Result<(), AISDKError> {
    // Step 1: Prepare and validate call settings
    let prepared_settings = prepare_call_settings(&settings)?;

    // Step 2: Validate and standardize the prompt
    let initial_prompt = validate_and_standardize(prompt)?;

    // Step 3: Convert to language model format (provider messages)
    let messages = convert_to_language_model_prompt(initial_prompt.clone())?;

    // TODO: Implement actual text generation logic
    // For now, just print the arguments

    println!("=== generate_text called ===");
    println!(
        "Model: provider={}, model_id={}",
        model.provider(),
        model.model_id()
    );
    println!("Initial Prompt (standardized):");
    println!("  - System: {:?}", initial_prompt.system);
    println!("  - Messages count: {}", initial_prompt.messages.len());
    println!("Converted Messages (provider format):");
    println!("  - Messages count: {}", messages.len());
    for (i, msg) in messages.iter().enumerate() {
        println!("  - Message {}: {:?}", i, msg);
    }
    println!("Settings: {:?}", settings);
    println!("Prepared Settings: {:?}", prepared_settings);
    println!(
        "Tools: {} tools provided",
        tools.as_ref().map(|t| t.len()).unwrap_or(0)
    );
    println!("Tool Choice: {:?}", tool_choice);
    println!(
        "Provider Options: {}",
        if provider_options.is_some() {
            "provided"
        } else {
            "none"
        }
    );
    println!("===========================");

    Ok(())
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
            unimplemented!("Mock implementation")
        }

        async fn do_stream(
            &self,
            _options: CallOptions,
        ) -> Result<LanguageModelStreamResponse, Box<dyn std::error::Error>> {
            unimplemented!("Mock implementation")
        }
    }

    #[test]
    fn test_generate_text_basic() {
        let model = MockLanguageModel::new();
        let prompt = Prompt::text("Tell me a joke");
        let settings = CallSettings::default();

        // This should just print the arguments and validate settings
        let result = generate_text(&model, prompt, settings, None, None, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_text_with_settings() {
        let model = MockLanguageModel::new();
        let prompt =
            Prompt::text("What is the weather?").with_system("You are a helpful assistant");
        let settings = CallSettings::default()
            .with_temperature(0.7)
            .with_max_output_tokens(100);

        // This should just print the arguments and validate settings
        let result = generate_text(&model, prompt, settings, None, None, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_text_with_tool_choice() {
        let model = MockLanguageModel::new();
        let prompt = Prompt::text("Use a tool to check the weather");
        let settings = CallSettings::default();
        let tool_choice = Some(ToolChoice::Auto);

        // This should just print the arguments and validate settings
        let result = generate_text(&model, prompt, settings, None, tool_choice, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_text_with_provider_options() {
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

        // This should just print the arguments and validate settings
        let result = generate_text(&model, prompt, settings, None, None, Some(provider_options));
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_text_with_invalid_temperature() {
        let model = MockLanguageModel::new();
        let prompt = Prompt::text("Hello");
        let settings = CallSettings::default().with_temperature(f64::NAN);

        // This should fail validation
        let result = generate_text(&model, prompt, settings, None, None, None);
        assert!(result.is_err());
        match result {
            Err(AISDKError::InvalidArgument { parameter, .. }) => {
                assert_eq!(parameter, "temperature");
            }
            _ => panic!("Expected InvalidArgument error for temperature"),
        }
    }

    #[test]
    fn test_generate_text_with_invalid_max_tokens() {
        let model = MockLanguageModel::new();
        let prompt = Prompt::text("Hello");
        let settings = CallSettings::default().with_max_output_tokens(0);

        // This should fail validation
        let result = generate_text(&model, prompt, settings, None, None, None);
        assert!(result.is_err());
        match result {
            Err(AISDKError::InvalidArgument { parameter, .. }) => {
                assert_eq!(parameter, "maxOutputTokens");
            }
            _ => panic!("Expected InvalidArgument error for maxOutputTokens"),
        }
    }

    #[test]
    fn test_generate_text_with_empty_messages() {
        let model = MockLanguageModel::new();
        let prompt = Prompt::messages(vec![]);
        let settings = CallSettings::default();

        // This should fail validation (empty messages)
        let result = generate_text(&model, prompt, settings, None, None, None);
        assert!(result.is_err());
        match result {
            Err(AISDKError::InvalidPrompt { message }) => {
                assert_eq!(message, "messages must not be empty");
            }
            _ => panic!("Expected InvalidPrompt error for empty messages"),
        }
    }
}
