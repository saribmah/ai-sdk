use std::collections::HashMap;

/// Settings for creating an ElevenLabs provider.
#[derive(Debug, Clone)]
pub struct ElevenLabsProviderSettings {
    /// API key for authenticating requests.
    /// If not provided, will attempt to load from ELEVENLABS_API_KEY environment variable.
    pub api_key: Option<String>,

    /// Custom headers to include in requests.
    pub headers: Option<HashMap<String, String>>,

    /// Base URL for the ElevenLabs API.
    /// Defaults to "https://api.elevenlabs.io".
    pub base_url: Option<String>,
}

impl ElevenLabsProviderSettings {
    /// Create new provider settings.
    pub fn new() -> Self {
        Self {
            api_key: None,
            headers: None,
            base_url: None,
        }
    }

    /// Set the API key.
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Set custom headers.
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Add a custom header.
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let headers = self.headers.get_or_insert_with(HashMap::new);
        headers.insert(key.into(), value.into());
        self
    }

    /// Set the base URL.
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    /// Get the base URL (defaults to ElevenLabs API URL).
    pub fn get_base_url(&self) -> String {
        self.base_url
            .clone()
            .unwrap_or_else(|| "https://api.elevenlabs.io".to_string())
    }

    /// Get the API key from settings or environment variable.
    pub fn get_api_key(&self) -> Result<String, String> {
        if let Some(key) = &self.api_key {
            return Ok(key.clone());
        }

        std::env::var("ELEVENLABS_API_KEY")
            .map_err(|_| "ElevenLabs API key not found. Set ELEVENLABS_API_KEY environment variable or provide api_key in settings.".to_string())
    }
}

impl Default for ElevenLabsProviderSettings {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = ElevenLabsProviderSettings::new();
        assert!(settings.api_key.is_none());
        assert!(settings.headers.is_none());
        assert_eq!(settings.get_base_url(), "https://api.elevenlabs.io");
    }

    #[test]
    fn test_with_api_key() {
        let settings = ElevenLabsProviderSettings::new().with_api_key("test-key");
        assert_eq!(settings.api_key, Some("test-key".to_string()));
    }

    #[test]
    fn test_with_header() {
        let settings = ElevenLabsProviderSettings::new().with_header("X-Custom", "value");
        assert_eq!(
            settings.headers.unwrap().get("X-Custom"),
            Some(&"value".to_string())
        );
    }

    #[test]
    fn test_with_base_url() {
        let settings = ElevenLabsProviderSettings::new().with_base_url("https://custom.api.com");
        assert_eq!(settings.get_base_url(), "https://custom.api.com");
    }
}
