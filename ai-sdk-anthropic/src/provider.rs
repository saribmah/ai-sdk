//! Anthropic Provider Implementation
//!
//! This module provides the main provider interface for creating Anthropic language models.
//! It follows the AI SDK provider pattern and offers multiple convenience methods for
//! model creation.
//!
//! # Example
//!
//! ```rust,no_run
//! use ai_sdk_anthropic::{create_anthropic, AnthropicProviderSettings};
//! use ai_sdk_provider::provider::Provider;
//!
//! // Create provider with default settings
//! let anthropic = create_anthropic(AnthropicProviderSettings::default());
//!
//! // Create a language model
//! let model = anthropic.language_model("claude-3-5-sonnet-20241022".to_string());
//! ```

use std::collections::HashMap;
use std::sync::Arc;

use crate::anthropic_tools;
use crate::language_model::{AnthropicMessagesConfig, AnthropicMessagesLanguageModel};
use crate::options::AnthropicMessagesModelId;
use ai_sdk_provider::embedding_model::EmbeddingModel;
use ai_sdk_provider::error::ProviderError;
use ai_sdk_provider::image_model::ImageModel;
use ai_sdk_provider::language_model::LanguageModel;
use ai_sdk_provider::provider::Provider;
use ai_sdk_provider::reranking_model::RerankingModel;
use ai_sdk_provider::speech_model::SpeechModel;
use ai_sdk_provider::transcription_model::TranscriptionModel;

/// Settings for configuring the Anthropic provider.
///
/// # Fields
///
/// * `base_url` - Use a different URL prefix for API calls, e.g. to use proxy servers.
///   The default prefix is `https://api.anthropic.com/v1`.
/// * `api_key` - API key that is being sent using the `x-api-key` header.
///   It defaults to the `ANTHROPIC_API_KEY` environment variable.
/// * `headers` - Custom headers to include in the requests.
/// * `name` - Custom provider name. Defaults to 'anthropic.messages'.
///
/// # Example
///
/// ```rust
/// use ai_sdk_anthropic::AnthropicProviderSettings;
/// use std::collections::HashMap;
///
/// let settings = AnthropicProviderSettings {
///     base_url: Some("https://api.anthropic.com/v1".to_string()),
///     api_key: Some("your-api-key".to_string()),
///     headers: Some({
///         let mut h = HashMap::new();
///         h.insert("Custom-Header".to_string(), "value".to_string());
///         h
///     }),
///     name: Some("my-anthropic".to_string()),
/// };
/// ```
#[derive(Debug, Clone, Default)]
pub struct AnthropicProviderSettings {
    /// Use a different URL prefix for API calls.
    /// Defaults to `https://api.anthropic.com/v1`.
    pub base_url: Option<String>,

    /// API key for authentication.
    /// Defaults to the `ANTHROPIC_API_KEY` environment variable.
    pub api_key: Option<String>,

    /// Custom headers to include in requests.
    pub headers: Option<HashMap<String, String>>,

    /// Custom provider name.
    /// Defaults to 'anthropic.messages'.
    pub name: Option<String>,
}

impl AnthropicProviderSettings {
    /// Create a new settings instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the base URL.
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Set the API key.
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Set custom headers.
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Add a single header.
    pub fn add_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        if let Some(ref mut headers) = self.headers {
            headers.insert(key.into(), value.into());
        } else {
            let mut headers = HashMap::new();
            headers.insert(key.into(), value.into());
            self.headers = Some(headers);
        }
        self
    }

    /// Set the provider name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

/// Anthropic provider for creating language models.
///
/// This struct implements the `Provider` trait and provides multiple ways to create
/// Anthropic language models.
///
/// # Example
///
/// ```rust,no_run
/// use ai_sdk_anthropic::{create_anthropic, AnthropicProviderSettings};
///
/// let provider = create_anthropic(AnthropicProviderSettings::default());
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
    /// use ai_sdk_anthropic::create_anthropic;
    ///
    /// let provider = create_anthropic(Default::default());
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

