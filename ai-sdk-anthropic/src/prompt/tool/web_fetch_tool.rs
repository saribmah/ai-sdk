use serde::{Deserialize, Serialize};

use crate::prompt::message::cache_control::AnthropicCacheControl;
use crate::prompt::message::content::document::DocumentCitations;

/// Web fetch tool (version 20250910).
///
/// This tool allows the model to fetch content from web URLs with various
/// configuration options including domain filtering, citations, and content limits.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::prompt::tool::web_fetch_tool::AnthropicWebFetchTool;
///
/// let tool = AnthropicWebFetchTool::new("web_fetch")
///     .with_max_uses(10)
///     .with_allowed_domain("example.com")
///     .with_citations_enabled();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicWebFetchTool {
    /// The type of tool (always "web_fetch_20250910")
    #[serde(rename = "type")]
    pub tool_type: String,

    /// The name of the tool
    pub name: String,

    /// Optional maximum number of times this tool can be used in a conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<u32>,

    /// Optional list of allowed domains (e.g., ["example.com", "*.github.com"])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_domains: Option<Vec<String>>,

    /// Optional list of blocked domains (e.g., ["blocked.com"])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_domains: Option<Vec<String>>,

    /// Optional citations configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citations: Option<DocumentCitations>,

    /// Optional maximum number of content tokens to fetch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_content_tokens: Option<u32>,

    /// Optional cache control configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}

