use llm_kit_storage::{
    AssistantMessage, MessageMetadata, MessagePart, Session, Storage, TextPart, UsageStats,
    UserMessage,
};
use llm_kit_storage_filesystem::FilesystemStorage;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let storage = Arc::new(FilesystemStorage::new("./test-storage")?);
    storage.initialize().await?;

    // Create session
    let session_id = storage.generate_session_id();
    let session = Session::new(session_id.clone()).with_title("Conversation Test".to_string());
    storage.store_session(&session).await?;

    // Simulate a conversation
    let conversations = vec![
        ("What is Rust?", "Rust is a systems programming language..."),
        (
            "Why should I use it?",
            "Rust provides memory safety without garbage collection...",
        ),
        (
            "How do I get started?",
            "You can start by installing Rust using rustup...",
        ),
    ];

    for (user_text, assistant_text) in conversations {
        // Store user message
        let user_msg_id = storage.generate_message_id();
        let user_part_id = storage.generate_part_id();
        let user_part =
            MessagePart::Text(TextPart::new(user_part_id.clone(), user_text.to_string()));
        let user_msg =
            UserMessage::new(user_msg_id.clone(), session_id.clone(), vec![user_part_id]);
        storage.store_user_message(&user_msg, &[user_part]).await?;

        // Store assistant message
        let asst_msg_id = storage.generate_message_id();
        let asst_part_id = storage.generate_part_id();
        let asst_part = MessagePart::Text(TextPart::new(
            asst_part_id.clone(),
            assistant_text.to_string(),
        ));
        let asst_msg =
            AssistantMessage::new(asst_msg_id.clone(), session_id.clone(), vec![asst_part_id])
                .with_metadata(MessageMetadata {
                    model_id: Some("gpt-4".to_string()),
                    provider: Some("openai".to_string()),
                    usage: Some(UsageStats::new(100, 50)),
                    finish_reason: Some("stop".to_string()),
                    custom: None,
                });
        storage
            .store_assistant_message(&asst_msg, &[asst_part])
            .await?;
    }

    // List all messages
    let message_ids = storage.list_messages(&session_id, None).await?;
    println!("Conversation history ({} messages):", message_ids.len());

    for msg_id in message_ids {
        let (role, parts) = storage.get_message(&session_id, &msg_id).await?;
        if let Some(MessagePart::Text(text_part)) = parts.first() {
            println!("  {:?}: {}", role, text_part.text);
        }
    }

    // Clean up
    storage.delete_session(&session_id).await?;

    Ok(())
}
