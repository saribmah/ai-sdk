use ai_sdk_provider::language_model::call_warning::LanguageModelCallWarning;
use ai_sdk_provider::language_model::prompt::LanguageModelPrompt;
use ai_sdk_provider::language_model::prompt::message::{
    LanguageModelAssistantMessage, LanguageModelAssistantMessagePart, LanguageModelDataContent,
    LanguageModelMessage, LanguageModelToolMessage, LanguageModelUserMessage,
    LanguageModelUserMessagePart,
};
use serde::{Deserialize, Serialize};

/// xAI chat message format.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum XaiChatMessage {
    System {
        content: String,
    },
    User {
        content: XaiUserContent,
    },
    Assistant {
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        reasoning_content: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<XaiToolCall>>,
    },
    Tool {
        tool_call_id: String,
        content: String,
    },
}

/// User content can be either string or array of content parts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum XaiUserContent {
    Text(String),
    Parts(Vec<XaiContentPart>),
}

/// Content parts for multimodal messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum XaiContentPart {
    Text { text: String },
    ImageUrl { image_url: ImageUrl },
}

/// Image URL object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUrl {
    pub url: String,
}

/// Tool call in xAI format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XaiToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String,
    pub function: XaiFunction,
}

/// Function call details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XaiFunction {
    pub name: String,
    pub arguments: String,
}

/// Convert AI SDK prompt to xAI chat messages.
pub fn convert_to_xai_chat_messages(
    prompt: &LanguageModelPrompt,
) -> Result<(Vec<XaiChatMessage>, Vec<LanguageModelCallWarning>), String> {
    let mut messages = Vec::new();
    let mut warnings = Vec::new();

    for message in prompt {
        match message {
            LanguageModelMessage::System(system_msg) => {
                messages.push(XaiChatMessage::System {
                    content: system_msg.content.clone(),
                });
            }
            LanguageModelMessage::User(user_msg) => {
                messages.push(convert_user_message(user_msg, &mut warnings));
            }
            LanguageModelMessage::Assistant(assistant_msg) => {
                messages.push(convert_assistant_message(assistant_msg));
            }
            LanguageModelMessage::Tool(tool_msg) => {
                messages.extend(convert_tool_message(tool_msg));
            }
        }
    }

    Ok((messages, warnings))
}

fn convert_user_message(
    user_msg: &LanguageModelUserMessage,
    warnings: &mut Vec<LanguageModelCallWarning>,
) -> XaiChatMessage {
    // Check if simple text message
    if user_msg.content.len() == 1
        && let LanguageModelUserMessagePart::Text(text_part) = &user_msg.content[0]
    {
        return XaiChatMessage::User {
            content: XaiUserContent::Text(text_part.text.clone()),
        };
    }

    // Multimodal message
    let mut parts = Vec::new();
    for part in &user_msg.content {
        match part {
            LanguageModelUserMessagePart::Text(text_part) => {
                parts.push(XaiContentPart::Text {
                    text: text_part.text.clone(),
                });
            }
            LanguageModelUserMessagePart::File(file_part) => {
                // Handle file parts (images)
                if file_part.media_type.starts_with("image/") {
                    let url = match &file_part.data {
                        LanguageModelDataContent::Url(url) => url.to_string(),
                        LanguageModelDataContent::Base64(base64) => {
                            let media_type = if file_part.media_type == "image/*" {
                                "image/jpeg"
                            } else {
                                &file_part.media_type
                            };
                            format!("data:{};base64,{}", media_type, base64)
                        }
                        LanguageModelDataContent::Bytes(bytes) => {
                            // Convert bytes to base64
                            use base64::{Engine as _, engine::general_purpose};
                            let base64_str = general_purpose::STANDARD.encode(bytes);
                            let media_type = if file_part.media_type == "image/*" {
                                "image/jpeg"
                            } else {
                                &file_part.media_type
                            };
                            format!("data:{};base64,{}", media_type, base64_str)
                        }
                    };
                    parts.push(XaiContentPart::ImageUrl {
                        image_url: ImageUrl { url },
                    });
                } else {
                    warnings.push(LanguageModelCallWarning::other(format!(
                        "File media type {} not supported",
                        file_part.media_type
                    )));
                }
            }
        }
    }

    XaiChatMessage::User {
        content: XaiUserContent::Parts(parts),
    }
}

