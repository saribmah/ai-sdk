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

impl Default for DeepSeekClient {
    fn default() -> Self {
        Self::new()
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
}
