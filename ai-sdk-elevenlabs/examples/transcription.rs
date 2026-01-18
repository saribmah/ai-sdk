/// Audio transcription example using ElevenLabs provider directly.
///
/// This example demonstrates how to use ElevenLabs' transcription model with only `llm-kit-provider`,
/// calling the `do_generate()` method directly on the `TranscriptionModel` trait.
///
/// Features demonstrated:
/// - Creating an ElevenLabs provider using the builder pattern
/// - Getting a transcription model instance
/// - Calling `do_generate()` directly with `TranscriptionModelCallOptions`
/// - Configuring diarization (speaker identification)
/// - Using provider-specific options (language code, timestamps)
/// - Processing transcription segments with timestamps
///
/// Run with:
/// ```bash
/// export ELEVENLABS_API_KEY="your-api-key"
/// cargo run --example transcription -p llm-kit-elevenlabs
/// ```
use llm_kit_elevenlabs::ElevenLabsClient;
use llm_kit_provider::transcription_model::call_options::{
    TranscriptionAudioData, TranscriptionModelCallOptions,
};
use llm_kit_provider::{Provider, TranscriptionModel};
use std::collections::HashMap;
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎤 ElevenLabs Transcription Example\n");

    // Check API key is set
    let _api_key = env::var("ELEVENLABS_API_KEY")
        .map_err(|_| "ELEVENLABS_API_KEY environment variable not set")?;

    println!("✓ API key loaded from environment\n");

    // Create ElevenLabs provider using builder pattern
    let provider = ElevenLabsClient::new().build();
    println!("✓ ElevenLabs provider created");

    // Get transcription model
    let model = provider.transcription_model("scribe_v1")?;
    println!("✓ Model loaded: {}", model.model_id());
    println!("✓ Provider: {}\n", model.provider());

    // Read audio file (or create sample audio for testing)
    // For this example, we'll check for an audio file or create a simple one
    let audio_path = "sample_audio.mp3";

    if !std::path::Path::new(audio_path).exists() {
        println!("⚠️  Note: '{}' not found.", audio_path);
        println!("   To run this example, provide an audio file (mp3, wav, etc.)");
        println!("   You can use the output from the speech_generation example:");
        println!("   cargo run --example speech_generation -p llm-kit-elevenlabs");
        println!("   mv output_speech.mp3 sample_audio.mp3");
        println!("   cargo run --example transcription -p llm-kit-elevenlabs\n");

        // Try to use output_speech.mp3 if it exists from speech_generation example
        if std::path::Path::new("output_speech.mp3").exists() {
            println!("✓ Found 'output_speech.mp3', using that instead...\n");
            let audio_bytes = fs::read("output_speech.mp3")?;
            return transcribe_audio(&model, audio_bytes, "audio/mpeg").await;
        }

        return Err("No audio file found. Please provide sample_audio.mp3 or run speech_generation example first.".into());
    }

    println!("📁 Reading audio file: {}", audio_path);
    let audio_bytes = fs::read(audio_path)?;
    println!("✓ Audio loaded: {} bytes\n", audio_bytes.len());

    transcribe_audio(&model, audio_bytes, "audio/mpeg").await
}

async fn transcribe_audio(
    model: &std::sync::Arc<dyn TranscriptionModel>,
    audio_bytes: Vec<u8>,
    media_type: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Configure provider-specific options for transcription
    let mut provider_options = HashMap::new();
    let mut elevenlabs_options = HashMap::new();
    elevenlabs_options.insert("diarize".to_string(), serde_json::json!(true));
    elevenlabs_options.insert("languageCode".to_string(), serde_json::json!("en"));
    elevenlabs_options.insert("tagAudioEvents".to_string(), serde_json::json!(true));
    elevenlabs_options.insert(
        "timestampsGranularity".to_string(),
        serde_json::json!("word"),
    );
    provider_options.insert("elevenlabs".to_string(), elevenlabs_options);

    // Configure transcription options
    let options = TranscriptionModelCallOptions {
        audio: TranscriptionAudioData::Binary(audio_bytes),
        media_type: media_type.to_string(),
        headers: None,
        provider_options: Some(provider_options),
        abort_signal: None,
    };

    // Transcribe audio using do_generate() directly
    println!("🔊 Transcribing audio...");
    let result = model.do_generate(options).await?;

    println!("✓ Transcription completed successfully!\n");

    // Display full transcription
    println!("📝 Full Transcription:");
    println!("   {}\n", result.text);

    // Display language (if detected)
    if let Some(ref language) = result.language {
        println!("🌐 Detected Language: {}", language);
    }

    // Display duration (if available)
    if let Some(duration) = result.duration_in_seconds {
        println!("⏱️  Duration: {:.2} seconds", duration);
    }

    // Display segments with timestamps
    if !result.segments.is_empty() {
        println!(
            "\n📊 Transcription Segments ({} words):",
            result.segments.len()
        );
        for (i, segment) in result.segments.iter().enumerate().take(20) {
            println!(
                "   [{:>2}] {:.2}s - {:.2}s: {}",
                i + 1,
                segment.start_second,
                segment.end_second,
                segment.text
            );
        }
        if result.segments.len() > 20 {
            println!("   ... and {} more segments", result.segments.len() - 20);
        }
    }

    // Display response metadata
    println!("\n📋 Response Metadata:");
    println!("   Model ID: {}", result.response.model_id);
    println!("   Timestamp: {:?}", result.response.timestamp);

    // Display any warnings
    if !result.warnings.is_empty() {
        println!("\n⚠️  Warnings:");
        for warning in &result.warnings {
            println!("  - {:?}", warning);
        }
    }

    Ok(())
}
