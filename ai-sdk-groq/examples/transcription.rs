/// Audio transcription example using Groq provider directly.
///
/// This example demonstrates how to use Groq's transcription model with only `ai-sdk-provider`,
/// calling the `do_generate()` method directly on the `TranscriptionModel` trait.
///
/// Features demonstrated:
/// - Creating a Groq provider using the builder pattern
/// - Getting a transcription model instance (Whisper)
/// - Calling `do_generate()` directly with `TranscriptionModelCallOptions`
/// - Configuring language and response format
/// - Using provider-specific options (timestamp granularities)
/// - Processing transcription segments with timestamps
///
/// Run with:
/// ```bash
/// export GROQ_API_KEY="your-api-key"
/// cargo run --example transcription -p ai-sdk-groq
/// ```
use ai_sdk_groq::GroqClient;
use ai_sdk_provider::TranscriptionModel;
use ai_sdk_provider::transcription_model::call_options::{
    TranscriptionAudioData, TranscriptionModelCallOptions,
};
use std::collections::HashMap;
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎤 Groq Transcription Example\n");

    // Check API key is set
    let api_key =
        env::var("GROQ_API_KEY").map_err(|_| "GROQ_API_KEY environment variable not set")?;

    println!("✓ API key loaded from environment\n");

    // Create Groq provider using builder pattern
    let provider = GroqClient::new().api_key(api_key).build();
    println!("✓ Groq provider created");

    // Get transcription model (Whisper)
    let model = provider.transcription_model("whisper-large-v3-turbo");
    println!("✓ Model loaded: {}", model.model_id());
    println!("✓ Provider: {}\n", model.provider());

    // Read audio file (or create sample audio for testing)
    // For this example, we'll check for an audio file or use the speech output
    let audio_path = "sample_audio.mp3";

    if !std::path::Path::new(audio_path).exists() {
        println!("⚠️  Note: '{}' not found.", audio_path);
        println!("   To run this example, provide an audio file (mp3, wav, etc.)");
        println!("   You can use the output from the speech_generation example:");
        println!("   cargo run --example speech_generation -p ai-sdk-groq");
        println!("   mv output_groq_speech.wav sample_audio.mp3");
        println!("   cargo run --example transcription -p ai-sdk-groq\n");

        // Try to use output_groq_speech.wav if it exists from speech_generation example
        if std::path::Path::new("output_groq_speech.wav").exists() {
            println!("✓ Found 'output_groq_speech.wav', using that instead...\n");
            let audio_bytes = fs::read("output_groq_speech.wav")?;
            return transcribe_audio(&model, audio_bytes, "audio/wav").await;
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
    let mut groq_options = HashMap::new();
    groq_options.insert("language".to_string(), serde_json::json!("en"));
    groq_options.insert(
        "responseFormat".to_string(),
        serde_json::json!("verbose_json"),
    );
    groq_options.insert(
        "timestampGranularities".to_string(),
        serde_json::json!(["word", "segment"]),
    );
    provider_options.insert("groq".to_string(), groq_options);

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
            "\n📊 Transcription Segments ({} segments):",
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

    println!("\n✅ Example completed successfully!");
    println!("\n💡 Key Features Demonstrated:");
    println!("   ✓ Using do_generate() directly (provider-only)");
    println!("   ✓ Whisper transcription with Groq");
    println!("   ✓ Language configuration");
    println!("   ✓ Verbose JSON response format");
    println!("   ✓ Timestamp granularities (word and segment)");
    println!("   ✓ Segment-level transcription data");

    Ok(())
}
