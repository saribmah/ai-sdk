//! Anthropic web search tool (version 20250305).
//!
//! This module provides a factory for creating web search tools that allow
//! the model to search the web. Unlike web fetch, citations are always enabled
//! for web search results.
//!
//! # Example
//!
//! ```
//! use llm_kit_anthropic::provider_tool::web_search_20250305;
//!
//! // Create a web search tool with default options
//! let builder = web_search_20250305();
//! let tool = builder.build();
//!
//! // Create with user location for geographically relevant results
//! let tool = web_search_20250305()
//!     .user_location("San Francisco", Some("California"), Some("US"), Some("America/Los_Angeles"))
//!     .max_uses(10)
//!     .build();
//! ```

use llm_kit_provider_utils::tool::{
    ProviderDefinedToolFactoryWithOutput, ProviderDefinedToolOptions, Tool,
};
use serde_json::json;

/// Builder for creating a web search tool (version 20250305).
///
/// This builder allows configuring various parameters for web searching including
/// domain restrictions, usage limits, and user location for geographically relevant results.
#[derive(Clone, Debug, Default)]
pub struct WebSearch20250305Builder {
    max_uses: Option<i64>,
    allowed_domains: Option<Vec<String>>,
    blocked_domains: Option<Vec<String>>,
    user_location: Option<UserLocation>,
    description: Option<String>,
    needs_approval: bool,
}

/// User location information for geographically relevant search results.
#[derive(Clone, Debug)]
struct UserLocation {
    city: Option<String>,
    region: Option<String>,
    country: Option<String>,
    timezone: Option<String>,
}

impl WebSearch20250305Builder {
    /// Creates a new builder with default settings.
    fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum number of web searches allowed.
    ///
    /// # Arguments
    ///
    /// * `max_uses` - Maximum number of searches
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::provider_tool::web_search_20250305;
    ///
    /// let tool = web_search_20250305()
    ///     .max_uses(10)
    ///     .build();
    /// ```
    pub fn max_uses(mut self, max_uses: i64) -> Self {
        self.max_uses = Some(max_uses);
        self
    }

    /// Sets the allowed domains for searching.
    ///
    /// Only results from these domains will be returned.
    ///
    /// # Arguments
    ///
    /// * `domains` - List of allowed domain names
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::provider_tool::web_search_20250305;
    ///
    /// let tool = web_search_20250305()
    ///     .allowed_domains(vec!["example.com".to_string(), "docs.rs".to_string()])
    ///     .build();
    /// ```
    pub fn allowed_domains(mut self, domains: Vec<String>) -> Self {
        self.allowed_domains = Some(domains);
        self
    }

    /// Sets the blocked domains for searching.
    ///
    /// Results from these domains will be excluded.
    ///
    /// # Arguments
    ///
    /// * `domains` - List of blocked domain names
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::provider_tool::web_search_20250305;
    ///
    /// let tool = web_search_20250305()
    ///     .blocked_domains(vec!["spam.com".to_string()])
    ///     .build();
    /// ```
    pub fn blocked_domains(mut self, domains: Vec<String>) -> Self {
        self.blocked_domains = Some(domains);
        self
    }

    /// Sets the user location for geographically relevant search results.
    ///
    /// All parameters are optional. Provide as much detail as available for
    /// the most relevant results.
    ///
    /// # Arguments
    ///
    /// * `city` - City name (e.g., "San Francisco")
    /// * `region` - Region or state (e.g., "California")
    /// * `country` - Country code (e.g., "US")
    /// * `timezone` - IANA timezone ID (e.g., "America/Los_Angeles")
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::provider_tool::web_search_20250305;
    ///
    /// let tool = web_search_20250305()
    ///     .user_location(
    ///         "San Francisco",
    ///         Some("California"),
    ///         Some("US"),
    ///         Some("America/Los_Angeles")
    ///     )
    ///     .build();
    /// ```
    pub fn user_location(
        mut self,
        city: impl Into<String>,
        region: Option<impl Into<String>>,
        country: Option<impl Into<String>>,
        timezone: Option<impl Into<String>>,
    ) -> Self {
        self.user_location = Some(UserLocation {
            city: Some(city.into()),
            region: region.map(|r| r.into()),
            country: country.map(|c| c.into()),
            timezone: timezone.map(|t| t.into()),
        });
        self
    }

