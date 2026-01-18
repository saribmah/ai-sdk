# LLM Kit Groq

Groq provider for [LLM Kit](https://github.com/saribmah/llm-kit) - Ultra-fast LLM inference with open-source models powered by Groq's LPU architecture.

> **Note**: This provider uses the standardized builder pattern. See the [Quick Start](#quick-start) section for the recommended usage.

## Features

- **Text Generation**: Generate text with Llama, Gemma, DeepSeek, Qwen, and other open-source models
- **Streaming**: Real-time response streaming with ultra-fast inference speeds
- **Tool Calling**: Support for function calling with chat models
- **Speech Generation**: Text-to-speech with PlayAI models
- **Transcription**: Audio-to-text with Whisper models (large-v3, large-v3-turbo, distilled)
- **Ultra-Fast Inference**: Groq's LPU architecture delivers industry-leading inference speeds
- **Provider Metadata**: Access cached token counts and performance metrics

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-kit-groq = "0.1"
llm-kit-core = "0.1"
llm-kit-provider = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Using the Client Builder (Recommended)

```rust
use llm_kit_groq::GroqClient;
use llm_kit_provider::language_model::LanguageModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider using the client builder
    let provider = GroqClient::new()
        .api_key("your-api-key")  // Or use GROQ_API_KEY env var
        .build();
    
    // Create a language model
    let model = provider.chat_model("llama-3.1-8b-instant");
    
    println!("Model: {}", model.model_id());
    println!("Provider: {}", model.provider());
    Ok(())
}
```

### Using Settings Directly (Alternative)

```rust
use llm_kit_groq::{GroqProvider, GroqProviderSettings};
use llm_kit_provider::language_model::LanguageModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider with settings
    let provider = GroqProvider::new(GroqProviderSettings::default());
    
    let model = provider.chat_model("llama-3.1-8b-instant");
    
    println!("Model: {}", model.model_id());
    Ok(())
}
```

## Configuration

### Environment Variables

Set your Groq API key as an environment variable:

```bash
export GROQ_API_KEY=your-api-key
export GROQ_BASE_URL=https://api.groq.com/openai/v1  # Optional
```

### Using the Client Builder

```rust
use llm_kit_groq::GroqClient;

let provider = GroqClient::new()
    .api_key("your-api-key")
    .base_url("https://api.groq.com/openai/v1")
    .header("Custom-Header", "value")
    .name("my-groq-provider")
    .build();
```

### Using Settings Directly

```rust
use llm_kit_groq::{GroqProvider, GroqProviderSettings};

let settings = GroqProviderSettings::new()
    .with_api_key("your-api-key")
    .with_base_url("https://api.groq.com/openai/v1")
    .add_header("Custom-Header", "value")
    .with_name("my-groq-provider");

let provider = GroqProvider::new(settings);
```

### Builder Methods

The `GroqClient` builder supports:

- `.api_key(key)` - Set the API key (overrides `GROQ_API_KEY` environment variable)
- `.load_api_key_from_env()` - Explicitly load API key from `GROQ_API_KEY` environment variable
- `.base_url(url)` - Set custom base URL (default: `https://api.groq.com/openai/v1`)
- `.name(name)` - Set provider name (optional)
- `.header(key, value)` - Add a single custom header
- `.headers(map)` - Add multiple custom headers
- `.build()` - Build the provider

## Supported Models

All Groq models are supported across multiple model families.

### Chat Models

- **Llama**: `llama-3.1-8b-instant`, `llama-3.3-70b-versatile`, `meta-llama/llama-guard-4-12b`, `meta-llama/llama-4-maverick-17b-128e-instruct`, `meta-llama/llama-4-scout-17b-16e-instruct`
- **Gemma**: `gemma2-9b-it`
- **DeepSeek**: `deepseek-r1-distill-llama-70b`
- **Qwen**: `qwen/qwen3-32b`
- **Mixtral**: `mixtral-8x7b-32768`

### Transcription Models (Whisper)

- **`whisper-large-v3`** - Most accurate Whisper model
- **`whisper-large-v3-turbo`** - Faster Whisper variant with lower latency
- **`distil-whisper-large-v3-en`** - English-optimized distilled model for faster performance

### Text-to-Speech Models

- **`playai-tts`** - PlayAI text-to-speech model

For a complete list of available models, see the [Groq Models documentation](https://console.groq.com/docs/models).

## Provider-Specific Options

Groq supports advanced features through provider options.

### Reasoning Format

Control how reasoning content is returned:

```rust
use llm_kit_core::GenerateText;
use serde_json::json;

let result = GenerateText::new(model, prompt)
    .provider_options(json!({
        "reasoningFormat": "parsed"  // "parsed", "raw", or "hidden"
    }))
    .execute()
    .await?;
```

### Reasoning Effort

Configure computational effort for reasoning models:

```rust
use llm_kit_core::GenerateText;
use serde_json::json;

let result = GenerateText::new(model, prompt)
    .provider_options(json!({
        "reasoningEffort": "high"  // Effort level as string
    }))
    .execute()
    .await?;
```

### Parallel Tool Calls

Enable or disable parallel tool execution (default: true):

```rust
use llm_kit_core::GenerateText;
use serde_json::json;

let result = GenerateText::new(model, prompt)
    .tools(tools)
    .provider_options(json!({
        "parallelToolCalls": true
    }))
    .execute()
    .await?;
```

### Service Tier

Select the service tier for processing:

```rust
use llm_kit_core::GenerateText;
use serde_json::json;

let result = GenerateText::new(model, prompt)
    .provider_options(json!({
        "serviceTier": "flex"  // "on_demand", "flex", or "auto"
    }))
    .execute()
    .await?;
```

### Cached Tokens Metadata

Groq provides metadata about cached tokens to help optimize performance:

```rust
use llm_kit_groq::GroqClient;
use llm_kit_core::{GenerateText, Prompt};

let provider = GroqClient::new().build();
let model = provider.chat_model("llama-3.1-8b-instant");

let result = GenerateText::new(model, Prompt::text("Hello!"))
    .execute()
    .await?;

// Access cache metadata
if let Some(metadata) = result.provider_metadata {
    if let Some(groq) = metadata.get("groq") {
        println!("Cached tokens: {:?}", groq.get("cachedTokens"));
    }
}
```

### Available Provider Options

| Option | Type | Description |
|--------|------|-------------|
| `reasoningFormat` | `string` | Reasoning content format: "parsed", "raw", "hidden" |
| `reasoningEffort` | `string` | Computational effort level for reasoning |
| `parallelToolCalls` | `bool` | Enable parallel tool execution (default: true) |
| `structuredOutputs` | `bool` | Enable structured outputs (default: true) |
| `serviceTier` | `string` | Service tier: "on_demand", "flex", "auto" |
| `user` | `string` | End-user identifier for abuse monitoring |

## Examples

See the `examples/` directory for complete examples:

- `chat.rs` - Basic chat completion using `do_generate()` directly
- `stream.rs` - Streaming responses using `do_stream()` directly
- `chat_tool_calling.rs` - Tool calling using `do_generate()` directly
- `stream_tool_calling.rs` - Streaming with tools using `do_stream()` directly
- `speech_generation.rs` - Text-to-speech using `do_generate()` directly
- `transcription.rs` - Audio transcription using `do_transcribe()` directly

Run examples with:

```bash
export GROQ_API_KEY="your-api-key"
cargo run --example chat
cargo run --example stream
cargo run --example chat_tool_calling
cargo run --example stream_tool_calling
cargo run --example speech_generation
cargo run --example transcription
```

## Documentation

- [API Documentation](https://docs.rs/llm-kit-groq)
- [LLM Kit Documentation](https://github.com/saribmah/llm-kit)
- [Groq API Reference](https://console.groq.com/docs)
- [Groq Models](https://console.groq.com/docs/models)

## License

Licensed under:

- MIT license ([LICENSE-MIT](LICENSE-MIT))

## Contributing

Contributions are welcome! Please see the [Contributing Guide](../CONTRIBUTING.md) for more details.
