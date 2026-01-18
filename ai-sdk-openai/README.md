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
llm-kit-openai = "0.1"
llm-kit-core = "0.1"
llm-kit-provider = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Using the Client Builder (Recommended)

```rust
use llm_kit_openai::OpenAIClient;
use llm_kit_provider::language_model::LanguageModel;

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
use llm_kit_openai::{OpenAIProvider, OpenAIProviderSettings};
use llm_kit_provider::language_model::LanguageModel;

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
use llm_kit_openai::OpenAIClient;

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
use llm_kit_openai::{OpenAIProvider, OpenAIProviderSettings};

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

### GPT-4 Family
- `gpt-4` - Most capable GPT-4 model
- `gpt-4-turbo` - Faster GPT-4 variant
- `gpt-4o` - Optimized GPT-4 model
- `gpt-4o-mini` - Smaller, faster GPT-4o variant

### GPT-3.5 Family
- `gpt-3.5-turbo` - Fast and efficient model

### Reasoning Models
- `o1` - Latest reasoning model
- `o1-preview` - Preview version of o1
- `o1-mini` - Smaller o1 variant
- `o3-mini` - Next-generation reasoning model

### GPT-5 Family
- Future models will be supported as they become available

For a complete list of available models, see the [OpenAI Models documentation](https://platform.openai.com/docs/models).

## OpenAI-Specific Features

### Reasoning Models

OpenAI reasoning models (o1, o1-preview, o1-mini, o3-mini) have special handling:

- **Developer role**: System messages automatically use the "developer" role instead of "system"
- **Parameter filtering**: Unsupported settings (temperature, top_p, presence_penalty, frequency_penalty, etc.) are automatically removed
- **Token limits**: Uses `max_completion_tokens` instead of `max_tokens`

These adjustments happen automatically when you use a reasoning model, so you don't need to make any code changes.

### Provider-Specific Options

OpenAI supports additional options beyond the standard AI SDK parameters:

#### Reasoning Effort

Control the computational effort for reasoning models:

```rust
use llm_kit_openai::chat::{OpenAIChatLanguageModelOptions, openai_chat_options::ReasoningEffort};

let options = OpenAIChatLanguageModelOptions {
    reasoning_effort: Some(ReasoningEffort::High),
    ..Default::default()
};
```

Available values:
- `ReasoningEffort::Low` - Faster, less thorough reasoning
- `ReasoningEffort::Medium` - Balanced reasoning
- `ReasoningEffort::High` - More thorough, slower reasoning

#### Logprobs

Request log probabilities for generated tokens:

```rust
use llm_kit_openai::chat::{OpenAIChatLanguageModelOptions, openai_chat_options::LogprobsOption};

let options = OpenAIChatLanguageModelOptions {
    logprobs: Some(LogprobsOption::Number(5)),  // Top 5 token probabilities
    ..Default::default()
};
```

#### Service Tier

Select the service tier for processing:

```rust
use llm_kit_openai::chat::{OpenAIChatLanguageModelOptions, openai_chat_options::ServiceTier};

let options = OpenAIChatLanguageModelOptions {
    service_tier: Some(ServiceTier::Auto),
    ..Default::default()
};
```

Available values:
- `ServiceTier::Auto` - Automatic tier selection
- `ServiceTier::Default` - Standard processing tier

#### Organization and Project

Configure organization and project IDs:

```rust
let provider = OpenAIClient::new()
    .api_key("your-api-key")
    .organization("org-123")
    .project("proj-456")
    .build();
```

## Usage Examples

### Basic Text Generation

See `examples/chat.rs` for a complete example.

### Streaming Responses

See `examples/stream.rs` for a complete example.

### Tool Calling

OpenAI supports function calling for tool integration. See `examples/chat_tool_calling.rs` for a complete example.

## Examples

See the `examples/` directory for complete examples:

- `chat.rs` - Basic chat completion
- `stream.rs` - Streaming responses
- `chat_tool_calling.rs` - Tool calling with chat models
- `stream_tool_calling.rs` - Streaming with tool calling

Run examples with:

```bash
cargo run --example chat
cargo run --example stream
cargo run --example chat_tool_calling
cargo run --example stream_tool_calling
```

## Documentation

- [API Documentation](https://docs.rs/llm-kit-openai)
- [AI SDK Documentation](https://github.com/saribmah/ai-sdk)
- [OpenAI API Reference](https://platform.openai.com/docs/api-reference)

## License

Licensed under:

- MIT license ([LICENSE-MIT](LICENSE-MIT))

## Contributing

Contributions are welcome! Please see the [Contributing Guide](../CONTRIBUTING.md) for more details.
