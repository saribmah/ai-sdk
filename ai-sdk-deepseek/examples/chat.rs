// Provider example: Basic chat using do_generate()
// This example demonstrates using the DeepSeek provider directly with ai-sdk-provider traits.
// It does NOT use ai-sdk-core (GenerateText, StreamText, etc.)

use ai_sdk_deepseek::DeepSeekClient;
use ai_sdk_provider::language_model::call_options::LanguageModelCallOptions;
use ai_sdk_provider::language_model::content::LanguageModelContent;
use ai_sdk_provider::language_model::prompt::LanguageModelMessage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load API key from environment
    let api_key =
        std::env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY environment variable not set");

    // Create the DeepSeek provider using the client builder
    let provider = DeepSeekClient::new().api_key(api_key).build();

    // Get a language model
    let model = provider.chat_model("deepseek-chat");

    println!("Generating text with DeepSeek Chat (using do_generate)...\n");

    // Create a prompt
    let prompt = vec![LanguageModelMessage::user_text(
        "Write a Rust function that calculates the factorial of a number",
    )];

    // Create call options
    let options = LanguageModelCallOptions::new(prompt).with_max_output_tokens(500);

    // Call do_generate() directly (provider trait method)
    let result = model.do_generate(options).await?;

    // Print the response
    println!("Response:");
    for content in &result.content {
        if let LanguageModelContent::Text(text) = content {
            println!("{}", text.text);
        }
    }
    println!();

    // Print usage information
    println!("Token Usage:");
    println!("  Input tokens: {}", result.usage.input_tokens);
    println!("  Output tokens: {}", result.usage.output_tokens);
    println!("  Total tokens: {}", result.usage.total_tokens);

    // Print finish reason
    println!("\nFinish Reason: {:?}", result.finish_reason);

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
