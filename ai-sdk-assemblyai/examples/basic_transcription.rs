use ai_sdk_assemblyai::AssemblyAIClient;
use ai_sdk_core::{AudioInput, Transcribe};
use ai_sdk_provider_utils::message::DataContent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Create provider
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

    // Transcribe the audio
    let audio_input = AudioInput::Data(DataContent::from(audio_data.to_vec()));
    let result = Transcribe::new(model, audio_input).execute().await?;

    println!("\n=== Transcription Results ===");
    println!("Text: {}", result.text);
    println!("Segments: {}", result.segments.len());

    if let Some(language) = result.language {
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

    Ok(())
}
