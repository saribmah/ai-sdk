use serde::{Deserialize, Serialize};

/// Usage information for a language model call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelUsage {
    /// The number of input (prompt) tokens used.
    #[serde(default)]
    pub input_tokens: u64,

    /// The number of output (completion) tokens used.
    #[serde(default)]
    pub output_tokens: u64,

    /// The total number of tokens as reported by the provider.
    #[serde(default)]
    pub total_tokens: u64,

    /// The number of reasoning tokens used.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub reasoning_tokens: u64,

    /// The number of cached input tokens.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub cached_input_tokens: u64,
}

fn is_zero(n: &u64) -> bool {
    *n == 0
}

impl LanguageModelUsage {
    pub fn new(input_tokens: u64, output_tokens: u64) -> Self {
        Self {
            input_tokens,
            output_tokens,
            total_tokens: input_tokens + output_tokens,
            reasoning_tokens: 0,
            cached_input_tokens: 0,
        }
    }

    pub fn total(&self) -> u64 {
        if self.total_tokens > 0 {
            self.total_tokens
        } else {
            self.input_tokens + self.output_tokens
        }
    }
}
