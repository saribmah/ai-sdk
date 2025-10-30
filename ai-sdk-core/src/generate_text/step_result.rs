use ai_sdk_provider::language_model::{
    call_warning::CallWarning, finish_reason::FinishReason, response_metadata::ResponseMetadata,
    source::Source, usage::Usage,
};
use ai_sdk_provider::shared::provider_metadata::ProviderMetadata;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::content_part::ContentPart;
use super::generated_file::GeneratedFile;
use super::reasoning_output::ReasoningOutput;

/// Metadata about the request sent to the language model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestMetadata {
    /// The request body (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Value>,
}

/// Response metadata including messages generated during the call.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct StepResponseMetadata {
    /// Response ID (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Response timestamp (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,

    /// Model ID used for the response (if available).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,

    /// Response body (available only for providers that use HTTP requests).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Value>,
}

impl From<ResponseMetadata> for StepResponseMetadata {
    fn from(metadata: ResponseMetadata) -> Self {
        Self {
            id: metadata.id,
            timestamp: metadata.timestamp,
            model_id: metadata.model_id,
            body: None,
        }
    }
}

/// The result of a single step in the generation process.
///
/// This struct contains all the information about a generation step, including
/// the generated content, token usage, finish reason, and metadata.
///
/// # Type Parameters
///
/// * `INPUT` - The input type for tool calls/results
/// * `OUTPUT` - The output type for tool calls/results
///
/// # Example
///
/// ```ignore
/// use ai_sdk_core::StepResult;
///
/// let result = StepResult::new(
///     content,
///     finish_reason,
///     usage,
///     warnings,
///     request_metadata,
///     response_metadata,
///     provider_metadata,
/// );
///
/// // Access derived fields
/// let text = result.text();
/// let tool_calls = result.tool_calls();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct StepResult<INPUT = Value, OUTPUT = Value> {
    /// The content that was generated in the step (user-facing ContentPart types).
    pub content: Vec<ContentPart<INPUT, OUTPUT>>,

    /// The reason why the generation finished.
    pub finish_reason: FinishReason,

    /// The token usage of the generated text.
    pub usage: Usage,

    /// Warnings from the model provider (e.g. unsupported settings).
    pub warnings: Option<Vec<CallWarning>>,

    /// Additional request information.
    pub request: RequestMetadata,

    /// Additional response information.
    pub response: StepResponseMetadata,

    /// Additional provider-specific metadata.
    pub provider_metadata: Option<ProviderMetadata>,
}

impl<INPUT, OUTPUT> StepResult<INPUT, OUTPUT> {
    /// Creates a new StepResult.
    ///
    /// # Arguments
    ///
    /// * `content` - The generated content parts (user-facing ContentPart types)
    /// * `finish_reason` - Why the generation finished
    /// * `usage` - Token usage information
    /// * `warnings` - Optional warnings from the provider
    /// * `request` - Request metadata
    /// * `response` - Response metadata
    /// * `provider_metadata` - Optional provider-specific metadata
    pub fn new(
        content: Vec<ContentPart<INPUT, OUTPUT>>,
        finish_reason: FinishReason,
        usage: Usage,
        warnings: Option<Vec<CallWarning>>,
        request: RequestMetadata,
        response: StepResponseMetadata,
        provider_metadata: Option<ProviderMetadata>,
    ) -> Self {
        Self {
            content,
            finish_reason,
            usage,
            warnings,
            request,
            response,
            provider_metadata,
        }
    }

