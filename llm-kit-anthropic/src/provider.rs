//! Anthropic Provider Implementation
//!
//! This module provides the main provider interface for creating Anthropic language models.
//! It follows the LLM Kit provider pattern and offers multiple convenience methods for
//! model creation.
//!
//! # Example
//!
//! ```rust,no_run
//! use llm_kit_anthropic::{AnthropicProvider, AnthropicProviderSettings};
//! use llm_kit_provider::provider::Provider;
//!
//! // Create provider with default settings
//! let anthropic = AnthropicProvider::new(AnthropicProviderSettings::default());
//!
//! // Create a language model
//! let model = anthropic.language_model("claude-3-5-sonnet-20241022".to_string());
//! ```

use std::collections::HashMap;
use std::sync::Arc;

use crate::anthropic_tools;
use crate::language_model::{AnthropicMessagesConfig, AnthropicMessagesLanguageModel};
use crate::options::AnthropicMessagesModelId;
use crate::settings::AnthropicProviderSettings;
use llm_kit_provider::embedding_model::EmbeddingModel;
use llm_kit_provider::error::ProviderError;
use llm_kit_provider::image_model::ImageModel;
use llm_kit_provider::language_model::LanguageModel;
use llm_kit_provider::provider::Provider;
use llm_kit_provider::reranking_model::RerankingModel;
use llm_kit_provider::speech_model::SpeechModel;
use llm_kit_provider::transcription_model::TranscriptionModel;

/// Anthropic provider for creating language models.
///
/// This struct implements the `Provider` trait and provides multiple ways to create
/// Anthropic language models.
///
/// # Example
///
/// ```rust,no_run
/// use llm_kit_anthropic::{AnthropicProvider, AnthropicProviderSettings};
///
/// let provider = AnthropicProvider::new(AnthropicProviderSettings::default());
///
/// // All of these create the same model:
/// let model1 = provider.language_model("claude-3-5-sonnet-20241022".to_string());
/// let model2 = provider.chat("claude-3-5-sonnet-20241022".to_string());
/// let model3 = provider.messages("claude-3-5-sonnet-20241022".to_string());
/// ```
#[derive(Clone)]
pub struct AnthropicProvider {
    base_url: String,
    provider_name: String,
    headers: Arc<dyn Fn() -> HashMap<String, String> + Send + Sync>,
}

impl AnthropicProvider {
    /// Creates a new Anthropic provider with the given settings.
    ///
    /// # Arguments
    ///
    /// * `settings` - Configuration settings for the provider
    ///
    /// # Returns
    ///
    /// An `AnthropicProvider` instance.
    ///
    /// # Panics
    ///
    /// Panics if the `ANTHROPIC_API_KEY` environment variable is not set and no API key is provided in settings.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use llm_kit_anthropic::{AnthropicProvider, AnthropicProviderSettings};
    ///
    /// let provider = AnthropicProvider::new(
    ///     AnthropicProviderSettings::new()
    ///         .with_api_key("your-api-key")
    ///         .with_base_url("https://api.anthropic.com/v1")
    /// );
    ///
    /// let model = provider.language_model("claude-3-5-sonnet-20241022".to_string());
    /// ```
    pub fn new(settings: AnthropicProviderSettings) -> Self {
        // Get base URL from settings or environment variable
        let base_url = settings
            .base_url
            .or_else(|| std::env::var("ANTHROPIC_BASE_URL").ok())
            .unwrap_or_else(|| "https://api.anthropic.com/v1".to_string());

        // Remove trailing slash if present
        let base_url = base_url.trim_end_matches('/').to_string();

        // Get provider name
        let provider_name = settings
            .name
            .unwrap_or_else(|| "anthropic.messages".to_string());

        // Create headers closure
        let api_key = settings
            .api_key
            .or_else(|| std::env::var("ANTHROPIC_API_KEY").ok())
            .expect("ANTHROPIC_API_KEY must be set either in settings or environment variable");

        let custom_headers = settings.headers.unwrap_or_default();

        let headers_fn = Arc::new(move || {
            let mut headers = HashMap::new();

            // Required Anthropic headers
            headers.insert("anthropic-version".to_string(), "2023-06-01".to_string());
            headers.insert("x-api-key".to_string(), api_key.clone());

            // Add custom headers
            headers.extend(custom_headers.clone());

            // TODO: Add User-Agent header with SDK version
            // headers.insert("User-Agent".to_string(), format!("llm-kit/anthropic/{}", VERSION));

            headers
        });

        Self {
            base_url,
            provider_name,
            headers: headers_fn,
        }
    }

    /// Create a language model with the given model ID.
    ///
    /// This is the primary method for creating language models.
    ///
    /// # Arguments
    ///
    /// * `model_id` - The Anthropic model identifier (e.g., "claude-3-5-sonnet-20241022")
    ///
    /// # Returns
    ///
    /// An `AnthropicMessagesLanguageModel` instance that implements `LanguageModel`.
    pub fn language_model(
        &self,
        model_id: AnthropicMessagesModelId,
    ) -> AnthropicMessagesLanguageModel {
        self.create_model(model_id)
    }

    /// Create a chat model with the given model ID.
    ///
    /// This is an alias for `language_model()`.
    pub fn chat(&self, model_id: AnthropicMessagesModelId) -> AnthropicMessagesLanguageModel {
        self.create_model(model_id)
    }

