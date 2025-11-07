use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LanguageModelToolChoice {
    Auto,
    None,
    Required,
    Tool { name: String },
}
