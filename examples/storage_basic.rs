//! Basic storage example demonstrating filesystem storage operations.
//!
//! This example shows how to:
//! - Create a filesystem storage provider
//! - Create conversation sessions
//! - Store messages manually
//! - Retrieve conversation history
//! - List sessions
//! - Delete sessions

use chrono::Utc;
use llm_kit_storage::{
    AssistantMessage, MessageMetadata, MessagePart, Session, SessionMetadata, Storage, TextPart,
    UsageStats, UserMessage,
};
use llm_kit_storage_filesystem::FilesystemStorage;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🗄️  AI SDK Filesystem Storage Example\n");

    // Create a filesystem storage provider in a temporary directory
    let storage_path = std::env::temp_dir().join("llm-kit-storage-example");
    println!("📁 Storage path: {}\n", storage_path.display());

    let storage = Arc::new(FilesystemStorage::new(&storage_path)?);
    storage.initialize().await?;

    // Create a conversation session
    println!("1️⃣  Creating conversation session...");
    let session_id = storage.generate_session_id();
    let session = Session {
        id: session_id.clone(),
        title: Some("My First Conversation".to_string()),
        metadata: SessionMetadata {
            user_id: Some("user-123".to_string()),
            tags: Some(vec!["example".to_string(), "tutorial".to_string()]),
            custom: None,
        },
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    storage.store_session(&session).await?;
    println!("✅ Session created: {}\n", session_id);

    // Store some messages
    println!("2️⃣  Storing messages...");

    // User message
    let user_msg_id = storage.generate_message_id();
    let user_part_id = storage.generate_part_id();
    let user_text_part = TextPart::new(
        user_part_id.clone(),
        "Hello! Can you help me understand Rust's ownership system?".to_string(),
    );
    let user_message =
        UserMessage::new(user_msg_id.clone(), session_id.clone(), vec![user_part_id]);

    storage
        .store_user_message(&user_message, &[MessagePart::Text(user_text_part)])
        .await?;
    println!("✅ User message stored");

    // Simulate a delay
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Assistant message
    let assistant_msg_id = storage.generate_message_id();
    let assistant_part_id = storage.generate_part_id();
    let assistant_text_part = TextPart::new(
        assistant_part_id.clone(),
        "I'd be happy to help! Rust's ownership system is one of its most distinctive features..."
            .to_string(),
    );

    let metadata = MessageMetadata {
        model_id: Some("gpt-4".to_string()),
        provider: Some("openai".to_string()),
        usage: Some(UsageStats {
            prompt_tokens: 20,
            completion_tokens: 50,
            total_tokens: 70,
        }),
        finish_reason: Some("stop".to_string()),
        custom: None,
    };

    let assistant_message = AssistantMessage::new(
        assistant_msg_id.clone(),
        session_id.clone(),
        vec![assistant_part_id],
    )
    .with_metadata(metadata);

    storage
        .store_assistant_message(
            &assistant_message,
            &[MessagePart::Text(assistant_text_part)],
        )
        .await?;
    println!("✅ Assistant message stored\n");

    // Retrieve conversation history
    println!("3️⃣  Retrieving conversation history...");
    let message_ids = storage.list_messages(&session_id, None).await?;

    println!("📜 Conversation has {} messages:", message_ids.len());
    for (i, msg_id) in message_ids.iter().enumerate() {
        let (role, parts) = storage.get_message(&session_id, msg_id).await?;
        if let Some(MessagePart::Text(text_part)) = parts.first() {
            println!("   {}. {:?}: {}", i + 1, role, text_part.text);
        }
    }
    println!();

    // Create another session
    println!("4️⃣  Creating another session...");
    let session_id2 = storage.generate_session_id();
    let session2 = Session {
        id: session_id2.clone(),
        title: Some("Learning about async/await".to_string()),
        metadata: SessionMetadata {
            user_id: Some("user-123".to_string()),
            tags: Some(vec!["async".to_string(), "rust".to_string()]),
            custom: None,
        },
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    storage.store_session(&session2).await?;
    println!("✅ Second session created\n");

    // List all sessions
    println!("5️⃣  Listing all sessions...");
    let sessions = storage.list_sessions(None).await?;
    println!("📋 Found {} sessions:", sessions.len());
    for (i, session) in sessions.iter().enumerate() {
        println!(
            "   {}. {} - {}",
            i + 1,
            session.id,
            session.title.as_deref().unwrap_or("(no title)")
        );
    }
    println!();

    // Delete a session
    println!("6️⃣  Deleting first session...");
    storage.delete_session(&session_id).await?;
    println!("✅ Session deleted\n");

    // Verify deletion
    println!("7️⃣  Verifying deletion...");
    let sessions = storage.list_sessions(None).await?;
    println!("📋 Remaining sessions: {}", sessions.len());
    for session in sessions.iter() {
        println!("   - {}", session.id);
    }
    println!();

    // Cleanup
    println!("🧹 Cleaning up...");
    storage.delete_session(&session_id2).await?;
    println!("✅ All sessions cleaned up\n");

    println!("✨ Example completed successfully!");

    Ok(())
}
