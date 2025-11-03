use serde::{Deserialize, Serialize};

/// Data content - can be bytes, base64 string, or URL
#[derive(Debug, Clone, PartialEq)]
pub enum LanguageModelDataContent {
    /// Binary data
    Bytes(Vec<u8>),

    /// Base64 encoded string
    Base64(String),

    /// URL
    Url(url::Url),
}

// Custom serialization for DataContent
impl Serialize for LanguageModelDataContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Bytes(bytes) => {
                // Serialize as base64
                use base64::{Engine as _, engine::general_purpose};
                let encoded = general_purpose::STANDARD.encode(bytes);
                serializer.serialize_str(&encoded)
            }
            Self::Base64(s) => serializer.serialize_str(s),
            Self::Url(url) => serializer.serialize_str(url.as_str()),
        }
    }
}

impl<'de> Deserialize<'de> for LanguageModelDataContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        // Try to parse as URL first
        if let Ok(url) = url::Url::parse(&s) {
            if url.scheme() == "http" || url.scheme() == "https" || url.scheme() == "data" {
                return Ok(Self::Url(url));
            }
        }

        // Otherwise treat as base64
        Ok(Self::Base64(s))
    }
}
