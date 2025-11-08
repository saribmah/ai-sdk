use serde::{Deserialize, Serialize};

/// Warning from the model provider for this call. The call will proceed, but e.g.
/// some settings might not be supported, which can lead to suboptimal results.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ImageModelCallWarning {
    /// A setting is not supported by the provider
    #[serde(rename_all = "camelCase")]
    UnsupportedSetting {
        /// The name of the unsupported setting.
        /// Valid settings are: "prompt", "n", "size", "aspectRatio", "seed",
        /// "providerOptions", "headers", "abortSignal"
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

impl ImageModelCallWarning {
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
impl ImageModelCallWarning {
    /// Create warning for unsupported "prompt" setting
    pub fn unsupported_prompt(details: Option<String>) -> Self {
        Self::UnsupportedSetting {
            setting: "prompt".to_string(),
            details,
        }
    }

    /// Create warning for unsupported "n" setting
    pub fn unsupported_n(details: Option<String>) -> Self {
        Self::UnsupportedSetting {
            setting: "n".to_string(),
            details,
        }
    }

    /// Create warning for unsupported "size" setting
    pub fn unsupported_size(details: Option<String>) -> Self {
        Self::UnsupportedSetting {
            setting: "size".to_string(),
            details,
        }
    }

    /// Create warning for unsupported "aspectRatio" setting
    pub fn unsupported_aspect_ratio(details: Option<String>) -> Self {
        Self::UnsupportedSetting {
            setting: "aspectRatio".to_string(),
            details,
        }
    }

    /// Create warning for unsupported "seed" setting
    pub fn unsupported_seed(details: Option<String>) -> Self {
        Self::UnsupportedSetting {
            setting: "seed".to_string(),
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
