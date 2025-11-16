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
ai-sdk-core = "0.1.0"
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
use ai_sdk_core::{GenerateText, prompt::Prompt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider (reads TOGETHER_AI_API_KEY from environment)
    let provider = TogetherAIClient::new()
        .load_api_key_from_env()
        .build();

    // Create a chat model
    let model = provider.chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");

    // Generate text
    let result = GenerateText::new(model, Prompt::text("What is Rust?"))
        .execute()
        .await?;

    println!("{}", result.text);
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

### Chat Model

```rust
use ai_sdk_togetherai::TogetherAIClient;
use ai_sdk_core::{GenerateText, prompt::Prompt};

let provider = TogetherAIClient::new()
    .load_api_key_from_env()
    .build();

let model = provider.chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");

let result = GenerateText::new(model, Prompt::text("Hello!"))
    .temperature(0.7)
    .max_output_tokens(100)
    .execute()
    .await?;

println!("{}", result.text);
```

### Streaming Chat

```rust
use ai_sdk_togetherai::TogetherAIClient;
use ai_sdk_core::{StreamText, prompt::Prompt};
use futures_util::StreamExt;

let provider = TogetherAIClient::new()
    .load_api_key_from_env()
    .build();

let model = provider.chat_model("meta-llama/Llama-3.3-70B-Instruct-Turbo");

let result = StreamText::new(model, Prompt::text("Tell me a story"))
    .execute()
    .await?;

let mut text_stream = result.text_stream();
while let Some(delta) = text_stream.next().await {
    print!("{}", delta);
}
```

### Text Embeddings

```rust
use ai_sdk_togetherai::TogetherAIClient;
use ai_sdk_core::Embed;

let provider = TogetherAIClient::new()
    .load_api_key_from_env()
    .build();

let model = provider.text_embedding_model("WhereIsAI/UAE-Large-V1");

let result = Embed::new(model, "Hello world".to_string())
    .execute()
    .await?;

println!("Embedding vector length: {}", result.embedding.len());
```

### Image Generation

```rust
use ai_sdk_togetherai::TogetherAIClient;
use ai_sdk_core::GenerateImage;

let provider = TogetherAIClient::new()
    .load_api_key_from_env()
    .build();

let model = provider.image_model("black-forest-labs/FLUX.1-schnell");

let result = GenerateImage::new(model, "A serene mountain landscape".to_string())
    .size("1024x1024")
    .n(1)
    .execute()
    .await?;

// Images are returned as base64 encoded strings
for image in result.images {
    // Save or process image
}
```

### Document Reranking

```rust
use ai_sdk_togetherai::TogetherAIClient;
use ai_sdk_core::Rerank;
use ai_sdk_provider::reranking_model::call_options::RerankingDocuments;

let provider = TogetherAIClient::new()
    .load_api_key_from_env()
    .build();

let model = provider.reranking_model("Salesforce/Llama-Rank-v1");

let documents = RerankingDocuments::from_strings(vec![
    "The capital of France is Paris".to_string(),
    "Python is a programming language".to_string(),
    "Paris is known for the Eiffel Tower".to_string(),
]);

let result = Rerank::new(model, documents, "What is the capital of France?".to_string())
    .top_n(2)
    .execute()
    .await?;

for ranked_doc in result.ranking {
    println!("Index: {}, Score: {}", ranked_doc.index, ranked_doc.relevance_score);
}
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
