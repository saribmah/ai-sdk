pub mod function_tool;
pub mod provider_defined_tool;

use function_tool::LanguageModelFunctionTool;
use provider_defined_tool::LanguageModelProviderDefinedTool;
use serde::{Deserialize, Serialize};

/// Tool types available for the model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LanguageModelTool {
    Function(LanguageModelFunctionTool),
    ProviderDefined(LanguageModelProviderDefinedTool),
}
