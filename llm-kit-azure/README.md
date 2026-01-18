# AI SDK Azure

Azure OpenAI provider for [LLM Kit](https://github.com/saribmah/llm-kit) - Complete integration with Azure OpenAI Service for chat, completions, embeddings, and image generation.

> **Note**: This provider uses the standardized builder pattern. See the [Quick Start](#quick-start) section for the recommended usage.

## Features

- **Text Generation**: Generate text using GPT-4, GPT-3.5-turbo, and other Azure OpenAI chat models
- **Streaming**: Stream responses in real-time for interactive applications
- **Tool Calling**: Support for function calling with Azure OpenAI models
- **Text Embedding**: Generate embeddings with text-embedding-ada-002 and other embedding models
- **Image Generation**: Create images using DALL-E 3 and other image generation models
- **Multi-modal**: Support for text and images in conversations
- **Completion Models**: Access to GPT-3.5-turbo-instruct and other completion models
- **Azure-specific Authentication**: Uses `api-key` header for Azure authentication
- **Flexible URL Formats**: Supports both v1 API and deployment-based URLs
- **Multiple API Versions**: Configure API version for different Azure OpenAI endpoints

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-kit-azure = "0.1"
llm-kit-core = "0.1"
llm-kit-provider = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Using the Client Builder (Recommended)

```rust
use llm_kit_azure::AzureClient;
use llm_kit_provider::LanguageModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider using the client builder
    let provider = AzureClient::new()
        .resource_name("my-azure-resource")
        .api_key("your-api-key")  // Or use AZURE_API_KEY env var
        .build();
    
    // Get a chat model using your deployment name
    let model = provider.chat_model("gpt-4-deployment");
    
    println!("Model: {}", model.model_id());
    println!("Provider: {}", model.provider());
    Ok(())
}
```

### Using Settings Directly (Alternative)

```rust
use llm_kit_azure::{AzureOpenAIProvider, AzureOpenAIProviderSettings};
use llm_kit_provider::LanguageModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider with settings
    let provider = AzureOpenAIProvider::new(
        AzureOpenAIProviderSettings::new()
            .with_resource_name("my-azure-resource")
            .with_api_key("your-api-key")
    );
    
    let model = provider.chat_model("gpt-4-deployment");
    
    println!("Model: {}", model.model_id());
    Ok(())
}
```

## Configuration

### Environment Variables

Set your Azure OpenAI credentials as environment variables:

```bash
export AZURE_API_KEY=your-api-key
export AZURE_RESOURCE_NAME=my-azure-resource  # Or use AZURE_BASE_URL
```

### Using the Client Builder

```rust
use llm_kit_azure::AzureClient;

// With resource name (most common)
let provider = AzureClient::new()
    .resource_name("my-azure-resource")
    .api_key("your-api-key")
    .api_version("2024-02-15-preview")
    .build();

// With custom base URL
let provider = AzureClient::new()
    .base_url("https://my-resource.openai.azure.com/openai")
    .api_key("your-api-key")
    .build();

// With custom headers
let provider = AzureClient::new()
    .resource_name("my-resource")
    .api_key("your-api-key")
    .header("X-Custom-Header", "value")
    .build();

// With deployment-based URLs (legacy format)
let provider = AzureClient::new()
    .resource_name("my-resource")
    .api_key("your-api-key")
    .use_deployment_based_urls(true)
    .build();
```

### Using Settings Directly

```rust
use llm_kit_azure::{AzureOpenAIProvider, AzureOpenAIProviderSettings};

let settings = AzureOpenAIProviderSettings::new()
    .with_resource_name("my-azure-resource")
    .with_api_key("your-api-key")
    .with_api_version("2024-02-15-preview")
    .with_header("X-Custom-Header", "value");

let provider = AzureOpenAIProvider::new(settings);
```

### Builder Methods

The `AzureClient` builder supports:

- `.resource_name(name)` - Set Azure OpenAI resource name
- `.base_url(url)` - Set custom base URL
- `.api_key(key)` - Set the API key
- `.api_version(version)` - Set API version (default: "v1")
- `.header(key, value)` - Add a single custom header
- `.headers(map)` - Add multiple custom headers
- `.use_deployment_based_urls(bool)` - Use legacy deployment-based URL format
- `.build()` - Build the provider

## Azure-Specific Features

### URL Formats

Azure OpenAI supports two URL formats:

### V1 API Format (Default)
```
https://{resource}.openai.azure.com/openai/v1{path}?api-version={version}
```

This is the recommended format.

### Deployment-Based Format (Legacy)
```
https://{resource}.openai.azure.com/openai/deployments/{deployment}{path}?api-version={version}
```

Enable this format with `.use_deployment_based_urls(true)`.

### API Versions

Azure OpenAI supports multiple API versions. Common versions include:

- `"v1"` - Default, recommended version
- `"2023-05-15"` - Stable release
- `"2024-02-15-preview"` - Preview features
- `"2024-08-01-preview"` - Latest preview

Set the API version using the builder:

```rust
let provider = AzureClient::new()
    .resource_name("my-resource")
    .api_key("your-api-key")
    .api_version("2024-02-15-preview")
    .build();
```

### Prerequisites

To use this provider, you need:

1. An Azure subscription
2. An Azure OpenAI resource
3. Deployed models in your Azure OpenAI resource
4. API key from Azure portal

## Supported Models

Azure OpenAI supports various model types through deployments. You must first deploy models in your Azure OpenAI resource before using them.

### Chat Models

GPT-4 and GPT-3.5 models for conversational AI:
- `gpt-4` - Most capable model
- `gpt-4-turbo` - Faster GPT-4 variant
- `gpt-35-turbo` - Fast and efficient
- And other chat models available in your Azure deployment

### Embedding Models

Text embedding models:
- `text-embedding-ada-002` - Standard embedding model
- `text-embedding-3-small` - Smaller embedding model
- `text-embedding-3-large` - Larger embedding model

### Image Models

Image generation models:
- `dall-e-3` - Latest DALL-E model
- `dall-e-2` - Previous generation

### Completion Models

Text completion models:
- `gpt-35-turbo-instruct` - Instruction-tuned completion model

**Note:** Model availability depends on your Azure OpenAI resource region and deployment. Use your deployment name (not the base model name) when creating models.

## Usage Examples

### Model Types

### Chat Models

Use `.chat_model()` or `.model()` for conversational AI:

```rust
use llm_kit_azure::AzureClient;
use llm_kit_core::{GenerateText, Prompt};

let provider = AzureClient::new()
    .resource_name("my-resource")
    .api_key("your-api-key")
    .build();

// Use your deployment name
let model = provider.chat_model("gpt-4-deployment");

let result = GenerateText::new(model, Prompt::text("Hello!"))
    .execute()
    .await?;
```

### Completion Models

Use `.completion_model()` for text completion:

```rust
let model = provider.completion_model("gpt-35-turbo-instruct");
```

### Embedding Models

Use `.text_embedding_model()` for embeddings:

```rust
let model = provider.text_embedding_model("text-embedding-ada-002");
```

### Image Models

Use `.image_model()` for image generation:

```rust
let model = provider.image_model("dall-e-3");
```

### Streaming

Stream responses for real-time output. See `examples/stream.rs` for a complete example.

### Tool Calling

Azure OpenAI supports function calling for tool integration. See `examples/chat_tool_calling.rs` for a complete example.

## Examples

See the `examples/` directory for complete examples:

- `chat.rs` - Basic chat completion
- `stream.rs` - Streaming responses
- `chat_tool_calling.rs` - Tool calling with chat models
- `stream_tool_calling.rs` - Streaming with tool calling
- `text_embedding.rs` - Text embeddings
- `image_generation.rs` - Image generation with DALL-E

Run examples with:

```bash
cargo run --example chat
cargo run --example stream
cargo run --example chat_tool_calling
cargo run --example text_embedding
cargo run --example image_generation
```

## Documentation

- [API Documentation](https://docs.rs/llm-kit-azure)
- [AI SDK Documentation](https://github.com/saribmah/llm-kit)
- [Azure OpenAI Documentation](https://learn.microsoft.com/en-us/azure/ai-services/openai/)

## License

Licensed under Apache License, Version 2.0 ([LICENSE](../LICENSE))

## Contributing

Contributions are welcome! Please see the [Contributing Guide](../CONTRIBUTING.md) for more details.
