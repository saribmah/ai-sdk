use ai_sdk_provider::error::ProviderError;
use ai_sdk_provider::language_model::LanguageModel;
use ai_sdk_provider::provider::Provider;
use std::collections::HashMap;
use std::sync::Arc;

use crate::language_model::{create_language_model, OpenAICompatibleChatConfig, UrlOptions};

/// Configuration options for creating an OpenAI-compatible provider.
#[derive(Debug, Clone)]
pub struct OpenAICompatibleProviderSettings {
    /// Base URL for the API calls (e.g., "https://api.openai.com/v1")
    pub base_url: String,

    /// Provider name (e.g., "openai", "azure-openai", "custom")
    pub name: String,

    /// API key for authenticating requests. If specified, adds an `Authorization`
    /// header with the value `Bearer <apiKey>`.
    pub api_key: Option<String>,

    /// Optional organization ID
    pub organization: Option<String>,

    /// Optional project ID
    pub project: Option<String>,

    /// Optional custom headers to include in requests. These will be added to request headers
    /// after any headers potentially added by use of the `api_key` option.
    pub headers: Option<HashMap<String, String>>,

    /// Optional custom URL query parameters to include in request URLs.
    pub query_params: Option<HashMap<String, String>>,
}

/// OpenAI-compatible provider implementation.
///
/// Provides methods to create different types of language models.
pub struct OpenAICompatibleProvider {
    settings: OpenAICompatibleProviderSettings,
}

impl OpenAICompatibleProvider {
    /// Creates a new OpenAI-compatible provider.
    fn new(settings: OpenAICompatibleProviderSettings) -> Self {
        Self { settings }
    }

    /// Creates a language model with the given model ID.
    /// This is an alias for `chat_model`.
    pub fn language_model(&self, model_id: impl Into<String>) -> Box<dyn LanguageModel> {
        self.chat_model(model_id)
    }

    /// Creates a chat language model with the given model ID.
    pub fn chat_model(&self, model_id: impl Into<String>) -> Box<dyn LanguageModel> {
        let model_id = model_id.into();
        let config = self.create_config();

        create_language_model(model_id, config)
    }

