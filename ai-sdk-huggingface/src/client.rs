use std::collections::HashMap;

use crate::provider::HuggingFaceProvider;
use crate::settings::HuggingFaceProviderSettings;

/// Type alias for URL generation function
pub type UrlGeneratorFn = Box<dyn Fn(&str, &str) -> String + Send + Sync>;

/// Type alias for headers generation function
pub type HeadersGeneratorFn = Box<dyn Fn() -> HashMap<String, String> + Send + Sync>;

/// Configuration for HTTP client used by Hugging Face models.
pub struct HuggingFaceClientConfig {
    /// Provider name (e.g., "huggingface.responses").
    pub provider: String,

    /// Function to generate the URL for API requests.
    /// Takes (model_id, path) and returns the full URL.
    pub url: UrlGeneratorFn,

    /// Function to generate headers for API requests.
    pub headers: HeadersGeneratorFn,
}

impl HuggingFaceClientConfig {
    /// Creates a new client configuration.
    pub fn new(
        provider: impl Into<String>,
        url: UrlGeneratorFn,
        headers: HeadersGeneratorFn,
    ) -> Self {
        Self {
            provider: provider.into(),
            url,
            headers,
        }
    }
}

/// Builder for creating a Hugging Face client.
///
/// Provides a fluent API for constructing a `HuggingFaceProvider` with various configuration options.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```no_run
/// use ai_sdk_huggingface::HuggingFaceClient;
///
/// let provider = HuggingFaceClient::new()
///     .api_key("your-api-key")
///     .build();
///
/// let model = provider.responses("meta-llama/Llama-3.1-8B-Instruct");
/// ```
///
/// ## Custom Base URL
///
/// ```no_run
/// use ai_sdk_huggingface::HuggingFaceClient;
///
/// let provider = HuggingFaceClient::new()
///     .base_url("https://custom.api.com/v1")
///     .api_key("your-api-key")
///     .build();
///
/// let model = provider.responses("meta-llama/Llama-3.1-8B-Instruct");
/// ```
///
/// ## With Custom Headers
///
/// ```no_run
/// use ai_sdk_huggingface::HuggingFaceClient;
///
/// let provider = HuggingFaceClient::new()
///     .api_key("your-api-key")
///     .header("X-Custom-Header", "value")
///     .build();
///
/// let model = provider.responses("meta-llama/Llama-3.1-8B-Instruct");
/// ```
#[derive(Debug, Clone, Default)]
pub struct HuggingFaceClient {
    base_url: Option<String>,
    api_key: Option<String>,
    headers: HashMap<String, String>,
}

impl HuggingFaceClient {
    /// Creates a new client builder with default settings.
    ///
    /// The default base URL is `https://router.huggingface.co/v1`.
    /// If no API key is provided, it will attempt to use the `HUGGINGFACE_API_KEY` environment variable.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the base URL for API calls.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL (e.g., "https://router.huggingface.co/v1")
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ai_sdk_huggingface::HuggingFaceClient;
    ///
    /// let client = HuggingFaceClient::new()
    ///     .base_url("https://custom.api.com/v1");
    /// ```
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Sets the API key for authentication.
    ///
    /// If not specified, the client will attempt to use the `HUGGINGFACE_API_KEY` environment variable.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ai_sdk_huggingface::HuggingFaceClient;
    ///
    /// let client = HuggingFaceClient::new()
    ///     .api_key("your-api-key");
    /// ```
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
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
    /// ```no_run
    /// use ai_sdk_huggingface::HuggingFaceClient;
    ///
    /// let client = HuggingFaceClient::new()
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
    /// ```no_run
    /// use ai_sdk_huggingface::HuggingFaceClient;
    /// use std::collections::HashMap;
    ///
    /// let mut headers = HashMap::new();
    /// headers.insert("X-Header-1".to_string(), "value1".to_string());
    /// headers.insert("X-Header-2".to_string(), "value2".to_string());
    ///
    /// let client = HuggingFaceClient::new()
    ///     .headers(headers);
    /// ```
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
    }

    /// Builds the `HuggingFaceProvider` with the configured settings.
    ///
    /// # Returns
    ///
    /// A `HuggingFaceProvider` instance.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ai_sdk_huggingface::HuggingFaceClient;
    ///
    /// let provider = HuggingFaceClient::new()
    ///     .api_key("your-api-key")
    ///     .build();
    /// ```
    pub fn build(self) -> HuggingFaceProvider {
        let mut settings = HuggingFaceProviderSettings::new();

        if let Some(base_url) = self.base_url {
            settings = settings.with_base_url(base_url);
        }

        if let Some(api_key) = self.api_key {
            settings = settings.with_api_key(api_key);
        } else {
            // Load from environment variable if not explicitly set
            settings = settings.load_api_key_from_env();
        }

        // Add custom headers
        for (key, value) in self.headers {
            settings = settings.with_header(key, value);
        }

        HuggingFaceProvider::new(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_client_builder() {
        let provider = HuggingFaceClient::new().api_key("test-key").build();

        // Default base URL should be set
        assert_eq!(provider.base_url(), "https://router.huggingface.co/v1");
    }

    #[test]
    fn test_custom_base_url() {
        let provider = HuggingFaceClient::new()
            .base_url("https://custom.api.com/v1")
            .api_key("test-key")
            .build();

        assert_eq!(provider.base_url(), "https://custom.api.com/v1");
    }

    #[test]
    fn test_with_headers() {
        let provider = HuggingFaceClient::new()
            .api_key("test-key")
            .header("X-Custom-Header", "value1")
            .header("X-Another-Header", "value2")
            .build();

        // Verify the provider was created successfully
        assert_eq!(provider.base_url(), "https://router.huggingface.co/v1");
    }

    #[test]
    fn test_bulk_headers() {
        let mut custom_headers = HashMap::new();
        custom_headers.insert("X-Header-1".to_string(), "value1".to_string());
        custom_headers.insert("X-Header-2".to_string(), "value2".to_string());

        let provider = HuggingFaceClient::new()
            .api_key("test-key")
            .headers(custom_headers)
            .build();

        // Verify the provider was created successfully
        assert_eq!(provider.base_url(), "https://router.huggingface.co/v1");
    }

    #[test]
    fn test_mixed_headers_methods() {
        let mut initial_headers = HashMap::new();
        initial_headers.insert("X-Initial".to_string(), "value".to_string());

        let provider = HuggingFaceClient::new()
            .api_key("test-key")
            .headers(initial_headers)
            .header("X-Additional", "another-value")
            .build();

        // Verify the provider was created successfully
        assert_eq!(provider.base_url(), "https://router.huggingface.co/v1");
    }

    #[test]
    fn test_chained_model_creation() {
        let model = HuggingFaceClient::new()
            .api_key("test-key")
            .build()
            .responses("meta-llama/Llama-3.1-8B-Instruct");

        assert_eq!(model.model_id(), "meta-llama/Llama-3.1-8B-Instruct");
        assert_eq!(model.provider(), "huggingface.responses");
    }

    #[test]
    fn test_default_values() {
        let provider = HuggingFaceClient::new().api_key("test-key").build();

        assert_eq!(provider.base_url(), "https://router.huggingface.co/v1");
    }

    #[test]
    fn test_trailing_slash_removed() {
        let provider = HuggingFaceClient::new()
            .base_url("https://router.huggingface.co/v1/")
            .api_key("test-key")
            .build();

        assert_eq!(provider.base_url(), "https://router.huggingface.co/v1");
    }
}
