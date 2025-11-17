/// Basic chat example using xAI provider with only ai-sdk-provider.
///
/// This example demonstrates:
/// - Using LanguageModel::do_generate() directly (no ai-sdk-core)
/// - Basic text generation without tools
/// - Working with LanguageModelCallOptions from ai-sdk-provider
///
/// Run with:
/// ```bash
/// export XAI_API_KEY="your-api-key"
/// cargo run --example chat -p ai-sdk-xai
/// ```
use ai_sdk_provider::language_model::call_options::LanguageModelCallOptions;
use ai_sdk_provider::language_model::content::LanguageModelContent;
use ai_sdk_provider::language_model::prompt::LanguageModelMessage;
use ai_sdk_xai::XaiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 xAI Basic Chat Example (Provider-Only)\n");

    // Get API key from environment
    let api_key = std::env::var("XAI_API_KEY").map_err(
        |_| "XAI_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create xAI provider using client builder
    let provider = XaiClient::new().api_key(api_key).build();

    // Create a language model
    let model = provider.chat_model("grok-beta");

    println!("✓ Model loaded: {}\n", model.model_id());

    // Example 1: Simple text generation
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 1: Simple Question");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt1 = vec![LanguageModelMessage::user_text(
        "What is the capital of France? Answer in one sentence.",
    )];

    let options1 = LanguageModelCallOptions::new(prompt1).with_max_output_tokens(100);

    println!("💬 User: What is the capital of France?\n");

    let result1 = model.do_generate(options1).await?;

    // Extract text from content
    let text1 = extract_text(&result1.content);
    println!("🤖 Assistant: {}\n", text1);
    println!(
        "📊 Usage: {} input tokens, {} output tokens",
        result1.usage.input_tokens, result1.usage.output_tokens
    );
    println!("🏁 Finish reason: {:?}\n", result1.finish_reason);

    // Example 2: With temperature
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: Creative Response");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt2 = vec![LanguageModelMessage::user_text(
        "Write a haiku about artificial intelligence.",
    )];

    let options2 = LanguageModelCallOptions::new(prompt2)
        .with_temperature(0.9)
        .with_max_output_tokens(200);

    println!("💬 User: Write a haiku about artificial intelligence.\n");

    let result2 = model.do_generate(options2).await?;

    let text2 = extract_text(&result2.content);
    println!("🤖 Assistant:\n{}\n", text2);
    println!(
        "📊 Usage: {} input tokens, {} output tokens\n",
        result2.usage.input_tokens, result2.usage.output_tokens
    );

    // Example 3: Longer response
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 3: Longer Response");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt3 = vec![LanguageModelMessage::user_text(
        "Explain what Rust's ownership system is in 2-3 sentences.",
    )];

    let options3 = LanguageModelCallOptions::new(prompt3).with_max_output_tokens(500);

    println!("💬 User: Explain Rust's ownership system.\n");

    let result3 = model.do_generate(options3).await?;

    let text3 = extract_text(&result3.content);
    println!("🤖 Assistant: {}\n", text3);
    println!(
        "📊 Usage: {} input tokens, {} output tokens",
        result3.usage.input_tokens, result3.usage.output_tokens
    );

    println!("\n✅ All examples completed successfully!");
    println!("\n💡 Key Features Demonstrated:");
    println!("   ✓ Using do_generate() directly (provider-only)");
    println!("   ✓ Basic text generation");
    println!("   ✓ Temperature control");
    println!("   ✓ Token limits");
    println!("   ✓ Usage tracking");
    println!("   ✓ Finish reason reporting");

    Ok(())
}

/// Helper function to extract text from content parts
fn extract_text(content: &[LanguageModelContent]) -> String {
    content
        .iter()
        .filter_map(|c| match c {
            LanguageModelContent::Text(text) => Some(text.text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("")
}
