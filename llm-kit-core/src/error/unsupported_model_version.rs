use crate::error::AISDKError;

/// Builder for [`AISDKError::UnsupportedModelVersion`].
///
/// # Examples
///
/// ```
/// use llm_kit_core::error::{AISDKError, UnsupportedModelVersionErrorBuilder};
///
/// let error = UnsupportedModelVersionErrorBuilder::new("v1", "openai", "gpt-4")
///     .build();
///
/// match error {
///     AISDKError::UnsupportedModelVersion { version, provider, model_id } => {
///         assert_eq!(version, "v1");
///         assert_eq!(provider, "openai");
///         assert_eq!(model_id, "gpt-4");
///     }
///     _ => panic!("Expected UnsupportedModelVersion"),
/// }
/// ```
#[derive(Debug, Clone)]
pub struct UnsupportedModelVersionErrorBuilder {
    version: String,
    provider: String,
    model_id: String,
}

impl UnsupportedModelVersionErrorBuilder {
    /// Creates a new builder for an unsupported model version error.
    ///
    /// # Arguments
    ///
    /// * `version` - The unsupported version string
    /// * `provider` - The provider name
    /// * `model_id` - The model identifier
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::UnsupportedModelVersionErrorBuilder;
    ///
    /// let builder = UnsupportedModelVersionErrorBuilder::new("v1", "openai", "gpt-4");
    /// ```
    pub fn new(
        version: impl Into<String>,
        provider: impl Into<String>,
        model_id: impl Into<String>,
    ) -> Self {
        Self {
            version: version.into(),
            provider: provider.into(),
            model_id: model_id.into(),
        }
    }

    /// Builds the [`AISDKError::UnsupportedModelVersion`] error.
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::UnsupportedModelVersionErrorBuilder;
    ///
    /// let error = UnsupportedModelVersionErrorBuilder::new("v1", "openai", "gpt-4")
    ///     .build();
    /// ```
    pub fn build(self) -> AISDKError {
        AISDKError::UnsupportedModelVersion {
            version: self.version,
            provider: self.provider,
            model_id: self.model_id,
        }
    }
}

impl AISDKError {
    /// Creates a new unsupported model version error.
    ///
    /// # Arguments
    ///
    /// * `version` - The unsupported version string
    /// * `provider` - The provider name
    /// * `model_id` - The model identifier
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::AISDKError;
    ///
    /// let error = AISDKError::unsupported_model_version("v1", "openai", "gpt-4");
    ///
    /// match error {
    ///     AISDKError::UnsupportedModelVersion { version, provider, model_id } => {
    ///         assert_eq!(version, "v1");
    ///         assert_eq!(provider, "openai");
    ///         assert_eq!(model_id, "gpt-4");
    ///     }
    ///     _ => panic!("Expected UnsupportedModelVersion"),
    /// }
    /// ```
    pub fn unsupported_model_version(
        version: impl Into<String>,
        provider: impl Into<String>,
        model_id: impl Into<String>,
    ) -> Self {
        Self::UnsupportedModelVersion {
            version: version.into(),
            provider: provider.into(),
            model_id: model_id.into(),
        }
    }

    /// Creates a builder for an unsupported model version error.
    ///
    /// # Arguments
    ///
    /// * `version` - The unsupported version string
    /// * `provider` - The provider name
    /// * `model_id` - The model identifier
    ///
    /// # Examples
    ///
    /// ```
    /// use llm_kit_core::error::AISDKError;
    ///
    /// let error = AISDKError::unsupported_model_version_builder("v1", "openai", "gpt-4")
    ///     .build();
    /// ```
    pub fn unsupported_model_version_builder(
        version: impl Into<String>,
        provider: impl Into<String>,
        model_id: impl Into<String>,
    ) -> UnsupportedModelVersionErrorBuilder {
        UnsupportedModelVersionErrorBuilder::new(version, provider, model_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsupported_model_version_simple() {
        let error = AISDKError::unsupported_model_version("v1", "openai", "gpt-4");

        match error {
            AISDKError::UnsupportedModelVersion {
                version,
                provider,
                model_id,
            } => {
                assert_eq!(version, "v1");
                assert_eq!(provider, "openai");
                assert_eq!(model_id, "gpt-4");
            }
            _ => panic!("Expected UnsupportedModelVersion"),
        }
    }

    #[test]
    fn test_unsupported_model_version_builder() {
        let error = UnsupportedModelVersionErrorBuilder::new("v1", "anthropic", "claude-3").build();

        match error {
            AISDKError::UnsupportedModelVersion {
                version,
                provider,
                model_id,
            } => {
                assert_eq!(version, "v1");
                assert_eq!(provider, "anthropic");
                assert_eq!(model_id, "claude-3");
            }
            _ => panic!("Expected UnsupportedModelVersion"),
        }
    }

    #[test]
    fn test_unsupported_model_version_builder_via_error() {
        let error =
            AISDKError::unsupported_model_version_builder("v0", "google", "gemini-pro").build();

        match error {
            AISDKError::UnsupportedModelVersion {
                version,
                provider,
                model_id,
            } => {
                assert_eq!(version, "v0");
                assert_eq!(provider, "google");
                assert_eq!(model_id, "gemini-pro");
            }
            _ => panic!("Expected UnsupportedModelVersion"),
        }
    }

    #[test]
    fn test_unsupported_model_version_display() {
        let error = AISDKError::unsupported_model_version("v1", "openai", "gpt-4");
        let display = format!("{}", error);
        assert!(display.contains("Unsupported model version"));
        assert!(display.contains("v1"));
        assert!(display.contains("openai"));
        assert!(display.contains("gpt-4"));
    }

    #[test]
    fn test_unsupported_model_version_with_different_versions() {
        let test_cases = vec![
            ("v0", "provider1", "model1"),
            ("v1", "provider2", "model2"),
            ("v3", "provider3", "model3"),
            ("legacy", "provider4", "model4"),
        ];

        for (version, provider, model_id) in test_cases {
            let error = AISDKError::unsupported_model_version(version, provider, model_id);

            match error {
                AISDKError::UnsupportedModelVersion {
                    version: v,
                    provider: p,
                    model_id: m,
                } => {
                    assert_eq!(v, version);
                    assert_eq!(p, provider);
                    assert_eq!(m, model_id);
                }
                _ => panic!("Expected UnsupportedModelVersion"),
            }
        }
    }

    #[test]
    fn test_unsupported_model_version_message_content() {
        let error = AISDKError::unsupported_model_version("v1", "test-provider", "test-model");
        let display = format!("{}", error);

        // The error message should mention v2 as the supported version
        assert!(display.contains("v2"));
        assert!(display.contains("AI SDK"));
    }

    #[test]
    fn test_unsupported_model_version_with_special_characters() {
        let error = AISDKError::unsupported_model_version(
            "v1.0-beta",
            "provider-with-dash",
            "model_with_underscore",
        );

        match error {
            AISDKError::UnsupportedModelVersion {
                version,
                provider,
                model_id,
            } => {
                assert_eq!(version, "v1.0-beta");
                assert_eq!(provider, "provider-with-dash");
                assert_eq!(model_id, "model_with_underscore");
            }
            _ => panic!("Expected UnsupportedModelVersion"),
        }
    }
}
