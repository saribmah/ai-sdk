use ai_sdk_provider::language_model::{
    call_warning::CallWarning,
    content::Content,
    file::File,
    finish_reason::FinishReason,
    reasoning::Reasoning,
    response_metadata::ResponseMetadata,
    source::Source,
    tool_call::ToolCall,
    tool_result::ToolResult as ProviderToolResult,
    usage::Usage,
};
use ai_sdk_provider::shared::provider_metadata::ProviderMetadata;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
pub struct StepResult {
    /// The content that was generated in the step.
    pub content: Vec<Content>,

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

impl StepResult {
    /// Creates a new StepResult.
    ///
    /// # Arguments
    ///
    /// * `content` - The generated content parts
    /// * `finish_reason` - Why the generation finished
    /// * `usage` - Token usage information
    /// * `warnings` - Optional warnings from the provider
    /// * `request` - Request metadata
    /// * `response` - Response metadata
    /// * `provider_metadata` - Optional provider-specific metadata
    pub fn new(
        content: Vec<Content>,
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
                if let Content::Text(text) = part {
                    Some(text.text.as_str())
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
    pub fn reasoning(&self) -> Vec<&Reasoning> {
        self.content
            .iter()
            .filter_map(|part| {
                if let Content::Reasoning(reasoning) = part {
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
    pub fn files(&self) -> Vec<&File> {
        self.content
            .iter()
            .filter_map(|part| {
                if let Content::File(file) = part {
                    Some(file)
                } else {
                    None
                }
            })
            .collect()
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
                if let Content::Source(source) = part {
                    Some(source)
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
    pub fn tool_calls(&self) -> Vec<&ToolCall> {
        self.content
            .iter()
            .filter_map(|part| {
                if let Content::ToolCall(tool_call) = part {
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
    pub fn tool_results(&self) -> Vec<&ProviderToolResult> {
        self.content
            .iter()
            .filter_map(|part| {
                if let Content::ToolResult(tool_result) = part {
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
    use ai_sdk_provider::language_model::{
        reasoning::Reasoning, text::Text, tool_call::ToolCall,
    };

    fn create_test_step_result() -> StepResult {
        let content = vec![
            Content::Text(Text::new("Hello ")),
            Content::Text(Text::new("world!")),
            Content::Reasoning(Reasoning::init("Thinking step 1. ")),
            Content::Reasoning(Reasoning::init("Thinking step 2.")),
            Content::ToolCall(ToolCall::new("call_1", "get_weather", r#"{"city":"SF"}"#)),
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
        let result = StepResult::new(
            vec![Content::Text(Text::new("Hello"))],
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
        assert_eq!(tool_calls[0].tool_name, "get_weather");
        assert_eq!(tool_calls[0].tool_call_id, "call_1");
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

        let result = StepResult::new(
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
