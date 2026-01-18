use std::collections::HashMap;

use crate::provider::AzureOpenAIProvider;
use crate::settings::AzureOpenAIProviderSettings;

/// Builder for creating an Azure OpenAI provider.
///
/// Provides a fluent API for constructing an `AzureOpenAIProvider` with various configuration options.
///
/// # Examples
///
/// ## Basic Usage with Resource Name
///
/// ```no_run
/// use llm_kit_azure::AzureClient;
///
/// let provider = AzureClient::new()
///     .resource_name("my-azure-resource")
///     .api_key("your-api-key")
///     .build();
///
/// let model = provider.chat_model("gpt-4-deployment");
/// ```
///
/// ## Custom Base URL
///
/// ```no_run
/// use llm_kit_azure::AzureClient;
///
/// let provider = AzureClient::new()
///     .base_url("https://my-resource.openai.azure.com/openai")
///     .api_key("your-api-key")
///     .build();
///
/// let model = provider.chat_model("gpt-4-deployment");
/// ```
///
/// ## With Custom Headers and API Version
///
/// ```no_run
/// use llm_kit_azure::AzureClient;
///
/// let provider = AzureClient::new()
///     .resource_name("my-resource")
///     .api_key("your-api-key")
///     .api_version("2024-02-15-preview")
///     .header("X-Custom-Header", "value")
///     .build();
///
/// let model = provider.chat_model("gpt-4-deployment");
/// ```
///
/// ## Deployment-Based URLs (Legacy Format)
///
/// ```no_run
/// use llm_kit_azure::AzureClient;
///
/// let provider = AzureClient::new()
///     .resource_name("my-resource")
///     .api_key("your-api-key")
///     .use_deployment_based_urls(true)
///     .build();
///
/// let model = provider.chat_model("gpt-4-deployment");
/// ```
#[derive(Debug, Clone, Default)]
pub struct AzureClient {
    resource_name: Option<String>,
    base_url: Option<String>,
    api_key: Option<String>,
    headers: HashMap<String, String>,
    api_version: Option<String>,
    use_deployment_based_urls: bool,
}

impl AzureClient {
    /// Creates a new client builder with default settings.
    ///
    /// The default API version is "v1" and uses the v1 API URL format.
    /// If no API key is provided, it will attempt to use the `AZURE_API_KEY` environment variable.
    /// If no resource name or base URL is provided, it will attempt to use the `AZURE_RESOURCE_NAME`
    /// or `AZURE_BASE_URL` environment variables.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the Azure OpenAI resource name.
    ///
    /// The resource name is used to construct the base URL:
    /// `https://{resource_name}.openai.azure.com/openai`
    ///
    /// # Arguments
    ///
    /// * `resource_name` - The Azure OpenAI resource name
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use llm_kit_azure::AzureClient;
    ///
    /// let client = AzureClient::new()
    ///     .resource_name("my-azure-resource");
    /// ```
    pub fn resource_name(mut self, resource_name: impl Into<String>) -> Self {
        self.resource_name = Some(resource_name.into());
        self
    }

