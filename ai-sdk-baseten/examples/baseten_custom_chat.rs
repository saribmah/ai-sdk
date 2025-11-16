use ai_sdk_baseten::BasetenClient;
use ai_sdk_core::{GenerateText, prompt::Prompt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Baseten provider with a custom model URL
    // Replace {model-id} with your actual model ID from Baseten
    let model_id =
        std::env::var("BASETEN_MODEL_ID").expect("BASETEN_MODEL_ID environment variable not set");

    let model_url = format!(
        "https://model-{}.api.baseten.co/environments/production/sync/v1",
        model_id
    );

    let provider = BasetenClient::new().model_url(model_url).build();

    // Create a chat model (model_id is optional when using custom URL)
    let model = provider.chat_model(None);

    // Generate text
    let result = GenerateText::new(
        model,
        Prompt::text("Tell me a short joke about programming."),
    )
    .execute()
    .await?;

    println!("Generated text: {}", result.text);
    println!("Usage: {:?}", result.usage);

    Ok(())
}
