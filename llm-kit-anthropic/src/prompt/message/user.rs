use serde::{Deserialize, Deserializer, Serialize};

use crate::prompt::message::content::document::AnthropicDocumentContent;
use crate::prompt::message::content::image::AnthropicImageContent;
use crate::prompt::message::content::text::AnthropicTextContent;
use crate::prompt::message::content::tool_result::AnthropicToolResultContent;

/// User message content types.
///
/// User messages can contain text, images, documents, or tool results.
/// Uses custom deserialization to properly distinguish between image and document content
/// based on the `type` field value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum UserMessageContent {
    /// Tool result content (has unique `tool_use_id` field)
    ToolResult(AnthropicToolResultContent),

    /// Text content (simplest structure with just `text` field)
    Text(AnthropicTextContent),

    /// Image content (has `source` field, no optional metadata fields)
    Image(AnthropicImageContent),

    /// Document content (has `source` + optional metadata)
    Document(AnthropicDocumentContent),
}

impl<'de> Deserialize<'de> for UserMessageContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        // Check for tool_use_id field to identify ToolResult
        if value.get("tool_use_id").is_some() {
            return serde_json::from_value(value)
                .map(UserMessageContent::ToolResult)
                .map_err(serde::de::Error::custom);
        }

        // Check the type field
        if let Some(content_type) = value.get("type").and_then(|v| v.as_str()) {
            match content_type {
                "text" => {
                    return serde_json::from_value(value)
                        .map(UserMessageContent::Text)
                        .map_err(serde::de::Error::custom);
                }
                "image" => {
                    return serde_json::from_value(value)
                        .map(UserMessageContent::Image)
                        .map_err(serde::de::Error::custom);
                }
                "document" => {
                    return serde_json::from_value(value)
                        .map(UserMessageContent::Document)
                        .map_err(serde::de::Error::custom);
                }
                _ => {}
            }
        }

        // Fallback: try each variant (for backwards compatibility)
        if let Ok(text) = serde_json::from_value::<AnthropicTextContent>(value.clone()) {
            return Ok(UserMessageContent::Text(text));
        }
        if let Ok(image) = serde_json::from_value::<AnthropicImageContent>(value.clone()) {
            return Ok(UserMessageContent::Image(image));
        }
        if let Ok(document) = serde_json::from_value::<AnthropicDocumentContent>(value.clone()) {
            return Ok(UserMessageContent::Document(document));
        }
        if let Ok(tool_result) = serde_json::from_value::<AnthropicToolResultContent>(value) {
            return Ok(UserMessageContent::ToolResult(tool_result));
        }

        Err(serde::de::Error::custom(
            "unable to deserialize UserMessageContent: unknown content type",
        ))
    }
}

impl From<AnthropicTextContent> for UserMessageContent {
    fn from(content: AnthropicTextContent) -> Self {
        Self::Text(content)
    }
}

impl From<AnthropicImageContent> for UserMessageContent {
    fn from(content: AnthropicImageContent) -> Self {
        Self::Image(content)
    }
}

impl From<AnthropicDocumentContent> for UserMessageContent {
    fn from(content: AnthropicDocumentContent) -> Self {
        Self::Document(content)
    }
}

impl From<AnthropicToolResultContent> for UserMessageContent {
    fn from(content: AnthropicToolResultContent) -> Self {
        Self::ToolResult(content)
    }
}

/// Anthropic user message.
///
/// Represents a message from the user in a conversation with the Anthropic API.
/// User messages can contain various types of content including text, images,
/// documents, and tool results.
///
/// # Example
///
/// ```
/// use llm_kit_anthropic::prompt::message::user::AnthropicUserMessage;
/// use llm_kit_anthropic::prompt::message::content::text::AnthropicTextContent;
///
/// // Create a simple text message
/// let message = AnthropicUserMessage::new(vec![
///     AnthropicTextContent::new("Hello, how are you?").into()
/// ]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnthropicUserMessage {
    /// The role (always "user")
    pub role: String,

    /// The message content (array of content blocks)
    pub content: Vec<UserMessageContent>,
}

