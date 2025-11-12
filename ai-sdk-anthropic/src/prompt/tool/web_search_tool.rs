use serde::{Deserialize, Serialize};

use crate::prompt::message::cache_control::AnthropicCacheControl;

/// User location information for web search context.
///
/// Provides approximate location information to help contextualize search results.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserLocation {
    /// The type of location (always "approximate")
    #[serde(rename = "type")]
    pub location_type: String,

    /// Optional city name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Optional region/state name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// Optional country name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Optional timezone (e.g., "America/New_York")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl UserLocation {
    /// Creates a new approximate user location.
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::tool::web_search_tool::UserLocation;
    ///
    /// let location = UserLocation::new()
    ///     .with_city("San Francisco")
    ///     .with_region("California")
    ///     .with_country("United States");
    /// ```
    pub fn new() -> Self {
        Self {
            location_type: "approximate".to_string(),
            city: None,
            region: None,
            country: None,
            timezone: None,
        }
    }

    /// Sets the city.
    pub fn with_city(mut self, city: impl Into<String>) -> Self {
        self.city = Some(city.into());
        self
    }

    /// Sets the region/state.
    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }

    /// Sets the country.
    pub fn with_country(mut self, country: impl Into<String>) -> Self {
        self.country = Some(country.into());
        self
    }

    /// Sets the timezone.
    pub fn with_timezone(mut self, timezone: impl Into<String>) -> Self {
        self.timezone = Some(timezone.into());
        self
    }

    /// Removes the city.
    pub fn without_city(mut self) -> Self {
        self.city = None;
        self
    }

    /// Removes the region.
    pub fn without_region(mut self) -> Self {
        self.region = None;
        self
    }

    /// Removes the country.
    pub fn without_country(mut self) -> Self {
        self.country = None;
        self
    }

    /// Removes the timezone.
    pub fn without_timezone(mut self) -> Self {
        self.timezone = None;
        self
    }
}

impl Default for UserLocation {
    fn default() -> Self {
        Self::new()
    }
}

/// Web search tool (version 20250305).
///
/// This tool allows the model to perform web searches with various configuration
/// options including domain filtering, location context, and usage limits.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::prompt::tool::web_search_tool::{AnthropicWebSearchTool, UserLocation};
///
/// let location = UserLocation::new()
///     .with_city("New York")
///     .with_country("United States");
///
/// let tool = AnthropicWebSearchTool::new("web_search")
///     .with_max_uses(10)
///     .with_allowed_domain("*.edu")
///     .with_user_location(location);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicWebSearchTool {
    /// The type of tool (always "web_search_20250305")
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

    /// Optional user location for contextualizing search results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_location: Option<UserLocation>,

    /// Optional cache control configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}

impl AnthropicWebSearchTool {
    /// Creates a new web search tool.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the tool
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::tool::web_search_tool::AnthropicWebSearchTool;
    ///
    /// let tool = AnthropicWebSearchTool::new("web_search");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            tool_type: "web_search_20250305".to_string(),
            name: name.into(),
            max_uses: None,
            allowed_domains: None,
            blocked_domains: None,
            user_location: None,
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

    /// Sets the user location.
    pub fn with_user_location(mut self, location: UserLocation) -> Self {
        self.user_location = Some(location);
        self
    }

