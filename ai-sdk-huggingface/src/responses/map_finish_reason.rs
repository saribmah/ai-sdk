use ai_sdk_provider::language_model::finish_reason::LanguageModelFinishReason;

/// Maps a Hugging Face Responses API finish reason to the standard finish reason.
pub fn map_huggingface_responses_finish_reason(reason: Option<&str>) -> LanguageModelFinishReason {
    match reason {
        Some("stop") => LanguageModelFinishReason::Stop,
        Some("length") => LanguageModelFinishReason::Length,
        Some("content_filter") => LanguageModelFinishReason::ContentFilter,
        Some("tool_calls") => LanguageModelFinishReason::ToolCalls,
        Some("error") => LanguageModelFinishReason::Error,
        _ => LanguageModelFinishReason::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finish_reason_stop() {
        assert_eq!(
            map_huggingface_responses_finish_reason(Some("stop")),
            LanguageModelFinishReason::Stop
        );
    }

    #[test]
    fn test_finish_reason_length() {
        assert_eq!(
            map_huggingface_responses_finish_reason(Some("length")),
            LanguageModelFinishReason::Length
        );
    }

    #[test]
    fn test_finish_reason_content_filter() {
        assert_eq!(
            map_huggingface_responses_finish_reason(Some("content_filter")),
            LanguageModelFinishReason::ContentFilter
        );
    }

    #[test]
    fn test_finish_reason_tool_calls() {
        assert_eq!(
            map_huggingface_responses_finish_reason(Some("tool_calls")),
            LanguageModelFinishReason::ToolCalls
        );
    }

    #[test]
    fn test_finish_reason_error() {
        assert_eq!(
            map_huggingface_responses_finish_reason(Some("error")),
            LanguageModelFinishReason::Error
        );
    }

    #[test]
    fn test_finish_reason_unknown() {
        assert_eq!(
            map_huggingface_responses_finish_reason(Some("unknown_reason")),
            LanguageModelFinishReason::Unknown
        );
        assert_eq!(
            map_huggingface_responses_finish_reason(None),
            LanguageModelFinishReason::Unknown
        );
    }
}
