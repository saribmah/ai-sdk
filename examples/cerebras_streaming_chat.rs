use futures_util::StreamExt;
use llm_kit_cerebras::CerebrasClient;
use llm_kit_core::StreamText;
use llm_kit_core::prompt::Prompt;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Streaming chat example with Cerebras.
///
/// This example shows how to:
/// - Create a Cerebras provider
/// - Use StreamText to get streaming responses
/// - Consume text deltas in real-time
/// - Access final results after streaming completes
///
/// Run with:
/// ```bash
/// export CEREBRAS_API_KEY="your-api-key"
/// cargo run --example cerebras_streaming_chat
/// ```
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AI SDK Rust - Cerebras Streaming Chat Example\n");

    // Load environment variables from .env file if present
    dotenvy::dotenv().ok();

    // Get API key from environment
    let api_key = env::var("CEREBRAS_API_KEY").map_err(
        |_| "CEREBRAS_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create Cerebras provider
    let provider = CerebrasClient::new().api_key(api_key).build();

    println!("✓ Provider created: {}", provider.name());
    println!("✓ Base URL: {}\n", provider.base_url());

    // Get a language model
    let model = provider.chat_model("llama-3.3-70b");
    println!("✓ Model loaded: {}", model.model_id());
    println!("✓ Provider: {}\n", model.provider());

    // Create a prompt
    let prompt = Prompt::text("Write a short poem about Rust programming. Make it 4 lines.");
    println!(
        "📤 Sending prompt: \"Write a short poem about Rust programming. Make it 4 lines.\"\n"
    );

    // Stream text with callbacks to capture metadata
    println!("⏳ Streaming response...\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    print!("📝 ");

    // Use Arc to share metadata between callback and main thread
    let metadata = Arc::new(Mutex::new(None));
    let metadata_clone = metadata.clone();

    let result = StreamText::new(model, prompt)
        .temperature(0.7)
        .max_output_tokens(200)
        .on_finish(Box::new(move |event| {
            let metadata = metadata_clone.clone();
            Box::pin(async move {
                let mut meta = metadata.lock().await;
                *meta = Some((
                    event.step_result.finish_reason.clone(),
                    event.step_result.usage,
                    event.total_usage,
                ));
            })
        }))
        .execute()
        .await?;

    // Stream text deltas
    let mut text_stream = result.text_stream();
    while let Some(text_delta) = text_stream.next().await {
        print!("{}", text_delta);
        std::io::Write::flush(&mut std::io::stdout())?;
    }

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Access metadata captured from on_finish callback
    println!("\n📊 Metadata:");
    if let Some((finish_reason, usage, total_usage)) = metadata.lock().await.as_ref() {
        println!("  • Finish reason: {:?}", finish_reason);
        println!("  • Input tokens: {}", usage.input_tokens);
        println!("  • Output tokens: {}", usage.output_tokens);
        println!("  • Total tokens: {}", usage.total_tokens);
        println!("  • Total usage (all steps): {}", total_usage.total_tokens);

        if usage.reasoning_tokens > 0 {
            println!("  • Reasoning tokens: {}", usage.reasoning_tokens);
        }

        if usage.cached_input_tokens > 0 {
            println!("  • Cached input tokens: {}", usage.cached_input_tokens);
        }
    }

    println!("\n✅ Example completed successfully!");

    Ok(())
}
