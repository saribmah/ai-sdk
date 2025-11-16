use std::collections::HashMap;

/// Configuration options for creating a Baseten provider.
#[derive(Debug, Clone)]
pub struct BasetenProviderSettings {
    /// Base URL for the Model APIs. Default: 'https://inference.baseten.co/v1'
    pub base_url: String,

    /// API key for authenticating requests. If not specified, will be loaded
    /// from the `BASETEN_API_KEY` environment variable.
    pub api_key: Option<String>,

    /// Model URL for custom models (chat or embeddings).
    /// If not supplied, the default Model APIs will be used.
    ///
    /// For chat models: Must be a /sync/v1 endpoint
    /// For embeddings: Must be a /sync or /sync/v1 endpoint
    pub model_url: Option<String>,

    /// Optional custom headers to include in requests.
    pub headers: Option<HashMap<String, String>>,
}

impl Default for BasetenProviderSettings {
    fn default() -> Self {
        Self {
            base_url: "https://inference.baseten.co/v1".to_string(),
            api_key: None,
            model_url: None,
            headers: None,
        }
    }
}

impl BasetenProviderSettings {
    /// Creates a new configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the API key.
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the base URL for Model APIs.
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Sets the custom model URL.
    pub fn with_model_url(mut self, model_url: impl Into<String>) -> Self {
        self.model_url = Some(model_url.into());
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
        let settings = BasetenProviderSettings::default();
        assert_eq!(settings.base_url, "https://inference.baseten.co/v1");
        assert!(settings.api_key.is_none());
        assert!(settings.model_url.is_none());
        assert!(settings.headers.is_none());
    }

    #[test]
    fn test_with_api_key() {
        let settings = BasetenProviderSettings::new().with_api_key("test-key");
        assert_eq!(settings.api_key, Some("test-key".to_string()));
    }

    #[test]
    fn test_with_base_url() {
        let settings = BasetenProviderSettings::new().with_base_url("https://custom.baseten.co/v1");
        assert_eq!(settings.base_url, "https://custom.baseten.co/v1");
    }

    #[test]
    fn test_with_model_url() {
        let settings = BasetenProviderSettings::new()
            .with_model_url("https://model-abc.api.baseten.co/sync/v1");
        assert_eq!(
            settings.model_url,
            Some("https://model-abc.api.baseten.co/sync/v1".to_string())
        );
    }

    #[test]
    fn test_with_header() {
        let settings = BasetenProviderSettings::new().with_header("X-Custom", "value");
        let headers = settings.headers.unwrap();
        assert_eq!(headers.get("X-Custom"), Some(&"value".to_string()));
    }

    #[test]
    fn test_with_headers() {
        let mut custom_headers = HashMap::new();
        custom_headers.insert("X-Header-1".to_string(), "value1".to_string());
        custom_headers.insert("X-Header-2".to_string(), "value2".to_string());

        let settings = BasetenProviderSettings::new().with_headers(custom_headers.clone());
        let headers = settings.headers.unwrap();
        assert_eq!(headers.get("X-Header-1"), Some(&"value1".to_string()));
        assert_eq!(headers.get("X-Header-2"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_chained_builder() {
        let settings = BasetenProviderSettings::new()
            .with_api_key("key")
            .with_base_url("https://custom.url")
            .with_model_url("https://model.url")
            .with_header("X-Test", "value");

        assert_eq!(settings.api_key, Some("key".to_string()));
        assert_eq!(settings.base_url, "https://custom.url");
        assert_eq!(settings.model_url, Some("https://model.url".to_string()));
        assert_eq!(
            settings.headers.unwrap().get("X-Test"),
            Some(&"value".to_string())
        );
    }
}
