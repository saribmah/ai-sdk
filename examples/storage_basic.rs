//! Basic storage example demonstrating filesystem storage operations.
//!
//! This example shows how to:
//! - Create a filesystem storage provider
//! - Create conversation sessions
//! - Store messages manually
//! - Retrieve conversation history
//! - List sessions
//! - Delete sessions

use ai_sdk_storage::{
    ConversationSession, MessageMetadata, MessageRole, SessionMetadata, StorageProvider,
    StoredMessage,
};
use ai_sdk_storage_filesystem::FilesystemStorageProvider;
use chrono::Utc;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🗄️  AI SDK Filesystem Storage Example\n");

    // Create a filesystem storage provider in a temporary directory
    let storage_path = std::env::temp_dir().join("ai-sdk-storage-example");
    println!("📁 Storage path: {}\n", storage_path.display());

    let provider = Arc::new(FilesystemStorageProvider::new(&storage_path)?);
    provider.initialize().await?;

    let storage = provider.conversation_storage();

    // Create a conversation session
    println!("1️⃣  Creating conversation session...");
    let session = ConversationSession {
        id: "example-session-1".to_string(),
        title: Some("My First Conversation".to_string()),
        metadata: SessionMetadata {
            user_id: Some("user-123".to_string()),
            tags: Some(vec!["example".to_string(), "tutorial".to_string()]),
            custom: None,
        },
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    storage.create_session(session).await?;
    println!("✅ Session created: example-session-1\n");

    // Store some messages
    println!("2️⃣  Storing messages...");

    let user_message = StoredMessage {
        id: "msg-1".to_string(),
        session_id: "example-session-1".to_string(),
        role: MessageRole::User,
        content: serde_json::json!({
            "text": "Hello! Can you help me understand Rust's ownership system?"
        }),
        metadata: MessageMetadata {
            model_id: None,
            provider: None,
            usage: None,
            finish_reason: None,
            tool_calls: None,
            custom: None,
        },
        created_at: Utc::now(),
    };

    storage.store_message(user_message).await?;
    println!("✅ User message stored");

    // Simulate a delay
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    let assistant_message = StoredMessage {
        id: "msg-2".to_string(),
        session_id: "example-session-1".to_string(),
        role: MessageRole::Assistant,
        content: serde_json::json!({
            "text": "I'd be happy to help! Rust's ownership system is one of its most distinctive features..."
        }),
        metadata: MessageMetadata {
            model_id: Some("gpt-4".to_string()),
            provider: Some("openai".to_string()),
            usage: Some(ai_sdk_storage::UsageStats {
                prompt_tokens: Some(20),
                completion_tokens: Some(50),
                total_tokens: Some(70),
            }),
            finish_reason: Some("stop".to_string()),
            tool_calls: None,
            custom: None,
        },
        created_at: Utc::now(),
    };

    storage.store_message(assistant_message).await?;
    println!("✅ Assistant message stored\n");

    // Retrieve conversation history
    println!("3️⃣  Retrieving conversation history...");
    let messages = storage.get_messages("example-session-1", None).await?;

    println!("📜 Conversation has {} messages:", messages.len());
    for (i, msg) in messages.iter().enumerate() {
        println!(
            "   {}. {:?}: {}",
            i + 1,
            msg.role,
            msg.content
                .get("text")
                .and_then(|v| v.as_str())
                .unwrap_or("N/A")
        );
    }
    println!();

    // Create another session
    println!("4️⃣  Creating another session...");
    let session2 = ConversationSession {
        id: "example-session-2".to_string(),
        title: Some("Learning about async/await".to_string()),
        metadata: SessionMetadata {
            user_id: Some("user-123".to_string()),
            tags: Some(vec!["async".to_string(), "rust".to_string()]),
            custom: None,
        },
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    storage.create_session(session2).await?;
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
    storage.delete_session("example-session-1").await?;
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
    storage.delete_session("example-session-2").await?;
    println!("✅ All sessions cleaned up\n");

    println!("✨ Example completed successfully!");

    Ok(())
}
