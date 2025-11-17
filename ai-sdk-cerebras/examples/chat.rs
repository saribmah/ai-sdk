/// Basic chat example using ai-sdk-provider traits only.
///
/// This example demonstrates direct usage of the LanguageModel trait
/// without ai-sdk-core abstractions.
///
/// Run with:
/// ```bash
/// export CEREBRAS_API_KEY="your-api-key"
/// cargo run --example chat
/// ```
use ai_sdk_cerebras::CerebrasClient;
use ai_sdk_provider::language_model::call_options::LanguageModelCallOptions;
use ai_sdk_provider::language_model::prompt::LanguageModelMessage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 Cerebras Chat Example (Provider Traits)\n");

    // Load environment variables
    dotenvy::dotenv().ok();

    // Get API key from environment
    let api_key = std::env::var("CEREBRAS_API_KEY")
        .map_err(|_| "CEREBRAS_API_KEY environment variable not set")?;

    println!("✓ API key loaded from environment");

    // Create provider using builder
    let provider = CerebrasClient::new().api_key(api_key).build();

    println!("✓ Provider created: {}", provider.name());
    println!("✓ Base URL: {}\n", provider.base_url());

    // Get a language model
    let model = provider.chat_model("llama-3.3-70b");
    println!("✓ Model: {}", model.model_id());
    println!("✓ Provider: {}\n", model.provider());

    // Create a simple prompt using provider types
    let messages = vec![LanguageModelMessage::user_text(
        "What is the capital of France? Answer in one sentence.",
    )];

    println!("📤 Sending prompt...\n");

    // Call do_generate directly (provider trait method)
    let options = LanguageModelCallOptions::new(messages)
        .with_temperature(0.7)
        .with_max_output_tokens(100);

    let result = model.do_generate(options).await?;

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

    println!("\n✅ Example completed successfully!");

    Ok(())
}