    /// Creates the configuration for language models
    fn create_config(&self) -> OpenAICompatibleChatConfig {
        let api_key = self.settings.api_key.clone();
        let custom_headers = self.settings.headers.clone().unwrap_or_default();
        let organization = self.settings.organization.clone();
        let project = self.settings.project.clone();
        let base_url = self.settings.base_url.clone();
        let query_params = self.settings.query_params.clone().unwrap_or_default();

        OpenAICompatibleChatConfig {
            provider: format!("{}.chat", self.settings.name),
            headers: Arc::new(move || {
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
            url: Arc::new(move |opts: &UrlOptions| {
                let mut url = format!("{}{}", base_url, opts.path);

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
            api_key: self.settings.api_key.clone(),
            organization: self.settings.organization.clone(),
            project: self.settings.project.clone(),
            query_params: self.settings.query_params.clone().unwrap_or_default(),
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
    fn language_model(&self, model_id: &str) -> Result<Box<dyn LanguageModel>, ProviderError> {
        Ok(self.chat_model(model_id))
    }
}

impl Default for OpenAICompatibleProviderSettings {
    fn default() -> Self {
        Self {
            base_url: "https://api.openai.com/v1".to_string(),
            name: "openai".to_string(),
            api_key: None,
            organization: None,
            project: None,
            headers: None,
            query_params: None,
        }
    }
}

impl OpenAICompatibleProviderSettings {
    /// Creates a new configuration with the given base URL and name.
    pub fn new(base_url: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            name: name.into(),
            api_key: None,
            organization: None,
            project: None,
            headers: None,
            query_params: None,
        }
    }

    /// Sets the API key.
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the organization ID.
    pub fn with_organization(mut self, organization: impl Into<String>) -> Self {
        self.organization = Some(organization.into());
        self
    }

    /// Sets the project ID.
    pub fn with_project(mut self, project: impl Into<String>) -> Self {
        self.project = Some(project.into());
        self
    }

    /// Sets additional headers.
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Adds a single header.
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let mut headers = self.headers.unwrap_or_default();
        headers.insert(key.into(), value.into());
        self.headers = Some(headers);
        self
    }

    /// Sets query parameters.
    pub fn with_query_params(mut self, query_params: HashMap<String, String>) -> Self {
        self.query_params = Some(query_params);
        self
    }

    /// Adds a single query parameter.
    pub fn with_query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let mut query_params = self.query_params.unwrap_or_default();
        query_params.insert(key.into(), value.into());
        self.query_params = Some(query_params);
        self
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
/// // Create an OpenAI provider and get a model
/// let provider = create_openai_compatible(
///     OpenAICompatibleProviderSettings::new(
///         "https://api.openai.com/v1",
///         "openai"
///     )
///     .with_api_key("your-api-key")
/// );
///
/// let model = provider.chat_model("gpt-4");
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
pub fn create_openai_compatible(settings: OpenAICompatibleProviderSettings) -> OpenAICompatibleProvider {
    OpenAICompatibleProvider::new(settings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_openai_compatible() {
        let settings = OpenAICompatibleProviderSettings::new(
            "https://api.openai.com/v1",
            "openai"
        )
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
            "azure-openai"
        )
        .with_api_key("test-key");

        let provider = create_openai_compatible(settings);
        let model = provider.chat_model("gpt-4");

        assert_eq!(model.provider(), "azure-openai.chat");
        assert_eq!(model.model_id(), "gpt-4");
    }

    #[test]
    fn test_language_model_alias() {
        let settings = OpenAICompatibleProviderSettings::new(
            "https://api.openai.com/v1",
            "openai"
        )
        .with_api_key("test-key");

        let provider = create_openai_compatible(settings);
        let model = provider.language_model("gpt-4");

        assert_eq!(model.provider(), "openai.chat");
        assert_eq!(model.model_id(), "gpt-4");
    }

    #[test]
    fn test_chained_usage() {
        // Test the chained usage pattern similar to Vercel AI SDK
        let model = create_openai_compatible(
            OpenAICompatibleProviderSettings::new(
                "https://api.example.com/v1",
                "example"
            )
            .with_api_key("test-key")
        )
        .chat_model("meta-llama/Llama-3-70b-chat-hf");

        assert_eq!(model.provider(), "example.chat");
        assert_eq!(model.model_id(), "meta-llama/Llama-3-70b-chat-hf");
    }

    #[test]
    fn test_settings_builder() {
        let settings = OpenAICompatibleProviderSettings::new(
            "https://api.example.com/v1",
            "custom"
        )
        .with_api_key("key")
        .with_organization("org")
        .with_project("proj")
        .with_header("X-Custom-Header", "value")
        .with_query_param("version", "2024-01");

        assert_eq!(settings.base_url, "https://api.example.com/v1");
        assert_eq!(settings.name, "custom");
        assert_eq!(settings.api_key, Some("key".to_string()));
        assert_eq!(settings.organization, Some("org".to_string()));
        assert_eq!(settings.project, Some("proj".to_string()));

        let headers = settings.headers.unwrap();
        assert_eq!(headers.get("X-Custom-Header"), Some(&"value".to_string()));

        let query_params = settings.query_params.unwrap();
        assert_eq!(query_params.get("version"), Some(&"2024-01".to_string()));
    }

    #[test]
    fn test_provider_getters() {
        let settings = OpenAICompatibleProviderSettings::new(
            "https://api.example.com/v1",
            "example"
        );

        let provider = create_openai_compatible(settings);
        assert_eq!(provider.name(), "example");
        assert_eq!(provider.base_url(), "https://api.example.com/v1");
    }

    #[test]
    fn test_provider_trait_implementation() {
        let settings = OpenAICompatibleProviderSettings::new(
            "https://api.openai.com/v1",
            "openai"
        )
        .with_api_key("test-key");

        let provider = create_openai_compatible(settings);

        // Use Provider trait method via trait object
        let provider_trait: &dyn Provider = &provider;
        let model = provider_trait.language_model("gpt-4").unwrap();
        assert_eq!(model.provider(), "openai.chat");
        assert_eq!(model.model_id(), "gpt-4");
    }

    #[test]
    fn test_both_apis() {
        let settings = OpenAICompatibleProviderSettings::new(
            "https://api.example.com/v1",
            "example"
        )
        .with_api_key("test-key");

        let provider = create_openai_compatible(settings);

        // Test Vercel-style API (direct struct method)
        let model1 = provider.chat_model("model-1");
        assert_eq!(model1.provider(), "example.chat");
        assert_eq!(model1.model_id(), "model-1");

        // Test Provider trait API (via trait object)
        let provider_trait: &dyn Provider = &provider;
        let model2 = provider_trait.language_model("model-2").unwrap();
        assert_eq!(model2.provider(), "example.chat");
        assert_eq!(model2.model_id(), "model-2");

        // Test language_model as struct method (alias for chat_model)
        let model3 = provider.language_model("model-3");
        assert_eq!(model3.provider(), "example.chat");
        assert_eq!(model3.model_id(), "model-3");
    }

    #[tokio::test]
    async fn test_generate_text_integration() {
        use ai_sdk_core::generate_text;
        use ai_sdk_core::prompt::{Prompt, call_settings::CallSettings};
        use ai_sdk_core::error::AISDKError;

        let settings = OpenAICompatibleProviderSettings::new(
            "https://api.openai.com/v1",
            "openai"
        )
        .with_api_key("test-key");

        let provider = create_openai_compatible(settings);
        let model = provider.chat_model("gpt-4");

        // Create a simple text prompt
        let prompt = Prompt::text("Hello, how are you?");
        let call_settings = CallSettings::default();

        // Call generate_text - this will make an actual HTTP request and fail
        // because we're using a test API key, but it tests the integration
        let result = generate_text(&*model, prompt, call_settings, None, None, None, None, None, None, None).await;

        // Expect a ModelError (either network error or API auth error)
        // The important part is that the integration between generate_text and our provider works
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
