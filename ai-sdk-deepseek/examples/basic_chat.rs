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

    // Get the chat model
    let model = provider.chat_model("deepseek-chat");

    println!("Generating text with DeepSeek Chat...\n");

    // Generate text
    let result = GenerateText::new(
        model,
        Prompt::text("Write a Rust function that calculates the factorial of a number"),
    )
    .temperature(0.7)
    .max_output_tokens(500)
    .execute()
    .await?;

    // Print the response
    println!("Response:");
    println!("{}", result.text);
    println!();

    // Print usage information
    println!("Token Usage:");
    println!("  Input tokens: {}", result.usage.input_tokens);
    println!("  Output tokens: {}", result.usage.output_tokens);
    println!("  Total tokens: {}", result.usage.total_tokens);

    // Print DeepSeek-specific metadata if available
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
