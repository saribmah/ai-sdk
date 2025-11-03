use crate::error::AISDKError;
use crate::prompt::message::{
    AssistantContent, AssistantContentPart, Message, ToolContentPart, UserContent, UserContentPart,
    content_parts::{
        FilePart, FileSource, ImagePart, ImageSource, ReasoningPart, TextPart, ToolCallPart,
        ToolResultPart,
    },
};
use crate::prompt::standardize::StandardizedPrompt;
use ai_sdk_provider::language_model::prompt::message::parts::{
    LanguageModelFilePart, LanguageModelReasoningPart, LanguageModelTextPart,
    LanguageModelToolCallPart,
};
use ai_sdk_provider::language_model::prompt::message::{
    LanguageModelAssistantMessage, LanguageModelSystemMessage, LanguageModelToolMessage,
    LanguageModelUserMessage,
};
use ai_sdk_provider::language_model::prompt::{
    LanguageModelAssistantMessagePart, LanguageModelDataContent, LanguageModelMessage,
    LanguageModelToolResultOutput, LanguageModelToolResultPart, LanguageModelUserMessagePart,
};

/// Convert a `StandardizedPrompt` to a provider `Prompt` (Vec<Message>).
///
/// This function converts the user-facing message types to the provider-level message format.
/// It also combines consecutive tool messages into a single tool message as an optimization.
///
/// # Arguments
///
/// * `prompt` - The standardized prompt to convert
///
/// # Returns
///
/// A `Vec<Message>` suitable for passing to a language model provider.
///
/// # Errors
///
/// Returns `AISDKError::InvalidPrompt` if the conversion fails.
pub fn convert_to_language_model_prompt(
    prompt: StandardizedPrompt,
) -> Result<Vec<LanguageModelMessage>, AISDKError> {
    // Start with optional system message
    let mut messages: Vec<LanguageModelMessage> = Vec::new();

    if let Some(system_content) = prompt.system {
        messages.push(LanguageModelMessage::System(
            LanguageModelSystemMessage::new(system_content),
        ));
    }

    // Convert all messages
    for message in prompt.messages {
        let converted = convert_to_language_model_message(message)?;
        messages.push(converted);
    }

    // Combine consecutive tool messages into a single tool message
    let combined_messages = combine_consecutive_tool_messages(messages);

    Ok(combined_messages)
}

/// Convert a single `ModelMessage` to a provider `Message`.
fn convert_to_language_model_message(message: Message) -> Result<LanguageModelMessage, AISDKError> {
    match message {
        Message::System(sys_msg) => Ok(LanguageModelMessage::System(
            LanguageModelSystemMessage::with_options(sys_msg.content, sys_msg.provider_options),
        )),

        Message::User(user_msg) => {
            let provider_options = user_msg.provider_options;
            let content = match user_msg.content {
                UserContent::Text(text) => {
                    vec![LanguageModelUserMessagePart::Text(
                        LanguageModelTextPart::new(text),
                    )]
                }
                UserContent::Parts(parts) => parts
                    .into_iter()
                    .map(convert_user_content_part)
                    .collect::<Result<Vec<_>, _>>()?
                    // Filter out empty text parts
                    .into_iter()
                    .filter(|part| match part {
                        LanguageModelUserMessagePart::Text(tp) => !tp.text.is_empty(),
                        _ => true,
                    })
                    .collect(),
            };

            Ok(LanguageModelMessage::User(
                LanguageModelUserMessage::with_options(content, provider_options),
            ))
        }

        Message::Assistant(asst_msg) => {
            let provider_options = asst_msg.provider_options;
            let content = match asst_msg.content {
                AssistantContent::Text(text) => {
                    vec![LanguageModelAssistantMessagePart::Text(
                        LanguageModelTextPart::new(text),
                    )]
                }
                AssistantContent::Parts(parts) => parts
                    .into_iter()
                    .filter_map(|part| {
                        // Filter out tool approval requests (not supported in provider)
                        match &part {
                            AssistantContentPart::ToolApprovalRequest(_) => None,
                            _ => Some(part),
                        }
                    })
                    .filter(|part| {
                        // Filter out empty text parts without provider options
                        match part {
                            AssistantContentPart::Text(TextPart {
                                text,
                                provider_options,
                            }) => !text.is_empty() || provider_options.is_some(),
                            _ => true,
                        }
                    })
                    .map(convert_assistant_content_part)
                    .collect::<Result<Vec<_>, _>>()?,
            };

            Ok(LanguageModelMessage::Assistant(
                LanguageModelAssistantMessage::with_options(content, provider_options),
            ))
        }

        Message::Tool(tool_msg) => {
            let provider_options = tool_msg.provider_options;
            let content = tool_msg
                .content
                .into_iter()
                .filter_map(|part| match part {
                    ToolContentPart::ToolResult(tr) => Some(tr),
                    // Filter out tool approval responses (not supported in provider)
                    _ => None,
                })
                .map(|tool_result| convert_tool_result_to_provider(tool_result))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(LanguageModelMessage::Tool(
                LanguageModelToolMessage::with_options(content, provider_options),
            ))
        }
    }
}

