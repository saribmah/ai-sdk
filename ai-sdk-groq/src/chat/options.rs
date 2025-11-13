use serde::{Deserialize, Serialize};

/// Groq chat model identifier.
///
/// Reference: <https://console.groq.com/docs/models>
pub type GroqChatModelId = String;

/// Groq-specific provider options for chat completions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroqProviderOptions {
    /// Reasoning format for models that support reasoning.
    /// Options: "parsed", "raw", "hidden"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_format: Option<ReasoningFormat>,

    /// Reasoning effort level (string value, provider-specific).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,

    /// Whether to enable parallel function calling during tool use.
    /// Default: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// A unique identifier representing your end-user, which can help Groq
    /// monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// Whether to use structured outputs.
    /// Default: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_outputs: Option<bool>,

    /// Service tier for the request.
    /// - "on_demand": Default tier with consistent performance
    /// - "flex": Higher throughput tier optimized for workloads that can handle occasional failures
    /// - "auto": Uses on_demand rate limits, then falls back to flex tier if exceeded
    ///
    ///   Default: "on_demand"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,
}

/// Reasoning format options for Groq models.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningFormat {
    /// Parsed reasoning output
    Parsed,
    /// Raw reasoning output
    Raw,
    /// Hidden reasoning output
    Hidden,
}

/// Service tier options for Groq API requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceTier {
    /// Default tier with consistent performance and fairness
    OnDemand,
    /// Higher throughput tier optimized for workloads that can handle occasional request failures
    Flex,
    /// Uses on_demand rate limits, then falls back to flex tier if exceeded
    Auto,
}

impl GroqProviderOptions {
    /// Creates a new instance of Groq provider options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the reasoning format.
    pub fn with_reasoning_format(mut self, format: ReasoningFormat) -> Self {
        self.reasoning_format = Some(format);
        self
    }

    /// Sets the reasoning effort.
    pub fn with_reasoning_effort(mut self, effort: impl Into<String>) -> Self {
        self.reasoning_effort = Some(effort.into());
        self
    }

    /// Sets whether to enable parallel tool calls.
    pub fn with_parallel_tool_calls(mut self, enabled: bool) -> Self {
        self.parallel_tool_calls = Some(enabled);
        self
    }

    /// Sets the user identifier.
    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }

    /// Sets whether to use structured outputs.
    pub fn with_structured_outputs(mut self, enabled: bool) -> Self {
        self.structured_outputs = Some(enabled);
        self
    }

    /// Sets the service tier.
    pub fn with_service_tier(mut self, tier: ServiceTier) -> Self {
        self.service_tier = Some(tier);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_options_builder() {
        let options = GroqProviderOptions::new()
            .with_reasoning_format(ReasoningFormat::Parsed)
            .with_reasoning_effort("high")
            .with_parallel_tool_calls(true)
            .with_user("test-user")
            .with_structured_outputs(true)
            .with_service_tier(ServiceTier::Flex);

        assert_eq!(options.reasoning_format, Some(ReasoningFormat::Parsed));
        assert_eq!(options.reasoning_effort, Some("high".to_string()));
        assert_eq!(options.parallel_tool_calls, Some(true));
        assert_eq!(options.user, Some("test-user".to_string()));
        assert_eq!(options.structured_outputs, Some(true));
        assert_eq!(options.service_tier, Some(ServiceTier::Flex));
    }

    #[test]
    fn test_reasoning_format_serialization() {
        let format = ReasoningFormat::Parsed;
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, r#""parsed""#);

        let format = ReasoningFormat::Raw;
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, r#""raw""#);

        let format = ReasoningFormat::Hidden;
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, r#""hidden""#);
    }

    #[test]
    fn test_service_tier_serialization() {
        let tier = ServiceTier::OnDemand;
        let json = serde_json::to_string(&tier).unwrap();
        assert_eq!(json, r#""on_demand""#);

        let tier = ServiceTier::Flex;
        let json = serde_json::to_string(&tier).unwrap();
        assert_eq!(json, r#""flex""#);

        let tier = ServiceTier::Auto;
        let json = serde_json::to_string(&tier).unwrap();
        assert_eq!(json, r#""auto""#);
    }

    #[test]
    fn test_default_options() {
        let options = GroqProviderOptions::default();
        assert!(options.reasoning_format.is_none());
        assert!(options.reasoning_effort.is_none());
        assert!(options.parallel_tool_calls.is_none());
        assert!(options.user.is_none());
        assert!(options.structured_outputs.is_none());
        assert!(options.service_tier.is_none());
    }
}