fn convert_assistant_message(assistant_msg: &LanguageModelAssistantMessage) -> XaiChatMessage {
    let mut text = String::new();
    let mut reasoning_content = None;
    let mut tool_calls = Vec::new();

    for part in &assistant_msg.content {
        match part {
            LanguageModelAssistantMessagePart::Text(text_part) => {
                text.push_str(&text_part.text);
            }
            LanguageModelAssistantMessagePart::Reasoning(reasoning_part) => {
                reasoning_content = Some(reasoning_part.text.clone());
            }
            LanguageModelAssistantMessagePart::ToolCall(tool_call_part) => {
                tool_calls.push(XaiToolCall {
                    id: tool_call_part.tool_call_id.clone(),
                    call_type: "function".to_string(),
                    function: XaiFunction {
                        name: tool_call_part.tool_name.clone(),
                        arguments: serde_json::to_string(&tool_call_part.input).unwrap_or_default(),
                    },
                });
            }
            _ => {}
        }
    }

    XaiChatMessage::Assistant {
        content: text,
        reasoning_content,
        tool_calls: if tool_calls.is_empty() {
            None
        } else {
            Some(tool_calls)
        },
    }
}

fn convert_tool_message(tool_msg: &LanguageModelToolMessage) -> Vec<XaiChatMessage> {
    tool_msg
        .content
        .iter()
        .map(|tool_result| {
            let content_value = match &tool_result.output {
                ai_sdk_provider::language_model::prompt::LanguageModelToolResultOutput::Text { value } => {
                    value.clone()
                }
                ai_sdk_provider::language_model::prompt::LanguageModelToolResultOutput::ErrorText { value } => {
                    value.clone()
                }
                ai_sdk_provider::language_model::prompt::LanguageModelToolResultOutput::Content { value } => {
                    serde_json::to_string(value).unwrap_or_default()
                }
                ai_sdk_provider::language_model::prompt::LanguageModelToolResultOutput::Json { value } => {
                    serde_json::to_string(value).unwrap_or_default()
                }
                ai_sdk_provider::language_model::prompt::LanguageModelToolResultOutput::ErrorJson { value } => {
                    serde_json::to_string(value).unwrap_or_default()
                }
            };

            XaiChatMessage::Tool {
                tool_call_id: tool_result.tool_call_id.clone(),
                content: content_value,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_message_serialization() {
        let msg = XaiChatMessage::System {
            content: "You are a helpful assistant.".to_string(),
        };

        let json = serde_json::to_value(&msg).unwrap();
        assert_eq!(json["role"], "system");
        assert_eq!(json["content"], "You are a helpful assistant.");
    }

    #[test]
    fn test_user_text_message() {
        let msg = XaiChatMessage::User {
            content: XaiUserContent::Text("Hello".to_string()),
        };

        let json = serde_json::to_value(&msg).unwrap();
        assert_eq!(json["role"], "user");
        assert_eq!(json["content"], "Hello");
    }

    #[test]
    fn test_assistant_message_with_tool_calls() {
        let msg = XaiChatMessage::Assistant {
            content: "Let me search for that.".to_string(),
            reasoning_content: None,
            tool_calls: Some(vec![XaiToolCall {
                id: "call_123".to_string(),
                call_type: "function".to_string(),
                function: XaiFunction {
                    name: "search".to_string(),
                    arguments: r#"{"query":"test"}"#.to_string(),
                },
            }]),
        };

        let json = serde_json::to_value(&msg).unwrap();
        assert_eq!(json["role"], "assistant");
        assert!(json["tool_calls"].is_array());
    }

    #[test]
    fn test_tool_message() {
        let msg = XaiChatMessage::Tool {
            tool_call_id: "call_123".to_string(),
            content: "Search results here".to_string(),
        };

        let json = serde_json::to_value(&msg).unwrap();
        assert_eq!(json["role"], "tool");
        assert_eq!(json["tool_call_id"], "call_123");
    }

    #[test]
    fn test_convert_system_message() {
        use ai_sdk_provider::language_model::prompt::message::LanguageModelSystemMessage;

        let prompt = vec![LanguageModelMessage::System(
            LanguageModelSystemMessage::new("You are helpful"),
        )];

        let (messages, warnings) = convert_to_xai_chat_messages(&prompt).unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(warnings.len(), 0);

        match &messages[0] {
            XaiChatMessage::System { content } => {
                assert_eq!(content, "You are helpful");
            }
            _ => panic!("Expected system message"),
        }
    }

    #[test]
    fn test_convert_user_text_message() {
        let prompt = vec![LanguageModelMessage::User(LanguageModelUserMessage::text(
            "Hello",
        ))];

        let (messages, warnings) = convert_to_xai_chat_messages(&prompt).unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(warnings.len(), 0);

        match &messages[0] {
            XaiChatMessage::User { content } => match content {
                XaiUserContent::Text(text) => assert_eq!(text, "Hello"),
                _ => panic!("Expected text content"),
            },
            _ => panic!("Expected user message"),
        }
    }
}
