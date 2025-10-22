use serde::{Deserialize, Serialize};
use ai_sdk_provider::shared::provider_options::ProviderOptions;
use super::text_part::TextPart;
use super::image_part::ImagePart;
use super::file_part::FilePart;

/// Content of a user message. It can be a string or an array of text, image, and file parts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserContent {
    /// Simple text content as a string.
    Text(String),
    /// Array of content parts (text, images, or files).
    Parts(Vec<UserContentPart>),
}

/// A part of user content, either text, image, or file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserContentPart {
    Text(TextPart),
    Image(ImagePart),
    File(FilePart),
}

/// A user message. It can contain text or a combination of text and images.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserModelMessage {
    /// Role is always 'user' for user messages.
    pub role: String,

    /// Content of the user message.
    pub content: UserContent,

    /// Additional provider-specific metadata. They are passed through
    /// to the provider from the AI SDK and enable provider-specific
    /// functionality that can be fully encapsulated in the provider.
    #[serde(rename = "providerOptions", skip_serializing_if = "Option::is_none")]
    pub provider_options: Option<ProviderOptions>,
}

impl UserModelMessage {
    /// Creates a new user model message with text content.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: UserContent::Text(content.into()),
            provider_options: None,
        }
    }

    /// Creates a new user model message with multi-part content.
    pub fn with_parts(parts: Vec<UserContentPart>) -> Self {
        Self {
            role: "user".to_string(),
            content: UserContent::Parts(parts),
            provider_options: None,
        }
    }

    /// Sets the provider options for this message.
    pub fn with_provider_options(mut self, options: ProviderOptions) -> Self {
        self.provider_options = Some(options);
        self
    }
}

impl From<String> for UserContent {
    fn from(s: String) -> Self {
        UserContent::Text(s)
    }
}

impl From<&str> for UserContent {
    fn from(s: &str) -> Self {
        UserContent::Text(s.to_string())
    }
}

impl From<Vec<UserContentPart>> for UserContent {
    fn from(parts: Vec<UserContentPart>) -> Self {
        UserContent::Parts(parts)
    }
}

impl From<TextPart> for UserContentPart {
    fn from(part: TextPart) -> Self {
        UserContentPart::Text(part)
    }
}

impl From<ImagePart> for UserContentPart {
    fn from(part: ImagePart) -> Self {
        UserContentPart::Image(part)
    }
}

