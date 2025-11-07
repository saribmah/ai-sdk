# AI SDK Rust

A unified Rust SDK for building AI-powered applications with multiple model providers.

> **Note**: This project is heavily inspired by [Vercel's AI SDK](https://github.com/vercel/ai) for TypeScript, bringing similar ergonomics and patterns to the Rust ecosystem.

## Features

- **Provider-agnostic API**: Write once, switch providers easily
- **Type-safe**: Leverages Rust's type system for compile-time safety
- **Type-safe Tools**: Define tools with compile-time checked inputs/outputs using the `TypeSafeTool` trait
- **Async/await**: Built on Tokio for efficient async operations
- **Streaming support**: Stream responses from language models with real-time processing
- **Tool calling**: Support for function/tool calling with LLMs, both dynamic and type-safe
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

The SDK provides a fluent builder API for ergonomic text generation:

```rust
use ai_sdk_core::{GenerateTextBuilder};
use ai_sdk_core::prompt::Prompt;
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

    // Generate text using the builder pattern
    let result = GenerateTextBuilder::new(&*model, Prompt::text("What is the capital of France?"))
        .temperature(0.7)
        .max_output_tokens(100)
        .execute()
        .await?;

    println!("Response: {:?}", result);
    Ok(())
}
```

Alternatively, you can use the function-based API:

```rust
use ai_sdk_core::generate_text;
use ai_sdk_core::prompt::{Prompt, call_settings::CallSettings};

let prompt = Prompt::text("What is the capital of France?");
let settings = CallSettings::default()
    .with_temperature(0.7)
    .with_max_output_tokens(100);

let result = generate_text(&*model, prompt, settings, None, None, None, None, None, None, None).await?;
```

### Type-Safe Tools

The SDK provides a `TypeSafeTool` trait for defining tools with compile-time type checking:

```rust
use ai_sdk_core::tool::TypeSafeTool;
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Define typed input/output structures
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct WeatherInput {
    city: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct WeatherOutput {
    temperature: f64,
    conditions: String,
}

// Implement the type-safe tool
struct WeatherTool;

#[async_trait]
impl TypeSafeTool for WeatherTool {
    type Input = WeatherInput;
    type Output = WeatherOutput;

    fn name(&self) -> &str { "get_weather" }
    fn description(&self) -> &str { "Get weather for a city" }

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, String> {
        Ok(WeatherOutput {
            temperature: 72.0,
            conditions: format!("Sunny in {}", input.city),
        })
    }
}

// Convert to untyped Tool for use with LLMs
let weather_tool = WeatherTool.into_tool();
```

**Benefits:**
- ✅ Compile-time type checking - catch errors before runtime
- ✅ Automatic JSON schema generation from Rust types
- ✅ IDE support - autocomplete, go-to-definition, refactoring
- ✅ Can be used with LLMs or called directly in your code
- ✅ Impossible to pass wrong types or forget required fields

See the `type_safe_tools` example for a complete demonstration.

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

### Builder Pattern API

Both `GenerateTextBuilder` and `StreamTextBuilder` provide fluent, chainable APIs for configuring text generation and streaming:

#### Text Generation

```rust
use ai_sdk_core::GenerateTextBuilder;
use ai_sdk_core::prompt::Prompt;

let result = GenerateTextBuilder::new(&*model, Prompt::text("Tell me a joke"))
    .temperature(0.7)            // Creativity control
    .max_output_tokens(100)      // Response length limit
    .top_p(0.9)                  // Nucleus sampling
    .presence_penalty(0.6)       // Discourage repetition
    .frequency_penalty(0.5)      // Vary word choice
    .seed(42)                    // Deterministic generation
    .max_retries(3)              // Retry on failures
    .execute()
    .await?;
```

#### Available Builder Methods

- **Sampling Parameters:**
  - `.temperature(f64)` - Controls randomness (0.0 to 2.0)
  - `.top_p(f64)` - Nucleus sampling threshold
  - `.top_k(u32)` - Top-K sampling parameter
  - `.presence_penalty(f64)` - Penalizes token presence
  - `.frequency_penalty(f64)` - Penalizes token frequency

- **Output Control:**
  - `.max_output_tokens(u32)` - Maximum tokens to generate
  - `.stop_sequences(Vec<String>)` - Stop generation at sequences
  - `.seed(u32)` - Seed for deterministic output

- **Tools and Advanced:**
  - `.tools(ToolSet)` - Add function calling tools
  - `.tool_choice(LanguageModelToolChoice)` - Control tool selection
  - `.stop_when(Vec<Box<dyn StopCondition>>)` - Multi-step stop conditions
  - `.prepare_step(Box<dyn PrepareStep>)` - Customize each generation step
  - `.on_step_finish(Box<dyn OnStepFinish>)` - Callback after each step
  - `.on_finish(Box<dyn OnFinish>)` - Callback when complete

- **Configuration:**
  - `.max_retries(u32)` - Maximum retry attempts
  - `.headers(HashMap<String, String>)` - Custom HTTP headers
  - `.abort_signal(CancellationToken)` - Cancellation support
  - `.provider_options(SharedProviderOptions)` - Provider-specific options
  - `.settings(CallSettings)` - Set all settings at once

#### Text Streaming

The `StreamTextBuilder` provides similar functionality for streaming responses:

```rust
use ai_sdk_core::StreamTextBuilder;
use ai_sdk_core::prompt::Prompt;
use futures_util::StreamExt;
use std::sync::Arc;

let result = StreamTextBuilder::new(Arc::from(model), Prompt::text("Tell me a story"))
    .temperature(0.8)
    .max_output_tokens(500)
    .include_raw_chunks(true)
    .on_chunk(Box::new(|event| {
        Box::pin(async move {
            // Process each chunk as it arrives
        })
    }))
    .on_finish(Box::new(|event| {
        Box::pin(async move {
            println!("Total tokens: {}", event.total_usage.total_tokens);
        })
    }))
    .execute()
    .await?;

// Stream text deltas in real-time
let mut text_stream = result.text_stream();
while let Some(delta) = text_stream.next().await {
    print!("{}", delta);
}
```

**Additional StreamTextBuilder Methods:**
- `.include_raw_chunks(bool)` - Include raw provider chunks
- `.transforms(Vec<Box<dyn StreamTransform>>)` - Apply stream transformations
- `.on_chunk(OnChunkCallback)` - Callback for each chunk
- `.on_error(OnErrorCallback)` - Error handling callback

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

# Type-safe tools example - compile-time type checking for tools
export OPENAI_API_KEY="your-api-key"
cargo run --example type_safe_tools

# Multi-step tools example - iterative tool calling
export OPENAI_API_KEY="your-api-key"
cargo run --example multi_step_tools

# Streaming examples
export OPENAI_API_KEY="your-api-key"
cargo run --example basic_stream
cargo run --example stream_tool_calling
cargo run --example stream_transforms
cargo run --example partial_output
```

The examples demonstrate:
- Creating providers with environment variables
- Using `generate_text` with real API calls
- Handling responses and metadata
- System messages and temperature settings
- Token usage tracking
- Tool/function calling and handling tool call responses
- **Type-safe tools** with compile-time type checking (see `type_safe_tools.rs`)
- Multi-step tool execution with iterative calls
- Streaming responses in real-time
- Partial output parsing for structured data

## Development

### Prerequisites

This project uses [just](https://github.com/casey/just) as a command runner. Install it with:

```bash
cargo install just
```

### Pre-Commit Hooks

We use pre-commit hooks to ensure code quality. Install them with:

```bash
just install-hooks
```

This will automatically:
- ✅ Format your code with `rustfmt` (auto-fixes)
- ✅ Run `clippy` to catch common mistakes (blocks commit if issues found)
- ✅ Verify code compiles with `cargo check`

### Available Just Commands

We use [just](https://github.com/casey/just) as a command runner. Install it with `cargo install just`.

```bash
just                # List all available commands
just install-hooks  # Install git pre-commit hooks
just fmt            # Format code (auto-fix)
just clippy         # Run clippy linter
just check          # Quick compile check
just test           # Run all tests
just build          # Build all crates
just doc            # Build documentation
just pre-commit     # Run all pre-commit checks
just ci             # Run all CI checks locally
```

Run `just --list` to see all available commands.

See [DEVELOPMENT.md](DEVELOPMENT.md) for detailed development guidelines.

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
