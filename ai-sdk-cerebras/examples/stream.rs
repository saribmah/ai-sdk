use futures_util::StreamExt;
/// Streaming chat example using llm-kit-provider traits only.
///
/// This example demonstrates direct usage of the LanguageModel::do_stream() trait
/// without llm-kit-core abstractions.
///
/// Run with:
/// ```bash
/// export CEREBRAS_API_KEY="your-api-key"
/// cargo run --example stream
/// ```
use llm_kit_cerebras::CerebrasClient;
use llm_kit_provider::language_model::call_options::LanguageModelCallOptions;
use llm_kit_provider::language_model::prompt::LanguageModelMessage;
use llm_kit_provider::language_model::stream_part::LanguageModelStreamPart;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 Cerebras Streaming Example (Provider Traits)\n");

    // Load environment variables
    dotenvy::dotenv().ok();

    // Get API key from environment
    let api_key = std::env::var("CEREBRAS_API_KEY")
        .map_err(|_| "CEREBRAS_API_KEY environment variable not set")?;

    println!("✓ API key loaded from environment");

    // Create provider using builder
    let provider = CerebrasClient::new().api_key(api_key).build();

    println!("✓ Provider created: {}", provider.name());

    // Get a language model
    let model = provider.chat_model("llama-3.3-70b");
    println!("✓ Model: {}\n", model.model_id());

    // Create a prompt using provider types
    let messages = vec![LanguageModelMessage::user_text(
        "Write a short poem about Rust programming.",
    )];

    println!("📤 Sending prompt...\n");

    // Call do_stream directly (provider trait method)
    let options = LanguageModelCallOptions::new(messages)
        .with_temperature(0.7)
        .with_max_output_tokens(200);

    let result = model.do_stream(options).await?;

    println!("⏳ Streaming response...\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    print!("📝 ");
    std::io::Write::flush(&mut std::io::stdout())?;

    // Process the stream
    let mut stream = result.stream;
    while let Some(part) = stream.next().await {
        match part {
            LanguageModelStreamPart::TextDelta(text_delta) => {
                print!("{}", text_delta.delta);
                std::io::Write::flush(&mut std::io::stdout())?;
            }
            LanguageModelStreamPart::Finish(finish) => {
                println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("\n📊 Stream Complete:");
                println!("  • Finish reason: {:?}", finish.finish_reason);
                println!("  • Input tokens: {}", finish.usage.input_tokens);
                println!("  • Output tokens: {}", finish.usage.output_tokens);
                println!("  • Total tokens: {}", finish.usage.total_tokens);

                if finish.usage.reasoning_tokens > 0 {
                    println!("  • Reasoning tokens: {}", finish.usage.reasoning_tokens);
                }
            }
            LanguageModelStreamPart::Error(error) => {
                eprintln!("\n❌ Stream error: {:?}", error);
            }
            _ => {}
        }
    }

    println!("\n✅ Example completed successfully!");

    Ok(())
}
