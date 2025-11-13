use ai_sdk_core::GenerateText;
use ai_sdk_core::prompt::Prompt;
use ai_sdk_deepseek::DeepSeekClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load API key from environment
    let api_key =
        std::env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY environment variable not set");

    // Create the DeepSeek provider
    let provider = DeepSeekClient::new().api_key(api_key).build();

    // Use the DeepSeek Reasoner (R1) model for advanced reasoning
    let model = provider.chat_model("deepseek-reasoner");

    println!("Generating response with DeepSeek Reasoner (R1)...\n");

    // Generate text with reasoning
    let result = GenerateText::new(
        model,
        Prompt::text(
            "A farmer needs to cross a river with a fox, a chicken, and a bag of grain. \
             The boat can only hold the farmer and one item at a time. If left alone, \
             the fox will eat the chicken, and the chicken will eat the grain. \
             How can the farmer get everything across safely?",
        ),
    )
    .temperature(0.3) // Lower temperature for more focused reasoning
    .max_output_tokens(1000)
    .execute()
    .await?;

    // Print the response, separating reasoning from the final answer
    println!("Response:");
    println!();

    // Check if we have reasoning content
    if !result.reasoning.is_empty() {
        println!("💭 Reasoning:");
        for reasoning in &result.reasoning {
            println!("{}", reasoning.text);
        }
        println!();
    }

    // Print the final answer
    println!("✨ Answer:");
    println!("{}", result.text);
    println!();

    // Print usage information
    println!("Token Usage:");
    println!("  Input tokens: {}", result.usage.input_tokens);
    println!("  Output tokens: {}", result.usage.output_tokens);
    println!("  Total tokens: {}", result.usage.total_tokens);

    if result.usage.reasoning_tokens > 0 {
        println!("  Reasoning tokens: {}", result.usage.reasoning_tokens);
    }

    // Print DeepSeek-specific metadata
    if let Some(provider_metadata) = result.provider_metadata
        && let Some(deepseek) = provider_metadata.get("deepseek")
    {
        println!("\nDeepSeek Cache Statistics:");
        if let Some(hit_tokens) = deepseek.get("promptCacheHitTokens") {
            println!("  Cache hit tokens: {}", hit_tokens);
        }
        if let Some(miss_tokens) = deepseek.get("promptCacheMissTokens") {
            println!("  Cache miss tokens: {}", miss_tokens);
        }
    }

    Ok(())
}
