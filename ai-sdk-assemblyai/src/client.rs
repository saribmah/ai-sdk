use crate::provider::AssemblyAIProvider;
use crate::settings::AssemblyAIProviderSettings;
use std::collections::HashMap;

/// Builder for creating an AssemblyAI provider.
///
/// Provides a fluent API for constructing an `AssemblyAIProvider` with various configuration options.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```no_run
/// use ai_sdk_assemblyai::AssemblyAIClient;
///
/// let provider = AssemblyAIClient::new()
///     .api_key("your-api-key")
///     .build();
///
/// let model = provider.transcription_model("best");
/// ```
///
/// ## Custom Configuration
///
/// ```no_run
/// use ai_sdk_assemblyai::AssemblyAIClient;
///
/// let provider = AssemblyAIClient::new()
///     .api_key("your-api-key")
///     .header("X-Custom-Header", "value")
///     .polling_interval_ms(5000)
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct AssemblyAIClient {
    api_key: Option<String>,
    headers: HashMap<String, String>,
    base_url: Option<String>,
    polling_interval_ms: Option<u64>,
}

impl AssemblyAIClient {
    /// Creates a new client builder with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the API key for authentication.
    ///
    /// If not provided, the API key will be loaded from the `ASSEMBLYAI_API_KEY` environment variable.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key
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

    /// Sets the base URL for API calls (for testing or custom endpoints).
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL (e.g., "<https://api.assemblyai.com>")
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Sets the polling interval in milliseconds for checking transcription status.
    ///
    /// Default is 3000ms (3 seconds).
    ///
    /// # Arguments
    ///
    /// * `interval_ms` - The polling interval in milliseconds
    pub fn polling_interval_ms(mut self, interval_ms: u64) -> Self {
        self.polling_interval_ms = Some(interval_ms);
        self
    }

    /// Builds the `AssemblyAIProvider` with the configured settings.
    ///
    /// # Returns
    ///
    /// An `AssemblyAIProvider` instance.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ai_sdk_assemblyai::AssemblyAIClient;
    ///
    /// let provider = AssemblyAIClient::new()
    ///     .api_key("your-api-key")
    ///     .build();
    /// ```
    pub fn build(self) -> AssemblyAIProvider {
        let mut settings = AssemblyAIProviderSettings::new();

        if let Some(api_key) = self.api_key {
            settings = settings.with_api_key(api_key);
        }

        if !self.headers.is_empty() {
            settings = settings.with_headers(self.headers);
        }

        if let Some(base_url) = self.base_url {
            settings = settings.with_base_url(base_url);
        }

        if let Some(polling_interval_ms) = self.polling_interval_ms {
            settings = settings.with_polling_interval_ms(polling_interval_ms);
        }

        AssemblyAIProvider::new(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_client_builder() {
        let provider = AssemblyAIClient::new().api_key("test-key").build();

        assert_eq!(provider.base_url(), "https://api.assemblyai.com");
    }

    #[test]
    fn test_custom_configuration() {
        let provider = AssemblyAIClient::new()
            .api_key("test-key")
            .header("X-Custom", "value")
            .polling_interval_ms(5000)
            .base_url("https://custom.example.com")
            .build();

        assert_eq!(provider.base_url(), "https://custom.example.com");
    }

    #[test]
    fn test_default_values() {
        let provider = AssemblyAIClient::new().build();
        assert_eq!(provider.base_url(), "https://api.assemblyai.com");
    }
}
