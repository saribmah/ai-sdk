use std::collections::HashMap;

/// Configuration options for creating a Together AI provider.
///
/// Together AI is an AI inference platform that provides access to various
/// open-source models including Llama, Mistral, Qwen, and more.
#[derive(Debug, Clone)]
pub struct TogetherAIProviderSettings {
    /// Base URL for the API calls
    ///
    /// Defaults to `https://api.together.xyz/v1`
    pub base_url: String,

    /// API key for authenticating requests
    ///
    /// If not provided, will attempt to load from `TOGETHER_AI_API_KEY` environment variable.
    /// Adds an `Authorization` header with the value `Bearer <apiKey>`.
    pub api_key: Option<String>,

    /// Optional custom headers to include in requests
    ///
    /// These will be added to request headers after any headers potentially
    /// added by use of the `api_key` option.
    pub headers: Option<HashMap<String, String>>,
}

impl Default for TogetherAIProviderSettings {
    fn default() -> Self {
        Self {
            base_url: "https://api.together.xyz/v1".to_string(),
            api_key: None,
            headers: None,
        }
    }
}

impl TogetherAIProviderSettings {
    /// Creates a new configuration with default settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_togetherai::TogetherAIProviderSettings;
    ///
    /// let settings = TogetherAIProviderSettings::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The Together AI API key
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_togetherai::TogetherAIProviderSettings;
    ///
    /// let settings = TogetherAIProviderSettings::new()
    ///     .with_api_key("your-api-key");
    /// ```
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the base URL.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL for API requests
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_togetherai::TogetherAIProviderSettings;
    ///
    /// let settings = TogetherAIProviderSettings::new()
    ///     .with_base_url("https://custom-url.com");
    /// ```
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Sets additional headers.
    ///
    /// # Arguments
    ///
    /// * `headers` - A map of header key-value pairs
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_togetherai::TogetherAIProviderSettings;
    /// use std::collections::HashMap;
    ///
    /// let mut headers = HashMap::new();
    /// headers.insert("X-Custom-Header".to_string(), "value".to_string());
    ///
    /// let settings = TogetherAIProviderSettings::new()
    ///     .with_headers(headers);
    /// ```
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Adds a single header.
    ///
    /// # Arguments
    ///
    /// * `key` - The header name
    /// * `value` - The header value
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_togetherai::TogetherAIProviderSettings;
    ///
    /// let settings = TogetherAIProviderSettings::new()
    ///     .with_header("X-Custom-Header", "value");
    /// ```
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let mut headers = self.headers.unwrap_or_default();
        headers.insert(key.into(), value.into());
        self.headers = Some(headers);
        self
    }

    /// Gets the API key, loading from environment if not set.
    ///
    /// Attempts to load from `TOGETHER_AI_API_KEY` environment variable if
    /// `api_key` is not explicitly set.
    pub(crate) fn get_api_key(&self) -> Option<String> {
        self.api_key
            .clone()
            .or_else(|| std::env::var("TOGETHER_AI_API_KEY").ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = TogetherAIProviderSettings::default();
        assert_eq!(settings.base_url, "https://api.together.xyz/v1");
        assert!(settings.api_key.is_none());
        assert!(settings.headers.is_none());
    }

    #[test]
    fn test_new_settings() {
        let settings = TogetherAIProviderSettings::new();
        assert_eq!(settings.base_url, "https://api.together.xyz/v1");
    }

    #[test]
    fn test_with_api_key() {
        let settings = TogetherAIProviderSettings::new().with_api_key("test-key");
        assert_eq!(settings.api_key, Some("test-key".to_string()));
    }

    #[test]
    fn test_with_base_url() {
        let settings = TogetherAIProviderSettings::new().with_base_url("https://custom.com");
        assert_eq!(settings.base_url, "https://custom.com");
    }

    #[test]
    fn test_with_header() {
        let settings = TogetherAIProviderSettings::new().with_header("X-Test", "value");
        let headers = settings.headers.unwrap();
        assert_eq!(headers.get("X-Test"), Some(&"value".to_string()));
    }

    #[test]
    fn test_with_headers() {
        let mut headers = HashMap::new();
        headers.insert("X-Test1".to_string(), "value1".to_string());
        headers.insert("X-Test2".to_string(), "value2".to_string());

        let settings = TogetherAIProviderSettings::new().with_headers(headers);
        let result_headers = settings.headers.unwrap();
        assert_eq!(result_headers.len(), 2);
    }

    #[test]
    fn test_builder_pattern() {
        let settings = TogetherAIProviderSettings::new()
            .with_api_key("key")
            .with_base_url("https://custom.com")
            .with_header("X-Test", "value");

        assert_eq!(settings.api_key, Some("key".to_string()));
        assert_eq!(settings.base_url, "https://custom.com");
        assert!(settings.headers.is_some());
    }
}
