use ai_sdk_provider::language_model::prompt::message::parts::{FilePart, TextPart, ToolCallPart};
use ai_sdk_provider::language_model::prompt::message::{Assistant, System, Tool, User};
use ai_sdk_provider::language_model::prompt::{
    AssistantMessagePart, DataContent, LanguageModelMessage, LanguageModelPrompt, ToolResultOutput, ToolResultPart,
    UserMessagePart,
};
use ai_sdk_provider::shared::provider_options::SharedProviderOptions;
use base64::{Engine as _, engine::general_purpose};
use serde_json::Value;

use crate::chat::api_types::{
    OpenAICompatibleAssistantMessage, OpenAICompatibleContentPart,
    OpenAICompatibleContentPartImage, OpenAICompatibleContentPartText, OpenAICompatibleMessage,
    OpenAICompatibleMessageToolCall, OpenAICompatibleSystemMessage, OpenAICompatibleToolMessage,
    OpenAICompatibleUserMessage, UserMessageContent,
};

/// Extracts OpenAI-compatible metadata from provider options
fn get_openai_metadata(provider_options: &Option<SharedProviderOptions>) -> Option<Value> {
    provider_options
        .as_ref()
        .and_then(|opts| opts.get("openaiCompatible"))
        .map(|metadata| serde_json::to_value(metadata).ok())
        .flatten()
}

