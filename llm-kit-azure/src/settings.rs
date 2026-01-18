use std::collections::HashMap;

/// Configuration options for creating an Azure OpenAI provider.
///
/// Azure OpenAI has unique authentication and URL patterns compared to standard OpenAI:
/// - Uses `api-key` header instead of `Authorization: Bearer`
/// - Requires API version in query parameters
/// - Supports both deployment-based and v1 API URL formats
/// - Can use either `resourceName` or custom `baseURL`
#[derive(Debug, Clone)]
pub struct AzureOpenAIProviderSettings {
    /// Name of the Azure OpenAI resource. Either this or `base_url` can be used.
    ///
    /// The resource name is used in the assembled URL:
    /// `https://{resource_name}.openai.azure.com/openai/v1{path}`
    pub resource_name: Option<String>,

    /// Use a different URL prefix for API calls, e.g. to use proxy servers.
    /// Either this or `resource_name` can be used.
    /// When a `base_url` is provided, the `resource_name` is ignored.
    ///
    /// With a `base_url`, the resolved URL is `{base_url}/v1{path}`.
    pub base_url: Option<String>,

    /// API key for authenticating requests.
    /// If not provided, will attempt to read from `AZURE_API_KEY` environment variable.
    pub api_key: Option<String>,

    /// Custom headers to include in the requests.
    pub headers: Option<HashMap<String, String>>,

    /// Custom API version to use. Defaults to "v1".
    ///
    /// Azure OpenAI requires an API version parameter in all requests.
    /// Common versions include "2023-05-15", "2024-02-15-preview", etc.
    pub api_version: String,

    /// Use deployment-based URLs for specific model types.
    ///
    /// Set to true to use legacy deployment format:
    /// `{base_url}/deployments/{deployment_id}{path}?api-version={api_version}`
    ///
    /// Set to false (default) to use v1 API format:
    /// `{base_url}/v1{path}?api-version={api_version}`
    pub use_deployment_based_urls: bool,
}

impl Default for AzureOpenAIProviderSettings {
    fn default() -> Self {
        Self {
            resource_name: None,
            base_url: None,
            api_key: None,
            headers: None,
            api_version: "v1".to_string(),
            use_deployment_based_urls: false,
        }
    }
}

impl AzureOpenAIProviderSettings {
    /// Creates a new Azure OpenAI provider configuration.
    ///
    /// You must provide either a `resource_name` or `base_url` before using the provider.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the Azure OpenAI resource name.
    ///
    /// The resource name is used to construct the base URL:
    /// `https://{resource_name}.openai.azure.com/openai`
    pub fn with_resource_name(mut self, resource_name: impl Into<String>) -> Self {
        self.resource_name = Some(resource_name.into());
        self
    }

