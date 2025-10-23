# AI SDK Rust

A unified Rust SDK for building AI-powered applications with multiple model providers.

> **Note**: This project is heavily inspired by [Vercel's AI SDK](https://github.com/vercel/ai) for TypeScript, bringing similar ergonomics and patterns to the Rust ecosystem.

## Features

- **Provider-agnostic API**: Write once, switch providers easily
- **Type-safe**: Leverages Rust's type system for compile-time safety
- **Async/await**: Built on Tokio for efficient async operations
- **Streaming support**: Stream responses from language models (in progress)
- **Tool calling**: Support for function/tool calling with LLMs
- **Multiple providers**: OpenAI-compatible APIs (OpenAI, Azure OpenAI, and others)

## Project Structure

This is a Cargo workspace with multiple crates:

- **`ai-sdk-core`**: Core functionality including `generate_text`, prompt handling, and message types
- **`ai-sdk-provider`**: Provider interface and traits for implementing new providers
- **`ai-sdk-openai-compatible`**: OpenAI-compatible provider implementation (supports OpenAI, Azure OpenAI, and compatible APIs)

## Installation

```toml
[dependencies]
ai-sdk-core = { path = "ai-sdk-core" }
ai-sdk-openai-compatible = { path = "ai-sdk-openai-compatible" }
tokio = { version = "1.41", features = ["full"] }
```

## Quick Start

### Basic Text Generation

```rust
use ai_sdk_core::generate_text;
use ai_sdk_core::prompt::{Prompt, call_settings::CallSettings};
use ai_sdk_openai_compatible::{create_openai_compatible, OpenAICompatibleProviderSettings};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an OpenAI provider
    let provider = create_openai_compatible(
        OpenAICompatibleProviderSettings::new(
            "https://api.openai.com/v1",
            "openai"
        )
        .with_api_key(std::env::var("OPENAI_API_KEY")?)
    );

    // Get a language model
    let model = provider.chat_model("gpt-4");

    // Create a prompt
    let prompt = Prompt::text("What is the capital of France?");
    let settings = CallSettings::default();

    // Generate text
    let result = generate_text(&*model, prompt, settings, None, None, None).await?;

    println!("Response: {:?}", result);
    Ok(())
}
```

### Using Different Providers

The SDK supports any OpenAI-compatible API:

```rust
// OpenAI
let openai = create_openai_compatible(
    OpenAICompatibleProviderSettings::new(
        "https://api.openai.com/v1",
        "openai"
    )
    .with_api_key(api_key)
);

// Azure OpenAI
let azure = create_openai_compatible(
    OpenAICompatibleProviderSettings::new(
        "https://my-resource.openai.azure.com/openai",
        "azure-openai"
    )
    .with_api_key(api_key)
);

// Custom provider
let custom = create_openai_compatible(
    OpenAICompatibleProviderSettings::new(
        "https://api.custom-provider.com/v1",
        "custom"
    )
    .with_api_key(api_key)
    .with_header("X-Custom-Header", "value")
    .with_query_param("version", "2024-01")
);
```

### Vercel-Style Chaining

Inspired by Vercel's AI SDK, you can chain method calls:

```rust
let model = create_openai_compatible(
    OpenAICompatibleProviderSettings::new(
        "https://api.openai.com/v1",
        "openai"
    )
    .with_api_key("your-api-key")
)
.chat_model("gpt-4");
```

### Provider Trait API

For flexibility, you can also use the Provider trait:

```rust
use ai_sdk_provider::Provider;

let provider = create_openai_compatible(settings);
let provider_trait: &dyn Provider = &provider;
let model = provider_trait.language_model("gpt-4")?;
```

## Architecture

The SDK follows a layered architecture inspired by Vercel's AI SDK:

### Core Layer (`ai-sdk-core`)
- User-facing APIs like `generate_text`
- Prompt standardization and validation
- Message type conversions
- Error handling

### Provider Layer (`ai-sdk-provider`)
- `Provider` trait for implementing new providers
- `LanguageModel` trait for model implementations
- Standardized types: `CallOptions`, `Content`, `FinishReason`, `Usage`
- Tool calling interfaces

### Implementation Layer (`ai-sdk-openai-compatible`)
- Concrete provider implementations
- API-specific request/response handling
- HTTP client management
- Format conversions (provider format ↔ OpenAI format)

## Current Status

✅ **Implemented:**
- Core `generate_text` function
- Prompt handling and standardization
- Message type system
- Provider trait system
- OpenAI-compatible provider with `do_generate`
- Tool calling support (types and conversion)
- Response format options (text/JSON)
- Usage tracking with extended token details
- Custom headers and query parameters

🚧 **In Progress:**
- Streaming support (`do_stream` implementation)
- Additional providers
- Embedding support
- Image generation

## Examples

The project includes real-world examples that you can run with your own API key:

### Setup

1. Copy the example environment file:
```bash
cp .env.example .env
```

2. Add your API key to `.env`:
```
OPENAI_API_KEY=sk-your-actual-api-key-here
```

### Running Examples

```bash
# Basic chat example - simple text generation
export OPENAI_API_KEY="your-api-key"
cargo run --example basic_chat

# Conversation example - system messages and temperature settings
export OPENAI_API_KEY="your-api-key"
cargo run --example conversation

# Tool calling example - function calling with a weather tool
export OPENAI_API_KEY="your-api-key"
cargo run --example tool_calling
```

The examples demonstrate:
- Creating providers with environment variables
- Using `generate_text` with real API calls
- Handling responses and metadata
- System messages and temperature settings
- Token usage tracking
- Tool/function calling and handling tool call responses

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p ai-sdk-core
cargo test -p ai-sdk-provider
cargo test -p ai-sdk-openai-compatible

# Run with output
cargo test -- --nocapture
```

### Building

```bash
# Build all crates
cargo build

# Build in release mode
cargo build --release

# Check without building
cargo check

# Check examples
cargo check --examples
```

## Contributing

Contributions are welcome! This project aims to bring the ergonomic patterns of Vercel's AI SDK to Rust while leveraging Rust's unique strengths.

## License

See LICENSE file for details.

## Acknowledgments

This project is heavily inspired by [Vercel's AI SDK](https://github.com/vercel/ai) and aims to provide similar developer experience in the Rust ecosystem.
