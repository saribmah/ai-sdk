//! Anthropic web fetch tool (version 20250910).
//!
//! This module provides a factory for creating web fetch tools that allow
//! the model to fetch content from URLs. Unlike web search, citations are
//! optional for web fetch.
//!
//! # Example
//!
//! ```
//! use ai_sdk_anthropic::provider_tool::web_fetch_20250910;
//!
//! // Create a web fetch tool with default options
//! let builder = web_fetch_20250910();
//! let tool = builder.build();
//!
//! // Create with domain restrictions
//! let tool = web_fetch_20250910()
//!     .allowed_domains(vec!["example.com".to_string(), "docs.rs".to_string()])
//!     .max_uses(10)
//!     .build();
//! ```

use ai_sdk_provider_utils::tool::{
    ProviderDefinedToolFactoryWithOutput, ProviderDefinedToolOptions, Tool,
};
use serde_json::json;

/// Builder for creating a web fetch tool (version 20250910).
///
/// This builder allows configuring various parameters for web fetching including
/// domain restrictions, usage limits, citations, and content limits.
#[derive(Clone, Debug, Default)]
pub struct WebFetch20250910Builder {
    max_uses: Option<i64>,
    allowed_domains: Option<Vec<String>>,
    blocked_domains: Option<Vec<String>>,
    citations_enabled: Option<bool>,
    max_content_tokens: Option<i64>,
    description: Option<String>,
    needs_approval: bool,
}

impl WebFetch20250910Builder {
    /// Creates a new builder with default settings.
    fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum number of web fetches allowed.
    ///
    /// # Arguments
    ///
    /// * `max_uses` - Maximum number of fetches
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::provider_tool::web_fetch_20250910;
    ///
    /// let tool = web_fetch_20250910()
    ///     .max_uses(10)
    ///     .build();
    /// ```
    pub fn max_uses(mut self, max_uses: i64) -> Self {
        self.max_uses = Some(max_uses);
        self
    }

    /// Sets the allowed domains for fetching.
    ///
    /// Only URLs from these domains will be fetched.
    ///
    /// # Arguments
    ///
    /// * `domains` - List of allowed domain names
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::provider_tool::web_fetch_20250910;
    ///
    /// let tool = web_fetch_20250910()
    ///     .allowed_domains(vec!["example.com".to_string(), "docs.rs".to_string()])
    ///     .build();
    /// ```
    pub fn allowed_domains(mut self, domains: Vec<String>) -> Self {
        self.allowed_domains = Some(domains);
        self
    }

    /// Sets the blocked domains for fetching.
    ///
    /// URLs from these domains will never be fetched.
    ///
    /// # Arguments
    ///
    /// * `domains` - List of blocked domain names
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::provider_tool::web_fetch_20250910;
    ///
    /// let tool = web_fetch_20250910()
    ///     .blocked_domains(vec!["spam.com".to_string()])
    ///     .build();
    /// ```
    pub fn blocked_domains(mut self, domains: Vec<String>) -> Self {
        self.blocked_domains = Some(domains);
        self
    }

    /// Enables or disables citations for fetched content.
    ///
    /// Unlike web search where citations are always enabled, citations are optional
    /// for web fetch. When enabled, Claude can cite specific passages from fetched documents.
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether to enable citations
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::provider_tool::web_fetch_20250910;
    ///
    /// let tool = web_fetch_20250910()
    ///     .citations(true)
    ///     .build();
    /// ```
    pub fn citations(mut self, enabled: bool) -> Self {
        self.citations_enabled = Some(enabled);
        self
    }

    /// Sets the maximum content tokens to include in context.
    ///
    /// This parameter limits the amount of fetched content included in the context.
    ///
    /// # Arguments
    ///
    /// * `max_tokens` - Maximum number of content tokens
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::provider_tool::web_fetch_20250910;
    ///
    /// let tool = web_fetch_20250910()
    ///     .max_content_tokens(5000)
    ///     .build();
    /// ```
    pub fn max_content_tokens(mut self, max_tokens: i64) -> Self {
        self.max_content_tokens = Some(max_tokens);
        self
    }

