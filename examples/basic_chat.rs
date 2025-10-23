/// Basic chat example demonstrating text generation with OpenAI-compatible providers.
///
/// This example shows how to:
/// - Create a provider from environment variables
/// - Use generate_text to get responses
/// - Handle the response properly
///
/// Run with:
/// ```bash
/// export OPENAI_API_KEY="your-api-key"
/// cargo run --example basic_chat
/// ```

use ai_sdk_core::generate_text;
use ai_sdk_core::prompt::{call_settings::CallSettings, Prompt};
use openai_compatible::{create_openai_compatible, OpenAICompatibleProviderSettings};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AI SDK Rust - Basic Chat Example\n");

    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        "OPENAI_API_KEY environment variable not set. Please set it with your API key."
    })?;

    println!("✓ API key loaded from environment");

    // Create OpenAI provider
    let provider = create_openai_compatible(
        OpenAICompatibleProviderSettings::new("https://api.openai.com/v1", "openai")
            .with_api_key(api_key),
    );

    println!("✓ Provider created: {}", provider.name());
    println!("✓ Base URL: {}\n", provider.base_url());

    // Get a language model
    let model = provider.chat_model("gpt-4o-mini");
    println!("✓ Model loaded: {}", model.model_id());
    println!("✓ Provider: {}\n", model.provider());

    // Create a prompt
    let prompt = Prompt::text("What is the capital of France? Answer in one sentence.");
    println!("📤 Sending prompt: \"What is the capital of France? Answer in one sentence.\"\n");

    // Configure settings
    let settings = CallSettings::default()
        .with_temperature(0.7)
        .with_max_output_tokens(100);

    // Generate text
    println!("⏳ Generating response...\n");
    let result = generate_text(&*model, prompt, settings, None, None, None).await?;

    // Display the response
    println!("✅ Response received!\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📝 Content:");
    for (i, content) in result.content.iter().enumerate() {
        println!("  [{}] {:?}", i + 1, content);
    }
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    println!("\n📊 Metadata:");
    println!("  • Finish reason: {:?}", result.finish_reason);
    println!("  • Input tokens: {}", result.usage.input_tokens);
    println!("  • Output tokens: {}", result.usage.output_tokens);
    println!("  • Total tokens: {}", result.usage.total_tokens);

    if result.usage.reasoning_tokens > 0 {
        println!("  • Reasoning tokens: {}", result.usage.reasoning_tokens);
    }

    if result.usage.cached_input_tokens > 0 {
        println!("  • Cached input tokens: {}", result.usage.cached_input_tokens);
    }

    if let Some(response_meta) = &result.response {
        if let Some(id) = &response_meta.id {
            println!("  • Response ID: {}", id);
        }
        if let Some(model_id) = &response_meta.model_id {
            println!("  • Model ID: {}", model_id);
        }
    }

    println!("\n✅ Example completed successfully!");

    Ok(())
}
