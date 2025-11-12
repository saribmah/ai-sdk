/// Simple chat example to test Anthropic provider with tool integration.
///
/// Run with:
/// ```bash
/// export ANTHROPIC_API_KEY="your-api-key"
/// cargo run --example simple_chat
/// ```
use ai_sdk_anthropic::{AnthropicProviderSettings, create_anthropic};
use ai_sdk_core::GenerateText;
use ai_sdk_core::prompt::Prompt;
use ai_sdk_provider::language_model::LanguageModel;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 Anthropic Simple Chat Example\n");

    // Get API key from environment
    let api_key = env::var("ANTHROPIC_API_KEY").map_err(
        |_| "ANTHROPIC_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded");

    // Create Anthropic provider
    let settings = AnthropicProviderSettings::new().with_api_key(api_key);
    let provider = create_anthropic(settings);
    let model = Arc::new(provider.language_model("claude-3-haiku-20240307".to_string()));

    println!("✓ Model loaded: {}\n", model.model_id());

    // Example 1: Simple text generation
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 1: Simple Question");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt1 = Prompt::text("What is the capital of France? Answer in one sentence.");
    println!("💬 User: What is the capital of France?\n");

    let result1 = GenerateText::new(model.clone(), prompt1)
        .max_output_tokens(100)
        .execute()
        .await?;

    println!("🤖 Assistant: {}\n", result1.text);
    println!(
        "📊 Usage: {} input tokens, {} output tokens",
        result1.usage.input_tokens, result1.usage.output_tokens
    );
    println!("🏁 Finish reason: {:?}\n", result1.finish_reason);

    // Example 2: With temperature
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: Creative Response");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt2 = Prompt::text("Write a haiku about programming.");
    println!("💬 User: Write a haiku about programming.\n");

    let result2 = GenerateText::new(model.clone(), prompt2)
        .temperature(0.9)
        .max_output_tokens(200)
        .execute()
        .await?;

    println!("🤖 Assistant:\n{}\n", result2.text);
    println!(
        "📊 Usage: {} input tokens, {} output tokens\n",
        result2.usage.input_tokens, result2.usage.output_tokens
    );

    // Example 3: Multi-turn conversation
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 3: Longer Response");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt3 = Prompt::text("Explain what Rust's ownership system is in 2-3 sentences.");
    println!("💬 User: Explain Rust's ownership system.\n");

    let result3 = GenerateText::new(model, prompt3)
        .max_output_tokens(500)
        .execute()
        .await?;

    println!("🤖 Assistant: {}\n", result3.text);
    println!(
        "📊 Usage: {} input tokens, {} output tokens",
        result3.usage.input_tokens, result3.usage.output_tokens
    );

    println!("\n✅ All examples completed successfully!");
    println!("\n💡 Key Features Demonstrated:");
    println!("   ✓ Basic text generation");
    println!("   ✓ Temperature control");
    println!("   ✓ Token limits");
    println!("   ✓ Usage tracking");
    println!("   ✓ Finish reason reporting");

    Ok(())
}
