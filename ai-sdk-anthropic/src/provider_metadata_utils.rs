//! Utilities for parsing provider-specific metadata from shared provider metadata.
//!
//! This module provides functions to extract and parse Anthropic-specific options
//! from provider metadata, including citation settings and document metadata.

use ai_sdk_provider::shared::provider_metadata::SharedProviderMetadata;
use ai_sdk_provider_utils::parse_provider_options::{SerdeSchema, parse_provider_options};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

/// Schema for Anthropic file part provider options.
///
/// These options can be provided in the provider metadata for file parts
/// to configure citations and document metadata.
#[derive(Debug, Clone, Deserialize)]
pub struct AnthropicFilePartProviderOptions {
    /// Citation settings
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citations: Option<CitationSettings>,

    /// Document title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Document context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

/// Schema for Anthropic reasoning metadata.
///
/// These options can be provided in the provider metadata for reasoning parts
/// to specify thinking signatures or redacted thinking data.
#[derive(Debug, Clone, Deserialize)]
pub struct AnthropicReasoningMetadata {
    /// Signature for thinking blocks
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,

    /// Redacted thinking data
    #[serde(rename = "redactedData", skip_serializing_if = "Option::is_none")]
    pub redacted_data: Option<String>,
}

/// Schema for Anthropic tool call provider options.
///
/// These options can be provided in the provider metadata for tool call parts
/// to specify MCP tool use information.
#[derive(Debug, Clone, Deserialize)]
pub struct AnthropicToolCallProviderOptions {
    /// Type of tool use (e.g., "mcp-tool-use")
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub tool_type: Option<String>,

    /// Server name for MCP tool use
    #[serde(rename = "serverName", skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
}

/// Citation settings for file parts.
#[derive(Debug, Clone, Deserialize)]
pub struct CitationSettings {
    /// Whether citations are enabled
    pub enabled: bool,
}

/// Document metadata extracted from provider options.
#[derive(Debug, Clone, Default)]
pub struct DocumentMetadata {
    /// Document title
    pub title: Option<String>,

    /// Document context
    pub context: Option<String>,
}

impl DocumentMetadata {
    /// Creates a new empty document metadata.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a document metadata with a title.
    pub fn with_title(title: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            context: None,
        }
    }

    /// Creates a document metadata with context.
    pub fn with_context(context: impl Into<String>) -> Self {
        Self {
            title: None,
            context: Some(context.into()),
        }
    }

    /// Creates a document metadata with both title and context.
    pub fn with_title_and_context(title: impl Into<String>, context: impl Into<String>) -> Self {
        Self {
            title: Some(title.into()),
            context: Some(context.into()),
        }
    }
}

/// Checks whether citations should be enabled based on provider metadata.
///
/// This function extracts Anthropic-specific options from the provider metadata
/// and returns whether citations are enabled. If the metadata is not present or
/// the citations setting is not specified, this returns `false`.
///
/// # Arguments
///
/// * `provider_metadata` - The shared provider metadata containing provider-specific options
///
/// # Returns
///
/// `true` if citations are explicitly enabled in the metadata, `false` otherwise.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::provider_metadata_utils::should_enable_citations;
/// use serde_json::json;
/// use std::collections::HashMap;
///
/// let mut metadata = HashMap::new();
/// let mut anthropic = HashMap::new();
/// anthropic.insert(
///     "citations".to_string(),
///     json!({ "enabled": true })
/// );
/// metadata.insert("anthropic".to_string(), anthropic);
///
/// let enabled = should_enable_citations(Some(&metadata));
/// assert!(enabled);
/// ```
///
/// # Example - No Metadata
///
/// ```
/// use ai_sdk_anthropic::provider_metadata_utils::should_enable_citations;
///
/// let enabled = should_enable_citations(None);
/// assert!(!enabled); // Returns false when no metadata is provided
/// ```
///
/// # Example - Citations Disabled
///
/// ```
/// use ai_sdk_anthropic::provider_metadata_utils::should_enable_citations;
/// use serde_json::json;
/// use std::collections::HashMap;
///
/// let mut metadata = HashMap::new();
/// let mut anthropic = HashMap::new();
/// anthropic.insert(
///     "citations".to_string(),
///     json!({ "enabled": false })
/// );
/// metadata.insert("anthropic".to_string(), anthropic);
///
/// let enabled = should_enable_citations(Some(&metadata));
/// assert!(!enabled);
/// ```
pub fn should_enable_citations(provider_metadata: Option<&SharedProviderMetadata>) -> bool {
    let schema = SerdeSchema::<AnthropicFilePartProviderOptions>::new();

    // Convert SharedProviderMetadata (nested HashMap) to the flat format expected by parse_provider_options
    let provider_options = provider_metadata.map(|metadata| {
        metadata
            .iter()
            .map(|(k, v)| {
                // Convert the inner HashMap<String, Value> to a single Value
                (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null))
            })
            .collect::<HashMap<String, Value>>()
    });

    match parse_provider_options("anthropic", provider_options.as_ref(), &schema) {
        Ok(Some(options)) => options.citations.map(|c| c.enabled).unwrap_or(false),
        Ok(None) | Err(_) => false,
    }
}

