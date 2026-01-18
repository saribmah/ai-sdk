/// Basic image generation example demonstrating AI image creation with OpenAI.
///
/// This example shows how to:
/// - Create an OpenAI provider from environment variables
/// - Use the DALL-E image model to generate images from text prompts
/// - Handle the response with image data and metadata
///
/// Note: This example uses the official OpenAI API for image generation.
/// OpenRouter and other providers may not support image generation endpoints.
///
/// Run with:
/// ```bash
/// export OPENAI_API_KEY="your-openai-api-key"
/// cargo run --example basic_image
/// ```
use llm_kit_core::GenerateImage;
use llm_kit_openai_compatible::OpenAICompatibleClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AI SDK Rust - Basic Image Generation Example\n");

    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(
        |_| "OPENAI_API_KEY environment variable not set. Please set it with your OpenAI API key.",
    )?;

    println!("✓ API key loaded from environment");
    println!("ℹ️  Note: This example requires an official OpenAI API key");
    println!("   (Get one at: https://platform.openai.com/api-keys)\n");

    // Create OpenAI provider using the client builder
    // Note: Image generation requires the official OpenAI API, not OpenRouter
    let provider = OpenAICompatibleClient::new()
        .base_url("https://api.openai.com/v1")
        .name("openai")
        .api_key(api_key)
        .build();

    println!("✓ Provider created: {}", provider.name());
    println!("✓ Base URL: {}\n", provider.base_url());

    // Get an image model (returns Arc<dyn ImageModel>)
    // Supported models: dall-e-2, dall-e-3
    let model = provider.image_model("dall-e-3");
    println!("✓ Model loaded: {}", model.model_id());
    println!("✓ Provider: {}\n", model.provider());

    // Create a prompt for image generation
    let prompt = "A serene landscape with mountains reflected in a crystal-clear lake at sunset, \
                  painted in the style of impressionism";

    println!("📤 Image prompt:");
    println!("  \"{}\"", prompt);
    println!();

    // Generate image
    println!("⏳ Generating image...\n");
    let result = GenerateImage::new(model, prompt.to_string())
        .n(1)
        .size("1024x1024")
        .execute()
        .await
        .map_err(|e| {
            eprintln!("\n❌ Full error details: {:?}\n", e);
            e
        })?;

    // Display the results
    println!("✅ Image(s) generated!\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 Image Information:");
    println!("  • Number of images: {}", result.images.len());

    for (i, image) in result.images.iter().enumerate() {
        println!("\n  Image [{}]:", i + 1);

        // Get base64 data
        let b64_data = image.base64();
        println!("    • Format: Base64");
        println!("    • Data length: {} bytes", b64_data.len());
        // Show first 50 chars as preview
        let preview = if b64_data.len() > 50 {
            &b64_data[..50]
        } else {
            b64_data
        };
        println!("    • Preview: {}...", preview);

        println!("    • MIME type: {}", image.media_type);

        if let Some(name) = &image.name {
            println!("    • Name: {}", name);
        }
    }
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    println!("\n📊 Metadata:");
    if !result.warnings.is_empty() {
        println!("  • Warnings: {} warning(s)", result.warnings.len());
        for warning in &result.warnings {
            println!("    - {:?}", warning);
        }
    } else {
        println!("  • No warnings");
    }

    println!("  • Responses: {}", result.responses.len());
    for (i, response) in result.responses.iter().enumerate() {
        println!("    Response [{}]:", i + 1);
        println!("      - Model ID: {}", response.model_id);
        if let Some(headers) = &response.headers {
            println!("      - Headers: {} header(s)", headers.len());
        }
    }

    println!("\n💡 Note: To save the image to a file, you would need to:");
    println!("   - Decode the base64 data if present");
    println!("   - Or download from the URL if provided");
    println!("   - Write to a file with the appropriate extension");

    println!("\n✅ Example completed successfully!");

    Ok(())
}
