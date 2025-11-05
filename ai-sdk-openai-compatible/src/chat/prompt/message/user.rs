use ai_sdk_provider::language_model::prompt::message::LanguageModelUserMessage;
use ai_sdk_provider::language_model::prompt::{
    LanguageModelDataContent, LanguageModelUserMessagePart,
};
use ai_sdk_provider::shared::provider_options::SharedProviderOptions;
use base64::{engine::general_purpose, Engine as _};
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

    /// Creates a user message from a provider user message
    pub fn from_provider(msg: LanguageModelUserMessage) -> Result<Self, String> {
        let content = msg.content;
        let provider_options = msg.provider_options;

        // Check if it's a simple text message
        if content.len() == 1 {
            if let LanguageModelUserMessagePart::Text(text_part) = &content[0] {
                let mut user_msg = Self::new_text(text_part.text.clone());
                if let Some(metadata) = get_openai_metadata(&text_part.provider_options) {
                    user_msg.additional_properties = Some(metadata);
                }
                return Ok(user_msg);
            }
        }

        // Multi-part message
        let mut parts: Vec<OpenAICompatibleContentPart> = Vec::new();

        for part in content {
            match part {
                LanguageModelUserMessagePart::Text(text_part) => {
                    let mut text_part_out = OpenAICompatibleContentPartText::new(text_part.text);
                    if let Some(metadata) = get_openai_metadata(&text_part.provider_options) {
                        text_part_out.additional_properties = Some(metadata);
                    }
                    parts.push(OpenAICompatibleContentPart::Text(text_part_out));
                }
                LanguageModelUserMessagePart::File(file_part) => {
                    if file_part.media_type.starts_with("image/") {
                        // Handle wildcard image type
                        let actual_media_type = if file_part.media_type == "image/*" {
                            "image/jpeg"
                        } else {
                            &file_part.media_type
                        };

                        let url = convert_data_to_url(&file_part.data, actual_media_type);
                        let mut image_part = OpenAICompatibleContentPartImage::new(url);
                        if let Some(metadata) = get_openai_metadata(&file_part.provider_options) {
                            image_part.additional_properties = Some(metadata);
                        }
                        parts.push(OpenAICompatibleContentPart::ImageUrl(image_part));
                    } else {
                        return Err(format!(
                            "Unsupported file media type: {}",
                            file_part.media_type
                        ));
                    }
                }
            }
        }

        let mut user_msg = Self::new_parts(parts);
        if let Some(metadata) = get_openai_metadata(&provider_options) {
            user_msg.additional_properties = Some(metadata);
        }
        Ok(user_msg)
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

/// Extracts OpenAI-compatible metadata from provider options
fn get_openai_metadata(provider_options: &Option<SharedProviderOptions>) -> Option<Value> {
    provider_options
        .as_ref()
        .and_then(|opts| opts.get("openaiCompatible"))
        .map(|metadata| serde_json::to_value(metadata).ok())
        .flatten()
}

/// Converts data content to a base64 or URL string for image URLs
fn convert_data_to_url(data: &LanguageModelDataContent, media_type: &str) -> String {
    match data {
        LanguageModelDataContent::Url(url) => url.to_string(),
        LanguageModelDataContent::Base64(base64) => {
            format!("data:{};base64,{}", media_type, base64)
        }
        LanguageModelDataContent::Bytes(bytes) => {
            let base64 = general_purpose::STANDARD.encode(bytes);
            format!("data:{};base64,{}", media_type, base64)
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
