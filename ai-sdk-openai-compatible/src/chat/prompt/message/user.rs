use serde::{Deserialize, Serialize};
use serde_json::Value;

/// User message with text or multimodal content
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAICompatibleUserMessage {
    /// The user message content (can be a string or array of content parts)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<UserMessageContent>,

    /// Additional properties for extensibility (flattened into the struct)
    #[serde(flatten)]
    pub additional_properties: Option<Value>,
}

/// User message content can be either a simple string or an array of content parts
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserMessageContent {
    /// Simple text content
    Text(String),

    /// Array of content parts (text, images, etc.)
    Parts(Vec<OpenAICompatibleContentPart>),
}

/// Content part in a user message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OpenAICompatibleContentPart {
    /// Text content part
    Text(OpenAICompatibleContentPartText),

    /// Image URL content part
    ImageUrl(OpenAICompatibleContentPartImage),
}

/// Text content part
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAICompatibleContentPartText {
    /// The text content
    pub text: String,

    /// Additional properties for extensibility (flattened into the struct)
    #[serde(flatten)]
    pub additional_properties: Option<Value>,
}

/// Image URL content part
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAICompatibleContentPartImage {
    /// Image URL information
    pub image_url: ImageUrl,

    /// Additional properties for extensibility (flattened into the struct)
    #[serde(flatten)]
    pub additional_properties: Option<Value>,
}

/// Image URL information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageUrl {
    /// The URL of the image
    pub url: String,
}

impl OpenAICompatibleUserMessage {
    /// Creates a new user message with text content
    pub fn new_text(content: String) -> Self {
        Self {
            content: Some(UserMessageContent::Text(content)),
            additional_properties: None,
        }
    }

    /// Creates a new user message with content parts
    pub fn new_parts(parts: Vec<OpenAICompatibleContentPart>) -> Self {
        Self {
            content: Some(UserMessageContent::Parts(parts)),
            additional_properties: None,
        }
    }
}

impl OpenAICompatibleContentPartText {
    /// Creates a new text content part
    pub fn new(text: String) -> Self {
        Self {
            text,
            additional_properties: None,
        }
    }
}

impl OpenAICompatibleContentPartImage {
    /// Creates a new image content part
    pub fn new(url: String) -> Self {
        Self {
            image_url: ImageUrl { url },
            additional_properties: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_message_text() {
        let msg = OpenAICompatibleUserMessage::new_text("Hello".to_string());
        match msg.content {
            Some(UserMessageContent::Text(text)) => assert_eq!(text, "Hello"),
            _ => panic!("Expected text content"),
        }
    }

    #[test]
    fn test_user_message_parts() {
        let parts = vec![OpenAICompatibleContentPart::Text(
            OpenAICompatibleContentPartText::new("Hello".to_string()),
        )];
        let msg = OpenAICompatibleUserMessage::new_parts(parts);
        match msg.content {
            Some(UserMessageContent::Parts(parts)) => assert_eq!(parts.len(), 1),
            _ => panic!("Expected parts content"),
        }
    }
}
