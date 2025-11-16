use ai_sdk_openai_compatible::{
    OpenAICompatibleChatConfig, OpenAICompatibleChatLanguageModel,
    OpenAICompatibleCompletionConfig, OpenAICompatibleCompletionLanguageModel,
    OpenAICompatibleEmbeddingConfig, OpenAICompatibleEmbeddingModel, OpenAICompatibleImageModel,
    OpenAICompatibleImageModelConfig,
};
use ai_sdk_provider::error::ProviderError;
use ai_sdk_provider::language_model::LanguageModel;
use ai_sdk_provider::provider::Provider;
use ai_sdk_provider::{EmbeddingModel, ImageModel};
use std::collections::HashMap;
use std::sync::Arc;

use crate::settings::AzureOpenAIProviderSettings;

/// Azure OpenAI provider implementation.
///
/// This provider creates language models, embedding models, and image models
/// that use Azure OpenAI endpoints. It handles Azure-specific authentication
/// (api-key header) and URL formatting (deployment-based or v1 API).
///
/// # Examples
///
/// ## Using Builder Pattern (Recommended)
///
/// ```no_run
/// use ai_sdk_azure::AzureClient;
///
/// let provider = AzureClient::new()
///     .resource_name("my-azure-resource")
///     .api_key("your-api-key")
///     .build();
///
/// let model = provider.chat_model("gpt-4-deployment");
/// ```
///
/// ## Direct Instantiation (Alternative)
///
/// ```no_run
/// use ai_sdk_azure::{AzureOpenAIProvider, AzureOpenAIProviderSettings};
///
/// let provider = AzureOpenAIProvider::new(
///     AzureOpenAIProviderSettings::new()
///         .with_resource_name("my-azure-resource")
///         .with_api_key("your-api-key")
/// );
///
/// let model = provider.chat_model("gpt-4-deployment");
/// ```
pub struct AzureOpenAIProvider {
    settings: AzureOpenAIProviderSettings,
}

impl AzureOpenAIProvider {
    /// Creates a new Azure OpenAI provider.
    ///
    /// # Panics
    ///
    /// Panics if the settings are invalid (missing both base_url and resource_name).
    pub fn new(settings: AzureOpenAIProviderSettings) -> Self {
        // Validate settings on creation
        if let Err(e) = settings.validate() {
            panic!("Invalid Azure OpenAI provider settings: {}", e);
        }
        Self { settings }
    }

    /// Helper function to build URLs for Azure OpenAI API calls.
    ///
    /// Azure supports two URL formats:
    /// 1. Deployment-based (legacy): `{base_url}/deployments/{deployment_id}{path}?api-version={version}`
    /// 2. V1 API (default): `{base_url}/v1{path}?api-version={version}`
    fn build_url(
        base_url: &str,
        deployment_id: &str,
        path: &str,
        api_version: &str,
        use_deployment_based: bool,
    ) -> String {
        let full_path = if use_deployment_based {
            format!("{}/deployments/{}{}", base_url, deployment_id, path)
        } else {
            format!("{}/v1{}", base_url, path)
        };

        // Add api-version query parameter
        match url::Url::parse(&full_path) {
            Ok(mut url) => {
                url.query_pairs_mut()
                    .append_pair("api-version", api_version);
                url.to_string()
            }
            Err(_) => full_path,
        }
    }

    /// Creates a language model with the given deployment ID.
    ///
    /// This is an alias for `chat_model()`.
    pub fn model(&self, deployment_id: impl Into<String>) -> Arc<dyn LanguageModel> {
        self.chat_model(deployment_id)
    }

    /// Creates a chat language model with the given deployment ID.
    ///
    /// # Arguments
    ///
    /// * `deployment_id` - The deployment name/ID in Azure OpenAI
    pub fn chat_model(&self, deployment_id: impl Into<String>) -> Arc<dyn LanguageModel> {
        let deployment_id = deployment_id.into();
        let config = self.create_chat_config();
        Arc::new(OpenAICompatibleChatLanguageModel::new(
            deployment_id,
            config,
        ))
    }

