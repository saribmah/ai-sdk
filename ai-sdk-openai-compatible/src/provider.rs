use ai_sdk_provider::EmbeddingModel;
use ai_sdk_provider::ImageModel;
use ai_sdk_provider::error::ProviderError;
use ai_sdk_provider::language_model::LanguageModel;
use ai_sdk_provider::provider::Provider;
use std::collections::HashMap;
use std::sync::Arc;

use crate::chat::{OpenAICompatibleChatConfig, OpenAICompatibleChatLanguageModel};
use crate::completion::{
    OpenAICompatibleCompletionConfig, OpenAICompatibleCompletionLanguageModel,
};
use crate::embedding::{OpenAICompatibleEmbeddingConfig, OpenAICompatibleEmbeddingModel};
use crate::image::{OpenAICompatibleImageModel, OpenAICompatibleImageModelConfig};
use crate::settings::OpenAICompatibleProviderSettings;

/// OpenAI-compatible provider implementation.
///
/// Provides methods to create different types of language models.
pub struct OpenAICompatibleProvider {
    settings: OpenAICompatibleProviderSettings,
}

impl OpenAICompatibleProvider {
    /// Creates a new OpenAI-compatible provider.
    pub fn new(settings: OpenAICompatibleProviderSettings) -> Self {
        Self { settings }
    }

    /// Creates a language model with the given model ID.
    pub fn model(&self, model_id: impl Into<String>) -> Arc<dyn LanguageModel> {
        self.chat_model(model_id)
    }

    /// Creates a chat language model with the given model ID.
    pub fn chat_model(&self, model_id: impl Into<String>) -> Arc<dyn LanguageModel> {
        let model_id = model_id.into();
        let config = self.create_chat_config();

        Arc::new(OpenAICompatibleChatLanguageModel::new(model_id, config))
    }

    /// Creates a completion language model with the given model ID.
    pub fn completion_model(&self, model_id: impl Into<String>) -> Arc<dyn LanguageModel> {
        let model_id = model_id.into();
        let config = self.create_completion_config();

        Arc::new(OpenAICompatibleCompletionLanguageModel::new(
            model_id, config,
        ))
    }

    /// Creates a text embedding model with the given model ID.
    pub fn text_embedding_model(
        &self,
        model_id: impl Into<String>,
    ) -> Arc<dyn EmbeddingModel<String>> {
        let model_id = model_id.into();
        let config = self.create_embedding_config();

        Arc::new(OpenAICompatibleEmbeddingModel::new(model_id, config))
    }

    /// Creates an image model with the given model ID.
    pub fn image_model(&self, model_id: impl Into<String>) -> Arc<dyn ImageModel> {
        let model_id = model_id.into();
        let config = self.create_image_config();

        Arc::new(OpenAICompatibleImageModel::new(model_id, config))
    }

    /// Creates the configuration for chat models
    fn create_chat_config(&self) -> OpenAICompatibleChatConfig {
        let api_key = self.settings.api_key.clone();
        let custom_headers = self.settings.headers.clone().unwrap_or_default();
        let organization = self.settings.organization.clone();
        let project = self.settings.project.clone();
        let base_url = self.settings.base_url.clone();
        let query_params = self.settings.query_params.clone().unwrap_or_default();

        OpenAICompatibleChatConfig {
            provider: format!("{}.chat", self.settings.name),
            headers: Box::new(move || {
                let mut headers = HashMap::new();

                // Add Authorization header if API key is present
                if let Some(ref key) = api_key {
                    headers.insert("Authorization".to_string(), format!("Bearer {}", key));
                }

                // Add organization header if present
                if let Some(ref org) = organization {
                    headers.insert("OpenAI-Organization".to_string(), org.clone());
                }

                // Add project header if present
                if let Some(ref proj) = project {
                    headers.insert("OpenAI-Project".to_string(), proj.clone());
                }

                // Add custom headers
                for (key, value) in &custom_headers {
                    headers.insert(key.clone(), value.clone());
                }

                headers
            }),
            url: Box::new(move |_model_id: &str, path: &str| {
                let mut url = format!("{}{}", base_url, path);

                // Add query parameters if present
                if !query_params.is_empty() {
                    let params: Vec<String> = query_params
                        .iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect();
                    url = format!("{}?{}", url, params.join("&"));
                }

                url
            }),
            fetch: None,
            include_usage: self.settings.include_usage,
            supports_structured_outputs: self.settings.supports_structured_outputs,
            supported_urls: None,
        }
    }

