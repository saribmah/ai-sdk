pub mod function_tool;
pub mod provider_defined_tool;

use function_tool::FunctionTool;
use provider_defined_tool::ProviderDefinedTool;
use serde::{Deserialize, Serialize};

/// Tool types available for the model.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Tool {
    Function(FunctionTool),
    ProviderDefined(ProviderDefinedTool),
}
