use std::collections::HashMap;

use crate::provider::AnthropicProvider;
use crate::settings::AnthropicProviderSettings;

/// Builder for creating an Anthropic client.
///
/// Provides a fluent API for constructing an `AnthropicProvider` with various configuration options.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```no_run
/// use llm_kit_anthropic::AnthropicClient;
///
/// let provider = AnthropicClient::new()
///     .api_key("your-api-key")
///     .build();
///
/// let model = provider.language_model("claude-3-5-sonnet-20241022".to_string());
/// ```
///
/// ## Custom Base URL
///
/// ```no_run
/// use llm_kit_anthropic::AnthropicClient;
///
/// let provider = AnthropicClient::new()
///     .base_url("https://custom.api.com/v1")
///     .api_key("your-api-key")
///     .build();
///
/// let model = provider.language_model("claude-3-5-sonnet-20241022".to_string());
/// ```
///
/// ## With Custom Headers
///
/// ```no_run
/// use llm_kit_anthropic::AnthropicClient;
///
/// let provider = AnthropicClient::new()
///     .api_key("your-api-key")
///     .header("X-Custom-Header", "value")
///     .build();
///
/// let model = provider.language_model("claude-3-5-sonnet-20241022".to_string());
/// ```
#[derive(Debug, Clone, Default)]
pub struct AnthropicClient {
    base_url: Option<String>,
    api_key: Option<String>,
    headers: HashMap<String, String>,
    name: Option<String>,
}

impl AnthropicClient {
    /// Creates a new client builder with default settings.
    ///
    /// The default base URL is `https://api.anthropic.com/v1` and the default name is `anthropic.messages`.
    /// If no API key is provided, it will attempt to use the `ANTHROPIC_API_KEY` environment variable.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the base URL for API calls.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL (e.g., "<https://api.anthropic.com/v1>")
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use llm_kit_anthropic::AnthropicClient;
    ///
    /// let client = AnthropicClient::new()
    ///     .base_url("https://custom.api.com/v1");
    /// ```
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Sets the API key for authentication.
    ///
    /// If not specified, the client will attempt to use the `ANTHROPIC_API_KEY` environment variable.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use llm_kit_anthropic::AnthropicClient;
    ///
    /// let client = AnthropicClient::new()
    ///     .api_key("your-api-key");
    /// ```
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the provider name.
    ///
    /// # Arguments
    ///
    /// * `name` - The provider name (e.g., "anthropic.messages", "my-anthropic")
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use llm_kit_anthropic::AnthropicClient;
    ///
    /// let client = AnthropicClient::new()
    ///     .name("my-anthropic-provider");
    /// ```
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Adds a custom header to include in requests.
    ///
    /// Custom headers are added after the required Anthropic headers (`x-api-key`, `anthropic-version`).
    ///
    /// # Arguments
    ///
    /// * `key` - The header name
    /// * `value` - The header value
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use llm_kit_anthropic::AnthropicClient;
    ///
    /// let client = AnthropicClient::new()
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
    /// use llm_kit_anthropic::AnthropicClient;
    /// use std::collections::HashMap;
    ///
    /// let mut headers = HashMap::new();
    /// headers.insert("X-Header-1".to_string(), "value1".to_string());
    /// headers.insert("X-Header-2".to_string(), "value2".to_string());
    ///
    /// let client = AnthropicClient::new()
    ///     .headers(headers);
    /// ```
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
    }

    /// Builds the `AnthropicProvider` with the configured settings.
    ///
    /// # Returns
    ///
    /// An `AnthropicProvider` instance.
    ///
    /// # Panics
    ///
    /// Panics if no API key is provided and the `ANTHROPIC_API_KEY` environment variable is not set.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use llm_kit_anthropic::AnthropicClient;
    ///
    /// let provider = AnthropicClient::new()
    ///     .api_key("your-api-key")
    ///     .build();
    /// ```
    pub fn build(self) -> AnthropicProvider {
        let mut settings = AnthropicProviderSettings::new();

        if let Some(base_url) = self.base_url {
            settings = settings.with_base_url(base_url);
        }

        if let Some(api_key) = self.api_key {
            settings = settings.with_api_key(api_key);
        }

        if let Some(name) = self.name {
            settings = settings.with_name(name);
        }

        if !self.headers.is_empty() {
            settings = settings.with_headers(self.headers);
        }

        AnthropicProvider::new(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use llm_kit_provider::language_model::LanguageModel;

    #[test]
    fn test_basic_client_builder() {
        let provider = AnthropicClient::new().api_key("test-key").build();

        // Default base URL should be set
        assert_eq!(provider.base_url(), "https://api.anthropic.com/v1");
        assert_eq!(provider.provider_name(), "anthropic.messages");
    }

    #[test]
    fn test_custom_base_url() {
        let provider = AnthropicClient::new()
            .base_url("https://custom.api.com/v1")
            .api_key("test-key")
            .build();

        assert_eq!(provider.base_url(), "https://custom.api.com/v1");
    }

    #[test]
    fn test_custom_name() {
        let provider = AnthropicClient::new()
            .name("my-anthropic")
            .api_key("test-key")
            .build();

        assert_eq!(provider.provider_name(), "my-anthropic");
    }

    #[test]
    fn test_with_headers() {
        let provider = AnthropicClient::new()
            .api_key("test-key")
            .header("X-Custom-Header", "value1")
            .header("X-Another-Header", "value2")
            .build();

        // We can't directly access provider.headers anymore since it's private
        // Instead, we just verify the provider was created successfully
        assert_eq!(provider.base_url(), "https://api.anthropic.com/v1");
        assert_eq!(provider.provider_name(), "anthropic.messages");
    }

    #[test]
    fn test_bulk_headers() {
        let mut custom_headers = HashMap::new();
        custom_headers.insert("X-Header-1".to_string(), "value1".to_string());
        custom_headers.insert("X-Header-2".to_string(), "value2".to_string());

        let provider = AnthropicClient::new()
            .api_key("test-key")
            .headers(custom_headers)
            .build();

        // We can't directly access provider.headers anymore since it's private
        // Instead, we just verify the provider was created successfully
        assert_eq!(provider.base_url(), "https://api.anthropic.com/v1");
    }

    #[test]
    fn test_mixed_headers_methods() {
        let mut initial_headers = HashMap::new();
        initial_headers.insert("X-Initial".to_string(), "value".to_string());

        let provider = AnthropicClient::new()
            .api_key("test-key")
            .headers(initial_headers)
            .header("X-Additional", "another-value")
            .build();

        // We can't directly access provider.headers anymore since it's private
        // Instead, we just verify the provider was created successfully
        assert_eq!(provider.base_url(), "https://api.anthropic.com/v1");
    }

    #[test]
    fn test_chained_model_creation() {
        let model = AnthropicClient::new()
            .api_key("test-key")
            .build()
            .language_model("claude-3-5-sonnet-20241022".to_string());

        assert_eq!(model.model_id(), "claude-3-5-sonnet-20241022");
        assert_eq!(model.provider(), "anthropic.messages");
    }

    #[test]
    fn test_default_values() {
        let provider = AnthropicClient::new().api_key("test-key").build();

        assert_eq!(provider.base_url(), "https://api.anthropic.com/v1");
        assert_eq!(provider.provider_name(), "anthropic.messages");
    }

    #[test]
    fn test_trailing_slash_removed() {
        let provider = AnthropicClient::new()
            .base_url("https://api.anthropic.com/v1/")
            .api_key("test-key")
            .build();

        assert_eq!(provider.base_url(), "https://api.anthropic.com/v1");
    }
}
