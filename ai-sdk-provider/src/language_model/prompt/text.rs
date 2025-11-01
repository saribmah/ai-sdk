use serde::{Deserialize, Serialize};
use crate::shared::provider_options::ProviderOptions;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageModelTextPart {
    /// The text content
    text: String,

    /// Additional provider-specific options
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_options: Option<ProviderOptions>,
}
