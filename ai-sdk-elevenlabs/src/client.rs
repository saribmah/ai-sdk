use crate::provider::ElevenLabsProvider;
use crate::settings::ElevenLabsProviderSettings;
use std::collections::HashMap;

/// Builder for creating an ElevenLabs provider.
///
/// # Examples
///
/// ```no_run
/// use ai_sdk_elevenlabs::ElevenLabsClient;
/// use ai_sdk_provider::Provider;
///
/// // Create provider with API key
/// let provider = ElevenLabsClient::new()
///     .api_key("your-api-key")
///     .build();
///
/// // Get a speech model
/// let model = provider.speech_model("eleven_multilingual_v2")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ```no_run
/// use ai_sdk_elevenlabs::ElevenLabsClient;
///
/// // Create provider with custom configuration
/// let provider = ElevenLabsClient::new()
///     .api_key("your-api-key")
///     .base_url("https://custom-api.elevenlabs.io")
///     .header("X-Custom-Header", "custom-value")
///     .build();
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, Default)]
pub struct ElevenLabsClient {
    base_url: Option<String>,
    api_key: Option<String>,
    headers: HashMap<String, String>,
}

impl ElevenLabsClient {
    /// Create a new ElevenLabs client builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_elevenlabs::ElevenLabsClient;
    ///
    /// let client = ElevenLabsClient::new();
    /// ```
    pub fn new() -> Self {
        Self {
            base_url: None,
            api_key: None,
            headers: HashMap::new(),
        }
    }

    /// Set the API key.
    ///
    /// If not provided, the API key will be loaded from the `ELEVENLABS_API_KEY`
    /// environment variable when the provider is first used.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_elevenlabs::ElevenLabsClient;
    ///
    /// let client = ElevenLabsClient::new()
    ///     .api_key("your-api-key");
    /// ```
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Set the base URL for the API.
    ///
    /// Defaults to `https://api.elevenlabs.io`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_elevenlabs::ElevenLabsClient;
    ///
    /// let client = ElevenLabsClient::new()
    ///     .base_url("https://custom-api.elevenlabs.io");
    /// ```
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Add a custom header to all requests.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_elevenlabs::ElevenLabsClient;
    ///
    /// let client = ElevenLabsClient::new()
    ///     .header("X-Custom-Header", "custom-value");
    /// ```
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Set multiple custom headers at once.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_elevenlabs::ElevenLabsClient;
    /// use std::collections::HashMap;
    ///
    /// let mut headers = HashMap::new();
    /// headers.insert("X-Custom-Header".to_string(), "custom-value".to_string());
    ///
    /// let client = ElevenLabsClient::new()
    ///     .headers(headers);
    /// ```
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
    }

    /// Build the ElevenLabs provider.
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_elevenlabs::ElevenLabsClient;
    ///
    /// let provider = ElevenLabsClient::new()
    ///     .api_key("your-api-key")
    ///     .build();
    /// ```
    pub fn build(self) -> ElevenLabsProvider {
        let mut settings = ElevenLabsProviderSettings::new();

        if let Some(base_url) = self.base_url {
            settings = settings.with_base_url(base_url);
        }

        if let Some(api_key) = self.api_key {
            settings = settings.with_api_key(api_key);
        }

        if !self.headers.is_empty() {
            settings = settings.with_headers(self.headers);
        }

        ElevenLabsProvider::new(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_sdk_provider::Provider;

    #[test]
    fn test_builder_new() {
        let client = ElevenLabsClient::new();
        let provider = client.build();
        assert_eq!(provider.specification_version(), "v3");
    }

    #[test]
    fn test_builder_with_api_key() {
        let provider = ElevenLabsClient::new().api_key("test-api-key").build();

        // Verify the provider was created
        assert_eq!(provider.specification_version(), "v3");
    }

    #[test]
    fn test_builder_with_base_url() {
        let provider = ElevenLabsClient::new()
            .base_url("https://custom.api.com")
            .build();

        assert_eq!(provider.specification_version(), "v3");
    }

    #[test]
    fn test_builder_with_header() {
        let provider = ElevenLabsClient::new().header("X-Custom", "value").build();

        assert_eq!(provider.specification_version(), "v3");
    }

    #[test]
    fn test_builder_with_headers() {
        let mut headers = HashMap::new();
        headers.insert("X-Custom-1".to_string(), "value-1".to_string());
        headers.insert("X-Custom-2".to_string(), "value-2".to_string());

        let provider = ElevenLabsClient::new().headers(headers).build();

        assert_eq!(provider.specification_version(), "v3");
    }

    #[test]
    fn test_builder_chaining() {
        let provider = ElevenLabsClient::new()
            .api_key("test-key")
            .base_url("https://custom.api.com")
            .header("X-Custom", "value")
            .build();

        assert_eq!(provider.specification_version(), "v3");
    }

    #[test]
    fn test_builder_with_mixed_headers() {
        let mut initial_headers = HashMap::new();
        initial_headers.insert("X-Initial".to_string(), "value".to_string());

        let provider = ElevenLabsClient::new()
            .api_key("test-key")
            .headers(initial_headers)
            .header("X-Additional", "another-value")
            .build();

        assert_eq!(provider.specification_version(), "v3");
    }
}