    /// Sets the description for the tool.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::provider_tool::web_fetch_20250910;
    ///
    /// let tool = web_fetch_20250910()
    ///     .with_description("Fetch web content from URLs")
    ///     .build();
    /// ```
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets whether the tool requires approval before execution.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::provider_tool::web_fetch_20250910;
    ///
    /// let tool = web_fetch_20250910()
    ///     .with_needs_approval(true)
    ///     .build();
    /// ```
    pub fn with_needs_approval(mut self, needs_approval: bool) -> Self {
        self.needs_approval = needs_approval;
        self
    }

    /// Builds the configured tool.
    ///
    /// # Returns
    ///
    /// A configured `Tool` instance ready to be used with the Anthropic API.
    pub fn build(self) -> Tool {
        let factory = ProviderDefinedToolFactoryWithOutput::new(
            "anthropic.web_fetch_20250910",
            "web_fetch",
            json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "The URL to fetch."
                    }
                },
                "required": ["url"]
            }),
            json!({
                "type": "object",
                "properties": {
                    "type": {
                        "type": "string",
                        "enum": ["web_fetch_result"]
                    },
                    "url": {
                        "type": "string",
                        "description": "Fetched content URL"
                    },
                    "content": {
                        "type": "object",
                        "properties": {
                            "type": {
                                "type": "string",
                                "enum": ["document"]
                            },
                            "title": {
                                "type": "string",
                                "description": "Title of the document"
                            },
                            "citations": {
                                "type": "object",
                                "properties": {
                                    "enabled": {
                                        "type": "boolean"
                                    }
                                },
                                "description": "Citation configuration for the document"
                            },
                            "source": {
                                "oneOf": [
                                    {
                                        "type": "object",
                                        "properties": {
                                            "type": {
                                                "type": "string",
                                                "enum": ["base64"]
                                            },
                                            "mediaType": {
                                                "type": "string",
                                                "enum": ["application/pdf"]
                                            },
                                            "data": {
                                                "type": "string"
                                            }
                                        },
                                        "required": ["type", "mediaType", "data"]
                                    },
                                    {
                                        "type": "object",
                                        "properties": {
                                            "type": {
                                                "type": "string",
                                                "enum": ["text"]
                                            },
                                            "mediaType": {
                                                "type": "string",
                                                "enum": ["text/plain"]
                                            },
                                            "data": {
                                                "type": "string"
                                            }
                                        },
                                        "required": ["type", "mediaType", "data"]
                                    }
                                ]
                            }
                        },
                        "required": ["type", "title", "source"]
                    },
                    "retrievedAt": {
                        "type": ["string", "null"],
                        "description": "ISO 8601 timestamp when the content was retrieved"
                    }
                },
                "required": ["type", "url", "content", "retrievedAt"]
            }),
        );

        // Build options with configured settings
        let mut opts = ProviderDefinedToolOptions::new();

        if let Some(desc) = self.description {
            opts = opts.with_description(desc);
        }

        if self.needs_approval {
            opts = opts.with_needs_approval(true);
        }

        // Add all args
        if let Some(max_uses) = self.max_uses {
            opts = opts.with_arg("maxUses", json!(max_uses));
        }

        if let Some(domains) = self.allowed_domains {
            opts = opts.with_arg("allowedDomains", json!(domains));
        }

        if let Some(domains) = self.blocked_domains {
            opts = opts.with_arg("blockedDomains", json!(domains));
        }

        if let Some(enabled) = self.citations_enabled {
            opts = opts.with_arg("citations", json!({"enabled": enabled}));
        }

        if let Some(max_tokens) = self.max_content_tokens {
            opts = opts.with_arg("maxContentTokens", json!(max_tokens));
        }

        factory.create(opts)
    }
}

