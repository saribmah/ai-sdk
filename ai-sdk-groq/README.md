# ai-sdk-groq

Groq provider for the AI SDK - Ultra-fast LLM inference with open-source models.

## Features

- ⚡ Ultra-fast inference speeds
- 🦙 Support for Llama, Gemma, and other open-source models
- 🌊 Streaming support
- 🛠️ Tool calling capabilities
- 🎤 Whisper transcription support
- 🔊 Text-to-speech (PlayAI models)
- 📊 Provider-specific metadata (cached tokens)
- 🖼️ Image URL support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ai-sdk-groq = "0.1.0"
ai-sdk-core = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use ai_sdk_groq::GroqClient;
use ai_sdk_core::{GenerateText, prompt::Prompt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider (reads from GROQ_API_KEY env var)
    let provider = GroqClient::new()
        .load_api_key_from_env()
        .build();

    // Create a model
    let model = provider.chat_model("llama-3.1-8b-instant");

    // Generate text
    let result = GenerateText::new(
        model,
        Prompt::text("Explain quantum computing briefly")
    )
    .temperature(0.7)
    .execute()
    .await?;

    println!("Response: {}", result.text);
    Ok(())
}
```

## Supported Models

### Chat Models
#### Production Models
- `llama-3.1-8b-instant` - Fast Llama 3.1 8B model
- `llama-3.3-70b-versatile` - Llama 3.3 70B model
- `gemma2-9b-it` - Google's Gemma 2 9B
- `meta-llama/llama-guard-4-12b` - Content moderation
- And more...

#### Preview Models
- `deepseek-r1-distill-llama-70b` - DeepSeek R1 reasoning
- `meta-llama/llama-4-maverick-17b-128e-instruct`
- `qwen/qwen3-32b`
- `mixtral-8x7b-32768`
- And more...

### Transcription Models (Whisper)
- `whisper-large-v3` - Most accurate Whisper model
- `whisper-large-v3-turbo` - Faster Whisper variant
- `distil-whisper-large-v3-en` - English-optimized distilled model

### Text-to-Speech Models
- `playai-tts` - PlayAI text-to-speech model

### Preview Models
- `deepseek-r1-distill-llama-70b` - DeepSeek R1 reasoning
- `meta-llama/llama-4-maverick-17b-128e-instruct`
- `meta-llama/llama-4-scout-17b-16e-instruct`
- `qwen/qwen3-32b`
- `mixtral-8x7b-32768`
- And more...

See [Groq's model documentation](https://console.groq.com/docs/models) for the full list.

## Usage Examples

### Basic Chat

```rust
use ai_sdk_groq::GroqClient;
use ai_sdk_core::{GenerateText, prompt::Prompt};

let provider = GroqClient::new()
    .api_key("your-api-key")
    .build();

let model = provider.chat_model("llama-3.1-8b-instant");

let result = GenerateText::new(model, Prompt::text("Hello!"))
    .execute()
    .await?;
```

### Streaming

```rust
use ai_sdk_groq::GroqClient;
use ai_sdk_core::{StreamText, prompt::Prompt};
use futures_util::StreamExt;

let provider = GroqClient::new()
    .load_api_key_from_env()
    .build();

let model = provider.chat_model("llama-3.1-8b-instant");

let result = StreamText::new(model, Prompt::text("Tell me a story"))
    .execute()
    .await?;

let mut stream = result.text_stream();
while let Some(delta) = stream.next().await {
    print!("{}", delta);
}
```

### Tool Calling

```rust
use ai_sdk_groq::GroqClient;
use ai_sdk_core::{GenerateText, prompt::Prompt, tool::Tool};
use serde_json::json;

let provider = GroqClient::new()
    .load_api_key_from_env()
    .build();

let model = provider.chat_model("llama-3.3-70b-versatile");

let get_weather = Tool::new(
    "get_weather",
    "Get weather for a location",
    json!({
        "type": "object",
        "properties": {
            "location": {"type": "string"}
        },
        "required": ["location"]
    }),
    |args| Box::pin(async move {
        Ok("Sunny, 72°F".to_string())
    })
);

let result = GenerateText::new(
    model,
    Prompt::text("What's the weather in SF?")
)
.tools(vec![get_weather])
.execute()
.await?;
```

### Provider Options

```rust
use ai_sdk_groq::{GroqClient, GroqProviderOptions, ReasoningFormat, ServiceTier};

