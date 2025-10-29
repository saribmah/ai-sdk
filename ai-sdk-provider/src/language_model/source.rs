use crate::shared::provider_metadata::ProviderMetadata;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "sourceType", rename_all = "camelCase")]
pub enum Source {
    #[serde(rename_all = "camelCase")]
    Url {
        id: String,

        url: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,
    },

    #[serde(rename_all = "camelCase")]
    Document {
        id: String,

        media_type: String,

        title: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        filename: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        provider_metadata: Option<ProviderMetadata>,
    },
}

impl Source {
    pub fn id(&self) -> &str {
        match self {
            Source::Url { id, .. } => id.as_ref(),
            Source::Document { id, .. } => id.as_ref(),
        }
    }

    pub fn provider_metadata(&self) -> Option<&ProviderMetadata> {
        match self {
            Source::Url {
                provider_metadata, ..
            } => provider_metadata.as_ref(),
            Source::Document {
                provider_metadata, ..
            } => provider_metadata.as_ref(),
        }
    }

    pub fn is_url(&self) -> bool {
        matches!(self, Self::Url { .. })
    }

    pub fn is_document(&self) -> bool {
        matches!(self, Self::Document { .. })
    }
}
