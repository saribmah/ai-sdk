use ai_sdk_storage::{MessagePart, Session, SessionMetadata, Storage, TextPart, UserMessage};
use ai_sdk_storage_filesystem::FilesystemStorage;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create storage
    let storage = Arc::new(FilesystemStorage::new("./test-storage")?);
    storage.initialize().await?;

    // Create a session
    let session_id = storage.generate_session_id();
    let session = Session::new(session_id.clone())
        .with_title("My First Session".to_string())
        .with_metadata(SessionMetadata::with_user_id("user-123".to_string()));

    storage.store_session(&session).await?;
    println!("Created session: {}", session_id);

    // Add a message
    let message_id = storage.generate_message_id();
    let part_id = storage.generate_part_id();

    let text_part = TextPart::new(part_id.clone(), "Hello, AI!".to_string());
    let parts = vec![MessagePart::Text(text_part)];
    let message = UserMessage::new(message_id.clone(), session_id.clone(), vec![part_id]);

    storage.store_user_message(&message, &parts).await?;
    println!("Stored message: {}", message_id);

    // Retrieve the message
    let (role, retrieved_parts) = storage.get_message(&session_id, &message_id).await?;
    println!("Retrieved message with role: {:?}", role);
    println!("Parts: {}", retrieved_parts.len());

    // List all sessions
    let sessions = storage.list_sessions(None).await?;
    println!("\nAll sessions ({}):", sessions.len());
    for s in sessions {
        println!(
            "  - {} ({})",
            s.id,
            s.title.unwrap_or("Untitled".to_string())
        );
    }

    // Clean up
    storage.delete_session(&session_id).await?;
    println!("\nDeleted session: {}", session_id);

    Ok(())
}
