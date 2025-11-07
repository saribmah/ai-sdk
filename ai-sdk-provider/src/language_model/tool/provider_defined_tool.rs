use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// The configuration of a tool that is defined by the provider.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageModelProviderDefinedTool {
    #[serde(rename = "type")]
    pub tool_type: ProviderDefinedToolType,

    pub id: String,

    pub name: String,

    pub args: HashMap<String, Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderDefinedToolType {
    #[default]
    ProviderDefined,
}

impl LanguageModelProviderDefinedTool {
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
