use crate::provider::XaiProvider;
use crate::settings::XaiProviderSettings;
use std::collections::HashMap;

/// Builder for creating xAI providers.
///
/// # Examples
///
/// ```no_run
/// use ai_sdk_xai::XaiClient;
///
/// let provider = XaiClient::new()
///     .api_key("your-api-key")
///     .build();
///
/// let model = provider.chat_model("grok-4");
/// ```
#[derive(Debug, Default)]
pub struct XaiClient {
    base_url: Option<String>,
    api_key: Option<String>,
    headers: Option<HashMap<String, String>>,
}

impl XaiClient {
    /// Creates a new xAI client builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the base URL for the xAI API.
    ///
    /// Default: `https://api.x.ai/v1`
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Sets the API key for authentication.
    ///
    /// Can also be set via the `XAI_API_KEY` environment variable.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets custom headers for all requests.
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Adds a single custom header.
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let mut headers = self.headers.unwrap_or_default();
        headers.insert(key.into(), value.into());
        self.headers = Some(headers);
        self
    }

    /// Builds the xAI provider.
    pub fn build(self) -> XaiProvider {
        let mut settings = XaiProviderSettings::new();

        if let Some(base_url) = self.base_url {
            settings = settings.with_base_url(base_url);
        }

        // Try API key from builder, then environment variable
        let api_key = self.api_key.or_else(|| std::env::var("XAI_API_KEY").ok());
        if let Some(key) = api_key {
            settings = settings.with_api_key(key);
        }

        if let Some(headers) = self.headers {
            settings = settings.with_headers(headers);
        }

        XaiProvider::new(settings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_builder() {
        let provider = XaiClient::new()
            .api_key("test-key")
            .header("X-Custom", "value")
            .build();

        assert_eq!(provider.base_url(), "https://api.x.ai/v1");
    }

    #[test]
    fn test_client_with_custom_base_url() {
        let provider = XaiClient::new()
            .base_url("https://custom.x.ai/v1")
            .api_key("test-key")
            .build();

        assert_eq!(provider.base_url(), "https://custom.x.ai/v1");
    }

    #[test]
    fn test_client_chained() {
        let model = XaiClient::new()
            .api_key("test-key")
            .build()
            .chat_model("grok-4");

        assert_eq!(model.provider(), "xai.chat");
        assert_eq!(model.model_id(), "grok-4");
    }
}
