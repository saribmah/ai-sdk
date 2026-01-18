/// Speech generation example using ElevenLabs provider directly.
///
/// This example demonstrates how to use ElevenLabs' speech model with only `llm-kit-provider`,
/// calling the `do_generate()` method directly on the `SpeechModel` trait.
///
/// Features demonstrated:
/// - Creating an ElevenLabs provider using the builder pattern
/// - Getting a speech model instance
/// - Calling `do_generate()` directly with `SpeechModelCallOptions`
/// - Configuring voice, output format, and speed
/// - Using provider-specific options (voice settings)
/// - Saving generated audio to a file
///
/// Run with:
/// ```bash
/// export ELEVENLABS_API_KEY="your-api-key"
/// cargo run --example speech_generation -p llm-kit-elevenlabs
/// ```
use llm_kit_elevenlabs::ElevenLabsClient;
use llm_kit_provider::Provider;
use llm_kit_provider::speech_model::call_options::SpeechModelCallOptions;
use std::collections::HashMap;
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎙️  ElevenLabs Speech Generation Example\n");

    // Check API key is set
    let _api_key = env::var("ELEVENLABS_API_KEY")
        .map_err(|_| "ELEVENLABS_API_KEY environment variable not set")?;

    println!("✓ API key loaded from environment\n");

    // Create ElevenLabs provider using builder pattern
    let provider = ElevenLabsClient::new().build();
    println!("✓ ElevenLabs provider created");

    // Get speech model
    let model = provider.speech_model("eleven_multilingual_v2")?;
    println!("✓ Model loaded: {}", model.model_id());
    println!("✓ Provider: {}\n", model.provider());

    // Text to convert to speech
    let text = "Hello! This is a test of the ElevenLabs text-to-speech API. \
                It supports multiple languages and natural-sounding voices with \
                customizable voice settings.";
    println!("📝 Text to speak: {}\n", text);

    // Configure provider-specific options for voice settings
    let mut provider_options = HashMap::new();
    let mut elevenlabs_options = HashMap::new();
    elevenlabs_options.insert(
        "voiceSettings".to_string(),
        serde_json::json!({
            "stability": 0.7,
            "similarityBoost": 0.8,
            "style": 0.5,
            "useSpeakerBoost": true
        }),
    );
    provider_options.insert("elevenlabs".to_string(), elevenlabs_options);

    // Configure speech generation options
    let options = SpeechModelCallOptions {
        text: text.to_string(),
        voice: Some("21m00Tcm4TlvDq8ikWAM".to_string()), // Rachel voice
        output_format: Some("mp3_44100_128".to_string()),
        speed: Some(1.0),
        language: Some("en".to_string()),
        instructions: None,
        headers: None,
        provider_options: Some(provider_options),
        abort_signal: None,
    };

    // Generate speech using do_generate() directly
    println!("🔊 Generating speech...");
    let result = model.do_generate(options).await?;

    println!("✓ Speech generated successfully!");

    let audio_bytes = match result.audio {
        llm_kit_provider::speech_model::AudioData::Binary(bytes) => bytes,
        llm_kit_provider::speech_model::AudioData::Base64(base64_str) => {
            use base64::{Engine as _, engine::general_purpose};
            general_purpose::STANDARD.decode(base64_str)?
        }
    };

    println!("  Audio size: {} bytes", audio_bytes.len());
    println!("  Model ID: {}", result.response.model_id);

    // Display any warnings
    if !result.warnings.is_empty() {
        println!("\n⚠️  Warnings:");
        for warning in &result.warnings {
            println!("  - {:?}", warning);
        }
    }

    // Save audio to file
    let output_file = "output_speech.mp3";
    fs::write(output_file, &audio_bytes)?;
    println!("\n💾 Audio saved to: {}", output_file);
    println!(
        "   You can play it with: afplay {} (macOS) or mpg123 {} (Linux)",
        output_file, output_file
    );

    Ok(())
}
