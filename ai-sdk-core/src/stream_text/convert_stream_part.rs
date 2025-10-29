/// Utilities for converting provider StreamPart to TextStreamPart.
///
/// This module provides functions to convert from the provider-level `StreamPart` type
/// to the AI SDK core `TextStreamPart` type.

use crate::stream_text::TextStreamPart;
use ai_sdk_provider::language_model::stream_part::StreamPart;
use serde_json::Value;

/// Converts a provider `StreamPart` to a core `TextStreamPart`.
///
/// This function handles the conversion from the provider-level stream part format
/// to the AI SDK core format. It maps field names (e.g., `delta` -> `text`) and
/// ensures compatibility between the two type systems.
///
/// # Type Parameters
///
/// * `INPUT` - The input type for tools (defaults to `Value`)
/// * `OUTPUT` - The output type from tools (defaults to `Value`)
///
/// # Arguments
///
/// * `stream_part` - The provider stream part to convert
///
/// # Returns
///
/// A `TextStreamPart` corresponding to the provider stream part.
///
/// # Example
///
/// ```ignore
/// use ai_sdk_provider::language_model::stream_part::StreamPart;
/// use ai_sdk_core::stream_text::convert_stream_part::convert_stream_part_to_text_stream_part;
///
/// let provider_part = StreamPart::TextDelta {
///     id: "text1".to_string(),
///     delta: "Hello".to_string(),
///     provider_metadata: None,
/// };
///
/// let text_part = convert_stream_part_to_text_stream_part(provider_part);
/// ```
pub fn convert_stream_part_to_text_stream_part<INPUT, OUTPUT>(
    stream_part: StreamPart,
) -> TextStreamPart<INPUT, OUTPUT>
where
    INPUT: Clone + serde::de::DeserializeOwned,
    OUTPUT: Clone + serde::de::DeserializeOwned,
{
    match stream_part {
        // Text blocks
        StreamPart::TextStart {
            id,
            provider_metadata,
        } => TextStreamPart::TextStart {
            id,
            provider_metadata,
        },

        StreamPart::TextDelta {
            id,
            delta,
            provider_metadata,
        } => TextStreamPart::TextDelta {
            id,
            provider_metadata,
            text: delta, // Note: field name changes from 'delta' to 'text'
        },

        StreamPart::TextEnd {
            id,
            provider_metadata,
        } => TextStreamPart::TextEnd {
            id,
            provider_metadata,
        },

        // Reasoning blocks
        StreamPart::ReasoningStart {
            id,
            provider_metadata,
        } => TextStreamPart::ReasoningStart {
            id,
            provider_metadata,
        },

        StreamPart::ReasoningDelta {
            id,
            delta,
            provider_metadata,
        } => TextStreamPart::ReasoningDelta {
            id,
            provider_metadata,
            text: delta, // Note: field name changes from 'delta' to 'text'
        },

        StreamPart::ReasoningEnd {
            id,
            provider_metadata,
        } => TextStreamPart::ReasoningEnd {
            id,
            provider_metadata,
        },

        // Tool input streaming
        StreamPart::ToolInputStart {
            id,
            tool_name,
            provider_metadata,
            provider_executed,
        } => TextStreamPart::ToolInputStart {
            id,
            tool_name,
            provider_metadata,
            provider_executed,
            dynamic: None,
            title: None,
        },

        StreamPart::ToolInputDelta {
            id,
            delta,
            provider_metadata,
        } => TextStreamPart::ToolInputDelta {
            id,
            delta,
            provider_metadata,
        },

        StreamPart::ToolInputEnd {
            id,
            provider_metadata,
        } => TextStreamPart::ToolInputEnd {
            id,
            provider_metadata,
        },

        // Complete tool call and result (pass through)
        StreamPart::ToolCall(tool_call) => {
            // Convert provider ToolCall to our typed tool call
            // Parse the input JSON string
            let parsed_input: INPUT = serde_json::from_str(&tool_call.input)
                .unwrap_or_else(|_| serde_json::from_value(Value::Null).unwrap());

            TextStreamPart::ToolCall {
                tool_call: crate::generate_text::TypedToolCall::Static(
                    crate::generate_text::StaticToolCall::new(
                        tool_call.tool_call_id,
                        tool_call.tool_name,
                        parsed_input,
                    )
                ),
            }
        }

        StreamPart::ToolResult(tool_result) => {
            // Convert provider ToolResult to our typed tool result
            // Parse the result value as OUTPUT
            let parsed_result: OUTPUT = serde_json::from_value(tool_result.result.clone())
                .unwrap_or_else(|_| serde_json::from_value(Value::Null).unwrap());

            // Input is not available in ToolResult, use Null
            let null_input: INPUT = serde_json::from_value(Value::Null).unwrap();

            TextStreamPart::ToolResult {
                tool_result: crate::generate_text::TypedToolResult::Static(
                    crate::generate_text::StaticToolResult::new(
                        tool_result.tool_call_id,
                        tool_result.tool_name,
                        null_input,
                        parsed_result,
                    )
                ),
            }
        }

        // Files and sources
        StreamPart::File(file) => {
            // Convert FileData to base64 string
            use ai_sdk_provider::language_model::file::FileData;
            let base64_content = match file.data {
                FileData::Base64(ref s) => s.clone(),
                FileData::Binary(ref bytes) => {
                    use base64::Engine;
                    base64::engine::general_purpose::STANDARD.encode(bytes)
                }
            };

            TextStreamPart::File {
                file: crate::stream_text::StreamGeneratedFile {
                    base64: base64_content,
                    media_type: file.media_type,
                    name: None, // File doesn't have a name field
                },
            }
        }

        StreamPart::Source(source) => {
            // Convert Source to SourceOutput
            TextStreamPart::Source {
                source: crate::generate_text::SourceOutput {
                    source_type: "source".to_string(),
                    source,
                    provider_metadata: None,
                },
            }
        }

        // Stream metadata
        StreamPart::StreamStart { .. } => {
            // TextStreamPart::Start has no fields
            TextStreamPart::Start
        }

        StreamPart::ResponseMetadata(metadata) => {
            // For Phase 2, we'll skip response metadata
            // This will be handled in later phases
            // For now, return a raw chunk
            TextStreamPart::Raw {
                raw_value: serde_json::to_value(metadata).unwrap_or(Value::Null),
            }
        }

        // Finish
        StreamPart::Finish {
            finish_reason,
            usage,
            ..
        } => TextStreamPart::Finish {
            finish_reason,
            total_usage: usage,
        },

        // Error
        StreamPart::Error { error } => TextStreamPart::Error { error },

        // Raw (for debugging or custom handling)
        StreamPart::Raw { raw_value } => TextStreamPart::Raw { raw_value },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_sdk_provider::language_model::finish_reason::FinishReason;
    use ai_sdk_provider::language_model::usage::Usage;

    #[test]
    fn test_convert_text_start() {
        let provider_part = StreamPart::TextStart {
            id: "text1".to_string(),
            provider_metadata: None,
        };

        let text_part: TextStreamPart<Value, Value> =
            convert_stream_part_to_text_stream_part(provider_part);

        match text_part {
            TextStreamPart::TextStart { id, .. } => {
                assert_eq!(id, "text1");
            }
            _ => panic!("Expected TextStart"),
        }
    }

    #[test]
    fn test_convert_text_delta() {
        let provider_part = StreamPart::TextDelta {
            id: "text1".to_string(),
            delta: "Hello".to_string(),
            provider_metadata: None,
        };

        let text_part: TextStreamPart<Value, Value> =
            convert_stream_part_to_text_stream_part(provider_part);

        match text_part {
            TextStreamPart::TextDelta { id, text, .. } => {
                assert_eq!(id, "text1");
                assert_eq!(text, "Hello");
            }
            _ => panic!("Expected TextDelta"),
        }
    }

    #[test]
    fn test_convert_reasoning_delta() {
        let provider_part = StreamPart::ReasoningDelta {
            id: "reasoning1".to_string(),
            delta: "Thinking...".to_string(),
            provider_metadata: None,
        };

        let text_part: TextStreamPart<Value, Value> =
            convert_stream_part_to_text_stream_part(provider_part);

        match text_part {
            TextStreamPart::ReasoningDelta { id, text, .. } => {
                assert_eq!(id, "reasoning1");
                assert_eq!(text, "Thinking...");
            }
            _ => panic!("Expected ReasoningDelta"),
        }
    }

    #[test]
    fn test_convert_finish() {
        let provider_part = StreamPart::Finish {
            finish_reason: FinishReason::Stop,
            usage: Usage::new(10, 20),
            provider_metadata: None,
        };

        let text_part: TextStreamPart<Value, Value> =
            convert_stream_part_to_text_stream_part(provider_part);

        match text_part {
            TextStreamPart::Finish {
                finish_reason,
                total_usage,
                ..
            } => {
                assert_eq!(finish_reason, FinishReason::Stop);
                assert_eq!(total_usage.input_tokens, 10);
                assert_eq!(total_usage.output_tokens, 20);
            }
            _ => panic!("Expected Finish"),
        }
    }

    #[test]
    fn test_convert_error() {
        let provider_part = StreamPart::Error {
            error: serde_json::json!({"message": "Test error"}),
        };

        let text_part: TextStreamPart<Value, Value> =
            convert_stream_part_to_text_stream_part(provider_part);

        match text_part {
            TextStreamPart::Error { error } => {
                assert_eq!(error, serde_json::json!({"message": "Test error"}));
            }
            _ => panic!("Expected Error"),
        }
    }
}
