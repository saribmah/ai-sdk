use ai_sdk_openai_compatible::{
    OpenAICompatibleChatConfig, OpenAICompatibleChatLanguageModel,
    OpenAICompatibleCompletionConfig, OpenAICompatibleCompletionLanguageModel,
    OpenAICompatibleEmbeddingConfig, OpenAICompatibleEmbeddingModel,
};
use ai_sdk_provider::error::ProviderError;
use ai_sdk_provider::provider::Provider;
use ai_sdk_provider::{EmbeddingModel, ImageModel, LanguageModel, RerankingModel};
use std::collections::HashMap;
use std::sync::Arc;

use crate::image::image_model::{TogetherAIImageModel, TogetherAIImageModelConfig};
use crate::reranking::reranking_model::{TogetherAIRerankingModel, TogetherAIRerankingModelConfig};
use crate::settings::TogetherAIProviderSettings;

/// Together AI provider implementation.
///
/// Provides access to Together AI's models including:
/// - Chat models (Llama, Mistral, Qwen, DeepSeek, etc.)
/// - Completion models
/// - Text embedding models
/// - Image generation models (FLUX, Stable Diffusion)
/// - Reranking models
///
/// # Examples
///
/// ```no_run
/// use ai_sdk_togetherai::{TogetherAIProvider, TogetherAIProviderSettings};
///
/// let provider = TogetherAIProvider::new(
///     TogetherAIProviderSettings::new()
///         .with_api_key("your-api-key")
/// );
///
/// let chat_model = provider.chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");
/// ```
pub struct TogetherAIProvider {
    settings: TogetherAIProviderSettings,
}

impl TogetherAIProvider {
    /// Creates a new Together AI provider.
    ///
    /// # Arguments
    ///
    /// * `settings` - Configuration settings for the provider
    pub fn new(settings: TogetherAIProviderSettings) -> Self {
        Self { settings }
    }

    /// Returns the provider name.
    pub fn name(&self) -> &str {
        "togetherai"
    }

    /// Returns the base URL.
    pub fn base_url(&self) -> &str {
        &self.settings.base_url
    }

    /// Creates a language model with the given model ID (alias for `chat_model`).
    ///
    /// # Arguments
    ///
    /// * `model_id` - The model identifier
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ai_sdk_togetherai::{TogetherAIProvider, TogetherAIProviderSettings};
    ///
    /// let provider = TogetherAIProvider::new(TogetherAIProviderSettings::new());
    /// let model = provider.model("meta-llama/Llama-3.3-70B-Instruct-Turbo");
    /// ```
    pub fn model(&self, model_id: impl Into<String>) -> Arc<dyn LanguageModel> {
        self.chat_model(model_id)
    }

    /// Creates a chat language model with the given model ID.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The model identifier (e.g., "meta-llama/Llama-3.3-70B-Instruct-Turbo")
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ai_sdk_togetherai::{TogetherAIProvider, TogetherAIProviderSettings};
    ///
    /// let provider = TogetherAIProvider::new(TogetherAIProviderSettings::new());
    /// let model = provider.chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");
    /// ```
    pub fn chat_model(&self, model_id: impl Into<String>) -> Arc<dyn LanguageModel> {
        let model_id = model_id.into();
        let config = self.create_chat_config();

        Arc::new(OpenAICompatibleChatLanguageModel::new(model_id, config))
    }

    /// Creates a completion language model with the given model ID.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The model identifier
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ai_sdk_togetherai::{TogetherAIProvider, TogetherAIProviderSettings};
    ///
    /// let provider = TogetherAIProvider::new(TogetherAIProviderSettings::new());
    /// let model = provider.completion_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");
    /// ```
    pub fn completion_model(&self, model_id: impl Into<String>) -> Arc<dyn LanguageModel> {
        let model_id = model_id.into();
        let config = self.create_completion_config();

        Arc::new(OpenAICompatibleCompletionLanguageModel::new(
            model_id, config,
        ))
    }

    /// Creates a text embedding model with the given model ID.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The model identifier (e.g., "WhereIsAI/UAE-Large-V1")
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ai_sdk_togetherai::{TogetherAIProvider, TogetherAIProviderSettings};
    ///
    /// let provider = TogetherAIProvider::new(TogetherAIProviderSettings::new());
    /// let model = provider.text_embedding_model("WhereIsAI/UAE-Large-V1");
    /// ```
    pub fn text_embedding_model(
        &self,
        model_id: impl Into<String>,
    ) -> Arc<dyn EmbeddingModel<String>> {
        let model_id = model_id.into();
        let config = self.create_embedding_config();

        Arc::new(OpenAICompatibleEmbeddingModel::new(model_id, config))
    }

