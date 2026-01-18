use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// The configuration of a tool that is defined by the provider
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageModelProviderDefinedTool {
    /// Type discriminator for provider-defined tools
    #[serde(rename = "type")]
    pub tool_type: ProviderDefinedToolType,

    /// Unique identifier for this tool
    pub id: String,

    /// Name of the tool
    pub name: String,

    /// Tool arguments
    pub args: HashMap<String, Value>,
}

/// Type discriminator for provider-defined tools
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderDefinedToolType {
    /// Provider-defined tool type
    #[default]
    ProviderDefined,
}

impl LanguageModelProviderDefinedTool {
    /// Create a new provider-defined tool
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        args: HashMap<String, Value>,
    ) -> Self {
        Self {
            tool_type: ProviderDefinedToolType::ProviderDefined,
            id: id.into(),
            name: name.into(),
            args,
        }
    }
}
