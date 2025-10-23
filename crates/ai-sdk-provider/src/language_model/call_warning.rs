use serde::{Deserialize, Serialize};
use crate::language_model::call_options::Tool;

/// Warning from the model provider for this call. The call will proceed, but e.g.
/// some settings might not be supported, which can lead to suboptimal results.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum CallWarning {
    /// A setting is not supported by the provider
    #[serde(rename_all = "camelCase")]
    UnsupportedSetting {
        /// The name of the unsupported setting
        setting: String,

        /// Additional details about why the setting is unsupported
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
    },

    /// A tool is not supported by the provider
    #[serde(rename_all = "camelCase")]
    UnsupportedTool {
        /// The unsupported tool
        tool: Tool,

        /// Additional details about why the tool is unsupported
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<String>,
    },

    /// Other warning type
    Other {
        /// The warning message
        message: String,
    },
}

impl CallWarning {
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

    /// Create an unsupported tool warning
    pub fn unsupported_tool(tool: Tool) -> Self {
        Self::UnsupportedTool {
            tool,
            details: None,
        }
    }

    /// Create an unsupported tool warning with details
    pub fn unsupported_tool_with_details(
        tool: Tool,
        details: impl Into<String>,
    ) -> Self {
        Self::UnsupportedTool {
            tool,
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