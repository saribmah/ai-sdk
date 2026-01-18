/// Speech generation example using Groq provider directly.
///
/// This example demonstrates how to use Groq's speech model with only `llm-kit-provider`,
/// calling the `do_generate()` method directly on the `SpeechModel` trait.
///
/// Features demonstrated:
/// - Creating a Groq provider using the builder pattern
/// - Getting a speech model instance
/// - Calling `do_generate()` directly with `SpeechModelCallOptions`
/// - Configuring voice, output format, and speed
/// - Using provider-specific options (sample rate)
/// - Saving generated audio to a file
///
/// Run with:
/// ```bash
/// export GROQ_API_KEY="your-api-key"
/// cargo run --example speech_generation -p llm-kit-groq
/// ```
use llm_kit_groq::GroqClient;
use llm_kit_provider::speech_model::call_options::SpeechModelCallOptions;
use std::collections::HashMap;
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎙️  Groq Speech Generation Example\n");

    // Check API key is set
    let api_key =
        env::var("GROQ_API_KEY").map_err(|_| "GROQ_API_KEY environment variable not set")?;

    println!("✓ API key loaded from environment\n");

    // Create Groq provider using builder pattern
    let provider = GroqClient::new().api_key(api_key).build();
    println!("✓ Groq provider created");

    // Get speech model
    let model = provider.speech_model("playai-tts");
    println!("✓ Model loaded: {}", model.model_id());
    println!("✓ Provider: {}\n", model.provider());

    // Text to convert to speech
    let text = "Hello! This is a test of the Groq text-to-speech API using PlayAI. \
                It provides fast and natural-sounding voice synthesis with \
                customizable voice options and sample rates.";
    println!("📝 Text to speak: {}\n", text);

    // Configure provider-specific options for sample rate
    let mut provider_options = HashMap::new();
    let mut groq_options = HashMap::new();
    groq_options.insert("sampleRate".to_string(), serde_json::json!(24000));
    provider_options.insert("groq".to_string(), groq_options);

    // Configure speech generation options
    let options = SpeechModelCallOptions {
        text: text.to_string(),
        voice: Some("Fritz-PlayAI".to_string()), // PlayAI voice
        output_format: Some("wav".to_string()),
        speed: Some(1.0),
        language: None,     // Not supported by Groq
        instructions: None, // Not supported by Groq
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
    let output_file = "output_groq_speech.wav";
    fs::write(output_file, &audio_bytes)?;
    println!("\n💾 Audio saved to: {}", output_file);
    println!(
        "   You can play it with: afplay {} (macOS) or aplay {} (Linux)",
        output_file, output_file
    );

    println!("\n✅ Example completed successfully!");
    println!("\n💡 Key Features Demonstrated:");
    println!("   ✓ Using do_generate() directly (provider-only)");
    println!("   ✓ Speech generation with PlayAI TTS");
    println!("   ✓ Voice configuration");
    println!("   ✓ Output format selection (WAV)");
    println!("   ✓ Speed control");
    println!("   ✓ Provider-specific options (sample rate)");

    Ok(())
}
