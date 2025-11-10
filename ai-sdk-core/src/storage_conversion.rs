//! Storage conversion utilities for ai-sdk-storage integration.
//!
//! This module provides conversion functions between `ai-sdk-core` types (prompts, outputs, messages)
//! and `ai-sdk-storage` types (StorageUserMessage, StorageAssistantMessage, MessagePart).
//!
//! # Key Functions
//!
//! - [`user_message_to_storage`]: Convert user messages from prompts to storage format
//! - [`assistant_output_to_storage`]: Convert assistant outputs to storage format
//! - [`load_conversation_history`]: Load and convert stored messages back to prompt format
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
    AssistantMessage as StorageAssistantMessage, MessageMetadata, MessagePart, MessageRole,
    ReasoningPart, Storage, StorageError, TextPart, ToolCallPart, UsageStats,
    UserMessage as StorageUserMessage,
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
/// A tuple of (StorageUserMessage, Vec<MessagePart>) ready to be stored
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

                if let crate::prompt::message::UserContentPart::Text(text_part) = content {
                    parts.push(MessagePart::Text(TextPart::new(
                        part_id.clone(),
                        text_part.text.clone(),
                    )));
                    part_ids.push(part_id);
                }
                // TODO: Handle image and file parts when needed
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
/// A tuple of (StorageAssistantMessage, Vec<MessagePart>) ready to be stored
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
        if let MessagePart::Text(text) = part {
            content_parts.push(crate::prompt::message::UserContentPart::Text(
                crate::prompt::message::TextPart::new(text.text.clone()),
            ));
        }
        // TODO: Handle other part types (images, files) when needed
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
