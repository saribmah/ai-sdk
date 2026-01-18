use crate::prompt::message::cache_control::AnthropicCacheControl;
use serde_json::Value;
use std::collections::HashMap;

/// Anthropic allows a maximum of 4 cache breakpoints per request
const MAX_CACHE_BREAKPOINTS: usize = 4;

/// Type alias for provider metadata (matches llm-kit-provider's SharedProviderMetadata)
pub type SharedProviderMetadata = HashMap<String, HashMap<String, Value>>;

/// Warning type for unsupported settings
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheControlWarning {
    /// Type of warning
    pub warning_type: String,
    /// Setting that caused the warning
    pub setting: String,
    /// Details about the warning
    pub details: String,
}

impl CacheControlWarning {
    /// Creates a new unsupported setting warning
    pub fn unsupported_setting(setting: impl Into<String>, details: impl Into<String>) -> Self {
        Self {
            warning_type: "unsupported-setting".to_string(),
            setting: setting.into(),
            details: details.into(),
        }
    }
}

/// Helper function to extract cache_control from provider metadata
/// Allows both cacheControl and cache_control for flexibility
fn get_cache_control_from_metadata(
    provider_metadata: Option<&SharedProviderMetadata>,
) -> Option<AnthropicCacheControl> {
    let anthropic = provider_metadata?.get("anthropic")?;

    // Allow both cacheControl and cache_control for flexibility
    let cache_control_value = anthropic
        .get("cacheControl")
        .or_else(|| anthropic.get("cache_control"))?;

    // Try to deserialize the value into AnthropicCacheControl
    serde_json::from_value(cache_control_value.clone()).ok()
}

/// Context for cache control validation
#[derive(Debug, Clone)]
pub struct CacheControlContext {
    /// Type of the element (e.g., "user message", "tool result")
    pub element_type: String,
    /// Whether caching is allowed in this context
    pub can_cache: bool,
}

impl CacheControlContext {
    /// Creates a new cache control context
    pub fn new(element_type: impl Into<String>, can_cache: bool) -> Self {
        Self {
            element_type: element_type.into(),
            can_cache,
        }
    }
}

/// Validator for cache control settings
///
/// Tracks cache breakpoints and validates cache control usage across a request.
/// Anthropic allows a maximum of 4 cache breakpoints per request.
///
/// # Example
///
/// ```
/// use llm_kit_anthropic::get_cache_control::{CacheControlValidator, CacheControlContext};
/// use std::collections::HashMap;
///
/// let mut validator = CacheControlValidator::new();
///
/// // Validate cache control for different contexts
/// let context = CacheControlContext::new("system message", true);
/// let cache_control = validator.get_cache_control(None, &context);
///
/// // Get any warnings that were generated
/// let warnings = validator.get_warnings();
/// ```
#[derive(Debug, Default)]
pub struct CacheControlValidator {
    breakpoint_count: usize,
    warnings: Vec<CacheControlWarning>,
}

impl CacheControlValidator {
    /// Creates a new cache control validator
    pub fn new() -> Self {
        Self {
            breakpoint_count: 0,
            warnings: Vec::new(),
        }
    }

    /// Gets cache control from provider metadata with validation
    ///
    /// # Arguments
    ///
    /// * `provider_metadata` - Provider-specific metadata that may contain cache control settings
    /// * `context` - Context information about where the cache control is being applied
    ///
    /// # Returns
    ///
    /// The cache control configuration if valid, or None if invalid or not present.
    /// Warnings are added to the validator's warning list if validation fails.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::get_cache_control::{CacheControlValidator, CacheControlContext};
    /// use std::collections::HashMap;
    /// use serde_json::json;
    ///
    /// let mut validator = CacheControlValidator::new();
    ///
    /// let mut metadata = HashMap::new();
    /// let mut anthropic_data = HashMap::new();
    /// anthropic_data.insert(
    ///     "cacheControl".to_string(),
    ///     json!({
    ///         "type": "ephemeral",
    ///         "ttl": "5m"
    ///     })
    /// );
    /// metadata.insert("anthropic".to_string(), anthropic_data);
    ///
    /// let context = CacheControlContext::new("system message", true);
    /// let cache_control = validator.get_cache_control(Some(&metadata), &context);
    ///
    /// assert!(cache_control.is_some());
    /// ```
    pub fn get_cache_control(
        &mut self,
        provider_metadata: Option<&SharedProviderMetadata>,
        context: &CacheControlContext,
    ) -> Option<AnthropicCacheControl> {
        let cache_control_value = get_cache_control_from_metadata(provider_metadata)?;

        // Validate that cache_control is allowed in this context
        if !context.can_cache {
            self.warnings.push(CacheControlWarning::unsupported_setting(
                "cacheControl",
                format!(
                    "cache_control cannot be set on {}. It will be ignored.",
                    context.element_type
                ),
            ));
            return None;
        }

        // Validate cache breakpoint limit
        self.breakpoint_count += 1;
        if self.breakpoint_count > MAX_CACHE_BREAKPOINTS {
            self.warnings.push(CacheControlWarning::unsupported_setting(
                "cacheControl",
                format!(
                    "Maximum {} cache breakpoints exceeded (found {}). This breakpoint will be ignored.",
                    MAX_CACHE_BREAKPOINTS, self.breakpoint_count
                ),
            ));
            return None;
        }

        Some(cache_control_value)
    }