    /// Gets the generated text by joining all text content parts.
    ///
    /// # Returns
    ///
    /// A string containing all text parts concatenated together.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let text = result.text();
    /// println!("Generated text: {}", text);
    /// ```
    pub fn text(&self) -> String {
        self.content
            .iter()
            .filter_map(|part| {
                if let ContentPart::Text(text_output) = part {
                    Some(text_output.text.as_str())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join("")
    }

    /// Gets all reasoning parts from the content.
    ///
    /// # Returns
    ///
    /// A vector of references to reasoning parts.
    pub fn reasoning(&self) -> Vec<&ReasoningOutput> {
        self.content
            .iter()
            .filter_map(|part| {
                if let ContentPart::Reasoning(reasoning) = part {
                    Some(reasoning)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Gets the reasoning text by joining all reasoning content parts.
    ///
    /// # Returns
    ///
    /// `Some(String)` if there are reasoning parts, `None` otherwise.
    pub fn reasoning_text(&self) -> Option<String> {
        let reasoning_parts = self.reasoning();
        if reasoning_parts.is_empty() {
            None
        } else {
            Some(
                reasoning_parts
                    .iter()
                    .map(|r| r.text.as_str())
                    .collect::<Vec<_>>()
                    .join(""),
            )
        }
    }

    /// Gets all file parts from the content.
    ///
    /// # Returns
    ///
    /// A vector of references to files.
    pub fn files(&self) -> Vec<&GeneratedFile> {
        // ContentPart doesn't have File variant in the user-facing API
        // Files are extracted separately in GenerateTextResult
        vec![]
    }

    /// Gets all source parts from the content.
    ///
    /// # Returns
    ///
    /// A vector of references to sources.
    pub fn sources(&self) -> Vec<&Source> {
        self.content
            .iter()
            .filter_map(|part| {
                if let ContentPart::Source(source_output) = part {
                    Some(&source_output.source)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Gets all tool calls from the content.
    ///
    /// # Returns
    ///
    /// A vector of references to tool calls.
    pub fn tool_calls(&self) -> Vec<&super::tool_call::TypedToolCall<INPUT>> {
        self.content
            .iter()
            .filter_map(|part| {
                if let ContentPart::ToolCall(tool_call) = part {
                    Some(tool_call)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Gets all tool results from the content.
    ///
    /// # Returns
    ///
    /// A vector of references to tool results.
    pub fn tool_results(&self) -> Vec<&super::tool_result::TypedToolResult<INPUT, OUTPUT>> {
        self.content
            .iter()
            .filter_map(|part| {
                if let ContentPart::ToolResult(tool_result) = part {
                    Some(tool_result)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn create_test_step_result() -> StepResult {
        let content = vec![
            ContentPart::Text(super::super::text_output::TextOutput::new(
                "Hello ".to_string(),
            )),
            ContentPart::Text(super::super::text_output::TextOutput::new(
                "world!".to_string(),
            )),
            ContentPart::Reasoning(super::super::reasoning_output::ReasoningOutput::new(
                "Thinking step 1. ".to_string(),
            )),
            ContentPart::Reasoning(super::super::reasoning_output::ReasoningOutput::new(
                "Thinking step 2.".to_string(),
            )),
            ContentPart::ToolCall(super::super::tool_call::TypedToolCall::Dynamic(
                super::super::tool_call::DynamicToolCall::new(
                    "call_1".to_string(),
                    "get_weather".to_string(),
                    json!({"city": "SF"}),
                ),
            )),
        ];

        StepResult::new(
            content,
            FinishReason::Stop,
            Usage::new(10, 20),
            None,
            RequestMetadata { body: None },
            StepResponseMetadata {
                id: Some("resp_123".to_string()),
                timestamp: None,
                model_id: Some("gpt-4".to_string()),
                body: None,
            },
            None,
        )
    }

    #[test]
    fn test_step_result_new() {
        let result = create_test_step_result();

        assert_eq!(result.content.len(), 5);
        assert_eq!(result.finish_reason, FinishReason::Stop);
        assert_eq!(result.usage.input_tokens, 10);
        assert_eq!(result.usage.output_tokens, 20);
        assert_eq!(result.usage.total_tokens, 30);
    }

    #[test]
    fn test_step_result_text() {
        let result = create_test_step_result();
        assert_eq!(result.text(), "Hello world!");
    }

    #[test]
    fn test_step_result_reasoning() {
        let result = create_test_step_result();
        let reasoning = result.reasoning();
        assert_eq!(reasoning.len(), 2);
        assert_eq!(reasoning[0].text, "Thinking step 1. ");
        assert_eq!(reasoning[1].text, "Thinking step 2.");
    }

    #[test]
    fn test_step_result_reasoning_text() {
        let result = create_test_step_result();
        assert_eq!(
            result.reasoning_text(),
            Some("Thinking step 1. Thinking step 2.".to_string())
        );
    }

    #[test]
    fn test_step_result_reasoning_text_empty() {
        let result: StepResult<Value, Value> = StepResult::new(
            vec![ContentPart::Text(
                super::super::text_output::TextOutput::new("Hello".to_string()),
            )],
            FinishReason::Stop,
            Usage::new(5, 5),
            None,
            RequestMetadata { body: None },
            StepResponseMetadata {
                id: None,
                timestamp: None,
                model_id: None,
                body: None,
            },
            None,
        );

        assert_eq!(result.reasoning_text(), None);
    }

    #[test]
    fn test_step_result_tool_calls() {
        let result = create_test_step_result();
        let tool_calls = result.tool_calls();
        assert_eq!(tool_calls.len(), 1);
        match &tool_calls[0] {
            super::super::tool_call::TypedToolCall::Dynamic(call) => {
                assert_eq!(call.tool_name, "get_weather");
                assert_eq!(call.tool_call_id, "call_1");
            }
            _ => panic!("Expected dynamic tool call"),
        }
    }

    #[test]
    fn test_step_result_files() {
        let result = create_test_step_result();
        let files = result.files();
        assert_eq!(files.len(), 0);
    }

    #[test]
    fn test_step_result_sources() {
        let result = create_test_step_result();
        let sources = result.sources();
        assert_eq!(sources.len(), 0);
    }

    #[test]
    fn test_step_result_with_warnings() {
        let warnings = vec![CallWarning::unsupported_setting_with_details(
            "temperature",
            "Temperature not supported",
        )];

        let result: StepResult<Value, Value> = StepResult::new(
            vec![],
            FinishReason::Stop,
            Usage::new(0, 0),
            Some(warnings.clone()),
            RequestMetadata { body: None },
            StepResponseMetadata {
                id: None,
                timestamp: None,
                model_id: None,
                body: None,
            },
            None,
        );

        assert!(result.warnings.is_some());
        assert_eq!(result.warnings.as_ref().unwrap().len(), 1);
        match &result.warnings.as_ref().unwrap()[0] {
            CallWarning::UnsupportedSetting { setting, details } => {
                assert_eq!(setting, "temperature");
                assert_eq!(details.as_ref().unwrap(), "Temperature not supported");
            }
            _ => panic!("Expected UnsupportedSetting warning"),
        }
    }

    #[test]
    fn test_step_response_metadata_from_response_metadata() {
        let response_metadata = ResponseMetadata {
            id: Some("resp_456".to_string()),
            timestamp: Some(1234567890),
            model_id: Some("gpt-3.5".to_string()),
        };

        let step_response: StepResponseMetadata = response_metadata.into();
        assert_eq!(step_response.id, Some("resp_456".to_string()));
        assert_eq!(step_response.timestamp, Some(1234567890));
        assert_eq!(step_response.model_id, Some("gpt-3.5".to_string()));
        assert_eq!(step_response.body, None);
    }
}
