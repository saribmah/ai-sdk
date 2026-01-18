//! Storage conversion utilities for llm-kit-storage integration.
//!
//! This module provides conversion functions between `llm-kit-core` types (prompts, outputs, messages)
//! and `llm-kit-storage` types (StorageUserMessage, StorageAssistantMessage, MessagePart).
//!
//! # Key Functions
//!
//! - `user_message_to_storage`: Convert user messages from prompts to storage format
//! - `assistant_output_to_storage`: Convert assistant outputs to storage format
//! - `load_conversation_history`: Load and convert stored messages back to prompt format
//!
//! # Example
//!
//! ```ignore
//! use llm_kit_storage_filesystem::FilesystemStorage;
//! use llm_kit_core::storage_conversion::*;
//!
//! let storage = Arc::new(FilesystemStorage::new("./storage")?);
//! let session_id = storage.generate_session_id();
//!
//! // Convert user message to storage
//! let (storage_msg, parts) = user_message_to_storage(
//!     &storage,
//!     session_id.clone(),
//!     &user_message,
//! );
//! storage.store_user_message(&storage_msg, &parts).await?;
//!
//! // Load conversation history
//! let history = load_conversation_history(&storage, &session_id).await?;
//! ```

use crate::generate_text::GenerateTextResult;
use crate::output::Output;
use llm_kit_provider_utils::message::{AssistantMessage, Message, UserMessage};
use llm_kit_storage::{
    AssistantMessage as StorageAssistantMessage, FileData as StorageFileData,
    FilePart as StorageFilePart, ImageData as StorageImageData, ImagePart as StorageImagePart,
    MessageMetadata, MessagePart, MessageRole, ReasoningPart, Storage, StorageError, TextPart,
    ToolCallPart, UsageStats, UserMessage as StorageUserMessage,
};
use std::sync::Arc;

/// Convert a user message to storage format.
///
/// Takes a user message from the prompt system and converts it to the storage format,
/// generating part IDs for each content component.
///
/// # Arguments
///
/// * `storage` - Storage instance for generating IDs
/// * `session_id` - Session this message belongs to
/// * `user_message` - The user message to convert
///
/// # Returns
///
/// A tuple of (StorageUserMessage, `Vec<MessagePart>`) ready to be stored
///
/// # Example
///
/// ```ignore
/// let user_msg = UserMessage::from_text("Hello!".to_string());
/// let (storage_msg, parts) = user_message_to_storage(&storage, session_id, &user_msg);
/// storage.store_user_message(&storage_msg, &parts).await?;
/// ```
pub fn user_message_to_storage(
    storage: &Arc<dyn Storage>,
    session_id: String,
    user_message: &UserMessage,
) -> (StorageUserMessage, Vec<MessagePart>) {
    let message_id = storage.generate_message_id();
    let mut parts = Vec::new();
    let mut part_ids = Vec::new();

    // Extract text content from user message
    match &user_message.content {
        llm_kit_provider_utils::message::UserContent::Text(text) => {
            let part_id = storage.generate_part_id();
            parts.push(MessagePart::Text(TextPart::new(
                part_id.clone(),
                text.clone(),
            )));
            part_ids.push(part_id);
        }
        llm_kit_provider_utils::message::UserContent::Parts(content_parts) => {
            for content in content_parts {
                let part_id = storage.generate_part_id();

                match content {
                    llm_kit_provider_utils::message::UserContentPart::Text(text_part) => {
                        parts.push(MessagePart::Text(TextPart::new(
                            part_id.clone(),
                            text_part.text.clone(),
                        )));
                        part_ids.push(part_id);
                    }
                    llm_kit_provider_utils::message::UserContentPart::Image(image_part) => {
                        if let Some(storage_part) =
                            image_part_to_storage(part_id.clone(), image_part)
                        {
                            parts.push(storage_part);
                            part_ids.push(part_id);
                        }
                    }
                    llm_kit_provider_utils::message::UserContentPart::File(file_part) => {
                        if let Some(storage_part) = file_part_to_storage(part_id.clone(), file_part)
                        {
                            parts.push(storage_part);
                            part_ids.push(part_id);
                        }
                    }
                }
            }
        }
    }

    let storage_message = StorageUserMessage::new(message_id, session_id, part_ids);
    (storage_message, parts)
}