/// Convert a user content part to a provider `UserMessagePart`.
fn convert_user_content_part(
    part: UserContentPart,
) -> Result<LanguageModelUserMessagePart, AISDKError> {
    match part {
        UserContentPart::Text(TextPart {
            text,
            provider_options,
        }) => Ok(LanguageModelUserMessagePart::Text(
            LanguageModelTextPart::with_options(text, provider_options),
        )),

        UserContentPart::Image(ImagePart {
            image,
            media_type,
            provider_options,
        }) => {
            let (data, detected_media_type) = convert_image_source_to_data_content(image)?;
            let final_media_type = detected_media_type
                .or(media_type)
                .unwrap_or_else(|| "image/*".to_string());

            Ok(LanguageModelUserMessagePart::File(
                LanguageModelFilePart::with_options(None, data, final_media_type, provider_options),
            ))
        }

        UserContentPart::File(FilePart {
            data,
            media_type,
            filename,
            provider_options,
        }) => {
            let (data, _) = convert_file_source_to_data_content(data)?;

            Ok(LanguageModelUserMessagePart::File(
                LanguageModelFilePart::with_options(filename, data, media_type, provider_options),
            ))
        }
    }
}

/// Convert an assistant content part to a provider `AssistantMessagePart`.
fn convert_assistant_content_part(
    part: AssistantContentPart,
) -> Result<LanguageModelAssistantMessagePart, AISDKError> {
    match part {
        AssistantContentPart::Text(TextPart {
            text,
            provider_options,
        }) => Ok(LanguageModelAssistantMessagePart::Text(
            LanguageModelTextPart::with_options(text, provider_options),
        )),

        AssistantContentPart::File(FilePart {
            data,
            media_type,
            filename,
            provider_options,
        }) => {
            let (data, _) = convert_file_source_to_data_content(data)?;

            Ok(LanguageModelAssistantMessagePart::File(
                LanguageModelFilePart::with_options(filename, data, media_type, provider_options),
            ))
        }

        AssistantContentPart::Reasoning(ReasoningPart {
            text,
            provider_options,
        }) => Ok(LanguageModelAssistantMessagePart::Reasoning(
            LanguageModelReasoningPart::with_options(text, provider_options),
        )),

        AssistantContentPart::ToolCall(ToolCallPart {
            tool_call_id,
            tool_name,
            input,
            provider_executed,
            provider_options,
        }) => Ok(LanguageModelAssistantMessagePart::ToolCall(
            LanguageModelToolCallPart::with_options(
                tool_call_id,
                tool_name,
                input,
                provider_executed,
                provider_options,
            ),
        )),

        AssistantContentPart::ToolResult(tool_result) => {
            use ai_sdk_provider::language_model::prompt::message::parts::LanguageModelToolResultPart;
            Ok(LanguageModelAssistantMessagePart::ToolResult(
                LanguageModelToolResultPart::with_options(
                    tool_result.tool_call_id,
                    tool_result.tool_name,
                    convert_tool_result_output(tool_result.output)?,
                    tool_result.provider_options,
                ),
            ))
        }

        AssistantContentPart::ToolApprovalRequest(_) => {
            // This should have been filtered out earlier
            Err(AISDKError::invalid_prompt(
                "Tool approval requests are not supported in provider messages",
            ))
        }
    }
}

/// Convert a tool result part to a provider `ToolResultPart`.
fn convert_tool_result_to_provider(
    tool_result: ToolResultPart,
) -> Result<LanguageModelToolResultPart, AISDKError> {
    let mut result = LanguageModelToolResultPart::new(
        tool_result.tool_call_id,
        tool_result.tool_name,
        convert_tool_result_output(tool_result.output)?,
    );

    if let Some(opts) = tool_result.provider_options {
        result.provider_options = Some(opts);
    }

    Ok(result)
}

/// Convert core `ToolResultOutput` to provider `ToolResultOutput`.
fn convert_tool_result_output(
    output: crate::prompt::message::content_parts::ToolResultOutput,
) -> Result<LanguageModelToolResultOutput, AISDKError> {
    use crate::prompt::message::content_parts::ToolResultOutput;
    use ai_sdk_provider::language_model::prompt::LanguageModelToolResultContentItem;

    match output {
        ToolResultOutput::Text {
            value,
            provider_options: _,
        } => Ok(LanguageModelToolResultOutput::Text { value }),
        ToolResultOutput::Json {
            value,
            provider_options: _,
        } => Ok(LanguageModelToolResultOutput::Json { value }),
        ToolResultOutput::ErrorText {
            value,
            provider_options: _,
        } => Ok(LanguageModelToolResultOutput::ErrorText { value }),
        ToolResultOutput::ErrorJson {
            value,
            provider_options: _,
        } => Ok(LanguageModelToolResultOutput::ErrorJson { value }),
        ToolResultOutput::Content { value } => {
            // Convert content items from core to provider format
            let converted_items: Vec<LanguageModelToolResultContentItem> = value
                .into_iter()
                .filter_map(|item| convert_tool_result_content_part(item))
                .collect();

            Ok(LanguageModelToolResultOutput::Content {
                value: converted_items,
            })
        }
        ToolResultOutput::ExecutionDenied {
            reason,
            provider_options: _,
        } => Ok(LanguageModelToolResultOutput::ErrorText {
            value: format!(
                "Execution denied: {}",
                reason.unwrap_or_else(|| "No reason provided".to_string())
            ),
        }),
    }
}

