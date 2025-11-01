pub mod function_tool;
pub mod provider_defined_tool;

use serde::{Deserialize, Serialize};
use function_tool::FunctionTool;
use provider_defined_tool::ProviderDefinedTool;

/// Tool types available for the model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Tool {
    Function(FunctionTool),
    ProviderDefined(ProviderDefinedTool),
}