    /// Sets a custom base URL for API calls.
    ///
    /// When set, this takes precedence over `resource_name`.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL (e.g., "<https://my-resource.openai.azure.com/openai>")
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use llm_kit_azure::AzureClient;
    ///
    /// let client = AzureClient::new()
    ///     .base_url("https://custom.endpoint.com/openai");
    /// ```
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Sets the API key for authentication.
    ///
    /// If not specified, the client will attempt to use the `AZURE_API_KEY` environment variable.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use llm_kit_azure::AzureClient;
    ///
    /// let client = AzureClient::new()
    ///     .api_key("your-api-key");
    /// ```
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Adds a custom header to include in requests.
    ///
    /// # Arguments
    ///
    /// * `key` - The header name
    /// * `value` - The header value
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use llm_kit_azure::AzureClient;
    ///
    /// let client = AzureClient::new()
    ///     .header("X-Custom-Header", "value");
    /// ```
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Sets multiple custom headers at once.
    ///
    /// # Arguments
    ///
    /// * `headers` - A HashMap of header names to values
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use llm_kit_azure::AzureClient;
    /// use std::collections::HashMap;
    ///
    /// let mut headers = HashMap::new();
    /// headers.insert("X-Custom-1".to_string(), "value1".to_string());
    /// headers.insert("X-Custom-2".to_string(), "value2".to_string());
    ///
    /// let client = AzureClient::new()
    ///     .headers(headers);
    /// ```
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
    }

    /// Sets the API version to use in requests.
    ///
    /// Azure OpenAI requires an API version parameter in all requests.
    /// Defaults to "v1" if not specified.
    ///
    /// Common versions include:
    /// - "2023-05-15"
    /// - "2024-02-15-preview"
    /// - "v1" (default)
    ///
    /// # Arguments
    ///
    /// * `api_version` - The API version string
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use llm_kit_azure::AzureClient;
    ///
    /// let client = AzureClient::new()
    ///     .api_version("2024-02-15-preview");
    /// ```
    pub fn api_version(mut self, api_version: impl Into<String>) -> Self {
        self.api_version = Some(api_version.into());
        self
    }

    /// Sets whether to use deployment-based URLs.
    ///
    /// When `true`, uses legacy format:
    /// `{base_url}/deployments/{deployment_id}{path}?api-version={version}`
    ///
    /// When `false` (default), uses v1 API format:
    /// `{base_url}/v1{path}?api-version={version}`
    ///
    /// # Arguments
    ///
    /// * `use_deployment_based_urls` - Whether to use deployment-based URLs
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use llm_kit_azure::AzureClient;
    ///
    /// let client = AzureClient::new()
    ///     .use_deployment_based_urls(true);
    /// ```
    pub fn use_deployment_based_urls(mut self, use_deployment_based_urls: bool) -> Self {
        self.use_deployment_based_urls = use_deployment_based_urls;
        self
    }

    /// Builds the Azure OpenAI provider.
    ///
    /// This method constructs the provider settings from the builder configuration
    /// and creates an `AzureOpenAIProvider` instance.
    ///
    /// # Panics
    ///
    /// Panics if neither resource name nor base URL is provided (either via builder or environment variables).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use llm_kit_azure::AzureClient;
    ///
    /// let provider = AzureClient::new()
    ///     .resource_name("my-resource")
    ///     .api_key("your-api-key")
    ///     .build();
    /// ```
    pub fn build(self) -> AzureOpenAIProvider {
        let mut settings = AzureOpenAIProviderSettings::new();

        // Set resource name or base URL (prefer explicit values, fall back to env vars)
        if let Some(resource_name) = self.resource_name {
            settings = settings.with_resource_name(resource_name);
        } else if let Some(base_url) = self.base_url {
            settings = settings.with_base_url(base_url);
        } else {
            // Try environment variables
            if let Ok(resource_name) = std::env::var("AZURE_RESOURCE_NAME") {
                settings = settings.with_resource_name(resource_name);
            } else if let Ok(base_url) = std::env::var("AZURE_BASE_URL") {
                settings = settings.with_base_url(base_url);
            }
        }

        // Set API key (prefer explicit value, fall back to env var)
        if let Some(api_key) = self.api_key {
            settings = settings.with_api_key(api_key);
        } else if let Ok(api_key) = std::env::var("AZURE_API_KEY") {
            settings = settings.with_api_key(api_key);
        }

        // Set custom headers if provided
        if !self.headers.is_empty() {
            settings = settings.with_headers(self.headers);
        }

        // Set API version if provided
        if let Some(api_version) = self.api_version {
            settings = settings.with_api_version(api_version);
        }

        // Set deployment-based URLs flag
        settings = settings.with_use_deployment_based_urls(self.use_deployment_based_urls);

        AzureOpenAIProvider::new(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_with_resource_name() {
        let provider = AzureClient::new()
            .resource_name("test-resource")
            .api_key("test-key")
            .build();

        assert_eq!(provider.name(), "azure");
    }

    #[test]
    fn test_builder_with_base_url() {
        let provider = AzureClient::new()
            .base_url("https://custom.endpoint.com/openai")
            .api_key("test-key")
            .build();

        assert_eq!(provider.name(), "azure");
    }

    #[test]
    fn test_builder_with_api_version() {
        let provider = AzureClient::new()
            .resource_name("test-resource")
            .api_key("test-key")
            .api_version("2024-02-15-preview")
            .build();

        assert_eq!(provider.name(), "azure");
    }

    #[test]
    fn test_builder_with_headers() {
        let provider = AzureClient::new()
            .resource_name("test-resource")
            .api_key("test-key")
            .header("X-Custom-1", "value1")
            .header("X-Custom-2", "value2")
            .build();

        assert_eq!(provider.name(), "azure");
    }

    #[test]
    fn test_builder_with_multiple_headers() {
        let mut headers = HashMap::new();
        headers.insert("X-Custom-1".to_string(), "value1".to_string());
        headers.insert("X-Custom-2".to_string(), "value2".to_string());

        let provider = AzureClient::new()
            .resource_name("test-resource")
            .api_key("test-key")
            .headers(headers)
            .build();

        assert_eq!(provider.name(), "azure");
    }

    #[test]
    fn test_builder_with_deployment_based_urls() {
        let provider = AzureClient::new()
            .resource_name("test-resource")
            .api_key("test-key")
            .use_deployment_based_urls(true)
            .build();

        assert_eq!(provider.name(), "azure");
    }

    #[test]
    fn test_builder_full_configuration() {
        let provider = AzureClient::new()
            .resource_name("test-resource")
            .api_key("test-key")
            .api_version("2024-02-15-preview")
            .header("X-Custom", "value")
            .use_deployment_based_urls(true)
            .build();

        assert_eq!(provider.name(), "azure");
    }

    #[test]
    fn test_builder_creates_working_models() {
        let provider = AzureClient::new()
            .resource_name("test-resource")
            .api_key("test-key")
            .build();

        // Test chat model
        let chat_model = provider.chat_model("gpt-4");
        assert_eq!(chat_model.provider(), "azure.chat");
        assert_eq!(chat_model.model_id(), "gpt-4");

        // Test completion model
        let completion_model = provider.completion_model("gpt-35-turbo-instruct");
        assert_eq!(completion_model.provider(), "azure.completion");
        assert_eq!(completion_model.model_id(), "gpt-35-turbo-instruct");

        // Test embedding model
        let embedding_model = provider.text_embedding_model("text-embedding-ada-002");
        assert_eq!(embedding_model.provider(), "azure.embedding");
        assert_eq!(embedding_model.model_id(), "text-embedding-ada-002");

        // Test image model
        let image_model = provider.image_model("dall-e-3");
        assert_eq!(image_model.provider(), "azure.image");
        assert_eq!(image_model.model_id(), "dall-e-3");
    }

    #[test]
    #[should_panic(expected = "Invalid Azure OpenAI provider settings")]
    fn test_builder_without_url_or_resource_panics() {
        AzureClient::new().api_key("test-key").build();
    }
}
