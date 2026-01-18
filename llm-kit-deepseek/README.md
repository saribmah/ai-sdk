# LLM Kit DeepSeek

DeepSeek provider for [LLM Kit](https://github.com/saribmah/llm-kit) - Complete integration with DeepSeek's chat and reasoning models.

> **Note**: This provider uses the standardized builder pattern. See the [Quick Start](#quick-start) section for the recommended usage.

## Features

- **Text Generation**: Generate text using DeepSeek models (deepseek-chat, deepseek-reasoner)
- **Streaming**: Stream responses in real-time for immediate feedback
- **Tool Calling**: Support for function/tool calling with custom tools
- **Reasoning Models**: Advanced reasoning capabilities with deepseek-reasoner (R1)
- **Prompt Caching**: Automatic prompt caching with cache hit/miss token tracking
- **Cache Metadata**: Track prompt cache efficiency for optimization

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-kit-deepseek = "0.1"
llm-kit-core = "0.1"
llm-kit-provider = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Using the Client Builder (Recommended)

```rust
use llm_kit_deepseek::DeepSeekClient;
use llm_kit_core::{GenerateText, Prompt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider using the client builder
    let provider = DeepSeekClient::new()
        .api_key("your-api-key")  // Or use DEEPSEEK_API_KEY env var
        .build();
    
    // Create a language model
    let model = provider.chat_model("deepseek-chat");
    
    // Generate text
    let result = GenerateText::new(std::sync::Arc::new(model), Prompt::text("Hello, DeepSeek!"))
        .temperature(0.7)
        .max_output_tokens(100)
        .execute()
        .await?;
    
    println!("{}", result.text);
    Ok(())
}
```

### Using Settings Directly (Alternative)

```rust
use llm_kit_deepseek::{DeepSeekProvider, DeepSeekProviderSettings};
use llm_kit_core::{GenerateText, Prompt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider with settings
    let provider = DeepSeekProvider::new(DeepSeekProviderSettings::default());
    
    let model = provider.chat_model("deepseek-chat");
    
    let result = GenerateText::new(std::sync::Arc::new(model), Prompt::text("Hello, DeepSeek!"))
        .execute()
        .await?;
    
    println!("{}", result.text);
    Ok(())
}
```

## Configuration

### Environment Variables

Set your DeepSeek API key as an environment variable:

```bash
export DEEPSEEK_API_KEY=your-api-key
export DEEPSEEK_BASE_URL=https://api.deepseek.com/v1  # Optional
```

### Using the Client Builder

```rust
use llm_kit_deepseek::DeepSeekClient;

let provider = DeepSeekClient::new()
    .api_key("your-api-key")
    .base_url("https://api.deepseek.com/v1")
    .header("Custom-Header", "value")
    .build();
```

### Using Settings Directly

```rust
use llm_kit_deepseek::{DeepSeekProvider, DeepSeekProviderSettings};

let settings = DeepSeekProviderSettings::new()
    .with_api_key("your-api-key")
    .with_base_url("https://api.deepseek.com/v1")
    .add_header("Custom-Header", "value");

let provider = DeepSeekProvider::new(settings);
```

### Loading from Environment

```rust
use llm_kit_deepseek::DeepSeekClient;

// Reads from DEEPSEEK_API_KEY environment variable
let provider = DeepSeekClient::new()
    .load_api_key_from_env()
    .build();
```

### Builder Methods

The `DeepSeekClient` builder supports:

- `.api_key(key)` - Set the API key
- `.base_url(url)` - Set custom base URL
- `.header(key, value)` - Add a single custom header
- `.headers(map)` - Add multiple custom headers
- `.load_api_key_from_env()` - Load API key from DEEPSEEK_API_KEY environment variable
- `.build()` - Build the provider

## Reasoning Models

DeepSeek's reasoner model (R1) provides advanced reasoning capabilities for complex problem-solving:

```rust
use llm_kit_deepseek::DeepSeekClient;
use llm_kit_core::{GenerateText, Prompt};

let provider = DeepSeekClient::new()
    .load_api_key_from_env()
    .build();

let model = provider.chat_model("deepseek-reasoner");

let result = GenerateText::new(std::sync::Arc::new(model), 
    Prompt::text("Solve this complex logic puzzle: ..."))
    .execute()
    .await?;

// Access reasoning and answer separately
for output in result.experimental_output.iter() {
    if let llm_kit_provider::language_model::Output::Reasoning(reasoning) = output {
        println!("Reasoning: {}", reasoning.text);
    }
}

println!("Answer: {}", result.text);
```

## Streaming

Stream responses for real-time output:

```rust
use llm_kit_deepseek::DeepSeekClient;
use llm_kit_core::{StreamText, Prompt};
use futures_util::StreamExt;

let provider = DeepSeekClient::new()
    .load_api_key_from_env()
    .build();

let model = provider.chat_model("deepseek-chat");

let result = StreamText::new(std::sync::Arc::new(model), 
    Prompt::text("Write a story"))
    .temperature(0.8)
    .execute()
    .await?;

let mut text_stream = result.text_stream();
while let Some(text_delta) = text_stream.next().await {
    print!("{}", text_delta);
}
```

## Supported Models

All DeepSeek models are supported, including:

- **deepseek-chat** - Main chat model for general tasks and conversations
- **deepseek-reasoner** - Advanced reasoning model (R1) for complex problem-solving and logic tasks

For a complete list of available models, see the [DeepSeek documentation](https://api-docs.deepseek.com/quick_start/pricing).

## Provider-Specific Features

### Prompt Cache Statistics

DeepSeek provides detailed information about prompt cache hits and misses to help optimize performance:

```rust
use llm_kit_deepseek::DeepSeekClient;
use llm_kit_core::{GenerateText, Prompt};

let provider = DeepSeekClient::new().build();
let model = provider.chat_model("deepseek-chat");

let result = GenerateText::new(std::sync::Arc::new(model), Prompt::text("Hello!"))
    .execute()
    .await?;

// Access cache metadata
if let Some(metadata) = result.provider_metadata {
    if let Some(deepseek) = metadata.get("deepseek") {
        println!("Cache hit tokens: {:?}", 
            deepseek.get("promptCacheHitTokens"));
        println!("Cache miss tokens: {:?}",
            deepseek.get("promptCacheMissTokens"));
    }
}
```

This helps you understand cache efficiency and optimize your prompts for better performance and cost savings.

## Examples

See the `examples/` directory for complete examples:

- `chat.rs` - Basic chat completion with DeepSeek
- `stream.rs` - Streaming responses
- `chat_tool_calling.rs` - Tool calling with custom tools
- `stream_tool_calling.rs` - Streaming with tool calls
- `reasoning.rs` - Using the deepseek-reasoner model for complex reasoning

Run examples with:

```bash
cargo run --example chat
cargo run --example stream
cargo run --example reasoning
cargo run --example chat_tool_calling
```

Make sure to set your `DEEPSEEK_API_KEY` environment variable first.

## Documentation

- [API Documentation](https://docs.rs/llm-kit-deepseek)
- [LLM Kit Documentation](https://github.com/saribmah/llm-kit)
- [DeepSeek API Reference](https://api-docs.deepseek.com/)

## License

Licensed under:

- MIT license ([LICENSE-MIT](LICENSE-MIT))

## Contributing

Contributions are welcome! Please see the [Contributing Guide](../CONTRIBUTING.md) for more details.
