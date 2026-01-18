use std::collections::HashMap;

use crate::provider::BasetenProvider;
use crate::settings::BasetenProviderSettings;

/// Builder for creating a Baseten client.
///
/// Provides a fluent API for constructing a `BasetenProvider` with various configuration options.
///
/// # Examples
///
/// ## Basic Usage with Model APIs
///
/// ```no_run
/// use llm_kit_baseten::BasetenClient;
///
/// let provider = BasetenClient::new()
///     .api_key("your-api-key")
///     .build();
///
/// let model = provider.chat_model(Some("deepseek-ai/DeepSeek-V3-0324"));
/// ```
///
/// ## Custom Model URL
///
/// ```no_run
/// use llm_kit_baseten::BasetenClient;
///
/// let provider = BasetenClient::new()
///     .api_key("your-api-key")
///     .model_url("https://model-abc123.api.baseten.co/environments/production/sync/v1")
///     .build();
///
/// let model = provider.chat_model(None);
/// ```
///
/// ## With Environment Variable
///
/// ```no_run
/// use llm_kit_baseten::BasetenClient;
///
/// // API key loaded from BASETEN_API_KEY environment variable
/// let provider = BasetenClient::new().build();
/// let model = provider.chat_model(Some("deepseek-ai/DeepSeek-V3-0324"));
/// ```
#[derive(Debug, Clone, Default)]
pub struct BasetenClient {
    api_key: Option<String>,
    base_url: Option<String>,
    model_url: Option<String>,
    headers: HashMap<String, String>,
}

impl BasetenClient {
    /// Creates a new client builder with default settings.
    ///
    /// The default base URL is `https://inference.baseten.co/v1` (Model APIs).
    /// API key will be loaded from the `BASETEN_API_KEY` environment variable if not explicitly set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the API key for authentication.
    ///
    /// If not specified, the API key will be loaded from the `BASETEN_API_KEY` environment variable.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The Baseten API key
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the base URL for Model APIs.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL (default: `https://inference.baseten.co/v1`)
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Sets the custom model URL for dedicated deployments.
    ///
    /// This is required for embedding models and optional for chat models.
    ///
    /// # Arguments
    ///
    /// * `model_url` - The model URL (e.g., `https://model-abc.api.baseten.co/environments/production/sync/v1`)
    pub fn model_url(mut self, model_url: impl Into<String>) -> Self {
        self.model_url = Some(model_url.into());
        self
    }

    /// Adds a custom header to include in requests.
    ///
    /// # Arguments
    ///
    /// * `key` - The header name
    /// * `value` - The header value
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Sets multiple custom headers at once.
    ///
    /// # Arguments
    ///
    /// * `headers` - A HashMap of header names to values
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
    }

    /// Builds the `BasetenProvider` with the configured settings.
    ///
    /// If no API key is explicitly set, attempts to load from the `BASETEN_API_KEY` environment variable.
    ///
    /// # Returns
    ///
    /// A `BasetenProvider` instance.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use llm_kit_baseten::BasetenClient;
    ///
    /// let provider = BasetenClient::new()
    ///     .api_key("your-api-key")
    ///     .build();
    /// ```
    pub fn build(self) -> BasetenProvider {
        let mut settings = BasetenProviderSettings::new();

        // Load API key from environment if not explicitly set
        if let Some(api_key) = self.api_key {
            settings = settings.with_api_key(api_key);
        } else if let Ok(env_key) = std::env::var("BASETEN_API_KEY") {
            settings = settings.with_api_key(env_key);
        }
        // Note: If no API key is found, it will be handled by the provider when making requests

        if let Some(base_url) = self.base_url {
            settings = settings.with_base_url(base_url);
        }

        if let Some(model_url) = self.model_url {
            settings = settings.with_model_url(model_url);
        }

        if !self.headers.is_empty() {
            settings = settings.with_headers(self.headers);
        }

        BasetenProvider::new(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_client_builder() {
        let provider = BasetenClient::new().api_key("test-key").build();

        assert_eq!(provider.base_url(), "https://inference.baseten.co/v1");
    }

    #[test]
    fn test_with_custom_base_url() {
        let provider = BasetenClient::new()
            .api_key("test-key")
            .base_url("https://custom.baseten.co/v1")
            .build();

        assert_eq!(provider.base_url(), "https://custom.baseten.co/v1");
    }

    #[test]
    fn test_with_model_url() {
        let provider = BasetenClient::new()
            .api_key("test-key")
            .model_url("https://model-abc.api.baseten.co/sync/v1")
            .build();

        assert_eq!(
            provider.model_url(),
            Some("https://model-abc.api.baseten.co/sync/v1")
        );
    }

    #[test]
    fn test_with_headers() {
        let provider = BasetenClient::new()
            .api_key("test-key")
            .header("X-Custom-Header", "value1")
            .header("X-Another-Header", "value2")
            .build();

        assert_eq!(provider.base_url(), "https://inference.baseten.co/v1");
    }

    #[test]
    fn test_chained_builder() {
        let provider = BasetenClient::new()
            .api_key("test-key")
            .base_url("https://custom.url")
            .model_url("https://model.url")
            .header("X-Test", "value")
            .build();

        assert_eq!(provider.base_url(), "https://custom.url");
        assert_eq!(provider.model_url(), Some("https://model.url"));
    }

    #[test]
    fn test_bulk_headers() {
        let mut headers = HashMap::new();
        headers.insert("X-Header-1".to_string(), "value1".to_string());
        headers.insert("X-Header-2".to_string(), "value2".to_string());

        let provider = BasetenClient::new()
            .api_key("test-key")
            .headers(headers)
            .build();

        assert_eq!(provider.base_url(), "https://inference.baseten.co/v1");
    }

    #[test]
    fn test_mixed_headers_methods() {
        let mut initial_headers = HashMap::new();
        initial_headers.insert("X-Initial".to_string(), "value".to_string());

        let provider = BasetenClient::new()
            .api_key("test-key")
            .headers(initial_headers)
            .header("X-Additional", "another-value")
            .build();

        assert_eq!(provider.base_url(), "https://inference.baseten.co/v1");
    }
}
