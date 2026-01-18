# LLM Kit OpenAI-Compatible

OpenAI-compatible provider for [LLM Kit](https://github.com/saribmah/llm-kit) - Universal provider supporting OpenAI, Azure OpenAI, and any OpenAI-compatible API.

> **Note**: This provider uses the standardized builder pattern. See the [Quick Start](#quick-start) section for the recommended usage.

## Features

- **Text Generation**: Generate text with chat and completion models (GPT-4, GPT-3.5, and compatible)
- **Streaming**: Real-time streaming responses for immediate feedback
- **Tool Calling**: Support for function/tool calling with compatible models
- **Text Embedding**: Generate embeddings with OpenAI embedding models
- **Image Generation**: Create images with DALL-E and compatible models
- **Multi-Provider Support**: Works with OpenAI, Azure OpenAI, Together AI, Perplexity, and any OpenAI-compatible API
- **Azure OpenAI**: Full support for Azure OpenAI deployments with query parameters
- **Custom Headers & Query Parameters**: Complete control over HTTP requests for custom APIs
- **Structured Outputs**: Support for structured response formats
- **Organization & Project**: OpenAI organization and project ID support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-kit-openai-compatible = "0.1"
llm-kit-core = "0.1"
llm-kit-provider = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Using the Client Builder (Recommended)

```rust
use llm_kit_openai_compatible::OpenAICompatibleClient;
use llm_kit_core::{GenerateText, Prompt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider using the client builder
    let provider = OpenAICompatibleClient::new()
        .base_url("https://api.openai.com/v1")
        .api_key("your-api-key")
        .build();
    
    // Create a language model
    let model = provider.chat_model("gpt-4");
    
    // Generate text
    let result = GenerateText::new(std::sync::Arc::new(model), Prompt::text("Hello, GPT!"))
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
use llm_kit_openai_compatible::{OpenAICompatibleProvider, OpenAICompatibleProviderSettings};
use llm_kit_core::{GenerateText, Prompt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider with settings
    let settings = OpenAICompatibleProviderSettings::new(
        "https://api.openai.com/v1",
        "openai"
    ).with_api_key("your-api-key");
    
    let provider = OpenAICompatibleProvider::new(settings);
    let model = provider.chat_model("gpt-4");
    
    let result = GenerateText::new(std::sync::Arc::new(model), Prompt::text("Hello, GPT!"))
        .execute()
        .await?;
    
    println!("{}", result.text);
    Ok(())
}
```

## Configuration

### Environment Variables

You can set environment variables for your API keys:

```bash
export OPENAI_API_KEY=your-api-key
export AZURE_API_KEY=your-azure-key
export TOGETHER_API_KEY=your-together-key
```

Note: This provider does not automatically load environment variables. Use the builder's `.api_key()` method to set keys explicitly.

### OpenAI

```rust
use llm_kit_openai_compatible::OpenAICompatibleClient;

let provider = OpenAICompatibleClient::new()
    .base_url("https://api.openai.com/v1")
    .api_key("your-api-key")
    .build();

let model = provider.chat_model("gpt-4");
```

### Azure OpenAI

```rust
use llm_kit_openai_compatible::OpenAICompatibleClient;

let provider = OpenAICompatibleClient::new()
    .base_url("https://my-resource.openai.azure.com/openai")
    .name("azure-openai")
    .api_key("your-api-key")
    .query_param("api-version", "2024-02-15-preview")
    .build();

let model = provider.chat_model("gpt-4");
```

### Together AI

```rust
use llm_kit_openai_compatible::OpenAICompatibleClient;

let provider = OpenAICompatibleClient::new()
    .base_url("https://api.together.xyz/v1")
    .name("together")
    .api_key("your-api-key")
    .build();

let model = provider.chat_model("meta-llama/Llama-3-70b-chat-hf");
```

### Custom OpenAI-Compatible Service

```rust
use llm_kit_openai_compatible::OpenAICompatibleClient;

let provider = OpenAICompatibleClient::new()
    .base_url("https://api.example.com/v1")
    .name("custom")
    .api_key("your-api-key")
    .header("X-Custom-Header", "value")
    .query_param("version", "2024-01")
    .build();

let model = provider.chat_model("custom-model");
```

## Builder Methods

The `OpenAICompatibleClient` builder supports:

- `.base_url(url)` - Set the API base URL (default: `https://api.openai.com/v1`)
- `.name(name)` - Set provider name (default: `openai`)
- `.api_key(key)` - Set the API key
- `.organization(org)` - Set organization ID (OpenAI-specific)
- `.project(project)` - Set project ID (OpenAI-specific)
- `.header(key, value)` - Add a single custom header
- `.headers(map)` - Add multiple custom headers
- `.query_param(key, value)` - Add a single URL query parameter
- `.query_params(map)` - Add multiple URL query parameters
- `.include_usage(bool)` - Include usage info in streaming responses
- `.supports_structured_outputs(bool)` - Enable structured outputs support
- `.build()` - Build the provider

## Model Types

### Chat Models

```rust
use llm_kit_openai_compatible::OpenAICompatibleClient;
use llm_kit_core::{GenerateText, Prompt};

let provider = OpenAICompatibleClient::new()
    .api_key("your-api-key")
    .build();

let model = provider.chat_model("gpt-4");

let result = GenerateText::new(std::sync::Arc::new(model), Prompt::text("Hello!"))
    .execute()
    .await?;
```

### Completion Models

```rust
use llm_kit_openai_compatible::OpenAICompatibleClient;
use llm_kit_core::{GenerateText, Prompt};

let provider = OpenAICompatibleClient::new()
    .api_key("your-api-key")
    .build();

let model = provider.completion_model("gpt-3.5-turbo-instruct");

let result = GenerateText::new(std::sync::Arc::new(model), Prompt::text("Complete this:"))
    .execute()
    .await?;
```

### Text Embedding Models

```rust
use llm_kit_openai_compatible::OpenAICompatibleClient;
use llm_kit_core::EmbedMany;

let provider = OpenAICompatibleClient::new()
    .api_key("your-api-key")
    .build();

let model = provider.text_embedding_model("text-embedding-3-small");

let texts = vec![
    "The capital of France is Paris.".to_string(),
    "The capital of Germany is Berlin.".to_string(),
];

let result = EmbedMany::new(std::sync::Arc::new(model), texts).execute().await?;
println!("Generated {} embeddings", result.embeddings.len());
```

### Image Models

```rust
use llm_kit_openai_compatible::OpenAICompatibleClient;
use llm_kit_core::GenerateImage;

let provider = OpenAICompatibleClient::new()
    .api_key("your-api-key")
    .build();

let model = provider.image_model("dall-e-3");

let result = GenerateImage::new(
    std::sync::Arc::new(model),
    "A beautiful sunset over the ocean".to_string(),
)
.size("1024x1024")
.n(1)
.execute()
.await?;

println!("Generated {} image(s)", result.images.len());
```

## Streaming

Stream responses for real-time output:

```rust
use llm_kit_openai_compatible::OpenAICompatibleClient;
use llm_kit_core::{StreamText, Prompt};
use futures_util::StreamExt;

let provider = OpenAICompatibleClient::new()
    .api_key("your-api-key")
    .build();

let model = provider.chat_model("gpt-4");

let result = StreamText::new(std::sync::Arc::new(model), Prompt::text("Write a story"))
    .temperature(0.8)
    .execute()
    .await?;

let mut text_stream = result.text_stream();
while let Some(text_delta) = text_stream.next().await {
    print!("{}", text_delta);
}
```

## Tool Calling

Support for function/tool calling with compatible models:

```rust
use llm_kit_openai_compatible::OpenAICompatibleClient;
use llm_kit_core::{GenerateText, Prompt, Tool, ToolSet};

let provider = OpenAICompatibleClient::new()
    .api_key("your-api-key")
    .build();

let model = provider.chat_model("gpt-4");

// Define a tool (see llm-kit-core docs for details)
let tools = ToolSet::from_vec(vec![
    // Your tools here
]);

let result = GenerateText::new(std::sync::Arc::new(model), Prompt::text("What's the weather?"))
    .tools(tools)
    .execute()
    .await?;
```

## Advanced Configuration

### Custom Headers and Organization

```rust
use llm_kit_openai_compatible::OpenAICompatibleClient;

let provider = OpenAICompatibleClient::new()
    .api_key("your-api-key")
    .organization("org-123")
    .project("proj-456")
    .header("X-Custom-Header", "value")
    .build();
```

### Query Parameters for Azure

```rust
use llm_kit_openai_compatible::OpenAICompatibleClient;

let provider = OpenAICompatibleClient::new()
    .base_url("https://my-resource.openai.azure.com/openai")
    .api_key("your-api-key")
    .query_param("api-version", "2024-02-15-preview")
    .build();
```

### Chained Usage

```rust
use llm_kit_openai_compatible::OpenAICompatibleClient;
use llm_kit_core::{GenerateText, Prompt};

// Create provider and model in one chain
let model = OpenAICompatibleClient::new()
    .base_url("https://api.openai.com/v1")
    .api_key("your-api-key")
    .build()
    .chat_model("gpt-4");

let result = GenerateText::new(std::sync::Arc::new(model), Prompt::text("Hello!"))
    .execute()
    .await?;
```

## Supported OpenAI Models

### Chat Models
- `gpt-4` - GPT-4 (8K context)
- `gpt-4-turbo` - GPT-4 Turbo
- `gpt-4o` - GPT-4 Optimized
- `gpt-3.5-turbo` - GPT-3.5 Turbo

### Completion Models
- `gpt-3.5-turbo-instruct` - GPT-3.5 Instruct

### Embedding Models
- `text-embedding-3-small` - Small embedding model
- `text-embedding-3-large` - Large embedding model
- `text-embedding-ada-002` - Legacy Ada embedding

### Image Models
- `dall-e-3` - DALL-E 3
- `dall-e-2` - DALL-E 2

## Compatible Services

This provider works with any OpenAI-compatible API, including:

- **OpenAI** - Official OpenAI API
- **Azure OpenAI** - Microsoft's Azure OpenAI Service
- **Together AI** - https://together.ai
- **Perplexity** - https://perplexity.ai
- **Anyscale** - https://anyscale.com
- **Groq** - https://groq.com (also has dedicated `llm-kit-groq` provider)
- **Local LLMs** - LocalAI, Ollama (with OpenAI compatibility), LM Studio

## Examples

See the `examples/` directory for complete examples:

- `chat.rs` - Basic chat completion
- `stream.rs` - Streaming responses
- `chat_tool_calling.rs` - Tool calling with custom tools
- `stream_tool_calling.rs` - Streaming with tool calls
- `text_embedding.rs` - Text embedding generation
- `image_generation.rs` - Image generation with DALL-E

Run examples with:

```bash
cargo run --example chat
cargo run --example stream
cargo run --example text_embedding
cargo run --example image_generation
```

## Documentation

- [API Documentation](https://docs.rs/llm-kit-openai-compatible)
- [LLM Kit Documentation](https://github.com/saribmah/llm-kit)
- [OpenAI API Reference](https://platform.openai.com/docs/api-reference)
- [Azure OpenAI Documentation](https://learn.microsoft.com/en-us/azure/ai-services/openai/)

## License

Licensed under:

- MIT license ([LICENSE-MIT](LICENSE-MIT))

## Contributing

Contributions are welcome! Please see the [Contributing Guide](../CONTRIBUTING.md) for more details.
