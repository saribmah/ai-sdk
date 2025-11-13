use std::collections::HashMap;

/// Configuration options for creating an xAI provider.
#[derive(Debug, Clone)]
pub struct XaiProviderSettings {
    /// Base URL for the API calls (default: "https://api.x.ai/v1")
    pub base_url: String,

    /// API key for authenticating requests. If specified, adds an `Authorization`
    /// header with the value `Bearer <apiKey>`.
    pub api_key: Option<String>,

    /// Optional custom headers to include in requests. These will be added to request headers
    /// after any headers potentially added by use of the `api_key` option.
    pub headers: Option<HashMap<String, String>>,
}

impl Default for XaiProviderSettings {
    fn default() -> Self {
        Self {
            base_url: "https://api.x.ai/v1".to_string(),
            api_key: None,
            headers: None,
        }
    }
}

impl XaiProviderSettings {
    /// Creates a new xAI provider configuration with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the base URL for API calls.
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Sets the API key.
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = XaiProviderSettings::default();
        assert_eq!(settings.base_url, "https://api.x.ai/v1");
        assert!(settings.api_key.is_none());
        assert!(settings.headers.is_none());
    }

    #[test]
    fn test_settings_builder() {
        let settings = XaiProviderSettings::new()
            .with_api_key("test-key")
            .with_header("X-Custom-Header", "value")
            .with_base_url("https://custom.api.x.ai/v1");

        assert_eq!(settings.base_url, "https://custom.api.x.ai/v1");
        assert_eq!(settings.api_key, Some("test-key".to_string()));

        let headers = settings.headers.unwrap();
        assert_eq!(headers.get("X-Custom-Header"), Some(&"value".to_string()));
    }

    #[test]
    fn test_multiple_headers() {
        let settings = XaiProviderSettings::new()
            .with_header("Header-1", "value-1")
            .with_header("Header-2", "value-2");

        let headers = settings.headers.unwrap();
        assert_eq!(headers.len(), 2);
        assert_eq!(headers.get("Header-1"), Some(&"value-1".to_string()));
        assert_eq!(headers.get("Header-2"), Some(&"value-2".to_string()));
    }
}
