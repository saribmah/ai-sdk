use serde::{Deserialize, Serialize};

/// Anthropic cache control configuration for prompt caching.
///
/// Anthropic's prompt caching allows you to cache parts of your prompt to reduce costs
/// and latency for repeated content. This is particularly useful for:
/// - Long system prompts that don't change
/// - Conversation history that remains constant
/// - Large context documents
///
/// # Example
///
/// ```
/// use llm_kit_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
///
/// // Create cache control with default TTL (5 minutes)
/// let cache_control = AnthropicCacheControl::default();
///
/// // Create cache control with 1 hour TTL
/// let cache_control = AnthropicCacheControl::new(CacheTTL::OneHour);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicCacheControl {
    /// The type of cache control (currently only "ephemeral" is supported)
    #[serde(rename = "type")]
    pub cache_type: String,

    /// Time-to-live for the cached content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<CacheTTL>,
}

/// Time-to-live options for cached content.
///
/// Determines how long Anthropic will cache the content before it expires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CacheTTL {
    /// Cache for 5 minutes
    #[serde(rename = "5m")]
    FiveMinutes,

    /// Cache for 1 hour
    #[serde(rename = "1h")]
    OneHour,
}

impl AnthropicCacheControl {
    /// Creates a new cache control with the specified TTL.
    ///
    /// # Arguments
    ///
    /// * `ttl` - The time-to-live for the cached content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    ///
    /// let cache_control = AnthropicCacheControl::new(CacheTTL::OneHour);
    /// ```
    pub fn new(ttl: CacheTTL) -> Self {
        Self {
            cache_type: "ephemeral".to_string(),
            ttl: Some(ttl),
        }
    }

    /// Creates a new ephemeral cache control without specifying TTL (uses default).
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::cache_control::AnthropicCacheControl;
    ///
    /// let cache_control = AnthropicCacheControl::ephemeral();
    /// ```
    pub fn ephemeral() -> Self {
        Self {
            cache_type: "ephemeral".to_string(),
            ttl: None,
        }
    }

    /// Sets the TTL for this cache control.
    ///
    /// # Arguments
    ///
    /// * `ttl` - The time-to-live for the cached content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    ///
    /// let cache_control = AnthropicCacheControl::ephemeral()
    ///     .with_ttl(CacheTTL::FiveMinutes);
    /// ```
    pub fn with_ttl(mut self, ttl: CacheTTL) -> Self {
        self.ttl = Some(ttl);
        self
    }
}

impl Default for AnthropicCacheControl {
    /// Creates a default cache control (ephemeral type with 5 minute TTL).
    fn default() -> Self {
        Self::new(CacheTTL::FiveMinutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new_with_five_minutes() {
        let cache_control = AnthropicCacheControl::new(CacheTTL::FiveMinutes);

        assert_eq!(cache_control.cache_type, "ephemeral");
        assert_eq!(cache_control.ttl, Some(CacheTTL::FiveMinutes));
    }

    #[test]
    fn test_new_with_one_hour() {
        let cache_control = AnthropicCacheControl::new(CacheTTL::OneHour);

        assert_eq!(cache_control.cache_type, "ephemeral");
        assert_eq!(cache_control.ttl, Some(CacheTTL::OneHour));
    }

    #[test]
    fn test_ephemeral() {
        let cache_control = AnthropicCacheControl::ephemeral();

        assert_eq!(cache_control.cache_type, "ephemeral");
        assert_eq!(cache_control.ttl, None);
    }

    #[test]
    fn test_with_ttl() {
        let cache_control = AnthropicCacheControl::ephemeral().with_ttl(CacheTTL::OneHour);

        assert_eq!(cache_control.ttl, Some(CacheTTL::OneHour));
    }

    #[test]
    fn test_default() {
        let cache_control = AnthropicCacheControl::default();

        assert_eq!(cache_control.cache_type, "ephemeral");
        assert_eq!(cache_control.ttl, Some(CacheTTL::FiveMinutes));
    }

    #[test]
    fn test_serialize_with_ttl() {
        let cache_control = AnthropicCacheControl::new(CacheTTL::FiveMinutes);
        let json = serde_json::to_value(&cache_control).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "ephemeral",
                "ttl": "5m"
            })
        );
    }

    #[test]
    fn test_serialize_one_hour() {
        let cache_control = AnthropicCacheControl::new(CacheTTL::OneHour);
        let json = serde_json::to_value(&cache_control).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "ephemeral",
                "ttl": "1h"
            })
        );
    }

    #[test]
    fn test_serialize_without_ttl() {
        let cache_control = AnthropicCacheControl::ephemeral();
        let json = serde_json::to_value(&cache_control).unwrap();

        // Should not include ttl field when it's None
        assert_eq!(
            json,
            json!({
                "type": "ephemeral"
            })
        );
    }

    #[test]
    fn test_deserialize_with_ttl() {
        let json = json!({
            "type": "ephemeral",
            "ttl": "5m"
        });

        let cache_control: AnthropicCacheControl = serde_json::from_value(json).unwrap();

        assert_eq!(cache_control.cache_type, "ephemeral");
        assert_eq!(cache_control.ttl, Some(CacheTTL::FiveMinutes));
    }

    #[test]
    fn test_deserialize_one_hour() {
        let json = json!({
            "type": "ephemeral",
            "ttl": "1h"
        });

        let cache_control: AnthropicCacheControl = serde_json::from_value(json).unwrap();

        assert_eq!(cache_control.cache_type, "ephemeral");
        assert_eq!(cache_control.ttl, Some(CacheTTL::OneHour));
    }

    #[test]
    fn test_deserialize_without_ttl() {
        let json = json!({
            "type": "ephemeral"
        });

        let cache_control: AnthropicCacheControl = serde_json::from_value(json).unwrap();

        assert_eq!(cache_control.cache_type, "ephemeral");
        assert_eq!(cache_control.ttl, None);
    }

    #[test]
    fn test_cache_ttl_serialization() {
        let five_min = CacheTTL::FiveMinutes;
        let one_hour = CacheTTL::OneHour;

        assert_eq!(serde_json::to_value(five_min).unwrap(), json!("5m"));
        assert_eq!(serde_json::to_value(one_hour).unwrap(), json!("1h"));
    }

    #[test]
    fn test_cache_ttl_deserialization() {
        let five_min: CacheTTL = serde_json::from_value(json!("5m")).unwrap();
        let one_hour: CacheTTL = serde_json::from_value(json!("1h")).unwrap();

        assert_eq!(five_min, CacheTTL::FiveMinutes);
        assert_eq!(one_hour, CacheTTL::OneHour);
    }

    #[test]
    fn test_equality() {
        let cache1 = AnthropicCacheControl::new(CacheTTL::FiveMinutes);
        let cache2 = AnthropicCacheControl::new(CacheTTL::FiveMinutes);
        let cache3 = AnthropicCacheControl::new(CacheTTL::OneHour);

        assert_eq!(cache1, cache2);
        assert_ne!(cache1, cache3);
    }

    #[test]
    fn test_clone() {
        let cache1 = AnthropicCacheControl::new(CacheTTL::OneHour);
        let cache2 = cache1.clone();

        assert_eq!(cache1, cache2);
    }
}