    /// Creates the configuration for completion models
    fn create_completion_config(&self) -> OpenAICompatibleCompletionConfig {
        let api_key = self.settings.api_key.clone();
        let custom_headers = self.settings.headers.clone().unwrap_or_default();
        let organization = self.settings.organization.clone();
        let project = self.settings.project.clone();
        let base_url = self.settings.base_url.clone();
        let query_params = self.settings.query_params.clone().unwrap_or_default();

        OpenAICompatibleCompletionConfig {
            provider: format!("{}.completion", self.settings.name),
            headers: Box::new(move || {
                let mut headers = HashMap::new();

                // Add Authorization header if API key is present
                if let Some(ref key) = api_key {
                    headers.insert("Authorization".to_string(), format!("Bearer {}", key));
                }

                // Add organization header if present
                if let Some(ref org) = organization {
                    headers.insert("OpenAI-Organization".to_string(), org.clone());
                }

                // Add project header if present
                if let Some(ref proj) = project {
                    headers.insert("OpenAI-Project".to_string(), proj.clone());
                }

                // Add custom headers
                for (key, value) in &custom_headers {
                    headers.insert(key.clone(), value.clone());
                }

                headers
            }),
            url: Box::new(move |_model_id: &str, path: &str| {
                let mut url = format!("{}{}", base_url, path);

                // Add query parameters if present
                if !query_params.is_empty() {
                    let params: Vec<String> = query_params
                        .iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect();
                    url = format!("{}?{}", url, params.join("&"));
                }

                url
            }),
            fetch: None,
            include_usage: self.settings.include_usage,
        }
    }

    /// Creates the configuration for embedding models
    fn create_embedding_config(&self) -> OpenAICompatibleEmbeddingConfig {
        let api_key = self.settings.api_key.clone();
        let custom_headers = self.settings.headers.clone().unwrap_or_default();
        let organization = self.settings.organization.clone();
        let project = self.settings.project.clone();
        let base_url = self.settings.base_url.clone();
        let query_params = self.settings.query_params.clone().unwrap_or_default();

        OpenAICompatibleEmbeddingConfig {
            provider: format!("{}.embedding", self.settings.name),
            headers: Box::new(move || {
                let mut headers = HashMap::new();

                // Add Authorization header if API key is present
                if let Some(ref key) = api_key {
                    headers.insert("Authorization".to_string(), format!("Bearer {}", key));
                }

                // Add organization header if present
                if let Some(ref org) = organization {
                    headers.insert("OpenAI-Organization".to_string(), org.clone());
                }

                // Add project header if present
                if let Some(ref proj) = project {
                    headers.insert("OpenAI-Project".to_string(), proj.clone());
                }

                // Add custom headers
                for (key, value) in &custom_headers {
                    headers.insert(key.clone(), value.clone());
                }

                headers
            }),
            url: Box::new(move |_model_id: &str, path: &str| {
                let mut url = format!("{}{}", base_url, path);

                // Add query parameters if present
                if !query_params.is_empty() {
                    let params: Vec<String> = query_params
                        .iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect();
                    url = format!("{}?{}", url, params.join("&"));
                }

                url
            }),
            max_embeddings_per_call: None,
            supports_parallel_calls: None,
            fetch: None,
        }
    }

