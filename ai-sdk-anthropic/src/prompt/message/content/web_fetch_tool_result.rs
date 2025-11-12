use serde::{Deserialize, Serialize};

use crate::prompt::message::cache_control::AnthropicCacheControl;
use crate::prompt::message::content::document::DocumentCitations;
use crate::prompt::message::content::source_type::AnthropicContentSource;

/// Web fetch result document content.
///
/// Represents the document content within a web fetch result, including
/// the document source, title, and optional citations configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebFetchResultDocument {
    /// The type of content (always "document")
    #[serde(rename = "type")]
    pub content_type: String,

    /// Optional title for the document
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Optional citations configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citations: Option<DocumentCitations>,

    /// The document source (base64 or text)
    pub source: AnthropicContentSource,
}

impl WebFetchResultDocument {
    /// Creates a new web fetch result document.
    ///
    /// # Arguments
    ///
    /// * `source` - The document source (base64 or text)
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::web_fetch_tool_result::WebFetchResultDocument;
    /// use ai_sdk_anthropic::prompt::message::content::source_type::AnthropicContentSource;
    ///
    /// let document = WebFetchResultDocument::new(
    ///     AnthropicContentSource::text("Fetched content")
    /// );
    /// ```
    pub fn new(source: AnthropicContentSource) -> Self {
        Self {
            content_type: "document".to_string(),
            title: None,
            citations: None,
            source,
        }
    }

    /// Creates a new web fetch result document from base64-encoded PDF data.
    ///
    /// # Arguments
    ///
    /// * `data` - The base64-encoded PDF data
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::web_fetch_tool_result::WebFetchResultDocument;
    ///
    /// let document = WebFetchResultDocument::from_pdf_base64("base64data...");
    /// ```
    pub fn from_pdf_base64(data: impl Into<String>) -> Self {
        Self::new(AnthropicContentSource::base64("application/pdf", data))
    }

    /// Creates a new web fetch result document from plain text.
    ///
    /// # Arguments
    ///
    /// * `text` - The plain text content
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::web_fetch_tool_result::WebFetchResultDocument;
    ///
    /// let document = WebFetchResultDocument::from_text("Fetched page content");
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
    /// use ai_sdk_anthropic::prompt::message::content::web_fetch_tool_result::WebFetchResultDocument;
    ///
    /// let document = WebFetchResultDocument::from_text("content")
    ///     .with_title("Web Page Title");
    /// ```
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
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
    /// use ai_sdk_anthropic::prompt::message::content::web_fetch_tool_result::WebFetchResultDocument;
    /// use ai_sdk_anthropic::prompt::message::content::document::DocumentCitations;
    ///
    /// let document = WebFetchResultDocument::from_text("content")
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
    /// use ai_sdk_anthropic::prompt::message::content::web_fetch_tool_result::WebFetchResultDocument;
    ///
    /// let document = WebFetchResultDocument::from_text("content")
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
    /// use ai_sdk_anthropic::prompt::message::content::web_fetch_tool_result::WebFetchResultDocument;
    ///
    /// let document = WebFetchResultDocument::from_text("content")
    ///     .with_citations_disabled();
    /// ```
    pub fn with_citations_disabled(self) -> Self {
        self.with_citations(DocumentCitations::disabled())
    }

    /// Updates the source of this document.
    ///
    /// # Arguments
    ///
    /// * `source` - The new document source
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::web_fetch_tool_result::WebFetchResultDocument;
    /// use ai_sdk_anthropic::prompt::message::content::source_type::AnthropicContentSource;
    ///
    /// let document = WebFetchResultDocument::from_text("old content")
    ///     .with_source(AnthropicContentSource::text("new content"));
    /// ```
    pub fn with_source(mut self, source: AnthropicContentSource) -> Self {
        self.source = source;
        self
    }
}

