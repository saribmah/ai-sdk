# AI SDK ElevenLabs

ElevenLabs provider for [LLM Kit](https://github.com/saribmah/llm-kit) - High-quality text-to-speech and speech-to-text with natural-sounding voices and advanced voice customization.

> **Note**: This provider uses the standardized builder pattern. See the [Quick Start](#quick-start) section for the recommended usage.

## Features

- **Text-to-Speech**: Convert text to natural-sounding speech with 9+ default voices
- **Speech-to-Text**: Transcribe audio files to text with word-level timestamps
- **Voice Settings**: Fine-tune stability, similarity boost, style, and speaker boost
- **Speaker Diarization**: Identify different speakers in transcriptions
- **Multilingual**: Support for 29+ languages
- **Multiple Audio Formats**: MP3, PCM, WAV, and more

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-kit-elevenlabs = "0.1"
llm-kit-core = "0.1"
llm-kit-provider = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Using the Client Builder (Recommended)

```rust
use llm_kit_elevenlabs::ElevenLabsClient;
use llm_kit_provider::{Provider, SpeechModel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider using the client builder
    let provider = ElevenLabsClient::new()
        .api_key("your-api-key")  // Or use ELEVENLABS_API_KEY env var
        .build();
    
    // Create a speech model
    let model = provider.speech_model("eleven_multilingual_v2")?;
    
    println!("Model: {}", model.model_id());
    println!("Provider: {}", model.provider());
    Ok(())
}
```

### Using Settings Directly (Alternative)

```rust
use llm_kit_elevenlabs::{ElevenLabsProvider, ElevenLabsProviderSettings};
use llm_kit_provider::{Provider, SpeechModel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider with settings
    let provider = ElevenLabsProvider::new(ElevenLabsProviderSettings::default());
    
    let model = provider.speech_model("eleven_multilingual_v2")?;
    
    println!("Model: {}", model.model_id());
    Ok(())
}
```



## Configuration

### Environment Variables

Set your ElevenLabs API key as an environment variable:

```bash
export ELEVENLABS_API_KEY=your-api-key
```

### Using the Client Builder

```rust
use llm_kit_elevenlabs::ElevenLabsClient;

let provider = ElevenLabsClient::new()
    .api_key("your-api-key")
    .base_url("https://api.elevenlabs.io")
    .header("X-Custom-Header", "value")
    .name("my-elevenlabs-provider")
    .build();
```

### Builder Methods

The `ElevenLabsClient` builder supports:

- `.api_key(key)` - Set the API key
- `.base_url(url)` - Set custom base URL
- `.name(name)` - Set provider name
- `.header(key, value)` - Add a single custom header
- `.headers(map)` - Add multiple custom headers
- `.build()` - Build the provider

## Supported Models

### Text-to-Speech Models

- **`eleven_multilingual_v2`**: Multilingual model supporting 29+ languages
- **`eleven_turbo_v2`**: Faster, lower-latency model
- **`eleven_monolingual_v1`**: English-only model with high quality

### Speech-to-Text Models

- **`scribe_v1`**: High-accuracy transcription model with speaker diarization

```rust
// Text-to-speech model
let tts_model = provider.speech_model("eleven_multilingual_v2")?;

// Speech-to-text model
let stt_model = provider.transcription_model("scribe_v1")?;
```

## Provider-Specific Options

ElevenLabs supports advanced features through provider options for both speech generation and transcription.

### Voice Settings (Text-to-Speech)

Customize voice characteristics:

```rust
use llm_kit_core::GenerateSpeech;
use std::collections::HashMap;
use serde_json::json;

let mut provider_options = HashMap::new();
provider_options.insert(
    "elevenlabs".to_string(),
    json!({
        "voiceSettings": {
            "stability": 0.7,          // 0.0-1.0: Higher = more consistent
            "similarityBoost": 0.8,    // 0.0-1.0: Higher = closer to original voice
            "style": 0.5,              // 0.0-1.0: Voice style/expression
            "useSpeakerBoost": true    // Enhance voice clarity
        },
        "seed": 12345                  // For reproducible output
    }).as_object().unwrap().clone(),
);

let result = GenerateSpeech::new(model, "Hello with custom voice!".to_string())
    .provider_options(provider_options)
    .execute()
    .await?;
```

### Available Voices

Common ElevenLabs voices:

- **`21m00Tcm4TlvDq8ikWAM`** - Rachel (American, Female)
- **`AZnzlk1XvdvUeBnXmlld`** - Domi (American, Female)
- **`EXAVITQu4vr4xnSDxMaL`** - Bella (American, Female)
- **`ErXwobaYiN019PkySvjV`** - Antoni (American, Male)
- **`TxGEqnHWrfWFTfGW9XjX`** - Josh (American, Male)
- **`VR6AewLTigWG4xSOukaG`** - Arnold (American, Male)
- **`pNInz6obpgDQGcFmaJgB`** - Adam (American, Male)
- **`yoZ06aMxZJJ28mfd3POQ`** - Sam (American, Male)

Or use your own custom voices from your ElevenLabs account.

### Speaker Diarization (Speech-to-Text)

Identify different speakers in transcriptions:

```rust
use llm_kit_core::Transcribe;
use llm_kit_provider::transcription_model::AudioInput;
use std::collections::HashMap;
use serde_json::json;

let mut provider_options = HashMap::new();
provider_options.insert(
    "elevenlabs".to_string(),
    json!({
        "diarize": true,                          // Enable speaker diarization
        "languageCode": "en",                     // Specify language
        "tagAudioEvents": true,                   // Tag non-speech events
        "timestampsGranularity": "word"          // word or segment
    }).as_object().unwrap().clone(),
);

let result = Transcribe::new(model, AudioInput::Data(audio_bytes))
    .provider_options(provider_options)
    .execute()
    .await?;
```

### Available Provider Options

**Text-to-Speech Options:**

| Option | Type | Description |
|--------|------|-------------|
| `voiceSettings.stability` | `f64` | Voice consistency (0.0-1.0) |
| `voiceSettings.similarityBoost` | `f64` | Voice similarity to original (0.0-1.0) |
| `voiceSettings.style` | `f64` | Voice style/expression (0.0-1.0) |
| `voiceSettings.useSpeakerBoost` | `bool` | Enhance voice clarity |
| `seed` | `i32` | Seed for reproducible output |

**Speech-to-Text Options:**

| Option | Type | Description |
|--------|------|-------------|
| `diarize` | `bool` | Enable speaker diarization |
| `languageCode` | `string` | Language code (e.g., 'en', 'es', 'fr') |
| `tagAudioEvents` | `bool` | Tag non-speech audio events |
| `timestampsGranularity` | `string` | 'word' or 'segment' level timestamps |

## Examples

See the `examples/` directory for complete examples:

- `speech_generation.rs` - Text-to-speech using `do_generate()` directly
- `transcription.rs` - Speech-to-text using `do_generate()` directly

Run examples with:

```bash
export ELEVENLABS_API_KEY="your-api-key"
cargo run --example speech_generation
cargo run --example transcription
```

## Documentation

- [API Documentation](https://docs.rs/llm-kit-elevenlabs)
- [AI SDK Documentation](https://github.com/saribmah/llm-kit)
- [ElevenLabs API Reference](https://elevenlabs.io/docs/api-reference)

## License

MIT

## Contributing

Contributions are welcome! Please see the [Contributing Guide](../CONTRIBUTING.md) for more details.