/// Extracts document metadata from provider metadata.
///
/// This function parses Anthropic-specific options from the provider metadata
/// and returns document metadata including title and context. If the metadata
/// is not present or parsing fails, this returns empty metadata.
///
/// # Arguments
///
/// * `provider_metadata` - The shared provider metadata containing provider-specific options
///
/// # Returns
///
/// A `DocumentMetadata` struct containing the title and context if present.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::provider_metadata_utils::get_document_metadata;
/// use serde_json::json;
/// use std::collections::HashMap;
///
/// let mut metadata = HashMap::new();
/// let mut anthropic = HashMap::new();
/// anthropic.insert("title".to_string(), json!("My Document"));
/// anthropic.insert("context".to_string(), json!("Additional context"));
/// metadata.insert("anthropic".to_string(), anthropic);
///
/// let doc_metadata = get_document_metadata(Some(&metadata));
/// assert_eq!(doc_metadata.title, Some("My Document".to_string()));
/// assert_eq!(doc_metadata.context, Some("Additional context".to_string()));
/// ```
///
/// # Example - No Metadata
///
/// ```
/// use ai_sdk_anthropic::provider_metadata_utils::get_document_metadata;
///
/// let doc_metadata = get_document_metadata(None);
/// assert!(doc_metadata.title.is_none());
/// assert!(doc_metadata.context.is_none());
/// ```
///
/// # Example - Partial Metadata
///
/// ```
/// use ai_sdk_anthropic::provider_metadata_utils::get_document_metadata;
/// use serde_json::json;
/// use std::collections::HashMap;
///
/// let mut metadata = HashMap::new();
/// let mut anthropic = HashMap::new();
/// anthropic.insert("title".to_string(), json!("My Document"));
/// // No context provided
/// metadata.insert("anthropic".to_string(), anthropic);
///
/// let doc_metadata = get_document_metadata(Some(&metadata));
/// assert_eq!(doc_metadata.title, Some("My Document".to_string()));
/// assert!(doc_metadata.context.is_none());
/// ```
pub fn get_document_metadata(
    provider_metadata: Option<&SharedProviderMetadata>,
) -> DocumentMetadata {
    let schema = SerdeSchema::<AnthropicFilePartProviderOptions>::new();

    // Convert SharedProviderMetadata (nested HashMap) to the flat format expected by parse_provider_options
    let provider_options = provider_metadata.map(|metadata| {
        metadata
            .iter()
            .map(|(k, v)| {
                // Convert the inner HashMap<String, Value> to a single Value
                (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null))
            })
            .collect::<HashMap<String, Value>>()
    });

    match parse_provider_options("anthropic", provider_options.as_ref(), &schema) {
        Ok(Some(options)) => DocumentMetadata {
            title: options.title,
            context: options.context,
        },
        Ok(None) | Err(_) => DocumentMetadata::default(),
    }
}

/// Parses reasoning metadata from provider metadata.
///
/// This function extracts Anthropic-specific reasoning options from the provider metadata,
/// including thinking signatures and redacted thinking data.
///
/// # Arguments
///
/// * `provider_metadata` - The shared provider metadata containing provider-specific options
///
/// # Returns
///
/// An `Option<AnthropicReasoningMetadata>` containing the reasoning metadata if present.
pub fn get_reasoning_metadata(
    provider_metadata: Option<&SharedProviderMetadata>,
) -> Option<AnthropicReasoningMetadata> {
    let schema = SerdeSchema::<AnthropicReasoningMetadata>::new();

    // Convert SharedProviderMetadata to the format expected by parse_provider_options
    let provider_options = provider_metadata.map(|metadata| {
        metadata
            .iter()
            .map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null)))
            .collect::<HashMap<String, Value>>()
    });

    match parse_provider_options("anthropic", provider_options.as_ref(), &schema) {
        Ok(Some(options)) => Some(options),
        Ok(None) | Err(_) => None,
    }
}

