use ai_sdk_core::{GenerateText, prompt::Prompt};
use ai_sdk_huggingface::{HuggingFaceProviderSettings, LLAMA_3_1_8B_INSTRUCT, create_huggingface};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file (if present)
    dotenv::dotenv().ok();

    // Create the Hugging Face provider
    // API key is loaded from HUGGINGFACE_API_KEY environment variable
    let provider = create_huggingface(HuggingFaceProviderSettings::new().load_api_key_from_env());

    // Create a model (using the Llama 3.1 8B Instruct model)
    let model = provider.responses(LLAMA_3_1_8B_INSTRUCT);

    // Generate text
    println!("Generating response...\n");

    let result = GenerateText::new(
        model,
        Prompt::text("What is the capital of France? Please be concise."),
    )
    .temperature(0.7)
    .max_output_tokens(100)
    .execute()
    .await?;

    // Print the response
    println!("Response: {}", result.text);
    println!("\nUsage:");
    println!("  Input tokens:  {}", result.usage.input_tokens);
    println!("  Output tokens: {}", result.usage.output_tokens);
    println!("  Total tokens:  {}", result.usage.total_tokens);
    println!("\nFinish reason: {:?}", result.finish_reason);

    Ok(())
}
