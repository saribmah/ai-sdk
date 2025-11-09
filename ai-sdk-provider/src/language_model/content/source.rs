use crate::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};

/// Source or reference content in a language model response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "sourceType", rename_all = "camelCase")]
pub enum LanguageModelSource {
    /// URL-based source
    #[serde(rename_all = "camelCase")]
    Url {
        /// Unique identifier for this source
        id: String,

        /// The URL of the source
        url: String,

        /// Optional title of the source
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,

        /// Additional provider-specific metadata
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<SharedProviderMetadata>,
    },

    /// Document-based source
    #[serde(rename_all = "camelCase")]
    Document {
        /// Unique identifier for this source
        id: String,

        /// IANA media type of the document
        media_type: String,

        /// Title of the document
        title: String,

        /// Optional filename of the document
        #[serde(skip_serializing_if = "Option::is_none")]
        filename: Option<String>,

        /// Additional provider-specific metadata
        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<SharedProviderMetadata>,
    },
}

impl LanguageModelSource {
    /// Get the content type string for this source
    pub fn content_type(&self) -> &str {
        "source"
    }

    /// Get the ID of this source
    pub fn id(&self) -> &str {
        match self {
            LanguageModelSource::Url { id, .. } => id.as_ref(),
            LanguageModelSource::Document { id, .. } => id.as_ref(),
        }
    }

    /// Get the provider metadata if present
    pub fn provider_metadata(&self) -> Option<&SharedProviderMetadata> {
        match self {
            LanguageModelSource::Url {
                provider_metadata, ..
            } => provider_metadata.as_ref(),
            LanguageModelSource::Document {
                provider_metadata, ..
            } => provider_metadata.as_ref(),
        }
    }

    /// Check if this source is a URL
    pub fn is_url(&self) -> bool {
        matches!(self, Self::Url { .. })
    }

    /// Check if this source is a document
    pub fn is_document(&self) -> bool {
        matches!(self, Self::Document { .. })
    }
}