/// Parses tool call provider options from provider metadata.
///
/// This function extracts Anthropic-specific tool call options from the provider metadata,
/// including MCP tool use information.
///
/// # Arguments
///
/// * `provider_metadata` - The shared provider metadata containing provider-specific options
///
/// # Returns
///
/// An `Option<AnthropicToolCallProviderOptions>` containing the tool call options if present.
pub fn get_tool_call_options(
    provider_metadata: Option<&SharedProviderMetadata>,
) -> Option<AnthropicToolCallProviderOptions> {
    let schema = SerdeSchema::<AnthropicToolCallProviderOptions>::new();

    // Convert SharedProviderMetadata to the format expected by parse_provider_options
    let provider_options = provider_metadata.map(|metadata| {
        metadata
            .iter()
            .map(|(k, v)| (k.clone(), serde_json::to_value(v).unwrap_or(Value::Null)))
            .collect::<HashMap<String, Value>>()
    });

    match parse_provider_options("anthropic", provider_options.as_ref(), &schema) {
        Ok(Some(options)) => Some(options),
        Ok(None) | Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;

    fn create_metadata_with_citations(enabled: bool) -> SharedProviderMetadata {
        let mut metadata = HashMap::new();
        let mut anthropic = HashMap::new();
        anthropic.insert("citations".to_string(), json!({ "enabled": enabled }));
        metadata.insert("anthropic".to_string(), anthropic);
        metadata
    }

    fn create_metadata_with_document(
        title: Option<&str>,
        context: Option<&str>,
    ) -> SharedProviderMetadata {
        let mut metadata = HashMap::new();
        let mut anthropic = HashMap::new();

        if let Some(t) = title {
            anthropic.insert("title".to_string(), json!(t));
        }
        if let Some(c) = context {
            anthropic.insert("context".to_string(), json!(c));
        }

        metadata.insert("anthropic".to_string(), anthropic);
        metadata
    }

    #[test]
    fn test_should_enable_citations_true() {
        let metadata = create_metadata_with_citations(true);
        assert!(should_enable_citations(Some(&metadata)));
    }

    #[test]
    fn test_should_enable_citations_false() {
        let metadata = create_metadata_with_citations(false);
        assert!(!should_enable_citations(Some(&metadata)));
    }

    #[test]
    fn test_should_enable_citations_none() {
        assert!(!should_enable_citations(None));
    }

    #[test]
    fn test_should_enable_citations_no_anthropic() {
        let metadata = HashMap::new();
        assert!(!should_enable_citations(Some(&metadata)));
    }

    #[test]
    fn test_should_enable_citations_no_citations_field() {
        let mut metadata = HashMap::new();
        let mut anthropic = HashMap::new();
        anthropic.insert("other_field".to_string(), json!("value"));
        metadata.insert("anthropic".to_string(), anthropic);

        assert!(!should_enable_citations(Some(&metadata)));
    }

    #[test]
    fn test_get_document_metadata_with_both() {
        let metadata =
            create_metadata_with_document(Some("Test Document"), Some("This is context"));

        let doc_metadata = get_document_metadata(Some(&metadata));
        assert_eq!(doc_metadata.title, Some("Test Document".to_string()));
        assert_eq!(doc_metadata.context, Some("This is context".to_string()));
    }

    #[test]
    fn test_get_document_metadata_with_title_only() {
        let metadata = create_metadata_with_document(Some("Test Document"), None);

        let doc_metadata = get_document_metadata(Some(&metadata));
        assert_eq!(doc_metadata.title, Some("Test Document".to_string()));
        assert!(doc_metadata.context.is_none());
    }

    #[test]
    fn test_get_document_metadata_with_context_only() {
        let metadata = create_metadata_with_document(None, Some("Context only"));

        let doc_metadata = get_document_metadata(Some(&metadata));
        assert!(doc_metadata.title.is_none());
        assert_eq!(doc_metadata.context, Some("Context only".to_string()));
    }

    #[test]
    fn test_get_document_metadata_none() {
        let doc_metadata = get_document_metadata(None);
        assert!(doc_metadata.title.is_none());
        assert!(doc_metadata.context.is_none());
    }

    #[test]
    fn test_get_document_metadata_no_anthropic() {
        let metadata = HashMap::new();
        let doc_metadata = get_document_metadata(Some(&metadata));
        assert!(doc_metadata.title.is_none());
        assert!(doc_metadata.context.is_none());
    }

    #[test]
    fn test_get_document_metadata_empty_anthropic() {
        let mut metadata = HashMap::new();
        let anthropic = HashMap::new();
        metadata.insert("anthropic".to_string(), anthropic);

        let doc_metadata = get_document_metadata(Some(&metadata));
        assert!(doc_metadata.title.is_none());
        assert!(doc_metadata.context.is_none());
    }

    #[test]
    fn test_combined_metadata() {
        let mut metadata = HashMap::new();
        let mut anthropic = HashMap::new();
        anthropic.insert("citations".to_string(), json!({ "enabled": true }));
        anthropic.insert("title".to_string(), json!("My Doc"));
        anthropic.insert("context".to_string(), json!("My Context"));
        metadata.insert("anthropic".to_string(), anthropic);

        assert!(should_enable_citations(Some(&metadata)));

        let doc_metadata = get_document_metadata(Some(&metadata));
        assert_eq!(doc_metadata.title, Some("My Doc".to_string()));
        assert_eq!(doc_metadata.context, Some("My Context".to_string()));
    }

    #[test]
    fn test_document_metadata_new() {
        let metadata = DocumentMetadata::new();
        assert!(metadata.title.is_none());
        assert!(metadata.context.is_none());
    }

    #[test]
    fn test_document_metadata_with_title() {
        let metadata = DocumentMetadata::with_title("Test");
        assert_eq!(metadata.title, Some("Test".to_string()));
        assert!(metadata.context.is_none());
    }

    #[test]
    fn test_document_metadata_with_context() {
        let metadata = DocumentMetadata::with_context("Context");
        assert!(metadata.title.is_none());
        assert_eq!(metadata.context, Some("Context".to_string()));
    }

    #[test]
    fn test_document_metadata_with_title_and_context() {
        let metadata = DocumentMetadata::with_title_and_context("Title", "Context");
        assert_eq!(metadata.title, Some("Title".to_string()));
        assert_eq!(metadata.context, Some("Context".to_string()));
    }

    #[test]
    fn test_citation_settings_deserialize() {
        let json = json!({ "enabled": true });
        let settings: CitationSettings = serde_json::from_value(json).unwrap();
        assert!(settings.enabled);
    }

    #[test]
    fn test_file_part_options_deserialize_full() {
        let json = json!({
            "citations": { "enabled": true },
            "title": "Document Title",
            "context": "Document Context"
        });

        let options: AnthropicFilePartProviderOptions = serde_json::from_value(json).unwrap();

        assert!(options.citations.is_some());
        assert!(options.citations.unwrap().enabled);
        assert_eq!(options.title, Some("Document Title".to_string()));
        assert_eq!(options.context, Some("Document Context".to_string()));
    }

    #[test]
    fn test_file_part_options_deserialize_partial() {
        let json = json!({
            "title": "Only Title"
        });

        let options: AnthropicFilePartProviderOptions = serde_json::from_value(json).unwrap();

        assert!(options.citations.is_none());
        assert_eq!(options.title, Some("Only Title".to_string()));
        assert!(options.context.is_none());
    }

    #[test]
    fn test_multiple_providers_in_metadata() {
        let mut metadata = HashMap::new();

        // Add OpenAI metadata
        let mut openai = HashMap::new();
        openai.insert("organization".to_string(), json!("my-org"));
        metadata.insert("openai".to_string(), openai);

        // Add Anthropic metadata
        let mut anthropic = HashMap::new();
        anthropic.insert("citations".to_string(), json!({ "enabled": true }));
        anthropic.insert("title".to_string(), json!("Doc"));
        metadata.insert("anthropic".to_string(), anthropic);

        // Should only parse Anthropic metadata
        assert!(should_enable_citations(Some(&metadata)));
        let doc_metadata = get_document_metadata(Some(&metadata));
        assert_eq!(doc_metadata.title, Some("Doc".to_string()));
    }

    #[test]
    fn test_invalid_citations_format() {
        let mut metadata = HashMap::new();
        let mut anthropic = HashMap::new();
        anthropic.insert("citations".to_string(), json!("invalid_format"));
        metadata.insert("anthropic".to_string(), anthropic);

        // Should return false on parse error
        assert!(!should_enable_citations(Some(&metadata)));
    }

    #[test]
    fn test_invalid_document_format() {
        let mut metadata = HashMap::new();
        let mut anthropic = HashMap::new();
        anthropic.insert("title".to_string(), json!(123)); // Wrong type
        metadata.insert("anthropic".to_string(), anthropic);

        // Should return default on parse error
        let doc_metadata = get_document_metadata(Some(&metadata));
        assert!(doc_metadata.title.is_none());
        assert!(doc_metadata.context.is_none());
    }
}
