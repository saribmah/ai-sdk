use std::collections::HashMap;

/// Internal configuration for ElevenLabs models.
#[derive(Clone)]
pub struct ElevenLabsConfig {
    /// Provider identifier (e.g., "elevenlabs.speech", "elevenlabs.transcription")
    pub provider: String,

    /// Base URL for API requests
    pub base_url: String,

    /// Headers to include in all requests
    pub headers: HashMap<String, String>,
}

impl ElevenLabsConfig {
    /// Create a new config.
    pub fn new(
        provider: impl Into<String>,
        base_url: impl Into<String>,
        headers: HashMap<String, String>,
    ) -> Self {
        Self {
            provider: provider.into(),
            base_url: base_url.into(),
            headers,
        }
    }

    /// Build the full URL for a given path.
    pub fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    /// Get headers as a reference.
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
}