    /// Returns all warnings generated during validation
    pub fn get_warnings(&self) -> &[CacheControlWarning] {
        &self.warnings
    }

    /// Consumes the validator and returns the warnings
    pub fn into_warnings(self) -> Vec<CacheControlWarning> {
        self.warnings
    }

    /// Clears all warnings
    pub fn clear_warnings(&mut self) {
        self.warnings.clear();
    }

    /// Gets the current breakpoint count
    pub fn breakpoint_count(&self) -> usize {
        self.breakpoint_count
    }

    /// Resets the validator state
    pub fn reset(&mut self) {
        self.breakpoint_count = 0;
        self.warnings.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::cache_control::CacheTTL;
    use serde_json::json;

    fn create_metadata_with_cache_control(cache_control: Value) -> SharedProviderMetadata {
        let mut metadata = HashMap::new();
        let mut anthropic_data = HashMap::new();
        anthropic_data.insert("cacheControl".to_string(), cache_control);
        metadata.insert("anthropic".to_string(), anthropic_data);
        metadata
    }

    fn create_metadata_with_cache_control_snake_case(
        cache_control: Value,
    ) -> SharedProviderMetadata {
        let mut metadata = HashMap::new();
        let mut anthropic_data = HashMap::new();
        anthropic_data.insert("cache_control".to_string(), cache_control);
        metadata.insert("anthropic".to_string(), anthropic_data);
        metadata
    }

    #[test]
    fn test_get_cache_control_from_metadata_camel_case() {
        let metadata = create_metadata_with_cache_control(json!({
            "type": "ephemeral",
            "ttl": "5m"
        }));

        let cache_control = get_cache_control_from_metadata(Some(&metadata));
        assert!(cache_control.is_some());
        assert_eq!(cache_control.as_ref().unwrap().cache_type, "ephemeral");
        assert_eq!(
            cache_control.as_ref().unwrap().ttl,
            Some(CacheTTL::FiveMinutes)
        );
    }

    #[test]
    fn test_get_cache_control_from_metadata_snake_case() {
        let metadata = create_metadata_with_cache_control_snake_case(json!({
            "type": "ephemeral",
            "ttl": "1h"
        }));

        let cache_control = get_cache_control_from_metadata(Some(&metadata));
        assert!(cache_control.is_some());
        assert_eq!(cache_control.as_ref().unwrap().cache_type, "ephemeral");
        assert_eq!(cache_control.as_ref().unwrap().ttl, Some(CacheTTL::OneHour));
    }

    #[test]
    fn test_get_cache_control_from_metadata_none() {
        let cache_control = get_cache_control_from_metadata(None);
        assert!(cache_control.is_none());
    }

    #[test]
    fn test_get_cache_control_from_metadata_no_anthropic() {
        let metadata = HashMap::new();
        let cache_control = get_cache_control_from_metadata(Some(&metadata));
        assert!(cache_control.is_none());
    }

    #[test]
    fn test_validator_new() {
        let validator = CacheControlValidator::new();
        assert_eq!(validator.breakpoint_count(), 0);
        assert_eq!(validator.get_warnings().len(), 0);
    }

    #[test]
    fn test_validator_valid_cache_control() {
        let mut validator = CacheControlValidator::new();
        let metadata = create_metadata_with_cache_control(json!({
            "type": "ephemeral",
            "ttl": "5m"
        }));

        let context = CacheControlContext::new("system message", true);
        let cache_control = validator.get_cache_control(Some(&metadata), &context);

        assert!(cache_control.is_some());
        assert_eq!(validator.breakpoint_count(), 1);
        assert_eq!(validator.get_warnings().len(), 0);
    }

    #[test]
    fn test_validator_cannot_cache_context() {
        let mut validator = CacheControlValidator::new();
        let metadata = create_metadata_with_cache_control(json!({
            "type": "ephemeral"
        }));

        let context = CacheControlContext::new("tool result", false);
        let cache_control = validator.get_cache_control(Some(&metadata), &context);

        assert!(cache_control.is_none());
        assert_eq!(validator.breakpoint_count(), 0);
        assert_eq!(validator.get_warnings().len(), 1);
        assert_eq!(validator.get_warnings()[0].setting, "cacheControl");
        assert!(
            validator.get_warnings()[0]
                .details
                .contains("cannot be set on tool result")
        );
    }

    #[test]
    fn test_validator_max_breakpoints() {
        let mut validator = CacheControlValidator::new();
        let metadata = create_metadata_with_cache_control(json!({
            "type": "ephemeral"
        }));

        let context = CacheControlContext::new("message", true);

        // Add 4 valid breakpoints
        for _ in 0..4 {
            let result = validator.get_cache_control(Some(&metadata), &context);
            assert!(result.is_some());
        }

        assert_eq!(validator.breakpoint_count(), 4);
        assert_eq!(validator.get_warnings().len(), 0);

        // 5th breakpoint should fail
        let result = validator.get_cache_control(Some(&metadata), &context);
        assert!(result.is_none());
        assert_eq!(validator.breakpoint_count(), 5);
        assert_eq!(validator.get_warnings().len(), 1);
        assert!(
            validator.get_warnings()[0]
                .details
                .contains("Maximum 4 cache breakpoints exceeded")
        );
    }

    #[test]
    fn test_validator_multiple_invalid_contexts() {
        let mut validator = CacheControlValidator::new();
        let metadata = create_metadata_with_cache_control(json!({
            "type": "ephemeral"
        }));

        let context1 = CacheControlContext::new("invalid type 1", false);
        let context2 = CacheControlContext::new("invalid type 2", false);

        validator.get_cache_control(Some(&metadata), &context1);
        validator.get_cache_control(Some(&metadata), &context2);

        assert_eq!(validator.breakpoint_count(), 0);
        assert_eq!(validator.get_warnings().len(), 2);
    }

    #[test]
    fn test_validator_no_metadata() {
        let mut validator = CacheControlValidator::new();
        let context = CacheControlContext::new("message", true);

        let result = validator.get_cache_control(None, &context);
        assert!(result.is_none());
        assert_eq!(validator.breakpoint_count(), 0);
        assert_eq!(validator.get_warnings().len(), 0);
    }

    #[test]
    fn test_validator_reset() {
        let mut validator = CacheControlValidator::new();
        let metadata = create_metadata_with_cache_control(json!({
            "type": "ephemeral"
        }));

        let context = CacheControlContext::new("message", true);
        validator.get_cache_control(Some(&metadata), &context);
        validator.get_cache_control(Some(&metadata), &CacheControlContext::new("bad", false));

        assert_eq!(validator.breakpoint_count(), 1);
        assert_eq!(validator.get_warnings().len(), 1);

        validator.reset();

        assert_eq!(validator.breakpoint_count(), 0);
        assert_eq!(validator.get_warnings().len(), 0);
    }

    #[test]
    fn test_validator_clear_warnings() {
        let mut validator = CacheControlValidator::new();
        let metadata = create_metadata_with_cache_control(json!({
            "type": "ephemeral"
        }));

        let context = CacheControlContext::new("bad", false);
        validator.get_cache_control(Some(&metadata), &context);

        assert_eq!(validator.get_warnings().len(), 1);

        validator.clear_warnings();

        assert_eq!(validator.get_warnings().len(), 0);
        // Breakpoint count should remain
        assert_eq!(validator.breakpoint_count(), 0);
    }

    #[test]
    fn test_validator_into_warnings() {
        let mut validator = CacheControlValidator::new();
        let metadata = create_metadata_with_cache_control(json!({
            "type": "ephemeral"
        }));

        let context = CacheControlContext::new("bad", false);
        validator.get_cache_control(Some(&metadata), &context);

        let warnings = validator.into_warnings();
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].setting, "cacheControl");
    }

    #[test]
    fn test_cache_control_warning_creation() {
        let warning =
            CacheControlWarning::unsupported_setting("testSetting", "This is a test detail");

        assert_eq!(warning.warning_type, "unsupported-setting");
        assert_eq!(warning.setting, "testSetting");
        assert_eq!(warning.details, "This is a test detail");
    }

    #[test]
    fn test_cache_control_context() {
        let context = CacheControlContext::new("system message", true);
        assert_eq!(context.element_type, "system message");
        assert!(context.can_cache);

        let context = CacheControlContext::new("tool result", false);
        assert_eq!(context.element_type, "tool result");
        assert!(!context.can_cache);
    }

    #[test]
    fn test_validator_default() {
        let validator = CacheControlValidator::default();
        assert_eq!(validator.breakpoint_count(), 0);
        assert_eq!(validator.get_warnings().len(), 0);
    }
}
