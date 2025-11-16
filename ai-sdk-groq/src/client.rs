use crate::provider::GroqProvider;
use crate::settings::GroqProviderSettings;
use std::collections::HashMap;

/// Builder for creating a Groq provider.
///
/// # Examples
///
/// ```no_run
/// use ai_sdk_groq::GroqClient;
///
/// // Basic usage
/// let provider = GroqClient::new()
///     .api_key("your-api-key")
///     .build();
///
/// // With environment variable
/// let provider = GroqClient::new()
///     .load_api_key_from_env()
///     .build();
///
/// // With custom base URL
/// let provider = GroqClient::new()
///     .api_key("your-api-key")
///     .base_url("https://custom.groq.com/openai/v1")
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct GroqClient {
    base_url: Option<String>,
    api_key: Option<String>,
    headers: HashMap<String, String>,
}

impl GroqClient {
    /// Creates a new Groq client builder.
    pub fn new() -> Self {
        Self {
            base_url: None,
            api_key: None,
            headers: HashMap::new(),
        }
    }

    /// Sets the base URL for API calls.
    ///
    /// Default: `https://api.groq.com/openai/v1`
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Sets the API key for authentication.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Loads the API key from the `GROQ_API_KEY` environment variable.
    pub fn load_api_key_from_env(mut self) -> Self {
        if let Ok(api_key) = std::env::var("GROQ_API_KEY") {
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
    /// use ai_sdk_groq::GroqClient;
    /// use std::collections::HashMap;
    ///
    /// let mut headers = HashMap::new();
    /// headers.insert("X-Header-1".to_string(), "value1".to_string());
    /// headers.insert("X-Header-2".to_string(), "value2".to_string());
    ///
    /// let client = GroqClient::new()
    ///     .headers(headers);
    /// ```
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
    }

    /// Builds the Groq provider.
    pub fn build(self) -> GroqProvider {
        let mut settings = GroqProviderSettings::new();

        if let Some(base_url) = self.base_url {
            settings = settings.with_base_url(base_url);
        }

        if let Some(api_key) = self.api_key {
            settings = settings.with_api_key(api_key);
        }

        if !self.headers.is_empty() {
            settings = settings.with_headers(self.headers);
        }

        GroqProvider::new(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_builder() {
        let client = GroqClient::new()
            .api_key("test-key")
            .base_url("https://custom.groq.com/v1")
            .header("X-Custom", "value");

        let provider = client.build();
        assert_eq!(provider.base_url(), "https://custom.groq.com/v1");
    }

    #[test]
    fn test_default_client() {
        let client = GroqClient::default();
        let provider = client.build();
        assert_eq!(provider.base_url(), "https://api.groq.com/openai/v1");
    }

    #[test]
    fn test_client_with_custom_headers() {
        let provider = GroqClient::new()
            .api_key("test-key")
            .header("X-Custom-1", "value-1")
            .header("X-Custom-2", "value-2")
            .build();

        assert_eq!(provider.name(), "groq");
    }

    #[test]
    fn test_client_with_headers_method() {
        let mut headers = HashMap::new();
        headers.insert("X-Header-1".to_string(), "value1".to_string());
        headers.insert("X-Header-2".to_string(), "value2".to_string());

        let provider = GroqClient::new()
            .api_key("test-key")
            .headers(headers)
            .build();

        assert_eq!(provider.name(), "groq");
    }

    #[test]
    fn test_client_with_mixed_headers() {
        let mut initial_headers = HashMap::new();
        initial_headers.insert("X-Initial".to_string(), "value".to_string());

        let provider = GroqClient::new()
            .api_key("test-key")
            .headers(initial_headers)
            .header("X-Additional", "another-value")
            .build();

        assert_eq!(provider.name(), "groq");
    }
}
