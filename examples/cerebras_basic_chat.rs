use llm_kit_cerebras::CerebrasClient;
use llm_kit_core::GenerateText;
use llm_kit_core::prompt::Prompt;
use std::env;

/// Basic chat example with Cerebras.
///
/// This example shows how to:
/// - Create a Cerebras provider from environment variables
/// - Use GenerateText to get responses
/// - Handle the response and display metadata
///
/// Run with:
/// ```bash
/// export CEREBRAS_API_KEY="your-api-key"
/// cargo run --example cerebras_basic_chat
/// ```
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AI SDK Rust - Cerebras Basic Chat Example\n");

    // Load environment variables from .env file if present
    dotenvy::dotenv().ok();

    // Get API key from environment
    let api_key = env::var("CEREBRAS_API_KEY").map_err(
        |_| "CEREBRAS_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create Cerebras provider using the client builder
    let provider = CerebrasClient::new().api_key(api_key).build();

    println!("✓ Provider created: {}", provider.name());
    println!("✓ Base URL: {}\n", provider.base_url());

    // Get a language model (returns Arc<dyn LanguageModel>)
    let model = provider.chat_model("llama-3.3-70b");
    println!("✓ Model loaded: {}", model.model_id());
    println!("✓ Provider: {}\n", model.provider());

    // Create a prompt
    let prompt = Prompt::text("What is the capital of France? Answer in one sentence.");
    println!("📤 Sending prompt: \"What is the capital of France? Answer in one sentence.\"\n");

    // Generate text using the builder pattern
    println!("⏳ Generating response...\n");
    let result = GenerateText::new(model, prompt)
        .temperature(0.7)
        .max_output_tokens(50)
        .execute()
        .await?;

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
        println!(
            "  • Cached input tokens: {}",
            result.usage.cached_input_tokens
        );
    }

    if let Some(id) = &result.response.id {
        println!("  • Response ID: {}", id);
    }
    if let Some(model_id) = &result.response.model_id {
        println!("  • Model ID: {}", model_id);
    }

    println!("\n✅ Example completed successfully!");

    Ok(())
}
