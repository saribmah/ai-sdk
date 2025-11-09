use ai_sdk_provider::language_model::call_warning::LanguageModelCallWarning;
use ai_sdk_provider::language_model::content::source::LanguageModelSource;
use ai_sdk_provider::language_model::finish_reason::LanguageModelFinishReason;
use ai_sdk_provider::language_model::usage::LanguageModelUsage;
use ai_sdk_provider::shared::provider_metadata::SharedProviderMetadata;
use serde_json::Value;

use super::generated_file::GeneratedFile;
use super::response_message::ResponseMessage;
use super::step_result::{RequestMetadata, StepResponseMetadata, StepResult};
use crate::output::Output;
use crate::output::reasoning::ReasoningOutput;
use crate::tool::{ToolCall, ToolResult};

/// Metadata for the response, including messages and optional body.
#[derive(Debug, Clone, PartialEq)]
pub struct ResponseMetadata {
    /// The response messages that were generated during the call.
    ///
    /// It consists of an assistant message, potentially containing tool calls.
    /// When there are tool results, there is an additional tool message with
    /// the tool results that are available.
    pub messages: Vec<ResponseMessage>,

    /// Response body (available only for providers that use HTTP requests).
    pub body: Option<Value>,

    /// Response ID from the provider.
    pub id: Option<String>,

    /// Model ID that was used for the generation.
    pub model_id: Option<String>,

    /// Timestamp of the response.
    pub timestamp: Option<i64>,
}

/// The result of a `generate_text` call.
///
/// It contains the generated text, the tool calls that were made during the generation,
/// and the results of the tool calls.
///
/// # Type Parameters
///
/// * `INPUT` - The input type for tools
/// * `OUTPUT` - The output type for tools and structured output
///
/// # Example
///
/// ```no_run
/// use ai_sdk_core::generate_text::GenerateTextResult;
/// # use ai_sdk_core::GenerateText;
/// # use ai_sdk_core::prompt::Prompt;
/// # use std::sync::Arc;
/// # use ai_sdk_provider::LanguageModel;
/// # async fn example(model: Arc<dyn LanguageModel>) -> Result<(), Box<dyn std::error::Error>> {
///
/// let result = GenerateText::new(model, Prompt::text("Hello")).execute().await?;
/// println!("Generated text: {}", result.text);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct GenerateTextResult {
    /// The content that was generated in the last step.
    pub content: Vec<Output>,

    /// The text that was generated in the last step.
    pub text: String,

    /// The full reasoning that the model has generated in the last step.
    pub reasoning: Vec<ReasoningOutput>,

    /// The reasoning text that the model has generated in the last step.
    ///
    /// Can be None if the model has only generated text.
    pub reasoning_text: Option<String>,

    /// The files that were generated in the last step.
    ///
    /// Empty array if no files were generated.
    pub files: Vec<GeneratedFile>,

    /// Sources that have been used as references in the last step.
    pub sources: Vec<LanguageModelSource>,

    /// The tool calls that were made in the last step.
    pub tool_calls: Vec<ToolCall>,

    /// The results of the tool calls from the last step.
    pub tool_results: Vec<ToolResult>,

    /// The reason why the generation finished.
    pub finish_reason: LanguageModelFinishReason,

    /// The token usage of the last step.
    pub usage: LanguageModelUsage,

    /// The total token usage of all steps.
    ///
    /// When there are multiple steps, the usage is the sum of all step usages.
    pub total_usage: LanguageModelUsage,

    /// Warnings from the model provider (e.g. unsupported settings).
    pub warnings: Option<Vec<LanguageModelCallWarning>>,

    /// Additional request information.
    pub request: RequestMetadata,

    /// Additional response information.
    pub response: ResponseMetadata,

    /// Additional provider-specific metadata.
    ///
    /// They are passed through from the provider to the AI SDK and enable
    /// provider-specific results that can be fully encapsulated in the provider.
    pub provider_metadata: Option<SharedProviderMetadata>,

    /// Details for all steps.
    ///
    /// You can use this to get information about intermediate steps,
    /// such as the tool calls or the response headers.
    pub steps: Vec<StepResult>,
}

