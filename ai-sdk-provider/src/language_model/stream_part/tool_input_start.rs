use crate::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageModelStreamToolInputStart {
    #[serde(rename = "type")]
    pub content_type: ToolInputStartType,

    /// Unique identifier for this tool call
    pub id: String,

    /// Name of the tool being called
    pub tool_name: String,

    /// Provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,

    /// Whether the tool will be executed by the provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_executed: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "tool-input-start")]
pub(crate) struct ToolInputStartType;

impl LanguageModelStreamToolInputStart {
    pub fn new(id: impl Into<String>, tool_name: impl Into<String>) -> Self {
        Self {
            content_type: ToolInputStartType,
            id: id.into(),
            tool_name: tool_name.into(),
            provider_metadata: None,
            provider_executed: None,
        }
    }

    pub fn with_options(
        id: impl Into<String>,
        tool_name: impl Into<String>,
        provider_metadata: Option<SharedProviderMetadata>,
        provider_executed: Option<bool>,
    ) -> Self {
        Self {
            content_type: ToolInputStartType,
            id: id.into(),
            tool_name: tool_name.into(),
            provider_metadata,
            provider_executed,
        }
    }
}
