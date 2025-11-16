use ai_sdk_baseten::BasetenClient;
use ai_sdk_core::{StreamText, prompt::Prompt};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Baseten provider using Model APIs
    let provider = BasetenClient::new().build();

    // Create a chat model
    let model = provider.chat_model(Some("deepseek-ai/DeepSeek-V3-0324"));

    // Stream text generation
    let result = StreamText::new(model, Prompt::text("Write a haiku about programming."))
        .execute()
        .await?;

    print!("Generated text: ");

    // Stream text deltas
    let mut text_stream = result.text_stream();
    while let Some(text_delta) = text_stream.next().await {
        print!("{}", text_delta);
        use std::io::Write;
        std::io::stdout().flush()?;
    }

    println!("\n\nStream completed!");

    Ok(())
}
