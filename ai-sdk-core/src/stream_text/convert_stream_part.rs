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
    INPUT: Clone,
    OUTPUT: Clone,
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

        // Tool calls
        StreamPart::ToolCallStart {
            id,
            tool_call_id,
            tool_name,
        } => TextStreamPart::ToolInputStart {
            id,
            tool_name,
            provider_metadata: None,
            provider_executed: None,
            dynamic: None,
            title: None,
        },

        StreamPart::ToolCallDelta { id, delta } => TextStreamPart::ToolInputDelta {
            id,
            delta,
            provider_metadata: None,
        },

        StreamPart::ToolCallEnd { id } => TextStreamPart::ToolInputEnd {
            id,
            provider_metadata: None,
        },

        // Finish
        StreamPart::Finish {
            finish_reason,
            usage,
            provider_metadata,
            ..
        } => TextStreamPart::Finish {
            finish_reason,
            total_usage: usage,
            provider_metadata,
            provider_response_metadata: None,
        },

        // Error
        StreamPart::Error { error } => TextStreamPart::Error {
            error: Value::String(error),
        },

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
            usage: Usage {
                prompt_tokens: 10,
                completion_tokens: 20,
                total_tokens: 30,
            },
            provider_metadata: None,
            log_probs: None,
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
                assert_eq!(total_usage.total_tokens, 30);
            }
            _ => panic!("Expected Finish"),
        }
    }

    #[test]
    fn test_convert_error() {
        let provider_part = StreamPart::Error {
            error: "Test error".to_string(),
        };

        let text_part: TextStreamPart<Value, Value> =
            convert_stream_part_to_text_stream_part(provider_part);

        match text_part {
            TextStreamPart::Error { error } => {
                assert_eq!(error, Value::String("Test error".to_string()));
            }
            _ => panic!("Expected Error"),
        }
    }
}
