//! Storage integration with GenerateText example.
//!
//! This example demonstrates how to automatically store conversations
//! when using GenerateText with the filesystem storage provider.
//!
//! Run with:
//! ```bash
//! export OPENAI_API_KEY="your-api-key"
//! cargo run --example storage_with_generate
//! ```

use ai_sdk_core::GenerateText;
use ai_sdk_core::prompt::Prompt;
use ai_sdk_openai_compatible::OpenAICompatibleClient;
use ai_sdk_storage::StorageProvider;
use ai_sdk_storage_filesystem::FilesystemStorageProvider;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AI SDK - Storage with GenerateText Example\n");

    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(
        |_| "OPENAI_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment\n");

    // Create OpenAI provider
    let provider = OpenAICompatibleClient::new()
        .base_url("https://openrouter.ai/api/v1")
        .api_key(api_key)
        .build();

    let model = provider.chat_model("gpt-4o-mini");
    println!("✓ Model: {}\n", model.model_id());

    // Create filesystem storage provider
    let storage_path = std::env::temp_dir().join("ai-sdk-storage-generate-example");
    println!("📁 Storage path: {}", storage_path.display());

    let storage = Arc::new(FilesystemStorageProvider::new(&storage_path)?);
    storage.initialize().await?;
    println!("✓ Storage initialized\n");

    // Create a session ID for this conversation
    let session_id = format!("session-{}", chrono::Utc::now().timestamp());
    println!("💬 Session ID: {}\n", session_id);

    // First question with storage
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Question 1: What is Rust?");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt1 = Prompt::text("What is Rust programming language? Answer in 2 sentences.");

    let result1 = GenerateText::new(model.clone(), prompt1)
        .with_storage(storage.clone())
        .with_session_id(session_id.clone())
        .temperature(0.7)
        .execute()
        .await?;

    println!("🤖 Response: {}\n", result1.text);
    println!("✓ Message automatically stored to session\n");

    // Second question with the same storage and session
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Question 2: What is ownership?");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt2 = Prompt::text("What is ownership in Rust? Answer in 2 sentences.");

    let result2 = GenerateText::new(model.clone(), prompt2)
        .with_storage(storage.clone())
        .with_session_id(session_id.clone())
        .temperature(0.7)
        .execute()
        .await?;

    println!("🤖 Response: {}\n", result2.text);
    println!("✓ Message automatically stored to session\n");

    // Retrieve and display conversation history
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Retrieving Conversation History");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let conversation_storage = storage.conversation_storage();
    let messages = conversation_storage.get_messages(&session_id, None).await?;

    println!("📜 Stored {} messages in this session:\n", messages.len());

    for (i, msg) in messages.iter().enumerate() {
        let role = match msg.role {
            ai_sdk_storage::MessageRole::User => "👤 User",
            ai_sdk_storage::MessageRole::Assistant => "🤖 Assistant",
            ai_sdk_storage::MessageRole::System => "⚙️  System",
            ai_sdk_storage::MessageRole::Tool => "🔧 Tool",
        };

        println!("{}. {}", i + 1, role);
        if let Some(text) = msg.content.get("text").and_then(|v| v.as_str()) {
            println!("   {}\n", text);
        }
    }

    // List all sessions
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("All Sessions");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let sessions = conversation_storage.list_sessions(None).await?;
    println!("📋 Total sessions: {}\n", sessions.len());

    for session in sessions.iter() {
        println!(
            "  • {} - {}",
            session.id,
            session.title.as_deref().unwrap_or("(no title)")
        );
        println!(
            "    Created: {}",
            session.created_at.format("%Y-%m-%d %H:%M:%S")
        );
        println!(
            "    Updated: {}",
            session.updated_at.format("%Y-%m-%d %H:%M:%S")
        );
        println!();
    }

    // Cleanup
    println!("🧹 Cleaning up...");
    conversation_storage.delete_session(&session_id).await?;
    println!("✓ Session deleted\n");

    println!("✨ Example completed successfully!");
    println!("\n💡 Key takeaways:");
    println!("   • Use .with_storage() to enable automatic storage");
    println!("   • Use .with_session_id() to group messages in a session");
    println!("   • Messages are stored automatically after generation");
    println!("   • Retrieve conversation history anytime with get_messages()");

    Ok(())
}
