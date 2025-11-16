use std::collections::HashMap;

use crate::openai_provider::OpenAIProvider;
use crate::settings::OpenAIProviderSettings;

/// Builder for creating an OpenAI client.
///
/// Provides a fluent API for constructing an `OpenAIProvider` with various configuration options.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```no_run
/// use ai_sdk_openai::OpenAIClient;
///
/// let provider = OpenAIClient::new()
///     .api_key("your-api-key")
///     .build();
///
/// let model = provider.chat("gpt-4");
/// ```
///
/// ## Custom Base URL
///
/// ```no_run
/// use ai_sdk_openai::OpenAIClient;
///
/// let provider = OpenAIClient::new()
///     .base_url("https://api.openai.com/v1")
///     .api_key("your-api-key")
///     .build();
///
/// let model = provider.chat("gpt-4");
/// ```
///
/// ## With Organization and Project
///
/// ```no_run
/// use ai_sdk_openai::OpenAIClient;
///
/// let provider = OpenAIClient::new()
///     .api_key("your-api-key")
///     .organization("org-123")
///     .project("proj-456")
///     .build();
///
/// let model = provider.chat("gpt-4");
/// ```
///
/// ## With Custom Headers
///
/// ```no_run
/// use ai_sdk_openai::OpenAIClient;
///
/// let provider = OpenAIClient::new()
///     .api_key("your-api-key")
///     .header("X-Custom-Header", "value")
///     .build();
///
/// let model = provider.chat("gpt-4");
/// ```
#[derive(Debug, Clone, Default)]
pub struct OpenAIClient {
    base_url: Option<String>,
    api_key: Option<String>,
    organization: Option<String>,
    project: Option<String>,
    headers: HashMap<String, String>,
    name: Option<String>,
}

impl OpenAIClient {
    /// Creates a new client builder with default settings.
    ///
    /// The default base URL is `https://api.openai.com/v1` and the default name is `openai`.
    /// If no API key is provided, it will attempt to use the `OPENAI_API_KEY` environment variable.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the base URL for API calls.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL (e.g., "https://api.openai.com/v1")
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ai_sdk_openai::OpenAIClient;
    ///
    /// let client = OpenAIClient::new()
    ///     .base_url("https://api.openai.com/v1");
    /// ```
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Sets the API key for authentication.
    ///
    /// If not specified, the client will attempt to use the `OPENAI_API_KEY` environment variable.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ai_sdk_openai::OpenAIClient;
    ///
    /// let client = OpenAIClient::new()
    ///     .api_key("your-api-key");
    /// ```
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the OpenAI organization ID.
    ///
    /// # Arguments
    ///
    /// * `organization` - The organization ID
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ai_sdk_openai::OpenAIClient;
    ///
    /// let client = OpenAIClient::new()
    ///     .organization("org-123");
    /// ```
    pub fn organization(mut self, organization: impl Into<String>) -> Self {
        self.organization = Some(organization.into());
        self
    }

    /// Sets the OpenAI project ID.
    ///
    /// # Arguments
    ///
    /// * `project` - The project ID
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ai_sdk_openai::OpenAIClient;
    ///
    /// let client = OpenAIClient::new()
    ///     .project("proj-456");
    /// ```
    pub fn project(mut self, project: impl Into<String>) -> Self {
        self.project = Some(project.into());
        self
    }

