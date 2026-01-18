use std::collections::HashMap;
use std::sync::Arc;

/// Type alias for header generation function
pub type HeadersFn = Arc<dyn Fn() -> HashMap<String, String> + Send + Sync>;

/// Type alias for URL generation function
pub type BuildRequestUrlFn = Arc<dyn Fn(&str, bool) -> String + Send + Sync>;

/// Type alias for request body transformation function
pub type TransformRequestBodyFn = Arc<dyn Fn(serde_json::Value) -> serde_json::Value + Send + Sync>;

/// Type alias for ID generation function
pub type GenerateIdFn = Arc<dyn Fn() -> String + Send + Sync>;

/// Configuration for Anthropic Messages API language model
#[derive(Clone)]
pub struct AnthropicMessagesConfig {
    /// Provider name (e.g., "anthropic")
    pub provider: String,

    /// Base URL for API requests
    pub base_url: String,

    /// Function to generate headers for API requests
    pub headers: HeadersFn,

    /// Optional function to build the request URL
    pub build_request_url: Option<BuildRequestUrlFn>,

    /// Optional function to transform the request body before sending
    pub transform_request_body: Option<TransformRequestBodyFn>,

    /// Optional function to generate unique IDs
    pub generate_id: Option<GenerateIdFn>,
}

impl AnthropicMessagesConfig {
    /// Create a new configuration
    pub fn new(provider: String, base_url: String, headers: HeadersFn) -> Self {
        Self {
            provider,
            base_url,
            headers,
            build_request_url: None,
            transform_request_body: None,
            generate_id: None,
        }
    }

    /// Set the request URL builder
    pub fn with_build_request_url(mut self, builder: BuildRequestUrlFn) -> Self {
        self.build_request_url = Some(builder);
        self
    }

    /// Set the request body transformer
    pub fn with_transform_request_body(mut self, transformer: TransformRequestBodyFn) -> Self {
        self.transform_request_body = Some(transformer);
        self
    }

    /// Set the ID generator
    pub fn with_generate_id(mut self, generator: GenerateIdFn) -> Self {
        self.generate_id = Some(generator);
        self
    }
}