/// Creates a builder for a web fetch tool (version 20250910).
///
/// This tool allows the model to fetch content from URLs. The tool is named
/// `web_fetch` in the Anthropic API.
///
/// # Tool ID
/// `anthropic.web_fetch_20250910`
///
/// # Tool Name
/// `web_fetch`
///
/// # Features
///
/// - Fetch content from any URL
/// - Domain allowlist/blocklist support
/// - Optional citations for fetched content
/// - Content token limits
/// - Usage limits
/// - Supports both PDF (base64) and plain text content
///
/// # Returns
///
/// A builder that can be configured before calling `.build()` to create the tool.
///
/// # Examples
///
/// ## Basic Usage
///
/// ```
/// use ai_sdk_anthropic::provider_tool::web_fetch_20250910;
///
/// let tool = web_fetch_20250910().build();
/// ```
///
/// ## With Domain Restrictions
///
/// ```
/// use ai_sdk_anthropic::provider_tool::web_fetch_20250910;
///
/// let tool = web_fetch_20250910()
///     .allowed_domains(vec!["example.com".to_string(), "docs.rs".to_string()])
///     .blocked_domains(vec!["spam.com".to_string()])
///     .build();
/// ```
///
/// ## With Citations and Limits
///
/// ```
/// use ai_sdk_anthropic::provider_tool::web_fetch_20250910;
///
/// let tool = web_fetch_20250910()
///     .citations(true)
///     .max_uses(10)
///     .max_content_tokens(5000)
///     .with_description("Fetch web content with citations")
///     .build();
/// ```
pub fn web_fetch_20250910() -> WebFetch20250910Builder {
    WebFetch20250910Builder::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_sdk_provider_utils::ToolType;

    #[test]
    fn test_web_fetch_20250910_default() {
        let tool = web_fetch_20250910().build();

        assert!(tool.is_provider_defined());

        if let ToolType::ProviderDefined { id, name, args } = &tool.tool_type {
            assert_eq!(id, "anthropic.web_fetch_20250910");
            assert_eq!(name, "web_fetch");
            assert!(args.is_empty());
        } else {
            panic!("Expected ProviderDefined tool type");
        }

        assert!(!tool.input_schema.is_null());
        assert!(tool.output_schema.is_some());
        assert!(tool.description.is_none());
        assert!(tool.execute.is_none());
    }

    #[test]
    fn test_web_fetch_20250910_with_max_uses() {
        let tool = web_fetch_20250910().max_uses(10).build();

        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            assert_eq!(args.get("maxUses"), Some(&json!(10)));
        } else {
            panic!("Expected ProviderDefined tool type");
        }
    }

    #[test]
    fn test_web_fetch_20250910_with_allowed_domains() {
        let domains = vec!["example.com".to_string(), "docs.rs".to_string()];
        let tool = web_fetch_20250910()
            .allowed_domains(domains.clone())
            .build();

        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            assert_eq!(args.get("allowedDomains"), Some(&json!(domains)));
        } else {
            panic!("Expected ProviderDefined tool type");
        }
    }

    #[test]
    fn test_web_fetch_20250910_with_blocked_domains() {
        let domains = vec!["spam.com".to_string()];
        let tool = web_fetch_20250910()
            .blocked_domains(domains.clone())
            .build();

        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            assert_eq!(args.get("blockedDomains"), Some(&json!(domains)));
        } else {
            panic!("Expected ProviderDefined tool type");
        }
    }

    #[test]
    fn test_web_fetch_20250910_with_citations() {
        let tool = web_fetch_20250910().citations(true).build();

        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            assert_eq!(args.get("citations"), Some(&json!({"enabled": true})));
        } else {
            panic!("Expected ProviderDefined tool type");
        }
    }

    #[test]
    fn test_web_fetch_20250910_with_max_content_tokens() {
        let tool = web_fetch_20250910().max_content_tokens(5000).build();

        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            assert_eq!(args.get("maxContentTokens"), Some(&json!(5000)));
        } else {
            panic!("Expected ProviderDefined tool type");
        }
    }

    #[test]
    fn test_web_fetch_20250910_with_description() {
        let tool = web_fetch_20250910()
            .with_description("Fetch web content")
            .build();

        assert_eq!(tool.description, Some("Fetch web content".to_string()));
    }

    #[test]
    fn test_web_fetch_20250910_with_approval() {
        let tool = web_fetch_20250910().with_needs_approval(true).build();

        assert!(matches!(
            tool.needs_approval,
            ai_sdk_provider_utils::NeedsApproval::Yes
        ));
    }

    #[test]
    fn test_web_fetch_20250910_input_schema() {
        let tool = web_fetch_20250910().build();

        let schema = &tool.input_schema;
        assert_eq!(schema["type"], "object");

        let properties = &schema["properties"];
        assert!(properties.is_object());

        // Check url property
        let url = &properties["url"];
        assert_eq!(url["type"], "string");

        // Check required fields
        let required = &schema["required"];
        assert!(required.is_array());
        let required_array = required.as_array().unwrap();
        assert!(required_array.contains(&json!("url")));
        assert_eq!(required_array.len(), 1);
    }

    #[test]
    fn test_web_fetch_20250910_output_schema() {
        let tool = web_fetch_20250910().build();

        let schema = tool
            .output_schema
            .as_ref()
            .expect("Should have output schema");
        assert_eq!(schema["type"], "object");

        let properties = &schema["properties"];
        assert!(properties.is_object());

        // Check type property
        assert_eq!(properties["type"]["enum"], json!(["web_fetch_result"]));

        // Check url property
        assert_eq!(properties["url"]["type"], "string");

        // Check content property
        let content = &properties["content"];
        assert_eq!(content["type"], "object");
        assert_eq!(content["properties"]["type"]["enum"], json!(["document"]));
        assert_eq!(content["properties"]["title"]["type"], "string");

        // Check source has oneOf with base64 and text options
        let source = &content["properties"]["source"];
        assert!(source.get("oneOf").is_some());
        let one_of = source["oneOf"].as_array().unwrap();
        assert_eq!(one_of.len(), 2);

        // Check retrievedAt property
        assert!(properties["retrievedAt"]["type"].is_array());
    }

    #[test]
    fn test_web_fetch_20250910_builder_chaining() {
        let tool = web_fetch_20250910()
            .max_uses(10)
            .allowed_domains(vec!["example.com".to_string()])
            .blocked_domains(vec!["spam.com".to_string()])
            .citations(true)
            .max_content_tokens(5000)
            .with_description("Fetch web content")
            .with_needs_approval(true)
            .build();

        // Check all args
        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            assert_eq!(args.get("maxUses"), Some(&json!(10)));
            assert_eq!(args.get("allowedDomains"), Some(&json!(["example.com"])));
            assert_eq!(args.get("blockedDomains"), Some(&json!(["spam.com"])));
            assert_eq!(args.get("citations"), Some(&json!({"enabled": true})));
            assert_eq!(args.get("maxContentTokens"), Some(&json!(5000)));
        } else {
            panic!("Expected ProviderDefined tool type");
        }

        // Check description
        assert_eq!(tool.description, Some("Fetch web content".to_string()));

        // Check approval
        assert!(matches!(
            tool.needs_approval,
            ai_sdk_provider_utils::NeedsApproval::Yes
        ));
    }

    #[test]
    fn test_web_fetch_20250910_clone() {
        let tool1 = web_fetch_20250910()
            .with_description("Web fetch tool")
            .build();
        let tool2 = tool1.clone();

        assert_eq!(tool1.description, tool2.description);
        assert_eq!(tool1.input_schema, tool2.input_schema);
        assert_eq!(tool1.output_schema, tool2.output_schema);

        if let (
            ToolType::ProviderDefined {
                id: id1,
                name: name1,
                ..
            },
            ToolType::ProviderDefined {
                id: id2,
                name: name2,
                ..
            },
        ) = (&tool1.tool_type, &tool2.tool_type)
        {
            assert_eq!(id1, id2);
            assert_eq!(name1, name2);
        }
    }

    #[test]
    fn test_web_fetch_20250910_tool_name() {
        let tool = web_fetch_20250910().build();

        if let ToolType::ProviderDefined { name, .. } = &tool.tool_type {
            assert_eq!(name, "web_fetch");
        }
    }

    #[test]
    fn test_web_fetch_20250910_builder_reuse() {
        let builder = web_fetch_20250910().max_uses(10).citations(true);

        // Clone the builder and create two different tools
        let tool1 = builder.clone().with_description("Tool 1").build();
        let tool2 = builder.with_description("Tool 2").build();

        // Both should have maxUses and citations
        if let ToolType::ProviderDefined { args, .. } = &tool1.tool_type {
            assert_eq!(args.get("maxUses"), Some(&json!(10)));
            assert_eq!(args.get("citations"), Some(&json!({"enabled": true})));
        }
        if let ToolType::ProviderDefined { args, .. } = &tool2.tool_type {
            assert_eq!(args.get("maxUses"), Some(&json!(10)));
            assert_eq!(args.get("citations"), Some(&json!({"enabled": true})));
        }

        // But different descriptions
        assert_eq!(tool1.description, Some("Tool 1".to_string()));
        assert_eq!(tool2.description, Some("Tool 2".to_string()));
    }
}
