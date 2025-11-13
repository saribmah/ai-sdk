# AI SDK - DeepSeek Provider

The **DeepSeek provider** for the [AI SDK](https://github.com/yourusername/ai-sdk) contains language model support for the [DeepSeek](https://www.deepseek.com) platform.

## Features

- ✅ **Chat Completions** - Standard chat with `deepseek-chat`
- ✅ **Advanced Reasoning** - Reasoning model with `deepseek-reasoner` (R1)
- ✅ **Streaming Support** - Real-time response streaming
- ✅ **Tool Calling** - Function/tool calling support
- ✅ **Cache Metadata** - Prompt cache hit/miss token tracking

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ai-sdk-core = { path = "../ai-sdk-core" }
ai-sdk-deepseek = { path = "../ai-sdk-deepseek" }
tokio = { version = "1.41", features = ["full"] }
```

## Setup

### API Key

Get your API key from [DeepSeek](https://platform.deepseek.com/) and set it as an environment variable:

```bash
export DEEPSEEK_API_KEY=your-api-key-here
```

## Usage

### Basic Text Generation

```rust
use ai_sdk_core::{GenerateText, prompt::Prompt};
use ai_sdk_deepseek::DeepSeekClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = DeepSeekClient::new()
        .load_api_key_from_env()
        .build();

    let model = provider.chat_model("deepseek-chat");

    let result = GenerateText::new(
        model,
        Prompt::text("Write a Rust function to calculate factorial")
    )
    .temperature(0.7)
    .execute()
    .await?;

    println!("{}", result.output.text());
    Ok(())
}
```

### Streaming Responses

```rust
use ai_sdk_core::{StreamText, prompt::Prompt};
use ai_sdk_deepseek::DeepSeekClient;
use futures_util::StreamExt;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = DeepSeekClient::new()
        .load_api_key_from_env()
        .build();

    let model = provider.chat_model("deepseek-chat");

    let result = StreamText::new(
        Arc::from(model),
        Prompt::text("Tell me a story")
    )
    .execute()
    .await?;

    let mut text_stream = result.text_stream();
    while let Some(delta) = text_stream.next().await {
        print!("{}", delta);
    }

    Ok(())
}
```

### Advanced Reasoning

Use the `deepseek-reasoner` model (R1) for complex reasoning tasks:

```rust
use ai_sdk_core::{GenerateText, prompt::Prompt, output::OutputContent};
use ai_sdk_deepseek::DeepSeekClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = DeepSeekClient::new()
        .load_api_key_from_env()
        .build();

    let model = provider.chat_model("deepseek-reasoner");

    let result = GenerateText::new(
        model,
        Prompt::text("Solve this logic puzzle: ...")
    )
    .execute()
    .await?;

    // Access reasoning and answer separately
    for content in &result.output.content {
        match content {
            OutputContent::Reasoning { content, .. } => {
                println!("Reasoning: {}", content);
            }
            OutputContent::Text { content, .. } => {
                println!("Answer: {}", content);
            }
            _ => {}
        }
    }

    Ok(())
}
```

### Configuration Options

#### Using Client Builder (Recommended)

```rust
use ai_sdk_deepseek::DeepSeekClient;

let provider = DeepSeekClient::new()
    .api_key("your-api-key")
    .base_url("https://api.deepseek.com/v1")  // Optional, this is the default
    .header("X-Custom-Header", "value")        // Optional custom headers
    .build();
```

#### Using Settings

```rust
use ai_sdk_deepseek::{create_deepseek, DeepSeekProviderSettings};

let provider = create_deepseek(
    DeepSeekProviderSettings::new()
        .with_api_key("your-api-key")
        .with_base_url("https://api.deepseek.com/v1")
);
```

#### Loading from Environment

```rust
use ai_sdk_deepseek::DeepSeekClient;

// Reads from DEEPSEEK_API_KEY environment variable
let provider = DeepSeekClient::new()
    .load_api_key_from_env()
    .build();
```

## Available Models

| Model ID | Description |
|----------|-------------|
| `deepseek-chat` | Main chat model for general tasks |
| `deepseek-reasoner` | Advanced reasoning model (R1) |

## DeepSeek-Specific Features

### Prompt Cache Statistics

DeepSeek provides information about prompt cache hits and misses:

```rust
let result = GenerateText::new(model, prompt).execute().await?;

if let Some(metadata) = result.metadata {
    if let Some(deepseek) = metadata.get("deepseek") {
        println!("Cache hit tokens: {:?}", 
            deepseek.get("promptCacheHitTokens"));
        println!("Cache miss tokens: {:?}",
            deepseek.get("promptCacheMissTokens"));
    }
}
```

This helps you understand cache efficiency and optimize your prompts for better performance.

## Examples

Run the examples with:

```bash
# Basic chat
cargo run --example basic_chat

# Streaming
cargo run --example streaming_chat

# Reasoning model
cargo run --example reasoning
```

Make sure to set your `DEEPSEEK_API_KEY` environment variable first.

## Provider Trait Implementation

The DeepSeek provider implements the `Provider` trait:

```rust
use ai_sdk_provider::Provider;

let provider_trait: &dyn Provider = &provider;
let model = provider_trait.language_model("deepseek-chat")?;
```

**Supported Models:**
- ✅ `language_model()` - Chat models
- ❌ `text_embedding_model()` - Not supported
- ❌ `image_model()` - Not supported
- ❌ `transcription_model()` - Not supported
- ❌ `speech_model()` - Not supported
- ❌ `reranking_model()` - Not supported

## Error Handling

```rust
use ai_sdk_core::GenerateText;

match GenerateText::new(model, prompt).execute().await {
    Ok(result) => println!("{}", result.output.text()),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Documentation

For more information about the AI SDK and its capabilities, see:

- [AI SDK Documentation](../README.md)
- [Provider Implementation Guide](../PROVIDER_IMPLEMENTATION.md)
- [DeepSeek API Documentation](https://api-docs.deepseek.com/)

## License

MIT
