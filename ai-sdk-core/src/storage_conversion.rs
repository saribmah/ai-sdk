//! Storage conversion utilities for ai-sdk-storage integration.
//!
//! This module provides conversion functions between `ai-sdk-core` types (prompts, outputs, messages)
//! and `ai-sdk-storage` types (StorageUserMessage, StorageAssistantMessage, MessagePart).
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
//! use ai_sdk_storage_filesystem::FilesystemStorage;
//! use ai_sdk_core::storage_conversion::*;
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
use crate::prompt::message::{AssistantMessage, Message, UserMessage};
use ai_sdk_storage::{
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
        crate::prompt::message::UserContent::Text(text) => {
            let part_id = storage.generate_part_id();
            parts.push(MessagePart::Text(TextPart::new(
                part_id.clone(),
                text.clone(),
            )));
            part_ids.push(part_id);
        }
        crate::prompt::message::UserContent::Parts(content_parts) => {
            for content in content_parts {
                let part_id = storage.generate_part_id();

                match content {
                    crate::prompt::message::UserContentPart::Text(text_part) => {
                        parts.push(MessagePart::Text(TextPart::new(
                            part_id.clone(),
                            text_part.text.clone(),
                        )));
                        part_ids.push(part_id);
                    }
                    crate::prompt::message::UserContentPart::Image(image_part) => {
                        if let Some(storage_part) =
                            image_part_to_storage(part_id.clone(), image_part)
                        {
                            parts.push(storage_part);
                            part_ids.push(part_id);
                        }
                    }
                    crate::prompt::message::UserContentPart::File(file_part) => {
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
                MessagePart::ToolResult(ai_sdk_storage::ToolResultPart::new_success(
                    part_id.clone(),
                    tool_result.tool_call_id.clone(),
                    tool_result.tool_name.clone(),
                    tool_result.output.clone(),
                ))
            }
            Output::ToolError(tool_error) => {
                // ToolError has error as JSON Value
                MessagePart::ToolResult(ai_sdk_storage::ToolResultPart::new_error(
                    part_id.clone(),
                    tool_error.tool_call_id.clone(),
                    tool_error.tool_name.clone(),
                    tool_error.error.to_string(),
                ))
            }
            // Skip source and file for now (not in storage types)
            _ => continue,
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
/// used by `GenerateText` and `StreamText`.
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
        let (role, parts) = storage.get_message(session_id, &message_id).await?;

        match role {
            MessageRole::User => {
                let message = parts_to_user_message(&parts);
                messages.push(Message::User(message));
            }
            MessageRole::Assistant => {
                let message = parts_to_assistant_message(&parts);
                messages.push(Message::Assistant(message));
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
                content_parts.push(crate::prompt::message::UserContentPart::Text(
                    crate::prompt::message::TextPart::new(text.text.clone()),
                ));
            }
            MessagePart::Image(image) => {
                if let Some(prompt_image) = storage_image_to_prompt(image) {
                    content_parts
                        .push(crate::prompt::message::UserContentPart::Image(prompt_image));
                }
            }
            MessagePart::File(file) => {
                if let Some(prompt_file) = storage_file_to_prompt(file) {
                    content_parts.push(crate::prompt::message::UserContentPart::File(prompt_file));
                }
            }
            _ => {
                // Skip other part types (tool calls, reasoning, etc.) in user messages
            }
        }
    }

    UserMessage::with_parts(content_parts)
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
                content_parts.push(crate::prompt::message::AssistantContentPart::Text(
                    crate::prompt::message::TextPart::new(text.text.clone()),
                ));
            }
            MessagePart::Reasoning(reasoning) => {
                content_parts.push(crate::prompt::message::AssistantContentPart::Reasoning(
                    crate::prompt::message::ReasoningPart::new(reasoning.content.clone()),
                ));
            }
            MessagePart::ToolCall(tool_call) => {
                content_parts.push(crate::prompt::message::AssistantContentPart::ToolCall(
                    crate::prompt::message::ToolCallPart::new(
                        tool_call.tool_call_id.clone(),
                        tool_call.tool_name.clone(),
                        tool_call.arguments.clone(),
                    ),
                ));
            }
            _ => {}
        }
    }

    AssistantMessage::with_parts(content_parts)
}

/// Convert a prompt ImagePart to storage format.
///
/// Returns `None` if the conversion cannot be performed (e.g., unsupported format).
fn image_part_to_storage(
    id: String,
    image: &crate::prompt::message::ImagePart,
) -> Option<MessagePart> {
    use crate::prompt::message::{DataContent, ImageSource};

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
    file: &crate::prompt::message::FilePart,
) -> Option<MessagePart> {
    use crate::prompt::message::{DataContent, FileSource};

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
fn storage_image_to_prompt(image: &StorageImagePart) -> Option<crate::prompt::message::ImagePart> {
    use crate::prompt::message::{DataContent, ImagePart as PromptImagePart};

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
fn storage_file_to_prompt(file: &StorageFilePart) -> Option<crate::prompt::message::FilePart> {
    use crate::prompt::message::{DataContent, FilePart as PromptFilePart};

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

#[cfg(all(test, feature = "storage"))]
mod tests {
    use super::*;

    use ai_sdk_storage_filesystem::FilesystemStorage;
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
        let session = ai_sdk_storage::Session::new(session_id.clone());
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
                crate::prompt::message::UserContent::Text(text) => {
                    assert_eq!(text, "Test message");
                }
                _ => panic!("Expected text content"),
            }
        }
    }
}
