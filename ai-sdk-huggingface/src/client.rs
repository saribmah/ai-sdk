use std::collections::HashMap;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_config() {
        let config = HuggingFaceClientConfig::new(
            "test-provider",
            Box::new(|_model_id, path| format!("https://api.test.com{}", path)),
            Box::new(|| {
                let mut headers = HashMap::new();
                headers.insert("Authorization".to_string(), "Bearer test".to_string());
                headers
            }),
        );

        assert_eq!(config.provider, "test-provider");
        assert_eq!((config.url)("model", "/test"), "https://api.test.com/test");

        let headers = (config.headers)();
        assert_eq!(
            headers.get("Authorization"),
            Some(&"Bearer test".to_string())
        );
    }
}