// Configure provider-specific options
let options = GroqProviderOptions::new()
    .with_reasoning_format(ReasoningFormat::Parsed)
    .with_parallel_tool_calls(true)
    .with_structured_outputs(true)
    .with_service_tier(ServiceTier::Flex);
```

### Environment Variables

Set your API key as an environment variable:

```bash
export GROQ_API_KEY=your-api-key-here
```

Then use it:

```rust
let provider = GroqClient::new()
    .load_api_key_from_env()  // Reads GROQ_API_KEY
    .build();
```

## API Reference

### Client Builder

- `GroqClient::new()` - Create new client builder
- `.api_key(key)` - Set API key
- `.load_api_key_from_env()` - Load from `GROQ_API_KEY` env var
- `.base_url(url)` - Set custom base URL
- `.header(key, value)` - Add custom header
- `.build()` - Build the provider

### Provider

- `provider.chat_model(id)` - Create chat model
- `provider.model(id)` - Alias for `chat_model`

### Provider Options

- `reasoning_format` - "parsed", "raw", or "hidden"
- `reasoning_effort` - Reasoning effort level (string)
- `parallel_tool_calls` - Enable parallel tool execution (default: true)
- `structured_outputs` - Enable structured outputs (default: true)
- `service_tier` - "on_demand", "flex", or "auto"
- `user` - End-user identifier for abuse monitoring

## Examples

Check the `examples/` directory for more:

- `groq_basic_chat.rs` - Basic text generation
- `groq_streaming_chat.rs` - Streaming responses
- `groq_tool_calling.rs` - Tool/function calling
- `groq_transcription.rs` - Audio transcription (Whisper)
- `groq_text_to_speech.rs` - Text-to-speech synthesis

Run an example:

```bash
cargo run --example groq_basic_chat
```

## License

MIT

### Transcription

```rust
use ai_sdk_groq::{GroqClient, GroqTranscriptionOptions};
use ai_sdk_provider::transcription_model::call_options::TranscriptionModelCallOptions;
use ai_sdk_provider::TranscriptionModel;

let provider = GroqClient::new()
    .load_api_key_from_env()
    .build();

let model = provider.transcription_model("whisper-large-v3");

// Read audio file
let audio_data = std::fs::read("audio.mp3")?;

// Configure transcription options
let mut provider_options = std::collections::HashMap::new();
let groq_options = GroqTranscriptionOptions::new()
    .with_language("en")
    .with_verbose_json() // Get segments
    .with_temperature(0.0);

provider_options.insert(
    "groq".to_string(),
    serde_json::to_value(&groq_options)?,
);

let options = TranscriptionModelCallOptions::mp3(audio_data)
    .with_provider_options(provider_options);

let result = model.do_generate(options).await?;
println!("Transcription: {}", result.text);

// Access segments if verbose_json was used
for segment in &result.segments {
    println!("[{:.2}s - {:.2}s]: {}", 
        segment.start_second, 
        segment.end_second, 
        segment.text
    );
}
```

### Text-to-Speech

```rust
use ai_sdk_groq::{GroqClient, GroqSpeechOptions};
use ai_sdk_core::GenerateSpeech;

let provider = GroqClient::new()
    .load_api_key_from_env()
    .build();

let model = provider.speech_model("playai-tts");

// Configure speech options
let mut provider_options = std::collections::HashMap::new();
let mut groq_opts_map = std::collections::HashMap::new();
let groq_options = GroqSpeechOptions::new()
    .with_sample_rate(24000);

let groq_value = serde_json::to_value(&groq_options)?;
if let serde_json::Value::Object(map) = groq_value {
    for (k, v) in map {
        groq_opts_map.insert(k, v);
    }
}
provider_options.insert("groq".to_string(), groq_opts_map);

let result = GenerateSpeech::new(
    model,
    "Hello! This is a test of text-to-speech.".to_string()
)
.voice("Fritz-PlayAI".to_string())
.output_format("wav".to_string())
.speed(1.0)
.provider_options(provider_options)
.execute()
.await?;

// Save audio to file
match &result.audio {
    ai_sdk_provider::speech_model::AudioData::Binary(bytes) => {
        std::fs::write("output.wav", bytes)?;
        println!("Audio saved to output.wav");
    }
    _ => {}
}
```