    /// Creates the configuration for image models
    fn create_image_config(&self) -> OpenAICompatibleImageModelConfig {
        let api_key = self.settings.api_key.clone();
        let custom_headers = self.settings.headers.clone().unwrap_or_default();
        let organization = self.settings.organization.clone();
        let project = self.settings.project.clone();
        let base_url = self.settings.base_url.clone();
        let query_params = self.settings.query_params.clone().unwrap_or_default();

        OpenAICompatibleImageModelConfig {
            provider: format!("{}.image", self.settings.name),
            headers: Box::new(move || {
                let mut headers = HashMap::new();

                // Add Authorization header if API key is present
                if let Some(ref key) = api_key {
                    headers.insert("Authorization".to_string(), format!("Bearer {}", key));
                }

                // Add organization header if present
                if let Some(ref org) = organization {
                    headers.insert("OpenAI-Organization".to_string(), org.clone());
                }

                // Add project header if present
                if let Some(ref proj) = project {
                    headers.insert("OpenAI-Project".to_string(), proj.clone());
                }

                // Add custom headers
                for (key, value) in &custom_headers {
                    headers.insert(key.clone(), value.clone());
                }

                headers
            }),
            url: Box::new(move |_model_id: &str, path: &str| {
                let mut url = format!("{}{}", base_url, path);

                // Add query parameters if present
                if !query_params.is_empty() {
                    let params: Vec<String> = query_params
                        .iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect();
                    url = format!("{}?{}", url, params.join("&"));
                }

                url
            }),
            fetch: None,
        }
    }

    /// Gets the provider name.
    pub fn name(&self) -> &str {
        &self.settings.name
    }

    /// Gets the base URL.
    pub fn base_url(&self) -> &str {
        &self.settings.base_url
    }
}

// Implement the Provider trait for compatibility with the provider interface
impl Provider for OpenAICompatibleProvider {
    fn language_model(&self, model_id: &str) -> Result<Arc<dyn LanguageModel>, ProviderError> {
        Ok(self.chat_model(model_id))
    }

    fn text_embedding_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn ai_sdk_provider::EmbeddingModel<String>>, ProviderError> {
        Ok(self.text_embedding_model(model_id))
    }

    fn image_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn ai_sdk_provider::ImageModel>, ProviderError> {
        Ok(self.image_model(model_id))
    }

    fn transcription_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn ai_sdk_provider::TranscriptionModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            format!("{}.transcription-model-not-supported", self.settings.name),
        ))
    }

    fn speech_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn ai_sdk_provider::SpeechModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            format!("{}.speech-model-not-supported", self.settings.name),
        ))
    }

    fn reranking_model(
        &self,
        model_id: &str,
    ) -> Result<Arc<dyn ai_sdk_provider::RerankingModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            model_id,
            format!("{}.reranking-model-not-supported", self.settings.name),
        ))
    }
}