/// Web fetch result content.
///
/// Represents the result of a web fetch operation, including the fetched URL,
/// retrieval timestamp, and document content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebFetchResultContent {
    /// The type of content (always "web_fetch_result")
    #[serde(rename = "type")]
    pub content_type: String,

    /// The URL that was fetched
    pub url: String,

    /// The timestamp when the content was retrieved (ISO 8601 format), or null if unknown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<String>,

    /// The fetched document content
    pub content: WebFetchResultDocument,
}

impl WebFetchResultContent {
    /// Creates a new web fetch result content.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL that was fetched
    /// * `content` - The fetched document content
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::web_fetch_tool_result::{
    ///     WebFetchResultContent, WebFetchResultDocument
    /// };
    ///
    /// let document = WebFetchResultDocument::from_text("Fetched content");
    /// let result = WebFetchResultContent::new("https://example.com", document);
    /// ```
    pub fn new(url: impl Into<String>, content: WebFetchResultDocument) -> Self {
        Self {
            content_type: "web_fetch_result".to_string(),
            url: url.into(),
            retrieved_at: None,
            content,
        }
    }

    /// Sets the retrieved_at timestamp for this web fetch result.
    ///
    /// # Arguments
    ///
    /// * `retrieved_at` - ISO 8601 formatted timestamp
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::web_fetch_tool_result::{
    ///     WebFetchResultContent, WebFetchResultDocument
    /// };
    ///
    /// let document = WebFetchResultDocument::from_text("content");
    /// let result = WebFetchResultContent::new("https://example.com", document)
    ///     .with_retrieved_at("2024-01-15T12:00:00Z");
    /// ```
    pub fn with_retrieved_at(mut self, retrieved_at: impl Into<String>) -> Self {
        self.retrieved_at = Some(retrieved_at.into());
        self
    }

    /// Removes the retrieved_at timestamp.
    pub fn without_retrieved_at(mut self) -> Self {
        self.retrieved_at = None;
        self
    }

    /// Updates the URL.
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    /// Updates the document content.
    pub fn with_content(mut self, content: WebFetchResultDocument) -> Self {
        self.content = content;
        self
    }
}

/// Anthropic web fetch tool result content block.
///
/// Represents the result of a web fetch tool execution. Contains the tool use ID,
/// the web fetch result with URL, timestamp, and document content, and optional
/// cache control configuration.
///
/// # Example
///
/// ```
/// use ai_sdk_anthropic::prompt::message::content::web_fetch_tool_result::{
///     AnthropicWebFetchToolResultContent, WebFetchResultContent, WebFetchResultDocument
/// };
///
/// let document = WebFetchResultDocument::from_text("Fetched webpage content")
///     .with_title("Example Page");
///
/// let result = WebFetchResultContent::new("https://example.com", document)
///     .with_retrieved_at("2024-01-15T12:00:00Z");
///
/// let web_fetch = AnthropicWebFetchToolResultContent::new("server_toolu_123", result);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicWebFetchToolResultContent {
    /// The type of content (always "web_fetch_tool_result")
    #[serde(rename = "type")]
    pub content_type: String,

    /// ID of the tool use this is a result for
    pub tool_use_id: String,

    /// The web fetch result
    pub content: WebFetchResultContent,

    /// Optional cache control configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}

impl AnthropicWebFetchToolResultContent {
    /// Creates a new web fetch tool result content block.
    ///
    /// # Arguments
    ///
    /// * `tool_use_id` - ID of the tool use this is a result for
    /// * `content` - The web fetch result
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::web_fetch_tool_result::{
    ///     AnthropicWebFetchToolResultContent, WebFetchResultContent, WebFetchResultDocument
    /// };
    ///
    /// let document = WebFetchResultDocument::from_text("content");
    /// let result = WebFetchResultContent::new("https://example.com", document);
    /// let web_fetch = AnthropicWebFetchToolResultContent::new("toolu_123", result);
    /// ```
    pub fn new(tool_use_id: impl Into<String>, content: WebFetchResultContent) -> Self {
        Self {
            content_type: "web_fetch_tool_result".to_string(),
            tool_use_id: tool_use_id.into(),
            content,
            cache_control: None,
        }
    }

    /// Sets the cache control for this web fetch tool result.
    ///
    /// # Arguments
    ///
    /// * `cache_control` - The cache control configuration
    ///
    /// # Example
    ///
    /// ```
    /// use ai_sdk_anthropic::prompt::message::content::web_fetch_tool_result::{
    ///     AnthropicWebFetchToolResultContent, WebFetchResultContent, WebFetchResultDocument
    /// };
    /// use ai_sdk_anthropic::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    ///
    /// let document = WebFetchResultDocument::from_text("content");
    /// let result = WebFetchResultContent::new("https://example.com", document);
    /// let web_fetch = AnthropicWebFetchToolResultContent::new("toolu_123", result)
    ///     .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
    /// ```
    pub fn with_cache_control(mut self, cache_control: AnthropicCacheControl) -> Self {
        self.cache_control = Some(cache_control);
        self
    }

    /// Removes cache control from this web fetch tool result.
    pub fn without_cache_control(mut self) -> Self {
        self.cache_control = None;
        self
    }

    /// Updates the tool use ID.
    pub fn with_tool_use_id(mut self, tool_use_id: impl Into<String>) -> Self {
        self.tool_use_id = tool_use_id.into();
        self
    }

    /// Updates the web fetch result content.
    pub fn with_content(mut self, content: WebFetchResultContent) -> Self {
        self.content = content;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    use serde_json::json;

    // WebFetchResultDocument tests
    #[test]
    fn test_document_new() {
        let source = AnthropicContentSource::text("content");
        let document = WebFetchResultDocument::new(source);

        assert_eq!(document.content_type, "document");
        assert!(document.title.is_none());
        assert!(document.citations.is_none());
    }

    #[test]
    fn test_document_from_pdf_base64() {
        let document = WebFetchResultDocument::from_pdf_base64("pdfbase64data");

        match document.source {
            AnthropicContentSource::Base64 { media_type, data } => {
                assert_eq!(media_type, "application/pdf");
                assert_eq!(data, "pdfbase64data");
            }
            _ => panic!("Expected Base64 source"),
        }
    }

    #[test]
    fn test_document_from_text() {
        let document = WebFetchResultDocument::from_text("Plain text content");

        match document.source {
            AnthropicContentSource::Text { media_type, data } => {
                assert_eq!(media_type, "text/plain");
                assert_eq!(data, "Plain text content");
            }
            _ => panic!("Expected Text source"),
        }
    }

    #[test]
    fn test_document_with_title() {
        let document = WebFetchResultDocument::from_text("content").with_title("My Title");

        assert_eq!(document.title, Some("My Title".to_string()));
    }

    #[test]
    fn test_document_with_citations() {
        let document = WebFetchResultDocument::from_text("content")
            .with_citations(DocumentCitations::enabled());

        assert_eq!(document.citations, Some(DocumentCitations::enabled()));
    }

    #[test]
    fn test_document_with_citations_enabled() {
        let document = WebFetchResultDocument::from_text("content").with_citations_enabled();

        assert_eq!(document.citations, Some(DocumentCitations::enabled()));
    }

    #[test]
    fn test_document_with_citations_disabled() {
        let document = WebFetchResultDocument::from_text("content").with_citations_disabled();

        assert_eq!(document.citations, Some(DocumentCitations::disabled()));
    }

    #[test]
    fn test_document_with_source() {
        let document = WebFetchResultDocument::from_text("old")
            .with_source(AnthropicContentSource::text("new"));

        match document.source {
            AnthropicContentSource::Text { data, .. } => {
                assert_eq!(data, "new");
            }
            _ => panic!("Expected Text source"),
        }
    }

    #[test]
    fn test_document_serialize() {
        let document = WebFetchResultDocument::from_text("content")
            .with_title("Title")
            .with_citations_enabled();

        let json = serde_json::to_value(&document).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "document",
                "title": "Title",
                "citations": {
                    "enabled": true
                },
                "source": {
                    "type": "text",
                    "media_type": "text/plain",
                    "data": "content"
                }
            })
        );
    }

    #[test]
    fn test_document_deserialize() {
        let json = json!({
            "type": "document",
            "title": "My Document",
            "citations": {
                "enabled": false
            },
            "source": {
                "type": "base64",
                "media_type": "application/pdf",
                "data": "base64data"
            }
        });

        let document: WebFetchResultDocument = serde_json::from_value(json).unwrap();

        assert_eq!(document.title, Some("My Document".to_string()));
        assert_eq!(document.citations, Some(DocumentCitations::disabled()));
    }

    // WebFetchResultContent tests
    #[test]
    fn test_result_content_new() {
        let document = WebFetchResultDocument::from_text("content");
        let result = WebFetchResultContent::new("https://example.com", document);

        assert_eq!(result.content_type, "web_fetch_result");
        assert_eq!(result.url, "https://example.com");
        assert!(result.retrieved_at.is_none());
    }

    #[test]
    fn test_result_content_with_retrieved_at() {
        let document = WebFetchResultDocument::from_text("content");
        let result = WebFetchResultContent::new("https://example.com", document)
            .with_retrieved_at("2024-01-15T12:00:00Z");

        assert_eq!(
            result.retrieved_at,
            Some("2024-01-15T12:00:00Z".to_string())
        );
    }

    #[test]
    fn test_result_content_without_retrieved_at() {
        let document = WebFetchResultDocument::from_text("content");
        let result = WebFetchResultContent::new("https://example.com", document)
            .with_retrieved_at("2024-01-15T12:00:00Z")
            .without_retrieved_at();

        assert!(result.retrieved_at.is_none());
    }

    #[test]
    fn test_result_content_with_url() {
        let document = WebFetchResultDocument::from_text("content");
        let result =
            WebFetchResultContent::new("https://old.com", document).with_url("https://new.com");

        assert_eq!(result.url, "https://new.com");
    }

    #[test]
    fn test_result_content_with_content() {
        let document1 = WebFetchResultDocument::from_text("content1");
        let document2 = WebFetchResultDocument::from_text("content2");
        let result = WebFetchResultContent::new("https://example.com", document1)
            .with_content(document2.clone());

        assert_eq!(result.content, document2);
    }

    #[test]
    fn test_result_content_serialize() {
        let document =
            WebFetchResultDocument::from_text("Fetched content").with_title("Page Title");
        let result = WebFetchResultContent::new("https://example.com", document)
            .with_retrieved_at("2024-01-15T12:00:00Z");

        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "web_fetch_result",
                "url": "https://example.com",
                "retrieved_at": "2024-01-15T12:00:00Z",
                "content": {
                    "type": "document",
                    "title": "Page Title",
                    "source": {
                        "type": "text",
                        "media_type": "text/plain",
                        "data": "Fetched content"
                    }
                }
            })
        );
    }

    #[test]
    fn test_result_content_deserialize() {
        let json = json!({
            "type": "web_fetch_result",
            "url": "https://example.com",
            "retrieved_at": "2024-01-15T12:00:00Z",
            "content": {
                "type": "document",
                "title": "Title",
                "source": {
                    "type": "text",
                    "media_type": "text/plain",
                    "data": "content"
                }
            }
        });

        let result: WebFetchResultContent = serde_json::from_value(json).unwrap();

        assert_eq!(result.url, "https://example.com");
        assert_eq!(
            result.retrieved_at,
            Some("2024-01-15T12:00:00Z".to_string())
        );
        assert_eq!(result.content.title, Some("Title".to_string()));
    }

    // AnthropicWebFetchToolResultContent tests
    #[test]
    fn test_web_fetch_tool_result_new() {
        let document = WebFetchResultDocument::from_text("content");
        let result = WebFetchResultContent::new("https://example.com", document);
        let web_fetch = AnthropicWebFetchToolResultContent::new("toolu_123", result);

        assert_eq!(web_fetch.content_type, "web_fetch_tool_result");
        assert_eq!(web_fetch.tool_use_id, "toolu_123");
        assert!(web_fetch.cache_control.is_none());
    }

    #[test]
    fn test_web_fetch_tool_result_with_cache_control() {
        let document = WebFetchResultDocument::from_text("content");
        let result = WebFetchResultContent::new("https://example.com", document);
        let cache = AnthropicCacheControl::new(CacheTTL::OneHour);
        let web_fetch = AnthropicWebFetchToolResultContent::new("toolu_123", result)
            .with_cache_control(cache.clone());

        assert_eq!(web_fetch.cache_control, Some(cache));
    }

    #[test]
    fn test_web_fetch_tool_result_without_cache_control() {
        let document = WebFetchResultDocument::from_text("content");
        let result = WebFetchResultContent::new("https://example.com", document);
        let web_fetch = AnthropicWebFetchToolResultContent::new("toolu_123", result)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
            .without_cache_control();

        assert!(web_fetch.cache_control.is_none());
    }

    #[test]
    fn test_web_fetch_tool_result_with_tool_use_id() {
        let document = WebFetchResultDocument::from_text("content");
        let result = WebFetchResultContent::new("https://example.com", document);
        let web_fetch =
            AnthropicWebFetchToolResultContent::new("old_id", result).with_tool_use_id("new_id");

        assert_eq!(web_fetch.tool_use_id, "new_id");
    }

    #[test]
    fn test_web_fetch_tool_result_with_content() {
        let document1 = WebFetchResultDocument::from_text("content1");
        let document2 = WebFetchResultDocument::from_text("content2");
        let result1 = WebFetchResultContent::new("https://example1.com", document1);
        let result2 = WebFetchResultContent::new("https://example2.com", document2);

        let web_fetch = AnthropicWebFetchToolResultContent::new("toolu_123", result1)
            .with_content(result2.clone());

        assert_eq!(web_fetch.content, result2);
    }

    #[test]
    fn test_web_fetch_tool_result_serialize_minimal() {
        let document = WebFetchResultDocument::from_text("Fetched content");
        let result = WebFetchResultContent::new("https://example.com", document);
        let web_fetch = AnthropicWebFetchToolResultContent::new("toolu_123", result);

        let json = serde_json::to_value(&web_fetch).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "web_fetch_tool_result",
                "tool_use_id": "toolu_123",
                "content": {
                    "type": "web_fetch_result",
                    "url": "https://example.com",
                    "content": {
                        "type": "document",
                        "source": {
                            "type": "text",
                            "media_type": "text/plain",
                            "data": "Fetched content"
                        }
                    }
                }
            })
        );
    }

    #[test]
    fn test_web_fetch_tool_result_serialize_full() {
        let document = WebFetchResultDocument::from_text("Fetched content")
            .with_title("Page Title")
            .with_citations_enabled();
        let result = WebFetchResultContent::new("https://example.com", document)
            .with_retrieved_at("2024-01-15T12:00:00Z");
        let web_fetch = AnthropicWebFetchToolResultContent::new("toolu_123", result)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&web_fetch).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "web_fetch_tool_result",
                "tool_use_id": "toolu_123",
                "content": {
                    "type": "web_fetch_result",
                    "url": "https://example.com",
                    "retrieved_at": "2024-01-15T12:00:00Z",
                    "content": {
                        "type": "document",
                        "title": "Page Title",
                        "citations": {
                            "enabled": true
                        },
                        "source": {
                            "type": "text",
                            "media_type": "text/plain",
                            "data": "Fetched content"
                        }
                    }
                },
                "cache_control": {
                    "type": "ephemeral",
                    "ttl": "1h"
                }
            })
        );
    }

    #[test]
    fn test_web_fetch_tool_result_serialize_with_pdf() {
        let document =
            WebFetchResultDocument::from_pdf_base64("base64pdfdata").with_title("PDF Document");
        let result = WebFetchResultContent::new("https://example.com/doc.pdf", document);
        let web_fetch = AnthropicWebFetchToolResultContent::new("toolu_456", result);

        let json = serde_json::to_value(&web_fetch).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "web_fetch_tool_result",
                "tool_use_id": "toolu_456",
                "content": {
                    "type": "web_fetch_result",
                    "url": "https://example.com/doc.pdf",
                    "content": {
                        "type": "document",
                        "title": "PDF Document",
                        "source": {
                            "type": "base64",
                            "media_type": "application/pdf",
                            "data": "base64pdfdata"
                        }
                    }
                }
            })
        );
    }

    #[test]
    fn test_web_fetch_tool_result_deserialize() {
        let json = json!({
            "type": "web_fetch_tool_result",
            "tool_use_id": "toolu_789",
            "content": {
                "type": "web_fetch_result",
                "url": "https://example.com",
                "retrieved_at": "2024-01-15T12:00:00Z",
                "content": {
                    "type": "document",
                    "title": "Document Title",
                    "source": {
                        "type": "text",
                        "media_type": "text/plain",
                        "data": "content"
                    }
                }
            },
            "cache_control": {
                "type": "ephemeral",
                "ttl": "5m"
            }
        });

        let web_fetch: AnthropicWebFetchToolResultContent = serde_json::from_value(json).unwrap();

        assert_eq!(web_fetch.tool_use_id, "toolu_789");
        assert_eq!(web_fetch.content.url, "https://example.com");
        assert_eq!(
            web_fetch.content.retrieved_at,
            Some("2024-01-15T12:00:00Z".to_string())
        );
        assert_eq!(
            web_fetch.content.content.title,
            Some("Document Title".to_string())
        );
        assert_eq!(
            web_fetch.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
        );
    }

    #[test]
    fn test_equality() {
        let document1 = WebFetchResultDocument::from_text("content");
        let document2 = WebFetchResultDocument::from_text("content");
        let result1 = WebFetchResultContent::new("https://example.com", document1);
        let result2 = WebFetchResultContent::new("https://example.com", document2);

        let wf1 = AnthropicWebFetchToolResultContent::new("id", result1);
        let wf2 = AnthropicWebFetchToolResultContent::new("id", result2);

        assert_eq!(wf1, wf2);
    }

    #[test]
    fn test_clone() {
        let document = WebFetchResultDocument::from_text("content");
        let result = WebFetchResultContent::new("https://example.com", document);
        let wf1 = AnthropicWebFetchToolResultContent::new("id", result)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let wf2 = wf1.clone();

        assert_eq!(wf1, wf2);
    }

    #[test]
    fn test_builder_chaining() {
        let document = WebFetchResultDocument::from_text("content1")
            .with_title("Title1")
            .with_citations_enabled()
            .with_title("Title2")
            .with_citations_disabled();

        let result = WebFetchResultContent::new("https://old.com", document)
            .with_url("https://new.com")
            .with_retrieved_at("2024-01-15T12:00:00Z")
            .without_retrieved_at()
            .with_retrieved_at("2024-01-16T12:00:00Z");

        let web_fetch = AnthropicWebFetchToolResultContent::new("id1", result)
            .with_tool_use_id("id2")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
            .without_cache_control()
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        assert_eq!(web_fetch.tool_use_id, "id2");
        assert_eq!(web_fetch.content.url, "https://new.com");
        assert_eq!(
            web_fetch.content.retrieved_at,
            Some("2024-01-16T12:00:00Z".to_string())
        );
        assert_eq!(web_fetch.content.content.title, Some("Title2".to_string()));
        assert_eq!(
            web_fetch.content.content.citations,
            Some(DocumentCitations::disabled())
        );
        assert_eq!(
            web_fetch.cache_control,
            Some(AnthropicCacheControl::new(CacheTTL::OneHour))
        );
    }

    #[test]
    fn test_roundtrip_serialization() {
        let document = WebFetchResultDocument::from_pdf_base64("pdfdata")
            .with_title("Test PDF")
            .with_citations_enabled();
        let result = WebFetchResultContent::new("https://example.com/doc.pdf", document)
            .with_retrieved_at("2024-01-15T12:00:00Z");
        let original = AnthropicWebFetchToolResultContent::new("toolu_test", result)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicWebFetchToolResultContent =
            serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }
}
