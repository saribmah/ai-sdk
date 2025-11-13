use std::collections::HashMap;

/// Settings for the Hugging Face provider.
#[derive(Debug, Clone)]
pub struct HuggingFaceProviderSettings {
    /// API key for authentication. If not provided, will try to load from
    /// HUGGINGFACE_API_KEY environment variable.
    pub api_key: Option<String>,

    /// Base URL for the Hugging Face API.
    /// Defaults to "https://router.huggingface.co/v1".
    pub base_url: String,

    /// Custom headers to include in all requests.
    pub headers: Option<HashMap<String, String>>,
}

impl HuggingFaceProviderSettings {
    /// Creates a new `HuggingFaceProviderSettings` with default values.
    pub fn new() -> Self {
        Self {
            api_key: None,
            base_url: "https://router.huggingface.co/v1".to_string(),
            headers: None,
        }
    }

    /// Sets the API key.
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the base URL.
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        let url = base_url.into();
        // Remove trailing slash if present
        self.base_url = url.trim_end_matches('/').to_string();
        self
    }

    /// Adds a custom header.
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers
            .get_or_insert_with(HashMap::new)
            .insert(key.into(), value.into());
        self
    }

    /// Loads API key from environment variable if not already set.
    pub fn load_api_key_from_env(mut self) -> Self {
        if self.api_key.is_none() {
            self.api_key = std::env::var("HUGGINGFACE_API_KEY").ok();
        }
        self
    }
}

impl Default for HuggingFaceProviderSettings {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = HuggingFaceProviderSettings::default();
        assert_eq!(settings.base_url, "https://router.huggingface.co/v1");
        assert!(settings.api_key.is_none());
        assert!(settings.headers.is_none());
    }

    #[test]
    fn test_with_api_key() {
        let settings = HuggingFaceProviderSettings::new().with_api_key("test-key");
        assert_eq!(settings.api_key, Some("test-key".to_string()));
    }

    #[test]
    fn test_with_base_url() {
        let settings = HuggingFaceProviderSettings::new().with_base_url("https://custom.api.com/");
        assert_eq!(settings.base_url, "https://custom.api.com");
    }

    #[test]
    fn test_with_header() {
        let settings = HuggingFaceProviderSettings::new().with_header("X-Custom", "value");

        let headers = settings.headers.unwrap();
        assert_eq!(headers.get("X-Custom"), Some(&"value".to_string()));
    }
}
