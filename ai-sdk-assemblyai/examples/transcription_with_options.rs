use ai_sdk_assemblyai::AssemblyAIClient;
use ai_sdk_core::{AudioInput, Transcribe};
use ai_sdk_provider::shared::provider_options::SharedProviderOptions;
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

    // Download audio from URL (a conversation with multiple speakers)
    let audio_url = "https://github.com/AssemblyAI-Examples/audio-examples/raw/main/espn-draft.mp3";
    let audio_data = reqwest::get(audio_url).await?.bytes().await?;

    println!("Transcribing audio with speaker diarization and sentiment analysis...");

    // Transcribe with advanced options
    let audio_input = AudioInput::Data(DataContent::from(audio_data.to_vec()));

    let mut provider_options = SharedProviderOptions::new();
    provider_options.insert(
        "assemblyai".to_string(),
        vec![
            ("speakerLabels".to_string(), serde_json::json!(true)),
            ("speakersExpected".to_string(), serde_json::json!(2)),
            ("sentimentAnalysis".to_string(), serde_json::json!(true)),
            ("autoChapters".to_string(), serde_json::json!(true)),
            ("punctuate".to_string(), serde_json::json!(true)),
            ("formatText".to_string(), serde_json::json!(true)),
        ]
        .into_iter()
        .collect(),
    );

    let result = Transcribe::new(model, audio_input)
        .provider_options(provider_options)
        .execute()
        .await?;

    println!("\n=== Transcription Results ===");
    println!("Text length: {} characters", result.text.len());
    println!("Segments: {}", result.segments.len());

    if let Some(language) = result.language {
        println!("Detected Language: {}", language);
    }

    if let Some(duration) = result.duration_in_seconds {
        println!("Duration: {:.2} seconds", duration);
    }

    // Print sample segments
    if !result.segments.is_empty() {
        println!("\nSample segments with timing:");
        for (i, segment) in result.segments.iter().take(10).enumerate() {
            println!(
                "  {}: [{:.2}s - {:.2}s] {}",
                i + 1,
                segment.start_second,
                segment.end_second,
                segment.text
            );
        }
    }

    // Print the full text
    println!("\n=== Full Transcript ===");
    println!("{}", result.text);

    Ok(())
}