    /// Creates a completion language model with the given deployment ID.
    ///
    /// # Arguments
    ///
    /// * `deployment_id` - The deployment name/ID in Azure OpenAI
    pub fn completion_model(&self, deployment_id: impl Into<String>) -> Arc<dyn LanguageModel> {
        let deployment_id = deployment_id.into();
        let config = self.create_completion_config();
        Arc::new(OpenAICompatibleCompletionLanguageModel::new(
            deployment_id,
            config,
        ))
    }

    /// Creates a text embedding model with the given deployment ID.
    ///
    /// # Arguments
    ///
    /// * `deployment_id` - The deployment name/ID in Azure OpenAI
    pub fn text_embedding_model(
        &self,
        deployment_id: impl Into<String>,
    ) -> Arc<dyn EmbeddingModel<String>> {
        let deployment_id = deployment_id.into();
        let config = self.create_embedding_config();
        Arc::new(OpenAICompatibleEmbeddingModel::new(deployment_id, config))
    }

    /// Creates an image model with the given deployment ID.
    ///
    /// # Arguments
    ///
    /// * `deployment_id` - The deployment name/ID in Azure OpenAI (e.g., dall-e-3 deployment)
    pub fn image_model(&self, deployment_id: impl Into<String>) -> Arc<dyn ImageModel> {
        let deployment_id = deployment_id.into();
        let config = self.create_image_config();
        Arc::new(OpenAICompatibleImageModel::new(deployment_id, config))
    }

    /// Creates the configuration for chat models.
    fn create_chat_config(&self) -> OpenAICompatibleChatConfig {
        let api_key = self.settings.api_key.clone();
        let custom_headers = self.settings.headers.clone().unwrap_or_default();
        let base_url = self
            .settings
            .get_base_url()
            .expect("Base URL should be validated");
        let api_version = self.settings.api_version.clone();
        let use_deployment_based = self.settings.use_deployment_based_urls;

        OpenAICompatibleChatConfig {
            provider: "azure.chat".to_string(),
            headers: Box::new(move || {
                let mut headers = HashMap::new();

                // Azure uses 'api-key' header instead of 'Authorization: Bearer'
                if let Some(ref key) = api_key {
                    headers.insert("api-key".to_string(), key.clone());
                }

                // Add custom headers
                for (key, value) in &custom_headers {
                    headers.insert(key.clone(), value.clone());
                }

                headers
            }),
            url: Box::new(move |model_id: &str, path: &str| {
                Self::build_url(
                    &base_url,
                    model_id,
                    path,
                    &api_version,
                    use_deployment_based,
                )
            }),
            include_usage: true,
            supports_structured_outputs: false,
            supported_urls: None,
        }
    }

    /// Creates the configuration for completion models.
    fn create_completion_config(&self) -> OpenAICompatibleCompletionConfig {
        let api_key = self.settings.api_key.clone();
        let custom_headers = self.settings.headers.clone().unwrap_or_default();
        let base_url = self
            .settings
            .get_base_url()
            .expect("Base URL should be validated");
        let api_version = self.settings.api_version.clone();
        let use_deployment_based = self.settings.use_deployment_based_urls;

        OpenAICompatibleCompletionConfig {
            provider: "azure.completion".to_string(),
            headers: Box::new(move || {
                let mut headers = HashMap::new();

                if let Some(ref key) = api_key {
                    headers.insert("api-key".to_string(), key.clone());
                }

                for (key, value) in &custom_headers {
                    headers.insert(key.clone(), value.clone());
                }

                headers
            }),
            url: Box::new(move |model_id: &str, path: &str| {
                Self::build_url(
                    &base_url,
                    model_id,
                    path,
                    &api_version,
                    use_deployment_based,
                )
            }),
            include_usage: true,
        }
    }

