use std::collections::HashMap;

/// Settings for configuring an AssemblyAI provider.
#[derive(Debug, Clone)]
pub struct AssemblyAIProviderSettings {
    /// API key for authenticating requests.
    pub api_key: Option<String>,

    /// Custom headers to include in requests.
    pub headers: Option<HashMap<String, String>>,

    /// Base URL for AssemblyAI API (default: "<https://api.assemblyai.com>")
    pub base_url: String,

    /// Polling interval in milliseconds for checking transcription status (default: 3000)
    pub polling_interval_ms: u64,
}

impl AssemblyAIProviderSettings {
    /// Creates new settings with default values.
    pub fn new() -> Self {
        Self {
            api_key: None,
            headers: None,
            base_url: "https://api.assemblyai.com".to_string(),
            polling_interval_ms: 3000,
        }
    }

    /// Sets the API key.
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets a custom header.
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    /// Sets multiple custom headers at once.
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers
            .get_or_insert_with(HashMap::new)
            .extend(headers);
        self
    }

    /// Sets the base URL (for testing or custom endpoints).
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Sets the polling interval in milliseconds.
    pub fn with_polling_interval_ms(mut self, interval_ms: u64) -> Self {
        self.polling_interval_ms = interval_ms;
        self
    }
}

impl Default for AssemblyAIProviderSettings {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = AssemblyAIProviderSettings::new();
        assert_eq!(settings.base_url, "https://api.assemblyai.com");
        assert_eq!(settings.polling_interval_ms, 3000);
        assert!(settings.api_key.is_none());
        assert!(settings.headers.is_none());
    }

    #[test]
    fn test_settings_builder() {
        let settings = AssemblyAIProviderSettings::new()
            .with_api_key("test-key")
            .with_header("X-Custom", "value")
            .with_polling_interval_ms(5000);

        assert_eq!(settings.api_key, Some("test-key".to_string()));
        assert_eq!(settings.polling_interval_ms, 5000);
        assert!(settings.headers.is_some());
        assert_eq!(
            settings.headers.unwrap().get("X-Custom"),
            Some(&"value".to_string())
        );
    }
}
