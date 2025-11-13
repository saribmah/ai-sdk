use ai_sdk_core::GenerateSpeech;
use ai_sdk_groq::{GroqClient, GroqSpeechOptions};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Groq provider
    // API key will be read from GROQ_API_KEY environment variable
    let provider = GroqClient::new().load_api_key_from_env().build();

    // Create a text-to-speech model
    let model = provider.speech_model("playai-tts");

    println!("Generating speech from text...\n");

    // Configure Groq-specific options
    let mut provider_options = HashMap::new();
    let mut groq_opts_map = HashMap::new();
    let groq_options = GroqSpeechOptions::new().with_sample_rate(24000); // 24kHz sample rate

    // Convert to nested structure required by provider_options
    let groq_value = serde_json::to_value(&groq_options)?;
    if let serde_json::Value::Object(map) = groq_value {
        for (k, v) in map {
            groq_opts_map.insert(k, v);
        }
    }
    provider_options.insert("groq".to_string(), groq_opts_map);

    // Generate speech
    let result = GenerateSpeech::new(
        model,
        "Hello! This is a test of Groq's text-to-speech capabilities using PlayAI.".to_string(),
    )
    .voice("Fritz-PlayAI".to_string()) // PlayAI voice
    .output_format("wav".to_string()) // WAV format
    .speed(1.0) // Normal speed
    .provider_options(provider_options)
    .execute()
    .await?;

    // Save the audio to a file
    let output_file = "groq_speech_output.wav";
    let bytes = result.audio.to_vec();
    std::fs::write(output_file, &bytes)?;

    println!("✅ Speech generated successfully!");
    println!("📁 Saved to: {}", output_file);
    println!("🎵 Size: {} bytes", bytes.len());

    // Display any warnings
    if !result.warnings.is_empty() {
        println!("\n⚠️  Warnings:");
        for warning in &result.warnings {
            println!("  - {:?}", warning);
        }
    }

    println!("\n💡 Tip: Play the audio with:");
    println!("   ffplay {}", output_file);
    println!("   # or");
    println!("   open {}", output_file);

    Ok(())
}
