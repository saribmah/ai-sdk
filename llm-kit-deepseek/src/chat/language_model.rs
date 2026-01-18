use async_trait::async_trait;
use llm_kit_openai_compatible::{
    OpenAICompatibleChatConfig, OpenAICompatibleChatLanguageModel, OpenAICompatibleChatModelId,
};
use llm_kit_provider::language_model::call_options::LanguageModelCallOptions;
use llm_kit_provider::language_model::{
    LanguageModel, LanguageModelGenerateResponse, LanguageModelStreamResponse,
};
use regex::Regex;
use std::collections::HashMap;

use crate::provider::DeepSeekChatConfig;

/// DeepSeek chat language model implementation.
///
/// Wraps `OpenAICompatibleChatLanguageModel` to provide DeepSeek-specific
/// configuration and model instantiation.
pub struct DeepSeekChatLanguageModel {
    model_id: String,
    inner: OpenAICompatibleChatLanguageModel,
}

impl DeepSeekChatLanguageModel {
    /// Creates a new DeepSeek chat language model.
    pub fn new(model_id: String, config: DeepSeekChatConfig) -> Self {
        // Convert DeepSeek config to OpenAI-compatible config
        let openai_config = OpenAICompatibleChatConfig {
            provider: config.provider.clone(),
            headers: config.headers,
            url: Box::new(move |_model_id: &str, path: &str| {
                format!("{}{}", config.base_url, path)
            }),
            include_usage: true, // DeepSeek always includes usage
            supports_structured_outputs: false,
            supported_urls: None,
        };

        let inner = OpenAICompatibleChatLanguageModel::new(
            OpenAICompatibleChatModelId::from(model_id.clone()),
            openai_config,
        );

        Self { model_id, inner }
    }
}

#[async_trait]
impl LanguageModel for DeepSeekChatLanguageModel {
    fn model_id(&self) -> &str {
        &self.model_id
    }

    fn provider(&self) -> &str {
        self.inner.provider()
    }

    async fn supported_urls(&self) -> HashMap<String, Vec<Regex>> {
        self.inner.supported_urls().await
    }

    async fn do_generate(
        &self,
        options: LanguageModelCallOptions,
    ) -> Result<LanguageModelGenerateResponse, Box<dyn std::error::Error>> {
        // Delegate to the inner OpenAI-compatible model
        // DeepSeek uses the same API format as OpenAI
        self.inner.do_generate(options).await
    }

    async fn do_stream(
        &self,
        options: LanguageModelCallOptions,
    ) -> Result<LanguageModelStreamResponse, Box<dyn std::error::Error>> {
        // Delegate to the inner OpenAI-compatible model
        // DeepSeek uses the same streaming format as OpenAI
        self.inner.do_stream(options).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::DeepSeekChatConfig;
    use std::collections::HashMap;

    fn create_test_config() -> DeepSeekChatConfig {
        DeepSeekChatConfig {
            provider: "deepseek.chat".to_string(),
            base_url: "https://api.deepseek.com/v1".to_string(),
            headers: Box::new(HashMap::new),
        }
    }

    #[test]
    fn test_model_creation() {
        let config = create_test_config();
        let model = DeepSeekChatLanguageModel::new("deepseek-chat".to_string(), config);

        assert_eq!(model.model_id(), "deepseek-chat");
        assert_eq!(model.provider(), "deepseek.chat");
    }

    #[test]
    fn test_reasoner_model_creation() {
        let config = create_test_config();
        let model = DeepSeekChatLanguageModel::new("deepseek-reasoner".to_string(), config);

        assert_eq!(model.model_id(), "deepseek-reasoner");
        assert_eq!(model.provider(), "deepseek.chat");
    }
}
