use ai_sdk_provider::language_model::finish_reason::LanguageModelFinishReason;

/// Maps Anthropic API stop/finish reasons to the SDK's standard finish reason enum.
///
/// Anthropic's API returns different stop reason strings that need to be normalized
/// to the SDK's standard `LanguageModelFinishReason` enum.
///
/// # Arguments
///
/// * `finish_reason` - The finish reason string from the Anthropic API response
/// * `is_json_response_from_tool` - Whether the response is a JSON response from a tool
///
/// # Returns
///
/// A normalized `LanguageModelFinishReason` enum value
///
/// # Anthropic Stop Reasons
///
/// - `end_turn`: The model reached a natural stopping point
/// - `pause_turn`: The model paused (extended thinking mode)
/// - `max_tokens`: Maximum token limit reached
/// - `stop_sequence`: A stop sequence was encountered
/// - `tool_use`: The model wants to use a tool
/// - `refusal`: The model refused to respond (content filter)
///
/// See: https://docs.anthropic.com/en/api/messages#response-stop-reason
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::map_stop_reason::map_anthropic_stop_reason;
/// use ai_sdk_provider::language_model::finish_reason::LanguageModelFinishReason;
///
/// let reason = map_anthropic_stop_reason(Some("end_turn"), false);
/// assert_eq!(reason, LanguageModelFinishReason::Stop);
///
/// let reason = map_anthropic_stop_reason(Some("tool_use"), false);
/// assert_eq!(reason, LanguageModelFinishReason::ToolCalls);
///
/// let reason = map_anthropic_stop_reason(Some("max_tokens"), false);
/// assert_eq!(reason, LanguageModelFinishReason::Length);
/// ```
pub fn map_anthropic_stop_reason(
    finish_reason: Option<&str>,
    is_json_response_from_tool: bool,
) -> LanguageModelFinishReason {
    match finish_reason {
        // Natural stopping points
        Some("pause_turn") | Some("end_turn") | Some("stop_sequence") => {
            LanguageModelFinishReason::Stop
        }

        // Content filtering/refusal
        Some("refusal") => LanguageModelFinishReason::ContentFilter,

        // Tool usage - special handling for JSON responses
        // When the tool returns JSON, it's treated as a normal stop
        Some("tool_use") => {
            if is_json_response_from_tool {
                LanguageModelFinishReason::Stop
            } else {
                LanguageModelFinishReason::ToolCalls
            }
        }

        // Length limit reached
        Some("max_tokens") => LanguageModelFinishReason::Length,

        // Unknown or null finish reason
        _ => LanguageModelFinishReason::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_end_turn() {
        let reason = map_anthropic_stop_reason(Some("end_turn"), false);
        assert_eq!(reason, LanguageModelFinishReason::Stop);
    }

    #[test]
    fn test_pause_turn() {
        let reason = map_anthropic_stop_reason(Some("pause_turn"), false);
        assert_eq!(reason, LanguageModelFinishReason::Stop);
    }

    #[test]
    fn test_stop_sequence() {
        let reason = map_anthropic_stop_reason(Some("stop_sequence"), false);
        assert_eq!(reason, LanguageModelFinishReason::Stop);
    }

    #[test]
    fn test_refusal() {
        let reason = map_anthropic_stop_reason(Some("refusal"), false);
        assert_eq!(reason, LanguageModelFinishReason::ContentFilter);
    }

    #[test]
    fn test_tool_use_without_json() {
        let reason = map_anthropic_stop_reason(Some("tool_use"), false);
        assert_eq!(reason, LanguageModelFinishReason::ToolCalls);
    }

    #[test]
    fn test_tool_use_with_json() {
        let reason = map_anthropic_stop_reason(Some("tool_use"), true);
        assert_eq!(reason, LanguageModelFinishReason::Stop);
    }

    #[test]
    fn test_max_tokens() {
        let reason = map_anthropic_stop_reason(Some("max_tokens"), false);
        assert_eq!(reason, LanguageModelFinishReason::Length);
    }

    #[test]
    fn test_unknown_reason() {
        let reason = map_anthropic_stop_reason(Some("unknown_reason"), false);
        assert_eq!(reason, LanguageModelFinishReason::Unknown);
    }

    #[test]
    fn test_none_reason() {
        let reason = map_anthropic_stop_reason(None, false);
        assert_eq!(reason, LanguageModelFinishReason::Unknown);
    }

    #[test]
    fn test_empty_string() {
        let reason = map_anthropic_stop_reason(Some(""), false);
        assert_eq!(reason, LanguageModelFinishReason::Unknown);
    }

    #[test]
    fn test_case_sensitivity() {
        // The function should be case-sensitive (as per Anthropic API)
        let reason = map_anthropic_stop_reason(Some("END_TURN"), false);
        assert_eq!(reason, LanguageModelFinishReason::Unknown);

        let reason = map_anthropic_stop_reason(Some("End_Turn"), false);
        assert_eq!(reason, LanguageModelFinishReason::Unknown);
    }

    #[test]
    fn test_all_stop_variants_without_json() {
        let stop_variants = vec!["pause_turn", "end_turn", "stop_sequence"];
        for variant in stop_variants {
            let reason = map_anthropic_stop_reason(Some(variant), false);
            assert_eq!(
                reason,
                LanguageModelFinishReason::Stop,
                "Failed for variant: {}",
                variant
            );
        }
    }

    #[test]
    fn test_all_stop_variants_with_json() {
        let stop_variants = vec!["pause_turn", "end_turn", "stop_sequence"];
        for variant in stop_variants {
            let reason = map_anthropic_stop_reason(Some(variant), true);
            assert_eq!(
                reason,
                LanguageModelFinishReason::Stop,
                "Failed for variant: {} with json flag",
                variant
            );
        }
    }
}
