use ai_sdk_provider::language_model::call_warning::CallWarning;
use ai_sdk_provider::language_model::finish_reason::FinishReason;
use ai_sdk_provider::language_model::source::Source;
use ai_sdk_provider::language_model::usage::Usage;
use ai_sdk_provider::shared::provider_metadata::ProviderMetadata;
use serde_json::Value;

use super::content_part::ContentPart;
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

/// A file that was generated during the generation process.
///
/// This represents a file with its content and metadata.
#[derive(Debug, Clone, PartialEq)]
pub struct GeneratedFile {
    /// The filename.
    pub name: String,

    /// The file content as base64-encoded data.
    pub data: String,

    /// The IANA media type of the file.
    pub media_type: String,
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
    pub sources: Vec<Source>,

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
    pub finish_reason: FinishReason,

    /// The token usage of the last step.
    pub usage: Usage,

    /// The total token usage of all steps.
    ///
    /// When there are multiple steps, the usage is the sum of all step usages.
    pub total_usage: Usage,

    /// Warnings from the model provider (e.g. unsupported settings).
    pub warnings: Option<Vec<CallWarning>>,

    /// Additional request information.
    pub request: RequestMetadata,

    /// Additional response information.
    pub response: ResponseMetadata,

    /// Additional provider-specific metadata.
    ///
    /// They are passed through from the provider to the AI SDK and enable
    /// provider-specific results that can be fully encapsulated in the provider.
    pub provider_metadata: Option<ProviderMetadata>,

    /// Details for all steps.
    ///
    /// You can use this to get information about intermediate steps,
    /// such as the tool calls or the response headers.
    pub steps: Vec<StepResult>,

    /// The generated structured output. It uses the `output` specification.
    ///
    /// **Deprecated**: Use `output` instead.
    #[deprecated(note = "Use `output` instead")]
    pub experimental_output: OUTPUT,

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
    pub fn from_steps(steps: Vec<StepResult>, total_usage: Usage, output: OUTPUT) -> Self
    where
        Self: Sized,
    {
        let final_step = steps.last().expect("steps cannot be empty");

        // Convert StepResult content to ContentPart
        let content: Vec<ContentPart<INPUT, OUTPUT>> = final_step
            .content
            .iter()
            .filter_map(|part| {
                use ai_sdk_provider::language_model::content::Content;
                match part {
                    Content::Text(text) => Some(ContentPart::Text(super::text_output::TextOutput::new(
                        text.text.clone(),
                    ))),
                    Content::Reasoning(reasoning) => Some(ContentPart::Reasoning(
                        super::reasoning_output::ReasoningOutput::new(reasoning.text.clone()),
                    )),
                    Content::Source(source) => Some(ContentPart::Source(
                        super::source_output::SourceOutput::new(source.clone()),
                    )),
                    _ => None,
                }
            })
            .collect();

        let text = final_step.text();
        let reasoning: Vec<ReasoningOutput> = final_step
            .reasoning()
            .iter()
            .map(|r| ReasoningOutput::new(r.text.clone()))
            .collect();
        let reasoning_text = final_step.reasoning_text();

        let files: Vec<GeneratedFile> = final_step
            .files()
            .iter()
            .map(|f| {
                use ai_sdk_provider::language_model::file::FileData;

                // Convert FileData to String (base64)
                let data_str = match &f.data {
                    FileData::Base64(s) => s.clone(),
                    FileData::Binary(bytes) => {
                        use base64::{Engine as _, engine::general_purpose};
                        general_purpose::STANDARD.encode(bytes)
                    }
                };

                GeneratedFile::new(
                    String::new(), // File doesn't have a name field
                    data_str,
                    f.media_type.clone(),
                )
            })
            .collect();

        let sources: Vec<Source> = final_step
            .sources()
            .iter()
            .map(|s| (*s).clone())
            .collect();

        Self {
            content,
            text,
            reasoning,
            reasoning_text,
            files,
            sources,
            tool_calls: vec![],
            static_tool_calls: vec![],
            dynamic_tool_calls: vec![],
            tool_results: vec![],
            static_tool_results: vec![],
            dynamic_tool_results: vec![],
            finish_reason: final_step.finish_reason.clone(),
            usage: final_step.usage.clone(),
            total_usage,
            warnings: final_step.warnings.clone(),
            request: final_step.request.clone(),
            response: ResponseMetadata::from_step_metadata(vec![], final_step.response.clone()),
            provider_metadata: final_step.provider_metadata.clone(),
            steps,
            experimental_output: output.clone(),
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
        sources: Vec<Source>,
        tool_calls: Vec<TypedToolCall<INPUT>>,
        static_tool_calls: Vec<StaticToolCall<INPUT>>,
        dynamic_tool_calls: Vec<DynamicToolCall>,
        tool_results: Vec<TypedToolResult<INPUT, OUTPUT>>,
        static_tool_results: Vec<StaticToolResult<INPUT, OUTPUT>>,
        dynamic_tool_results: Vec<DynamicToolResult>,
        finish_reason: FinishReason,
        usage: Usage,
        total_usage: Usage,
        warnings: Option<Vec<CallWarning>>,
        request: RequestMetadata,
        response: ResponseMetadata,
        provider_metadata: Option<ProviderMetadata>,
        steps: Vec<StepResult>,
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
            experimental_output: output.clone(),
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

impl GeneratedFile {
    /// Creates a new `GeneratedFile`.
    pub fn new(name: String, data: String, media_type: String) -> Self {
        Self {
            name,
            data,
            media_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_generated_file_new() {
        let file = GeneratedFile::new(
            "test.txt".to_string(),
            "SGVsbG8gV29ybGQ=".to_string(),
            "text/plain".to_string(),
        );

        assert_eq!(file.name, "test.txt");
        assert_eq!(file.data, "SGVsbG8gV29ybGQ=");
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
            vec![],                      // content
            "Hello".to_string(),         // text
            vec![],                      // reasoning
            None,                        // reasoning_text
            vec![],                      // files
            vec![],                      // sources
            vec![],                      // tool_calls
            vec![],                      // static_tool_calls
            vec![],                      // dynamic_tool_calls
            vec![],                      // tool_results
            vec![],                      // static_tool_results
            vec![],                      // dynamic_tool_results
            FinishReason::Stop,          // finish_reason
            Usage::default(),            // usage
            Usage::default(),            // total_usage
            None,                        // warnings
            RequestMetadata::default(),  // request
            ResponseMetadata::new(vec![]), // response
            None,                        // provider_metadata
            vec![],                      // steps
            "output".to_string(),        // output
        );

        assert_eq!(result.text, "Hello");
        assert_eq!(result.finish_reason, FinishReason::Stop);
        assert_eq!(result.output, "output");
        #[allow(deprecated)]
        {
            assert_eq!(result.experimental_output, "output");
        }
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
            FinishReason::Length,
            Usage::default(),
            Usage::default(),
            None,
            RequestMetadata::default(),
            ResponseMetadata::new(vec![]),
            None,
            vec![],
            json!({"key": "value"}),
        );

        assert_eq!(result.text, "Generated text");
        assert_eq!(result.reasoning_text, Some("Reasoning text".to_string()));
        assert_eq!(result.finish_reason, FinishReason::Length);
        assert_eq!(result.output, json!({"key": "value"}));
    }
}