impl AnthropicUserMessage {
    /// Creates a new user message.
    ///
    /// # Arguments
    ///
    /// * `content` - The message content as a vector of content blocks
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::user::AnthropicUserMessage;
    /// use llm_kit_anthropic::prompt::message::content::text::AnthropicTextContent;
    ///
    /// let message = AnthropicUserMessage::new(vec![
    ///     AnthropicTextContent::new("What is the weather today?").into()
    /// ]);
    /// ```
    pub fn new(content: Vec<UserMessageContent>) -> Self {
        Self {
            role: "user".to_string(),
            content,
        }
    }

    /// Creates a new user message with a single text content block.
    ///
    /// # Arguments
    ///
    /// * `text` - The text content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::user::AnthropicUserMessage;
    ///
    /// let message = AnthropicUserMessage::from_text("Hello, Claude!");
    /// ```
    pub fn from_text(text: impl Into<String>) -> Self {
        Self::new(vec![AnthropicTextContent::new(text).into()])
    }

    /// Creates a new user message with a single image content block.
    ///
    /// # Arguments
    ///
    /// * `image` - The image content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::user::AnthropicUserMessage;
    /// use llm_kit_anthropic::prompt::message::content::image::AnthropicImageContent;
    /// use llm_kit_anthropic::prompt::message::content::source_type::AnthropicContentSource;
    ///
    /// let message = AnthropicUserMessage::from_image(
    ///     AnthropicImageContent::new(
    ///         AnthropicContentSource::url("https://example.com/image.png")
    ///     )
    /// );
    /// ```
    pub fn from_image(image: AnthropicImageContent) -> Self {
        Self::new(vec![image.into()])
    }

    /// Creates a new user message with a single document content block.
    ///
    /// # Arguments
    ///
    /// * `document` - The document content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::user::AnthropicUserMessage;
    /// use llm_kit_anthropic::prompt::message::content::document::AnthropicDocumentContent;
    /// use llm_kit_anthropic::prompt::message::content::source_type::AnthropicContentSource;
    ///
    /// let message = AnthropicUserMessage::from_document(
    ///     AnthropicDocumentContent::new(
    ///         AnthropicContentSource::url("https://example.com/doc.pdf")
    ///     )
    /// );
    /// ```
    pub fn from_document(document: AnthropicDocumentContent) -> Self {
        Self::new(vec![document.into()])
    }

    /// Creates a new user message with a single tool result content block.
    ///
    /// # Arguments
    ///
    /// * `tool_result` - The tool result content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::user::AnthropicUserMessage;
    /// use llm_kit_anthropic::prompt::message::content::tool_result::AnthropicToolResultContent;
    ///
    /// let message = AnthropicUserMessage::from_tool_result(
    ///     AnthropicToolResultContent::from_string("toolu_123", "Tool executed successfully")
    /// );
    /// ```
    pub fn from_tool_result(tool_result: AnthropicToolResultContent) -> Self {
        Self::new(vec![tool_result.into()])
    }

    /// Adds a content block to the message.
    ///
    /// # Arguments
    ///
    /// * `content` - The content block to add
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::user::AnthropicUserMessage;
    /// use llm_kit_anthropic::prompt::message::content::text::AnthropicTextContent;
    ///
    /// let message = AnthropicUserMessage::from_text("First message")
    ///     .with_content(AnthropicTextContent::new("Second message").into());
    /// ```
    pub fn with_content(mut self, content: UserMessageContent) -> Self {
        self.content.push(content);
        self
    }

