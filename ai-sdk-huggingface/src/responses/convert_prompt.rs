use ai_sdk_provider::language_model::call_warning::LanguageModelCallWarning;
use ai_sdk_provider::language_model::prompt::message::{
    LanguageModelAssistantMessagePart, LanguageModelDataContent, LanguageModelMessage,
    LanguageModelUserMessagePart,
};
use serde_json::{Value, json};

/// Converts a prompt to Hugging Face Responses API format.
///
/// Returns (input array, warnings).
pub async fn convert_to_huggingface_responses_messages(
    prompt: Vec<LanguageModelMessage>,
) -> Result<(Vec<Value>, Vec<LanguageModelCallWarning>), Box<dyn std::error::Error>> {
    let mut messages = Vec::new();
    let mut warnings = Vec::new();

    for message in prompt {
        match message {
            LanguageModelMessage::System(sys_msg) => {
                messages.push(json!({
                    "role": "system",
                    "content": sys_msg.content
                }));
            }

            LanguageModelMessage::User(user_msg) => {
                let content_parts: Vec<Value> = user_msg
                    .content
                    .into_iter()
                    .filter_map(|part| match part {
                        LanguageModelUserMessagePart::Text(text_part) => Some(json!({
                            "type": "input_text",
                            "text": text_part.text
                        })),
                        LanguageModelUserMessagePart::File(file_part) => {
                            // Check if this is an image
                            if file_part.media_type.starts_with("image/") {
                                let media_type = if file_part.media_type == "image/*" {
                                    "image/jpeg"
                                } else {
                                    &file_part.media_type
                                };

                                let image_url = match file_part.data {
                                    LanguageModelDataContent::Url(url) => url.to_string(),
                                    LanguageModelDataContent::Base64(base64) => {
                                        format!("data:{};base64,{}", media_type, base64)
                                    }
                                    LanguageModelDataContent::Bytes(bytes) => {
                                        use base64::{Engine as _, engine::general_purpose};
                                        let encoded = general_purpose::STANDARD.encode(&bytes);
                                        format!("data:{};base64,{}", media_type, encoded)
                                    }
                                };

                                Some(json!({
                                    "type": "input_image",
                                    "image_url": image_url
                                }))
                            } else {
                                // Unsupported file type
                                None
                            }
                        }
                    })
                    .collect();

                messages.push(json!({
                    "role": "user",
                    "content": content_parts
                }));
            }

            LanguageModelMessage::Assistant(asst_msg) => {
                for part in asst_msg.content {
                    match part {
                        LanguageModelAssistantMessagePart::Text(text_part) => {
                            messages.push(json!({
                                "role": "assistant",
                                "content": [json!({
                                    "type": "output_text",
                                    "text": text_part.text
                                })]
                            }));
                        }
                        LanguageModelAssistantMessagePart::ToolCall(_) => {
                            // Skip - handled by API
                        }
                        LanguageModelAssistantMessagePart::ToolResult(_) => {
                            // Skip - handled by API
                        }
                        LanguageModelAssistantMessagePart::File(_) => {
                            // Skip file content in assistant messages
                        }
                        LanguageModelAssistantMessagePart::Reasoning(reasoning_part) => {
                            // Include reasoning as output_text
                            messages.push(json!({
                                "role": "assistant",
                                "content": [json!({
                                    "type": "output_text",
                                    "text": reasoning_part.text
                                })]
                            }));
                        }
                    }
                }
            }

            LanguageModelMessage::Tool(_) => {
                // Tool messages are handled by the API itself
                warnings.push(LanguageModelCallWarning::UnsupportedSetting {
                    setting: "tool messages".to_string(),
                    details: Some("Tool messages are handled by the API itself and should not be included in the input".to_string()),
                });
            }
        }
    }

    Ok((messages, warnings))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_sdk_provider::language_model::prompt::message::{
        LanguageModelAssistantMessage, LanguageModelSystemMessage, LanguageModelToolMessage,
        LanguageModelUserMessage,
    };

    #[tokio::test]
    async fn test_convert_system_message() {
        let prompt = vec![LanguageModelMessage::System(
            LanguageModelSystemMessage::new("You are helpful"),
        )];

        let (messages, warnings) = convert_to_huggingface_responses_messages(prompt)
            .await
            .unwrap();

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0]["role"], "system");
        assert_eq!(messages[0]["content"], "You are helpful");
        assert!(warnings.is_empty());
    }

    #[tokio::test]
    async fn test_convert_user_message() {
        let prompt = vec![LanguageModelMessage::User(LanguageModelUserMessage::text(
            "Hello",
        ))];

        let (messages, warnings) = convert_to_huggingface_responses_messages(prompt)
            .await
            .unwrap();

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0]["role"], "user");
        assert_eq!(messages[0]["content"][0]["type"], "input_text");
        assert_eq!(messages[0]["content"][0]["text"], "Hello");
        assert!(warnings.is_empty());
    }

    #[tokio::test]
    async fn test_convert_assistant_message() {
        let prompt = vec![LanguageModelMessage::Assistant(
            LanguageModelAssistantMessage::text("Hi there"),
        )];

        let (messages, warnings) = convert_to_huggingface_responses_messages(prompt)
            .await
            .unwrap();

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0]["role"], "assistant");
        assert_eq!(messages[0]["content"][0]["type"], "output_text");
        assert_eq!(messages[0]["content"][0]["text"], "Hi there");
        assert!(warnings.is_empty());
    }

    #[tokio::test]
    async fn test_convert_image_url() {
        use ai_sdk_provider::language_model::prompt::message::LanguageModelFilePart;
        use url::Url;

        let file_part = LanguageModelFilePart::new(
            LanguageModelDataContent::Url(Url::parse("https://example.com/image.jpg").unwrap()),
            "image/jpeg",
        );

        let prompt = vec![LanguageModelMessage::User(LanguageModelUserMessage::new(
            vec![LanguageModelUserMessagePart::File(file_part)],
        ))];

        let (messages, _) = convert_to_huggingface_responses_messages(prompt)
            .await
            .unwrap();

        assert_eq!(messages[0]["content"][0]["type"], "input_image");
        assert_eq!(
            messages[0]["content"][0]["image_url"],
            "https://example.com/image.jpg"
        );
    }

    #[tokio::test]
    async fn test_convert_image_base64() {
        use ai_sdk_provider::language_model::prompt::message::LanguageModelFilePart;

        let file_part = LanguageModelFilePart::new(
            LanguageModelDataContent::Base64("abc123".to_string()),
            "image/png",
        );

        let prompt = vec![LanguageModelMessage::User(LanguageModelUserMessage::new(
            vec![LanguageModelUserMessagePart::File(file_part)],
        ))];

        let (messages, _) = convert_to_huggingface_responses_messages(prompt)
            .await
            .unwrap();

        assert_eq!(messages[0]["content"][0]["type"], "input_image");
        assert_eq!(
            messages[0]["content"][0]["image_url"],
            "data:image/png;base64,abc123"
        );
    }

    #[tokio::test]
    async fn test_convert_tool_message_warning() {
        use ai_sdk_provider::language_model::prompt::message::{
            LanguageModelToolResultOutput, LanguageModelToolResultPart,
        };

        let tool_result = LanguageModelToolResultPart::new(
            "call_123",
            "test",
            LanguageModelToolResultOutput::text("result"),
        );

        let prompt = vec![LanguageModelMessage::Tool(LanguageModelToolMessage::new(
            vec![tool_result],
        ))];

        let (messages, warnings) = convert_to_huggingface_responses_messages(prompt)
            .await
            .unwrap();

        assert_eq!(messages.len(), 0);
        assert_eq!(warnings.len(), 1);
        matches!(
            warnings[0],
            LanguageModelCallWarning::UnsupportedSetting { .. }
        );
    }
}