    /// Creates the configuration for embedding models.
    fn create_embedding_config(&self) -> OpenAICompatibleEmbeddingConfig {
        let api_key = self.settings.api_key.clone();
        let custom_headers = self.settings.headers.clone().unwrap_or_default();
        let base_url = self
            .settings
            .get_base_url()
            .expect("Base URL should be validated");
        let api_version = self.settings.api_version.clone();
        let use_deployment_based = self.settings.use_deployment_based_urls;

        OpenAICompatibleEmbeddingConfig {
            provider: "azure.embedding".to_string(),
            headers: Box::new(move || {
                let mut headers = HashMap::new();

                if let Some(ref key) = api_key {
                    headers.insert("api-key".to_string(), key.clone());
                }

                for (key, value) in &custom_headers {
                    headers.insert(key.clone(), value.clone());
                }

                headers
            }),
            url: Box::new(move |model_id: &str, path: &str| {
                Self::build_url(
                    &base_url,
                    model_id,
                    path,
                    &api_version,
                    use_deployment_based,
                )
            }),
            max_embeddings_per_call: None,
            supports_parallel_calls: None,
        }
    }

    /// Creates the configuration for image models.
    fn create_image_config(&self) -> OpenAICompatibleImageModelConfig {
        let api_key = self.settings.api_key.clone();
        let custom_headers = self.settings.headers.clone().unwrap_or_default();
        let base_url = self
            .settings
            .get_base_url()
            .expect("Base URL should be validated");
        let api_version = self.settings.api_version.clone();
        let use_deployment_based = self.settings.use_deployment_based_urls;

        OpenAICompatibleImageModelConfig {
            provider: "azure.image".to_string(),
            headers: Box::new(move || {
                let mut headers = HashMap::new();

                if let Some(ref key) = api_key {
                    headers.insert("api-key".to_string(), key.clone());
                }

                for (key, value) in &custom_headers {
                    headers.insert(key.clone(), value.clone());
                }

                headers
            }),
            url: Box::new(move |model_id: &str, path: &str| {
                Self::build_url(
                    &base_url,
                    model_id,
                    path,
                    &api_version,
                    use_deployment_based,
                )
            }),
        }
    }

    /// Gets the provider name.
    pub fn name(&self) -> &str {
        "azure"
    }
}

// Implement the Provider trait
impl Provider for AzureOpenAIProvider {
    fn language_model(&self, deployment_id: &str) -> Result<Arc<dyn LanguageModel>, ProviderError> {
        Ok(self.chat_model(deployment_id))
    }

    fn text_embedding_model(
        &self,
        deployment_id: &str,
    ) -> Result<Arc<dyn EmbeddingModel<String>>, ProviderError> {
        Ok(self.text_embedding_model(deployment_id))
    }

    fn image_model(&self, deployment_id: &str) -> Result<Arc<dyn ImageModel>, ProviderError> {
        Ok(self.image_model(deployment_id))
    }