impl From<FilePart> for UserContentPart {
    fn from(part: FilePart) -> Self {
        UserContentPart::File(part)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn test_user_model_message_new_text() {
        let message = UserModelMessage::new("Hello, world!");

        assert_eq!(message.role, "user");
        match message.content {
            UserContent::Text(text) => assert_eq!(text, "Hello, world!"),
            _ => panic!("Expected Text variant"),
        }
        assert!(message.provider_options.is_none());
    }

    #[test]
    fn test_user_model_message_with_parts() {
        let parts = vec![
            UserContentPart::Text(TextPart::new("Hello")),
            UserContentPart::Image(ImagePart::from_url(
                Url::parse("https://example.com/image.png").unwrap()
            )),
        ];

        let message = UserModelMessage::with_parts(parts);

        assert_eq!(message.role, "user");
        match message.content {
            UserContent::Parts(parts) => {
                assert_eq!(parts.len(), 2);
                assert!(matches!(parts[0], UserContentPart::Text(_)));
                assert!(matches!(parts[1], UserContentPart::Image(_)));
            }
            _ => panic!("Expected Parts variant"),
        }
    }

    #[test]
    fn test_user_model_message_with_provider_options() {
        let provider_options = ProviderOptions::new();

        let message = UserModelMessage::new("Hello")
            .with_provider_options(provider_options.clone());

        assert_eq!(message.provider_options, Some(provider_options));
    }

    #[test]
    fn test_user_content_from_string() {
        let content: UserContent = "Hello".to_string().into();

        match content {
            UserContent::Text(text) => assert_eq!(text, "Hello"),
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_user_content_from_str() {
        let content: UserContent = "Hello".into();

        match content {
            UserContent::Text(text) => assert_eq!(text, "Hello"),
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_user_content_from_parts() {
        let parts = vec![UserContentPart::Text(TextPart::new("Hello"))];
        let content: UserContent = parts.into();

        match content {
            UserContent::Parts(parts) => assert_eq!(parts.len(), 1),
            _ => panic!("Expected Parts variant"),
        }
    }

    #[test]
    fn test_user_content_part_from_text() {
        let part: UserContentPart = TextPart::new("Hello").into();

        assert!(matches!(part, UserContentPart::Text(_)));
    }

    #[test]
    fn test_user_content_part_from_image() {
        let part: UserContentPart = ImagePart::from_url(
            Url::parse("https://example.com/image.png").unwrap()
        ).into();

        assert!(matches!(part, UserContentPart::Image(_)));
    }

    #[test]
    fn test_user_content_part_from_file() {
        let part: UserContentPart = FilePart::from_url(
            Url::parse("https://example.com/file.pdf").unwrap(),
            "application/pdf"
        ).into();

        assert!(matches!(part, UserContentPart::File(_)));
    }

    #[test]
    fn test_user_model_message_serialization_text() {
        let message = UserModelMessage::new("Hello, world!");

        let serialized = serde_json::to_value(&message).unwrap();

        assert_eq!(serialized["role"], "user");
        assert_eq!(serialized["content"], "Hello, world!");
        assert!(serialized.get("providerOptions").is_none());
    }

    #[test]
    fn test_user_model_message_serialization_parts() {
        let parts = vec![
            UserContentPart::Text(TextPart::new("Hello")),
        ];

        let message = UserModelMessage::with_parts(parts);

        let serialized = serde_json::to_value(&message).unwrap();

        assert_eq!(serialized["role"], "user");
        assert!(serialized["content"].is_array());
        assert_eq!(serialized["content"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_user_model_message_serialization_with_provider_options() {
        let provider_options = ProviderOptions::new();

        let message = UserModelMessage::new("Hello")
            .with_provider_options(provider_options);

        let serialized = serde_json::to_value(&message).unwrap();

        assert_eq!(serialized["role"], "user");
        assert!(serialized.get("providerOptions").is_some());
    }

    #[test]
    fn test_user_model_message_deserialization_text() {
        let json = serde_json::json!({
            "role": "user",
            "content": "Hello, world!"
        });

        let message: UserModelMessage = serde_json::from_value(json).unwrap();

        assert_eq!(message.role, "user");
        match message.content {
            UserContent::Text(text) => assert_eq!(text, "Hello, world!"),
            _ => panic!("Expected Text variant"),
        }
        assert!(message.provider_options.is_none());
    }

    #[test]
    fn test_user_model_message_deserialization_parts() {
        let json = serde_json::json!({
            "role": "user",
            "content": [
                {
                    "type": "text",
                    "text": "Hello"
                }
            ]
        });

        let message: UserModelMessage = serde_json::from_value(json).unwrap();

        assert_eq!(message.role, "user");
        match message.content {
            UserContent::Parts(parts) => {
                assert_eq!(parts.len(), 1);
                assert!(matches!(parts[0], UserContentPart::Text(_)));
            }
            _ => panic!("Expected Parts variant"),
        }
    }

    #[test]
    fn test_user_model_message_clone() {
        let message = UserModelMessage::new("Hello");
        let cloned = message.clone();

        assert_eq!(message, cloned);
    }

    #[test]
    fn test_user_model_message_mixed_content() {
        let parts = vec![
            UserContentPart::Text(TextPart::new("Check out this image:")),
            UserContentPart::Image(ImagePart::from_url(
                Url::parse("https://example.com/image.png").unwrap()
            )),
            UserContentPart::Text(TextPart::new("And this file:")),
            UserContentPart::File(FilePart::from_url(
                Url::parse("https://example.com/file.pdf").unwrap(),
                "application/pdf"
            )),
        ];

        let message = UserModelMessage::with_parts(parts);

        match message.content {
            UserContent::Parts(parts) => {
                assert_eq!(parts.len(), 4);
                assert!(matches!(parts[0], UserContentPart::Text(_)));
                assert!(matches!(parts[1], UserContentPart::Image(_)));
                assert!(matches!(parts[2], UserContentPart::Text(_)));
                assert!(matches!(parts[3], UserContentPart::File(_)));
            }
            _ => panic!("Expected Parts variant"),
        }
    }
}
