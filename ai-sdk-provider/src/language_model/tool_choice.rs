use serde::{Deserialize, Serialize};

/// Tool selection mode for language model calls
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LanguageModelToolChoice {
    /// Let the model decide whether to use tools
    Auto,
    /// Prevent the model from using any tools
    None,
    /// Require the model to use at least one tool
    Required,
    /// Require the model to use a specific tool
    Tool {
        /// Name of the required tool
        name: String,
    },
}
