# AI SDK xAI

xAI (Grok) provider for [AI SDK Rust](https://github.com/saribmah/llm-kit) - Complete integration with xAI's Grok models featuring reasoning capabilities, integrated search, and image generation.

> **Note**: This provider uses the standardized builder pattern. See the [Quick Start](#quick-start) section for the recommended usage.

## Features

- **Text Generation**: Full support for Grok models with streaming and tool calling
- **Streaming**: Real-time response streaming with Server-Sent Events
- **Tool Calling**: Complete function calling support with tool execution
- **Image Generation**: Create images with grok-2-image model
- **Reasoning Mode**: Access model reasoning with configurable effort levels
- **Integrated Search**: Web, X (Twitter), news, and RSS search capabilities
- **Citations**: Automatic citation extraction from search results
- **Response Format**: JSON mode and structured outputs with JSON schema

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-kit-xai = "0.1"
llm-kit-core = "0.1"
llm-kit-provider = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Using the Client Builder (Recommended)

```rust
use llm_kit_xai::XaiClient;
use llm_kit_provider::{Provider, LanguageModel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider using the client builder
    let provider = XaiClient::new()
        .api_key("your-api-key")  // Or use XAI_API_KEY env var
        .build();
    
    // Create a language model
    let model = provider.chat_model("grok-4");
    
    println!("Model: {}", model.model_id());
    println!("Provider: {}", model.provider());
    Ok(())
}
```

### Using Settings Directly (Alternative)

```rust
use llm_kit_xai::{XaiProvider, XaiProviderSettings};
use llm_kit_provider::{Provider, LanguageModel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider with settings
    let provider = XaiProvider::new(XaiProviderSettings::default());
    
    let model = provider.chat_model("grok-4");
    
    println!("Model: {}", model.model_id());
    Ok(())
}
```

## Configuration

### Environment Variables

Set your xAI API key as an environment variable:

```bash
export XAI_API_KEY=your-api-key
```

### Using the Client Builder

```rust
use llm_kit_xai::XaiClient;

let provider = XaiClient::new()
    .api_key("your-api-key")
    .base_url("https://api.x.ai/v1")
    .header("X-Custom-Header", "value")
    .name("my-xai-provider")
    .build();
```

### Builder Methods

The `XaiClient` builder supports:

- `.api_key(key)` - Set the API key
- `.base_url(url)` - Set custom base URL
- `.name(name)` - Set provider name
- `.header(key, value)` - Add a single custom header
- `.headers(map)` - Add multiple custom headers
- `.build()` - Build the provider

## Supported Models

### Chat Models

- **`grok-4`** - Latest Grok-4 model with advanced capabilities
- **`grok-4-fast-reasoning`** - Fast model with reasoning capabilities
- **`grok-4-fast-non-reasoning`** - Fast model without reasoning
- **`grok-code-fast-1`** - Optimized for code generation
- **`grok-3`** - Grok-3 base model
- **`grok-3-fast`** - Faster Grok-3 variant
- **`grok-3-mini`** - Smaller, efficient model
- **`grok-2-vision-1212`** - Vision-capable model
- **`grok-2-1212`** - Grok-2 model with December 2024 updates
- **`grok-beta`** - Beta model with latest features

```rust
// Create a chat model
let model = provider.chat_model("grok-4");
```

### Image Models

- **`grok-2-image`** - Image generation model

```rust
// Create an image model
let model = provider.image_model("grok-2-image");
```

## Provider-Specific Options

xAI supports advanced features through provider options that can be passed using the `llm-kit-core` API.

### Reasoning Mode

Control the model's reasoning effort level:

```rust
use llm_kit_core::GenerateText;
use serde_json::json;

let result = GenerateText::new(model, prompt)
    .provider_options(json!({
        "reasoningEffort": "high"  // "low", "medium", or "high"
    }))
    .execute()
    .await?;
```

Access reasoning content in the response:

```rust
// Reasoning content is automatically extracted to result.content
for content in result.content {
    if let llm_kit_core::output::Output::Reasoning(reasoning) = content {
        println!("Model reasoning: {}", reasoning.text);
    }
}
```

### Integrated Search

Enable web, X (Twitter), news, or RSS search:

```rust
use llm_kit_core::GenerateText;
use serde_json::json;

let result = GenerateText::new(model, prompt)
    .provider_options(json!({
        "searchParameters": {
            "recencyFilter": "day",  // "hour", "day", "week", "month", "year"
            "sources": [
                {"type": "web"},
                {"type": "x"},       // X (Twitter) search
                {"type": "news"},
                {"type": "rss", "url": "https://example.com/feed.xml"}
            ]
        }
    }))
    .execute()
    .await?;
```

### Citations

Citations are automatically extracted from search results:

```rust
let result = GenerateText::new(model, prompt).execute().await?;

// Citations available in result.content
for content in result.content {
    if let llm_kit_core::output::Output::Source(source) = content {
        println!("Source: {} - {}", source.title, source.url);
    }
}
```

### Response Format (JSON Mode)

Force structured JSON outputs:

```rust
use llm_kit_core::GenerateText;
use llm_kit_provider::language_model::call_options::LanguageModelResponseFormat;
use serde_json::json;

// Simple JSON mode
let result = GenerateText::new(model, prompt)
    .with_response_format(LanguageModelResponseFormat::Json {
        schema: None,
        name: None,
        description: None,
    })
    .execute()
    .await?;

// Structured outputs with JSON schema
let schema = json!({
    "type": "object",
    "properties": {
        "name": {"type": "string"},
        "age": {"type": "integer"}
    },
    "required": ["name", "age"]
});

let result = GenerateText::new(model, prompt)
    .with_response_format(LanguageModelResponseFormat::Json {
        schema: Some(schema),
        name: Some("UserProfile".to_string()),
        description: Some("A user profile".to_string()),
    })
    .execute()
    .await?;
```

### Parallel Function Calling

Control parallel tool execution:

```rust
use llm_kit_core::GenerateText;
use serde_json::json;

let result = GenerateText::new(model, prompt)
    .tools(tools)
    .provider_options(json!({
        "parallelFunctionCalling": true
    }))
    .execute()
    .await?;
```

### Available Provider Options

| Option | Type | Description |
|--------|------|-------------|
| `reasoningEffort` | `string` | Reasoning effort level: "low", "medium", "high" |
| `searchParameters.recencyFilter` | `string` | Time filter: "hour", "day", "week", "month", "year" |
| `searchParameters.sources` | `array` | Search sources: web, x, news, rss |
| `parallelFunctionCalling` | `bool` | Enable parallel tool execution |

## Examples

See the `examples/` directory for complete examples:

- `chat.rs` - Basic chat completion using `do_generate()` directly
- `stream.rs` - Streaming responses using `do_stream()` directly
- `chat_tool_calling.rs` - Tool calling using `do_generate()` directly
- `stream_tool_calling.rs` - Streaming with tools using `do_stream()` directly
- `image_generation.rs` - Image generation using `do_generate()` directly

Run examples with:

```bash
export XAI_API_KEY="your-api-key"
cargo run --example chat
cargo run --example stream
cargo run --example image_generation
```

## Documentation

- [API Documentation](https://docs.rs/llm-kit-xai)
- [AI SDK Documentation](https://github.com/saribmah/llm-kit)
- [xAI API Reference](https://docs.x.ai/)
- [xAI Console](https://console.x.ai/)

## License

MIT

## Contributing

Contributions are welcome! Please see the [Contributing Guide](../CONTRIBUTING.md) for more details.
