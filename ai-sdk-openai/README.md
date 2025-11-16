# AI SDK OpenAI

OpenAI provider for [AI SDK Rust](https://github.com/saribmah/ai-sdk) - Complete integration with OpenAI's chat completion API.

> **Note**: This provider uses the standardized builder pattern. See the [Quick Start](#quick-start) section for the recommended usage.

## Features

- **Text Generation**: Generate text using GPT models with support for all OpenAI chat models
- **Streaming**: Stream responses in real-time
- **Tool Calling**: Support for function calling
- **Multi-modal**: Support for text, images, audio, and PDFs
- **Reasoning Models**: Special handling for o1, o3, and other reasoning models
- **Provider Options**: Logprobs, reasoning effort, service tiers, and more
- **Type-safe Configuration**: Builder pattern for easy setup

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ai-sdk-openai = "0.1"
ai-sdk-core = "0.1"
ai-sdk-provider = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Using the Client Builder (Recommended)

```rust
use ai_sdk_openai::OpenAIClient;
use ai_sdk_provider::language_model::LanguageModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider using the client builder
    let provider = OpenAIClient::new()
        .api_key("your-api-key")  // Or use OPENAI_API_KEY env var
        .build();
    
    // Create a language model
    let model = provider.chat("gpt-4o");
    
    println!("Model: {}", model.model_id());
    println!("Provider: {}", model.provider());
    Ok(())
}
```

### Using Settings Directly (Alternative)

```rust
use ai_sdk_openai::{OpenAIProvider, OpenAIProviderSettings};
use ai_sdk_provider::language_model::LanguageModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider with settings
    let provider = OpenAIProvider::new(OpenAIProviderSettings::default());
    
    let model = provider.chat("gpt-4o");
    
    println!("Model: {}", model.model_id());
    Ok(())
}
```

## Configuration

### Environment Variables

Set your OpenAI API key as an environment variable:

```bash
export OPENAI_API_KEY=your-api-key
export OPENAI_BASE_URL=https://api.openai.com/v1  # Optional
```

### Using the Client Builder

```rust
use ai_sdk_openai::OpenAIClient;

let provider = OpenAIClient::new()
    .api_key("your-api-key")
    .base_url("https://api.openai.com/v1")
    .organization("org-123")
    .project("proj-456")
    .header("Custom-Header", "value")
    .name("my-openai-provider")
    .build();
```

### Using Settings Directly

```rust
use ai_sdk_openai::{OpenAIProvider, OpenAIProviderSettings};

let settings = OpenAIProviderSettings::new()
    .with_api_key("your-api-key")
    .with_base_url("https://api.openai.com/v1")
    .with_organization("org-123")
    .with_project("proj-456")
    .add_header("Custom-Header", "value")
    .with_name("my-openai-provider");

let provider = OpenAIProvider::new(settings);
```

### Builder Methods

The `OpenAIClient` builder supports:

- `.api_key(key)` - Set the API key
- `.base_url(url)` - Set custom base URL
- `.organization(org)` - Set OpenAI organization ID
- `.project(project)` - Set OpenAI project ID
- `.name(name)` - Set provider name
- `.header(key, value)` - Add a single custom header
- `.headers(map)` - Add multiple custom headers
- `.build()` - Build the provider

## Supported Models

All OpenAI chat models are supported, including:

- **GPT-4 family**: `gpt-4`, `gpt-4-turbo`, `gpt-4o`, `gpt-4o-mini`
- **GPT-3.5**: `gpt-3.5-turbo`
- **Reasoning models**: `o1`, `o1-preview`, `o1-mini`, `o3-mini`
- **GPT-5 family** (when available)

## Reasoning Models

Reasoning models (o1, o3, etc.) have special handling:

- System messages use "developer" role instead of "system"
- Unsupported settings (temperature, top_p, etc.) are automatically removed
- Uses `max_completion_tokens` instead of `max_tokens`

## Provider-Specific Options

OpenAI-specific options can be passed through `provider_options`:

```rust
use ai_sdk_openai::chat::{OpenAIChatLanguageModelOptions, openai_chat_options::*};

let options = OpenAIChatLanguageModelOptions {
    reasoning_effort: Some(ReasoningEffort::High),
    logprobs: Some(LogprobsOption::Number(5)),
    service_tier: Some(ServiceTier::Auto),
    ..Default::default()
};
```

## Examples

See the `examples/` directory for complete examples:

- `basic_chat.rs` - Basic chat completion
- `streaming_chat.rs` - Streaming responses

Run examples with:

```bash
cargo run --example basic_chat
cargo run --example streaming_chat
```

## Documentation

- [API Documentation](https://docs.rs/ai-sdk-openai)
- [AI SDK Documentation](https://github.com/saribmah/ai-sdk)
- [OpenAI API Reference](https://platform.openai.com/docs/api-reference)

## License

Licensed under:

- MIT license ([LICENSE-MIT](LICENSE-MIT))

## Contributing

Contributions are welcome! Please see the [Contributing Guide](../CONTRIBUTING.md) for more details.
