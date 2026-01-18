use std::collections::HashMap;

/// Settings for configuring the OpenAI provider.
///
/// # Fields
///
/// * `base_url` - Base URL for the OpenAI API calls.
///   Defaults to `https://api.openai.com/v1`.
/// * `api_key` - API key for authenticating requests.
///   Defaults to the `OPENAI_API_KEY` environment variable.
/// * `organization` - OpenAI Organization ID.
/// * `project` - OpenAI project ID.
/// * `headers` - Custom headers to include in the requests.
/// * `name` - Provider name. Overrides the `openai` default name for 3rd party providers.
///
/// # Example
///
/// ```rust
/// use llm_kit_openai::OpenAIProviderSettings;
/// use std::collections::HashMap;
///
/// let settings = OpenAIProviderSettings {
///     base_url: Some("https://api.openai.com/v1".to_string()),
///     api_key: Some("your-api-key".to_string()),
///     organization: None,
///     project: None,
///     headers: Some({
///         let mut h = HashMap::new();
///         h.insert("Custom-Header".to_string(), "value".to_string());
///         h
///     }),
///     name: Some("my-openai".to_string()),
/// };
/// ```
#[derive(Debug, Clone, Default)]
pub struct OpenAIProviderSettings {
    /// Base URL for the OpenAI API calls.
    /// Defaults to `https://api.openai.com/v1`.
    pub base_url: Option<String>,

    /// API key for authenticating requests.
    /// Defaults to the `OPENAI_API_KEY` environment variable.
    pub api_key: Option<String>,

    /// OpenAI Organization ID.
    pub organization: Option<String>,

    /// OpenAI project ID.
    pub project: Option<String>,

    /// Custom headers to include in the requests.
    pub headers: Option<HashMap<String, String>>,

    /// Provider name. Overrides the `openai` default name for 3rd party providers.
    pub name: Option<String>,
}

impl OpenAIProviderSettings {
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

    /// Set the organization ID.
    pub fn with_organization(mut self, organization: impl Into<String>) -> Self {
        self.organization = Some(organization.into());
        self
    }

    /// Set the project ID.
    pub fn with_project(mut self, project: impl Into<String>) -> Self {
        self.project = Some(project.into());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_default() {
        let settings = OpenAIProviderSettings::default();
        assert!(settings.base_url.is_none());
        assert!(settings.api_key.is_none());
        assert!(settings.organization.is_none());
        assert!(settings.project.is_none());
        assert!(settings.headers.is_none());
        assert!(settings.name.is_none());
    }

    #[test]
    fn test_settings_new() {
        let settings = OpenAIProviderSettings::new();
        assert!(settings.base_url.is_none());
        assert!(settings.api_key.is_none());
        assert!(settings.headers.is_none());
        assert!(settings.name.is_none());
    }

    #[test]
    fn test_settings_builder() {
        let settings = OpenAIProviderSettings::new()
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
        let settings = OpenAIProviderSettings::new()
            .add_header("Custom-Header", "value1")
            .add_header("Another-Header", "value2");

        let headers = settings.headers.unwrap();
        assert_eq!(headers.get("Custom-Header"), Some(&"value1".to_string()));
        assert_eq!(headers.get("Another-Header"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_settings_with_headers() {
        let mut custom_headers = HashMap::new();
        custom_headers.insert("X-Header-1".to_string(), "value1".to_string());
        custom_headers.insert("X-Header-2".to_string(), "value2".to_string());

        let settings = OpenAIProviderSettings::new().with_headers(custom_headers);

        let headers = settings.headers.unwrap();
        assert_eq!(headers.get("X-Header-1"), Some(&"value1".to_string()));
        assert_eq!(headers.get("X-Header-2"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_settings_mixed_headers() {
        let mut initial_headers = HashMap::new();
        initial_headers.insert("X-Initial".to_string(), "value".to_string());

        let settings = OpenAIProviderSettings::new()
            .with_headers(initial_headers)
            .add_header("X-Additional", "another-value");

        let headers = settings.headers.unwrap();
        assert_eq!(headers.get("X-Initial"), Some(&"value".to_string()));
        assert_eq!(
            headers.get("X-Additional"),
            Some(&"another-value".to_string())
        );
    }

    #[test]
    fn test_settings_organization_and_project() {
        let settings = OpenAIProviderSettings::new()
            .with_organization("org-123")
            .with_project("proj-456");

        assert_eq!(settings.organization, Some("org-123".to_string()));
        assert_eq!(settings.project, Some("proj-456".to_string()));
    }

    #[test]
    fn test_settings_clone() {
        let settings1 = OpenAIProviderSettings::new()
            .with_api_key("test-key")
            .with_base_url("https://api.openai.com/v1");

        let settings2 = settings1.clone();

        assert_eq!(settings1.api_key, settings2.api_key);
        assert_eq!(settings1.base_url, settings2.base_url);
    }
}
