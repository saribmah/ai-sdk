/// Conversation example demonstrating more advanced features.
///
/// This example shows how to:
/// - Use system messages
/// - Configure temperature and other settings
/// - Create a multi-turn conversation simulation
///
/// Run with:
/// ```bash
/// export OPENAI_API_KEY="your-api-key"
/// cargo run --example conversation
/// ```
use ai_sdk_core::GenerateTextBuilder;
use ai_sdk_core::prompt::Prompt;
use ai_sdk_openai_compatible::OpenAICompatibleClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AI SDK Rust - Conversation Example\n");

    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(
        |_| "OPENAI_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    // Create OpenAI provider using the client builder
    let provider = OpenAICompatibleClient::new()
        .base_url("https://openrouter.ai/api/v1")
        .api_key(api_key)
        .build();

    let model = provider.chat_model("gpt-4o-mini");

    println!("✓ Using model: {} ({})", model.model_id(), model.provider());
    println!();

    // Example 1: With system message
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 1: Using System Messages");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt = Prompt::text("Explain quantum computing")
        .with_system("You are a helpful teacher who explains complex topics simply.");

    println!("📤 System: You are a helpful teacher who explains complex topics simply.");
    println!("📤 User: Explain quantum computing\n");

    println!("⏳ Generating response...\n");
    let result = GenerateTextBuilder::new(model.clone(), prompt)
        .temperature(0.7)
        .max_output_tokens(150)
        .execute()
        .await?;

    println!("📝 Response:");
    for content in &result.content {
        println!("{:?}\n", content);
    }
    println!(
        "📊 Tokens used: {} in, {} out\n",
        result.usage.input_tokens, result.usage.output_tokens
    );

    // Example 2: Different temperature settings
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: Temperature Comparison");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let creative_question = "Write a creative opening line for a sci-fi novel.";

    // Low temperature (more focused)
    println!("📤 Prompt: {}", creative_question);
    println!("🌡️  Temperature: 0.2 (focused)\n");

    let focused_prompt = Prompt::text(creative_question);

    let focused_result = GenerateTextBuilder::new(model.clone(), focused_prompt)
        .temperature(0.2)
        .max_output_tokens(100)
        .execute()
        .await?;

    println!("📝 Focused Response:");
    for content in &focused_result.content {
        println!("{:?}\n", content);
    }

    // High temperature (more creative)
    println!("🌡️  Temperature: 1.2 (creative)\n");

    let creative_prompt = Prompt::text(creative_question);

    let creative_result = GenerateTextBuilder::new(model, creative_prompt)
        .temperature(1.2)
        .max_output_tokens(100)
        .execute()
        .await?;

    println!("📝 Creative Response:");
    for content in &creative_result.content {
        println!("{:?}\n", content);
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("\n✅ Example completed successfully!");
    println!("💡 Notice how temperature affects the creativity of responses.");

    Ok(())
}