/// Convert response messages to storage format.
///
/// Takes all response messages from a generation (assistant + tool messages) and converts
/// them to storage format. Tool messages are stored as assistant messages containing only
/// tool result parts, which allows them to be distinguished when loading.
///
/// # Arguments
///
/// * `storage` - Storage instance for generating IDs
/// * `session_id` - Session these messages belong to
/// * `response_messages` - All response messages from the generation
/// * `result` - The final generation result for metadata (model_id, usage, finish_reason)
///
/// # Returns
///
/// A vector of tuples (StorageAssistantMessage, `Vec<MessagePart>`) ready to be stored
///
/// # Example
///
/// ```ignore
/// let storage_messages = response_messages_to_storage(&storage, session_id, &response_messages, &result);
/// for (msg, parts) in storage_messages {
///     storage.store_assistant_message(&msg, &parts).await?;
/// }
/// ```
pub fn response_messages_to_storage(
    storage: &Arc<dyn Storage>,
    session_id: String,
    response_messages: &[crate::ResponseMessage],
    result: &GenerateTextResult,
) -> Vec<(StorageAssistantMessage, Vec<MessagePart>)> {
    use crate::ResponseMessage;
    use llm_kit_provider_utils::message::AssistantContent;

    let mut storage_messages = Vec::new();

    for response_msg in response_messages {
        let message_id = storage.generate_message_id();
        let mut parts = Vec::new();
        let mut part_ids = Vec::new();

        match response_msg {
            ResponseMessage::Assistant(assistant_msg) => {
                // Convert assistant message content to storage parts
                match &assistant_msg.content {
                    AssistantContent::Text(text) => {
                        let part_id = storage.generate_part_id();
                        parts.push(MessagePart::Text(TextPart::new(
                            part_id.clone(),
                            text.clone(),
                        )));
                        part_ids.push(part_id);
                    }
                    AssistantContent::Parts(content_parts) => {
                        for content_part in content_parts {
                            let part_id = storage.generate_part_id();

                            let part = match content_part {
                                llm_kit_provider_utils::message::AssistantContentPart::Text(text) => {
                                    MessagePart::Text(TextPart::new(
                                        part_id.clone(),
                                        text.text.clone(),
                                    ))
                                }
                                llm_kit_provider_utils::message::AssistantContentPart::Reasoning(
                                    reasoning,
                                ) => MessagePart::Reasoning(ReasoningPart::new(
                                    part_id.clone(),
                                    reasoning.text.clone(),
                                )),
                                llm_kit_provider_utils::message::AssistantContentPart::ToolCall(
                                    tool_call,
                                ) => MessagePart::ToolCall(ToolCallPart::new(
                                    part_id.clone(),
                                    tool_call.tool_call_id.clone(),
                                    tool_call.tool_name.clone(),
                                    tool_call.input.clone(),
                                )),
                                llm_kit_provider_utils::message::AssistantContentPart::ToolResult(
                                    tool_result,
                                ) => {
                                    // Convert ToolResultOutput to storage format
                                    use llm_kit_provider_utils::message::content_parts::ToolResultOutput;
                                    match &tool_result.output {
                                        ToolResultOutput::Text { value, .. } => {
                                            MessagePart::ToolResult(
                                                llm_kit_storage::ToolResultPart::new_success(
                                                    part_id.clone(),
                                                    tool_result.tool_call_id.clone(),
                                                    tool_result.tool_name.clone(),
                                                    serde_json::Value::String(value.clone()),
                                                ),
                                            )
                                        }
                                        ToolResultOutput::Json { value, .. } => {
                                            MessagePart::ToolResult(
                                                llm_kit_storage::ToolResultPart::new_success(
                                                    part_id.clone(),
                                                    tool_result.tool_call_id.clone(),
                                                    tool_result.tool_name.clone(),
                                                    value.clone(),
                                                ),
                                            )
                                        }
                                        ToolResultOutput::Content { value, .. } => {
                                            // For content array, serialize the whole array
                                            MessagePart::ToolResult(
                                                llm_kit_storage::ToolResultPart::new_success(
                                                    part_id.clone(),
                                                    tool_result.tool_call_id.clone(),
                                                    tool_result.tool_name.clone(),
                                                    serde_json::to_value(value).unwrap_or_default(),
                                                ),
                                            )
                                        }
                                        ToolResultOutput::ExecutionDenied { reason, .. } => {
                                            let error_msg = reason
                                                .clone()
                                                .unwrap_or_else(|| "Execution denied".to_string());
                                            MessagePart::ToolResult(
                                                llm_kit_storage::ToolResultPart::new_error(
                                                    part_id.clone(),
                                                    tool_result.tool_call_id.clone(),
                                                    tool_result.tool_name.clone(),
                                                    error_msg,
                                                ),
                                            )
                                        }
                                        ToolResultOutput::ErrorText { value, .. } => {
                                            MessagePart::ToolResult(
                                                llm_kit_storage::ToolResultPart::new_error(
                                                    part_id.clone(),
                                                    tool_result.tool_call_id.clone(),
                                                    tool_result.tool_name.clone(),
                                                    value.clone(),
                                                ),
                                            )
                                        }
                                        ToolResultOutput::ErrorJson { value, .. } => {
                                            MessagePart::ToolResult(
                                                llm_kit_storage::ToolResultPart::new_error(
                                                    part_id.clone(),
                                                    tool_result.tool_call_id.clone(),
                                                    tool_result.tool_name.clone(),
                                                    value.to_string(),
                                                ),
                                            )
                                        }
                                    }
                                }
                                llm_kit_provider_utils::message::AssistantContentPart::File(file) => {
                                    generated_file_to_storage_from_file_part(part_id.clone(), file)
                                }
                                llm_kit_provider_utils::message::AssistantContentPart::ToolApprovalRequest(
                                    _,
                                ) => {
                                    // Skip tool approval requests - they're not persisted
                                    continue;
                                }
                            };

                            parts.push(part);
                            part_ids.push(part_id);
                        }
                    }
                }

                // Build metadata only for the last message (final result)
                let metadata = if response_msg == response_messages.last().unwrap()
                    && matches!(response_msg, ResponseMessage::Assistant(_))
                {
                    MessageMetadata {
                        model_id: result.response.model_id.clone(),
                        provider: None,
                        usage: Some(UsageStats::new(
                            result.usage.input_tokens as u32,
                            result.usage.output_tokens as u32,
                        )),
                        finish_reason: Some(format!("{:?}", result.finish_reason)),
                        custom: None,
                    }
                } else {
                    MessageMetadata::default()
                };

                let storage_message =
                    StorageAssistantMessage::new(message_id, session_id.clone(), part_ids)
                        .with_metadata(metadata);

                storage_messages.push((storage_message, parts));
            }
            ResponseMessage::Tool(tool_msg) => {
                // Convert tool message to assistant message with only tool result parts
                for tool_content_part in &tool_msg.content {
                    match tool_content_part {
                        llm_kit_provider_utils::message::tool::ToolContentPart::ToolResult(
                            tool_result,
                        ) => {
                            let part_id = storage.generate_part_id();

                            let part = {
                                use llm_kit_provider_utils::message::content_parts::ToolResultOutput;
                                match &tool_result.output {
                                    ToolResultOutput::Text { value, .. } => {
                                        MessagePart::ToolResult(
                                            llm_kit_storage::ToolResultPart::new_success(
                                                part_id.clone(),
                                                tool_result.tool_call_id.clone(),
                                                tool_result.tool_name.clone(),
                                                serde_json::Value::String(value.clone()),
                                            ),
                                        )
                                    }
                                    ToolResultOutput::Json { value, .. } => {
                                        MessagePart::ToolResult(
                                            llm_kit_storage::ToolResultPart::new_success(
                                                part_id.clone(),
                                                tool_result.tool_call_id.clone(),
                                                tool_result.tool_name.clone(),
                                                value.clone(),
                                            ),
                                        )
                                    }
                                    ToolResultOutput::Content { value, .. } => {
                                        MessagePart::ToolResult(
                                            llm_kit_storage::ToolResultPart::new_success(
                                                part_id.clone(),
                                                tool_result.tool_call_id.clone(),
                                                tool_result.tool_name.clone(),
                                                serde_json::to_value(value).unwrap_or_default(),
                                            ),
                                        )
                                    }
                                    ToolResultOutput::ExecutionDenied { reason, .. } => {
                                        let error_msg = reason
                                            .clone()
                                            .unwrap_or_else(|| "Execution denied".to_string());
                                        MessagePart::ToolResult(
                                            llm_kit_storage::ToolResultPart::new_error(
                                                part_id.clone(),
                                                tool_result.tool_call_id.clone(),
                                                tool_result.tool_name.clone(),
                                                error_msg,
                                            ),
                                        )
                                    }
                                    ToolResultOutput::ErrorText { value, .. } => {
                                        MessagePart::ToolResult(
                                            llm_kit_storage::ToolResultPart::new_error(
                                                part_id.clone(),
                                                tool_result.tool_call_id.clone(),
                                                tool_result.tool_name.clone(),
                                                value.clone(),
                                            ),
                                        )
                                    }
                                    ToolResultOutput::ErrorJson { value, .. } => {
                                        MessagePart::ToolResult(
                                            llm_kit_storage::ToolResultPart::new_error(
                                                part_id.clone(),
                                                tool_result.tool_call_id.clone(),
                                                tool_result.tool_name.clone(),
                                                value.to_string(),
                                            ),
                                        )
                                    }
                                }
                            };

                            parts.push(part);
                            part_ids.push(part_id);
                        }
                        llm_kit_provider_utils::message::tool::ToolContentPart::ApprovalResponse(
                            _,
                        ) => {
                            // Skip approval responses - they're not persisted
                            continue;
                        }
                    }
                }

                // Store as assistant message with metadata flag indicating client-executed tools
                let metadata = MessageMetadata {
                    custom: Some(serde_json::json!({
                        "is_client_tool_message": true
                    })),
                    ..Default::default()
                };

                let storage_message =
                    StorageAssistantMessage::new(message_id, session_id.clone(), part_ids)
                        .with_metadata(metadata);

                storage_messages.push((storage_message, parts));
            }
        }
    }

    storage_messages
}

