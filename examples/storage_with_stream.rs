//! Storage integration with StreamText example.
//!
//! This example demonstrates how to automatically store streamed conversations
//! when using StreamText with the filesystem storage provider.
//!
//! Run with:
//! ```bash
//! export OPENAI_API_KEY="your-api-key"
//! cargo run --example storage_with_stream
//! ```

use ai_sdk_core::StreamText;
use ai_sdk_core::prompt::Prompt;
use ai_sdk_openai_compatible::OpenAICompatibleClient;
use ai_sdk_storage::StorageProvider;
use ai_sdk_storage_filesystem::FilesystemStorageProvider;
use futures_util::StreamExt;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AI SDK - Storage with StreamText Example\n");

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
    let storage_path = std::env::temp_dir().join("ai-sdk-storage-stream-example");
    println!("📁 Storage path: {}", storage_path.display());

    let storage = Arc::new(FilesystemStorageProvider::new(&storage_path)?);
    storage.initialize().await?;
    println!("✓ Storage initialized\n");

    // Create a session ID for this conversation
    let session_id = format!("stream-session-{}", chrono::Utc::now().timestamp());
    println!("💬 Session ID: {}\n", session_id);

    // Stream a response with storage
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Streaming Question: Tell me a short story about Rust");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt = Prompt::text("Tell me a very short 2-sentence story about a Rust programmer.");

    let result = StreamText::new(model.clone(), prompt)
        .with_storage(storage.clone())
        .with_session_id(session_id.clone())
        .temperature(0.8)
        .execute()
        .await?;

    println!("🤖 Response (streaming):\n");

    let mut stream = result.text_stream();
    let mut full_text = String::new();

    while let Some(text) = stream.next().await {
        print!("{}", text);
        full_text.push_str(&text);
        // Flush stdout to see the streaming effect
        use std::io::Write;
        std::io::stdout().flush().unwrap();
    }

    println!("\n\n✓ Stream completed");
    println!("✓ Messages automatically stored to session\n");

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

        println!("{}. {} ({})", i + 1, role, msg.id);
        if let Some(text) = msg.content.get("text").and_then(|v| v.as_str()) {
            println!("   {}\n", text);
        }

        // Show metadata for assistant messages
        if matches!(msg.role, ai_sdk_storage::MessageRole::Assistant)
            && let Some(usage) = &msg.metadata.usage
        {
            println!("   📊 Token usage:");
            if let Some(prompt_tokens) = usage.prompt_tokens {
                println!("      Prompt: {}", prompt_tokens);
            }
            if let Some(completion_tokens) = usage.completion_tokens {
                println!("      Completion: {}", completion_tokens);
            }
            if let Some(total_tokens) = usage.total_tokens {
                println!("      Total: {}", total_tokens);
            }
            println!();
        }
    }

    // Stream another response to the same session
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Streaming Follow-up: What makes Rust special?");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt2 = Prompt::text("What makes Rust special? Answer in one sentence.");

    let result2 = StreamText::new(model.clone(), prompt2)
        .with_storage(storage.clone())
        .with_session_id(session_id.clone())
        .temperature(0.7)
        .execute()
        .await?;

    println!("🤖 Response (streaming):\n");

    let mut stream2 = result2.text_stream();

    while let Some(text) = stream2.next().await {
        print!("{}", text);
        use std::io::Write;
        std::io::stdout().flush().unwrap();
    }

    println!("\n\n✓ Second stream completed\n");

    // Show updated conversation history
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Updated Conversation History");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let messages = conversation_storage.get_messages(&session_id, None).await?;

    println!("📜 Now {} messages in session:\n", messages.len());
    for (i, msg) in messages.iter().enumerate() {
        let role = match msg.role {
            ai_sdk_storage::MessageRole::User => "👤",
            ai_sdk_storage::MessageRole::Assistant => "🤖",
            ai_sdk_storage::MessageRole::System => "⚙️",
            ai_sdk_storage::MessageRole::Tool => "🔧",
        };
        println!("{}. {} {:?}", i + 1, role, msg.role);
    }
    println!();

    // Cleanup
    println!("🧹 Cleaning up...");
    conversation_storage.delete_session(&session_id).await?;
    println!("✓ Session deleted\n");

    println!("✨ Example completed successfully!");
    println!("\n💡 Key takeaways:");
    println!("   • StreamText supports automatic storage just like GenerateText");
    println!("   • Messages are stored after the stream completes");
    println!("   • Full conversation context is preserved across multiple streams");
    println!("   • Token usage and metadata are automatically captured");

    Ok(())
}
