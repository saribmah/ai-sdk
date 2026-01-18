use serde::{Deserialize, Serialize};

use crate::prompt::message::cache_control::AnthropicCacheControl;
use crate::prompt::message::content::source_type::AnthropicContentSource;

/// Nested text content for tool results (without cache_control).
///
/// Sub-content blocks cannot be cached directly according to Anthropic docs.
/// This is identical to `AnthropicTextContent` but without the `cache_control` field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicNestedTextContent {
    /// The type of content (always "text")
    #[serde(rename = "type")]
    pub content_type: String,

    /// The text content
    pub text: String,
}

impl AnthropicNestedTextContent {
    /// Creates a new nested text content block.
    ///
    /// # Arguments
    ///
    /// * `text` - The text content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::tool_result::AnthropicNestedTextContent;
    ///
    /// let text = AnthropicNestedTextContent::new("Tool result text");
    /// ```
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            content_type: "text".to_string(),
            text: text.into(),
        }
    }
}

impl From<String> for AnthropicNestedTextContent {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

impl From<&str> for AnthropicNestedTextContent {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

/// Nested image content for tool results (without cache_control).
///
/// Sub-content blocks cannot be cached directly according to Anthropic docs.
/// This is identical to `AnthropicImageContent` but without the `cache_control` field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicNestedImageContent {
    /// The type of content (always "image")
    #[serde(rename = "type")]
    pub content_type: String,

    /// The image source (base64, URL, or text)
    pub source: AnthropicContentSource,
}

impl AnthropicNestedImageContent {
    /// Creates a new nested image content block.
    ///
    /// # Arguments
    ///
    /// * `source` - The image source
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::tool_result::AnthropicNestedImageContent;
    /// use llm_kit_anthropic::prompt::message::content::source_type::AnthropicContentSource;
    ///
    /// let image = AnthropicNestedImageContent::new(
    ///     AnthropicContentSource::url("https://example.com/image.png")
    /// );
    /// ```
    pub fn new(source: AnthropicContentSource) -> Self {
        Self {
            content_type: "image".to_string(),
            source,
        }
    }

    /// Creates a new nested image from a URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The image URL
    pub fn from_url(url: impl Into<String>) -> Self {
        Self::new(AnthropicContentSource::url(url))
    }

    /// Creates a new nested image from base64 data.
    ///
    /// # Arguments
    ///
    /// * `media_type` - The MIME type
    /// * `data` - The base64-encoded data
    pub fn from_base64(media_type: impl Into<String>, data: impl Into<String>) -> Self {
        Self::new(AnthropicContentSource::base64(media_type, data))
    }
}

/// Nested document content for tool results (without cache_control).
///
/// Sub-content blocks cannot be cached directly according to Anthropic docs.
/// This is identical to `AnthropicDocumentContent` but without the `cache_control` field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicNestedDocumentContent {
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
}

/// Citations configuration for nested document content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentCitations {
    /// Whether citations are enabled
    pub enabled: bool,
}

impl DocumentCitations {
    /// Creates a new citations configuration.
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    /// Creates citations configuration with citations enabled.
    pub fn enabled() -> Self {
        Self { enabled: true }
    }

    /// Creates citations configuration with citations disabled.
    pub fn disabled() -> Self {
        Self { enabled: false }
    }
}

impl AnthropicNestedDocumentContent {
    /// Creates a new nested document content block.
    ///
    /// # Arguments
    ///
    /// * `source` - The document source
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::tool_result::AnthropicNestedDocumentContent;
    /// use llm_kit_anthropic::prompt::message::content::source_type::AnthropicContentSource;
    ///
    /// let document = AnthropicNestedDocumentContent::new(
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
        }
    }

    /// Creates a new nested document from a URL.
    pub fn from_url(url: impl Into<String>) -> Self {
        Self::new(AnthropicContentSource::url(url))
    }

    /// Creates a new nested document from base64 data.
    pub fn from_base64(media_type: impl Into<String>, data: impl Into<String>) -> Self {
        Self::new(AnthropicContentSource::base64(media_type, data))
    }

    /// Creates a new nested document from plain text.
    pub fn from_text(text: impl Into<String>) -> Self {
        Self::new(AnthropicContentSource::text(text))
    }

    /// Sets the title for this document.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the context for this document.
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Sets the citations configuration for this document.
    pub fn with_citations(mut self, citations: DocumentCitations) -> Self {
        self.citations = Some(citations);
        self
    }

    /// Enables citations for this document.
    pub fn with_citations_enabled(self) -> Self {
        self.with_citations(DocumentCitations::enabled())
    }

    /// Disables citations for this document.
    pub fn with_citations_disabled(self) -> Self {
        self.with_citations(DocumentCitations::disabled())
    }
}

