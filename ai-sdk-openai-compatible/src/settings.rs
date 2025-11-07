use std::collections::HashMap;

/// Configuration options for creating an OpenAI-compatible provider.
#[derive(Debug, Clone)]
pub struct OpenAICompatibleProviderSettings {
    /// Base URL for the API calls (e.g., "<https://api.openai.com/v1>")
    pub base_url: String,

    /// Provider name (e.g., "openai", "azure-openai", "custom")
    pub name: String,

    /// API key for authenticating requests. If specified, adds an `Authorization`
    /// header with the value `Bearer <apiKey>`.
    pub api_key: Option<String>,

    /// Optional organization ID
    pub organization: Option<String>,

    /// Optional project ID
    pub project: Option<String>,

    /// Optional custom headers to include in requests. These will be added to request headers
    /// after any headers potentially added by use of the `api_key` option.
    pub headers: Option<HashMap<String, String>>,

    /// Optional custom URL query parameters to include in request URLs.
    pub query_params: Option<HashMap<String, String>>,

    /// Include usage information in streaming responses.
    pub include_usage: bool,

    /// Whether the provider supports structured outputs in chat models.
    pub supports_structured_outputs: bool,
}

impl Default for OpenAICompatibleProviderSettings {
    fn default() -> Self {
        Self {
            base_url: "https://api.openai.com/v1".to_string(),
            name: "openai".to_string(),
            api_key: None,
            organization: None,
            project: None,
            headers: None,
            query_params: None,
            include_usage: false,
            supports_structured_outputs: false,
        }
    }
}

impl OpenAICompatibleProviderSettings {
    /// Creates a new configuration with the given base URL and name.
    pub fn new(base_url: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            name: name.into(),
            api_key: None,
            organization: None,
            project: None,
            headers: None,
            query_params: None,
            include_usage: false,
            supports_structured_outputs: false,
        }
    }

    /// Sets the API key.
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Sets the organization ID.
    pub fn with_organization(mut self, organization: impl Into<String>) -> Self {
        self.organization = Some(organization.into());
        self
    }

    /// Sets the project ID.
    pub fn with_project(mut self, project: impl Into<String>) -> Self {
        self.project = Some(project.into());
        self
    }

    /// Sets additional headers.
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Adds a single header.
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let mut headers = self.headers.unwrap_or_default();
        headers.insert(key.into(), value.into());
        self.headers = Some(headers);
        self
    }

    /// Sets query parameters.
    pub fn with_query_params(mut self, query_params: HashMap<String, String>) -> Self {
        self.query_params = Some(query_params);
        self
    }

    /// Adds a single query parameter.
    pub fn with_query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let mut query_params = self.query_params.unwrap_or_default();
        query_params.insert(key.into(), value.into());
        self.query_params = Some(query_params);
        self
    }

    /// Sets whether to include usage information in streaming responses.
    pub fn with_include_usage(mut self, include_usage: bool) -> Self {
        self.include_usage = include_usage;
        self
    }

    /// Sets whether the provider supports structured outputs.
    pub fn with_supports_structured_outputs(mut self, supports: bool) -> Self {
        self.supports_structured_outputs = supports;
        self
    }
}
