use ai_sdk_core::StreamText;
use ai_sdk_core::prompt::Prompt;
use ai_sdk_deepseek::DeepSeekClient;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load API key from environment
    let api_key =
        std::env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY environment variable not set");

    // Create the DeepSeek provider
    let provider = DeepSeekClient::new().api_key(api_key).build();

    // Get the chat model
    let model = provider.chat_model("deepseek-chat");

    println!("Streaming response from DeepSeek Chat...\n");
    println!("Response:");

    // Stream text generation
    let result = StreamText::new(
        model,
        Prompt::text("Tell me a short story about a robot learning to code"),
    )
    .temperature(0.8)
    .max_output_tokens(500)
    .execute()
    .await?;

    // Stream and print text deltas
    let mut text_stream = result.text_stream();
    while let Some(delta) = text_stream.next().await {
        print!("{}", delta);
        // Flush to ensure immediate output
        use std::io::Write;
        std::io::stdout().flush()?;
    }

    println!("\n");

    // Print usage information (available after stream completes)
    let usage = result.usage().await?;
    println!("Token Usage:");
    println!("  Input tokens: {}", usage.input_tokens);
    println!("  Output tokens: {}", usage.output_tokens);
    println!("  Total tokens: {}", usage.total_tokens);

    Ok(())
}