impl GenerateTextResult {
    /// Creates a new `GenerateTextResult` from steps.
    ///
    /// This is the primary constructor that computes most fields from the final step.
    ///
    /// # Arguments
    ///
    /// * `steps` - All generation steps
    /// * `total_usage` - Total token usage across all steps
    ///
    /// # Panics
    ///
    /// Panics if `steps` is empty.
    pub fn from_steps(steps: Vec<StepResult>, total_usage: LanguageModelUsage) -> Self {
        let final_step = steps.last().expect("steps cannot be empty");

        // Content is already Output, just clone it
        let content: Vec<Output> = final_step.content.clone();

        let text = final_step.text();
        let reasoning: Vec<ReasoningOutput> =
            final_step.reasoning().iter().cloned().cloned().collect();
        let reasoning_text = final_step.reasoning_text();

        // Files are extracted directly from Output (though currently empty)
        let files: Vec<GeneratedFile> = final_step.files().iter().cloned().cloned().collect();

        let sources: Vec<LanguageModelSource> =
            final_step.sources().iter().cloned().cloned().collect();

        // Tool calls and results are already the correct types in StepResult
        let tool_calls: Vec<ToolCall> = final_step.tool_calls().iter().cloned().cloned().collect();
        let tool_results: Vec<ToolResult> =
            final_step.tool_results().iter().cloned().cloned().collect();

        Self {
            content,
            text,
            reasoning,
            reasoning_text,
            files,
            sources,
            tool_calls,
            tool_results,
            finish_reason: final_step.finish_reason.clone(),
            usage: final_step.usage,
            total_usage,
            warnings: final_step.warnings.clone(),
            request: final_step.request.clone(),
            response: ResponseMetadata::from_step_metadata(vec![], final_step.response.clone()),
            provider_metadata: final_step.provider_metadata.clone(),
            steps,
        }
    }

    /// Creates a new `GenerateTextResult` with the given parameters.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        content: Vec<Output>,
        text: String,
        reasoning: Vec<ReasoningOutput>,
        reasoning_text: Option<String>,
        files: Vec<GeneratedFile>,
        sources: Vec<LanguageModelSource>,
        tool_calls: Vec<ToolCall>,
        tool_results: Vec<ToolResult>,
        finish_reason: LanguageModelFinishReason,
        usage: LanguageModelUsage,
        total_usage: LanguageModelUsage,
        warnings: Option<Vec<LanguageModelCallWarning>>,
        request: RequestMetadata,
        response: ResponseMetadata,
        provider_metadata: Option<SharedProviderMetadata>,
        steps: Vec<StepResult>,
    ) -> Self {
        Self {
            content,
            text,
            reasoning,
            reasoning_text,
            files,
            sources,
            tool_calls,
            tool_results,
            finish_reason,
            usage,
            total_usage,
            warnings,
            request,
            response,
            provider_metadata,
            steps,
        }
    }
}

impl ResponseMetadata {
    /// Creates a new `ResponseMetadata`.
    pub fn new(messages: Vec<ResponseMessage>) -> Self {
        Self {
            messages,
            body: None,
            id: None,
            model_id: None,
            timestamp: None,
        }
    }

    /// Creates a new `ResponseMetadata` from step metadata.
    pub fn from_step_metadata(
        messages: Vec<ResponseMessage>,
        metadata: StepResponseMetadata,
    ) -> Self {
        Self {
            messages,
            body: None,
            id: metadata.id,
            model_id: metadata.model_id,
            timestamp: metadata.timestamp,
        }
    }

    /// Sets the response body.
    pub fn with_body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// Sets the response ID.
    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Sets the model ID.
    pub fn with_model_id(mut self, model_id: String) -> Self {
        self.model_id = Some(model_id);
        self
    }

