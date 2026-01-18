/// Basic embedding example demonstrating text embeddings with OpenAI-compatible providers.
///
/// This example shows how to:
/// - Create a provider from environment variables
/// - Use the embedding model to generate embeddings for text
/// - Handle the response with usage information
///
/// Run with:
/// ```bash
/// export OPENAI_API_KEY="your-api-key"
/// cargo run --example basic_embedding
/// ```
use llm_kit_core::EmbedMany;
use llm_kit_openai_compatible::OpenAICompatibleClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AI SDK Rust - Basic Embedding Example\n");

    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(
        |_| "OPENAI_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create OpenAI provider using the client builder
    let provider = OpenAICompatibleClient::new()
        .base_url("https://openrouter.ai/api/v1")
        .api_key(api_key)
        .build();

    println!("✓ Provider created: {}", provider.name());
    println!("✓ Base URL: {}\n", provider.base_url());

    // Get an embedding model (returns Arc<dyn EmbeddingModel<String>>)
    let model = provider.text_embedding_model("openai/text-embedding-3-small");
    println!("✓ Model loaded: {}", model.model_id());
    println!("✓ Provider: {}\n", model.provider());

    // Create some text to embed
    let texts = vec![
        "The capital of France is Paris.".to_string(),
        "The capital of Germany is Berlin.".to_string(),
        "The capital of Spain is Madrid.".to_string(),
    ];

    println!("📤 Generating embeddings for {} texts:", texts.len());
    for (i, text) in texts.iter().enumerate() {
        println!("  [{}] \"{}\"", i + 1, text);
    }
    println!();

    // Generate embeddings
    println!("⏳ Generating embeddings...\n");
    let result = EmbedMany::new(model, texts).execute().await?;

    // Display the results
    println!("✅ Embeddings generated!\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 Embedding Information:");
    println!("  • Number of embeddings: {}", result.embeddings.len());

    for (i, embedding) in result.embeddings.iter().enumerate() {
        println!("  • Embedding [{}] dimensions: {}", i + 1, embedding.len());
        // Show first 5 values as a preview
        let preview: Vec<String> = embedding
            .iter()
            .take(5)
            .map(|v| format!("{:.6}", v))
            .collect();
        println!("    Preview: [{}, ...]", preview.join(", "));
    }
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    println!("\n📊 Metadata:");
    println!("  • Tokens used: {}", result.usage.tokens);

    println!("\n✅ Example completed successfully!");

    Ok(())
}
