use super::ProviderError;

/// Builder for constructing LoadSetting error with a fluent interface
///
/// # Examples
///
/// ```
/// use ai_sdk_provider::error::{LoadSettingErrorBuilder, ProviderError};
///
/// let error = LoadSettingErrorBuilder::new(
///     "Failed to load model configuration from config file"
/// )
/// .build();
///
/// match error {
///     ProviderError::LoadSetting { message } => {
///         assert_eq!(message, "Failed to load model configuration from config file");
///     }
///     _ => panic!("Expected LoadSetting error"),
/// }
/// ```
#[derive(Debug, Default)]
pub struct LoadSettingErrorBuilder {
    message: String,
}

impl LoadSettingErrorBuilder {
    /// Create a new LoadSettingErrorBuilder with required fields
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Build the ProviderError
    pub fn build(self) -> ProviderError {
        ProviderError::LoadSetting {
            message: self.message,
        }
    }
}

impl ProviderError {
    /// Create a new LoadSetting error
    ///
    /// # Examples
    ///
    /// ```
    /// use ai_sdk_provider::error::ProviderError;
    ///
    /// let error = ProviderError::load_setting_error(
    ///     "Configuration file not found at ~/.ai/config.json"
    /// );
    ///
    /// match error {
    ///     ProviderError::LoadSetting { message } => {
    ///         assert_eq!(message, "Configuration file not found at ~/.ai/config.json");
    ///     }
    ///     _ => panic!("Expected LoadSetting error"),
    /// }
    /// ```
    pub fn load_setting_error(message: impl Into<String>) -> Self {
        Self::LoadSetting {
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_setting_error_basic() {
        let error = ProviderError::load_setting_error("Setting is missing");

        match error {
            ProviderError::LoadSetting { message } => {
                assert_eq!(message, "Setting is missing");
            }
            _ => panic!("Expected LoadSetting error"),
        }
    }

    #[test]
    fn test_load_setting_error_builder() {
        let error = LoadSettingErrorBuilder::new("Failed to load configuration from file").build();

        match error {
            ProviderError::LoadSetting { message } => {
                assert_eq!(message, "Failed to load configuration from file");
            }
            _ => panic!("Expected LoadSetting error"),
        }
    }

    #[test]
    fn test_load_setting_error_not_retryable() {
        let error = ProviderError::load_setting_error("test");
        assert!(!error.is_retryable());
        assert!(error.status_code().is_none());
        assert!(error.url().is_none());
    }

    #[test]
    fn test_error_display() {
        let error = ProviderError::load_setting_error("Configuration not found");
        let display = format!("{}", error);
        assert!(display.contains("Load setting error"));
        assert!(display.contains("Configuration not found"));
    }

    #[test]
    fn test_different_error_messages() {
        let messages = vec![
            "MODEL_NAME setting is not configured",
            "Invalid value for 'temperature' in config file",
            "Required setting 'endpoint_url' is missing",
            "Configuration file is corrupted or unreadable",
        ];

        for msg in messages {
            let error = ProviderError::load_setting_error(msg);
            if let ProviderError::LoadSetting { message } = error {
                assert_eq!(message, msg);
            } else {
                panic!("Expected LoadSetting error");
            }
        }
    }

    #[test]
    fn test_builder_creates_same_as_direct() {
        let msg = "Setting error";
        let error1 = ProviderError::load_setting_error(msg);
        let error2 = LoadSettingErrorBuilder::new(msg).build();

        match (error1, error2) {
            (
                ProviderError::LoadSetting { message: msg1 },
                ProviderError::LoadSetting { message: msg2 },
            ) => {
                assert_eq!(msg1, msg2);
            }
            _ => panic!("Expected LoadSetting errors"),
        }
    }

    #[test]
    fn test_config_file_specific_messages() {
        let error = ProviderError::load_setting_error(
            "Failed to parse configuration file at /etc/ai/config.toml: invalid TOML syntax",
        );

        if let ProviderError::LoadSetting { message } = error {
            assert!(message.contains("config.toml"));
            assert!(message.contains("invalid TOML syntax"));
        } else {
            panic!("Expected LoadSetting error");
        }
    }

    #[test]
    fn test_setting_validation_messages() {
        let test_cases = vec![
            (
                "temperature must be between 0.0 and 2.0, got 3.5",
                "temperature",
                "3.5",
            ),
            (
                "max_tokens must be positive, got -100",
                "max_tokens",
                "-100",
            ),
            (
                "model must be one of [gpt-4, gpt-3.5-turbo], got gpt-5",
                "model",
                "gpt-5",
            ),
        ];

        for (msg, setting, value) in test_cases {
            let error = ProviderError::load_setting_error(msg);
            if let ProviderError::LoadSetting { message } = error {
                assert!(message.contains(setting));
                assert!(message.contains(value));
            } else {
                panic!("Expected LoadSetting error");
            }
        }
    }

    #[test]
    fn test_empty_message() {
        let error = ProviderError::load_setting_error("");
        match error {
            ProviderError::LoadSetting { message } => {
                assert_eq!(message, "");
            }
            _ => panic!("Expected LoadSetting error"),
        }
    }

    #[test]
    fn test_long_message() {
        let long_msg = format!(
            "Failed to load setting: {}",
            "configuration issue ".repeat(50)
        );
        let error = ProviderError::load_setting_error(&long_msg);

        if let ProviderError::LoadSetting { message } = error {
            assert_eq!(message, long_msg);
        } else {
            panic!("Expected LoadSetting error");
        }
    }

    #[test]
    fn test_multiple_settings_error() {
        let error = ProviderError::load_setting_error(
            "Multiple settings are invalid: 'api_endpoint' is empty, 'timeout' is negative, 'retry_count' exceeds maximum",
        );

        if let ProviderError::LoadSetting { message } = error {
            assert!(message.contains("api_endpoint"));
            assert!(message.contains("timeout"));
            assert!(message.contains("retry_count"));
        } else {
            panic!("Expected LoadSetting error");
        }
    }
}