impl AnthropicWebFetchTool {
    /// Creates a new web fetch tool.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the tool
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::tool::web_fetch_tool::AnthropicWebFetchTool;
    ///
    /// let tool = AnthropicWebFetchTool::new("web_fetch");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            tool_type: "web_fetch_20250910".to_string(),
            name: name.into(),
            max_uses: None,
            allowed_domains: None,
            blocked_domains: None,
            citations: None,
            max_content_tokens: None,
            cache_control: None,
        }
    }

    /// Sets the maximum number of times this tool can be used.
    pub fn with_max_uses(mut self, max_uses: u32) -> Self {
        self.max_uses = Some(max_uses);
        self
    }

    /// Removes the max_uses limit.
    pub fn without_max_uses(mut self) -> Self {
        self.max_uses = None;
        self
    }

    /// Sets the allowed domains.
    pub fn with_allowed_domains(mut self, domains: Vec<String>) -> Self {
        self.allowed_domains = Some(domains);
        self
    }

    /// Adds a single allowed domain.
    pub fn with_allowed_domain(mut self, domain: impl Into<String>) -> Self {
        let mut domains = self.allowed_domains.unwrap_or_default();
        domains.push(domain.into());
        self.allowed_domains = Some(domains);
        self
    }

    /// Removes the allowed domains restriction.
    pub fn without_allowed_domains(mut self) -> Self {
        self.allowed_domains = None;
        self
    }

    /// Sets the blocked domains.
    pub fn with_blocked_domains(mut self, domains: Vec<String>) -> Self {
        self.blocked_domains = Some(domains);
        self
    }

    /// Adds a single blocked domain.
    pub fn with_blocked_domain(mut self, domain: impl Into<String>) -> Self {
        let mut domains = self.blocked_domains.unwrap_or_default();
        domains.push(domain.into());
        self.blocked_domains = Some(domains);
        self
    }

    /// Removes the blocked domains restriction.
    pub fn without_blocked_domains(mut self) -> Self {
        self.blocked_domains = None;
        self
    }

    /// Sets the citations configuration.
    pub fn with_citations(mut self, citations: DocumentCitations) -> Self {
        self.citations = Some(citations);
        self
    }

    /// Enables citations.
    pub fn with_citations_enabled(self) -> Self {
        self.with_citations(DocumentCitations::enabled())
    }

    /// Disables citations.
    pub fn with_citations_disabled(self) -> Self {
        self.with_citations(DocumentCitations::disabled())
    }

    /// Removes the citations configuration.
    pub fn without_citations(mut self) -> Self {
        self.citations = None;
        self
    }

    /// Sets the maximum content tokens.
    pub fn with_max_content_tokens(mut self, max_tokens: u32) -> Self {
        self.max_content_tokens = Some(max_tokens);
        self
    }

    /// Removes the max_content_tokens limit.
    pub fn without_max_content_tokens(mut self) -> Self {
        self.max_content_tokens = None;
        self
    }

    /// Sets the cache control for this tool.
    pub fn with_cache_control(mut self, cache_control: AnthropicCacheControl) -> Self {
        self.cache_control = Some(cache_control);
        self
    }

    /// Removes cache control from this tool.
    pub fn without_cache_control(mut self) -> Self {
        self.cache_control = None;
        self
    }

    /// Updates the name of this tool.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    use serde_json::json;

    #[test]
    fn test_new() {
        let tool = AnthropicWebFetchTool::new("web_fetch");

        assert_eq!(tool.tool_type, "web_fetch_20250910");
        assert_eq!(tool.name, "web_fetch");
        assert!(tool.max_uses.is_none());
        assert!(tool.allowed_domains.is_none());
        assert!(tool.blocked_domains.is_none());
        assert!(tool.citations.is_none());
        assert!(tool.max_content_tokens.is_none());
        assert!(tool.cache_control.is_none());
    }

    #[test]
    fn test_with_max_uses() {
        let tool = AnthropicWebFetchTool::new("fetch").with_max_uses(5);

        assert_eq!(tool.max_uses, Some(5));
    }

    #[test]
    fn test_with_allowed_domains() {
        let tool = AnthropicWebFetchTool::new("fetch")
            .with_allowed_domains(vec!["example.com".to_string(), "*.github.com".to_string()]);

        assert_eq!(
            tool.allowed_domains,
            Some(vec!["example.com".to_string(), "*.github.com".to_string()])
        );
    }

    #[test]
    fn test_with_allowed_domain() {
        let tool = AnthropicWebFetchTool::new("fetch")
            .with_allowed_domain("example.com")
            .with_allowed_domain("github.com");

        assert_eq!(
            tool.allowed_domains,
            Some(vec!["example.com".to_string(), "github.com".to_string()])
        );
    }

    #[test]
    fn test_with_blocked_domain() {
        let tool = AnthropicWebFetchTool::new("fetch").with_blocked_domain("blocked.com");

        assert_eq!(tool.blocked_domains, Some(vec!["blocked.com".to_string()]));
    }

    #[test]
    fn test_with_citations_enabled() {
        let tool = AnthropicWebFetchTool::new("fetch").with_citations_enabled();

        assert_eq!(tool.citations, Some(DocumentCitations::enabled()));
    }

    #[test]
    fn test_with_max_content_tokens() {
        let tool = AnthropicWebFetchTool::new("fetch").with_max_content_tokens(1000);

        assert_eq!(tool.max_content_tokens, Some(1000));
    }

    #[test]
    fn test_serialize_minimal() {
        let tool = AnthropicWebFetchTool::new("web_fetch");
        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "web_fetch_20250910",
                "name": "web_fetch"
            })
        );
    }

    #[test]
    fn test_serialize_full() {
        let tool = AnthropicWebFetchTool::new("web_fetch")
            .with_max_uses(10)
            .with_allowed_domain("example.com")
            .with_blocked_domain("blocked.com")
            .with_citations_enabled()
            .with_max_content_tokens(2000)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "web_fetch_20250910",
                "name": "web_fetch",
                "max_uses": 10,
                "allowed_domains": ["example.com"],
                "blocked_domains": ["blocked.com"],
                "citations": {
                    "enabled": true
                },
                "max_content_tokens": 2000,
                "cache_control": {
                    "type": "ephemeral",
                    "ttl": "1h"
                }
            })
        );
    }

    #[test]
    fn test_deserialize() {
        let json = json!({
            "type": "web_fetch_20250910",
            "name": "my_fetch",
            "max_uses": 5,
            "allowed_domains": ["example.com"],
            "citations": {
                "enabled": false
            },
            "max_content_tokens": 1500,
            "cache_control": {
                "type": "ephemeral",
                "ttl": "5m"
            }
        });

        let tool: AnthropicWebFetchTool = serde_json::from_value(json).unwrap();

        assert_eq!(tool.name, "my_fetch");
        assert_eq!(tool.max_uses, Some(5));
        assert_eq!(tool.allowed_domains, Some(vec!["example.com".to_string()]));
        assert_eq!(tool.citations, Some(DocumentCitations::disabled()));
        assert_eq!(tool.max_content_tokens, Some(1500));
    }

    #[test]
    fn test_roundtrip() {
        let original = AnthropicWebFetchTool::new("web_fetch")
            .with_max_uses(10)
            .with_allowed_domain("example.com")
            .with_citations_enabled()
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicWebFetchTool = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }
}
