//! OpenAI Chat Prompt Conversion
//!
//! Converts SDK prompt messages to OpenAI chat API format.

use ai_sdk_provider::language_model::call_warning::LanguageModelCallWarning;
use ai_sdk_provider::language_model::prompt::LanguageModelPrompt;
use ai_sdk_provider::language_model::prompt::message::{
    LanguageModelAssistantMessage, LanguageModelAssistantMessagePart, LanguageModelMessage,
    LanguageModelSystemMessage, LanguageModelToolMessage, LanguageModelUserMessage,
    LanguageModelUserMessagePart,
};
use base64::Engine;
use serde_json::{Value as JsonValue, json};

/// OpenAI chat prompt is an array of messages.
pub type OpenAIChatPrompt = Vec<JsonValue>;

/// System message handling mode.
#[derive(Debug, Clone, Copy)]
pub enum SystemMessageMode {
    /// Use "system" role.
    System,
    /// Use "developer" role (for reasoning models).
    Developer,
    /// Remove system messages (not supported).
    Remove,
}

/// Convert SDK prompt to OpenAI chat messages.
///
/// # Arguments
///
/// * `prompt` - The SDK prompt to convert
/// * `system_message_mode` - How to handle system messages
///
/// # Returns
///
/// The converted OpenAI messages and any warnings
pub fn convert_to_openai_chat_messages(
    prompt: &LanguageModelPrompt,
    system_message_mode: SystemMessageMode,
) -> (OpenAIChatPrompt, Vec<LanguageModelCallWarning>) {
    let mut messages = Vec::new();
    let mut warnings = Vec::new();

    for message in prompt {
        match message {
            LanguageModelMessage::System(system_msg) => {
                convert_system_message(
                    system_msg,
                    system_message_mode,
                    &mut messages,
                    &mut warnings,
                );
            }
            LanguageModelMessage::User(user_msg) => {
                convert_user_message(user_msg, &mut messages, &mut warnings);
            }
            LanguageModelMessage::Assistant(assistant_msg) => {
                convert_assistant_message(assistant_msg, &mut messages);
            }
            LanguageModelMessage::Tool(tool_msg) => {
                convert_tool_message(tool_msg, &mut messages);
            }
        }
    }

    (messages, warnings)
}

fn convert_system_message(
    msg: &LanguageModelSystemMessage,
    mode: SystemMessageMode,
    messages: &mut OpenAIChatPrompt,
    warnings: &mut Vec<LanguageModelCallWarning>,
) {
    match mode {
        SystemMessageMode::System => {
            messages.push(json!({
                "role": "system",
                "content": msg.content
            }));
        }
        SystemMessageMode::Developer => {
            messages.push(json!({
                "role": "developer",
                "content": msg.content
            }));
        }
        SystemMessageMode::Remove => {
            warnings.push(LanguageModelCallWarning::Other {
                message: "system messages are removed for this model".to_string(),
            });
        }
    }
}