/// Nested content type for tool results.
///
/// Can be text, image, or document, but never includes cache_control.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AnthropicNestedContent {
    /// Nested text content
    #[serde(rename = "text")]
    Text {
        /// The text content
        text: String,
    },

    /// Nested image content
    #[serde(rename = "image")]
    Image {
        /// The image source
        source: AnthropicContentSource,
    },

    /// Nested document content
    #[serde(rename = "document")]
    Document {
        /// The document source
        source: AnthropicContentSource,
        /// Optional title for the document
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        /// Optional context about the document
        #[serde(skip_serializing_if = "Option::is_none")]
        context: Option<String>,
        /// Optional citations configuration
        #[serde(skip_serializing_if = "Option::is_none")]
        citations: Option<DocumentCitations>,
    },
}

/// Tool result content type for tool results.
///
/// Can be either a simple string or an array of nested content blocks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolResultContentType {
    /// Simple string content
    String(String),

    /// Array of nested content blocks
    Array(Vec<AnthropicNestedContent>),
}

/// Anthropic tool result content block.
///
/// Represents the result of a tool execution. The content can be either a simple
/// string or an array of nested content blocks (text, image, or document).
///
/// # Example
///
/// ```
/// use llm_kit_anthropic::prompt::message::content::tool_result::{
///     AnthropicToolResultContent, ToolResultContentType
/// };
///
/// // Simple string result
/// let result = AnthropicToolResultContent::new(
///     "toolu_123",
///     ToolResultContentType::String("The weather is sunny".to_string())
/// );
///
/// // With error flag
/// let error_result = AnthropicToolResultContent::new(
///     "toolu_456",
///     ToolResultContentType::String("Tool execution failed".to_string())
/// ).with_error(true);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicToolResultContent {
    /// The type of content (always "tool_result")
    #[serde(rename = "type")]
    pub content_type: String,

    /// ID of the tool use this is a result for
    pub tool_use_id: String,

    /// The result content (string or array of nested content)
    pub content: ToolResultContentType,

    /// Whether this result represents an error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,

    /// Optional cache control configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<AnthropicCacheControl>,
}

impl AnthropicToolResultContent {
    /// Creates a new tool result content block.
    ///
    /// # Arguments
    ///
    /// * `tool_use_id` - ID of the tool use this is a result for
    /// * `content` - The result content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::content::tool_result::{
    ///     AnthropicToolResultContent, ToolResultContentType
    /// };
    ///
    /// let result = AnthropicToolResultContent::new(
    ///     "toolu_abc123",
    ///     ToolResultContentType::String("Success".to_string())
    /// );
    /// ```
    pub fn new(tool_use_id: impl Into<String>, content: ToolResultContentType) -> Self {
        Self {
            content_type: "tool_result".to_string(),
            tool_use_id: tool_use_id.into(),
            content,
            is_error: None,
            cache_control: None,
        }
    }

    /// Creates a new tool result with string content.
    ///
    /// # Arguments
    ///
    /// * `tool_use_id` - ID of the tool use this is a result for
    /// * `content` - The result content as a string
    pub fn from_string(tool_use_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self::new(tool_use_id, ToolResultContentType::String(content.into()))
    }

    /// Creates a new tool result with array content.
    ///
    /// # Arguments
    ///
    /// * `tool_use_id` - ID of the tool use this is a result for
    /// * `content` - The result content as an array of nested content
    pub fn from_array(
        tool_use_id: impl Into<String>,
        content: Vec<AnthropicNestedContent>,
    ) -> Self {
        Self::new(tool_use_id, ToolResultContentType::Array(content))
    }

    /// Sets whether this result represents an error.
    ///
    /// # Arguments
    ///
    /// * `is_error` - Whether this is an error
    pub fn with_error(mut self, is_error: bool) -> Self {
        self.is_error = Some(is_error);
        self
    }

    /// Sets the cache control for this tool result.
    ///
    /// # Arguments
    ///
    /// * `cache_control` - The cache control configuration
    pub fn with_cache_control(mut self, cache_control: AnthropicCacheControl) -> Self {
        self.cache_control = Some(cache_control);
        self
    }

    /// Removes cache control from this tool result.
    pub fn without_cache_control(mut self) -> Self {
        self.cache_control = None;
        self
    }

    /// Updates the tool use ID.
    pub fn with_tool_use_id(mut self, tool_use_id: impl Into<String>) -> Self {
        self.tool_use_id = tool_use_id.into();
        self
    }

