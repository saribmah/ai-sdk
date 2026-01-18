# AI SDK Baseten

Baseten provider for [LLM Kit](https://github.com/saribmah/llm-kit) - Complete integration with Baseten's Model APIs and custom model deployments.

> **Note**: This provider uses the standardized builder pattern. See the [Quick Start](#quick-start) section for the recommended usage.

## Features

- **Text Generation**: Generate text using Baseten-hosted models or custom deployments
- **Streaming**: Stream responses in real-time with support for tool calls
- **Tool Calling**: Support for function calling with both hosted and custom models
- **Text Embedding**: Generate embeddings using custom model deployments
- **Model APIs**: Access to hosted models like DeepSeek V3, Kimi K2, and Qwen 3
- **Custom Deployments**: Support for dedicated model deployments with custom URLs

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-kit-baseten = "0.1"
llm-kit-core = "0.1"
llm-kit-provider = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Using the Client Builder (Recommended)

```rust
use llm_kit_baseten::BasetenClient;
use llm_kit_provider::language_model::LanguageModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider using the client builder
    let provider = BasetenClient::new()
        .api_key("your-api-key")  // Or use BASETEN_API_KEY env var
        .build();
    
    // Create a language model
    let model = provider.chat_model(Some("deepseek-ai/DeepSeek-V3-0324"));
    
    println!("Model: {}", model.model_id());
    println!("Provider: {}", model.provider());
    Ok(())
}
```

### Using Settings Directly (Alternative)

```rust
use llm_kit_baseten::{BasetenProvider, BasetenProviderSettings};
use llm_kit_provider::language_model::LanguageModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider with settings
    let provider = BasetenProvider::new(BasetenProviderSettings::default());
    
    let model = provider.chat_model(Some("deepseek-ai/DeepSeek-V3-0324"));
    
    println!("Model: {}", model.model_id());
    Ok(())
}
```

## Configuration

### Environment Variables

Set your Baseten API key as an environment variable:

```bash
export BASETEN_API_KEY=your-api-key
```

### Using the Client Builder

```rust
use llm_kit_baseten::BasetenClient;

let provider = BasetenClient::new()
    .api_key("your-api-key")
    .base_url("https://inference.baseten.co/v1")
    .header("Custom-Header", "value")
    .build();
```

### Builder Methods

The `BasetenClient` builder supports:

- `.api_key(key)` - Set the API key (overrides `BASETEN_API_KEY` environment variable)
- `.base_url(url)` - Set the base URL for Model APIs (default: `https://inference.baseten.co/v1`)
- `.model_url(url)` - Set a custom model URL for dedicated deployments
- `.name(name)` - Set provider name (optional)
- `.header(key, value)` - Add a single custom header
- `.headers(map)` - Add multiple custom headers
- `.build()` - Build the provider

## Provider-Specific Options

### Model APIs vs Custom Deployments

Baseten supports two deployment modes:

#### Model APIs (Hosted Models)

Use the default base URL to access Baseten's hosted models:

```rust
use llm_kit_baseten::BasetenClient;

let provider = BasetenClient::new()
    .api_key("your-api-key")
    .build();

// Specify model ID from hosted models
let model = provider.chat_model(Some("deepseek-ai/DeepSeek-V3-0324"));
```

#### Custom Model Deployments

For dedicated deployments, specify a custom model URL:

```rust
use llm_kit_baseten::BasetenClient;

let provider = BasetenClient::new()
    .api_key("your-api-key")
    .model_url("https://model-{id}.api.baseten.co/environments/production/sync/v1")
    .build();

// Model ID is optional when using custom URL
let model = provider.chat_model(None);
```

**Important:** 
- Chat models require `/sync/v1` endpoints
- Embedding models require `/sync` or `/sync/v1` endpoints

### Text Embeddings

Embeddings require a custom model URL and are not available via Model APIs:

```rust
use llm_kit_baseten::BasetenClient;

let provider = BasetenClient::new()
    .api_key("your-api-key")
    .model_url("https://model-{id}.api.baseten.co/environments/production/sync")
    .build();

let model = provider.text_embedding_model(None);
```

## Supported Models

### Model APIs (Hosted Models)

All Baseten Model API models are supported, including:

- **DeepSeek**: `deepseek-ai/DeepSeek-R1-0528`, `deepseek-ai/DeepSeek-V3-0324`, `deepseek-ai/DeepSeek-V3.1`
- **Kimi**: `moonshotai/Kimi-K2-Instruct-0905`
- **Qwen**: `Qwen/Qwen3-235B-A22B-Instruct-2507`, `Qwen/Qwen3-Coder-480B-A35B-Instruct`
- **OpenAI**: `openai/gpt-oss-120b`
- **GLM**: `zai-org/GLM-4.6`

### Custom Models

Any model deployed on Baseten can be used with a custom model URL. Supported model types:

- **Chat Models** - Text generation with optional tool calling
- **Embedding Models** - Text embeddings for similarity search

For a complete list of available hosted models, see the [Baseten Model APIs documentation](https://docs.baseten.co/development/model-apis/overview).

## Examples

See the `examples/` directory for complete examples:

- `chat.rs` - Basic chat completion with Model APIs
- `stream.rs` - Streaming responses
- `chat_tool_calling.rs` - Tool calling with function definitions
- `stream_tool_calling.rs` - Streaming with tool calls
- `text_embedding.rs` - Text embeddings with custom deployments

Run examples with:

```bash
export BASETEN_API_KEY=your-api-key
cargo run --example chat
cargo run --example stream
cargo run --example chat_tool_calling
cargo run --example stream_tool_calling
cargo run --example text_embedding
```

## Documentation

- [API Documentation](https://docs.rs/llm-kit-baseten)
- [AI SDK Documentation](https://github.com/saribmah/llm-kit)
- [Baseten API Reference](https://docs.baseten.co/)
- [Baseten Model APIs](https://docs.baseten.co/development/model-apis/overview)

## License

Apache-2.0

## Contributing

Contributions are welcome! Please see the [Contributing Guide](../CONTRIBUTING.md) for more details.
