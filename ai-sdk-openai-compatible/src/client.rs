use std::collections::HashMap;

use crate::provider::OpenAICompatibleProvider;
use crate::settings::OpenAICompatibleProviderSettings;

/// Builder for creating an OpenAI-compatible client.
///
/// Provides a fluent API for constructing an `OpenAICompatibleProvider` with various configuration options.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```ignore
/// use ai_sdk_openai_compatible::OpenAICompatibleClient;
///
/// let provider = OpenAICompatibleClient::new()
///     .base_url("https://api.openai.com/v1")
///     .api_key("your-api-key")
///     .build();
///
/// let model = provider.chat_model("gpt-4");
/// ```
///
/// ## Custom Provider
///
/// ```ignore
/// use ai_sdk_openai_compatible::OpenAICompatibleClient;
///
/// let provider = OpenAICompatibleClient::new()
///     .base_url("https://api.together.xyz/v1")
///     .name("together")
///     .api_key("your-api-key")
///     .header("X-Custom-Header", "value")
///     .build();
///
/// let model = provider.chat_model("meta-llama/Llama-3-70b-chat-hf");
/// ```
///
/// ## Azure OpenAI
///
/// ```ignore
/// use ai_sdk_openai_compatible::OpenAICompatibleClient;
///
/// let provider = OpenAICompatibleClient::new()
///     .base_url("https://my-resource.openai.azure.com/openai")
///     .name("azure-openai")
///     .api_key("your-api-key")
///     .query_param("api-version", "2024-02-15-preview")
///     .build();
///
/// let model = provider.chat_model("gpt-4");
/// ```
#[derive(Debug, Clone, Default)]
pub struct OpenAICompatibleClient {
    base_url: Option<String>,
    name: Option<String>,
    api_key: Option<String>,
    organization: Option<String>,
    project: Option<String>,
    headers: HashMap<String, String>,
    query_params: HashMap<String, String>,
    include_usage: bool,
    supports_structured_outputs: bool,
}

impl OpenAICompatibleClient {
    /// Creates a new client builder with default settings.
    ///
    /// The default base URL is `https://api.openai.com/v1` and the default name is `openai`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the base URL for API calls.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL (e.g., "<https://api.openai.com/v1>")
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Sets the provider name.
    ///
    /// # Arguments
    ///
    /// * `name` - The provider name (e.g., "openai", "azure-openai", "together")
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the API key for authentication.
    ///
    /// If specified, adds an `Authorization` header with the value `Bearer <apiKey>`.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the organization ID.
    ///
    /// # Arguments
    ///
    /// * `organization` - The organization ID
    pub fn organization(mut self, organization: impl Into<String>) -> Self {
        self.organization = Some(organization.into());
        self
    }

    /// Sets the project ID.
    ///
    /// # Arguments
    ///
    /// * `project` - The project ID
    pub fn project(mut self, project: impl Into<String>) -> Self {
        self.project = Some(project.into());
        self
    }

