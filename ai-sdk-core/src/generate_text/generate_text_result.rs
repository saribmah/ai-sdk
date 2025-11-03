use ai_sdk_provider::language_model::call_warning::LanguageModelCallWarning;
use ai_sdk_provider::language_model::content::source::LanguageModelSource;
use ai_sdk_provider::language_model::finish_reason::LanguageModelFinishReason;
use ai_sdk_provider::language_model::usage::LanguageModelUsage;
use ai_sdk_provider::shared::provider_metadata::SharedProviderMetadata;
use serde_json::Value;

use super::content_part::ContentPart;
use super::generated_file::GeneratedFile;
use super::reasoning_output::ReasoningOutput;
use super::response_message::ResponseMessage;
use super::step_result::{RequestMetadata, StepResponseMetadata, StepResult};
use super::tool_call::{DynamicToolCall, StaticToolCall, TypedToolCall};
use super::tool_result::{DynamicToolResult, StaticToolResult, TypedToolResult};

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
/// ```ignore
/// use ai_sdk_core::generate_text::{generate_text, GenerateTextResult};
///
/// let result: GenerateTextResult<Value, Value> = generate_text(/* ... */).await?;
/// println!("Generated text: {}", result.text);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct GenerateTextResult<INPUT = Value, OUTPUT = Value> {
    /// The content that was generated in the last step.
    pub content: Vec<ContentPart<INPUT, OUTPUT>>,

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
    pub tool_calls: Vec<TypedToolCall<INPUT>>,

    /// The static tool calls that were made in the last step.
    pub static_tool_calls: Vec<StaticToolCall<INPUT>>,

    /// The dynamic tool calls that were made in the last step.
    pub dynamic_tool_calls: Vec<DynamicToolCall>,

    /// The results of the tool calls from the last step.
    pub tool_results: Vec<TypedToolResult<INPUT, OUTPUT>>,

    /// The static tool results that were made in the last step.
    pub static_tool_results: Vec<StaticToolResult<INPUT, OUTPUT>>,

    /// The dynamic tool results that were made in the last step.
    pub dynamic_tool_results: Vec<DynamicToolResult>,

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
    pub steps: Vec<StepResult<INPUT, OUTPUT>>,

    /// The generated structured output. It uses the `output` specification.
    pub output: OUTPUT,
}

