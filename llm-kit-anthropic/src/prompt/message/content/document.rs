use serde::{Deserialize, Serialize};

use crate::prompt::message::cache_control::AnthropicCacheControl;
use crate::prompt::message::content::source_type::AnthropicContentSource;

/// Citations configuration for document content.
///
/// Controls whether citations are enabled for the document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentCitations {
    /// Whether citations are enabled
    pub enabled: bool,
}

impl DocumentCitations {
    /// Creates a new citations configuration.
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether citations should be enabled
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    /// Creates a citations configuration with citations enabled.
    pub fn enabled() -> Self {
        Self { enabled: true }
    }

    /// Creates a citations configuration with citations disabled.
    pub fn disabled() -> Self {
        Self { enabled: false }
    }
}

/// Anthropic document content block.
///
/// Represents a document content block in an Anthropic message. Documents can be provided
/// as base64-encoded data, URLs, or plain text. Documents support additional metadata like
/// title, context, and citation configuration.
///
/// # Example
///
/// ```
/// use llm_kit_anthropic::prompt::message::content::document::AnthropicDocumentContent;
/// use llm_kit_anthropic::prompt::message::content::source_type::AnthropicContentSource;
///
/// // Create a document from a URL
/// let document = AnthropicDocumentContent::new(
///     AnthropicContentSource::url("https://example.com/document.pdf")
/// );
///
/// // Create a document with title and citations
/// use llm_kit_anthropic::prompt::message::content::document::DocumentCitations;
///
/// let document = AnthropicDocumentContent::new(
///     AnthropicContentSource::url("https://example.com/research.pdf")
/// )
/// .with_title("Research Paper")
/// .with_context("This is a scientific research paper about AI")
/// .with_citations(DocumentCitations::enabled());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicDocumentContent {
    /// The type of content (always "document")
    #[serde(rename = "type")]
    pub content_type: String,

    /// The document source (base64, URL, or text)
    pub source: AnthropicContentSource,

    /// Optional title for the document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Optional context about the document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,

    /// Optional citations configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citations: Option<DocumentCitations>,

    /// Optional cache control configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}

impl AnthropicDocumentContent {
    /// Creates a new document content block.
    ///
    /// # Arguments
    ///
    /// * `source` - The document source (base64, URL, or text)
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::document::AnthropicDocumentContent;
    /// use llm_kit_anthropic::prompt::message::content::source_type::AnthropicContentSource;
    ///
    /// let document = AnthropicDocumentContent::new(
    ///     AnthropicContentSource::url("https://example.com/doc.pdf")
    /// );
    /// ```
    pub fn new(source: AnthropicContentSource) -> Self {
        Self {
            content_type: "document".to_string(),
            source,
            title: None,
            context: None,
            citations: None,
            cache_control: None,
        }
    }

    /// Creates a new document content block from a URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The document URL
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::document::AnthropicDocumentContent;
    ///
    /// let document = AnthropicDocumentContent::from_url("https://example.com/file.pdf");
    /// ```
    pub fn from_url(url: impl Into<String>) -> Self {
        Self::new(AnthropicContentSource::url(url))
    }

    /// Creates a new document content block from base64-encoded data.
    ///
    /// # Arguments
    ///
    /// * `media_type` - The MIME type (e.g., "application/pdf")
    /// * `data` - The base64-encoded document data
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::document::AnthropicDocumentContent;
    ///
    /// let document = AnthropicDocumentContent::from_base64("application/pdf", "base64data...");
    /// ```
    pub fn from_base64(media_type: impl Into<String>, data: impl Into<String>) -> Self {
        Self::new(AnthropicContentSource::base64(media_type, data))
    }

    /// Creates a new document content block from plain text.
    ///
    /// # Arguments
    ///
    /// * `text` - The plain text content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::document::AnthropicDocumentContent;
    ///
    /// let document = AnthropicDocumentContent::from_text("This is the document content");
    /// ```
    pub fn from_text(text: impl Into<String>) -> Self {
        Self::new(AnthropicContentSource::text(text))
    }

    /// Sets the title for this document.
    ///
    /// # Arguments
    ///
    /// * `title` - The document title
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::document::AnthropicDocumentContent;
    ///
    /// let document = AnthropicDocumentContent::from_url("https://example.com/doc.pdf")
    ///     .with_title("Important Document");
    /// ```
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the context for this document.
    ///
    /// # Arguments
    ///
    /// * `context` - Context information about the document
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::document::AnthropicDocumentContent;
    ///
    /// let document = AnthropicDocumentContent::from_url("https://example.com/doc.pdf")
    ///     .with_context("This document contains the company's financial statements");
    /// ```
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Sets the citations configuration for this document.
    ///
    /// # Arguments
    ///
    /// * `citations` - The citations configuration
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::document::{AnthropicDocumentContent, DocumentCitations};
    ///
    /// let document = AnthropicDocumentContent::from_url("https://example.com/doc.pdf")
    ///     .with_citations(DocumentCitations::enabled());
    /// ```
    pub fn with_citations(mut self, citations: DocumentCitations) -> Self {
        self.citations = Some(citations);
        self
    }

