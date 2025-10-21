use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::shared::provider_options::ProviderOptions;

/// A tool has a name, a description, and a set of parameters.
///
/// Note: this is **not** the user-facing tool definition. The AI SDK methods will
/// map the user-facing tool definitions to this format.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionTool {
    /// The type of the tool (always 'function').
    #[serde(rename = "type")]
    tool_type: FunctionToolType,

    /// The name of the tool. Unique within this model call.
    pub name: String,

    /// A description of the tool. The language model uses this to understand the
    /// tool's purpose and to provide better completion suggestions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The parameters that the tool expects. The language model uses this to
    /// understand the tool's input requirements and to provide matching suggestions.
    pub input_schema: Value,

    /// The provider-specific options for the tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<ProviderOptions>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "function")]
struct FunctionToolType;

impl FunctionTool {
    /// Create a new function tool
    pub fn new(
        name: impl Into<String>,
        input_schema: Value,
    ) -> Self {
        Self {
            tool_type: FunctionToolType,
            name: name.into(),
            description: None,
            input_schema,
            provider_options: None,
        }
    }

    /// Create a function tool with all fields
    pub fn with_options(
        name: impl Into<String>,
        input_schema: Value,
        description: Option<String>,
        provider_options: Option<ProviderOptions>,
    ) -> Self {
        Self {
            tool_type: FunctionToolType,
            name: name.into(),
            description,
            input_schema,
            provider_options,
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set provider options
    pub fn with_provider_options(mut self, provider_options: ProviderOptions) -> Self {
        self.provider_options = Some(provider_options);
        self
    }
}
