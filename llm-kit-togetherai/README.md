# AI SDK Together AI

Together AI provider for [LLM Kit](https://github.com/saribmah/llm-kit) - Complete integration with Together AI's extensive collection of open-source models.

> **Note**: This provider uses the standardized builder pattern. See the [Quick Start](#quick-start) section for the recommended usage.

## Features

- **Text Generation**: Generate text using Llama, Mistral, Qwen, DeepSeek, and more
- **Streaming**: Stream responses in real-time with support for tool calls
- **Tool Calling**: Support for function calling with chat models
- **Text Embedding**: Generate embeddings for semantic search and similarity
- **Image Generation**: Create images with FLUX and Stable Diffusion models
- **Reranking**: Improve search results with document reranking models
- **Multiple Model Families**: Access to Llama, Mistral, Qwen, DeepSeek, Gemma, and more

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-kit-togetherai = "0.1"
llm-kit-core = "0.1"
llm-kit-provider = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Using the Client Builder (Recommended)

```rust
use llm_kit_togetherai::TogetherAIClient;
use llm_kit_provider::language_model::LanguageModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider using the client builder
    let provider = TogetherAIClient::new()
        .api_key("your-api-key")  // Or use TOGETHER_AI_API_KEY env var
        .build();
    
    // Create a language model
    let model = provider.chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");
    
    println!("Model: {}", model.model_id());
    println!("Provider: {}", model.provider());
    Ok(())
}
```

### Using Settings Directly (Alternative)

```rust
use llm_kit_togetherai::{TogetherAIProvider, TogetherAIProviderSettings};
use llm_kit_provider::language_model::LanguageModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider with settings
    let provider = TogetherAIProvider::new(
        TogetherAIProviderSettings::new()
            .with_api_key("your-api-key")
    );
    
    let model = provider.chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");
    
    println!("Model: {}", model.model_id());
    Ok(())
}
```

## Configuration

### Environment Variables

Set your Together AI API key as an environment variable:

```bash
export TOGETHER_AI_API_KEY=your-api-key
```

### Using the Client Builder

```rust
use llm_kit_togetherai::TogetherAIClient;

let provider = TogetherAIClient::new()
    .api_key("your-api-key")
    .base_url("https://api.together.xyz/v1")
    .header("Custom-Header", "value")
    .name("my-togetherai")
    .build();
```

### Builder Methods

The `TogetherAIClient` builder supports:

- `.api_key(key)` - Set the API key (overrides `TOGETHER_AI_API_KEY` environment variable)
- `.base_url(url)` - Set custom base URL (default: `https://api.together.xyz/v1`)
- `.name(name)` - Set provider name (optional)
- `.header(key, value)` - Add a single custom header
- `.headers(map)` - Add multiple custom headers
- `.load_api_key_from_env()` - Explicitly load API key from environment variable
- `.build()` - Build the provider

## Supported Models

### Chat Models

All Together AI chat models are supported, including:

- **Llama**: `meta-llama/Llama-3.3-70B-Instruct-Turbo`, `meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo`, `meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo`, `meta-llama/Meta-Llama-3.1-405B-Instruct-Turbo`
- **Qwen**: `Qwen/Qwen2.5-Coder-32B-Instruct`, `Qwen/Qwen2.5-7B-Instruct-Turbo`, `Qwen/Qwen2.5-72B-Instruct-Turbo`
- **DeepSeek**: `deepseek-ai/DeepSeek-V3`
- **Mistral**: `mistralai/Mistral-7B-Instruct-v0.3`, `mistralai/Mixtral-8x7B-Instruct-v0.1`, `mistralai/Mixtral-8x22B-Instruct-v0.1`
- **Gemma**: `google/gemma-2-9b-it`, `google/gemma-2-27b-it`

### Embedding Models

- **WhereIsAI**: `WhereIsAI/UAE-Large-V1`
- **BAAI**: `BAAI/bge-large-en-v1.5`, `BAAI/bge-base-en-v1.5`
- **Sentence Transformers**: `sentence-transformers/msmarco-bert-base-dot-v5`

### Image Models

- **FLUX**: `black-forest-labs/FLUX.1-schnell`, `black-forest-labs/FLUX.1-dev`, `black-forest-labs/FLUX.1.1-pro`
- **Stable Diffusion**: `stabilityai/stable-diffusion-xl-base-1.0`, `stabilityai/stable-diffusion-2-1`

### Reranking Models

- **Salesforce**: `Salesforce/Llama-Rank-v1`
- **Mixedbread**: `mixedbread-ai/Mxbai-Rerank-Large-V2`

For a complete list of available models, see the [Together AI Models documentation](https://docs.together.ai/docs/serverless-models).

## Provider-Specific Options

### Chained Model Creation

Together AI provider supports convenient chained model creation:

```rust
use llm_kit_togetherai::TogetherAIClient;

// Create model directly from builder
let model = TogetherAIClient::new()
    .api_key("your-api-key")
    .build()
    .chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");
```

### Multiple Model Types

Together AI supports multiple model types in a single provider:

```rust
use llm_kit_togetherai::TogetherAIClient;

let provider = TogetherAIClient::new()
    .api_key("your-api-key")
    .build();

// Chat models
let chat_model = provider.chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");

// Embedding models
let embedding_model = provider.text_embedding_model("WhereIsAI/UAE-Large-V1");

// Image models
let image_model = provider.image_model("black-forest-labs/FLUX.1-schnell");

// Reranking models
let reranking_model = provider.reranking_model("Salesforce/Llama-Rank-v1");
```

## Examples

See the `examples/` directory for complete examples:

- `chat.rs` - Basic chat completion
- `stream.rs` - Streaming responses
- `chat_tool_calling.rs` - Tool calling with function definitions
- `stream_tool_calling.rs` - Streaming with tool calls
- `text_embedding.rs` - Text embeddings for semantic search
- `image_generation.rs` - Image generation with FLUX and Stable Diffusion
- `reranking.rs` - Document reranking for improved search

Run examples with:

```bash
export TOGETHER_AI_API_KEY=your-api-key
cargo run --example chat
cargo run --example stream
cargo run --example chat_tool_calling
cargo run --example stream_tool_calling
cargo run --example text_embedding
cargo run --example image_generation
cargo run --example reranking
```

## Documentation

- [API Documentation](https://docs.rs/llm-kit-togetherai)
- [AI SDK Documentation](https://github.com/saribmah/llm-kit)
- [Together AI API Reference](https://docs.together.ai/)
- [Together AI Models](https://docs.together.ai/docs/serverless-models)

## License

Apache-2.0

## Contributing

Contributions are welcome! Please see the [Contributing Guide](../CONTRIBUTING.md) for more details.