    /// Adds a text content block to the message.
    ///
    /// # Arguments
    ///
    /// * `text` - The text content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::user::AnthropicUserMessage;
    ///
    /// let message = AnthropicUserMessage::from_text("Hello")
    ///     .with_text("How are you?");
    /// ```
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.content.push(AnthropicTextContent::new(text).into());
        self
    }

    /// Adds an image content block to the message.
    ///
    /// # Arguments
    ///
    /// * `image` - The image content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::user::AnthropicUserMessage;
    /// use llm_kit_anthropic::prompt::message::content::image::AnthropicImageContent;
    /// use llm_kit_anthropic::prompt::message::content::source_type::AnthropicContentSource;
    ///
    /// let message = AnthropicUserMessage::from_text("Look at this image:")
    ///     .with_image(AnthropicImageContent::new(
    ///         AnthropicContentSource::url("https://example.com/image.png")
    ///     ));
    /// ```
    pub fn with_image(mut self, image: AnthropicImageContent) -> Self {
        self.content.push(image.into());
        self
    }

    /// Adds a document content block to the message.
    ///
    /// # Arguments
    ///
    /// * `document` - The document content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::user::AnthropicUserMessage;
    /// use llm_kit_anthropic::prompt::message::content::document::AnthropicDocumentContent;
    /// use llm_kit_anthropic::prompt::message::content::source_type::AnthropicContentSource;
    ///
    /// let message = AnthropicUserMessage::from_text("Please review this document:")
    ///     .with_document(AnthropicDocumentContent::new(
    ///         AnthropicContentSource::url("https://example.com/doc.pdf")
    ///     ));
    /// ```
    pub fn with_document(mut self, document: AnthropicDocumentContent) -> Self {
        self.content.push(document.into());
        self
    }

    /// Adds a tool result content block to the message.
    ///
    /// # Arguments
    ///
    /// * `tool_result` - The tool result content
    ///
    /// # Example
    ///
    /// ```
    /// use llm_kit_anthropic::prompt::message::user::AnthropicUserMessage;
    /// use llm_kit_anthropic::prompt::message::content::tool_result::AnthropicToolResultContent;
    ///
    /// let message = AnthropicUserMessage::from_text("Here are the tool results:")
    ///     .with_tool_result(
    ///         AnthropicToolResultContent::from_string("toolu_123", "Success")
    ///     );
    /// ```
    pub fn with_tool_result(mut self, tool_result: AnthropicToolResultContent) -> Self {
        self.content.push(tool_result.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::cache_control::{AnthropicCacheControl, CacheTTL};
    use crate::prompt::message::content::source_type::AnthropicContentSource;
    use crate::prompt::message::content::tool_result::ToolResultContentType;
    use serde_json::json;

    #[test]
    fn test_user_message_content_from_text() {
        let text = AnthropicTextContent::new("Hello");
        let content: UserMessageContent = text.into();

        match content {
            UserMessageContent::Text(t) => assert_eq!(t.text, "Hello"),
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_user_message_content_from_image() {
        let image = AnthropicImageContent::new(AnthropicContentSource::url(
            "https://example.com/image.png",
        ));
        let content: UserMessageContent = image.into();

        match content {
            UserMessageContent::Image(_) => (),
            _ => panic!("Expected Image variant"),
        }
    }

    #[test]
    fn test_user_message_content_from_document() {
        let document = AnthropicDocumentContent::new(AnthropicContentSource::url(
            "https://example.com/doc.pdf",
        ));
        let content: UserMessageContent = document.into();

        match content {
            UserMessageContent::Document(_) => (),
            _ => panic!("Expected Document variant"),
        }
    }

    #[test]
    fn test_user_message_content_from_tool_result() {
        let tool_result = AnthropicToolResultContent::from_string("toolu_123", "Success");
        let content: UserMessageContent = tool_result.into();

        match content {
            UserMessageContent::ToolResult(_) => (),
            _ => panic!("Expected ToolResult variant"),
        }
    }

    #[test]
    fn test_new() {
        let message = AnthropicUserMessage::new(vec![AnthropicTextContent::new("Hello").into()]);

        assert_eq!(message.role, "user");
        assert_eq!(message.content.len(), 1);
    }

    #[test]
    fn test_from_text() {
        let message = AnthropicUserMessage::from_text("Hello, Claude!");

        assert_eq!(message.role, "user");
        assert_eq!(message.content.len(), 1);
        match &message.content[0] {
            UserMessageContent::Text(t) => assert_eq!(t.text, "Hello, Claude!"),
            _ => panic!("Expected Text content"),
        }
    }

    #[test]
    fn test_from_image() {
        let image = AnthropicImageContent::new(AnthropicContentSource::url(
            "https://example.com/image.png",
        ));
        let message = AnthropicUserMessage::from_image(image);

        assert_eq!(message.role, "user");
        assert_eq!(message.content.len(), 1);
        match &message.content[0] {
            UserMessageContent::Image(_) => (),
            _ => panic!("Expected Image content"),
        }
    }

    #[test]
    fn test_from_document() {
        let document = AnthropicDocumentContent::new(AnthropicContentSource::url(
            "https://example.com/doc.pdf",
        ));
        let message = AnthropicUserMessage::from_document(document);

        assert_eq!(message.role, "user");
        assert_eq!(message.content.len(), 1);
        match &message.content[0] {
            UserMessageContent::Document(_) => (),
            _ => panic!("Expected Document content"),
        }
    }

    #[test]
    fn test_from_tool_result() {
        let tool_result = AnthropicToolResultContent::from_string("toolu_123", "Success");
        let message = AnthropicUserMessage::from_tool_result(tool_result);

        assert_eq!(message.role, "user");
        assert_eq!(message.content.len(), 1);
        match &message.content[0] {
            UserMessageContent::ToolResult(_) => (),
            _ => panic!("Expected ToolResult content"),
        }
    }

    #[test]
    fn test_with_content() {
        let message = AnthropicUserMessage::from_text("First")
            .with_content(AnthropicTextContent::new("Second").into());

        assert_eq!(message.content.len(), 2);
    }

    #[test]
    fn test_with_text() {
        let message = AnthropicUserMessage::from_text("First").with_text("Second");

        assert_eq!(message.content.len(), 2);
        match &message.content[1] {
            UserMessageContent::Text(t) => assert_eq!(t.text, "Second"),
            _ => panic!("Expected Text content"),
        }
    }

    #[test]
    fn test_with_image() {
        let message = AnthropicUserMessage::from_text("Look at this:").with_image(
            AnthropicImageContent::new(AnthropicContentSource::url(
                "https://example.com/image.png",
            )),
        );

        assert_eq!(message.content.len(), 2);
        match &message.content[1] {
            UserMessageContent::Image(_) => (),
            _ => panic!("Expected Image content"),
        }
    }

    #[test]
    fn test_with_document() {
        let message = AnthropicUserMessage::from_text("Review this:").with_document(
            AnthropicDocumentContent::new(AnthropicContentSource::url(
                "https://example.com/doc.pdf",
            )),
        );

        assert_eq!(message.content.len(), 2);
        match &message.content[1] {
            UserMessageContent::Document(_) => (),
            _ => panic!("Expected Document content"),
        }
    }

    #[test]
    fn test_with_tool_result() {
        let message = AnthropicUserMessage::from_text("Tool results:").with_tool_result(
            AnthropicToolResultContent::from_string("toolu_123", "Success"),
        );

        assert_eq!(message.content.len(), 2);
        match &message.content[1] {
            UserMessageContent::ToolResult(_) => (),
            _ => panic!("Expected ToolResult content"),
        }
    }

    #[test]
    fn test_builder_chaining() {
        let message = AnthropicUserMessage::from_text("Text 1")
            .with_text("Text 2")
            .with_image(AnthropicImageContent::new(AnthropicContentSource::url(
                "https://example.com/img.png",
            )))
            .with_document(AnthropicDocumentContent::new(AnthropicContentSource::url(
                "https://example.com/doc.pdf",
            )))
            .with_tool_result(AnthropicToolResultContent::from_string(
                "toolu_123",
                "Result",
            ));

        assert_eq!(message.content.len(), 5);
    }

    #[test]
    fn test_serialize_text_only() {
        let message = AnthropicUserMessage::from_text("Hello, Claude!");
        let json = serde_json::to_value(&message).unwrap();

        assert_eq!(
            json,
            json!({
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Hello, Claude!"
                    }
                ]
            })
        );
    }

    #[test]
    fn test_serialize_mixed_content() {
        let message = AnthropicUserMessage::from_text("Here is an image:").with_image(
            AnthropicImageContent::new(AnthropicContentSource::url(
                "https://example.com/image.png",
            )),
        );
        let json = serde_json::to_value(&message).unwrap();

        assert_eq!(
            json,
            json!({
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Here is an image:"
                    },
                    {
                        "type": "image",
                        "source": {
                            "type": "url",
                            "url": "https://example.com/image.png"
                        }
                    }
                ]
            })
        );
    }

    #[test]
    fn test_serialize_with_tool_result() {
        let message = AnthropicUserMessage::from_tool_result(
            AnthropicToolResultContent::new(
                "toolu_123",
                ToolResultContentType::String("Success".to_string()),
            )
            .with_error(false),
        );
        let json = serde_json::to_value(&message).unwrap();

        assert_eq!(
            json,
            json!({
                "role": "user",
                "content": [
                    {
                        "type": "tool_result",
                        "tool_use_id": "toolu_123",
                        "content": "Success",
                        "is_error": false
                    }
                ]
            })
        );
    }

    #[test]
    fn test_serialize_with_cache_control() {
        let text = AnthropicTextContent::new("Cached content")
            .with_cache_control(AnthropicCacheControl::new(CacheTTL::OneHour));
        let message = AnthropicUserMessage::new(vec![text.into()]);
        let json = serde_json::to_value(&message).unwrap();

        assert_eq!(
            json,
            json!({
                "role": "user",
                "content": [
                    {
                        "type": "text",
                        "text": "Cached content",
                        "cache_control": {
                            "type": "ephemeral",
                            "ttl": "1h"
                        }
                    }
                ]
            })
        );
    }

    #[test]
    fn test_deserialize_text_only() {
        let json = json!({
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Hello, Claude!"
                }
            ]
        });

        let message: AnthropicUserMessage = serde_json::from_value(json).unwrap();

        assert_eq!(message.role, "user");
        assert_eq!(message.content.len(), 1);
        match &message.content[0] {
            UserMessageContent::Text(t) => assert_eq!(t.text, "Hello, Claude!"),
            _ => panic!("Expected Text content"),
        }
    }

    #[test]
    fn test_deserialize_mixed_content() {
        let json = json!({
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Here is an image:"
                },
                {
                    "type": "image",
                    "source": {
                        "type": "url",
                        "url": "https://example.com/image.png"
                    }
                }
            ]
        });

        let message: AnthropicUserMessage = serde_json::from_value(json).unwrap();

        assert_eq!(message.content.len(), 2);
        match &message.content[0] {
            UserMessageContent::Text(_) => (),
            _ => panic!("Expected Text content"),
        }
        match &message.content[1] {
            UserMessageContent::Image(_) => (),
            _ => panic!("Expected Image content"),
        }
    }

    #[test]
    fn test_deserialize_with_tool_result() {
        let json = json!({
            "role": "user",
            "content": [
                {
                    "type": "tool_result",
                    "tool_use_id": "toolu_123",
                    "content": "Success"
                }
            ]
        });

        let message: AnthropicUserMessage = serde_json::from_value(json).unwrap();

        assert_eq!(message.content.len(), 1);
        match &message.content[0] {
            UserMessageContent::ToolResult(tr) => {
                assert_eq!(tr.tool_use_id, "toolu_123");
            }
            _ => panic!("Expected ToolResult content"),
        }
    }

    #[test]
    fn test_deserialize_with_document() {
        let json = json!({
            "role": "user",
            "content": [
                {
                    "type": "document",
                    "source": {
                        "type": "url",
                        "url": "https://example.com/doc.pdf"
                    }
                }
            ]
        });

        let message: AnthropicUserMessage = serde_json::from_value(json).unwrap();

        assert_eq!(message.content.len(), 1);
        match &message.content[0] {
            UserMessageContent::Document(_) => (),
            _ => panic!("Expected Document content"),
        }
    }

    #[test]
    fn test_roundtrip_serialization() {
        let original = AnthropicUserMessage::from_text("Hello")
            .with_image(AnthropicImageContent::new(AnthropicContentSource::url(
                "https://example.com/img.png",
            )))
            .with_tool_result(AnthropicToolResultContent::from_string(
                "toolu_123",
                "Result",
            ));

        let json = serde_json::to_value(&original).unwrap();
        let deserialized: AnthropicUserMessage = serde_json::from_value(json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_equality() {
        let msg1 = AnthropicUserMessage::from_text("Hello");
        let msg2 = AnthropicUserMessage::from_text("Hello");
        let msg3 = AnthropicUserMessage::from_text("Goodbye");

        assert_eq!(msg1, msg2);
        assert_ne!(msg1, msg3);
    }

    #[test]
    fn test_clone() {
        let msg1 = AnthropicUserMessage::from_text("Hello").with_image(AnthropicImageContent::new(
            AnthropicContentSource::url("https://example.com/img.png"),
        ));
        let msg2 = msg1.clone();

        assert_eq!(msg1, msg2);
    }

    #[test]
    fn test_empty_content() {
        let message = AnthropicUserMessage::new(vec![]);

        assert_eq!(message.role, "user");
        assert!(message.content.is_empty());
    }
}
