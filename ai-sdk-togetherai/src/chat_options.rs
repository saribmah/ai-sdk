/// Together AI chat model identifiers.
///
/// These are the supported chat models available through Together AI.
/// For a complete list of available models, see: <https://docs.together.ai/docs/serverless-models#chat-models>
pub type TogetherAIChatModelId = String;

/// Common Together AI chat model IDs
#[allow(dead_code)]
pub mod models {
    /// Meta Llama 3.3 70B Instruct Turbo
    pub const LLAMA_3_3_70B_INSTRUCT_TURBO: &str = "meta-llama/Llama-3.3-70B-Instruct-Turbo";

    /// Meta Llama 3.1 8B Instruct Turbo
    pub const LLAMA_3_1_8B_INSTRUCT_TURBO: &str = "meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo";

    /// Meta Llama 3.1 70B Instruct Turbo
    pub const LLAMA_3_1_70B_INSTRUCT_TURBO: &str = "meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo";

    /// Meta Llama 3.1 405B Instruct Turbo
    pub const LLAMA_3_1_405B_INSTRUCT_TURBO: &str = "meta-llama/Meta-Llama-3.1-405B-Instruct-Turbo";

    /// Qwen 2.5 Coder 32B Instruct
    pub const QWEN_2_5_CODER_32B_INSTRUCT: &str = "Qwen/Qwen2.5-Coder-32B-Instruct";

    /// Qwen 2.5 7B Instruct Turbo
    pub const QWEN_2_5_7B_INSTRUCT_TURBO: &str = "Qwen/Qwen2.5-7B-Instruct-Turbo";

    /// Qwen 2.5 72B Instruct Turbo
    pub const QWEN_2_5_72B_INSTRUCT_TURBO: &str = "Qwen/Qwen2.5-72B-Instruct-Turbo";

    /// DeepSeek V3
    pub const DEEPSEEK_V3: &str = "deepseek-ai/DeepSeek-V3";

    /// Mistral 7B Instruct v0.3
    pub const MISTRAL_7B_INSTRUCT_V0_3: &str = "mistralai/Mistral-7B-Instruct-v0.3";

    /// Mixtral 8x7B Instruct v0.1
    pub const MIXTRAL_8X7B_INSTRUCT_V0_1: &str = "mistralai/Mixtral-8x7B-Instruct-v0.1";

    /// Mixtral 8x22B Instruct v0.1
    pub const MIXTRAL_8X22B_INSTRUCT_V0_1: &str = "mistralai/Mixtral-8x22B-Instruct-v0.1";
}
