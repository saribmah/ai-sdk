// Provider example: DeepSeek Reasoner (R1) model
// This example demonstrates using DeepSeek's reasoning model with do_generate().
// It does NOT use llm-kit-core (GenerateText, StreamText, etc.)

use llm_kit_deepseek::DeepSeekClient;
use llm_kit_provider::language_model::call_options::LanguageModelCallOptions;
use llm_kit_provider::language_model::content::LanguageModelContent;
use llm_kit_provider::language_model::prompt::LanguageModelMessage;

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

    // Create a prompt with a logic puzzle
    let prompt = vec![LanguageModelMessage::user_text(
        "A farmer needs to cross a river with a fox, a chicken, and a bag of grain. \
         The boat can only hold the farmer and one item at a time. If left alone, \
         the fox will eat the chicken, and the chicken will eat the grain. \
         How can the farmer get everything across safely?",
    )];

    // Create call options with lower temperature for focused reasoning
    let options = LanguageModelCallOptions::new(prompt)
        .with_temperature(0.3)
        .with_max_output_tokens(1000);

    // Call do_generate() directly
    let result = model.do_generate(options).await?;

    // Print the response, separating reasoning from the final answer
    println!("Response:");
    println!();

    let mut has_reasoning = false;
    let mut text_content = String::new();

    // Process content parts
    for content in &result.content {
        match content {
            LanguageModelContent::Reasoning(reasoning) => {
                if !has_reasoning {
                    println!("💭 Reasoning:");
                    has_reasoning = true;
                }
                println!("{}", reasoning.text);
            }
            LanguageModelContent::Text(text) => {
                text_content.push_str(&text.text);
            }
            _ => {}
        }
    }

    if has_reasoning {
        println!();
    }

    // Print the final answer
    if !text_content.is_empty() {
        println!("✨ Answer:");
        println!("{}", text_content);
        println!();
    }

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