/// Create an Anthropic provider instance.
///
/// This is the main entry point for creating Anthropic language models.
///
/// # Arguments
///
/// * `settings` - Configuration settings for the provider
///
/// # Returns
///
/// An `AnthropicProvider` instance that can create language models.
///
/// # Example
///
/// ```rust,no_run
/// use ai_sdk_anthropic::{create_anthropic, AnthropicProviderSettings};
///
/// // Create with default settings (uses ANTHROPIC_API_KEY env var)
/// let provider = create_anthropic(AnthropicProviderSettings::default());
///
/// // Create with custom settings
/// let provider = create_anthropic(
///     AnthropicProviderSettings::new()
///         .with_api_key("your-api-key")
///         .with_base_url("https://api.anthropic.com/v1")
/// );
///
/// // Create a model
/// let model = provider.language_model("claude-3-5-sonnet-20241022".to_string());
/// ```
pub fn create_anthropic(settings: AnthropicProviderSettings) -> AnthropicProvider {
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
        // headers.insert("User-Agent".to_string(), format!("ai-sdk/anthropic/{}", VERSION));

        headers
    });

    AnthropicProvider {
        base_url,
        provider_name,
        headers: headers_fn,
    }
}

/// Default Anthropic provider instance.
///
/// This is a convenience function that creates a provider with default settings.
///
/// # Panics
///
/// Panics if the `ANTHROPIC_API_KEY` environment variable is not set.
///
/// # Example
///
/// ```rust,no_run
/// use ai_sdk_anthropic::anthropic;
///
/// let model = anthropic().language_model("claude-3-5-sonnet-20241022".to_string());
/// ```
pub fn anthropic() -> AnthropicProvider {
    create_anthropic(AnthropicProviderSettings::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_builder() {
        let settings = AnthropicProviderSettings::new()
            .with_api_key("test-key")
            .with_base_url("https://custom.api.com")
            .with_name("custom-provider");

        assert_eq!(settings.api_key, Some("test-key".to_string()));
        assert_eq!(
            settings.base_url,
            Some("https://custom.api.com".to_string())
        );
        assert_eq!(settings.name, Some("custom-provider".to_string()));
    }

    #[test]
    fn test_settings_add_header() {
        let settings = AnthropicProviderSettings::new()
            .add_header("Custom-Header", "value1")
            .add_header("Another-Header", "value2");

        let headers = settings.headers.unwrap();
        assert_eq!(headers.get("Custom-Header"), Some(&"value1".to_string()));
        assert_eq!(headers.get("Another-Header"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_create_provider_removes_trailing_slash() {
        let provider = create_anthropic(
            AnthropicProviderSettings::new()
                .with_api_key("test-key")
                .with_base_url("https://api.anthropic.com/v1/"),
        );

        assert_eq!(provider.base_url, "https://api.anthropic.com/v1");
    }

    #[test]
    fn test_provider_name_default() {
        let provider = create_anthropic(AnthropicProviderSettings::new().with_api_key("test-key"));

        assert_eq!(provider.provider_name, "anthropic.messages");
    }

    #[test]
    fn test_provider_name_custom() {
        let provider = create_anthropic(
            AnthropicProviderSettings::new()
                .with_api_key("test-key")
                .with_name("my-provider"),
        );

        assert_eq!(provider.provider_name, "my-provider");
    }

    #[test]
    fn test_headers_function() {
        let provider = create_anthropic(
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
        let provider = create_anthropic(AnthropicProviderSettings::new().with_api_key("test-key"));

        let model = provider.language_model("claude-3-5-sonnet-20241022".to_string());
        assert_eq!(model.model_id(), "claude-3-5-sonnet-20241022");
        assert_eq!(model.provider(), "anthropic.messages");
    }

    #[test]
    fn test_chat_alias() {
        let provider = create_anthropic(AnthropicProviderSettings::new().with_api_key("test-key"));

        let model = provider.chat("claude-3-5-sonnet-20241022".to_string());
        assert_eq!(model.model_id(), "claude-3-5-sonnet-20241022");
    }

    #[test]
    fn test_messages_alias() {
        let provider = create_anthropic(AnthropicProviderSettings::new().with_api_key("test-key"));

        let model = provider.messages("claude-3-5-sonnet-20241022".to_string());
        assert_eq!(model.model_id(), "claude-3-5-sonnet-20241022");
    }

    #[test]
    fn test_provider_trait() {
        let provider = create_anthropic(AnthropicProviderSettings::new().with_api_key("test-key"));

        assert_eq!(provider.specification_version(), "v3");
        assert_eq!(provider.provider_name, "anthropic.messages");
    }
}
