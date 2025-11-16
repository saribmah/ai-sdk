# AI SDK ElevenLabs Provider

ElevenLabs provider for the AI SDK, supporting text-to-speech (TTS) and speech-to-text (STT) models.

## Features

- ✅ **Text-to-Speech (TTS)**: Convert text to natural-sounding speech
- ✅ **Speech-to-Text (STT)**: Transcribe audio files to text with timestamps
- ✅ **Multiple Voices**: Support for 9+ default voices
- ✅ **Voice Settings**: Fine-tune stability, similarity, style, and speed
- ✅ **Speaker Diarization**: Identify different speakers in transcriptions
- ✅ **29+ Languages**: Multilingual support
- ✅ **Multiple Audio Formats**: MP3, PCM, WAV, and more

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ai-sdk-core = "0.1"
ai-sdk-elevenlabs = "0.1"
```

## Quick Start

### Recommended: Builder Pattern

```rust
use ai_sdk_core::GenerateSpeech;
use ai_sdk_elevenlabs::ElevenLabsClient;
use ai_sdk_provider::Provider;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider using builder (reads ELEVENLABS_API_KEY from environment)
    let provider = ElevenLabsClient::new().build();
    let model = provider.speech_model("eleven_multilingual_v2")?;

    // Generate speech
    let result = GenerateSpeech::new(model, "Hello, world!".to_string())
        .voice("21m00Tcm4TlvDq8ikWAM") // Rachel voice
        .output_format("mp3")
        .execute()
        .await?;

    // Save audio
    std::fs::write("output.mp3", result.audio.bytes())?;
    Ok(())
}
```

### Alternative: Direct Instantiation

```rust
use ai_sdk_elevenlabs::{ElevenLabsProvider, ElevenLabsProviderSettings};
use ai_sdk_provider::Provider;

let provider = ElevenLabsProvider::new(
    ElevenLabsProviderSettings::new()
        .with_api_key("your-api-key")
);
```

## Usage Examples

### Text-to-Speech

```rust
use ai_sdk_core::GenerateSpeech;
use ai_sdk_elevenlabs::ElevenLabsClient;
use ai_sdk_provider::Provider;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = ElevenLabsClient::new().build();
    let model = provider.speech_model("eleven_multilingual_v2")?;

    let result = GenerateSpeech::new(model, "Hello, world!".to_string())
        .voice("21m00Tcm4TlvDq8ikWAM")
        .output_format("mp3")
        .execute()
        .await?;

    std::fs::write("output.mp3", result.audio.bytes())?;
    Ok(())
}
```

### Speech-to-Text Transcription

```rust
use ai_sdk_core::Transcribe;
use ai_sdk_elevenlabs::ElevenLabsClient;
use ai_sdk_provider::Provider;
use ai_sdk_provider::transcription_model::call_options::TranscriptionAudioData;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = ElevenLabsClient::new().build();
    let model = provider.transcription_model("scribe_v1")?;

    let audio_bytes = std::fs::read("audio.mp3")?;
    let result = Transcribe::new(model, TranscriptionAudioData::Binary(audio_bytes))
        .media_type("audio/mpeg")
        .execute()
        .await?;

    println!("Transcription: {}", result.text);
    Ok(())
}
```

## Configuration

### Builder Pattern

```rust
use ai_sdk_elevenlabs::ElevenLabsClient;

// With API key
let provider = ElevenLabsClient::new()
    .api_key("your-api-key")
    .build();

// With custom base URL
let provider = ElevenLabsClient::new()
    .api_key("your-api-key")
    .base_url("https://custom-api.elevenlabs.io")
    .build();

// With custom headers
let provider = ElevenLabsClient::new()
    .api_key("your-api-key")
    .header("X-Custom-Header", "custom-value")
    .build();
```

### Direct Instantiation

```rust
use ai_sdk_elevenlabs::{ElevenLabsProvider, ElevenLabsProviderSettings};

let settings = ElevenLabsProviderSettings::new()
    .with_api_key("your-api-key")
    .with_base_url("https://api.elevenlabs.io");

let provider = ElevenLabsProvider::new(settings);
```

### Voice Settings

```rust
use std::collections::HashMap;
use serde_json::json;

let mut provider_options = HashMap::new();
provider_options.insert(
    "elevenlabs".to_string(),
    json!({
        "voiceSettings": {
            "stability": 0.7,
            "similarityBoost": 0.8,
            "style": 0.5,
            "useSpeakerBoost": true
        }
    }).as_object().unwrap().clone(),
);

let result = GenerateSpeech::new(model, "Text with custom voice settings".to_string())
    .provider_options(provider_options)
    .execute()
    .await?;
```

## Available Voices

The crate provides constants for common voices:

```rust
use ai_sdk_elevenlabs::speech::options::voices;

// Use predefined voice constants
let result = GenerateSpeech::new(model, "Hello!".to_string())
    .voice(voices::RACHEL)  // or DOMI, BELLA, ANTONI, JOSH, etc.
    .execute()
    .await?;
```

## Examples

Run the included examples:

```bash
# Text-to-Speech
export ELEVENLABS_API_KEY="your-api-key"
cargo run --example basic_speech -p ai-sdk-elevenlabs
```

## API Reference

See the [API documentation](https://docs.rs/ai-sdk-elevenlabs) for detailed information.

## License

MIT
