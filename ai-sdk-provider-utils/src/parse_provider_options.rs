//! Provider options parsing utilities
//!
//! This module provides functionality to extract and parse provider-specific options
//! from a generic provider options map.

use llm_kit_provider::error::ProviderError;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;

/// A flexible schema trait for validating and parsing values.
///
/// This trait abstracts over different validation approaches:
/// - Direct deserialization via serde
/// - Custom validation logic
/// - Schema-based validation
///
/// # Type Parameters
///
/// * `T` - The target type to parse into
pub trait Schema<T> {
    /// Validate and parse a JSON value into the target type.
    ///
    /// # Arguments
    ///
    /// * `value` - The JSON value to validate and parse
    ///
    /// # Returns
    ///
    /// `Ok(T)` if validation succeeds, or `Err(ProviderError)` if validation fails.
    #[allow(clippy::result_large_err)]
    fn validate(&self, value: Value) -> Result<T, ProviderError>;
}

/// A schema implementation that uses serde for deserialization.
///
/// This is the most common schema type, which simply deserializes
/// a JSON value into the target type using serde.
#[derive(Debug, Clone, Copy, Default)]
pub struct SerdeSchema<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> SerdeSchema<T> {
    /// Create a new serde-based schema.
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: DeserializeOwned> Schema<T> for SerdeSchema<T> {
    fn validate(&self, value: Value) -> Result<T, ProviderError> {
        serde_json::from_value(value.clone()).map_err(|e| {
            ProviderError::type_validation_error(
                serde_json::to_string(&value).unwrap_or_else(|_| "invalid value".to_string()),
                e,
            )
        })
    }
}

/// Result of a safe validation operation.
#[derive(Debug)]
pub struct ValidationResult<T> {
    /// Whether the validation succeeded
    pub success: bool,
    /// The parsed value if validation succeeded
    pub value: Option<T>,
    /// The validation error if validation failed
    pub error: Option<ProviderError>,
}

impl<T> ValidationResult<T> {
    /// Create a successful validation result.
    pub fn success(value: T) -> Self {
        Self {
            success: true,
            value: Some(value),
            error: None,
        }
    }