/// Convert a core ToolResultContentPart to a provider ToolResultContentItem.
///
/// Returns None for items that cannot be converted (like FileId which providers don't support).
fn convert_tool_result_content_part(
    part: crate::prompt::message::content_parts::ToolResultContentPart,
) -> Option<ai_sdk_provider::language_model::prompt::LanguageModelToolResultContentItem> {
    use crate::prompt::message::content_parts::ToolResultContentPart;
    use ai_sdk_provider::language_model::prompt::LanguageModelToolResultContentItem;

    match part {
        ToolResultContentPart::Text {
            text,
            provider_options: _,
        } => Some(LanguageModelToolResultContentItem::Text { text }),

        #[allow(deprecated)]
        ToolResultContentPart::Media { data, media_type } => {
            Some(LanguageModelToolResultContentItem::Media { data, media_type })
        }

        ToolResultContentPart::FileData {
            data, media_type, ..
        } => {
            // Convert FileData to Media (provider doesn't have FileData variant)
            Some(LanguageModelToolResultContentItem::Media { data, media_type })
        }

        ToolResultContentPart::ImageData {
            data, media_type, ..
        } => {
            // Convert ImageData to Media
            Some(LanguageModelToolResultContentItem::Media { data, media_type })
        }

        // For URL-based and ID-based items, we can't convert them directly
        // The provider expects base64 data, not URLs
        ToolResultContentPart::FileUrl { .. }
        | ToolResultContentPart::FileId { .. }
        | ToolResultContentPart::ImageUrl { .. }
        | ToolResultContentPart::ImageFileId { .. } => {
            // TODO: In a real implementation, we might want to download these
            // For now, skip them
            None
        }

        ToolResultContentPart::Custom { .. } => {
            // Custom items aren't supported by provider
            None
        }
    }
}

/// Convert an image source to provider `DataContent`.
fn convert_image_source_to_data_content(
    source: ImageSource,
) -> Result<(LanguageModelDataContent, Option<String>), AISDKError> {
    match source {
        ImageSource::Data(data) => {
            let base64 = data.to_base64();
            Ok((LanguageModelDataContent::Base64(base64), None))
        }
        ImageSource::Url(url) => Ok((LanguageModelDataContent::Url(url), None)),
    }
}

/// Convert a file source to provider `DataContent`.
fn convert_file_source_to_data_content(
    source: FileSource,
) -> Result<(LanguageModelDataContent, Option<String>), AISDKError> {
    match source {
        FileSource::Data(data) => {
            let base64 = data.to_base64();
            Ok((LanguageModelDataContent::Base64(base64), None))
        }
        FileSource::Url(url) => Ok((LanguageModelDataContent::Url(url), None)),
    }
}

/// Combine consecutive tool messages into a single tool message.
///
/// This is an optimization to reduce the number of messages sent to the provider.
fn combine_consecutive_tool_messages(
    messages: Vec<LanguageModelMessage>,
) -> Vec<LanguageModelMessage> {
    let mut combined = Vec::new();

    for message in messages {
        if let LanguageModelMessage::Tool(tool_msg) = message {
            // Check if the last message was also a tool message
            if let Some(LanguageModelMessage::Tool(last_tool)) = combined.last_mut() {
                // Append to the existing tool message
                last_tool.content.extend(tool_msg.content);
            } else {
                // Add as a new tool message
                combined.push(LanguageModelMessage::Tool(tool_msg));
            }
        } else {
            combined.push(message);
        }
    }

    combined
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prompt::message::{SystemMessage, UserMessage};

    #[test]
    fn test_convert_text_prompt() {
        let prompt = StandardizedPrompt {
            system: Some("You are helpful".to_string()),
            messages: vec![Message::User(UserMessage {
                role: "user".to_string(),
                content: UserContent::Text("Hello".to_string()),
                provider_options: None,
            })],
        };

        let result = convert_to_language_model_prompt(prompt).unwrap();
        assert_eq!(result.len(), 2); // system + user
    }

    #[test]
    fn test_combine_consecutive_tool_messages() {
        // Create two consecutive tool messages
        let messages = vec![
            LanguageModelMessage::Tool(LanguageModelToolMessage::new(vec![])),
            LanguageModelMessage::Tool(LanguageModelToolMessage::new(vec![])),
        ];

        let combined = combine_consecutive_tool_messages(messages);
        assert_eq!(combined.len(), 1);
    }
}
