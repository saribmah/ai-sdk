// examples/transcription.rs
//
// This example demonstrates using AssemblyAI's TranscriptionModel directly
// with only ai-sdk-provider (no ai-sdk-core).
//
// This validates the provider implementation works independently and shows
// how to use the do_generate() method directly.

use ai_sdk_assemblyai::AssemblyAIClient;
use ai_sdk_provider::transcription_model::call_options::TranscriptionModelCallOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Create provider using the builder pattern
    let provider = AssemblyAIClient::new()
        .api_key(std::env::var("ASSEMBLYAI_API_KEY")?)
        .build();

    // Create transcription model
    let model = provider.transcription_model("best");

    println!("Downloading audio file...");

    // Download audio from URL
    let audio_url =
        "https://github.com/AssemblyAI-Examples/audio-examples/raw/main/20230607_me_canadian_wildfires.mp3";
    let audio_data = reqwest::get(audio_url).await?.bytes().await?;

    println!("Transcribing audio...");

    // Prepare call options with audio data (using mp3() convenience method)
    let call_options = TranscriptionModelCallOptions::mp3(audio_data.to_vec());

    // Call do_generate() directly (provider trait method)
    let result = model.do_generate(call_options).await?;

    println!("\n=== Transcription Results ===");
    println!("Text: {}", result.text);
    println!("Segments: {}", result.segments.len());

    if let Some(language) = &result.language {
        println!("Detected Language: {}", language);
    }

    if let Some(duration) = result.duration_in_seconds {
        println!("Duration: {:.2} seconds", duration);
    }

    // Print first few segments
    if !result.segments.is_empty() {
        println!("\nFirst 5 segments:");
        for (i, segment) in result.segments.iter().take(5).enumerate() {
            println!(
                "  {}: [{:.2}s - {:.2}s] {}",
                i + 1,
                segment.start_second,
                segment.end_second,
                segment.text
            );
        }
    }

    println!("\n=== Provider Information ===");
    println!("Provider: {}", model.provider());
    println!("Model ID: {}", model.model_id());

    Ok(())
}
