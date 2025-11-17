# Together AI Provider for AI SDK

This crate provides a Together AI provider implementation for the AI SDK, enabling access to Together AI's extensive collection of open-source models.

## Features

- **Chat Models**: Access to Llama, Mistral, Qwen, DeepSeek, and more
- **Completion Models**: Traditional text completion interface
- **Embedding Models**: Text embedding models for semantic search
- **Image Generation**: FLUX and Stable Diffusion models
- **Reranking**: Document reranking for improved search results

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ai-sdk-togetherai = "0.1.0"
ai-sdk-provider = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## API Key Setup

Get your API key from [Together AI](https://api.together.xyz/) and set it up in one of two ways:

### 1. Environment Variable (Recommended)

```bash
export TOGETHER_AI_API_KEY="your-api-key"
```

### 2. Direct Configuration

#### Builder Pattern (Recommended)

```rust
use ai_sdk_togetherai::TogetherAIClient;

let provider = TogetherAIClient::new()
    .api_key("your-api-key")
    .build();
```

#### Direct Instantiation

```rust
use ai_sdk_togetherai::{TogetherAIProvider, TogetherAIProviderSettings};

let provider = TogetherAIProvider::new(
    TogetherAIProviderSettings::new()
        .with_api_key("your-api-key")
);
```

## Quick Start

### Builder Pattern (Recommended)

```rust
use ai_sdk_togetherai::TogetherAIClient;
use ai_sdk_provider::LanguageModel;
use ai_sdk_provider::language_model::call_options::LanguageModelCallOptions;
use ai_sdk_provider::language_model::prompt::LanguageModelMessage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider (reads TOGETHER_AI_API_KEY from environment)
    let provider = TogetherAIClient::new()
        .load_api_key_from_env()
        .build();

    // Create a chat model
    let model = provider.chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");

    // Generate text using provider traits
    let prompt = vec![LanguageModelMessage::user_text("What is Rust?")];
    let options = LanguageModelCallOptions::new(prompt);
    let result = model.do_generate(options).await?;

    println!("{:?}", result.content);
    Ok(())
}
```

### Chained Usage

```rust
use ai_sdk_togetherai::TogetherAIClient;

let model = TogetherAIClient::new()
    .api_key("your-api-key")
    .build()
    .chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");
```

## Usage Examples

See the `examples/` directory for comprehensive usage examples:

- `chat.rs` - Basic chat using `LanguageModel::do_generate()`
- `stream.rs` - Streaming chat using `LanguageModel::do_stream()`
- `chat_tool_calling.rs` - Tool calling with `do_generate()`
- `stream_tool_calling.rs` - Streaming tool calling
- `text_embedding.rs` - Text embeddings using `EmbeddingModel::do_embed()`
- `image_generation.rs` - Image generation using `ImageModel::do_generate()`
- `reranking.rs` - Document reranking using `RerankingModel::do_rerank()`

Run examples with:
```bash
cargo run --example chat -p ai-sdk-togetherai
```

## Available Models

### Chat Models

- `meta-llama/Llama-3.3-70B-Instruct-Turbo`
- `meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo`
- `meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo`
- `meta-llama/Meta-Llama-3.1-405B-Instruct-Turbo`
- `Qwen/Qwen2.5-Coder-32B-Instruct`
- `Qwen/Qwen2.5-7B-Instruct-Turbo`
- `Qwen/Qwen2.5-72B-Instruct-Turbo`
- `deepseek-ai/DeepSeek-V3`
- `mistralai/Mistral-7B-Instruct-v0.3`
- `mistralai/Mixtral-8x7B-Instruct-v0.1`
- `mistralai/Mixtral-8x22B-Instruct-v0.1`
- And many more...

### Embedding Models

- `WhereIsAI/UAE-Large-V1`
- `BAAI/bge-large-en-v1.5`
- `BAAI/bge-base-en-v1.5`

### Image Models

- `black-forest-labs/FLUX.1-schnell`
- `black-forest-labs/FLUX.1-dev`
- `black-forest-labs/FLUX.1.1-pro`
- `stabilityai/stable-diffusion-xl-base-1.0`

### Reranking Models

- `Salesforce/Llama-Rank-v1`
- `mixedbread-ai/Mxbai-Rerank-Large-V2`

For a complete list of available models, see the [Together AI documentation](https://docs.together.ai/docs/serverless-models).

## Configuration Options

### Builder Pattern

```rust
use ai_sdk_togetherai::TogetherAIClient;

let provider = TogetherAIClient::new()
    .api_key("your-api-key")
    .base_url("https://api.together.xyz/v1")  // Custom base URL
    .header("X-Custom-Header", "value")        // Add custom header
    .build();
```

### Direct Instantiation

```rust
use ai_sdk_togetherai::{TogetherAIProvider, TogetherAIProviderSettings};
use std::collections::HashMap;

let mut headers = HashMap::new();
headers.insert("X-Custom-Header".to_string(), "value".to_string());

let provider = TogetherAIProvider::new(
    TogetherAIProviderSettings::new()
        .with_api_key("your-api-key")
        .with_base_url("https://api.together.xyz/v1")  // Custom base URL
        .with_headers(headers)                          // Custom headers
);
```

## Error Handling

All async operations return `Result<T, Box<dyn std::error::Error>>`. Handle errors appropriately:

```rust
match GenerateText::new(model, prompt).execute().await {
    Ok(result) => println!("Success: {}", result.text),
    Err(e) => eprintln!("Error: {}", e),
}
```

## License

Apache-2.0

## Links

- [Together AI API Documentation](https://docs.together.ai/)
- [AI SDK Documentation](https://github.com/yourusername/ai-sdk)
- [Together AI Models](https://api.together.ai/models)