    /// Creates an image generation model with the given model ID.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The model identifier (e.g., "black-forest-labs/FLUX.1-schnell")
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ai_sdk_togetherai::{TogetherAIProvider, TogetherAIProviderSettings};
    ///
    /// let provider = TogetherAIProvider::new(TogetherAIProviderSettings::new());
    /// let model = provider.image_model("black-forest-labs/FLUX.1-schnell");
    /// ```
    pub fn image_model(&self, model_id: impl Into<String>) -> Arc<dyn ImageModel> {
        let model_id = model_id.into();
        let config = self.create_image_config();

        Arc::new(TogetherAIImageModel::new(model_id, config))
    }

    /// Creates a reranking model with the given model ID.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The model identifier (e.g., "Salesforce/Llama-Rank-v1")
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ai_sdk_togetherai::{TogetherAIProvider, TogetherAIProviderSettings};
    ///
    /// let provider = TogetherAIProvider::new(TogetherAIProviderSettings::new());
    /// let model = provider.reranking_model("Salesforce/Llama-Rank-v1");
    /// ```
    pub fn reranking_model(&self, model_id: impl Into<String>) -> Arc<dyn RerankingModel> {
        let model_id = model_id.into();
        let config = self.create_reranking_config();

        Arc::new(TogetherAIRerankingModel::new(model_id, config))
    }

    /// Creates the configuration for chat models.
    fn create_chat_config(&self) -> OpenAICompatibleChatConfig {
        let api_key = self.settings.get_api_key();
        let custom_headers = self.settings.headers.clone().unwrap_or_default();
        let base_url = self.settings.base_url.clone();

        OpenAICompatibleChatConfig {
            provider: "togetherai.chat".to_string(),
            headers: Box::new(move || {
                let mut headers = HashMap::new();

                // Add Authorization header if API key is present
                if let Some(ref key) = api_key {
                    headers.insert("Authorization".to_string(), format!("Bearer {}", key));
                }

                // Add custom headers
                for (key, value) in &custom_headers {
                    headers.insert(key.clone(), value.clone());
                }

                headers
            }),
            url: Box::new(move |_model_id: &str, path: &str| format!("{}{}", base_url, path)),
            include_usage: true,
            supports_structured_outputs: false,
            supported_urls: None,
        }
    }

    /// Creates the configuration for completion models.
    fn create_completion_config(&self) -> OpenAICompatibleCompletionConfig {
        let api_key = self.settings.get_api_key();
        let custom_headers = self.settings.headers.clone().unwrap_or_default();
        let base_url = self.settings.base_url.clone();

        OpenAICompatibleCompletionConfig {
            provider: "togetherai.completion".to_string(),
            headers: Box::new(move || {
                let mut headers = HashMap::new();

                // Add Authorization header if API key is present
                if let Some(ref key) = api_key {
                    headers.insert("Authorization".to_string(), format!("Bearer {}", key));
                }

                // Add custom headers
                for (key, value) in &custom_headers {
                    headers.insert(key.clone(), value.clone());
                }

                headers
            }),
            url: Box::new(move |_model_id: &str, path: &str| format!("{}{}", base_url, path)),
            include_usage: true,
        }
    }

    /// Creates the configuration for embedding models.
    fn create_embedding_config(&self) -> OpenAICompatibleEmbeddingConfig {
        let api_key = self.settings.get_api_key();
        let custom_headers = self.settings.headers.clone().unwrap_or_default();
        let base_url = self.settings.base_url.clone();

        OpenAICompatibleEmbeddingConfig {
            provider: "togetherai.embedding".to_string(),
            headers: Box::new(move || {
                let mut headers = HashMap::new();

                // Add Authorization header if API key is present
                if let Some(ref key) = api_key {
                    headers.insert("Authorization".to_string(), format!("Bearer {}", key));
                }

                // Add custom headers
                for (key, value) in &custom_headers {
                    headers.insert(key.clone(), value.clone());
                }

                headers
            }),
            url: Box::new(move |_model_id: &str, path: &str| format!("{}{}", base_url, path)),
            max_embeddings_per_call: None,
            supports_parallel_calls: None,
        }
    }

    /// Creates the configuration for image models.
    fn create_image_config(&self) -> TogetherAIImageModelConfig {
        let api_key = self.settings.get_api_key();
        let custom_headers = self.settings.headers.clone().unwrap_or_default();
        let base_url = self.settings.base_url.clone();

        TogetherAIImageModelConfig {
            provider: "togetherai.image".to_string(),
            base_url: base_url.clone(),
            headers: Box::new(move || {
                let mut headers = HashMap::new();

                // Add Authorization header if API key is present
                if let Some(ref key) = api_key {
                    headers.insert("Authorization".to_string(), format!("Bearer {}", key));
                }

                // Add custom headers
                for (key, value) in &custom_headers {
                    headers.insert(key.clone(), value.clone());
                }

                headers
            }),
        }
    }

