use ai_sdk_groq::{GroqClient, GroqTranscriptionOptions};
use ai_sdk_provider::transcription_model::call_options::TranscriptionModelCallOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Groq provider
    // API key will be read from GROQ_API_KEY environment variable
    let provider = GroqClient::new().load_api_key_from_env().build();

    // Create a Whisper transcription model
    let model = provider.transcription_model("whisper-large-v3");

    // For this example, we'll create some dummy audio data
    // In a real application, you would read from a file:
    // let audio_data = std::fs::read("path/to/audio.mp3")?;

    println!("Note: This example requires actual audio data to work.");
    println!("Replace the dummy data below with real audio file contents.\n");

    // Dummy audio data (you would replace this with actual audio bytes)
    let audio_data = vec![0u8; 100]; // This won't actually transcribe anything

    // Create transcription options with Groq-specific settings
    let mut provider_options = std::collections::HashMap::new();
    let mut groq_opts_map = std::collections::HashMap::new();
    let groq_options = GroqTranscriptionOptions::new()
        .with_language("en") // English audio
        .with_verbose_json() // Get detailed segments
        .with_temperature(0.0); // Deterministic output

    // Convert to nested structure required by provider_options
    let groq_value = serde_json::to_value(&groq_options)?;
    if let serde_json::Value::Object(map) = groq_value {
        for (k, v) in map {
            groq_opts_map.insert(k, v);
        }
    }
    provider_options.insert("groq".to_string(), groq_opts_map);

    let options =
        TranscriptionModelCallOptions::mp3(audio_data).with_provider_options(provider_options);

    // Transcribe the audio
    println!("Transcribing audio...\n");

    match model.do_generate(options).await {
        Ok(result) => {
            println!("Transcription: {}", result.text);
            println!("\nLanguage: {:?}", result.language);
            println!("Duration: {:?} seconds", result.duration_in_seconds);

            if !result.segments.is_empty() {
                println!("\nSegments:");
                for (i, segment) in result.segments.iter().enumerate() {
                    println!(
                        "  [{}] {:.2}s - {:.2}s: {}",
                        i + 1,
                        segment.start_second,
                        segment.end_second,
                        segment.text
                    );
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("\nThis is expected if you're using dummy audio data.");
            eprintln!("To use this example properly:");
            eprintln!("1. Replace the audio_data with actual audio file contents");
            eprintln!("2. Ensure GROQ_API_KEY environment variable is set");
            eprintln!("3. Use a supported audio format (MP3, WAV, FLAC, etc.)");
        }
    }

    Ok(())
}
