use ai_sdk_core::GenerateText;
use ai_sdk_core::prompt::Prompt;
use ai_sdk_xai::XaiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load API key from environment variable
    let api_key =
        std::env::var("XAI_API_KEY").expect("XAI_API_KEY environment variable must be set");

    // Create xAI provider
    let provider = XaiClient::new().api_key(api_key).build();

    // Create a chat model
    let model = provider.chat_model("grok-3-fast");

    println!("🤖 xAI Basic Chat Example");
    println!("Model: grok-3-fast");
    println!("----------------------------------------\n");

    // Create a simple prompt
    let prompt = Prompt::text("What is the capital of France? Answer in one sentence.");

    println!("💬 Prompt: What is the capital of France?\n");

    // Generate response
    let result = GenerateText::new(model, prompt)
        .temperature(0.7)
        .max_output_tokens(100)
        .execute()
        .await?;

    // Print the response
    println!("🎯 Response:");
    println!("{}", result.text);

    // Print usage statistics
    println!("\n📊 Usage Statistics:");
    println!("  Input tokens:  {}", result.usage.input_tokens);
    println!("  Output tokens: {}", result.usage.output_tokens);
    println!("  Total tokens:  {}", result.usage.total_tokens);
    if result.usage.reasoning_tokens > 0 {
        println!("  Reasoning tokens: {}", result.usage.reasoning_tokens);
    }

    println!("\n✅ Example completed successfully!");

    Ok(())
}