    /// Updates the content.
    pub fn with_content(mut self, content: ToolResultContentType) -> Self {
        self.content = content;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    use serde_json::json;

    #[test]
    fn test_nested_text_new() {
        let text = AnthropicNestedTextContent::new("Hello");

        assert_eq!(text.content_type, "text");
        assert_eq!(text.text, "Hello");
    }

    #[test]
    fn test_nested_text_from_string() {
        let text: AnthropicNestedTextContent = "Hello".to_string().into();
        assert_eq!(text.text, "Hello");
    }

    #[test]
    fn test_nested_text_from_str() {
        let text: AnthropicNestedTextContent = "Hello".into();
        assert_eq!(text.text, "Hello");
    }

    #[test]
    fn test_nested_text_serialize() {
        let text = AnthropicNestedTextContent::new("Test");
        let json = serde_json::to_value(&text).unwrap();

        assert_eq!(json, json!({"type": "text", "text": "Test"}));
        // Verify no cache_control field
        assert!(!json.as_object().unwrap().contains_key("cache_control"));
    }

    #[test]
    fn test_nested_image_new() {
        let image = AnthropicNestedImageContent::from_url("https://example.com/img.png");

        assert_eq!(image.content_type, "image");
    }

    #[test]
    fn test_nested_image_serialize() {
        let image = AnthropicNestedImageContent::from_url("https://example.com/img.png");
        let json = serde_json::to_value(&image).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "image",
                "source": {"type": "url", "url": "https://example.com/img.png"}
            })
        );
        // Verify no cache_control field
        assert!(!json.as_object().unwrap().contains_key("cache_control"));
    }

    #[test]
    fn test_nested_document_new() {
        let document = AnthropicNestedDocumentContent::from_url("https://example.com/doc.pdf");

        assert_eq!(document.content_type, "document");
        assert!(document.title.is_none());
    }

    #[test]
    fn test_nested_document_with_metadata() {
        let document = AnthropicNestedDocumentContent::from_url("https://example.com/doc.pdf")
            .with_title("Title")
            .with_context("Context")
            .with_citations_enabled();

        assert_eq!(document.title, Some("Title".to_string()));
        assert_eq!(document.context, Some("Context".to_string()));
        assert_eq!(document.citations, Some(DocumentCitations::enabled()));
    }

    #[test]
    fn test_nested_document_serialize() {
        let document = AnthropicNestedDocumentContent::from_url("https://example.com/doc.pdf")
            .with_title("Doc");
        let json = serde_json::to_value(&document).unwrap();

        // Verify no cache_control field
        assert!(!json.as_object().unwrap().contains_key("cache_control"));
    }

    #[test]
    fn test_tool_result_content_type_string() {
        let content = ToolResultContentType::String("Result".to_string());

        match content {
            ToolResultContentType::String(s) => assert_eq!(s, "Result"),
            _ => panic!("Expected String variant"),
        }
    }

    #[test]
    fn test_tool_result_content_type_array() {
        let content = ToolResultContentType::Array(vec![AnthropicNestedContent::Text {
            text: "Text".to_string(),
        }]);

        match content {
            ToolResultContentType::Array(arr) => assert_eq!(arr.len(), 1),
            _ => panic!("Expected Array variant"),
        }
    }

    #[test]
    fn test_tool_result_new() {
        let result = AnthropicToolResultContent::new(
            "toolu_123",
            ToolResultContentType::String("Success".to_string()),
        );

        assert_eq!(result.content_type, "tool_result");
        assert_eq!(result.tool_use_id, "toolu_123");
        assert!(result.is_error.is_none());
        assert!(result.cache_control.is_none());
    }

    #[test]
    fn test_tool_result_from_string() {
        let result = AnthropicToolResultContent::from_string("toolu_123", "Result");

        match result.content {
            ToolResultContentType::String(s) => assert_eq!(s, "Result"),
            _ => panic!("Expected String content"),
        }
    }

    #[test]
    fn test_tool_result_from_array() {
        let content = vec![AnthropicNestedContent::Text {
            text: "Text".to_string(),
        }];
        let result = AnthropicToolResultContent::from_array("toolu_123", content);

        match result.content {
            ToolResultContentType::Array(arr) => assert_eq!(arr.len(), 1),
            _ => panic!("Expected Array content"),
        }
    }

    #[test]
    fn test_tool_result_with_error() {
        let result = AnthropicToolResultContent::from_string("toolu_123", "Error").with_error(true);

        assert_eq!(result.is_error, Some(true));
    }

    #[test]
    fn test_tool_result_with_cache_control() {
        let cache = AnthropicCacheControl::new(CacheTTL::FiveMinutes);
        let result = AnthropicToolResultContent::from_string("toolu_123", "Result")
            .with_cache_control(cache.clone());

        assert_eq!(result.cache_control, Some(cache));
    }

    #[test]
    fn test_tool_result_without_cache_control() {
        let result = AnthropicToolResultContent::from_string("toolu_123", "Result")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour))
            .without_cache_control();

        assert!(result.cache_control.is_none());
    }

    #[test]
    fn test_serialize_string_content() {
        let result = AnthropicToolResultContent::from_string("toolu_123", "Success");
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "tool_result",
                "tool_use_id": "toolu_123",
                "content": "Success"
            })
        );
    }

    #[test]
    fn test_serialize_array_content() {
        let content = vec![
            AnthropicNestedContent::Text {
                text: "Text".to_string(),
            },
            AnthropicNestedContent::Image {
                source: AnthropicContentSource::url("https://example.com/img.png"),
            },
        ];
        let result = AnthropicToolResultContent::from_array("toolu_123", content);
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "tool_result",
                "tool_use_id": "toolu_123",
                "content": [
                    {"type": "text", "text": "Text"},
                    {"type": "image", "source": {"type": "url", "url": "https://example.com/img.png"}}
                ]
            })
        );
    }

    #[test]
    fn test_serialize_with_error() {
        let result = AnthropicToolResultContent::from_string("toolu_123", "Error").with_error(true);
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "tool_result",
                "tool_use_id": "toolu_123",
                "content": "Error",
                "is_error": true
            })
        );
    }

    #[test]
    fn test_serialize_with_cache() {
        let result = AnthropicToolResultContent::from_string("toolu_123", "Cached")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let json = serde_json::to_value(&result).unwrap();

        assert_eq!(
            json,
            json!({
                "type": "tool_result",
                "tool_use_id": "toolu_123",
                "content": "Cached",
                "cache_control": {
                    "type": "ephemeral",
                    "ttl": "1h"
                }
            })
        );
    }

    #[test]
    fn test_deserialize_string_content() {
        let json = json!({
            "type": "tool_result",
            "tool_use_id": "toolu_123",
            "content": "Result"
        });

        let result: AnthropicToolResultContent = serde_json::from_value(json).unwrap();

        assert_eq!(result.tool_use_id, "toolu_123");
        match result.content {
            ToolResultContentType::String(s) => assert_eq!(s, "Result"),
            _ => panic!("Expected String content"),
        }
    }

    #[test]
    fn test_deserialize_array_content() {
        let json = json!({
            "type": "tool_result",
            "tool_use_id": "toolu_123",
            "content": [
                {"type": "text", "text": "Hello"}
            ]
        });

        let result: AnthropicToolResultContent = serde_json::from_value(json).unwrap();

        match result.content {
            ToolResultContentType::Array(arr) => {
                assert_eq!(arr.len(), 1);
                match &arr[0] {
                    AnthropicNestedContent::Text { text } => assert_eq!(text, "Hello"),
                    _ => panic!("Expected Text nested content"),
                }
            }
            _ => panic!("Expected Array content"),
        }
    }

    #[test]
    fn test_equality() {
        let result1 = AnthropicToolResultContent::from_string("id", "same");
        let result2 = AnthropicToolResultContent::from_string("id", "same");
        let result3 = AnthropicToolResultContent::from_string("id", "different");

        assert_eq!(result1, result2);
        assert_ne!(result1, result3);
    }

    #[test]
    fn test_clone() {
        let result1 = AnthropicToolResultContent::from_string("id", "content")
            .with_error(true)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let result2 = result1.clone();

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_builder_chaining() {
        let result = AnthropicToolResultContent::from_string("id1", "content1")
            .with_tool_use_id("id2")
            .with_error(false)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes))
            .with_content(ToolResultContentType::String("content2".to_string()))
            .with_error(true)
            .without_cache_control();

        assert_eq!(result.tool_use_id, "id2");
        match result.content {
            ToolResultContentType::String(s) => assert_eq!(s, "content2"),
            _ => panic!("Expected String content"),
        }
        assert_eq!(result.is_error, Some(true));
        assert!(result.cache_control.is_none());
    }

    #[test]
    fn test_roundtrip_serialization() {
        let content = vec![
            AnthropicNestedContent::Text {
                text: "Text".to_string(),
            },
            AnthropicNestedContent::Document {
                source: AnthropicContentSource::url("https://example.com/doc.pdf"),
                title: Some("Doc".to_string()),
                context: None,
                citations: None,
            },
        ];
        let original = AnthropicToolResultContent::from_array("toolu_roundtrip", content)
            .with_error(false)
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::FiveMinutes));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicToolResultContent = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_nested_content_no_cache_control() {
        // Verify that nested content types don't have cache_control field
        let text_json = serde_json::to_value(AnthropicNestedTextContent::new("text")).unwrap();
        assert!(!text_json.as_object().unwrap().contains_key("cache_control"));

        let image_json =
            serde_json::to_value(AnthropicNestedImageContent::from_url("url")).unwrap();
        assert!(
            !image_json
                .as_object()
                .unwrap()
                .contains_key("cache_control")
        );

        let doc_json =
            serde_json::to_value(AnthropicNestedDocumentContent::from_url("url")).unwrap();
        assert!(!doc_json.as_object().unwrap().contains_key("cache_control"));
    }
}
