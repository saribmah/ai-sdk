use ai_sdk_core::StreamText;
use ai_sdk_core::prompt::Prompt;
use ai_sdk_groq::GroqClient;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Groq provider
    let provider = GroqClient::new().load_api_key_from_env().build();

    // Create a chat model
    let model = provider.chat_model("llama-3.1-8b-instant");

    // Stream a response
    println!("Streaming response...\n");

    let result = StreamText::new(model, Prompt::text("Write a haiku about coding"))
        .temperature(0.8)
        .execute()
        .await?;

    // Stream and print text deltas
    let mut text_stream = result.text_stream();
    while let Some(delta) = text_stream.next().await {
        print!("{}", delta);
        tokio::io::AsyncWriteExt::flush(&mut tokio::io::stdout()).await?;
    }

    println!("\n\nStream complete!");

    Ok(())
}