    /// Enables citations for this document.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::document::AnthropicDocumentContent;
    ///
    /// let document = AnthropicDocumentContent::from_url("https://example.com/doc.pdf")
    ///     .with_citations_enabled();
    /// ```
    pub fn with_citations_enabled(self) -> Self {
        self.with_citations(DocumentCitations::enabled())
    }

    /// Disables citations for this document.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::document::AnthropicDocumentContent;
    ///
    /// let document = AnthropicDocumentContent::from_url("https://example.com/doc.pdf")
    ///     .with_citations_disabled();
    /// ```
    pub fn with_citations_disabled(self) -> Self {
        self.with_citations(DocumentCitations::disabled())
    }

    /// Sets the cache control for this document content block.
    ///
    /// # Arguments
    ///
    /// * `cache_control` - The cache control configuration
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::document::AnthropicDocumentContent;
    /// use llm_kit_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    ///
    /// let document = AnthropicDocumentContent::from_url("https://example.com/doc.pdf")
    ///     .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
    /// ```
    pub fn with_cache_control(mut self, cache_control: AnthropicCacheControl) -> Self {
        self.cache_control = Some(cache_control);
        self
    }

    /// Removes cache control from this document content block.
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::document::AnthropicDocumentContent;
    /// use llm_kit_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    ///
    /// let document = AnthropicDocumentContent::from_url("https://example.com/doc.pdf")
    ///     .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour))
    ///     .without_cache_control();
    ///
    /// assert!(document.cache_control.is_none());
    /// ```
    pub fn without_cache_control(mut self) -> Self {
        self.cache_control = None;
        self
    }

    /// Updates the source of this document content block.
    ///
    /// # Arguments
    ///
    /// * `source` - The new document source
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::document::AnthropicDocumentContent;
    /// use llm_kit_anthropic::prompt::message::content::source_type::AnthropicContentSource;
    ///
    /// let document = AnthropicDocumentContent::from_url("https://example.com/old.pdf")
    ///     .with_source(AnthropicContentSource::url("https://example.com/new.pdf"));
    /// ```
    pub fn with_source(mut self, source: AnthropicContentSource) -> Self {
        self.source = source;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    use serde_json::json;

    #[test]
    fn test_citations_new() {
        let citations = DocumentCitations::new(true);
        assert!(citations.enabled);

        let citations = DocumentCitations::new(false);
        assert!(!citations.enabled);
    }

    #[test]
    fn test_citations_enabled() {
        let citations = DocumentCitations::enabled();
        assert!(citations.enabled);
    }

    #[test]
    fn test_citations_disabled() {
        let citations = DocumentCitations::disabled();
        assert!(!citations.enabled);
    }

    #[test]
    fn test_new() {
        let source = AnthropicContentSource::url("https://example.com/doc.pdf");
        let document = AnthropicDocumentContent::new(source);

        assert_eq!(document.content_type, "document");
        assert!(document.title.is_none());
        assert!(document.context.is_none());
        assert!(document.citations.is_none());
        assert!(document.cache_control.is_none());
    }

    #[test]
    fn test_from_url() {
        let document = AnthropicDocumentContent::from_url("https://example.com/file.pdf");

        assert_eq!(document.content_type, "document");
        match document.source {
            AnthropicContentSource::Url { url } => {
                assert_eq!(url, "https://example.com/file.pdf");
            }
            _ => panic!("Expected Url source"),
        }
    }

    #[test]
    fn test_from_base64() {
        let document = AnthropicDocumentContent::from_base64("application/pdf", "pdfbase64data");

        match document.source {
            AnthropicContentSource::Base64 { media_type, data } => {
                assert_eq!(media_type, "application/pdf");
                assert_eq!(data, "pdfbase64data");
            }
            _ => panic!("Expected Base64 source"),
        }
    }

    #[test]
    fn test_from_text() {
        let document = AnthropicDocumentContent::from_text("Plain text document content");

        match document.source {
            AnthropicContentSource::Text { media_type, data } => {
                assert_eq!(media_type, "text/plain");
                assert_eq!(data, "Plain text document content");
            }
            _ => panic!("Expected Text source"),
        }
    }

    #[test]
    fn test_with_title() {
        let document = AnthropicDocumentContent::from_url("https://example.com/doc.pdf")
            .with_title("My Document");

        assert_eq!(document.title, Some("My Document".to_string()));
    }

    #[test]
    fn test_with_context() {
        let document = AnthropicDocumentContent::from_url("https://example.com/doc.pdf")
            .with_context("This is the context");

        assert_eq!(document.context, Some("This is the context".to_string()));
    }

    #[test]
    fn test_with_citations() {
        let document = AnthropicDocumentContent::from_url("https://example.com/doc.pdf")
            .with_citations(DocumentCitations::enabled());

        assert!(document.citations.is_some());
        assert!(document.citations.unwrap().enabled);
    }

    #[test]
    fn test_with_citations_enabled() {
        let document = AnthropicDocumentContent::from_url("https://example.com/doc.pdf")
            .with_citations_enabled();

        assert_eq!(document.citations, Some(DocumentCitations::enabled()));
    }

    #[test]
    fn test_with_citations_disabled() {
        let document = AnthropicDocumentContent::from_url("https://example.com/doc.pdf")
            .with_citations_disabled();

        assert_eq!(document.citations, Some(DocumentCitations::disabled()));
    }

    #[test]
    fn test_with_cache_control() {
        let cache_control = AnthropicCacheControl::new(CacheTTL::FiveMinutes);
        let document = AnthropicDocumentContent::from_url("https://example.com/doc.pdf")
            .with_cache_control(cache_control.clone());

        assert_eq!(document.cache_control, Some(cache_control));
    }

    #[test]
    fn test_without_cache_control() {
        let document = AnthropicDocumentContent::from_url("https://example.com/doc.pdf")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour))
            .without_cache_control();

