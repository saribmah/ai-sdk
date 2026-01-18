use llm_kit_provider::language_model::finish_reason::LanguageModelFinishReason;

/// Maps OpenAI-compatible finish reasons to the standard FinishReason enum.
///
/// # Arguments
///
/// * `finish_reason` - The finish reason string from the OpenAI-compatible API response
///
/// # Returns
///
/// The corresponding `FinishReason` enum variant
///
/// # Mapping
///
/// - `"stop"` → `FinishReason::Stop`
/// - `"length"` → `FinishReason::Length`
/// - `"content_filter"` → `FinishReason::ContentFilter`
/// - `"function_call"` → `FinishReason::ToolCalls`
/// - `"tool_calls"` → `FinishReason::ToolCalls`
/// - `None` or unknown → `FinishReason::Unknown`
pub fn map_openai_compatible_finish_reason(
    finish_reason: Option<&str>,
) -> LanguageModelFinishReason {
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
        let result = map_openai_compatible_finish_reason(Some("stop"));
        assert!(matches!(result, LanguageModelFinishReason::Stop));
    }

    #[test]
    fn test_map_length() {
        let result = map_openai_compatible_finish_reason(Some("length"));
        assert!(matches!(result, LanguageModelFinishReason::Length));
    }

    #[test]
    fn test_map_content_filter() {
        let result = map_openai_compatible_finish_reason(Some("content_filter"));
        assert!(matches!(result, LanguageModelFinishReason::ContentFilter));
    }

    #[test]
    fn test_map_function_call() {
        let result = map_openai_compatible_finish_reason(Some("function_call"));
        assert!(matches!(result, LanguageModelFinishReason::ToolCalls));
    }

    #[test]
    fn test_map_tool_calls() {
        let result = map_openai_compatible_finish_reason(Some("tool_calls"));
        assert!(matches!(result, LanguageModelFinishReason::ToolCalls));
    }

    #[test]
    fn test_map_none() {
        let result = map_openai_compatible_finish_reason(None);
        assert!(matches!(result, LanguageModelFinishReason::Unknown));
    }

    #[test]
    fn test_map_unknown_string() {
        let result = map_openai_compatible_finish_reason(Some("unknown_reason"));
        assert!(matches!(result, LanguageModelFinishReason::Unknown));
    }

    #[test]
    fn test_map_empty_string() {
        let result = map_openai_compatible_finish_reason(Some(""));
        assert!(matches!(result, LanguageModelFinishReason::Unknown));
    }
}