/// Creates an OpenAI-compatible provider.
///
/// This function creates a provider that can be used with OpenAI-compatible APIs,
/// including OpenAI, Azure OpenAI, and other compatible services.
///
/// # Arguments
///
/// * `settings` - Configuration settings for the provider
///
/// # Returns
///
/// An `OpenAICompatibleProvider` instance.
///
/// # Examples
///
/// ```ignore
/// use openai_compatible::{create_openai_compatible, OpenAICompatibleProviderSettings};
///
/// // Create an OpenAI provider and get a chat model
/// let provider = create_openai_compatible(
///     OpenAICompatibleProviderSettings::new(
///         "https://api.openai.com/v1",
///         "openai"
///     )
///     .with_api_key("your-api-key")
/// );
///
/// let chat_model = provider.chat_model("gpt-4");
///
/// // Or get a completion model
/// let completion_model = provider.completion_model("gpt-3.5-turbo-instruct");
///
/// // Or chain it directly
/// let model = create_openai_compatible(
///     OpenAICompatibleProviderSettings::new(
///         "https://api.example.com/v1",
///         "example"
///     )
///     .with_api_key("your-api-key")
/// )
/// .chat_model("meta-llama/Llama-3-70b-chat-hf");
/// ```
pub fn create_openai_compatible(
    settings: OpenAICompatibleProviderSettings,
) -> OpenAICompatibleProvider {
    OpenAICompatibleProvider::new(settings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_openai_compatible() {
        let settings = OpenAICompatibleProviderSettings::new("https://api.openai.com/v1", "openai")
            .with_api_key("test-key")
            .with_organization("test-org");

        let provider = create_openai_compatible(settings);
        let model = provider.chat_model("gpt-4");

        assert_eq!(model.provider(), "openai.chat");
        assert_eq!(model.model_id(), "gpt-4");
    }

    #[test]
    fn test_create_azure_openai_compatible() {
        let settings = OpenAICompatibleProviderSettings::new(
            "https://my-resource.openai.azure.com/openai",
            "azure-openai",
        )
        .with_api_key("test-key");

        let provider = create_openai_compatible(settings);
        let model = provider.chat_model("gpt-4");

        assert_eq!(model.provider(), "azure-openai.chat");
        assert_eq!(model.model_id(), "gpt-4");
    }

    #[test]
    fn test_completion_model() {
        let settings = OpenAICompatibleProviderSettings::new("https://api.openai.com/v1", "openai")
            .with_api_key("test-key");

        let provider = create_openai_compatible(settings);
        let model = provider.completion_model("gpt-3.5-turbo-instruct");

        assert_eq!(model.provider(), "openai.completion");
        assert_eq!(model.model_id(), "gpt-3.5-turbo-instruct");
    }

    #[test]
    fn test_text_embedding_model() {
        let settings = OpenAICompatibleProviderSettings::new("https://api.openai.com/v1", "openai")
            .with_api_key("test-key");

        let provider = create_openai_compatible(settings);
        let model = provider.text_embedding_model("text-embedding-3-small");

        assert_eq!(model.provider(), "openai.embedding");
        assert_eq!(model.model_id(), "text-embedding-3-small");
    }

    #[test]
    fn test_image_model() {
        let settings = OpenAICompatibleProviderSettings::new("https://api.openai.com/v1", "openai")
            .with_api_key("test-key");

        let provider = create_openai_compatible(settings);
        let model = provider.image_model("dall-e-3");

        assert_eq!(model.provider(), "openai.image");
        assert_eq!(model.model_id(), "dall-e-3");
    }

    #[test]
    fn test_language_model_alias() {
        let settings = OpenAICompatibleProviderSettings::new("https://api.openai.com/v1", "openai")
            .with_api_key("test-key");

        let provider = create_openai_compatible(settings);
        let model_result = provider.language_model("gpt-4");
        let model = model_result.unwrap();

        assert_eq!(model.provider(), "openai.chat");
        assert_eq!(model.model_id(), "gpt-4");
    }

    #[test]
    fn test_chained_usage() {
        // Test the chained usage pattern similar to Vercel AI SDK
        let model = create_openai_compatible(
            OpenAICompatibleProviderSettings::new("https://api.example.com/v1", "example")
                .with_api_key("test-key"),
        )
        .chat_model("meta-llama/Llama-3-70b-chat-hf");

        assert_eq!(model.provider(), "example.chat");
        assert_eq!(model.model_id(), "meta-llama/Llama-3-70b-chat-hf");
    }

    #[test]
    fn test_settings_builder() {
        let settings =
            OpenAICompatibleProviderSettings::new("https://api.example.com/v1", "custom")
                .with_api_key("key")
                .with_organization("org")
                .with_project("proj")
                .with_header("X-Custom-Header", "value")
                .with_query_param("version", "2024-01")
                .with_include_usage(true)
                .with_supports_structured_outputs(true);

        assert_eq!(settings.base_url, "https://api.example.com/v1");
        assert_eq!(settings.name, "custom");
        assert_eq!(settings.api_key, Some("key".to_string()));
        assert_eq!(settings.organization, Some("org".to_string()));
        assert_eq!(settings.project, Some("proj".to_string()));
        assert!(settings.include_usage);
        assert!(settings.supports_structured_outputs);

        let headers = settings.headers.unwrap();
        assert_eq!(headers.get("X-Custom-Header"), Some(&"value".to_string()));

        let query_params = settings.query_params.unwrap();
        assert_eq!(query_params.get("version"), Some(&"2024-01".to_string()));
    }

    #[test]
    fn test_provider_getters() {
        let settings =
            OpenAICompatibleProviderSettings::new("https://api.example.com/v1", "example");

        let provider = create_openai_compatible(settings);
        assert_eq!(provider.name(), "example");
        assert_eq!(provider.base_url(), "https://api.example.com/v1");
    }

    #[test]
    fn test_provider_trait_implementation() {
        let settings = OpenAICompatibleProviderSettings::new("https://api.openai.com/v1", "openai")
            .with_api_key("test-key");

        let provider = create_openai_compatible(settings);

        // Use Provider trait method via trait object
        let provider_trait: &dyn Provider = &provider;

        // Test language model
        let model = provider_trait.language_model("gpt-4").unwrap();
        assert_eq!(model.provider(), "openai.chat");
        assert_eq!(model.model_id(), "gpt-4");

        // Test text embedding model
        let embedding_model = provider_trait
            .text_embedding_model("text-embedding-3-small")
            .unwrap();
        assert_eq!(embedding_model.provider(), "openai.embedding");
        assert_eq!(embedding_model.model_id(), "text-embedding-3-small");

        // Test image model
        let image_model = provider_trait.image_model("dall-e-3").unwrap();
        assert_eq!(image_model.provider(), "openai.image");
        assert_eq!(image_model.model_id(), "dall-e-3");
    }

    #[test]
    fn test_both_apis() {
        let settings =
            OpenAICompatibleProviderSettings::new("https://api.example.com/v1", "example")
                .with_api_key("test-key");

        let provider = create_openai_compatible(settings);

        // Test chat model
        let model1 = provider.chat_model("model-1");
        assert_eq!(model1.provider(), "example.chat");
        assert_eq!(model1.model_id(), "model-1");

        // Test completion model
        let model2 = provider.completion_model("model-2");
        assert_eq!(model2.provider(), "example.completion");
        assert_eq!(model2.model_id(), "model-2");

        // Test embedding model
        let model3 = provider.text_embedding_model("model-3");
        assert_eq!(model3.provider(), "example.embedding");
        assert_eq!(model3.model_id(), "model-3");

        // Test image model
        let model4 = provider.image_model("model-4");
        assert_eq!(model4.provider(), "example.image");
        assert_eq!(model4.model_id(), "model-4");

        // Test Provider trait API (via trait object)
        let provider_trait: &dyn Provider = &provider;
        let model5 = provider_trait.language_model("model-5").unwrap();
        assert_eq!(model5.provider(), "example.chat");
        assert_eq!(model5.model_id(), "model-5");

        // Test language_model as struct method (alias for chat_model)
        let model6_result = provider.language_model("model-6");
        let model6 = model6_result.unwrap();
        assert_eq!(model6.provider(), "example.chat");
        assert_eq!(model6.model_id(), "model-6");
    }

    #[tokio::test]
    async fn test_generate_text_integration() {
        use ai_sdk_core::GenerateTextBuilder;
        use ai_sdk_core::error::AISDKError;
        use ai_sdk_core::prompt::Prompt;

        let settings = OpenAICompatibleProviderSettings::new("https://api.openai.com/v1", "openai")
            .with_api_key("test-key");

        let provider = create_openai_compatible(settings);
        let model = provider.chat_model("gpt-4");

        // Create a simple text prompt
        let prompt = Prompt::text("Hello, how are you?");

        // Call GenerateTextBuilder - this will make an actual HTTP request and fail
        // because we're using a test API key, but it tests the integration
        let result = GenerateTextBuilder::new(model, prompt).execute().await;

        // Expect a ModelError (either network error or API auth error)
        // The important part is that the integration between GenerateTextBuilder and our provider works
        assert!(result.is_err());
        match result {
            Err(AISDKError::ModelError { .. }) => {
                // Expected - either network error or API error
                // This confirms the integration works
            }
            other => panic!("Expected ModelError but got: {:?}", other),
        }
    }
}
