use serde::{Deserialize, Serialize};

/// Warning from the model provider for this call. The call will proceed, but e.g.
/// some settings might not be supported, which can lead to suboptimal results.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum TranscriptionModelCallWarning {
    /// A setting is not supported by the provider
    #[serde(rename_all = "camelCase")]
    UnsupportedSetting {
        /// The name of the unsupported setting.
        /// Valid settings are: "audio", "mediaType", "providerOptions", "headers", "abortSignal"
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

impl TranscriptionModelCallWarning {
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
impl TranscriptionModelCallWarning {
    /// Create warning for unsupported "audio" setting
    pub fn unsupported_audio(details: Option<String>) -> Self {
        Self::UnsupportedSetting {
            setting: "audio".to_string(),
            details,
        }
    }

    /// Create warning for unsupported "mediaType" setting
    pub fn unsupported_media_type(details: Option<String>) -> Self {
        Self::UnsupportedSetting {
            setting: "mediaType".to_string(),
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
        let warning = TranscriptionModelCallWarning::unsupported_setting("mediaType");
        match warning {
            TranscriptionModelCallWarning::UnsupportedSetting { setting, details } => {
                assert_eq!(setting, "mediaType");
                assert_eq!(details, None);
            }
            _ => panic!("Expected UnsupportedSetting variant"),
        }
    }

    #[test]
    fn test_unsupported_setting_with_details() {
        let warning = TranscriptionModelCallWarning::unsupported_setting_with_details(
            "audio",
            "Audio format not supported by this provider",
        );
        match warning {
            TranscriptionModelCallWarning::UnsupportedSetting { setting, details } => {
                assert_eq!(setting, "audio");
                assert_eq!(
                    details,
                    Some("Audio format not supported by this provider".to_string())
                );
            }
            _ => panic!("Expected UnsupportedSetting variant"),
        }
    }

    #[test]
    fn test_type_safe_constructors() {
        let warning = TranscriptionModelCallWarning::unsupported_media_type(Some(
            "WAV not supported".to_string(),
        ));
        match warning {
            TranscriptionModelCallWarning::UnsupportedSetting { setting, details } => {
                assert_eq!(setting, "mediaType");
                assert_eq!(details, Some("WAV not supported".to_string()));
            }
            _ => panic!("Expected UnsupportedSetting variant"),
        }
    }

    #[test]
    fn test_other() {
        let warning = TranscriptionModelCallWarning::other("Custom warning message");
        match warning {
            TranscriptionModelCallWarning::Other { message } => {
                assert_eq!(message, "Custom warning message");
            }
            _ => panic!("Expected Other variant"),
        }
    }

    #[test]
    fn test_serialization() {
        let warning =
            TranscriptionModelCallWarning::unsupported_audio(Some("Not supported".to_string()));
        let json = serde_json::to_string(&warning).unwrap();
        assert!(json.contains("\"type\":\"unsupported-setting\""));
        assert!(json.contains("\"setting\":\"audio\""));
        assert!(json.contains("\"details\":\"Not supported\""));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{"type":"unsupported-setting","setting":"mediaType","details":"Format not available"}"#;
        let warning: TranscriptionModelCallWarning = serde_json::from_str(json).unwrap();
        match warning {
            TranscriptionModelCallWarning::UnsupportedSetting { setting, details } => {
                assert_eq!(setting, "mediaType");
                assert_eq!(details, Some("Format not available".to_string()));
            }
            _ => panic!("Expected UnsupportedSetting variant"),
        }
    }
}