/// Converts data content to a base64 or URL string for image URLs
fn convert_data_to_url(data: &DataContent, media_type: &str) -> String {
    match data {
        DataContent::Url(url) => url.to_string(),
        DataContent::Base64(base64) => {
            format!("data:{};base64,{}", media_type, base64)
        }
        DataContent::Bytes(bytes) => {
            let base64 = general_purpose::STANDARD.encode(bytes);
            format!("data:{};base64,{}", media_type, base64)
        }
    }
}

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
                let mut system_msg = OpenAICompatibleSystemMessage::new(sys_msg.content);
                if let Some(metadata) = get_openai_metadata(&sys_msg.provider_options) {
                    system_msg.additional_properties = Some(metadata);
                }
                messages.push(OpenAICompatibleMessage::System(system_msg));
            }

            LanguageModelMessage::User(user_msg) => {
                let content = user_msg.content;
                let provider_options = user_msg.provider_options;
                // Check if it's a simple text message
                if content.len() == 1 {
                    if let UserMessagePart::Text(text_part) = &content[0] {
                        let mut user_msg =
                            OpenAICompatibleUserMessage::new_text(text_part.text.clone());
                        if let Some(metadata) = get_openai_metadata(&text_part.provider_options) {
                            user_msg.additional_properties = Some(metadata);
                        }
                        messages.push(OpenAICompatibleMessage::User(user_msg));
                        continue;
                    }
                }

                // Multi-part message
                let mut parts: Vec<OpenAICompatibleContentPart> = Vec::new();

                for part in content {
                    match part {
                        UserMessagePart::Text(text_part) => {
                            let mut text_part_out =
                                OpenAICompatibleContentPartText::new(text_part.text);
                            if let Some(metadata) = get_openai_metadata(&text_part.provider_options)
                            {
                                text_part_out.additional_properties = Some(metadata);
                            }
                            parts.push(OpenAICompatibleContentPart::Text(text_part_out));
                        }
                        UserMessagePart::File(file_part) => {
                            if file_part.media_type.starts_with("image/") {
                                // Handle wildcard image type
                                let actual_media_type = if file_part.media_type == "image/*" {
                                    "image/jpeg"
                                } else {
                                    &file_part.media_type
                                };

                                let url = convert_data_to_url(&file_part.data, actual_media_type);
                                let mut image_part = OpenAICompatibleContentPartImage::new(url);
                                if let Some(metadata) =
                                    get_openai_metadata(&file_part.provider_options)
                                {
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

                let mut user_msg = OpenAICompatibleUserMessage::new_parts(parts);
                if let Some(metadata) = get_openai_metadata(&provider_options) {
                    user_msg.additional_properties = Some(metadata);
                }
                messages.push(OpenAICompatibleMessage::User(user_msg));
            }

            LanguageModelMessage::Assistant(asst_msg) => {
                let content = asst_msg.content;
                let provider_options = asst_msg.provider_options;
                let mut text = String::new();
                let mut tool_calls: Vec<OpenAICompatibleMessageToolCall> = Vec::new();

                for part in content {
                    match part {
                        AssistantMessagePart::Text(text_part) => {
                            text.push_str(&text_part.text);
                        }
                        AssistantMessagePart::ToolCall(tool_call_part) => {
                            let arguments = serde_json::to_string(&tool_call_part.input)
                                .unwrap_or_else(|_| "{}".to_string());
                            let mut tool_call = OpenAICompatibleMessageToolCall::new(
                                tool_call_part.tool_call_id,
                                tool_call_part.tool_name,
                                arguments,
                            );
                            if let Some(metadata) =
                                get_openai_metadata(&tool_call_part.provider_options)
                            {
                                tool_call.additional_properties = Some(metadata);
                            }
                            tool_calls.push(tool_call);
                        }
                        // Ignore other assistant content types (File, Reasoning, ToolResult)
                        _ => {}
                    }
                }

                let content_opt = if text.is_empty() { None } else { Some(text) };

                let tool_calls_opt = if tool_calls.is_empty() {
                    None
                } else {
                    Some(tool_calls)
                };

                let mut assistant_msg =
                    OpenAICompatibleAssistantMessage::new(content_opt, tool_calls_opt);
                if let Some(metadata) = get_openai_metadata(&provider_options) {
                    assistant_msg.additional_properties = Some(metadata);
                }
                messages.push(OpenAICompatibleMessage::Assistant(assistant_msg));
            }

            LanguageModelMessage::Tool(tool_msg) => {
                for tool_response in tool_msg.content {
                    let content_value = match &tool_response.output {
                        ToolResultOutput::Text { value } => value.clone(),
                        ToolResultOutput::ErrorText { value } => value.clone(),
                        ToolResultOutput::Json { value } => {
                            serde_json::to_string(&value).unwrap_or_else(|_| "{}".to_string())
                        }
                        ToolResultOutput::ErrorJson { value } => {
                            serde_json::to_string(&value).unwrap_or_else(|_| "{}".to_string())
                        }
                        ToolResultOutput::Content { value } => {
                            serde_json::to_string(&value).unwrap_or_else(|_| "[]".to_string())
                        }
                    };

                    let mut tool_message = OpenAICompatibleToolMessage::new(
                        content_value,
                        tool_response.tool_call_id.clone(),
                    );
                    if let Some(metadata) = get_openai_metadata(&tool_response.provider_options) {
                        tool_message.additional_properties = Some(metadata);
                    }
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
    use serde_json::json;

    #[test]
    fn test_convert_system_message() {
        let prompt = vec![LanguageModelMessage::System(System::new(
            "You are a helpful assistant.".to_string(),
        ))];

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
        let prompt = vec![LanguageModelMessage::User(User::new(vec![UserMessagePart::Text(
            TextPart::new("Hello!"),
        )]))];

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
        let prompt = vec![LanguageModelMessage::User(User::new(vec![
            UserMessagePart::Text(TextPart::new("What's in this image?")),
            UserMessagePart::File(FilePart::with_options(
                None,
                DataContent::Url("https://example.com/image.jpg".parse().unwrap()),
                "image/jpeg",
                None,
            )),
        ]))];

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
        let prompt = vec![LanguageModelMessage::Assistant(Assistant::new(vec![
            AssistantMessagePart::Text(TextPart::new("I can help you!")),
        ]))];

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
        let prompt = vec![LanguageModelMessage::Assistant(Assistant::new(vec![
            AssistantMessagePart::ToolCall(ToolCallPart::new(
                "call_123",
                "get_weather",
                json!({"city": "San Francisco"}),
            )),
        ]))];

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
        let prompt = vec![LanguageModelMessage::Tool(Tool::new(vec![ToolResultPart::new(
            "call_123",
            "get_weather",
            ToolResultOutput::Text {
                value: "Sunny, 72°F".to_string(),
            },
        )]))];

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
        let prompt = vec![LanguageModelMessage::User(User::new(vec![UserMessagePart::File(
            FilePart::new(
                DataContent::Base64("base64data".to_string()),
                "application/pdf",
            ),
        )]))];

        let result = convert_to_openai_compatible_chat_messages(prompt);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported file media type"));
    }

    #[test]
    fn test_convert_image_with_wildcard_type() {
        let prompt = vec![LanguageModelMessage::User(User::new(vec![UserMessagePart::File(
            FilePart::new(DataContent::Base64("imagedata".to_string()), "image/*"),
        )]))];

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
            LanguageModelMessage::System(System::new("System prompt".to_string())),
            LanguageModelMessage::User(User::new(vec![UserMessagePart::Text(TextPart::new(
                "User message",
            ))])),
            LanguageModelMessage::Assistant(Assistant::new(vec![AssistantMessagePart::Text(
                TextPart::new("Assistant message"),
            )])),
        ];

        let result = convert_to_openai_compatible_chat_messages(prompt).unwrap();

        assert_eq!(result.len(), 3);
    }
}