        assert!(document.cache_control.is_none());
    }

    #[test]
    fn test_with_source() {
        let document = AnthropicDocumentContent::from_url("https://example.com/old.pdf")
            .with_source(AnthropicContentSource::url("https://example.com/new.pdf"));

        match document.source {
            AnthropicContentSource::Url { url } => {
                assert_eq!(url, "https://example.com/new.pdf");
            }
            _ => panic!("Expected Url source"),
        }
    }

    #[test]
    fn test_serialize_minimal() {
        let document = AnthropicDocumentContent::from_url("https://example.com/doc.pdf");
        let json = serde_json::to_value(&document).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "document",
                "source": {
                    "type": "url",
                    "url": "https://example.com/doc.pdf"
                }
            })
        );
    }

    #[test]
    fn test_serialize_with_all_fields() {
        let document = AnthropicDocumentContent::from_url("https://example.com/doc.pdf")
            .with_title("Research Paper")
            .with_context("A paper about AI")
            .with_citations(DocumentCitations::enabled())
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&document).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "document",
                "source": {
                    "type": "url",
                    "url": "https://example.com/doc.pdf"
                },
                "title": "Research Paper",
                "context": "A paper about AI",
                "citations": {
                    "enabled": true
                },
                "cache_control": {
                    "type": "ephemeral",
                    "ttl": "1h"
                }
            })
        );
    }

    #[test]
    fn test_deserialize_minimal() {
        let json = json!({
            "type": "document",
            "source": {
                "type": "url",
                "url": "https://example.com/doc.pdf"
            }
        });

        let document: AnthropicDocumentContent = serde_json::from_value(json).unwrap();

        assert_eq!(document.content_type, "document");
        assert!(document.title.is_none());
        assert!(document.context.is_none());
        assert!(document.citations.is_none());
        assert!(document.cache_control.is_none());
    }

    #[test]
    fn test_deserialize_with_all_fields() {
        let json = json!({
            "type": "document",
            "source": {
                "type": "base64",
                "media_type": "application/pdf",
                "data": "base64data"
            },
            "title": "Important Doc",
            "context": "Context here",
            "citations": {
                "enabled": false
            },
            "cache_control": {
                "type": "ephemeral",
                "ttl": "5m"
            }
        });

        let document: AnthropicDocumentContent = serde_json::from_value(json).unwrap();

        assert_eq!(document.title, Some("Important Doc".to_string()));
        assert_eq!(document.context, Some("Context here".to_string()));
        assert_eq!(document.citations, Some(DocumentCitations::disabled()));
        assert_eq!(
            document.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
        );
    }

    #[test]
    fn test_equality() {
        let doc1 = AnthropicDocumentContent::from_url("https://example.com/same.pdf");
        let doc2 = AnthropicDocumentContent::from_url("https://example.com/same.pdf");
        let doc3 = AnthropicDocumentContent::from_url("https://example.com/different.pdf");

        assert_eq!(doc1, doc2);
        assert_ne!(doc1, doc3);
    }

    #[test]
    fn test_clone() {
        let doc1 = AnthropicDocumentContent::from_url("https://example.com/doc.pdf")
            .with_title("Title")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let doc2 = doc1.clone();

        assert_eq!(doc1, doc2);
    }

    #[test]
    fn test_builder_chaining() {
        let document = AnthropicDocumentContent::from_url("https://example.com/1.pdf")
            .with_title("Title 1")
            .with_context("Context 1")
            .with_citations_enabled()
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
            .with_title("Title 2")
            .with_citations_disabled()
            .without_cache_control();

        assert_eq!(document.title, Some("Title 2".to_string()));
        assert_eq!(document.context, Some("Context 1".to_string()));
        assert_eq!(document.citations, Some(DocumentCitations::disabled()));
        assert!(document.cache_control.is_none());
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original = AnthropicDocumentContent::from_base64("application/pdf", "data")
            .with_title("Test Doc")
            .with_context("Test context")
            .with_citations_enabled()
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicDocumentContent = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }
}
