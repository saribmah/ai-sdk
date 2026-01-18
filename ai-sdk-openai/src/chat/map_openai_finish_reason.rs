//! OpenAI Finish Reason Mapping
//!
//! Maps OpenAI finish reasons to SDK finish reason types.

use llm_kit_provider::language_model::finish_reason::LanguageModelFinishReason;

/// Map OpenAI finish reason to SDK finish reason.
///
/// # Arguments
///
/// * `finish_reason` - The finish reason string from OpenAI API
///
/// # Returns
///
/// The corresponding SDK finish reason
pub fn map_openai_finish_reason(finish_reason: Option<&str>) -> LanguageModelFinishReason {
    match finish_reason {
        Some("stop") => LanguageModelFinishReason::Stop,
        Some("length") => LanguageModelFinishReason::Length,
        Some("content_filter") => LanguageModelFinishReason::ContentFilter,
        Some("function_call") | Some("tool_calls") => LanguageModelFinishReason::ToolCalls,
        _ => LanguageModelFinishReason::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_stop() {
        assert_eq!(
            map_openai_finish_reason(Some("stop")),
            LanguageModelFinishReason::Stop
        );
    }

    #[test]
    fn test_map_length() {
        assert_eq!(
            map_openai_finish_reason(Some("length")),
            LanguageModelFinishReason::Length
        );
    }

    #[test]
    fn test_map_content_filter() {
        assert_eq!(
            map_openai_finish_reason(Some("content_filter")),
            LanguageModelFinishReason::ContentFilter
        );
    }

    #[test]
    fn test_map_tool_calls() {
        assert_eq!(
            map_openai_finish_reason(Some("tool_calls")),
            LanguageModelFinishReason::ToolCalls
        );
        assert_eq!(
            map_openai_finish_reason(Some("function_call")),
            LanguageModelFinishReason::ToolCalls
        );
    }

    #[test]
    fn test_map_unknown() {
        assert_eq!(
            map_openai_finish_reason(None),
            LanguageModelFinishReason::Unknown
        );
        assert_eq!(
            map_openai_finish_reason(Some("unknown_reason")),
            LanguageModelFinishReason::Unknown
        );
    }
}