impl<INPUT, OUTPUT> GenerateTextResult<INPUT, OUTPUT>
where
    INPUT: Clone,
    OUTPUT: Clone,
{
    /// Creates a new `GenerateTextResult` from steps and output.
    ///
    /// This is the primary constructor that follows the TypeScript `DefaultGenerateTextResult` pattern.
    /// It computes most fields from the final step.
    ///
    /// # Arguments
    ///
    /// * `steps` - All generation steps
    /// * `total_usage` - Total token usage across all steps
    /// * `output` - The resolved output value
    ///
    /// # Panics
    ///
    /// Panics if `steps` is empty.
    pub fn from_steps(
        steps: Vec<StepResult<INPUT, OUTPUT>>,
        total_usage: LanguageModelUsage,
        output: OUTPUT,
    ) -> Self
    where
        Self: Sized,
    {
        let final_step = steps.last().expect("steps cannot be empty");

        // Content is already ContentPart, just clone it
        let content: Vec<ContentPart<INPUT, OUTPUT>> = final_step.content.clone();

        let text = final_step.text();
        let reasoning: Vec<ReasoningOutput> =
            final_step.reasoning().iter().cloned().cloned().collect();
        let reasoning_text = final_step.reasoning_text();

        // Files are extracted directly from ContentPart (though currently empty)
        let files: Vec<GeneratedFile> = final_step.files().iter().cloned().cloned().collect();

        let sources: Vec<LanguageModelSource> =
            final_step.sources().iter().cloned().cloned().collect();

        // Tool calls are already TypedToolCall in StepResult
        let all_tool_calls: Vec<TypedToolCall<INPUT>> =
            final_step.tool_calls().iter().cloned().cloned().collect();

        // Split into static and dynamic
        let static_tool_calls: Vec<StaticToolCall<INPUT>> = all_tool_calls
            .iter()
            .filter_map(|tc| match tc {
                TypedToolCall::Static(call) => Some(call.clone()),
                _ => None,
            })
            .collect();

        let dynamic_tool_calls: Vec<DynamicToolCall> = all_tool_calls
            .iter()
            .filter_map(|tc| match tc {
                TypedToolCall::Dynamic(call) => Some(call.clone()),
                _ => None,
            })
            .collect();

        // Tool results are already TypedToolResult in StepResult
        let all_tool_results: Vec<TypedToolResult<INPUT, OUTPUT>> =
            final_step.tool_results().iter().cloned().cloned().collect();

        // Split into static and dynamic
        let static_tool_results: Vec<StaticToolResult<INPUT, OUTPUT>> = all_tool_results
            .iter()
            .filter_map(|tr| match tr {
                TypedToolResult::Static(result) => Some(result.clone()),
                _ => None,
            })
            .collect();

        let dynamic_tool_results: Vec<DynamicToolResult> = all_tool_results
            .iter()
            .filter_map(|tr| match tr {
                TypedToolResult::Dynamic(result) => Some(result.clone()),
                _ => None,
            })
            .collect();

        Self {
            content,
            text,
            reasoning,
            reasoning_text,
            files,
            sources,
            tool_calls: all_tool_calls,
            static_tool_calls,
            dynamic_tool_calls,
            tool_results: all_tool_results,
            static_tool_results,
            dynamic_tool_results,
            finish_reason: final_step.finish_reason.clone(),
            usage: final_step.usage.clone(),
            total_usage,
            warnings: final_step.warnings.clone(),
            request: final_step.request.clone(),
            response: ResponseMetadata::from_step_metadata(vec![], final_step.response.clone()),
            provider_metadata: final_step.provider_metadata.clone(),
            steps,
            output,
        }
    }

    /// Creates a new `GenerateTextResult` with the given parameters.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        content: Vec<ContentPart<INPUT, OUTPUT>>,
        text: String,
        reasoning: Vec<ReasoningOutput>,
        reasoning_text: Option<String>,
        files: Vec<GeneratedFile>,
        sources: Vec<LanguageModelSource>,
        tool_calls: Vec<TypedToolCall<INPUT>>,
        static_tool_calls: Vec<StaticToolCall<INPUT>>,
        dynamic_tool_calls: Vec<DynamicToolCall>,
        tool_results: Vec<TypedToolResult<INPUT, OUTPUT>>,
        static_tool_results: Vec<StaticToolResult<INPUT, OUTPUT>>,
        dynamic_tool_results: Vec<DynamicToolResult>,
        finish_reason: LanguageModelFinishReason,
        usage: LanguageModelUsage,
        total_usage: LanguageModelUsage,
        warnings: Option<Vec<LanguageModelCallWarning>>,
        request: RequestMetadata,
        response: ResponseMetadata,
        provider_metadata: Option<SharedProviderMetadata>,
        steps: Vec<StepResult<INPUT, OUTPUT>>,
        output: OUTPUT,
    ) -> Self
    where
        OUTPUT: Clone,
    {
        Self {
            content,
            text,
            reasoning,
            reasoning_text,
            files,
            sources,
            tool_calls,
            static_tool_calls,
            dynamic_tool_calls,
            tool_results,
            static_tool_results,
            dynamic_tool_results,
            finish_reason,
            usage,
            total_usage,
            warnings,
            request,
            response,
            provider_metadata,
            steps,
            output,
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
        let result: GenerateTextResult<Value, String> = GenerateTextResult::new(
            vec![],                          // content
            "Hello".to_string(),             // text
            vec![],                          // reasoning
            None,                            // reasoning_text
            vec![],                          // files
            vec![],                          // sources
            vec![],                          // tool_calls
            vec![],                          // static_tool_calls
            vec![],                          // dynamic_tool_calls
            vec![],                          // tool_results
            vec![],                          // static_tool_results
            vec![],                          // dynamic_tool_results
            LanguageModelFinishReason::Stop, // finish_reason
            LanguageModelUsage::default(),   // usage
            LanguageModelUsage::default(),   // total_usage
            None,                            // warnings
            RequestMetadata::default(),      // request
            ResponseMetadata::new(vec![]),   // response
            None,                            // provider_metadata
            vec![],                          // steps
            "output".to_string(),            // output
        );

        assert_eq!(result.text, "Hello");
        assert_eq!(result.finish_reason, LanguageModelFinishReason::Stop);
        assert_eq!(result.output, "output");
    }

    #[test]
    fn test_generate_text_result_fields() {
        let result: GenerateTextResult<Value, Value> = GenerateTextResult::new(
            vec![],
            "Generated text".to_string(),
            vec![],
            Some("Reasoning text".to_string()),
            vec![],
            vec![],
            vec![],
            vec![],
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
            json!({"key": "value"}),
        );

        assert_eq!(result.text, "Generated text");
        assert_eq!(result.reasoning_text, Some("Reasoning text".to_string()));
        assert_eq!(result.finish_reason, LanguageModelFinishReason::Length);
        assert_eq!(result.output, json!({"key": "value"}));
    }

    #[test]
    fn test_generate_text_result_from_steps_with_tool_calls() {
        use super::super::{
            DynamicToolCall, DynamicToolResult, TextOutput, TypedToolCall, TypedToolResult,
        };

        // Create a step with tool calls and tool results using ContentPart
        let content = vec![
            ContentPart::Text(TextOutput::new("Hello".to_string())),
            ContentPart::ToolCall(TypedToolCall::Dynamic(DynamicToolCall::new(
                "call_1".to_string(),
                "get_weather".to_string(),
                json!({"city": "SF"}),
            ))),
            ContentPart::ToolResult(TypedToolResult::Dynamic(
                DynamicToolResult::new(
                    "call_1".to_string(),
                    "get_weather".to_string(),
                    json!({"city": "SF"}),
                    json!({"temp": 72}),
                )
                .with_provider_executed(true),
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

        let result: GenerateTextResult<Value, Value> = GenerateTextResult::from_steps(
            vec![step],
            LanguageModelUsage::new(10, 20),
            json!(null),
        );

        // Verify tool calls are populated
        assert_eq!(result.tool_calls.len(), 1);
        assert_eq!(result.dynamic_tool_calls.len(), 1);
        assert_eq!(result.static_tool_calls.len(), 0);

        // Verify tool call details
        match &result.tool_calls[0] {
            TypedToolCall::Dynamic(call) => {
                assert_eq!(call.tool_call_id, "call_1");
                assert_eq!(call.tool_name, "get_weather");
                assert_eq!(call.input, json!({"city": "SF"}));
            }
            _ => panic!("Expected dynamic tool call"),
        }

        // Verify tool results are populated
        assert_eq!(result.tool_results.len(), 1);
        assert_eq!(result.dynamic_tool_results.len(), 1);
        assert_eq!(result.static_tool_results.len(), 0);

        // Verify tool result details
        match &result.tool_results[0] {
            TypedToolResult::Dynamic(res) => {
                assert_eq!(res.tool_call_id, "call_1");
                assert_eq!(res.tool_name, "get_weather");
                assert_eq!(res.input, json!({"city": "SF"}));
                assert_eq!(res.output, json!({"temp": 72}));
                assert_eq!(res.provider_executed, Some(true));
            }
            _ => panic!("Expected dynamic tool result"),
        }
    }
}