    /// Sets the description for the tool.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::provider_tool::web_search_20250305;
    ///
    /// let tool = web_search_20250305()
    ///     .with_description("Search the web for information")
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
    /// use llm_kit_anthropic::provider_tool::web_search_20250305;
    ///
    /// let tool = web_search_20250305()
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
            "anthropic.web_search_20250305",
            "web_search",
            json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The search query to execute."
                    }
                },
                "required": ["query"]
            }),
            json!({
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "type": {
                            "type": "string",
                            "enum": ["web_search_result"]
                        },
                        "url": {
                            "type": "string",
                            "description": "The URL of the source page."
                        },
                        "title": {
                            "type": "string",
                            "description": "The title of the source page."
                        },
                        "pageAge": {
                            "type": ["string", "null"],
                            "description": "When the site was last updated"
                        },
                        "encryptedContent": {
                            "type": "string",
                            "description": "Encrypted content that must be passed back in multi-turn conversations for citations"
                        }
                    },
                    "required": ["type", "url", "title", "pageAge", "encryptedContent"]
                }
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

        if let Some(location) = self.user_location {
            let mut location_obj = json!({
                "type": "approximate"
            });

            if let Some(city) = location.city {
                location_obj["city"] = json!(city);
            }
            if let Some(region) = location.region {
                location_obj["region"] = json!(region);
            }
            if let Some(country) = location.country {
                location_obj["country"] = json!(country);
            }
            if let Some(timezone) = location.timezone {
                location_obj["timezone"] = json!(timezone);
            }

            opts = opts.with_arg("userLocation", location_obj);
        }

        factory.create(opts)
    }
}