    fn transcription_model(
        &self,
        deployment_id: &str,
    ) -> Result<Arc<dyn ai_sdk_provider::TranscriptionModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            deployment_id,
            "azure.transcription-model-not-supported",
        ))
    }

    fn speech_model(
        &self,
        deployment_id: &str,
    ) -> Result<Arc<dyn ai_sdk_provider::SpeechModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            deployment_id,
            "azure.speech-model-not-supported",
        ))
    }

    fn reranking_model(
        &self,
        deployment_id: &str,
    ) -> Result<Arc<dyn ai_sdk_provider::RerankingModel>, ProviderError> {
        Err(ProviderError::no_such_model(
            deployment_id,
            "azure.reranking-model-not-supported",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_provider() -> AzureOpenAIProvider {
        AzureOpenAIProvider::new(
            AzureOpenAIProviderSettings::new()
                .with_resource_name("test-resource")
                .with_api_key("test-key"),
        )
    }

    #[test]
    fn test_create_azure_provider() {
        let provider = create_test_provider();
        assert_eq!(provider.name(), "azure");
    }

    #[test]
    fn test_chat_model() {
        let provider = create_test_provider();
        let model = provider.chat_model("gpt-4-deployment");
        assert_eq!(model.provider(), "azure.chat");
        assert_eq!(model.model_id(), "gpt-4-deployment");
    }

    #[test]
    fn test_completion_model() {
        let provider = create_test_provider();
        let model = provider.completion_model("gpt-35-turbo-instruct");
        assert_eq!(model.provider(), "azure.completion");
        assert_eq!(model.model_id(), "gpt-35-turbo-instruct");
    }

    #[test]
    fn test_text_embedding_model() {
        let provider = create_test_provider();
        let model = provider.text_embedding_model("text-embedding-ada-002");
        assert_eq!(model.provider(), "azure.embedding");
        assert_eq!(model.model_id(), "text-embedding-ada-002");
    }

    #[test]
    fn test_image_model() {
        let provider = create_test_provider();
        let model = provider.image_model("dall-e-3");
        assert_eq!(model.provider(), "azure.image");
        assert_eq!(model.model_id(), "dall-e-3");
    }

    #[test]
    fn test_model_alias() {
        let provider = create_test_provider();
        let model = provider.model("gpt-4-deployment");
        assert_eq!(model.provider(), "azure.chat");
        assert_eq!(model.model_id(), "gpt-4-deployment");
    }

    #[test]
    fn test_provider_trait_implementation() {
        let provider = create_test_provider();
        let provider_trait: &dyn Provider = &provider;

        // Test language model
        let model = provider_trait.language_model("gpt-4-deployment").unwrap();
        assert_eq!(model.provider(), "azure.chat");
        assert_eq!(model.model_id(), "gpt-4-deployment");

        // Test text embedding model
        let embedding_model = provider_trait
            .text_embedding_model("text-embedding-ada-002")
            .unwrap();
        assert_eq!(embedding_model.provider(), "azure.embedding");
        assert_eq!(embedding_model.model_id(), "text-embedding-ada-002");

        // Test image model
        let image_model = provider_trait.image_model("dall-e-3").unwrap();
        assert_eq!(image_model.provider(), "azure.image");
        assert_eq!(image_model.model_id(), "dall-e-3");

        // Test unsupported models
        assert!(provider_trait.transcription_model("whisper").is_err());
        assert!(provider_trait.speech_model("tts-1").is_err());
        assert!(provider_trait.reranking_model("rerank-1").is_err());
    }

    #[test]
    fn test_build_url_v1_format() {
        let url = AzureOpenAIProvider::build_url(
            "https://test.openai.azure.com/openai",
            "gpt-4-deployment",
            "/chat/completions",
            "2024-02-15-preview",
            false,
        );

        assert!(url.contains("/v1/chat/completions"));
        assert!(url.contains("api-version=2024-02-15-preview"));
    }

    #[test]
    fn test_build_url_deployment_based_format() {
        let url = AzureOpenAIProvider::build_url(
            "https://test.openai.azure.com/openai",
            "gpt-4-deployment",
            "/chat/completions",
            "2024-02-15-preview",
            true,
        );

        assert!(url.contains("/deployments/gpt-4-deployment/chat/completions"));
        assert!(url.contains("api-version=2024-02-15-preview"));
    }

    #[test]
    fn test_with_base_url() {
        let provider = AzureOpenAIProvider::new(
            AzureOpenAIProviderSettings::new()
                .with_base_url("https://custom.endpoint.com/openai")
                .with_api_key("test-key"),
        );

        let model = provider.chat_model("gpt-4");
        assert_eq!(model.provider(), "azure.chat");
    }

    #[test]
    fn test_with_custom_api_version() {
        let provider = AzureOpenAIProvider::new(
            AzureOpenAIProviderSettings::new()
                .with_resource_name("test-resource")
                .with_api_key("test-key")
                .with_api_version("2023-05-15"),
        );

        let model = provider.chat_model("gpt-4");
        assert_eq!(model.provider(), "azure.chat");
    }

    #[test]
    fn test_with_deployment_based_urls() {
        let provider = AzureOpenAIProvider::new(
            AzureOpenAIProviderSettings::new()
                .with_resource_name("test-resource")
                .with_api_key("test-key")
                .with_use_deployment_based_urls(true),
        );

        let model = provider.chat_model("gpt-4");
        assert_eq!(model.provider(), "azure.chat");
    }

    #[test]
    #[should_panic(expected = "Invalid Azure OpenAI provider settings")]
    fn test_provider_without_url_or_resource_panics() {
        AzureOpenAIProvider::new(AzureOpenAIProviderSettings::new().with_api_key("test-key"));
    }
}
