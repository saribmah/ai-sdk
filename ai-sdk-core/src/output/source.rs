use ai_sdk_provider::language_model::content::source::LanguageModelSource;
use ai_sdk_provider::shared::provider_metadata::SharedProviderMetadata;
use serde::{Deserialize, Serialize};

/// Source output representing a source/reference in the generation.
///
/// This wraps the provider's Source type and adds a type discriminator.
///
/// # Example
///
/// ```no_run
/// use ai_sdk_core::output::SourceOutput;
/// use ai_sdk_provider::language_model::content::source::LanguageModelSource;
///
/// # let source = LanguageModelSource::Url {
/// #     id: "source_1".to_string(),
/// #     url: "https://example.com".to_string(),
/// #     title: None,
/// #     provider_metadata: None,
/// # };
/// let source_output = SourceOutput::new(source);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceOutput {
    /// Type discriminator (always "source").
    #[serde(rename = "type")]
    pub source_type: String,

    /// The source data from the provider.
    #[serde(flatten)]
    pub source: LanguageModelSource,

    /// Additional provider-specific metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<SharedProviderMetadata>,
}

impl SourceOutput {
    /// Creates a new source output.
    ///
    /// # Arguments
    ///
    /// * `source` - The source data from the provider
    pub fn new(source: LanguageModelSource) -> Self {
        Self {
            source_type: "source".to_string(),
            source,
            provider_metadata: None,
        }
    }

    /// Sets the provider metadata for this source output.
    ///
    /// # Arguments
    ///
    /// * `metadata` - Provider-specific metadata
    pub fn with_provider_metadata(mut self, metadata: SharedProviderMetadata) -> Self {
        self.provider_metadata = Some(metadata);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_source_output_new() {
        let source = LanguageModelSource::Url {
            id: "source_1".to_string(),
            url: "https://example.com/doc".to_string(),
            title: Some("Example Document".to_string()),
            provider_metadata: None,
        };
        let source_output = SourceOutput::new(source.clone());

        assert_eq!(source_output.source_type, "source");
        assert_eq!(source_output.source.id(), "source_1");
        assert!(source_output.provider_metadata.is_none());
    }

    #[test]
    fn test_source_output_with_provider_metadata() {
        let source = LanguageModelSource::Url {
            id: "source_1".to_string(),
            url: "https://example.com/doc".to_string(),
            title: None,
            provider_metadata: None,
        };
        let mut metadata = SharedProviderMetadata::new();
        let mut inner = HashMap::new();
        inner.insert("key".to_string(), serde_json::json!("value"));
        metadata.insert("provider".to_string(), inner);

        let source_output = SourceOutput::new(source).with_provider_metadata(metadata.clone());

        assert_eq!(source_output.provider_metadata, Some(metadata));
    }

    #[test]
    fn test_source_output_serialization() {
        let source = LanguageModelSource::Url {
            id: "source_1".to_string(),
            url: "https://example.com/doc".to_string(),
            title: None,
            provider_metadata: None,
        };
        let source_output = SourceOutput::new(source);

        let serialized = serde_json::to_value(&source_output).unwrap();

        assert_eq!(serialized["type"], "source");
        assert_eq!(serialized["sourceType"], "url");
        assert_eq!(serialized["id"], "source_1");
        assert_eq!(serialized["url"], "https://example.com/doc");
        assert!(serialized.get("providerMetadata").is_none());
    }

    #[test]
    fn test_source_output_deserialization() {
        let json = serde_json::json!({
            "type": "source",
            "sourceType": "url",
            "id": "source_1",
            "url": "https://example.com/doc"
        });

        let source_output: SourceOutput = serde_json::from_value(json).unwrap();

        assert_eq!(source_output.source_type, "source");
        assert_eq!(source_output.source.id(), "source_1");
        assert!(source_output.provider_metadata.is_none());
    }

    #[test]
    fn test_source_output_clone() {
        let source = LanguageModelSource::Document {
            id: "doc_1".to_string(),
            media_type: "text/plain".to_string(),
            title: "Test Document".to_string(),
            filename: Some("test.txt".to_string()),
            provider_metadata: None,
        };
        let source_output = SourceOutput::new(source);
        let cloned = source_output.clone();

        assert_eq!(source_output, cloned);
    }
}