fn convert_user_message(
    msg: &LanguageModelUserMessage,
    messages: &mut OpenAIChatPrompt,
    warnings: &mut Vec<LanguageModelCallWarning>,
) {
    // If there's only one text part, use simple string format
    if msg.content.len() == 1
        && let LanguageModelUserMessagePart::Text(text_part) = &msg.content[0]
    {
        messages.push(json!({
            "role": "user",
            "content": text_part.text
        }));
        return;
    }

    // Otherwise, use array of content parts
    let mut content_parts = Vec::new();

    for (index, part) in msg.content.iter().enumerate() {
        match part {
            LanguageModelUserMessagePart::Text(text_part) => {
                content_parts.push(json!({
                    "type": "text",
                    "text": text_part.text
                }));
            }
            LanguageModelUserMessagePart::File(file_part) => {
                let media_type = &file_part.media_type;

                if media_type.starts_with("image/") {
                    // Handle images
                    let actual_media_type = if media_type == "image/*" {
                        "image/jpeg"
                    } else {
                        media_type.as_str()
                    };

                    let url = match &file_part.data {
                        ai_sdk_provider::language_model::prompt::message::LanguageModelDataContent::Url(url) => {
                            url.to_string()
                        }
                        ai_sdk_provider::language_model::prompt::message::LanguageModelDataContent::Base64(base64_str) => {
                            format!("data:{};base64,{}", actual_media_type, base64_str)
                        }
                        ai_sdk_provider::language_model::prompt::message::LanguageModelDataContent::Bytes(bytes) => {
                            let base64_str = base64::engine::general_purpose::STANDARD.encode(bytes);
                            format!("data:{};base64,{}", actual_media_type, base64_str)
                        }
                    };

                    content_parts.push(json!({
                        "type": "image_url",
                        "image_url": {
                            "url": url
                        }
                    }));
                } else if media_type.starts_with("audio/") {
                    // Handle audio
                    match &file_part.data {
                        ai_sdk_provider::language_model::prompt::message::LanguageModelDataContent::Url(_) => {
                            warnings.push(LanguageModelCallWarning::other(
                                "audio file parts with URLs are not supported"
                            ));
                        }
                        ai_sdk_provider::language_model::prompt::message::LanguageModelDataContent::Base64(base64_str) => {
                            let format = match media_type.as_str() {
                                "audio/wav" => "wav",
                                "audio/mp3" | "audio/mpeg" => "mp3",
                                _ => {
                                    warnings.push(LanguageModelCallWarning::other(
                                        format!("audio content parts with media type {} are not supported", media_type)
                                    ));
                                    continue;
                                }
                            };

                            content_parts.push(json!({
                                "type": "input_audio",
                                "input_audio": {
                                    "data": base64_str,
                                    "format": format
                                }
                            }));
                        }
                        ai_sdk_provider::language_model::prompt::message::LanguageModelDataContent::Bytes(bytes) => {
                            let base64_str = base64::engine::general_purpose::STANDARD.encode(bytes);
                            let format = match media_type.as_str() {
                                "audio/wav" => "wav",
                                "audio/mp3" | "audio/mpeg" => "mp3",
                                _ => {
                                    warnings.push(LanguageModelCallWarning::other(
                                        format!("audio content parts with media type {} are not supported", media_type)
                                    ));
                                    continue;
                                }
                            };

                            content_parts.push(json!({
                                "type": "input_audio",
                                "input_audio": {
                                    "data": base64_str,
                                    "format": format
                                }
                            }));
                        }
                    }
                } else if media_type == "application/pdf" {
                    // Handle PDF files
                    match &file_part.data {
                        ai_sdk_provider::language_model::prompt::message::LanguageModelDataContent::Url(_) => {
                            warnings.push(LanguageModelCallWarning::other(
                                "PDF file parts with URLs are not supported"
                            ));
                        }
                        ai_sdk_provider::language_model::prompt::message::LanguageModelDataContent::Base64(base64_str) => {
                            // Check if it's a file ID
                            if base64_str.starts_with("file-") {
                                content_parts.push(json!({
                                    "type": "file",
                                    "file": {
                                        "file_id": base64_str
                                    }
                                }));
                            } else {
                                let filename = file_part.filename.clone()
                                    .unwrap_or_else(|| format!("part-{}.pdf", index));
                                content_parts.push(json!({
                                    "type": "file",
                                    "file": {
                                        "filename": filename,
                                        "file_data": format!("data:application/pdf;base64,{}", base64_str)
                                    }
                                }));
                            }
                        }
                        ai_sdk_provider::language_model::prompt::message::LanguageModelDataContent::Bytes(bytes) => {
                            let base64_str = base64::engine::general_purpose::STANDARD.encode(bytes);
                            let filename = file_part.filename.clone()
                                .unwrap_or_else(|| format!("part-{}.pdf", index));
                            content_parts.push(json!({
                                "type": "file",
                                "file": {
                                    "filename": filename,
                                    "file_data": format!("data:application/pdf;base64,{}", base64_str)
                                }
                            }));
                        }
                    }
                } else {
                    warnings.push(LanguageModelCallWarning::other(format!(
                        "file part media type {} is not supported",
                        media_type
                    )));
                }
            }
        }
    }

    messages.push(json!({
        "role": "user",
        "content": content_parts
    }));
}