    /// Sets the timestamp.
    pub fn with_timestamp(mut self, timestamp: i64) -> Self {
        self.timestamp = Some(timestamp);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TextOutput;
    use serde_json::json;

    #[test]
    fn test_generated_file_new() {
        let file =
            GeneratedFile::from_base64("SGVsbG8gV29ybGQ=", "text/plain").with_name("test.txt");

        assert_eq!(file.name, Some("test.txt".to_string()));
        assert_eq!(file.base64(), "SGVsbG8gV29ybGQ=");
        assert_eq!(file.media_type, "text/plain");
    }

    #[test]
    fn test_response_metadata_new() {
        let messages = vec![];
        let metadata = ResponseMetadata::new(messages);

        assert!(metadata.messages.is_empty());
        assert!(metadata.body.is_none());
        assert!(metadata.id.is_none());
        assert!(metadata.model_id.is_none());
    }

    #[test]
    fn test_response_metadata_from_step_metadata() {
        let messages = vec![];
        let step_metadata = StepResponseMetadata {
            id: Some("test_id".to_string()),
            model_id: Some("test_model".to_string()),
            timestamp: None,
            body: None,
        };

        let metadata = ResponseMetadata::from_step_metadata(messages, step_metadata);

        assert!(metadata.messages.is_empty());
        assert!(metadata.body.is_none());
        assert_eq!(metadata.id, Some("test_id".to_string()));
        assert_eq!(metadata.model_id, Some("test_model".to_string()));
    }

    #[test]
    fn test_response_metadata_with_body() {
        let messages = vec![];
        let body = json!({"key": "value"});
        let metadata = ResponseMetadata::new(messages).with_body(body.clone());

        assert_eq!(metadata.body, Some(body));
    }

    #[test]
    fn test_response_metadata_builder_methods() {
        let metadata = ResponseMetadata::new(vec![])
            .with_id("resp_123".to_string())
            .with_model_id("gpt-4".to_string())
            .with_timestamp(1234567890);

        assert_eq!(metadata.id, Some("resp_123".to_string()));
        assert_eq!(metadata.model_id, Some("gpt-4".to_string()));
        assert_eq!(metadata.timestamp, Some(1234567890));
    }

    #[test]
    fn test_generate_text_result_new() {
        let result = GenerateTextResult::new(
            vec![],                          // content
            "Hello".to_string(),             // text
            vec![],                          // reasoning
            None,                            // reasoning_text
            vec![],                          // files
            vec![],                          // sources
            vec![],                          // tool_calls
            vec![],                          // tool_results
            LanguageModelFinishReason::Stop, // finish_reason
            LanguageModelUsage::default(),   // usage
            LanguageModelUsage::default(),   // total_usage
            None,                            // warnings
            RequestMetadata::default(),      // request
            ResponseMetadata::new(vec![]),   // response
            None,                            // provider_metadata
            vec![],                          // steps
        );

        assert_eq!(result.text, "Hello");
        assert_eq!(result.finish_reason, LanguageModelFinishReason::Stop);
    }

    #[test]
    fn test_generate_text_result_fields() {
        let result = GenerateTextResult::new(
            vec![],
            "Generated text".to_string(),
            vec![],
            Some("Reasoning text".to_string()),
            vec![],
            vec![],
            vec![],
            vec![],
            LanguageModelFinishReason::Length,
            LanguageModelUsage::default(),
            LanguageModelUsage::default(),
            None,
            RequestMetadata::default(),
            ResponseMetadata::new(vec![]),
            None,
            vec![],
        );

        assert_eq!(result.text, "Generated text");
        assert_eq!(result.reasoning_text, Some("Reasoning text".to_string()));
        assert_eq!(result.finish_reason, LanguageModelFinishReason::Length);
    }

    #[test]
    fn test_generate_text_result_from_steps_with_tool_calls() {
        use crate::tool::{ToolCall, ToolResult};

        // Create a step with tool calls and tool results using Output
        let content = vec![
            Output::Text(TextOutput::new("Hello".to_string())),
            Output::ToolCall(ToolCall::new(
                "call_1".to_string(),
                "get_weather".to_string(),
                json!({"city": "SF"}),
            )),
            Output::ToolResult(ToolResult::new(
                "call_1".to_string(),
                "get_weather".to_string(),
                json!({"city": "SF"}),
                json!({"temp": 72}),
            )),
        ];

        let step = StepResult::new(
            content,
            LanguageModelFinishReason::Stop,
            LanguageModelUsage::new(10, 20),
            None,
            RequestMetadata { body: None },
            StepResponseMetadata {
                id: Some("resp_123".to_string()),
                timestamp: None,
                model_id: Some("gpt-4".to_string()),
                body: None,
            },
            None,
        );

        let result = GenerateTextResult::from_steps(vec![step], LanguageModelUsage::new(10, 20));

        // Verify tool calls are populated
        assert_eq!(result.tool_calls.len(), 1);

        // Verify tool call details
        let call = &result.tool_calls[0];
        assert_eq!(call.tool_call_id, "call_1");
        assert_eq!(call.tool_name, "get_weather");
        assert_eq!(call.input, json!({"city": "SF"}));

        // Verify tool results are populated
        assert_eq!(result.tool_results.len(), 1);

        // Verify tool result details
        let res = &result.tool_results[0];
        assert_eq!(res.tool_call_id, "call_1");
        assert_eq!(res.tool_name, "get_weather");
        assert_eq!(res.output, json!({"temp": 72}));
    }
}