    /// Sets the provider name.
    ///
    /// # Arguments
    ///
    /// * `name` - The provider name (e.g., "openai", "my-openai-provider")
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ai_sdk_openai::OpenAIClient;
    ///
    /// let client = OpenAIClient::new()
    ///     .name("my-openai-provider");
    /// ```
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Adds a custom header to include in requests.
    ///
    /// Custom headers are added after the required OpenAI headers (`Authorization`, `OpenAI-Organization`, etc.).
    ///
    /// # Arguments
    ///
    /// * `key` - The header name
    /// * `value` - The header value
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ai_sdk_openai::OpenAIClient;
    ///
    /// let client = OpenAIClient::new()
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
    /// use ai_sdk_openai::OpenAIClient;
    /// use std::collections::HashMap;
    ///
    /// let mut headers = HashMap::new();
    /// headers.insert("X-Header-1".to_string(), "value1".to_string());
    /// headers.insert("X-Header-2".to_string(), "value2".to_string());
    ///
    /// let client = OpenAIClient::new()
    ///     .headers(headers);
    /// ```
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
    }

    /// Builds the `OpenAIProvider` with the configured settings.
    ///
    /// # Returns
    ///
    /// An `OpenAIProvider` instance.
    ///
    /// # Panics
    ///
    /// Panics if no API key is provided and the `OPENAI_API_KEY` environment variable is not set.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ai_sdk_openai::OpenAIClient;
    ///
    /// let provider = OpenAIClient::new()
    ///     .api_key("your-api-key")
    ///     .build();
    /// ```
    pub fn build(self) -> OpenAIProvider {
        let mut settings = OpenAIProviderSettings::new();

        if let Some(base_url) = self.base_url {
            settings = settings.with_base_url(base_url);
        }

        if let Some(api_key) = self.api_key {
            settings = settings.with_api_key(api_key);
        }

        if let Some(organization) = self.organization {
            settings = settings.with_organization(organization);
        }

        if let Some(project) = self.project {
            settings = settings.with_project(project);
        }

        if let Some(name) = self.name {
            settings = settings.with_name(name);
        }

        if !self.headers.is_empty() {
            settings = settings.with_headers(self.headers);
        }

        OpenAIProvider::new(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_sdk_provider::language_model::LanguageModel;

    #[test]
    fn test_basic_client_builder() {
        let provider = OpenAIClient::new().api_key("test-key").build();

        // Default base URL should be set
        assert_eq!(provider.base_url(), "https://api.openai.com/v1");
        assert_eq!(provider.provider_name(), "openai");
    }

    #[test]
    fn test_custom_base_url() {
        let provider = OpenAIClient::new()
            .base_url("https://custom.api.com/v1")
            .api_key("test-key")
            .build();

        assert_eq!(provider.base_url(), "https://custom.api.com/v1");
    }

    #[test]
    fn test_custom_name() {
        let provider = OpenAIClient::new()
            .name("my-openai")
            .api_key("test-key")
            .build();

        assert_eq!(provider.provider_name(), "my-openai");
    }

    #[test]
    fn test_with_headers() {
        let provider = OpenAIClient::new()
            .api_key("test-key")
            .header("X-Custom-Header", "value1")
            .header("X-Another-Header", "value2")
            .build();

        // Verify the provider was created successfully
        assert_eq!(provider.base_url(), "https://api.openai.com/v1");
        assert_eq!(provider.provider_name(), "openai");
    }

    #[test]
    fn test_bulk_headers() {
        let mut custom_headers = HashMap::new();
        custom_headers.insert("X-Header-1".to_string(), "value1".to_string());
        custom_headers.insert("X-Header-2".to_string(), "value2".to_string());

        let provider = OpenAIClient::new()
            .api_key("test-key")
            .headers(custom_headers)
            .build();

        assert_eq!(provider.base_url(), "https://api.openai.com/v1");
    }

    #[test]
    fn test_mixed_headers_methods() {
        let mut initial_headers = HashMap::new();
        initial_headers.insert("X-Initial".to_string(), "value".to_string());

        let provider = OpenAIClient::new()
            .api_key("test-key")
            .headers(initial_headers)
            .header("X-Additional", "another-value")
            .build();

        assert_eq!(provider.base_url(), "https://api.openai.com/v1");
    }

    #[test]
    fn test_organization_and_project() {
        let provider = OpenAIClient::new()
            .api_key("test-key")
            .organization("org-123")
            .project("proj-456")
            .build();

        assert_eq!(provider.base_url(), "https://api.openai.com/v1");
    }

    #[test]
    fn test_chained_model_creation() {
        let model = OpenAIClient::new()
            .api_key("test-key")
            .build()
            .chat("gpt-4");

        assert_eq!(model.model_id(), "gpt-4");
        assert_eq!(model.provider(), "openai.chat");
    }

    #[test]
    fn test_default_values() {
        let provider = OpenAIClient::new().api_key("test-key").build();

        assert_eq!(provider.base_url(), "https://api.openai.com/v1");
        assert_eq!(provider.provider_name(), "openai");
    }

    #[test]
    fn test_trailing_slash_removed() {
        let provider = OpenAIClient::new()
            .base_url("https://api.openai.com/v1/")
            .api_key("test-key")
            .build();

        assert_eq!(provider.base_url(), "https://api.openai.com/v1");
    }
}
