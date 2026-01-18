//! Full conversation example with automatic storage and history loading.
//!
//! This example demonstrates Phase 3 storage integration:
//! - Automatic conversation history loading
//! - Automatic message storage after generation
//! - Multi-turn conversations
//! - Using `without_history()` for the first message
//!
//! Run with: `cargo run --example storage_conversation_full`

use llm_kit_core::{GenerateText, prompt::Prompt};
use llm_kit_openai_compatible::OpenAICompatibleClient;
use llm_kit_storage::Storage;
use llm_kit_storage_filesystem::FilesystemStorage;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AI SDK Storage - Full Conversation Example\n");

    // Initialize provider
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let provider = OpenAICompatibleClient::new()
        .base_url("https://openrouter.ai/api/v1")
        .api_key(api_key)
        .build();
    let model = provider.chat_model("openai/gpt-4o");

    // Initialize storage
    let storage_path = std::env::temp_dir().join("llm-kit-conversation-example");
    println!("📁 Storage path: {}\n", storage_path.display());

    let storage = Arc::new(FilesystemStorage::new(&storage_path)?);
    storage.initialize().await?;

    // Create a new session
    let session_id = storage.generate_session_id();
    println!("💬 Started conversation: {}\n", session_id);

    // First message - no history yet, use without_history()
    println!("👤 User: What is Rust?");
    let result1 = GenerateText::new(model.clone(), Prompt::text("What is Rust?"))
        .with_storage(storage.clone())
        .with_session_id(session_id.clone())
        .without_history() // First message, no history to load
        .execute()
        .await?;
    println!("🤖 Assistant: {}\n", result1.text);

    // Second message - automatically loads first Q&A as context
    println!("👤 User: Why should I learn it?");
    let result2 = GenerateText::new(model.clone(), Prompt::text("Why should I learn it?"))
        .with_storage(storage.clone())
        .with_session_id(session_id.clone())
        // History loading enabled by default - will include previous message
        .execute()
        .await?;
    println!("🤖 Assistant: {}\n", result2.text);

    // Third message - full context from all previous messages
    println!("👤 User: How do I get started?");
    let result3 = GenerateText::new(model.clone(), Prompt::text("How do I get started?"))
        .with_storage(storage.clone())
        .with_session_id(session_id.clone())
        .execute()
        .await?;
    println!("🤖 Assistant: {}\n", result3.text);

    // List conversation history
    println!("\n📜 === Conversation History ===");
    let message_ids = storage.list_messages(&session_id, None).await?;
    println!("Total messages: {}\n", message_ids.len());

    for (i, msg_id) in message_ids.iter().enumerate() {
        let (role, parts) = storage.get_message(&session_id, msg_id).await?;
        if let Some(llm_kit_storage::MessagePart::Text(text_part)) = parts.first() {
            println!("{}. {:?}: {}", i + 1, role, text_part.text);
        }
    }

    // Show session metadata
    println!("\n📊 === Session Info ===");
    let session = storage.get_session(&session_id).await?;
    println!("Session ID: {}", session.id);
    println!("Created: {}", session.created_at);
    println!("Updated: {}", session.updated_at);

    println!("\n✅ Example completed successfully!");
    println!("💡 Tip: Run again to start a new conversation, or modify to continue this one!");

    Ok(())
}
