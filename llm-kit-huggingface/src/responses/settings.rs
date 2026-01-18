use std::collections::HashMap;

/// Model ID for Hugging Face Responses API.
/// Can be any string, but we provide constants for common models.
pub type HuggingFaceResponsesModelId = String;

// Llama models
pub const LLAMA_3_1_8B_INSTRUCT: &str = "meta-llama/Llama-3.1-8B-Instruct";
pub const LLAMA_3_1_70B_INSTRUCT: &str = "meta-llama/Llama-3.1-70B-Instruct";
pub const LLAMA_3_1_405B_INSTRUCT: &str = "meta-llama/Llama-3.1-405B-Instruct";
pub const LLAMA_3_3_70B_INSTRUCT: &str = "meta-llama/Llama-3.3-70B-Instruct";
pub const LLAMA_3_8B_INSTRUCT: &str = "meta-llama/Meta-Llama-3-8B-Instruct";
pub const LLAMA_3_70B_INSTRUCT: &str = "meta-llama/Meta-Llama-3-70B-Instruct";
pub const LLAMA_3_2_3B_INSTRUCT: &str = "meta-llama/Llama-3.2-3B-Instruct";

// DeepSeek models
pub const DEEPSEEK_V3_1: &str = "deepseek-ai/DeepSeek-V3.1";
pub const DEEPSEEK_V3_0324: &str = "deepseek-ai/DeepSeek-V3-0324";
pub const DEEPSEEK_R1: &str = "deepseek-ai/DeepSeek-R1";
pub const DEEPSEEK_R1_0528: &str = "deepseek-ai/DeepSeek-R1-0528";
pub const DEEPSEEK_R1_DISTILL_QWEN_7B: &str = "deepseek-ai/DeepSeek-R1-Distill-Qwen-7B";
pub const DEEPSEEK_R1_DISTILL_LLAMA_8B: &str = "deepseek-ai/DeepSeek-R1-Distill-Llama-8B";

// Qwen models
pub const QWEN3_32B: &str = "Qwen/Qwen3-32B";
pub const QWEN3_14B: &str = "Qwen/Qwen3-14B";
pub const QWEN3_8B: &str = "Qwen/Qwen3-8B";
pub const QWEN2_5_7B_INSTRUCT: &str = "Qwen/Qwen2.5-7B-Instruct";
pub const QWEN2_5_CODER_7B_INSTRUCT: &str = "Qwen/Qwen2.5-Coder-7B-Instruct";
pub const QWEN2_5_VL_7B_INSTRUCT: &str = "Qwen/Qwen2.5-VL-7B-Instruct";

// Gemma models
pub const GEMMA_2_9B_IT: &str = "google/gemma-2-9b-it";
pub const GEMMA_3_27B_IT: &str = "google/gemma-3-27b-it";

// Other models
pub const KIMI_K2_INSTRUCT: &str = "moonshotai/Kimi-K2-Instruct";

/// Provider-specific settings for Hugging Face Responses API.
#[derive(Debug, Clone, Default)]
pub struct HuggingFaceResponsesSettings {
    /// Metadata to include in the request.
    pub metadata: Option<HashMap<String, String>>,

    /// Instructions to include in the request.
    pub instructions: Option<String>,

    /// Whether to use strict JSON schema validation.
    pub strict_json_schema: Option<bool>,
}

impl HuggingFaceResponsesSettings {
    /// Creates a new `HuggingFaceResponsesSettings` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets metadata.
    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Adds a single metadata entry.
    pub fn with_metadata_entry(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    /// Sets instructions.
    pub fn with_instructions(mut self, instructions: impl Into<String>) -> Self {
        self.instructions = Some(instructions.into());
        self
    }

    /// Sets strict JSON schema validation.
    pub fn with_strict_json_schema(mut self, strict: bool) -> Self {
        self.strict_json_schema = Some(strict);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_constants() {
        assert_eq!(LLAMA_3_1_8B_INSTRUCT, "meta-llama/Llama-3.1-8B-Instruct");
        assert_eq!(DEEPSEEK_V3_1, "deepseek-ai/DeepSeek-V3.1");
        assert_eq!(QWEN3_32B, "Qwen/Qwen3-32B");
    }

    #[test]
    fn test_settings_builder() {
        let settings = HuggingFaceResponsesSettings::new()
            .with_instructions("Be concise")
            .with_strict_json_schema(true)
            .with_metadata_entry("user_id", "123");

        assert_eq!(settings.instructions, Some("Be concise".to_string()));
        assert_eq!(settings.strict_json_schema, Some(true));
        assert_eq!(
            settings.metadata.as_ref().unwrap().get("user_id"),
            Some(&"123".to_string())
        );
    }
}