/// Convert assistant output to storage format.
///
/// Takes the model's output and converts it to storage format, including all outputs
/// (text, reasoning, tool calls, tool results) and metadata (usage, finish reason).
///
/// # Arguments
///
/// * `storage` - Storage instance for generating IDs
/// * `session_id` - Session this message belongs to
/// * `result` - The generation result containing outputs and metadata
///
/// # Returns
///
/// A tuple of (StorageAssistantMessage, `Vec<MessagePart>`) ready to be stored
///
/// # Example
///
/// ```ignore
/// let (storage_msg, parts) = assistant_output_to_storage(&storage, session_id, &result);
/// storage.store_assistant_message(&storage_msg, &parts).await?;
/// ```
pub fn assistant_output_to_storage(
    storage: &Arc<dyn Storage>,
    session_id: String,
    result: &GenerateTextResult,
) -> (StorageAssistantMessage, Vec<MessagePart>) {
    let message_id = storage.generate_message_id();
    let mut parts = Vec::new();
    let mut part_ids = Vec::new();

    // Convert each output to a message part
    for output in &result.content {
        let part_id = storage.generate_part_id();

        let part = match output {
            Output::Text(text) => {
                MessagePart::Text(TextPart::new(part_id.clone(), text.text.clone()))
            }
            Output::Reasoning(reasoning) => {
                MessagePart::Reasoning(ReasoningPart::new(part_id.clone(), reasoning.text.clone()))
            }
            Output::ToolCall(tool_call) => MessagePart::ToolCall(ToolCallPart::new(
                part_id.clone(),
                tool_call.tool_call_id.clone(),
                tool_call.tool_name.clone(),
                tool_call.input.clone(),
            )),
            Output::ToolResult(tool_result) => {
                // ToolResult already has output as JSON Value
                MessagePart::ToolResult(llm_kit_storage::ToolResultPart::new_success(
                    part_id.clone(),
                    tool_result.tool_call_id.clone(),
                    tool_result.tool_name.clone(),
                    tool_result.output.clone(),
                ))
            }
            Output::ToolError(tool_error) => {
                // ToolError has error as JSON Value
                MessagePart::ToolResult(llm_kit_storage::ToolResultPart::new_error(
                    part_id.clone(),
                    tool_error.tool_call_id.clone(),
                    tool_error.tool_name.clone(),
                    tool_error.error.to_string(),
                ))
            }
            Output::Source(source) => {
                // Convert LanguageModelSource to storage SourcePart
                if let Some(storage_source) = source_output_to_storage(part_id.clone(), source) {
                    storage_source
                } else {
                    continue;
                }
            }
            Output::File(file) => {
                // Convert GeneratedFile to storage FilePart
                generated_file_to_storage(part_id.clone(), file)
            }
        };

        parts.push(part);
        part_ids.push(part_id);
    }

    // Build metadata
    let metadata = MessageMetadata {
        model_id: result.response.model_id.clone(),
        provider: None, // Provider info not directly available in result
        usage: Some(UsageStats::new(
            result.usage.input_tokens as u32,
            result.usage.output_tokens as u32,
        )),
        finish_reason: Some(format!("{:?}", result.finish_reason)),
        custom: None,
    };

    let storage_message =
        StorageAssistantMessage::new(message_id, session_id, part_ids).with_metadata(metadata);

    (storage_message, parts)
}

