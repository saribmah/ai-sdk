/// Basic text-to-speech example with ElevenLabs.
///
/// This example shows how to:
/// - Create an ElevenLabs provider from environment variables
/// - Use a speech model to convert text to audio
/// - Save the generated audio to a file
///
/// Run with:
/// ```bash
/// export ELEVENLABS_API_KEY="your-api-key"
/// cargo run --example basic_speech -p ai-sdk-elevenlabs
/// ```
use ai_sdk_core::GenerateSpeech;
use ai_sdk_elevenlabs::create_elevenlabs;
use ai_sdk_provider::Provider;
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎙️  ElevenLabs Text-to-Speech Example\n");

    // Check API key is set
    let _api_key = env::var("ELEVENLABS_API_KEY")
        .map_err(|_| "ELEVENLABS_API_KEY environment variable not set")?;

    println!("✓ API key loaded from environment\n");

    // Create ElevenLabs provider
    let provider = create_elevenlabs();
    println!("✓ ElevenLabs provider created");

    // Create speech model
    let model = provider.speech_model("eleven_multilingual_v2")?;
    println!("✓ Model loaded: {}", model.model_id());
    println!("✓ Provider: {}\n", model.provider());

    // Text to convert to speech
    let text = "Hello! This is a test of the ElevenLabs text-to-speech API. It supports multiple languages and natural-sounding voices.";
    println!("📝 Text to speak: {}\n", text);

    // Generate speech
    println!("🔊 Generating speech...");
    let result = GenerateSpeech::new(model, text.to_string())
        .voice("21m00Tcm4TlvDq8ikWAM") // Rachel voice
        .output_format("mp3")
        .execute()
        .await?;

    println!("✓ Speech generated successfully!");
    println!("  Audio size: {} bytes", result.audio.bytes().len());
    println!("  Model ID: {}", result.responses[0].model_id);
    println!("  Audio format: {}", result.audio.format);

    // Save audio to file
    let output_file = "output_speech.mp3";
    fs::write(output_file, result.audio.bytes())?;
    println!("\n💾 Audio saved to: {}", output_file);
    println!(
        "   You can play it with: afplay {} (macOS) or mpg123 {} (Linux)",
        output_file, output_file
    );

    Ok(())
}
