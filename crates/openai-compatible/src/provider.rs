use ai_sdk_provider::error::ProviderError;
use ai_sdk_provider::language_model::LanguageModel;
use ai_sdk_provider::provider::Provider;
use std::collections::HashMap;

use crate::language_model::{create_language_model, OpenAICompatibleLanguageModelConfig};

/// OpenAI-compatible provider implementation.
pub struct OpenAICompatibleProvider {
    /// The provider identifier (e.g., "openai", "azure-openai")
    provider_id: String,

    /// Configuration for language models
    config: OpenAICompatibleLanguageModelConfig,
}

impl OpenAICompatibleProvider {
    /// Creates a new OpenAI-compatible provider.
    pub fn new(provider_id: String, config: OpenAICompatibleLanguageModelConfig) -> Self {
        Self {
            provider_id,
            config,
        }
    }
}

impl Provider for OpenAICompatibleProvider {
    fn language_model(&self, model_id: &str) -> Result<Box<dyn LanguageModel>, ProviderError> {
        // Create a language model with the given model ID
        Ok(create_language_model(model_id, self.config.clone()))
    }
}

/// Configuration options for creating an OpenAI-compatible provider.
#[derive(Debug, Clone)]
pub struct OpenAICompatibleConfig {
    /// The base URL for the API (e.g., "https://api.openai.com/v1")
    pub base_url: String,

    /// Optional API key for authentication
    pub api_key: Option<String>,

    /// Optional organization ID
    pub organization: Option<String>,

    /// Optional project ID
    pub project: Option<String>,

    /// Additional headers to send with requests
    pub headers: Option<HashMap<String, String>>,
}

impl Default for OpenAICompatibleConfig {
    fn default() -> Self {
        Self {
            base_url: "https://api.openai.com/v1".to_string(),
            api_key: None,
            organization: None,
            project: None,
            headers: None,
        }
    }
}

impl OpenAICompatibleConfig {
    /// Creates a new configuration with the given base URL.
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            ..Default::default()
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
}

/// Creates an OpenAI-compatible provider.
///
/// This function creates a provider that can be used with OpenAI-compatible APIs,
/// including OpenAI, Azure OpenAI, and other compatible services.
///
/// # Arguments
///
/// * `config` - Configuration for the provider
///
/// # Returns
///
/// A boxed `Provider` trait object.
///
/// # Examples
///
/// ```ignore
/// use openai_compatible::{create_openai_compatible, OpenAICompatibleConfig};
///
/// // Create an OpenAI provider
/// let config = OpenAICompatibleConfig::default()
///     .with_api_key("your-api-key");
/// let provider = create_openai_compatible(config);
///
/// // Create an Azure OpenAI provider
/// let azure_config = OpenAICompatibleConfig::new("https://your-resource.openai.azure.com/openai")
///     .with_api_key("your-api-key");
/// let azure_provider = create_openai_compatible(azure_config);
/// ```
pub fn create_openai_compatible(config: OpenAICompatibleConfig) -> Box<dyn Provider> {
    let provider_id = if config.base_url.contains("azure") {
        "azure-openai"
    } else {
        "openai"
    };

    let language_model_config = OpenAICompatibleLanguageModelConfig {
        provider: provider_id.to_string(),
        base_url: config.base_url,
        model_id: String::new(), // Will be set when creating individual models
        api_key: config.api_key,
        organization: config.organization,
        project: config.project,
        headers: config.headers.unwrap_or_default(),
    };

    Box::new(OpenAICompatibleProvider::new(
        provider_id.to_string(),
        language_model_config,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_openai_compatible() {
        let config = OpenAICompatibleConfig::default()
            .with_api_key("test-key")
            .with_organization("test-org");

        let provider = create_openai_compatible(config);
        let model = provider.language_model("gpt-4").unwrap();

        assert_eq!(model.provider(), "openai");
        assert_eq!(model.model_id(), "gpt-4");
    }

    #[test]
    fn test_create_azure_openai_compatible() {
        let config = OpenAICompatibleConfig::new("https://my-resource.openai.azure.com/openai")
            .with_api_key("test-key");

        let provider = create_openai_compatible(config);
        let model = provider.language_model("gpt-4").unwrap();

        assert_eq!(model.provider(), "azure-openai");
        assert_eq!(model.model_id(), "gpt-4");
    }

    #[test]
    fn test_config_builder() {
        let config = OpenAICompatibleConfig::new("https://api.example.com")
            .with_api_key("key")
            .with_organization("org")
            .with_project("proj")
            .with_header("X-Custom-Header", "value");

        assert_eq!(config.base_url, "https://api.example.com");
        assert_eq!(config.api_key, Some("key".to_string()));
        assert_eq!(config.organization, Some("org".to_string()));
        assert_eq!(config.project, Some("proj".to_string()));
        assert!(config.headers.is_some());
        assert_eq!(
            config.headers.unwrap().get("X-Custom-Header"),
            Some(&"value".to_string())
        );
    }
}
