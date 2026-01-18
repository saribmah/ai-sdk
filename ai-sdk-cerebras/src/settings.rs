use std::collections::HashMap;

/// Settings for configuring the Cerebras provider.
///
/// These settings control how the provider connects to the Cerebras API
/// and what default options are used for model calls.
#[derive(Debug, Clone)]
pub struct CerebrasProviderSettings {
    /// Base URL for API calls (default: <https://api.cerebras.ai/v1>)
    pub base_url: String,

    /// API key for authentication
    pub api_key: Option<String>,

    /// Custom headers to include in all requests
    pub headers: Option<HashMap<String, String>>,
}

impl CerebrasProviderSettings {
    /// Creates new settings with the given base URL.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL for API calls
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_cerebras::CerebrasProviderSettings;
    ///
    /// let settings = CerebrasProviderSettings::new("https://api.cerebras.ai/v1");
    /// ```
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: None,
            headers: None,
        }
    }

    /// Sets the API key for authentication.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key to use
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_cerebras::CerebrasProviderSettings;
    ///
    /// let settings = CerebrasProviderSettings::new("https://api.cerebras.ai/v1")
    ///     .with_api_key("your-api-key");
    /// ```
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
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
    /// ```
    /// use llm_kit_cerebras::CerebrasProviderSettings;
    ///
    /// let settings = CerebrasProviderSettings::new("https://api.cerebras.ai/v1")
    ///     .with_header("X-Custom-Header", "value");
    /// ```
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
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
    /// ```
    /// use llm_kit_cerebras::CerebrasProviderSettings;
    /// use std::collections::HashMap;
    ///
    /// let mut headers = HashMap::new();
    /// headers.insert("X-Header-1".to_string(), "value1".to_string());
    ///
    /// let settings = CerebrasProviderSettings::new("https://api.cerebras.ai/v1")
    ///     .with_headers(headers);
    /// ```
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        match &mut self.headers {
            Some(existing) => existing.extend(headers),
            None => self.headers = Some(headers),
        }
        self
    }
}

impl Default for CerebrasProviderSettings {
    fn default() -> Self {
        Self::new("https://api.cerebras.ai/v1")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let settings = CerebrasProviderSettings::new("https://api.cerebras.ai/v1");
        assert_eq!(settings.base_url, "https://api.cerebras.ai/v1");
        assert!(settings.api_key.is_none());
        assert!(settings.headers.is_none());
    }

    #[test]
    fn test_with_api_key() {
        let settings =
            CerebrasProviderSettings::new("https://api.cerebras.ai/v1").with_api_key("test-key");
        assert_eq!(settings.api_key, Some("test-key".to_string()));
    }

    #[test]
    fn test_with_header() {
        let settings = CerebrasProviderSettings::new("https://api.cerebras.ai/v1")
            .with_header("X-Custom", "value");
        assert!(settings.headers.is_some());
        let headers = settings.headers.unwrap();
        assert_eq!(headers.get("X-Custom"), Some(&"value".to_string()));
    }

    #[test]
    fn test_with_headers() {
        let mut headers = HashMap::new();
        headers.insert("X-Header-1".to_string(), "value1".to_string());
        headers.insert("X-Header-2".to_string(), "value2".to_string());

        let settings =
            CerebrasProviderSettings::new("https://api.cerebras.ai/v1").with_headers(headers);
        assert!(settings.headers.is_some());
        let result_headers = settings.headers.unwrap();
        assert_eq!(
            result_headers.get("X-Header-1"),
            Some(&"value1".to_string())
        );
        assert_eq!(
            result_headers.get("X-Header-2"),
            Some(&"value2".to_string())
        );
    }

    #[test]
    fn test_default() {
        let settings = CerebrasProviderSettings::default();
        assert_eq!(settings.base_url, "https://api.cerebras.ai/v1");
    }

    #[test]
    fn test_chained_builder() {
        let settings = CerebrasProviderSettings::new("https://api.cerebras.ai/v1")
            .with_api_key("key")
            .with_header("X-Custom", "value");

        assert_eq!(settings.api_key, Some("key".to_string()));
        assert!(settings.headers.is_some());
    }
}
