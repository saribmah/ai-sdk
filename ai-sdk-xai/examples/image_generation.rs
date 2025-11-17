/// Image generation example using xAI provider with only ai-sdk-provider.
///
/// This example demonstrates:
/// - Using ImageModel::do_generate() directly (no ai-sdk-core)
/// - Generating images from text prompts
/// - Working with ImageModelCallOptions from ai-sdk-provider
///
/// Run with:
/// ```bash
/// export XAI_API_KEY="your-api-key"
/// cargo run --example image_generation -p ai-sdk-xai
/// ```
use ai_sdk_provider::image_model::call_options::{ImageModelCallOptions, ImageSize};
use ai_sdk_xai::XaiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 xAI Image Generation Example (Provider-Only)\n");

    // Get API key from environment
    let api_key = std::env::var("XAI_API_KEY").map_err(
        |_| "XAI_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create xAI provider using client builder
    let provider = XaiClient::new().api_key(api_key).build();

    // Create an image model
    let model = provider.image_model("grok-2-vision-1212");

    println!("✓ Model loaded: {}\n", model.model_id());

    // Example 1: Basic image generation
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 1: Basic Image Generation");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt1 =
        "A serene mountain landscape at sunset with a lake reflecting the colors of the sky";
    let options1 = ImageModelCallOptions::new(prompt1.to_string(), 1);

    println!("📝 Prompt: \"{}\"\n", prompt1);
    println!("🎨 Generating image...");

    let result1 = model.do_generate(options1).await?;

    println!("✅ Image generated successfully!");
    println!("   Number of images: {}", result1.images.len());
    for (i, image) in result1.images.iter().enumerate() {
        println!("\n   Image #{}:", i + 1);
        match image {
            ai_sdk_provider::image_model::ImageData::Base64(data) => {
                println!("      Base64 data length: {} characters", data.len());
                println!("      Preview: {}...", &data[..data.len().min(50)]);
            }
            ai_sdk_provider::image_model::ImageData::Binary(data) => {
                println!("      Binary data size: {} bytes", data.len());
            }
        }
    }
    println!();

    // Example 2: High-quality image with specific size
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: High-Quality Image (1024x1024)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt2 = "A futuristic city with flying cars and neon lights, cyberpunk style";
    let options2 =
        ImageModelCallOptions::new(prompt2.to_string(), 1).with_size(ImageSize::new(1024, 1024));

    println!("📝 Prompt: \"{}\"\n", prompt2);
    println!("🎨 Generating high-quality 1024x1024 image...");

    let result2 = model.do_generate(options2).await?;

    println!("✅ Image generated successfully!");
    println!("   Size: 1024x1024");
    for (i, image) in result2.images.iter().enumerate() {
        println!("\n   Image #{}:", i + 1);
        match image {
            ai_sdk_provider::image_model::ImageData::Base64(data) => {
                println!("      Base64 data length: {} characters", data.len());
            }
            ai_sdk_provider::image_model::ImageData::Binary(data) => {
                println!("      Binary data size: {} bytes", data.len());
            }
        }
    }
    println!();

    // Example 3: Multiple style variations
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 3: Different Image Styles");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let styles = [
        "A cute robot learning to code, cartoon style",
        "An abstract representation of artificial intelligence, geometric shapes",
        "A steampunk workshop with gears and machinery, vintage aesthetic",
    ];

    for (i, prompt) in styles.iter().enumerate() {
        println!("🎨 Style #{}: {}\n", i + 1, prompt);
        println!("   Generating...");

        let options = ImageModelCallOptions::new(prompt.to_string(), 1);

        let result = model.do_generate(options).await?;

        println!("   ✅ Generated!");
        for image in &result.images {
            match image {
                ai_sdk_provider::image_model::ImageData::Base64(data) => {
                    println!("      Base64 data length: {} characters", data.len());
                }
                ai_sdk_provider::image_model::ImageData::Binary(data) => {
                    println!("      Binary data size: {} bytes", data.len());
                }
            }
        }
        println!();
    }

    // Example 4: Inspecting response metadata
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 4: Response Metadata");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt4 = "A peaceful zen garden with cherry blossoms, watercolor painting style";
    let options4 = ImageModelCallOptions::new(prompt4.to_string(), 1);

    println!("📝 Prompt: \"{}\"\n", prompt4);
    println!("🎨 Generating image and inspecting metadata...");

    let result4 = model.do_generate(options4).await?;

    println!("\n📊 Response Details:");
    println!("   Images generated: {}", result4.images.len());
    println!("   Model: {}", result4.response.model_id);
    println!("   Timestamp: {:?}", result4.response.timestamp);

    if !result4.warnings.is_empty() {
        println!("\n⚠️  Warnings:");
        for warning in &result4.warnings {
            println!("   - {:?}", warning);
        }
    }

    for (i, image) in result4.images.iter().enumerate() {
        println!("\n   Image #{}:", i + 1);
        match image {
            ai_sdk_provider::image_model::ImageData::Base64(data) => {
                println!("      Base64 data length: {} characters", data.len());
            }
            ai_sdk_provider::image_model::ImageData::Binary(data) => {
                println!("      Binary data size: {} bytes", data.len());
            }
        }
    }

    println!("\n✅ All examples completed successfully!");
    println!("\n💡 Key Features Demonstrated:");
    println!("   ✓ Using do_generate() directly (provider-only)");
    println!("   ✓ Basic image generation");
    println!("   ✓ Custom size settings");
    println!("   ✓ Different image styles");
    println!("   ✓ Response metadata inspection");
    println!("\n📌 Note: Generated images are returned as base64-encoded data.");
    println!("   Decode and save them to disk or display them in your application as needed.");

    Ok(())
}