    /// Adds a custom header to include in requests.
    ///
    /// Custom headers are added after any headers potentially added by use of the `api_key` option.
    ///
    /// # Arguments
    ///
    /// * `key` - The header name
    /// * `value` - The header value
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Sets multiple custom headers at once.
    ///
    /// # Arguments
    ///
    /// * `headers` - A HashMap of header names to values
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
    }

    /// Adds a custom URL query parameter to include in request URLs.
    ///
    /// # Arguments
    ///
    /// * `key` - The query parameter name
    /// * `value` - The query parameter value
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.insert(key.into(), value.into());
        self
    }

    /// Sets multiple query parameters at once.
    ///
    /// # Arguments
    ///
    /// * `query_params` - A HashMap of query parameter names to values
    pub fn query_params(mut self, query_params: HashMap<String, String>) -> Self {
        self.query_params.extend(query_params);
        self
    }

    /// Sets whether to include usage information in streaming responses.
    ///
    /// # Arguments
    ///
    /// * `include_usage` - Whether to include usage information
    pub fn include_usage(mut self, include_usage: bool) -> Self {
        self.include_usage = include_usage;
        self
    }

    /// Sets whether the provider supports structured outputs in chat models.
    ///
    /// # Arguments
    ///
    /// * `supports` - Whether structured outputs are supported
    pub fn supports_structured_outputs(mut self, supports: bool) -> Self {
        self.supports_structured_outputs = supports;
        self
    }

    /// Builds the `OpenAICompatibleProvider` with the configured settings.
    ///
    /// # Returns
    ///
    /// An `OpenAICompatibleProvider` instance.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use ai_sdk_openai_compatible::OpenAICompatibleClient;
    ///
    /// let provider = OpenAICompatibleClient::new()
    ///     .base_url("https://api.openai.com/v1")
    ///     .api_key("your-api-key")
    ///     .build();
    /// ```
    pub fn build(self) -> OpenAICompatibleProvider {
        let base_url = self
            .base_url
            .unwrap_or_else(|| "https://api.openai.com/v1".to_string());
        let name = self.name.unwrap_or_else(|| "openai".to_string());

        let mut settings = OpenAICompatibleProviderSettings::new(base_url, name);

        if let Some(api_key) = self.api_key {
            settings = settings.with_api_key(api_key);
        }

        if let Some(organization) = self.organization {
            settings = settings.with_organization(organization);
        }

        if let Some(project) = self.project {
            settings = settings.with_project(project);
        }

        if !self.headers.is_empty() {
            settings = settings.with_headers(self.headers);
        }

        if !self.query_params.is_empty() {
            settings = settings.with_query_params(self.query_params);
        }

        settings = settings.with_include_usage(self.include_usage);
        settings = settings.with_supports_structured_outputs(self.supports_structured_outputs);

        OpenAICompatibleProvider::new(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_client_builder() {
        let provider = OpenAICompatibleClient::new()
            .base_url("https://api.openai.com/v1")
            .api_key("test-key")
            .build();

        assert_eq!(provider.name(), "openai");
        assert_eq!(provider.base_url(), "https://api.openai.com/v1");
    }

    #[test]
    fn test_custom_provider() {
        let provider = OpenAICompatibleClient::new()
            .base_url("https://api.together.xyz/v1")
            .name("together")
            .api_key("test-key")
            .build();

        assert_eq!(provider.name(), "together");
        assert_eq!(provider.base_url(), "https://api.together.xyz/v1");
    }

    #[test]
    fn test_with_headers_and_query_params() {
        let provider = OpenAICompatibleClient::new()
            .base_url("https://api.example.com/v1")
            .name("example")
            .api_key("test-key")
            .header("X-Custom-Header", "value1")
            .header("X-Another-Header", "value2")
            .query_param("api-version", "2024-01")
            .query_param("model-version", "v2")
            .build();

        assert_eq!(provider.name(), "example");
        assert_eq!(provider.base_url(), "https://api.example.com/v1");
    }

    #[test]
    fn test_with_organization_and_project() {
        let provider = OpenAICompatibleClient::new()
            .api_key("test-key")
            .organization("test-org")
            .project("test-project")
            .build();

        assert_eq!(provider.name(), "openai");
        assert_eq!(provider.base_url(), "https://api.openai.com/v1");
    }

    #[test]
    fn test_with_usage_and_structured_outputs() {
        let provider = OpenAICompatibleClient::new()
            .api_key("test-key")
            .include_usage(true)
            .supports_structured_outputs(true)
            .build();

        assert_eq!(provider.name(), "openai");
    }

    #[test]
    fn test_default_values() {
        let provider = OpenAICompatibleClient::new().build();

        assert_eq!(provider.name(), "openai");
        assert_eq!(provider.base_url(), "https://api.openai.com/v1");
    }

    #[test]
    fn test_chained_model_creation() {
        let model = OpenAICompatibleClient::new()
            .base_url("https://api.openai.com/v1")
            .api_key("test-key")
            .build()
            .chat_model("gpt-4");

        assert_eq!(model.model_id(), "gpt-4");
        assert_eq!(model.provider(), "openai.chat");
    }

    #[test]
    fn test_azure_openai_pattern() {
        let provider = OpenAICompatibleClient::new()
            .base_url("https://my-resource.openai.azure.com/openai")
            .name("azure-openai")
            .api_key("test-key")
            .query_param("api-version", "2024-02-15-preview")
            .build();

        assert_eq!(provider.name(), "azure-openai");
        assert_eq!(
            provider.base_url(),
            "https://my-resource.openai.azure.com/openai"
        );
    }

    #[test]
    fn test_bulk_headers() {
        let mut headers = HashMap::new();
        headers.insert("X-Header-1".to_string(), "value1".to_string());
        headers.insert("X-Header-2".to_string(), "value2".to_string());

        let provider = OpenAICompatibleClient::new()
            .api_key("test-key")
            .headers(headers)
            .build();

        assert_eq!(provider.name(), "openai");
    }

    #[test]
    fn test_bulk_query_params() {
        let mut query_params = HashMap::new();
        query_params.insert("api-version".to_string(), "2024-01".to_string());
        query_params.insert("model-version".to_string(), "v2".to_string());

        let provider = OpenAICompatibleClient::new()
            .api_key("test-key")
            .query_params(query_params)
            .build();

        assert_eq!(provider.name(), "openai");
    }

    #[test]
    fn test_mixed_headers_methods() {
        let mut initial_headers = HashMap::new();
        initial_headers.insert("X-Initial".to_string(), "value".to_string());

        let provider = OpenAICompatibleClient::new()
            .api_key("test-key")
            .headers(initial_headers)
            .header("X-Additional", "another-value")
            .build();

        assert_eq!(provider.name(), "openai");
    }
}
