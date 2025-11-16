use crate::provider::DeepSeekProvider;
use crate::settings::DeepSeekProviderSettings;
use std::collections::HashMap;

/// Builder for creating a DeepSeek provider.
///
/// # Examples
///
/// ```no_run
/// use ai_sdk_deepseek::DeepSeekClient;
///
/// // Basic usage
/// let provider = DeepSeekClient::new()
///     .api_key("your-api-key")
///     .build();
///
/// // With environment variable
/// let provider = DeepSeekClient::new()
///     .load_api_key_from_env()
///     .build();
///
/// // With custom base URL
/// let provider = DeepSeekClient::new()
///     .api_key("your-api-key")
///     .base_url("https://custom.deepseek.com/v1")
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct DeepSeekClient {
    base_url: Option<String>,
    api_key: Option<String>,
    headers: HashMap<String, String>,
}

impl DeepSeekClient {
    /// Creates a new DeepSeek client builder.
    pub fn new() -> Self {
        Self {
            base_url: None,
            api_key: None,
            headers: HashMap::new(),
        }
    }

    /// Sets the base URL for API calls.
    ///
    /// Default: `https://api.deepseek.com/v1`
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Sets the API key for authentication.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Loads the API key from the `DEEPSEEK_API_KEY` environment variable.
    pub fn load_api_key_from_env(mut self) -> Self {
        if let Ok(api_key) = std::env::var("DEEPSEEK_API_KEY") {
            self.api_key = Some(api_key);
        }
        self
    }

    /// Adds a custom header to all requests.
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Sets multiple custom headers at once.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_deepseek::DeepSeekClient;
    /// use std::collections::HashMap;
    ///
    /// let mut headers = HashMap::new();
    /// headers.insert("X-Header-1".to_string(), "value1".to_string());
    /// headers.insert("X-Header-2".to_string(), "value2".to_string());
    ///
    /// let client = DeepSeekClient::new()
    ///     .headers(headers);
    /// ```
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
    }

    /// Builds the DeepSeek provider.
    pub fn build(self) -> DeepSeekProvider {
        let mut settings = DeepSeekProviderSettings::new();

        if let Some(base_url) = self.base_url {
            settings = settings.with_base_url(base_url);
        }

        if let Some(api_key) = self.api_key {
            settings = settings.with_api_key(api_key);
        }

        if !self.headers.is_empty() {
            settings = settings.with_headers(self.headers);
        }

        DeepSeekProvider::new(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_builder() {
        let client = DeepSeekClient::new()
            .api_key("test-key")
            .base_url("https://custom.deepseek.com/v1")
            .header("X-Custom", "value");

        let provider = client.build();
        assert_eq!(provider.base_url(), "https://custom.deepseek.com/v1");
    }

    #[test]
    fn test_default_client() {
        let client = DeepSeekClient::default();
        let provider = client.build();
        assert_eq!(provider.base_url(), "https://api.deepseek.com/v1");
    }

    #[test]
    fn test_client_with_custom_headers() {
        let provider = DeepSeekClient::new()
            .api_key("test-key")
            .header("X-Custom-1", "value-1")
            .header("X-Custom-2", "value-2")
            .build();

        assert_eq!(provider.name(), "deepseek");
    }

    #[test]
    fn test_client_with_headers_method() {
        let mut headers = HashMap::new();
        headers.insert("X-Header-1".to_string(), "value1".to_string());
        headers.insert("X-Header-2".to_string(), "value2".to_string());

        let provider = DeepSeekClient::new()
            .api_key("test-key")
            .headers(headers)
            .build();

        assert_eq!(provider.name(), "deepseek");
    }

    #[test]
    fn test_client_with_mixed_headers() {
        let mut initial_headers = HashMap::new();
        initial_headers.insert("X-Initial".to_string(), "value".to_string());

        let provider = DeepSeekClient::new()
            .api_key("test-key")
            .headers(initial_headers)
            .header("X-Additional", "another-value")
            .build();

        assert_eq!(provider.name(), "deepseek");
    }
}
