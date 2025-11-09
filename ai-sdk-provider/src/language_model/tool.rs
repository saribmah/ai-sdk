/// Function tool definition types.
pub mod function_tool;
/// Provider-defined tool types.
pub mod provider_defined_tool;

use function_tool::LanguageModelFunctionTool;
use provider_defined_tool::LanguageModelProviderDefinedTool;
use serde::{Deserialize, Serialize};

/// Tool types available for the model.
/// Tool types available for the model
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LanguageModelTool {
    /// User-defined function tool
    Function(LanguageModelFunctionTool),
    /// Provider-specific tool
    ProviderDefined(LanguageModelProviderDefinedTool),
}