/// Load conversation history from storage and convert to prompt messages.
///
/// Retrieves all messages in a session and converts them back to the prompt format
/// used by `GenerateText` and `StreamText`. Assistant messages that contain ONLY
/// tool results are reconstructed as `Message::Tool` to preserve the original
/// conversation structure.
///
/// # Arguments
///
/// * `storage` - Storage instance to load from
/// * `session_id` - Session ID to load messages for
///
/// # Returns
///
/// A vector of `Message` objects in chronological order (oldest first)
///
/// # Errors
///
/// Returns `StorageError` if loading fails or session doesn't exist
///
/// # Example
///
/// ```ignore
/// let history = load_conversation_history(&storage, &session_id).await?;
/// let prompt = Prompt::messages(history);
/// ```
pub async fn load_conversation_history(
    storage: &Arc<dyn Storage>,
    session_id: &str,
) -> Result<Vec<Message>, StorageError> {
    let message_ids = storage.list_messages(session_id, None).await?;
    let mut messages = Vec::new();

    for message_id in message_ids {
        let (role, parts, metadata) = storage
            .get_message_with_metadata(session_id, &message_id)
            .await?;

        match role {
            MessageRole::User => {
                let message = parts_to_user_message(&parts);
                messages.push(Message::User(message));
            }
            MessageRole::Assistant => {
                // Check if this is a client-executed tool message by looking at metadata
                let is_client_tool_message = metadata
                    .as_ref()
                    .and_then(|m| m.custom.as_ref())
                    .and_then(|c| c.get("is_client_tool_message"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                if is_client_tool_message {
                    // Reconstruct as Tool message (client-executed tools)
                    let tool_message = parts_to_tool_message(&parts);
                    messages.push(Message::Tool(tool_message));
                } else {
                    // Regular assistant message (including provider-executed tool results)
                    let message = parts_to_assistant_message(&parts);
                    messages.push(Message::Assistant(message));
                }
            }
            MessageRole::System => {
                // Handle system messages if needed in the future
            }
        }
    }

    Ok(messages)
}

/// Convert storage parts to a UserMessage.
///
/// Internal helper function that reconstructs a user message from stored parts.
fn parts_to_user_message(parts: &[MessagePart]) -> UserMessage {
    // Simple text message case
    if parts.len() == 1
        && let MessagePart::Text(text_part) = &parts[0]
    {
        return UserMessage::new(text_part.text.clone());
    }

    // Multi-part message - convert each part
    let mut content_parts = Vec::new();
    for part in parts {
        match part {
            MessagePart::Text(text) => {
                content_parts.push(llm_kit_provider_utils::message::UserContentPart::Text(
                    llm_kit_provider_utils::message::TextPart::new(text.text.clone()),
                ));
            }
            MessagePart::Image(image) => {
                if let Some(prompt_image) = storage_image_to_prompt(image) {
                    content_parts.push(llm_kit_provider_utils::message::UserContentPart::Image(
                        prompt_image,
                    ));
                }
            }
            MessagePart::File(file) => {
                if let Some(prompt_file) = storage_file_to_prompt(file) {
                    content_parts.push(llm_kit_provider_utils::message::UserContentPart::File(
                        prompt_file,
                    ));
                }
            }
            _ => {
                // Skip other part types (tool calls, reasoning, etc.) in user messages
            }
        }
    }

    UserMessage::with_parts(content_parts)
}

/// Convert storage parts to a ToolMessage.
///
/// Internal helper function that reconstructs a tool message from stored parts.
/// This is used when an assistant message contains ONLY tool results.
fn parts_to_tool_message(parts: &[MessagePart]) -> llm_kit_provider_utils::message::ToolMessage {
    use llm_kit_provider_utils::message::tool::{ToolContentPart, ToolMessage};

    let mut tool_content: Vec<ToolContentPart> = Vec::new();

    for part in parts {
        if let MessagePart::ToolResult(tool_result) = part
            && let Some(result_part) = storage_tool_result_to_tool_content(tool_result)
        {
            tool_content.push(result_part);
        }
    }

    ToolMessage::new(tool_content)
}

/// Convert storage parts to an AssistantMessage.
///
/// Internal helper function that reconstructs an assistant message from stored parts.
fn parts_to_assistant_message(parts: &[MessagePart]) -> AssistantMessage {
    // Simple text message case
    if parts.len() == 1
        && let MessagePart::Text(text_part) = &parts[0]
    {
        return AssistantMessage::new(text_part.text.clone());
    }

    // Multi-part message - convert each part
    let mut content_parts = Vec::new();
    for part in parts {
        match part {
            MessagePart::Text(text) => {
                content_parts.push(llm_kit_provider_utils::message::AssistantContentPart::Text(
                    llm_kit_provider_utils::message::TextPart::new(text.text.clone()),
                ));
            }
            MessagePart::Reasoning(reasoning) => {
                content_parts.push(
                    llm_kit_provider_utils::message::AssistantContentPart::Reasoning(
                        llm_kit_provider_utils::message::ReasoningPart::new(
                            reasoning.content.clone(),
                        ),
                    ),
                );
            }
            MessagePart::ToolCall(tool_call) => {
                content_parts.push(
                    llm_kit_provider_utils::message::AssistantContentPart::ToolCall(
                        llm_kit_provider_utils::message::ToolCallPart::new(
                            tool_call.tool_call_id.clone(),
                            tool_call.tool_name.clone(),
                            tool_call.arguments.clone(),
                        ),
                    ),
                );
            }
            MessagePart::ToolResult(tool_result) => {
                if let Some(part) = storage_tool_result_to_assistant(tool_result) {
                    content_parts.push(part);
                }
            }
            MessagePart::File(file) => {
                if let Some(part) = storage_file_to_assistant(file) {
                    content_parts.push(part);
                }
            }
            // Skip source and image parts - sources are metadata only, images not in AssistantContentPart
            MessagePart::Source(_) | MessagePart::Image(_) => {}
        }
    }

    AssistantMessage::with_parts(content_parts)
}

/// Convert a prompt ImagePart to storage format.
///
/// Returns `None` if the conversion cannot be performed (e.g., unsupported format).
fn image_part_to_storage(
    id: String,
    image: &llm_kit_provider_utils::message::ImagePart,
) -> Option<MessagePart> {
    use llm_kit_provider_utils::message::{DataContent, ImageSource};

    let media_type = image
        .media_type
        .clone()
        .unwrap_or_else(|| "image/jpeg".to_string());

    let storage_data = match &image.image {
        ImageSource::Data(data_content) => match data_content {
            DataContent::Base64(base64_str) => StorageImageData::Base64 {
                data: base64_str.clone(),
            },
            DataContent::Bytes(bytes) => StorageImageData::Binary {
                data: bytes.clone(),
            },
        },
        ImageSource::Url(url) => StorageImageData::Url {
            url: url.to_string(),
        },
    };

    Some(MessagePart::Image(StorageImagePart::new(
        id,
        media_type,
        storage_data,
    )))
}

/// Convert a prompt FilePart to storage format.
///
/// Returns `None` if the conversion cannot be performed.
fn file_part_to_storage(
    id: String,
    file: &llm_kit_provider_utils::message::FilePart,
) -> Option<MessagePart> {
    use llm_kit_provider_utils::message::{DataContent, FileSource};

    let storage_data = match &file.data {
        FileSource::Data(data_content) => match data_content {
            DataContent::Base64(base64_str) => StorageFileData::Base64 {
                data: base64_str.clone(),
            },
            DataContent::Bytes(bytes) => StorageFileData::Binary {
                data: bytes.clone(),
            },
        },
        FileSource::Url(url) => StorageFileData::Url {
            url: url.to_string(),
        },
    };

    let mut storage_part = StorageFilePart::new(id, file.media_type.clone(), storage_data);

    if let Some(filename) = &file.filename {
        storage_part = storage_part.with_filename(filename.clone());
    }

    Some(MessagePart::File(storage_part))
}

/// Convert storage ImagePart to prompt format.
///
/// Returns `None` if the conversion cannot be performed.
fn storage_image_to_prompt(
    image: &StorageImagePart,
) -> Option<llm_kit_provider_utils::message::ImagePart> {
    use llm_kit_provider_utils::message::{DataContent, ImagePart as PromptImagePart};

    let mut prompt_image = match &image.data {
        StorageImageData::Base64 { data } => {
            PromptImagePart::from_data(DataContent::Base64(data.clone()))
        }
        StorageImageData::Binary { data } => {
            PromptImagePart::from_data(DataContent::Bytes(data.clone()))
        }
        StorageImageData::Url { url } => {
            // Parse URL - if it fails, skip this image
            match url::Url::parse(url) {
                Ok(parsed_url) => PromptImagePart::from_url(parsed_url),
                Err(_) => return None,
            }
        }
    };

    prompt_image.media_type = Some(image.media_type.clone());

    Some(prompt_image)
}

/// Convert storage FilePart to prompt format.
///
/// Returns `None` if the conversion cannot be performed.
fn storage_file_to_prompt(
    file: &StorageFilePart,
) -> Option<llm_kit_provider_utils::message::FilePart> {
    use llm_kit_provider_utils::message::{DataContent, FilePart as PromptFilePart};

    let mut prompt_file = match &file.data {
        StorageFileData::Base64 { data } => {
            PromptFilePart::from_data(DataContent::Base64(data.clone()), file.media_type.clone())
        }
        StorageFileData::Binary { data } => {
            PromptFilePart::from_data(DataContent::Bytes(data.clone()), file.media_type.clone())
        }
        StorageFileData::Url { url } => {
            // Parse URL - if it fails, skip this file
            match url::Url::parse(url) {
                Ok(parsed_url) => PromptFilePart::from_url(parsed_url, file.media_type.clone()),
                Err(_) => return None,
            }
        }
    };

    if let Some(filename) = &file.filename {
        prompt_file = prompt_file.with_filename(filename.clone());
    }

    Some(prompt_file)
}

/// Convert a SourceOutput to storage format.
///
/// Returns `None` if the conversion cannot be performed.
fn source_output_to_storage(
    id: String,
    source_output: &crate::output::SourceOutput,
) -> Option<MessagePart> {
    use llm_kit_provider::language_model::content::source::LanguageModelSource;
    use llm_kit_storage::SourcePart;

    let (reference, title) = match &source_output.source {
        LanguageModelSource::Url { url, title, .. } => (url.clone(), title.clone()),
        LanguageModelSource::Document {
            title, filename, ..
        } => (
            filename.clone().unwrap_or_else(|| title.clone()),
            Some(title.clone()),
        ),
    };

    let mut source_part = SourcePart::new(id, reference);
    if let Some(title_str) = title {
        source_part = source_part.with_title(title_str);
    }

    Some(MessagePart::Source(source_part))
}

/// Convert a GeneratedFile to storage format.
fn generated_file_to_storage(
    id: String,
    file: &crate::generate_text::GeneratedFile,
) -> MessagePart {
    use llm_kit_storage::FilePart as StorageFilePart;

    // Use base64 format for storage
    let storage_data = StorageFileData::Base64 {
        data: file.base64().to_string(),
    };

    let mut storage_part = StorageFilePart::new(id, file.media_type.clone(), storage_data);

    if let Some(filename) = &file.name {
        storage_part = storage_part.with_filename(filename.clone());
    }

    MessagePart::File(storage_part)
}

/// Convert a FilePart (from assistant message) to storage format.
fn generated_file_to_storage_from_file_part(
    id: String,
    file: &llm_kit_provider_utils::message::FilePart,
) -> MessagePart {
    use llm_kit_provider_utils::message::{DataContent, FileSource};
    use llm_kit_storage::FilePart as StorageFilePart;

    let storage_data = match &file.data {
        FileSource::Data(data_content) => match data_content {
            DataContent::Base64(base64_str) => StorageFileData::Base64 {
                data: base64_str.clone(),
            },
            DataContent::Bytes(bytes) => StorageFileData::Binary {
                data: bytes.clone(),
            },
        },
        FileSource::Url(url) => StorageFileData::Url {
            url: url.to_string(),
        },
    };

    let mut storage_part = StorageFilePart::new(id, file.media_type.clone(), storage_data);

    if let Some(filename) = &file.filename {
        storage_part = storage_part.with_filename(filename.clone());
    }

    MessagePart::File(storage_part)
}

/// Convert storage FilePart to assistant message part.
fn storage_file_to_assistant(
    file: &StorageFilePart,
) -> Option<llm_kit_provider_utils::message::AssistantContentPart> {
    use llm_kit_provider_utils::message::{
        AssistantContentPart, DataContent, FilePart as PromptFilePart,
    };

    let prompt_file_data = match &file.data {
        StorageFileData::Base64 { data } => DataContent::Base64(data.clone()),
        StorageFileData::Binary { data } => DataContent::Bytes(data.clone()),
        StorageFileData::Url { url: _ } => {
            // For URL, we'll convert to a prompt FilePart with URL source
            // However, prompt FilePart doesn't have a direct URL constructor for assistant messages
            // We'll use Base64 or skip - let's skip for now
            return None;
        }
    };

    let mut prompt_file = PromptFilePart::from_data(prompt_file_data, file.media_type.clone());
    if let Some(filename) = &file.filename {
        prompt_file = prompt_file.with_filename(filename.clone());
    }

    Some(AssistantContentPart::File(prompt_file))
}

/// Convert storage ToolResultPart to assistant message part.
fn storage_tool_result_to_assistant(
    tool_result: &llm_kit_storage::ToolResultPart,
) -> Option<llm_kit_provider_utils::message::AssistantContentPart> {
    use llm_kit_provider_utils::message::{
        AssistantContentPart, ToolResultOutput, ToolResultPart as PromptToolResultPart,
    };
    use llm_kit_storage::ToolResultData;

    let output = match &tool_result.result {
        ToolResultData::Success { output } => ToolResultOutput::Json {
            value: output.clone(),
            provider_options: None,
        },
        ToolResultData::Error { error } => ToolResultOutput::Text {
            value: format!("Error: {}", error),
            provider_options: None,
        },
    };

    let prompt_tool_result = PromptToolResultPart::new(
        tool_result.tool_call_id.clone(),
        tool_result.tool_name.clone(),
        output,
    );

    Some(AssistantContentPart::ToolResult(prompt_tool_result))
}

/// Convert storage ToolResultPart to tool content part.
fn storage_tool_result_to_tool_content(
    tool_result: &llm_kit_storage::ToolResultPart,
) -> Option<llm_kit_provider_utils::message::tool::ToolContentPart> {
    use llm_kit_provider_utils::message::tool::ToolContentPart;
    use llm_kit_provider_utils::message::{
        ToolResultOutput, ToolResultPart as PromptToolResultPart,
    };
    use llm_kit_storage::ToolResultData;

    let output = match &tool_result.result {
        ToolResultData::Success { output } => ToolResultOutput::Json {
            value: output.clone(),
            provider_options: None,
        },
        ToolResultData::Error { error } => ToolResultOutput::Text {
            value: format!("Error: {}", error),
            provider_options: None,
        },
    };

    let prompt_tool_result = PromptToolResultPart::new(
        tool_result.tool_call_id.clone(),
        tool_result.tool_name.clone(),
        output,
    );

    Some(ToolContentPart::ToolResult(prompt_tool_result))
}

#[cfg(all(test, feature = "storage"))]
mod tests {
    use super::*;

    use llm_kit_storage_filesystem::FilesystemStorage;
    use tempfile::TempDir;

    async fn setup_storage() -> (Arc<dyn Storage>, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let storage = FilesystemStorage::new(temp_dir.path()).unwrap();
        storage.initialize().await.unwrap();
        (Arc::new(storage), temp_dir)
    }

    #[tokio::test]
    async fn test_user_message_conversion() {
        let (storage, _dir) = setup_storage().await;
        let session_id = storage.generate_session_id();

        let user_msg = UserMessage::new("Hello, AI!");
        let (storage_msg, parts) = user_message_to_storage(&storage, session_id.clone(), &user_msg);

        assert_eq!(storage_msg.session_id, session_id);
        assert_eq!(parts.len(), 1);
        assert!(matches!(parts[0], MessagePart::Text(_)));

        if let MessagePart::Text(text_part) = &parts[0] {
            assert_eq!(text_part.text, "Hello, AI!");
        }
    }

    #[tokio::test]
    async fn test_conversation_history_roundtrip() {
        let (storage, _dir) = setup_storage().await;
        let session_id = storage.generate_session_id();

        // Create session
        let session = llm_kit_storage::Session::new(session_id.clone());
        storage.store_session(&session).await.unwrap();

        // Store a user message
        let user_msg = UserMessage::new("Test message");
        let (storage_user_msg, user_parts) =
            user_message_to_storage(&storage, session_id.clone(), &user_msg);
        storage
            .store_user_message(&storage_user_msg, &user_parts)
            .await
            .unwrap();

        // Load history
        let history = load_conversation_history(&storage, &session_id)
            .await
            .unwrap();

        assert_eq!(history.len(), 1);
        assert!(matches!(history[0], Message::User(_)));

        if let Message::User(loaded_msg) = &history[0] {
            match &loaded_msg.content {
                llm_kit_provider_utils::message::UserContent::Text(text) => {
                    assert_eq!(text, "Test message");
                }
                _ => panic!("Expected text content"),
            }
        }
    }

    #[tokio::test]
    async fn test_multi_turn_conversation_history() {
        let (storage, _dir) = setup_storage().await;
        let session_id = storage.generate_session_id();

        // Create session
        let session = llm_kit_storage::Session::new(session_id.clone());
        storage.store_session(&session).await.unwrap();

        // Store 3 user messages
        for i in 1..=3 {
            let user_msg = UserMessage::new(format!("User message {}", i));
            let (storage_user_msg, user_parts) =
                user_message_to_storage(&storage, session_id.clone(), &user_msg);
            storage
                .store_user_message(&storage_user_msg, &user_parts)
                .await
                .unwrap();
        }

        // Load history
        let history = load_conversation_history(&storage, &session_id)
            .await
            .unwrap();

        // Should have 3 user messages
        assert_eq!(history.len(), 3);

        // Verify all are user messages
        for msg in &history {
            assert!(matches!(msg, Message::User(_)));
        }
    }

    #[tokio::test]
    async fn test_image_part_conversion_roundtrip() {
        use llm_kit_provider_utils::message::{DataContent, ImagePart as PromptImagePart};

        let (storage, _dir) = setup_storage().await;

        // Test Base64 image conversion
        let image_data = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==".to_string();
        let prompt_image = PromptImagePart::from_data(DataContent::Base64(image_data.clone()))
            .with_media_type("image/png");

        // Convert to storage
        let storage_part =
            image_part_to_storage(storage.generate_part_id(), &prompt_image).unwrap();

        // Convert back to prompt
        if let MessagePart::Image(image_storage) = storage_part {
            let roundtrip_image = storage_image_to_prompt(&image_storage).unwrap();
            assert_eq!(roundtrip_image.media_type, Some("image/png".to_string()));

            // Verify the image part was created successfully
            // Note: The internal structure may vary, so just verify basic properties
            assert!(roundtrip_image.media_type.is_some());
        } else {
            panic!("Expected Image part");
        }
    }

    #[tokio::test]
    async fn test_file_part_conversion_roundtrip() {
        use llm_kit_provider_utils::message::{DataContent, FilePart as PromptFilePart};

        let (storage, _dir) = setup_storage().await;

        // Test file conversion with filename
        let file_data = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello" in bytes
        let prompt_file = PromptFilePart::from_data(
            DataContent::Bytes(file_data.clone()),
            "text/plain".to_string(),
        )
        .with_filename("test.txt");

        // Convert to storage
        let storage_part = file_part_to_storage(storage.generate_part_id(), &prompt_file).unwrap();

        // Convert back to prompt
        if let MessagePart::File(file_storage) = storage_part {
            let roundtrip_file = storage_file_to_prompt(&file_storage).unwrap();
            assert_eq!(roundtrip_file.media_type, "text/plain");
            assert_eq!(roundtrip_file.filename, Some("test.txt".to_string()));

            // Basic verification that file part was created successfully
            assert!(!roundtrip_file.media_type.is_empty());
        } else {
            panic!("Expected File part");
        }
    }

    // Note: Tool call conversion testing requires GenerateTextResult with actual tool calls
    // This is better tested via end-to-end examples rather than unit tests

    #[tokio::test]
    async fn test_large_conversation_history() {
        let (storage, _dir) = setup_storage().await;
        let session_id = storage.generate_session_id();

        // Create session
        let session = llm_kit_storage::Session::new(session_id.clone());
        storage.store_session(&session).await.unwrap();

        // Store 50 user messages to test large history loading
        for i in 1..=50 {
            let user_msg = UserMessage::new(format!("Message number {}", i));
            let (storage_user_msg, user_parts) =
                user_message_to_storage(&storage, session_id.clone(), &user_msg);
            storage
                .store_user_message(&storage_user_msg, &user_parts)
                .await
                .unwrap();
        }

        // Load all history
        let history = load_conversation_history(&storage, &session_id)
            .await
            .unwrap();

        // Verify all 50 messages loaded
        assert_eq!(history.len(), 50);

        // Verify first and last messages
        if let Message::User(first) = &history[0] {
            match &first.content {
                llm_kit_provider_utils::message::UserContent::Text(text) => {
                    assert_eq!(text, "Message number 1");
                }
                _ => panic!("Expected text content"),
            }
        }

        if let Message::User(last) = &history[49] {
            match &last.content {
                llm_kit_provider_utils::message::UserContent::Text(text) => {
                    assert_eq!(text, "Message number 50");
                }
                _ => panic!("Expected text content"),
            }
        }
    }

    #[tokio::test]
    async fn test_message_ordering_preservation() {
        let (storage, _dir) = setup_storage().await;
        let session_id = storage.generate_session_id();

        // Create session
        let session = llm_kit_storage::Session::new(session_id.clone());
        storage.store_session(&session).await.unwrap();

        // Store messages with deliberate timing
        let messages = vec!["First", "Second", "Third"];

        for msg_text in messages {
            let user_msg = UserMessage::new(msg_text);
            let (storage_user_msg, user_parts) =
                user_message_to_storage(&storage, session_id.clone(), &user_msg);
            storage
                .store_user_message(&storage_user_msg, &user_parts)
                .await
                .unwrap();

            // Small delay to ensure different timestamps
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }

        // Load history
        let history = load_conversation_history(&storage, &session_id)
            .await
            .unwrap();

        assert_eq!(history.len(), 3);

        // Verify messages are in correct order
        let expected_order = ["First", "Second", "Third"];
        for (i, msg) in history.iter().enumerate() {
            if let Message::User(user) = msg {
                match &user.content {
                    llm_kit_provider_utils::message::UserContent::Text(text) => {
                        assert_eq!(text, expected_order[i]);
                    }
                    _ => panic!("Expected text content"),
                }
            }
        }
    }

    #[tokio::test]
    async fn test_empty_session_history() {
        let (storage, _dir) = setup_storage().await;
        let session_id = storage.generate_session_id();

        // Create session but don't store any messages
        let session = llm_kit_storage::Session::new(session_id.clone());
        storage.store_session(&session).await.unwrap();

        // Load history from empty session
        let history = load_conversation_history(&storage, &session_id)
            .await
            .unwrap();

        assert_eq!(history.len(), 0);
    }

    #[tokio::test]
    async fn test_multimodal_user_message_conversion() {
        use llm_kit_provider_utils::message::content_parts::TextPart;
        use llm_kit_provider_utils::message::{
            DataContent, ImagePart as PromptImagePart, UserContentPart,
        };

        let (storage, _dir) = setup_storage().await;
        let session_id = storage.generate_session_id();

        // Create user message with text and image
        let image = PromptImagePart::from_data(DataContent::Base64("base64data".to_string()))
            .with_media_type("image/png");

        let user_msg = UserMessage::with_parts(vec![
            UserContentPart::Text(TextPart::new("Check out this image")),
            UserContentPart::Image(image),
        ]);

        // Convert to storage
        let (_storage_msg, parts) =
            user_message_to_storage(&storage, session_id.clone(), &user_msg);

        // Should have 2 parts: text + image
        assert_eq!(parts.len(), 2);

        let text_parts: Vec<_> = parts
            .iter()
            .filter(|p| matches!(p, MessagePart::Text(_)))
            .collect();
        assert_eq!(text_parts.len(), 1);

        let image_parts: Vec<_> = parts
            .iter()
            .filter(|p| matches!(p, MessagePart::Image(_)))
            .collect();
        assert_eq!(image_parts.len(), 1);
    }
}
