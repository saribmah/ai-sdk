use ai_sdk_baseten::BasetenClient;
use ai_sdk_core::{GenerateText, prompt::Prompt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Baseten provider using Model APIs
    // API key will be loaded from BASETEN_API_KEY environment variable
    let provider = BasetenClient::new().build();

    // Create a chat model
    let model = provider.chat_model(Some("deepseek-ai/DeepSeek-V3-0324"));

    // Generate text
    let result = GenerateText::new(
        model,
        Prompt::text("What is the meaning of life? Answer in one sentence."),
    )
    .execute()
    .await?;

    println!("Generated text: {}", result.text);
    println!("Usage: {:?}", result.usage);

    Ok(())
}