    /// Removes the user location.
    pub fn without_user_location(mut self) -> Self {
        self.user_location = None;
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

    // Tests for UserLocation
    #[test]
    fn test_user_location_new() {
        let location = UserLocation::new();

        assert_eq!(location.location_type, "approximate");
        assert!(location.city.is_none());
        assert!(location.region.is_none());
        assert!(location.country.is_none());
        assert!(location.timezone.is_none());
    }

    #[test]
    fn test_user_location_builder() {
        let location = UserLocation::new()
            .with_city("San Francisco")
            .with_region("California")
            .with_country("United States")
            .with_timezone("America/Los_Angeles");

        assert_eq!(location.city, Some("San Francisco".to_string()));
        assert_eq!(location.region, Some("California".to_string()));
        assert_eq!(location.country, Some("United States".to_string()));
        assert_eq!(location.timezone, Some("America/Los_Angeles".to_string()));
    }

    #[test]
    fn test_user_location_serialize() {
        let location = UserLocation::new()
            .with_city("New York")
            .with_country("USA");

        let json = serde_json::to_value(&location).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "approximate",
                "city": "New York",
                "country": "USA"
            })
        );
    }

    // Tests for AnthropicWebSearchTool
    #[test]
    fn test_new() {
        let tool = AnthropicWebSearchTool::new("web_search");

        assert_eq!(tool.tool_type, "web_search_20250305");
        assert_eq!(tool.name, "web_search");
        assert!(tool.max_uses.is_none());
        assert!(tool.allowed_domains.is_none());
        assert!(tool.blocked_domains.is_none());
        assert!(tool.user_location.is_none());
        assert!(tool.cache_control.is_none());
    }

    #[test]
    fn test_with_max_uses() {
        let tool = AnthropicWebSearchTool::new("search").with_max_uses(5);

        assert_eq!(tool.max_uses, Some(5));
    }

    #[test]
    fn test_with_allowed_domain() {
        let tool = AnthropicWebSearchTool::new("search")
            .with_allowed_domain("example.com")
            .with_allowed_domain("github.com");

        assert_eq!(
            tool.allowed_domains,
            Some(vec!["example.com".to_string(), "github.com".to_string()])
        );
    }

    #[test]
    fn test_with_user_location() {
        let location = UserLocation::new()
            .with_city("Paris")
            .with_country("France");

        let tool = AnthropicWebSearchTool::new("search").with_user_location(location.clone());

        assert_eq!(tool.user_location, Some(location));
    }

    #[test]
    fn test_serialize_minimal() {
        let tool = AnthropicWebSearchTool::new("web_search");
        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "web_search_20250305",
                "name": "web_search"
            })
        );
    }

    #[test]
    fn test_serialize_full() {
        let location = UserLocation::new()
            .with_city("Tokyo")
            .with_country("Japan")
            .with_timezone("Asia/Tokyo");

        let tool = AnthropicWebSearchTool::new("web_search")
            .with_max_uses(15)
            .with_allowed_domain("*.edu")
            .with_blocked_domain("spam.com")
            .with_user_location(location)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&tool).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "web_search_20250305",
                "name": "web_search",
                "max_uses": 15,
                "allowed_domains": ["*.edu"],
                "blocked_domains": ["spam.com"],
                "user_location": {
                    "type": "approximate",
                    "city": "Tokyo",
                    "country": "Japan",
                    "timezone": "Asia/Tokyo"
                },
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
            "type": "web_search_20250305",
            "name": "my_search",
            "max_uses": 10,
            "allowed_domains": ["example.com"],
            "user_location": {
                "type": "approximate",
                "city": "London",
                "country": "UK"
            },
            "cache_control": {
                "type": "ephemeral",
                "ttl": "5m"
            }
        });

        let tool: AnthropicWebSearchTool = serde_json::from_value(json).unwrap();

        assert_eq!(tool.name, "my_search");
        assert_eq!(tool.max_uses, Some(10));
        assert_eq!(tool.allowed_domains, Some(vec!["example.com".to_string()]));
        assert!(tool.user_location.is_some());
        assert_eq!(
            tool.user_location.as_ref().unwrap().city,
            Some("London".to_string())
        );
    }

    #[test]
    fn test_roundtrip() {
        let location = UserLocation::new()
            .with_city("Berlin")
            .with_country("Germany");

        let original = AnthropicWebSearchTool::new("web_search")
            .with_max_uses(10)
            .with_allowed_domain("example.com")
            .with_user_location(location)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicWebSearchTool = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }
}
