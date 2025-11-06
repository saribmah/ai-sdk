use ai_sdk_provider::language_model::prompt::{LanguageModelMessage, LanguageModelPrompt};

use crate::chat::prompt::message::{
    OpenAICompatibleAssistantMessage, OpenAICompatibleMessage, OpenAICompatibleSystemMessage,
    OpenAICompatibleToolMessage, OpenAICompatibleUserMessage,
};

/// Converts a provider prompt to OpenAI-compatible chat messages.
///
/// # Arguments
///
/// * `prompt` - The provider prompt (vector of messages) to convert
///
/// # Returns
///
/// A vector of OpenAI-compatible messages
///
/// # Errors
///
/// Returns an error if:
/// - A file part has an unsupported media type (non-image files)
/// - An unsupported message role or content type is encountered
pub fn convert_to_openai_compatible_chat_messages(
    prompt: LanguageModelPrompt,
) -> Result<Vec<OpenAICompatibleMessage>, String> {
    let mut messages: Vec<OpenAICompatibleMessage> = Vec::new();

    for message in prompt {
        match message {
            LanguageModelMessage::System(sys_msg) => {
                let system_msg = OpenAICompatibleSystemMessage::from_provider(sys_msg);
                messages.push(OpenAICompatibleMessage::System(system_msg));
            }

            LanguageModelMessage::User(user_msg) => {
                let user_msg = OpenAICompatibleUserMessage::from_provider(user_msg)?;
                messages.push(OpenAICompatibleMessage::User(user_msg));
            }

            LanguageModelMessage::Assistant(asst_msg) => {
                let assistant_msg = OpenAICompatibleAssistantMessage::from_provider(asst_msg);
                messages.push(OpenAICompatibleMessage::Assistant(assistant_msg));
            }

            LanguageModelMessage::Tool(tool_msg) => {
                let tool_messages = OpenAICompatibleToolMessage::from_provider(tool_msg);
                for tool_message in tool_messages {
                    messages.push(OpenAICompatibleMessage::Tool(tool_message));
                }
            }
        }
    }

    Ok(messages)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chat::prompt::message::{OpenAICompatibleContentPart, UserMessageContent};
    use ai_sdk_provider::language_model::prompt::message::parts::{
        LanguageModelFilePart, LanguageModelTextPart, LanguageModelToolCallPart,
    };
    use ai_sdk_provider::language_model::prompt::message::{
        LanguageModelAssistantMessage, LanguageModelSystemMessage, LanguageModelToolMessage,
        LanguageModelUserMessage,
    };
    use ai_sdk_provider::language_model::prompt::{
        LanguageModelAssistantMessagePart, LanguageModelDataContent, LanguageModelToolResultOutput,
        LanguageModelToolResultPart, LanguageModelUserMessagePart,
    };
    use serde_json::json;

    #[test]
    fn test_convert_system_message() {
        let prompt = vec![LanguageModelMessage::System(
            LanguageModelSystemMessage::new("You are a helpful assistant.".to_string()),
        )];

        let result = convert_to_openai_compatible_chat_messages(prompt).unwrap();

        assert_eq!(result.len(), 1);
        match &result[0] {
            OpenAICompatibleMessage::System(msg) => {
                assert_eq!(msg.content, "You are a helpful assistant.");
            }
            _ => panic!("Expected system message"),
        }
    }

    #[test]
    fn test_convert_user_text_message() {
        let prompt = vec![LanguageModelMessage::User(LanguageModelUserMessage::new(
            vec![LanguageModelUserMessagePart::Text(
                LanguageModelTextPart::new("Hello!"),
            )],
        ))];

        let result = convert_to_openai_compatible_chat_messages(prompt).unwrap();

        assert_eq!(result.len(), 1);
        match &result[0] {
            OpenAICompatibleMessage::User(msg) => match &msg.content {
                Some(UserMessageContent::Text(text)) => assert_eq!(text, "Hello!"),
                _ => panic!("Expected text content"),
            },
            _ => panic!("Expected user message"),
        }
    }

    #[test]
    fn test_convert_user_multipart_message() {
        let prompt = vec![LanguageModelMessage::User(LanguageModelUserMessage::new(
            vec![
                LanguageModelUserMessagePart::Text(LanguageModelTextPart::new(
                    "What's in this image?",
                )),
                LanguageModelUserMessagePart::File(LanguageModelFilePart::with_options(
                    None,
                    LanguageModelDataContent::Url("https://example.com/image.jpg".parse().unwrap()),
                    "image/jpeg",
                    None,
                )),
            ],
        ))];

        let result = convert_to_openai_compatible_chat_messages(prompt).unwrap();

        assert_eq!(result.len(), 1);
        match &result[0] {
            OpenAICompatibleMessage::User(msg) => match &msg.content {
                Some(UserMessageContent::Parts(parts)) => {
                    assert_eq!(parts.len(), 2);
                }
                _ => panic!("Expected parts content"),
            },
            _ => panic!("Expected user message"),
        }
    }

    #[test]
    fn test_convert_assistant_text_message() {
        let prompt = vec![LanguageModelMessage::Assistant(
            LanguageModelAssistantMessage::new(vec![LanguageModelAssistantMessagePart::Text(
                LanguageModelTextPart::new("I can help you!"),
            )]),
        )];

        let result = convert_to_openai_compatible_chat_messages(prompt).unwrap();

        assert_eq!(result.len(), 1);
        match &result[0] {
            OpenAICompatibleMessage::Assistant(msg) => {
                assert_eq!(msg.content, Some("I can help you!".to_string()));
                assert!(msg.tool_calls.is_none());
            }
            _ => panic!("Expected assistant message"),
        }
    }

    #[test]
    fn test_convert_assistant_with_tool_calls() {
        let prompt = vec![LanguageModelMessage::Assistant(
            LanguageModelAssistantMessage::new(vec![LanguageModelAssistantMessagePart::ToolCall(
                LanguageModelToolCallPart::new(
                    "call_123",
                    "get_weather",
                    json!({"city": "San Francisco"}),
                ),
            )]),
        )];

        let result = convert_to_openai_compatible_chat_messages(prompt).unwrap();

        assert_eq!(result.len(), 1);
        match &result[0] {
            OpenAICompatibleMessage::Assistant(msg) => {
                assert_eq!(msg.content, None);
                assert!(msg.tool_calls.is_some());
                let tool_calls = msg.tool_calls.as_ref().unwrap();
                assert_eq!(tool_calls.len(), 1);
                assert_eq!(tool_calls[0].id, "call_123");
                assert_eq!(tool_calls[0].function.name, "get_weather");
            }
            _ => panic!("Expected assistant message"),
        }
    }

    #[test]
    fn test_convert_tool_message() {
        let prompt = vec![LanguageModelMessage::Tool(LanguageModelToolMessage::new(
            vec![LanguageModelToolResultPart::new(
                "call_123",
                "get_weather",
                LanguageModelToolResultOutput::Text {
                    value: "Sunny, 72°F".to_string(),
                },
            )],
        ))];

        let result = convert_to_openai_compatible_chat_messages(prompt).unwrap();

        assert_eq!(result.len(), 1);
        match &result[0] {
            OpenAICompatibleMessage::Tool(msg) => {
                assert_eq!(msg.content, "Sunny, 72°F");
                assert_eq!(msg.tool_call_id, "call_123");
            }
            _ => panic!("Expected tool message"),
        }
    }

    #[test]
    fn test_convert_unsupported_file_type() {
        let prompt = vec![LanguageModelMessage::User(LanguageModelUserMessage::new(
            vec![LanguageModelUserMessagePart::File(
                LanguageModelFilePart::new(
                    LanguageModelDataContent::Base64("base64data".to_string()),
                    "application/pdf",
                ),
            )],
        ))];

        let result = convert_to_openai_compatible_chat_messages(prompt);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported file media type"));
    }

    #[test]
    fn test_convert_image_with_wildcard_type() {
        let prompt = vec![LanguageModelMessage::User(LanguageModelUserMessage::new(
            vec![LanguageModelUserMessagePart::File(
                LanguageModelFilePart::new(
                    LanguageModelDataContent::Base64("imagedata".to_string()),
                    "image/*",
                ),
            )],
        ))];

        let result = convert_to_openai_compatible_chat_messages(prompt).unwrap();

        assert_eq!(result.len(), 1);
        match &result[0] {
            OpenAICompatibleMessage::User(msg) => match &msg.content {
                Some(UserMessageContent::Parts(parts)) => {
                    assert_eq!(parts.len(), 1);
                    match &parts[0] {
                        OpenAICompatibleContentPart::ImageUrl(img) => {
                            assert!(img.image_url.url.contains("data:image/jpeg;base64,"));
                        }
                        _ => panic!("Expected image URL part"),
                    }
                }
                _ => panic!("Expected parts content"),
            },
            _ => panic!("Expected user message"),
        }
    }

    #[test]
    fn test_convert_multiple_messages() {
        let prompt = vec![
            LanguageModelMessage::System(LanguageModelSystemMessage::new(
                "System prompt".to_string(),
            )),
            LanguageModelMessage::User(LanguageModelUserMessage::new(vec![
                LanguageModelUserMessagePart::Text(LanguageModelTextPart::new("User message")),
            ])),
            LanguageModelMessage::Assistant(LanguageModelAssistantMessage::new(vec![
                LanguageModelAssistantMessagePart::Text(LanguageModelTextPart::new(
                    "Assistant message",
                )),
            ])),
        ];

        let result = convert_to_openai_compatible_chat_messages(prompt).unwrap();

        assert_eq!(result.len(), 3);
    }
}
