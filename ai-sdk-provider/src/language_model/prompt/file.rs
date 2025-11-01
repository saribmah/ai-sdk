use serde::{Deserialize, Serialize};
use crate::language_model::prompt::DataContent;
use crate::shared::provider_options::ProviderOptions;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageModelFilePart {
    /// Optional filename of the file
    #[serde(skip_serializing_if = "Option::is_none")]
    filename: Option<String>,

    /// File data. Can be binary, base64, or URL
    data: DataContent,

    /// IANA media type of the file
    media_type: String,

    /// Additional provider-specific options
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_options: Option<ProviderOptions>,
}
