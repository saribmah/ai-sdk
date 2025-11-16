/// Baseten chat model identifier.
///
/// # Supported Models (via Model APIs)
///
/// - `deepseek-ai/DeepSeek-R1-0528` - DeepSeek R1 with reasoning
/// - `deepseek-ai/DeepSeek-V3-0324` - DeepSeek V3
/// - `deepseek-ai/DeepSeek-V3.1` - DeepSeek V3.1
/// - `moonshotai/Kimi-K2-Instruct-0905` - Kimi K2
/// - `Qwen/Qwen3-235B-A22B-Instruct-2507` - Qwen 3
/// - `Qwen/Qwen3-Coder-480B-A35B-Instruct` - Qwen 3 Coder
/// - `openai/gpt-oss-120b` - GPT OSS
/// - `zai-org/GLM-4.6` - GLM 4.6
///
/// Custom models can also be used by providing their model ID or using a custom model URL.
pub type BasetenChatModelId = String;

/// Creates a chat model ID from a string.
///
/// # Examples
///
/// ```
/// use ai_sdk_baseten::chat::chat_model_id;
///
/// let model_id = chat_model_id("deepseek-ai/DeepSeek-V3-0324");
/// ```
pub fn chat_model_id(id: impl Into<String>) -> BasetenChatModelId {
    id.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_model_id() {
        let id = chat_model_id("deepseek-ai/DeepSeek-V3-0324");
        assert_eq!(id, "deepseek-ai/DeepSeek-V3-0324");

        let id = chat_model_id("custom-model");
        assert_eq!(id, "custom-model");
    }
}