fn convert_assistant_message(msg: &LanguageModelAssistantMessage, messages: &mut OpenAIChatPrompt) {
    let mut text = String::new();
    let mut tool_calls = Vec::new();

    for part in &msg.content {
        match part {
            LanguageModelAssistantMessagePart::Text(text_part) => {
                text.push_str(&text_part.text);
            }
            LanguageModelAssistantMessagePart::ToolCall(tool_call_part) => {
                tool_calls.push(json!({
                    "id": tool_call_part.tool_call_id,
                    "type": "function",
                    "function": {
                        "name": tool_call_part.tool_name,
                        "arguments": serde_json::to_string(&tool_call_part.input).unwrap_or_default()
                    }
                }));
            }
            _ => {
                // Skip other types (reasoning, file, tool result)
            }
        }
    }

    let mut message = json!({
        "role": "assistant",
        "content": text
    });

    if !tool_calls.is_empty() {
        message["tool_calls"] = json!(tool_calls);
    }

    messages.push(message);
}

fn convert_tool_message(msg: &LanguageModelToolMessage, messages: &mut OpenAIChatPrompt) {
    for tool_response in &msg.content {
        let content_value = match &tool_response.output {
            ai_sdk_provider::language_model::prompt::message::LanguageModelToolResultOutput::Text { value } => {
                value.clone()
            }
            ai_sdk_provider::language_model::prompt::message::LanguageModelToolResultOutput::ErrorText { value } => {
                value.clone()
            }
            ai_sdk_provider::language_model::prompt::message::LanguageModelToolResultOutput::Content { value } => {
                serde_json::to_string(value).unwrap_or_default()
            }
            ai_sdk_provider::language_model::prompt::message::LanguageModelToolResultOutput::Json { value } => {
                serde_json::to_string(value).unwrap_or_default()
            }
            ai_sdk_provider::language_model::prompt::message::LanguageModelToolResultOutput::ErrorJson { value } => {
                serde_json::to_string(value).unwrap_or_default()
            }
        };

        messages.push(json!({
            "role": "tool",
            "tool_call_id": tool_response.tool_call_id,
            "content": content_value
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_simple_system_message() {
        let prompt = vec![LanguageModelMessage::system("You are a helpful assistant")];
        let (messages, warnings) =
            convert_to_openai_chat_messages(&prompt, SystemMessageMode::System);

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0]["role"], "system");
        assert_eq!(messages[0]["content"], "You are a helpful assistant");
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_convert_simple_user_message() {
        let prompt = vec![LanguageModelMessage::user_text("Hello")];
        let (messages, warnings) =
            convert_to_openai_chat_messages(&prompt, SystemMessageMode::System);

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0]["role"], "user");
        assert_eq!(messages[0]["content"], "Hello");
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_system_message_developer_mode() {
        let prompt = vec![LanguageModelMessage::system("You are helpful")];
        let (messages, _) = convert_to_openai_chat_messages(&prompt, SystemMessageMode::Developer);

        assert_eq!(messages[0]["role"], "developer");
    }

    #[test]
    fn test_system_message_remove_mode() {
        let prompt = vec![LanguageModelMessage::system("You are helpful")];
        let (messages, warnings) =
            convert_to_openai_chat_messages(&prompt, SystemMessageMode::Remove);

        assert_eq!(messages.len(), 0);
        assert_eq!(warnings.len(), 1);
    }
}
