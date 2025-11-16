# AI SDK - AssemblyAI Provider

The **AssemblyAI provider** for the [AI SDK](https://github.com/saribmah/ai-sdk) provides transcription model support for the AssemblyAI API.

## Features

- 🎙️ High-quality speech-to-text transcription
- 🗣️ Speaker diarization (identify different speakers)
- 😊 Sentiment analysis
- 📝 Auto-chapters and highlights
- 🔒 PII redaction
- 🌍 100+ language support
- ⚡ Two models: `best` (highest accuracy) and `nano` (faster, lower cost)

## Setup

The AssemblyAI provider is available in the `ai-sdk-assemblyai` crate. You can add it to your project with:

```toml
[dependencies]
ai-sdk-assemblyai = "0.1"
ai-sdk-core = "*"
```

## Provider Instance

### Recommended: Using the Client Builder

```rust
use ai_sdk_assemblyai::AssemblyAIClient;

let provider = AssemblyAIClient::new()
    .api_key("your-api-key")
    .build();
```

If you don't provide an API key, it will be loaded from the `ASSEMBLYAI_API_KEY` environment variable:

```rust
use ai_sdk_assemblyai::AssemblyAIClient;

let provider = AssemblyAIClient::new().build();
```

### Alternative: Direct Instantiation

```rust
use ai_sdk_assemblyai::{AssemblyAIProvider, AssemblyAIProviderSettings};

let provider = AssemblyAIProvider::new(
    AssemblyAIProviderSettings::new()
        .with_api_key("your-api-key")
);
```

## Basic Example

```rust
use ai_sdk_assemblyai::{AssemblyAIClient, AssemblyAIProvider};
use ai_sdk_core::Transcribe;
use ai_sdk_provider::transcription_model::AudioInput;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a provider
    let provider = AssemblyAIClient::new()
        .api_key("your-api-key")
        .build();

    let model = provider.transcription_model("best");

    // Download audio file
    let audio_url = "https://example.com/audio.mp3";
    let audio_data = reqwest::get(audio_url).await?.bytes().await?;

    // Transcribe
    let result = Transcribe::new(model, AudioInput::Data(audio_data.to_vec()))
        .execute()
        .await?;

    println!("Transcription: {}", result.text);
    println!("Detected language: {:?}", result.language);
    println!("Duration: {:?} seconds", result.duration_in_seconds);

    Ok(())
}
```

## Advanced Features

AssemblyAI supports many advanced features through provider options:

### Speaker Diarization

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

### Sentiment Analysis

Analyze the sentiment of the transcribed text:

```rust
let result = Transcribe::new(model, AudioInput::Data(audio_data))
    .with_provider_options(serde_json::json!({
        "sentimentAnalysis": true
    }))
    .execute()
    .await?;
```

### PII Redaction

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

### Auto Chapters and Highlights

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

### Language Detection

Automatically detect the language:

```rust
let result = Transcribe::new(model, AudioInput::Data(audio_data))
    .with_provider_options(serde_json::json!({
        "languageDetection": true
    }))
    .execute()
    .await?;
```

## Provider Options

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

## Models

AssemblyAI provides two transcription models:

- **`best`**: Highest accuracy model (default)
- **`nano`**: Faster processing, lower cost

```rust
// Use the best model
let model = provider.transcription_model("best");

// Or use the nano model for faster processing
let model = provider.transcription_model("nano");
```

## Configuration

### API Key

Set your AssemblyAI API key either directly or via environment variable:

```rust
// Direct
let provider = AssemblyAIClient::new()
    .api_key("your-api-key")
    .build();

// Or use ASSEMBLYAI_API_KEY environment variable
let provider = AssemblyAIClient::new().build();
```

### Custom Headers

Add custom headers to requests:

```rust
let provider = AssemblyAIClient::new()
    .api_key("your-api-key")
    .header("X-Custom-Header", "value")
    .build();
```

### Polling Interval

Configure how often to check transcription status (default 3000ms):

```rust
let provider = AssemblyAIClient::new()
    .api_key("your-api-key")
    .polling_interval_ms(5000)  // Check every 5 seconds
    .build();
```

## Error Handling

The provider returns detailed error information:

```rust
match Transcribe::new(model, AudioInput::Data(audio_data))
    .execute()
    .await
{
    Ok(result) => println!("Success: {}", result.text),
    Err(e) => eprintln!("Transcription error: {}", e),
}
```

## Examples

See the `examples/` directory for complete examples:

- `basic_transcription.rs` - Basic transcription from URL
- `transcription_with_options.rs` - Using advanced features

Run examples with:

```bash
cargo run --example basic_transcription
cargo run --example transcription_with_options
```

## Documentation

For more information, see the [AssemblyAI API documentation](https://www.assemblyai.com/docs).

## License

Apache-2.0