/// Creates a builder for a web search tool (version 20250305).
///
/// This tool allows the model to search the web for information. The tool is named
/// `web_search` in the Anthropic API.
///
/// **Note:** Citations are always enabled for web search (unlike web fetch where
/// citations are optional).
///
/// # Tool ID
/// `anthropic.web_search_20250305`
///
/// # Tool Name
/// `web_search`
///
/// # Features
///
/// - Search the web with natural language queries
/// - Domain allowlist/blocklist support
/// - User location for geographically relevant results
/// - Usage limits
/// - Citations always enabled
/// - Returns encrypted content for multi-turn citations
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
/// use llm_kit_anthropic::provider_tool::web_search_20250305;
///
/// let tool = web_search_20250305().build();
/// ```
///
/// ## With Domain Restrictions
///
/// ```
/// use llm_kit_anthropic::provider_tool::web_search_20250305;
///
/// let tool = web_search_20250305()
///     .allowed_domains(vec!["example.com".to_string(), "docs.rs".to_string()])
///     .blocked_domains(vec!["spam.com".to_string()])
///     .build();
/// ```
///
/// ## With User Location
///
/// ```
/// use llm_kit_anthropic::provider_tool::web_search_20250305;
///
/// let tool = web_search_20250305()
///     .user_location(
///         "San Francisco",
///         Some("California"),
///         Some("US"),
///         Some("America/Los_Angeles")
///     )
///     .max_uses(10)
///     .with_description("Search for local information")
///     .build();
/// ```
pub fn web_search_20250305() -> WebSearch20250305Builder {
    WebSearch20250305Builder::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use llm_kit_provider_utils::ToolType;

    #[test]
    fn test_web_search_20250305_default() {
        let tool = web_search_20250305().build();

        assert!(tool.is_provider_defined());

        if let ToolType::ProviderDefined { id, name, args } = &tool.tool_type {
            assert_eq!(id, "anthropic.web_search_20250305");
            assert_eq!(name, "web_search");
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
    fn test_web_search_20250305_with_max_uses() {
        let tool = web_search_20250305().max_uses(10).build();

        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            assert_eq!(args.get("maxUses"), Some(&json!(10)));
        } else {
            panic!("Expected ProviderDefined tool type");
        }
    }

    #[test]
    fn test_web_search_20250305_with_allowed_domains() {
        let domains = vec!["example.com".to_string(), "docs.rs".to_string()];
        let tool = web_search_20250305()
            .allowed_domains(domains.clone())
            .build();

        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            assert_eq!(args.get("allowedDomains"), Some(&json!(domains)));
        } else {
            panic!("Expected ProviderDefined tool type");
        }
    }

    #[test]
    fn test_web_search_20250305_with_blocked_domains() {
        let domains = vec!["spam.com".to_string()];
        let tool = web_search_20250305()
            .blocked_domains(domains.clone())
            .build();

        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            assert_eq!(args.get("blockedDomains"), Some(&json!(domains)));
        } else {
            panic!("Expected ProviderDefined tool type");
        }
    }

    #[test]
    fn test_web_search_20250305_with_user_location_full() {
        let tool = web_search_20250305()
            .user_location(
                "San Francisco",
                Some("California"),
                Some("US"),
                Some("America/Los_Angeles"),
            )
            .build();

        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            let location = args
                .get("userLocation")
                .expect("userLocation should be set");
            assert_eq!(location["type"], "approximate");
            assert_eq!(location["city"], "San Francisco");
            assert_eq!(location["region"], "California");
            assert_eq!(location["country"], "US");
            assert_eq!(location["timezone"], "America/Los_Angeles");
        } else {
            panic!("Expected ProviderDefined tool type");
        }
    }

    #[test]
    fn test_web_search_20250305_with_user_location_partial() {
        let tool = web_search_20250305()
            .user_location("Tokyo", Some(""), None::<String>, Some("Asia/Tokyo"))
            .build();

        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            let location = args
                .get("userLocation")
                .expect("userLocation should be set");
            assert_eq!(location["type"], "approximate");
            assert_eq!(location["city"], "Tokyo");
            assert_eq!(location["region"], "");
            assert!(location.get("country").is_none());
            assert_eq!(location["timezone"], "Asia/Tokyo");
        } else {
            panic!("Expected ProviderDefined tool type");
        }
    }

    #[test]
    fn test_web_search_20250305_with_description() {
        let tool = web_search_20250305()
            .with_description("Search the web")
            .build();

        assert_eq!(tool.description, Some("Search the web".to_string()));
    }

    #[test]
    fn test_web_search_20250305_with_approval() {
        let tool = web_search_20250305().with_needs_approval(true).build();

        assert!(matches!(
            tool.needs_approval,
            llm_kit_provider_utils::NeedsApproval::Yes
        ));
    }

    #[test]
    fn test_web_search_20250305_input_schema() {
        let tool = web_search_20250305().build();

        let schema = &tool.input_schema;
        assert_eq!(schema["type"], "object");

        let properties = &schema["properties"];
        assert!(properties.is_object());

        // Check query property
        let query = &properties["query"];
        assert_eq!(query["type"], "string");

        // Check required fields
        let required = &schema["required"];
        assert!(required.is_array());
        let required_array = required.as_array().unwrap();
        assert!(required_array.contains(&json!("query")));
        assert_eq!(required_array.len(), 1);
    }

    #[test]
    fn test_web_search_20250305_output_schema() {
        let tool = web_search_20250305().build();

        let schema = tool
            .output_schema
            .as_ref()
            .expect("Should have output schema");
        assert_eq!(schema["type"], "array");

        let items = &schema["items"];
        assert_eq!(items["type"], "object");

        let properties = &items["properties"];

        // Check type property
        assert_eq!(properties["type"]["enum"], json!(["web_search_result"]));

        // Check url property
        assert_eq!(properties["url"]["type"], "string");

        // Check title property
        assert_eq!(properties["title"]["type"], "string");

        // Check pageAge property (nullable)
        assert!(properties["pageAge"]["type"].is_array());

        // Check encryptedContent property
        assert_eq!(properties["encryptedContent"]["type"], "string");

        // Check required fields
        let required = &items["required"];
        assert!(required.is_array());
        let required_array = required.as_array().unwrap();
        assert_eq!(required_array.len(), 5);
        assert!(required_array.contains(&json!("type")));
        assert!(required_array.contains(&json!("url")));
        assert!(required_array.contains(&json!("title")));
        assert!(required_array.contains(&json!("pageAge")));
        assert!(required_array.contains(&json!("encryptedContent")));
    }

    #[test]
    fn test_web_search_20250305_builder_chaining() {
        let tool = web_search_20250305()
            .max_uses(10)
            .allowed_domains(vec!["example.com".to_string()])
            .blocked_domains(vec!["spam.com".to_string()])
            .user_location("New York", Some("NY"), Some("US"), Some("America/New_York"))
            .with_description("Search the web")
            .with_needs_approval(true)
            .build();

        // Check all args
        if let ToolType::ProviderDefined { args, .. } = &tool.tool_type {
            assert_eq!(args.get("maxUses"), Some(&json!(10)));
            assert_eq!(args.get("allowedDomains"), Some(&json!(["example.com"])));
            assert_eq!(args.get("blockedDomains"), Some(&json!(["spam.com"])));

            let location = args
                .get("userLocation")
                .expect("userLocation should be set");
            assert_eq!(location["type"], "approximate");
            assert_eq!(location["city"], "New York");
            assert_eq!(location["region"], "NY");
            assert_eq!(location["country"], "US");
            assert_eq!(location["timezone"], "America/New_York");
        } else {
            panic!("Expected ProviderDefined tool type");
        }

        // Check description
        assert_eq!(tool.description, Some("Search the web".to_string()));

        // Check approval
        assert!(matches!(
            tool.needs_approval,
            llm_kit_provider_utils::NeedsApproval::Yes
        ));
    }

    #[test]
    fn test_web_search_20250305_clone() {
        let tool1 = web_search_20250305()
            .with_description("Web search tool")
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
    fn test_web_search_20250305_tool_name() {
        let tool = web_search_20250305().build();

        if let ToolType::ProviderDefined { name, .. } = &tool.tool_type {
            assert_eq!(name, "web_search");
        }
    }

    #[test]
    fn test_web_search_20250305_builder_reuse() {
        let builder = web_search_20250305().max_uses(10).user_location(
            "London",
            Some("England"),
            Some("UK"),
            Some("Europe/London"),
        );

        // Clone the builder and create two different tools
        let tool1 = builder.clone().with_description("Tool 1").build();
        let tool2 = builder.with_description("Tool 2").build();

        // Both should have maxUses and userLocation
        if let ToolType::ProviderDefined { args, .. } = &tool1.tool_type {
            assert_eq!(args.get("maxUses"), Some(&json!(10)));
            assert!(args.get("userLocation").is_some());
        }
        if let ToolType::ProviderDefined { args, .. } = &tool2.tool_type {
            assert_eq!(args.get("maxUses"), Some(&json!(10)));
            assert!(args.get("userLocation").is_some());
        }

        // But different descriptions
        assert_eq!(tool1.description, Some("Tool 1".to_string()));
        assert_eq!(tool2.description, Some("Tool 2".to_string()));
    }
}