    /// Create a messages model with the given model ID.
    ///
    /// This is an alias for `language_model()`.
    pub fn messages(&self, model_id: AnthropicMessagesModelId) -> AnthropicMessagesLanguageModel {
        self.create_model(model_id)
    }

    /// Create a model instance with the given model ID.
    fn create_model(&self, model_id: AnthropicMessagesModelId) -> AnthropicMessagesLanguageModel {
        let config = AnthropicMessagesConfig::new(
            self.provider_name.clone(),
            self.base_url.clone(),
            self.headers.clone(),
        );

        AnthropicMessagesLanguageModel::new(model_id, config)
    }

    /// Access to Anthropic provider-defined tools.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use llm_kit_anthropic::{AnthropicProvider, AnthropicProviderSettings};
    ///
    /// let provider = AnthropicProvider::new(AnthropicProviderSettings::default());
    ///
    /// // Access tools through the provider
    /// let bash = provider.tools().bash_20250124(None);
    /// let computer = provider.tools().computer_20250124(1920, 1080, None);
    /// ```
    pub fn tools(&self) -> &anthropic_tools::AnthropicTools {
        // Return a reference to the tools namespace
        // In Rust, we can't have a module as a field, so we'll use a static instance
        &anthropic_tools::TOOLS
    }

    /// Gets the base URL for API calls.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Gets the provider name.
    pub fn provider_name(&self) -> &str {
        &self.provider_name
    }
}

impl Provider for AnthropicProvider {
    fn specification_version(&self) -> &str {
        "v3"
    }

    fn language_model(&self, model_id: &str) -> Result<Arc<dyn LanguageModel>, ProviderError> {
        Ok(Arc::new(self.create_model(model_id.to_string())))
    }

    fn text_embedding_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn EmbeddingModel<String>>, ProviderError> {
        Err(ProviderError::no_such_model(model_id, "textEmbeddingModel"))
    }

    fn image_model(&self, model_id: &str) -> Result<Arc<dyn ImageModel>, ProviderError> {
        Err(ProviderError::no_such_model(model_id, "imageModel"))
    }

    fn transcription_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn TranscriptionModel>, ProviderError> {
        Err(ProviderError::no_such_model(model_id, "transcriptionModel"))
    }

    fn speech_model(&self, model_id: &str) -> Result<Arc<dyn SpeechModel>, ProviderError> {
        Err(ProviderError::no_such_model(model_id, "speechModel"))
    }

    fn reranking_model(&self, model_id: &str) -> Result<Arc<dyn RerankingModel>, ProviderError> {
        Err(ProviderError::no_such_model(model_id, "rerankingModel"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_provider_removes_trailing_slash() {
        let provider = AnthropicProvider::new(
            AnthropicProviderSettings::new()
                .with_api_key("test-key")
                .with_base_url("https://api.anthropic.com/v1/"),
        );

        assert_eq!(provider.base_url(), "https://api.anthropic.com/v1");
    }

    #[test]
    fn test_provider_name_default() {
        let provider =
            AnthropicProvider::new(AnthropicProviderSettings::new().with_api_key("test-key"));

        assert_eq!(provider.provider_name(), "anthropic.messages");
    }

    #[test]
    fn test_provider_name_custom() {
        let provider = AnthropicProvider::new(
            AnthropicProviderSettings::new()
                .with_api_key("test-key")
                .with_name("my-provider"),
        );

        assert_eq!(provider.provider_name(), "my-provider");
    }

    #[test]
    fn test_headers_function() {
        let provider = AnthropicProvider::new(
            AnthropicProviderSettings::new()
                .with_api_key("test-key")
                .add_header("Custom", "value"),
        );

        let headers = (provider.headers)();
        assert_eq!(headers.get("x-api-key"), Some(&"test-key".to_string()));
        assert_eq!(
            headers.get("anthropic-version"),
            Some(&"2023-06-01".to_string())
        );
        assert_eq!(headers.get("Custom"), Some(&"value".to_string()));
    }

    #[test]
    fn test_create_model() {
        let provider =
            AnthropicProvider::new(AnthropicProviderSettings::new().with_api_key("test-key"));

        let model = provider.language_model("claude-3-5-sonnet-20241022".to_string());
        assert_eq!(model.model_id(), "claude-3-5-sonnet-20241022");
        assert_eq!(model.provider(), "anthropic.messages");
    }

    #[test]
    fn test_chat_alias() {
        let provider =
            AnthropicProvider::new(AnthropicProviderSettings::new().with_api_key("test-key"));

        let model = provider.chat("claude-3-5-sonnet-20241022".to_string());
        assert_eq!(model.model_id(), "claude-3-5-sonnet-20241022");
    }

    #[test]
    fn test_messages_alias() {
        let provider =
            AnthropicProvider::new(AnthropicProviderSettings::new().with_api_key("test-key"));

        let model = provider.messages("claude-3-5-sonnet-20241022".to_string());
        assert_eq!(model.model_id(), "claude-3-5-sonnet-20241022");
    }

    #[test]
    fn test_provider_trait() {
        let provider =
            AnthropicProvider::new(AnthropicProviderSettings::new().with_api_key("test-key"));

        assert_eq!(provider.specification_version(), "v3");
        assert_eq!(provider.provider_name(), "anthropic.messages");
    }
}
