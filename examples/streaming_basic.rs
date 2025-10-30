/// Basic streaming example demonstrating real-time text generation.
///
/// This example shows how to:
/// - Create a provider from environment variables
/// - Use stream_text to get streaming responses
/// - Consume text deltas in real-time
/// - Access final results after streaming completes
///
/// Run with:
/// ```bash
/// export OPENAI_API_KEY="your-api-key"
/// cargo run --example streaming_basic
/// ```
///
/// NOTE: This example should work with the completed Phase 2 implementation!
/// Basic single-step streaming is fully functional.
/// See STREAM_TEXT_IMPLEMENTATION_PLAN.md for current implementation status.

use ai_sdk_core::stream_text;
use ai_sdk_core::prompt::{Prompt, call_settings::CallSettings};
use ai_sdk_openai_compatible::{OpenAICompatibleProviderSettings, create_openai_compatible};
use futures_util::StreamExt;
use std::env;
use std::io::{self, Write as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AI SDK Rust - Streaming Basic Example\n");

    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(
        |_| "OPENAI_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create OpenAI provider
    let provider = create_openai_compatible(
        OpenAICompatibleProviderSettings::new("https://openrouter.ai/api/v1", "openai")
            .with_api_key(api_key),
    );

    println!("✓ Provider created: {}", provider.name());
    println!("✓ Base URL: {}\n", provider.base_url());

    // Get a language model
    let model = provider.chat_model("gpt-4o-mini");
    println!("✓ Model loaded: {}", model.model_id());
    println!("✓ Provider: {}\n", model.provider());

    // Create a prompt
    let prompt = Prompt::text("Write a short poem about Rust programming. Keep it under 100 words.");
    println!("📤 Sending prompt: \"Write a short poem about Rust programming. Keep it under 100 words.\"\n");

    // Configure settings
    let settings = CallSettings::default()
        .with_temperature(0.7)
        .with_max_output_tokens(200);

    // Stream text with optional callbacks
    println!("⏳ Streaming response...\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📝 Streaming text:\n");

    let result = stream_text(
        settings,
        prompt,
        &*model,
        None,        // tools
        None,        // tool_choice
        None,        // stop_when
        None,        // provider_options
        None,        // prepare_step
        false,       // include_raw_chunks
        None,        // on_chunk callback
        None,        // on_error callback
        None,        // on_step_finish callback
        None,        // on_finish callback
        None,        // on_abort callback
    )
    .await?;

    // Consume the text stream and display deltas in real-time
    let mut text_stream = result.text_stream();
    print!("   ");
    io::stdout().flush()?;

    while let Some(delta) = text_stream.next().await {
        print!("{}", delta);
        io::stdout().flush()?;
    }

    println!("\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Access final results after streaming completes
    println!("\n✅ Streaming completed!\n");
    println!("📊 Final Metadata:");

    let finish_reason = result.finish_reason().await?;
    println!("  • Finish reason: {:?}", finish_reason);

    let usage = result.usage().await?;
    println!("  • Input tokens: {}", usage.input_tokens);
    println!("  • Output tokens: {}", usage.output_tokens);
    println!("  • Total tokens: {}", usage.total_tokens);

    if usage.reasoning_tokens > 0 {
        println!("  • Reasoning tokens: {}", usage.reasoning_tokens);
    }

    if usage.cached_input_tokens > 0 {
        println!("  • Cached input tokens: {}", usage.cached_input_tokens);
    }

    // Get the complete text
    let complete_text = result.text().await?;
    println!("\n📄 Complete text ({} characters):", complete_text.len());
    println!("   {}", complete_text.replace('\n', "\n   "));

    println!("\n✅ Example completed successfully!");

    Ok(())
}