    /// Creates the configuration for reranking models.
    fn create_reranking_config(&self) -> TogetherAIRerankingModelConfig {
        let api_key = self.settings.get_api_key();
        let custom_headers = self.settings.headers.clone().unwrap_or_default();
        let base_url = self.settings.base_url.clone();

        TogetherAIRerankingModelConfig {
            provider: "togetherai.reranking".to_string(),
            base_url: base_url.clone(),
            headers: Box::new(move || {
                let mut headers = HashMap::new();

                // Add Authorization header if API key is present
                if let Some(ref key) = api_key {
                    headers.insert("Authorization".to_string(), format!("Bearer {}", key));
                }

                // Add custom headers
                for (key, value) in &custom_headers {
                    headers.insert(key.clone(), value.clone());
                }

                headers
            }),
        }
    }
}

// Implement the Provider trait for compatibility with the provider interface
impl Provider for TogetherAIProvider {
    fn language_model(&self, model_id: &str) -> Result<Arc<dyn LanguageModel>, ProviderError> {
        Ok(self.chat_model(model_id))
    }

    fn text_embedding_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn EmbeddingModel<String>>, ProviderError> {
        Ok(self.text_embedding_model(model_id))
    }

    fn image_model(&self, model_id: &str) -> Result<Arc<dyn ImageModel>, ProviderError> {
        Ok(self.image_model(model_id))
    }

    fn transcription_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn ai_sdk_provider::TranscriptionModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "togetherai.transcription-model-not-supported",
        ))
    }

    fn speech_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn ai_sdk_provider::SpeechModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            "togetherai.speech-model-not-supported",
        ))
    }

    fn reranking_model(&self, model_id: &str) -> Result<Arc<dyn RerankingModel>, ProviderError> {
        Ok(self.reranking_model(model_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_togetherai() {
        let settings = TogetherAIProviderSettings::new().with_api_key("test-key");

        let provider = TogetherAIProvider::new(settings);
        let model = provider.chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");

        assert_eq!(model.provider(), "togetherai.chat");
        assert_eq!(model.model_id(), "meta-llama/Llama-3.3-70B-Instruct-Turbo");
    }

    #[test]
    fn test_completion_model() {
        let settings = TogetherAIProviderSettings::new().with_api_key("test-key");

        let provider = TogetherAIProvider::new(settings);
        let model = provider.completion_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");

        assert_eq!(model.provider(), "togetherai.completion");
        assert_eq!(model.model_id(), "meta-llama/Llama-3.3-70B-Instruct-Turbo");
    }

    #[test]
    fn test_text_embedding_model() {
        let settings = TogetherAIProviderSettings::new().with_api_key("test-key");

        let provider = TogetherAIProvider::new(settings);
        let model = provider.text_embedding_model("WhereIsAI/UAE-Large-V1");

        assert_eq!(model.provider(), "togetherai.embedding");
        assert_eq!(model.model_id(), "WhereIsAI/UAE-Large-V1");
    }

    #[test]
    fn test_image_model() {
        let settings = TogetherAIProviderSettings::new().with_api_key("test-key");

        let provider = TogetherAIProvider::new(settings);
        let model = provider.image_model("black-forest-labs/FLUX.1-schnell");

        assert_eq!(model.provider(), "togetherai.image");
        assert_eq!(model.model_id(), "black-forest-labs/FLUX.1-schnell");
    }

    #[test]
    fn test_reranking_model() {
        let settings = TogetherAIProviderSettings::new().with_api_key("test-key");

        let provider = TogetherAIProvider::new(settings);
        let model = provider.reranking_model("Salesforce/Llama-Rank-v1");

        assert_eq!(model.provider(), "togetherai.reranking");
        assert_eq!(model.model_id(), "Salesforce/Llama-Rank-v1");
    }

    #[test]
    fn test_provider_trait_implementation() {
        let settings = TogetherAIProviderSettings::new().with_api_key("test-key");

        let provider = TogetherAIProvider::new(settings);
        let provider_trait: &dyn Provider = &provider;

        // Test language model
        let model = provider_trait
            .language_model("meta-llama/Llama-3.3-70B-Instruct-Turbo")
            .unwrap();
        assert_eq!(model.provider(), "togetherai.chat");

        // Test embedding model
        let embedding_model = provider_trait
            .text_embedding_model("WhereIsAI/UAE-Large-V1")
            .unwrap();
        assert_eq!(embedding_model.provider(), "togetherai.embedding");

        // Test image model
        let image_model = provider_trait
            .image_model("black-forest-labs/FLUX.1-schnell")
            .unwrap();
        assert_eq!(image_model.provider(), "togetherai.image");

        // Test reranking model
        let reranking_model = provider_trait
            .reranking_model("Salesforce/Llama-Rank-v1")
            .unwrap();
        assert_eq!(reranking_model.provider(), "togetherai.reranking");
    }
}
