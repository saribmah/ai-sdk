/// Type alias for Cerebras chat model identifiers.
///
/// Cerebras offers several language models for chat completion:
///
/// ## Production Models
/// - `llama3.1-8b` - Llama 3.1 8B parameter model
/// - `llama-3.3-70b` - Llama 3.3 70B parameter model
/// - `gpt-oss-120b` - GPT-OSS 120B parameter model
/// - `qwen-3-32b` - Qwen 3 32B parameter model
///
/// ## Preview Models
/// - `qwen-3-235b-a22b-instruct-2507` - Qwen 3 235B instruct model
/// - `qwen-3-235b-a22b-thinking-2507` - Qwen 3 235B thinking/reasoning model
/// - `zai-glm-4.6` - ZAI GLM 4.6 model
///
/// You can also use any custom model ID string.
///
/// # Examples
///
/// ```
/// use llm_kit_cerebras::CerebrasChatModelId;
///
/// let model_id: CerebrasChatModelId = "llama-3.3-70b".to_string();
/// ```
///
/// For more information on available models, see:
/// <https://inference-docs.cerebras.ai/models/overview>
pub type CerebrasChatModelId = String;

/// Common production model identifiers
pub mod models {
    /// Llama 3.1 8B parameter model
    pub const LLAMA_3_1_8B: &str = "llama3.1-8b";

    /// Llama 3.3 70B parameter model
    pub const LLAMA_3_3_70B: &str = "llama-3.3-70b";

    /// GPT-OSS 120B parameter model
    pub const GPT_OSS_120B: &str = "gpt-oss-120b";

    /// Qwen 3 32B parameter model
    pub const QWEN_3_32B: &str = "qwen-3-32b";

    /// Qwen 3 235B instruct model (preview)
    pub const QWEN_3_235B_INSTRUCT: &str = "qwen-3-235b-a22b-instruct-2507";

    /// Qwen 3 235B thinking/reasoning model (preview)
    pub const QWEN_3_235B_THINKING: &str = "qwen-3-235b-a22b-thinking-2507";

    /// ZAI GLM 4.6 model (preview)
    pub const ZAI_GLM_4_6: &str = "zai-glm-4.6";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_id_type() {
        let model_id: CerebrasChatModelId = "llama-3.3-70b".to_string();
        assert_eq!(model_id, "llama-3.3-70b");
    }

    #[test]
    fn test_model_constants() {
        assert_eq!(models::LLAMA_3_1_8B, "llama3.1-8b");
        assert_eq!(models::LLAMA_3_3_70B, "llama-3.3-70b");
        assert_eq!(models::GPT_OSS_120B, "gpt-oss-120b");
        assert_eq!(models::QWEN_3_32B, "qwen-3-32b");
        assert_eq!(
            models::QWEN_3_235B_INSTRUCT,
            "qwen-3-235b-a22b-instruct-2507"
        );
        assert_eq!(
            models::QWEN_3_235B_THINKING,
            "qwen-3-235b-a22b-thinking-2507"
        );
        assert_eq!(models::ZAI_GLM_4_6, "zai-glm-4.6");
    }
}
