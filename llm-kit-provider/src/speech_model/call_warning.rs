use serde::{Deserialize, Serialize};

/// Warning from the model provider for this call. The call will proceed, but e.g.
/// some settings might not be supported, which can lead to suboptimal results.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum SpeechModelCallWarning {
    /// A setting is not supported by the provider
    #[serde(rename_all = "camelCase")]
    UnsupportedSetting {
        /// The name of the unsupported setting.
        /// Valid settings are: "text", "voice", "outputFormat", "instructions",
        /// "speed", "language", "providerOptions", "headers", "abortSignal"
        setting: String,

        /// Additional details about why the setting is unsupported
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
    },

    /// Other warning type
    Other {
        /// The warning message
        message: String,
    },
}

impl SpeechModelCallWarning {
    /// Create an unsupported setting warning
    pub fn unsupported_setting(setting: impl Into<String>) -> Self {
        Self::UnsupportedSetting {
            setting: setting.into(),
            details: None,
        }
    }

    /// Create an unsupported setting warning with details
    pub fn unsupported_setting_with_details(
        setting: impl Into<String>,
        details: impl Into<String>,
    ) -> Self {
        Self::UnsupportedSetting {
            setting: setting.into(),
            details: Some(details.into()),
        }
    }

    /// Create an other warning
    pub fn other(message: impl Into<String>) -> Self {
        Self::Other {
            message: message.into(),
        }
    }
}

// Type-safe setting names
impl SpeechModelCallWarning {
    /// Create warning for unsupported "text" setting
    pub fn unsupported_text(details: Option<String>) -> Self {
        Self::UnsupportedSetting {
            setting: "text".to_string(),
            details,
        }
    }

    /// Create warning for unsupported "voice" setting
    pub fn unsupported_voice(details: Option<String>) -> Self {
        Self::UnsupportedSetting {
            setting: "voice".to_string(),
            details,
        }
    }

    /// Create warning for unsupported "outputFormat" setting
    pub fn unsupported_output_format(details: Option<String>) -> Self {
        Self::UnsupportedSetting {
            setting: "outputFormat".to_string(),
            details,
        }
    }

    /// Create warning for unsupported "instructions" setting
    pub fn unsupported_instructions(details: Option<String>) -> Self {
        Self::UnsupportedSetting {
            setting: "instructions".to_string(),
            details,
        }
    }

    /// Create warning for unsupported "speed" setting
    pub fn unsupported_speed(details: Option<String>) -> Self {
        Self::UnsupportedSetting {
            setting: "speed".to_string(),
            details,
        }
    }

    /// Create warning for unsupported "language" setting
    pub fn unsupported_language(details: Option<String>) -> Self {
        Self::UnsupportedSetting {
            setting: "language".to_string(),
            details,
        }
    }

    /// Create warning for unsupported "providerOptions" setting
    pub fn unsupported_provider_options(details: Option<String>) -> Self {
        Self::UnsupportedSetting {
            setting: "providerOptions".to_string(),
            details,
        }
    }

    /// Create warning for unsupported "headers" setting
    pub fn unsupported_headers(details: Option<String>) -> Self {
        Self::UnsupportedSetting {
            setting: "headers".to_string(),
            details,
        }
    }

    /// Create warning for unsupported "abortSignal" setting
    pub fn unsupported_abort_signal(details: Option<String>) -> Self {
        Self::UnsupportedSetting {
            setting: "abortSignal".to_string(),
            details,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsupported_setting() {
        let warning = SpeechModelCallWarning::unsupported_setting("voice");
        match warning {
            SpeechModelCallWarning::UnsupportedSetting { setting, details } => {
                assert_eq!(setting, "voice");
                assert_eq!(details, None);
            }
            _ => panic!("Expected UnsupportedSetting variant"),
        }
    }

    #[test]
    fn test_unsupported_setting_with_details() {
        let warning = SpeechModelCallWarning::unsupported_setting_with_details(
            "speed",
            "Speed control not available for this model",
        );
        match warning {
            SpeechModelCallWarning::UnsupportedSetting { setting, details } => {
                assert_eq!(setting, "speed");
                assert_eq!(
                    details,
                    Some("Speed control not available for this model".to_string())
                );
            }
            _ => panic!("Expected UnsupportedSetting variant"),
        }
    }

    #[test]
    fn test_type_safe_constructors() {
        let warning =
            SpeechModelCallWarning::unsupported_voice(Some("Voice not found".to_string()));
        match warning {
            SpeechModelCallWarning::UnsupportedSetting { setting, details } => {
                assert_eq!(setting, "voice");
                assert_eq!(details, Some("Voice not found".to_string()));
            }
            _ => panic!("Expected UnsupportedSetting variant"),
        }
    }

    #[test]
    fn test_other() {
        let warning = SpeechModelCallWarning::other("Custom warning message");
        match warning {
            SpeechModelCallWarning::Other { message } => {
                assert_eq!(message, "Custom warning message");
            }
            _ => panic!("Expected Other variant"),
        }
    }

    #[test]
    fn test_serialization() {
        let warning = SpeechModelCallWarning::unsupported_speed(Some("Not supported".to_string()));
        let json = serde_json::to_string(&warning).unwrap();
        assert!(json.contains("\"type\":\"unsupported-setting\""));
        assert!(json.contains("\"setting\":\"speed\""));
        assert!(json.contains("\"details\":\"Not supported\""));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{"type":"unsupported-setting","setting":"outputFormat","details":"Format not available"}"#;
        let warning: SpeechModelCallWarning = serde_json::from_str(json).unwrap();
        match warning {
            SpeechModelCallWarning::UnsupportedSetting { setting, details } => {
                assert_eq!(setting, "outputFormat");
                assert_eq!(details, Some("Format not available".to_string()));
            }
            _ => panic!("Expected UnsupportedSetting variant"),
        }
    }
}