    /// Sets a custom base URL for API calls.
    ///
    /// When set, this takes precedence over `resource_name`.
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Sets the API key for authentication.
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets additional headers to include in requests.
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Adds a single header to include in requests.
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let mut headers = self.headers.unwrap_or_default();
        headers.insert(key.into(), value.into());
        self.headers = Some(headers);
        self
    }

    /// Sets the API version to use in requests.
    ///
    /// Defaults to "v1". Common versions include "2023-05-15", "2024-02-15-preview", etc.
    pub fn with_api_version(mut self, api_version: impl Into<String>) -> Self {
        self.api_version = api_version.into();
        self
    }

    /// Sets whether to use deployment-based URLs.
    ///
    /// When true, uses format: `{base_url}/deployments/{deployment_id}{path}`
    /// When false (default), uses format: `{base_url}/v1{path}`
    pub fn with_use_deployment_based_urls(mut self, use_deployment_based_urls: bool) -> Self {
        self.use_deployment_based_urls = use_deployment_based_urls;
        self
    }

    /// Gets the base URL prefix for API calls.
    ///
    /// Returns the custom `base_url` if set, otherwise constructs from `resource_name`.
    pub(crate) fn get_base_url(&self) -> Option<String> {
        if let Some(ref base_url) = self.base_url {
            Some(base_url.clone())
        } else {
            self.resource_name
                .as_ref()
                .map(|resource_name| format!("https://{}.openai.azure.com/openai", resource_name))
        }
    }

    /// Validates the settings to ensure required fields are present.
    pub(crate) fn validate(&self) -> Result<(), String> {
        if self.base_url.is_none() && self.resource_name.is_none() {
            return Err(
                "Either base_url or resource_name must be provided for Azure OpenAI".to_string(),
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = AzureOpenAIProviderSettings::new();
        assert_eq!(settings.api_version, "v1");
        assert!(!settings.use_deployment_based_urls);
        assert!(settings.resource_name.is_none());
        assert!(settings.base_url.is_none());
    }

    #[test]
    fn test_with_resource_name() {
        let settings = AzureOpenAIProviderSettings::new().with_resource_name("my-resource");
        assert_eq!(settings.resource_name, Some("my-resource".to_string()));
        assert_eq!(
            settings.get_base_url(),
            Some("https://my-resource.openai.azure.com/openai".to_string())
        );
    }

    #[test]
    fn test_with_base_url() {
        let settings =
            AzureOpenAIProviderSettings::new().with_base_url("https://custom.endpoint.com/openai");
        assert_eq!(
            settings.base_url,
            Some("https://custom.endpoint.com/openai".to_string())
        );
        assert_eq!(
            settings.get_base_url(),
            Some("https://custom.endpoint.com/openai".to_string())
        );
    }

    #[test]
    fn test_base_url_takes_precedence() {
        let settings = AzureOpenAIProviderSettings::new()
            .with_resource_name("my-resource")
            .with_base_url("https://custom.endpoint.com/openai");

        // base_url should take precedence
        assert_eq!(
            settings.get_base_url(),
            Some("https://custom.endpoint.com/openai".to_string())
        );
    }

    #[test]
    fn test_with_api_key() {
        let settings = AzureOpenAIProviderSettings::new().with_api_key("test-key");
        assert_eq!(settings.api_key, Some("test-key".to_string()));
    }

    #[test]
    fn test_with_api_version() {
        let settings = AzureOpenAIProviderSettings::new().with_api_version("2024-02-15-preview");
        assert_eq!(settings.api_version, "2024-02-15-preview");
    }

    #[test]
    fn test_with_headers() {
        let mut headers = HashMap::new();
        headers.insert("X-Custom".to_string(), "value".to_string());

        let settings = AzureOpenAIProviderSettings::new().with_headers(headers.clone());
        assert_eq!(settings.headers, Some(headers));
    }

    #[test]
    fn test_with_header() {
        let settings = AzureOpenAIProviderSettings::new()
            .with_header("X-Custom-1", "value1")
            .with_header("X-Custom-2", "value2");

        let headers = settings.headers.unwrap();
        assert_eq!(headers.get("X-Custom-1"), Some(&"value1".to_string()));
        assert_eq!(headers.get("X-Custom-2"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_with_deployment_based_urls() {
        let settings = AzureOpenAIProviderSettings::new().with_use_deployment_based_urls(true);
        assert!(settings.use_deployment_based_urls);
    }

    #[test]
    fn test_validate_success() {
        let settings = AzureOpenAIProviderSettings::new().with_resource_name("my-resource");
        assert!(settings.validate().is_ok());

        let settings2 =
            AzureOpenAIProviderSettings::new().with_base_url("https://custom.endpoint.com");
        assert!(settings2.validate().is_ok());
    }

    #[test]
    fn test_validate_failure() {
        let settings = AzureOpenAIProviderSettings::new();
        assert!(settings.validate().is_err());
    }

    #[test]
    fn test_builder_pattern() {
        let settings = AzureOpenAIProviderSettings::new()
            .with_resource_name("test-resource")
            .with_api_key("test-key")
            .with_api_version("2024-02-15-preview")
            .with_use_deployment_based_urls(true)
            .with_header("X-Custom", "value");

        assert_eq!(settings.resource_name, Some("test-resource".to_string()));
        assert_eq!(settings.api_key, Some("test-key".to_string()));
        assert_eq!(settings.api_version, "2024-02-15-preview");
        assert!(settings.use_deployment_based_urls);
        assert!(settings.headers.is_some());
        assert_eq!(
            settings.headers.unwrap().get("X-Custom"),
            Some(&"value".to_string())
        );
    }
}
