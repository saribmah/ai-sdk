use std::collections::HashMap;

use crate::provider::CerebrasProvider;
use crate::settings::CerebrasProviderSettings;

/// Builder for creating a Cerebras client.
///
/// Provides a fluent API for constructing a `CerebrasProvider` with various configuration options.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```no_run
/// use llm_kit_cerebras::CerebrasClient;
///
/// let provider = CerebrasClient::new()
///     .api_key("your-api-key")
///     .build();
///
/// let model = provider.chat_model("llama-3.3-70b");
/// ```
///
/// ## Custom Base URL
///
/// ```no_run
/// use llm_kit_cerebras::CerebrasClient;
///
/// let provider = CerebrasClient::new()
///     .base_url("https://custom.cerebras.ai/v1")
///     .api_key("your-api-key")
///     .build();
///
/// let model = provider.chat_model("llama-3.3-70b");
/// ```
///
/// ## With Custom Headers
///
/// ```no_run
/// use llm_kit_cerebras::CerebrasClient;
///
/// let provider = CerebrasClient::new()
///     .api_key("your-api-key")
///     .header("X-Custom-Header", "value")
///     .build();
///
/// let model = provider.chat_model("llama-3.3-70b");
/// ```
#[derive(Debug, Clone, Default)]
pub struct CerebrasClient {
    base_url: Option<String>,
    api_key: Option<String>,
    headers: HashMap<String, String>,
}

impl CerebrasClient {
    /// Creates a new client builder with default settings.
    ///
    /// The default base URL is `https://api.cerebras.ai/v1`.
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_cerebras::CerebrasClient;
    ///
    /// let client = CerebrasClient::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the base URL for API calls.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL (e.g., "<https://api.cerebras.ai/v1>")
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_cerebras::CerebrasClient;
    ///
    /// let client = CerebrasClient::new()
    ///     .base_url("https://custom.cerebras.ai/v1");
    /// ```
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Sets the API key for authentication.
    ///
    /// If not provided here, the provider will attempt to read from the
    /// `CEREBRAS_API_KEY` environment variable.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_cerebras::CerebrasClient;
    ///
    /// let client = CerebrasClient::new()
    ///     .api_key("your-api-key");
    /// ```
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Adds a custom header to include in requests.
    ///
    /// Custom headers are added after the Authorization header.
    ///
    /// # Arguments
    ///
    /// * `key` - The header name
    /// * `value` - The header value
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_cerebras::CerebrasClient;
    ///
    /// let client = CerebrasClient::new()
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
    /// ```
    /// use llm_kit_cerebras::CerebrasClient;
    /// use std::collections::HashMap;
    ///
    /// let mut headers = HashMap::new();
    /// headers.insert("X-Header-1".to_string(), "value1".to_string());
    ///
    /// let client = CerebrasClient::new()
    ///     .headers(headers);
    /// ```
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
    }

    /// Builds the `CerebrasProvider` with the configured settings.
    ///
    /// If no API key is provided via the builder, the provider will attempt
    /// to load it from the `CEREBRAS_API_KEY` environment variable.
    ///
    /// # Returns
    ///
    /// A `CerebrasProvider` instance.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use llm_kit_cerebras::CerebrasClient;
    ///
    /// let provider = CerebrasClient::new()
    ///     .api_key("your-api-key")
    ///     .build();
    /// ```
    pub fn build(self) -> CerebrasProvider {
        let base_url = self
            .base_url
            .unwrap_or_else(|| "https://api.cerebras.ai/v1".to_string());

        let mut settings = CerebrasProviderSettings::new(base_url);

        // Try to get API key from builder, then environment variable
        let api_key = self
            .api_key
            .or_else(|| std::env::var("CEREBRAS_API_KEY").ok());

        if let Some(key) = api_key {
            settings = settings.with_api_key(key);
        }

        if !self.headers.is_empty() {
            settings = settings.with_headers(self.headers);
        }

        CerebrasProvider::new(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_client_builder() {
        let provider = CerebrasClient::new().api_key("test-key").build();

        assert_eq!(provider.name(), "cerebras");
        assert_eq!(provider.base_url(), "https://api.cerebras.ai/v1");
    }

    #[test]
    fn test_custom_base_url() {
        let provider = CerebrasClient::new()
            .base_url("https://custom.cerebras.ai/v1")
            .api_key("test-key")
            .build();

        assert_eq!(provider.base_url(), "https://custom.cerebras.ai/v1");
    }

    #[test]
    fn test_with_headers() {
        let provider = CerebrasClient::new()
            .api_key("test-key")
            .header("X-Custom-Header", "value1")
            .header("X-Another-Header", "value2")
            .build();

        assert_eq!(provider.name(), "cerebras");
    }

    #[test]
    fn test_default_values() {
        let provider = CerebrasClient::new().build();

        assert_eq!(provider.name(), "cerebras");
        assert_eq!(provider.base_url(), "https://api.cerebras.ai/v1");
    }

    #[test]
    fn test_chained_model_creation() {
        let model = CerebrasClient::new()
            .api_key("test-key")
            .build()
            .chat_model("llama-3.3-70b");

        assert_eq!(model.model_id(), "llama-3.3-70b");
        assert_eq!(model.provider(), "cerebras.chat");
    }

    #[test]
    fn test_bulk_headers() {
        let mut headers = HashMap::new();
        headers.insert("X-Header-1".to_string(), "value1".to_string());
        headers.insert("X-Header-2".to_string(), "value2".to_string());

        let provider = CerebrasClient::new()
            .api_key("test-key")
            .headers(headers)
            .build();

        assert_eq!(provider.name(), "cerebras");
    }

    #[test]
    fn test_mixed_headers_methods() {
        let mut initial_headers = HashMap::new();
        initial_headers.insert("X-Initial".to_string(), "value".to_string());

        let provider = CerebrasClient::new()
            .api_key("test-key")
            .headers(initial_headers)
            .header("X-Additional", "another-value")
            .build();

        assert_eq!(provider.name(), "cerebras");
    }
}
