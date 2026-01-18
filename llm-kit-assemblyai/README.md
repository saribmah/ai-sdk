# LLM Kit AssemblyAI

AssemblyAI provider for [LLM Kit](https://github.com/saribmah/llm-kit) - High-quality speech-to-text transcription with advanced features like speaker diarization, sentiment analysis, and PII redaction.

> **Note**: This provider uses the standardized builder pattern. See the [Quick Start](#quick-start) section for the recommended usage.

## Features

- **Transcription**: High-quality speech-to-text transcription with 100+ language support
- **Speaker Diarization**: Identify and label different speakers in audio
- **Sentiment Analysis**: Analyze sentiment of transcribed text
- **Auto-Chapters and Highlights**: Automatically generate chapters and key highlights
- **PII Redaction**: Redact personally identifiable information from transcripts
- **Language Detection**: Automatic language detection for audio files
- **Two Models**: `best` (highest accuracy) and `nano` (faster, lower cost)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-kit-assemblyai = "0.1"
llm-kit-core = "0.1"
llm-kit-provider = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Using the Client Builder (Recommended)

```rust
use llm_kit_assemblyai::AssemblyAIClient;
use llm_kit_provider::TranscriptionModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider using the client builder
    let provider = AssemblyAIClient::new()
        .api_key("your-api-key")  // Or use ASSEMBLYAI_API_KEY env var
        .build();
    
    // Create a transcription model
    let model = provider.transcription_model("best");
    
    println!("Model: {}", model.model_id());
    println!("Provider: {}", model.provider());
    Ok(())
}
```

### Using Settings Directly (Alternative)

```rust
use llm_kit_assemblyai::{AssemblyAIProvider, AssemblyAIProviderSettings};
use llm_kit_provider::TranscriptionModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider with settings
    let provider = AssemblyAIProvider::new(AssemblyAIProviderSettings::default());
    
    let model = provider.transcription_model("best");
    
    println!("Model: {}", model.model_id());
    Ok(())
}
```

## Configuration

### Environment Variables

Set your AssemblyAI API key as an environment variable:

```bash
export ASSEMBLYAI_API_KEY=your-api-key
```

### Using the Client Builder

```rust
use llm_kit_assemblyai::AssemblyAIClient;

let provider = AssemblyAIClient::new()
    .api_key("your-api-key")
    .header("Custom-Header", "value")
    .polling_interval_ms(5000)  // Check transcription status every 5s
    .name("my-assemblyai-provider")
    .build();
```

### Builder Methods

The `AssemblyAIClient` builder supports:

- `.api_key(key)` - Set the API key
- `.name(name)` - Set provider name
- `.header(key, value)` - Add a single custom header
- `.headers(map)` - Add multiple custom headers
- `.polling_interval_ms(ms)` - Set polling interval for transcription status (default: 3000ms)
- `.build()` - Build the provider

## Supported Models

AssemblyAI provides two transcription models:

- **`best`**: Highest accuracy model (default)
- **`nano`**: Faster processing with lower cost

```rust
// Use the best model
let model = provider.transcription_model("best");

// Or use the nano model for faster processing
let model = provider.transcription_model("nano");
```

## Provider-Specific Options

AssemblyAI supports advanced transcription features through provider options. These can be passed using the `llm-kit-core` API or through the provider's direct interface.

### Using Provider Options

```rust
use llm_kit_core::Transcribe;
use llm_kit_provider::transcription_model::AudioInput;

// Enable speaker diarization
let result = Transcribe::new(model, AudioInput::Data(audio_data))
    .with_provider_options(serde_json::json!({
        "speakerLabels": true,
        "speakersExpected": 2
    }))
    .execute()
    .await?;
```

### Advanced Features

#### Speaker Diarization

Identify different speakers in the audio:

```rust
let result = Transcribe::new(model, AudioInput::Data(audio_data))
    .with_provider_options(serde_json::json!({
        "speakerLabels": true,
        "speakersExpected": 2
    }))
    .execute()
    .await?;
```

#### Sentiment Analysis

Analyze the sentiment of transcribed text:

```rust
let result = Transcribe::new(model, AudioInput::Data(audio_data))
    .with_provider_options(serde_json::json!({
        "sentimentAnalysis": true
    }))
    .execute()
    .await?;
```

#### PII Redaction

Redact personally identifiable information:

```rust
let result = Transcribe::new(model, AudioInput::Data(audio_data))
    .with_provider_options(serde_json::json!({
        "redactPii": true,
        "redactPiiPolicies": ["person_name", "phone_number", "email_address"]
    }))
    .execute()
    .await?;
```

#### Auto-Chapters and Highlights

Generate chapters and highlights automatically:

```rust
let result = Transcribe::new(model, AudioInput::Data(audio_data))
    .with_provider_options(serde_json::json!({
        "autoChapters": true,
        "autoHighlights": true
    }))
    .execute()
    .await?;
```

#### Language Detection

Automatically detect the language:

```rust
let result = Transcribe::new(model, AudioInput::Data(audio_data))
    .with_provider_options(serde_json::json!({
        "languageDetection": true
    }))
    .execute()
    .await?;
```

### Available Provider Options

All available provider options:

| Option | Type | Description |
|--------|------|-------------|
| `audioEndAt` | `i64` | End time in milliseconds |
| `audioStartFrom` | `i64` | Start time in milliseconds |
| `autoChapters` | `bool` | Enable auto chapter generation |
| `autoHighlights` | `bool` | Enable auto highlights |
| `boostParam` | `string` | Word boost level: 'low', 'default', 'high' |
| `contentSafety` | `bool` | Enable content moderation |
| `contentSafetyConfidence` | `i32` | Confidence threshold (25-100) |
| `customSpelling` | `array` | Custom spelling rules |
| `disfluencies` | `bool` | Include filler words |
| `entityDetection` | `bool` | Enable entity detection |
| `filterProfanity` | `bool` | Filter profanity |
| `formatText` | `bool` | Format text with punctuation |
| `iabCategories` | `bool` | Enable IAB categories |
| `languageCode` | `string` | Language code (e.g., 'en', 'es') |
| `languageConfidenceThreshold` | `f64` | Language detection confidence |
| `languageDetection` | `bool` | Auto language detection |
| `multichannel` | `bool` | Multichannel transcription |
| `punctuate` | `bool` | Add punctuation |
| `redactPii` | `bool` | Redact PII |
| `redactPiiAudio` | `bool` | Redact PII in audio |
| `redactPiiAudioQuality` | `string` | Audio format: 'mp3', 'wav' |
| `redactPiiPolicies` | `array` | PII types to redact |
| `redactPiiSub` | `string` | Substitution method: 'entity_name', 'hash' |
| `sentimentAnalysis` | `bool` | Enable sentiment analysis |
| `speakerLabels` | `bool` | Identify different speakers |
| `speakersExpected` | `i32` | Number of speakers expected |
| `speechThreshold` | `f64` | Speech detection threshold (0-1) |
| `summarization` | `bool` | Generate summary |
| `summaryModel` | `string` | Summary model: 'informative', 'conversational', 'catchy' |
| `summaryType` | `string` | Summary type: 'bullets', 'bullets_verbose', 'gist', 'headline', 'paragraph' |
| `webhookUrl` | `string` | Webhook URL for notifications |
| `wordBoost` | `array` | Words to boost recognition for |



## Examples

See the `examples/` directory for complete examples:

- `transcription.rs` - Basic transcription using `do_generate()` directly

Run examples with:

```bash
cargo run --example transcription
```

## Documentation

- [API Documentation](https://docs.rs/llm-kit-assemblyai)
- [LLM Kit Documentation](https://github.com/saribmah/llm-kit)
- [AssemblyAI API Reference](https://www.assemblyai.com/docs)

## License

Apache-2.0

## Contributing

Contributions are welcome! Please see the [Contributing Guide](../CONTRIBUTING.md) for more details.