    /// Create a failed validation result.
    pub fn failure(error: ProviderError) -> Self {
        Self {
            success: false,
            value: None,
            error: Some(error),
        }
    }
}

/// Safely validates a value against a schema without panicking.
///
/// This function wraps the schema validation in a safe error-handling context,
/// ensuring that any validation failures are captured and returned as errors
/// rather than panicking.
///
/// # Type Parameters
///
/// * `T` - The target type to validate into
/// * `S` - The schema type implementing validation
///
/// # Arguments
///
/// * `value` - The JSON value to validate
/// * `schema` - The schema to validate against
///
/// # Returns
///
/// A `ValidationResult<T>` containing either the successfully parsed value
/// or the validation error.
///
/// # Example
///
/// ```
/// use llm_kit_provider_utils::parse_provider_options::{safe_validate_types, SerdeSchema};
/// use serde::Deserialize;
/// use serde_json::json;
///
/// #[derive(Debug, Deserialize, PartialEq)]
/// struct Config {
///     temperature: f64,
///     max_tokens: u32,
/// }
///
/// let value = json!({
///     "temperature": 0.7,
///     "max_tokens": 100
/// });
///
/// let schema = SerdeSchema::<Config>::new();
/// let result = safe_validate_types(value, &schema);
///
/// assert!(result.success);
/// assert_eq!(result.value.unwrap().temperature, 0.7);
/// ```
pub fn safe_validate_types<T, S>(value: Value, schema: &S) -> ValidationResult<T>
where
    S: Schema<T>,
{
    match schema.validate(value) {
        Ok(parsed_value) => ValidationResult::success(parsed_value),
        Err(error) => ValidationResult::failure(error),
    }
}

/// Parses provider-specific options from a provider options map.
///
/// This function extracts options for a specific provider from a generic
/// options map and validates them against a schema. If the provider options
/// are not present or are null, it returns `Ok(None)`. If validation fails,
/// it returns an `InvalidArgument` error.
///
/// # Type Parameters
///
/// * `T` - The type to parse the provider options into
/// * `S` - The schema type for validation
///
/// # Arguments
///
/// * `provider` - The provider name to look up in the options map
/// * `provider_options` - The map of provider-specific options
/// * `schema` - The schema to validate the options against
///
/// # Returns
///
/// * `Ok(Some(T))` - Provider options found and successfully parsed
/// * `Ok(None)` - Provider options not present or null
/// * `Err(ProviderError)` - Validation failed
///
/// # Errors
///
/// Returns an `InvalidArgument` error if the provider options exist but fail
/// validation according to the schema.
///
/// # Example
///
/// ```
/// use llm_kit_provider_utils::parse_provider_options::{parse_provider_options, SerdeSchema};
/// use serde::Deserialize;
/// use serde_json::{json, Value};
/// use std::collections::HashMap;
///
/// #[derive(Debug, Deserialize, PartialEq)]
/// struct OpenAIOptions {
///     organization: String,
///     project: Option<String>,
/// }
///
/// let mut provider_options = HashMap::new();
/// provider_options.insert(
///     "openai".to_string(),
///     json!({
///         "organization": "my-org",
///         "project": "my-project"
///     })
/// );
///
/// let schema = SerdeSchema::<OpenAIOptions>::new();
/// let result = parse_provider_options(
///     "openai",
///     Some(&provider_options),
///     &schema
/// );
///
/// assert!(result.is_ok());
/// let options = result.unwrap();
/// assert!(options.is_some());
/// assert_eq!(options.unwrap().organization, "my-org");
/// ```
///
/// # Example - Missing Provider Options
///
/// ```
/// use llm_kit_provider_utils::parse_provider_options::{parse_provider_options, SerdeSchema};
/// use serde::Deserialize;
/// use std::collections::HashMap;
///
/// #[derive(Debug, Deserialize)]
/// struct AnthropicOptions {
///     api_version: String,
/// }
///
/// let provider_options: HashMap<String, serde_json::Value> = HashMap::new();
/// let schema = SerdeSchema::<AnthropicOptions>::new();
///
/// let result = parse_provider_options(
///     "anthropic",
///     Some(&provider_options),
///     &schema
/// );
///
/// assert!(result.is_ok());
/// assert!(result.unwrap().is_none()); // No options for "anthropic"
/// ```
///
/// # Example - Validation Error
///
/// ```
/// use llm_kit_provider_utils::parse_provider_options::{parse_provider_options, SerdeSchema};
/// use llm_kit_provider::error::ProviderError;
/// use serde::Deserialize;
/// use serde_json::json;
/// use std::collections::HashMap;
///
/// #[derive(Debug, Deserialize)]
/// struct Config {
///     temperature: f64, // Required field
/// }
///
/// let mut provider_options = HashMap::new();
/// provider_options.insert(
///     "test".to_string(),
///     json!({ "invalid_field": "value" }) // Missing "temperature"
/// );
///
/// let schema = SerdeSchema::<Config>::new();
/// let result = parse_provider_options(
///     "test",
///     Some(&provider_options),
///     &schema
/// );
///
/// assert!(result.is_err());
/// match result.unwrap_err() {
///     ProviderError::InvalidArgument { argument, message, .. } => {
///         assert_eq!(argument, "providerOptions");
///         assert!(message.contains("invalid test provider options"));
///     }
///     _ => panic!("Expected InvalidArgument error"),
/// }
/// ```
#[allow(clippy::result_large_err)]
pub fn parse_provider_options<T, S>(
    provider: &str,
    provider_options: Option<&HashMap<String, Value>>,
    schema: &S,
) -> Result<Option<T>, ProviderError>
where
    S: Schema<T>,
{
    // Check if provider_options exists and contains the provider key
    let provider_value = match provider_options {
        Some(options) => match options.get(provider) {
            Some(value) if !value.is_null() => value.clone(),
            _ => return Ok(None),
        },
        None => return Ok(None),
    };

    // Validate the provider options using the schema
    let parsed_options = safe_validate_types(provider_value, schema);

    if !parsed_options.success {
        return Err(ProviderError::invalid_argument_with_cause(
            "providerOptions",
            format!("invalid {} provider options", provider),
            parsed_options
                .error
                .unwrap_or_else(|| ProviderError::type_validation_error_without_cause("unknown")),
        ));
    }

    Ok(parsed_options.value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestConfig {
        api_key: String,
        timeout: Option<u64>,
    }

    #[test]
    fn test_serde_schema_valid() {
        let value = json!({
            "api_key": "test-key",
            "timeout": 30
        });

        let schema = SerdeSchema::<TestConfig>::new();
        let result = schema.validate(value);

        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.api_key, "test-key");
        assert_eq!(config.timeout, Some(30));
    }

    #[test]
    fn test_serde_schema_invalid() {
        let value = json!({
            "invalid": "field"
        });

        let schema = SerdeSchema::<TestConfig>::new();
        let result = schema.validate(value);

        assert!(result.is_err());
        match result.unwrap_err() {
            ProviderError::TypeValidation { .. } => {}
            _ => panic!("Expected TypeValidation error"),
        }
    }

    #[test]
    fn test_safe_validate_types_success() {
        let value = json!({
            "api_key": "my-key",
            "timeout": 60
        });

        let schema = SerdeSchema::<TestConfig>::new();
        let result = safe_validate_types(value, &schema);

        assert!(result.success);
        assert!(result.value.is_some());
        assert!(result.error.is_none());

        let config = result.value.unwrap();
        assert_eq!(config.api_key, "my-key");
        assert_eq!(config.timeout, Some(60));
    }

    #[test]
    fn test_safe_validate_types_failure() {
        let value = json!({
            "wrong": "structure"
        });

        let schema = SerdeSchema::<TestConfig>::new();
        let result = safe_validate_types(value, &schema);

        assert!(!result.success);
        assert!(result.value.is_none());
        assert!(result.error.is_some());
    }

    #[test]
    fn test_parse_provider_options_success() {
        let mut options = HashMap::new();
        options.insert(
            "openai".to_string(),
            json!({
                "api_key": "sk-test",
                "timeout": 120
            }),
        );

        let schema = SerdeSchema::<TestConfig>::new();
        let result = parse_provider_options("openai", Some(&options), &schema);

        assert!(result.is_ok());
        let config = result.unwrap();
        assert!(config.is_some());

        let config = config.unwrap();
        assert_eq!(config.api_key, "sk-test");
        assert_eq!(config.timeout, Some(120));
    }

    #[test]
    fn test_parse_provider_options_missing_provider() {
        let options = HashMap::new();

        let schema = SerdeSchema::<TestConfig>::new();
        let result = parse_provider_options("openai", Some(&options), &schema);

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_parse_provider_options_null_value() {
        let mut options = HashMap::new();
        options.insert("openai".to_string(), Value::Null);

        let schema = SerdeSchema::<TestConfig>::new();
        let result = parse_provider_options("openai", Some(&options), &schema);

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_parse_provider_options_none() {
        let schema = SerdeSchema::<TestConfig>::new();
        let result = parse_provider_options::<TestConfig, _>("openai", None, &schema);

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_parse_provider_options_validation_error() {
        let mut options = HashMap::new();
        options.insert(
            "anthropic".to_string(),
            json!({
                "invalid_structure": true
            }),
        );

        let schema = SerdeSchema::<TestConfig>::new();
        let result = parse_provider_options("anthropic", Some(&options), &schema);

        assert!(result.is_err());
        match result.unwrap_err() {
            ProviderError::InvalidArgument {
                argument, message, ..
            } => {
                assert_eq!(argument, "providerOptions");
                assert!(message.contains("invalid anthropic provider options"));
            }
            _ => panic!("Expected InvalidArgument error"),
        }
    }

    #[test]
    fn test_parse_provider_options_multiple_providers() {
        let mut options = HashMap::new();
        options.insert(
            "openai".to_string(),
            json!({
                "api_key": "openai-key",
                "timeout": 30
            }),
        );
        options.insert(
            "anthropic".to_string(),
            json!({
                "api_key": "anthropic-key",
                "timeout": 60
            }),
        );

        let schema = SerdeSchema::<TestConfig>::new();

        // Parse openai options
        let result1 = parse_provider_options("openai", Some(&options), &schema);
        assert!(result1.is_ok());
        let config1 = result1.unwrap().unwrap();
        assert_eq!(config1.api_key, "openai-key");

        // Parse anthropic options
        let result2 = parse_provider_options("anthropic", Some(&options), &schema);
        assert!(result2.is_ok());
        let config2 = result2.unwrap().unwrap();
        assert_eq!(config2.api_key, "anthropic-key");
    }

    #[test]
    fn test_validation_result_success() {
        let result = ValidationResult::success(42);
        assert!(result.success);
        assert_eq!(result.value, Some(42));
        assert!(result.error.is_none());
    }

    #[test]
    fn test_validation_result_failure() {
        let error = ProviderError::invalid_argument("test", "test error");
        let result = ValidationResult::<()>::failure(error);
        assert!(!result.success);
        assert!(result.value.is_none());
        assert!(result.error.is_some());
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct ComplexConfig {
        nested: NestedConfig,
        array: Vec<String>,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct NestedConfig {
        value: i32,
    }

    #[test]
    fn test_parse_complex_structure() {
        let mut options = HashMap::new();
        options.insert(
            "provider".to_string(),
            json!({
                "nested": {
                    "value": 42
                },
                "array": ["one", "two", "three"]
            }),
        );

        let schema = SerdeSchema::<ComplexConfig>::new();
        let result = parse_provider_options("provider", Some(&options), &schema);

        assert!(result.is_ok());
        let config = result.unwrap().unwrap();
        assert_eq!(config.nested.value, 42);
        assert_eq!(config.array, vec!["one", "two", "three"]);
    }

    #[test]
    fn test_serde_schema_default() {
        // SerdeSchema implements Default regardless of T's Default implementation
        let _schema: SerdeSchema<String> = SerdeSchema::default();
    }
}
