use serde::{Deserialize, Serialize};

/// Warning from the model that certain features are e.g. unsupported or that compatibility
/// functionality is used (which might lead to suboptimal results).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum SharedWarning {
    /// A configuration setting is not supported by the model.
    #[serde(rename_all = "camelCase")]
    UnsupportedSetting {
        /// The name of the unsupported setting
        setting: String,

        /// Additional details about why the setting is unsupported
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
    },

    /// A compatibility feature is used that might lead to suboptimal results.
    #[serde(rename_all = "camelCase")]
    Compatibility {
        /// The name of the compatibility feature being used
        feature: String,

        /// Additional details about the compatibility feature
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
    },

    /// Other warning.
    Other {
        /// The warning message
        message: String,
    },
}

impl SharedWarning {
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

    /// Create a compatibility warning
    pub fn compatibility(feature: impl Into<String>) -> Self {
        Self::Compatibility {
            feature: feature.into(),
            details: None,
        }
    }

    /// Create a compatibility warning with details
    pub fn compatibility_with_details(
        feature: impl Into<String>,
        details: impl Into<String>,
    ) -> Self {
        Self::Compatibility {
            feature: feature.into(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsupported_setting() {
        let warning = SharedWarning::unsupported_setting("temperature");
        match warning {
            SharedWarning::UnsupportedSetting { setting, details } => {
                assert_eq!(setting, "temperature");
                assert_eq!(details, None);
            }
            _ => panic!("Expected UnsupportedSetting variant"),
        }
    }

    #[test]
    fn test_unsupported_setting_with_details() {
        let warning = SharedWarning::unsupported_setting_with_details(
            "temperature",
            "This model does not support temperature settings",
        );
        match warning {
            SharedWarning::UnsupportedSetting { setting, details } => {
                assert_eq!(setting, "temperature");
                assert_eq!(
                    details,
                    Some("This model does not support temperature settings".to_string())
                );
            }
            _ => panic!("Expected UnsupportedSetting variant"),
        }
    }

    #[test]
    fn test_compatibility() {
        let warning = SharedWarning::compatibility("tool-calling");
        match warning {
            SharedWarning::Compatibility { feature, details } => {
                assert_eq!(feature, "tool-calling");
                assert_eq!(details, None);
            }
            _ => panic!("Expected Compatibility variant"),
        }
    }

    #[test]
    fn test_compatibility_with_details() {
        let warning = SharedWarning::compatibility_with_details(
            "tool-calling",
            "Tool calling is emulated and may not work perfectly",
        );
        match warning {
            SharedWarning::Compatibility { feature, details } => {
                assert_eq!(feature, "tool-calling");
                assert_eq!(
                    details,
                    Some("Tool calling is emulated and may not work perfectly".to_string())
                );
            }
            _ => panic!("Expected Compatibility variant"),
        }
    }

    #[test]
    fn test_other() {
        let warning = SharedWarning::other("Custom warning message");
        match warning {
            SharedWarning::Other { message } => {
                assert_eq!(message, "Custom warning message");
            }
            _ => panic!("Expected Other variant"),
        }
    }

    #[test]
    fn test_serialization() {
        let warning =
            SharedWarning::unsupported_setting_with_details("max_tokens", "Limit is 4096");
        let json = serde_json::to_string(&warning).unwrap();
        assert!(json.contains("\"type\":\"unsupported-setting\""));
        assert!(json.contains("\"setting\":\"max_tokens\""));
        assert!(json.contains("\"details\":\"Limit is 4096\""));
    }

    #[test]
    fn test_deserialization() {
        let json =
            r#"{"type":"unsupported-setting","setting":"temperature","details":"Not supported"}"#;
        let warning: SharedWarning = serde_json::from_str(json).unwrap();
        match warning {
            SharedWarning::UnsupportedSetting { setting, details } => {
                assert_eq!(setting, "temperature");
                assert_eq!(details, Some("Not supported".to_string()));
            }
            _ => panic!("Expected UnsupportedSetting variant"),
        }
    }
}
