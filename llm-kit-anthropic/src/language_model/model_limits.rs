/// Result of checking max output tokens for a model
pub struct MaxOutputTokensResult {
    pub max_output_tokens: u64,
    pub known_model: bool,
}

/// Get the maximum output tokens for a given Anthropic model
///
/// See: <https://docs.claude.com/en/docs/about-claude/models/overview#model-comparison-table>
pub fn get_max_output_tokens_for_model(model_id: &str) -> MaxOutputTokensResult {
    if model_id.contains("claude-sonnet-4-")
        || model_id.contains("claude-3-7-sonnet")
        || model_id.contains("claude-haiku-4-5")
    {
        MaxOutputTokensResult {
            max_output_tokens: 64000,
            known_model: true,
        }
    } else if model_id.contains("claude-opus-4-") {
        MaxOutputTokensResult {
            max_output_tokens: 32000,
            known_model: true,
        }
    } else if model_id.contains("claude-3-5-haiku") {
        MaxOutputTokensResult {
            max_output_tokens: 8192,
            known_model: true,
        }
    } else if model_id.contains("claude-3-haiku") {
        MaxOutputTokensResult {
            max_output_tokens: 4096,
            known_model: true,
        }
    } else {
        MaxOutputTokensResult {
            max_output_tokens: 4096,
            known_model: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claude_sonnet_4() {
        let result = get_max_output_tokens_for_model("claude-sonnet-4-20250514");
        assert_eq!(result.max_output_tokens, 64000);
        assert!(result.known_model);
    }

    #[test]
    fn test_claude_3_7_sonnet() {
        let result = get_max_output_tokens_for_model("claude-3-7-sonnet-20250219");
        assert_eq!(result.max_output_tokens, 64000);
        assert!(result.known_model);
    }

    #[test]
    fn test_claude_haiku_4_5() {
        let result = get_max_output_tokens_for_model("claude-haiku-4-5-20250514");
        assert_eq!(result.max_output_tokens, 64000);
        assert!(result.known_model);
    }

    #[test]
    fn test_claude_opus_4() {
        let result = get_max_output_tokens_for_model("claude-opus-4-20250514");
        assert_eq!(result.max_output_tokens, 32000);
        assert!(result.known_model);
    }

    #[test]
    fn test_claude_3_5_haiku() {
        let result = get_max_output_tokens_for_model("claude-3-5-haiku-20241022");
        assert_eq!(result.max_output_tokens, 8192);
        assert!(result.known_model);
    }

    #[test]
    fn test_claude_3_haiku() {
        let result = get_max_output_tokens_for_model("claude-3-haiku-20240307");
        assert_eq!(result.max_output_tokens, 4096);
        assert!(result.known_model);
    }

    #[test]
    fn test_unknown_model() {
        let result = get_max_output_tokens_for_model("unknown-model");
        assert_eq!(result.max_output_tokens, 4096);
        assert!(!result.known_model);
    }
}
