use ai_sdk_core::GenerateText;
use ai_sdk_core::prompt::Prompt;
use ai_sdk_groq::GroqClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Groq provider
    // API key will be read from GROQ_API_KEY environment variable
    let provider = GroqClient::new().load_api_key_from_env().build();

    // Create a chat model - using Llama 3.1 8B Instant for fast responses
    let model = provider.chat_model("llama-3.1-8b-instant");

    // Generate a response
    println!("Generating response...\n");

    let result = GenerateText::new(
        model,
        Prompt::text("Explain quantum computing in simple terms in 2-3 sentences."),
    )
    .temperature(0.7)
    .max_output_tokens(150)
    .execute()
    .await?;

    println!("Response: {}", result.text);
    println!("\nUsage:");
    println!("  Input tokens: {:?}", result.usage.input_tokens);
    println!("  Output tokens: {:?}", result.usage.output_tokens);
    println!("  Total tokens: {:?}", result.usage.total_tokens);

    // Check for Groq-specific metadata (cached tokens)
    if let Some(provider_metadata) = &result.provider_metadata
        && let Some(groq) = provider_metadata.get("groq")
    {
        println!("\nGroq Metadata:");
        println!("  Cached tokens: {:?}", groq.get("cachedTokens"));
    }

    Ok(())
}
