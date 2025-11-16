# AI SDK Azure

Azure OpenAI provider for [AI SDK Rust](https://github.com/saribmah/ai-sdk) - Complete integration with Azure OpenAI Service for chat, completions, embeddings, and image generation.

> **Note**: This provider uses the standardized builder pattern. See the [Quick Start](#quick-start) section for the recommended usage.

## Features

- **Chat Models**: GPT-4, GPT-3.5-turbo, and other chat models
- **Completion Models**: GPT-3.5-turbo-instruct and other completion models
- **Embedding Models**: text-embedding-ada-002 and other embedding models
- **Image Models**: DALL-E 3 and other image generation models
- **Azure-specific Authentication**: Uses `api-key` header
- **Flexible URL Formats**: Supports both v1 API and deployment-based URLs
- **Multiple API Versions**: Configure API version for different Azure OpenAI endpoints

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ai-sdk-azure = "0.1"
ai-sdk-core = "0.1"
ai-sdk-provider = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Using the Client Builder (Recommended)

```rust
use ai_sdk_azure::AzureClient;
use ai_sdk_core::{GenerateText, Prompt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider using the client builder
    let provider = AzureClient::new()
        .resource_name("my-azure-resource")
        .api_key("your-api-key")  // Or use AZURE_API_KEY env var
        .build();
    
    // Get a chat model using your deployment name
    let model = provider.chat_model("gpt-4-deployment");
    
    // Generate text
    let result = GenerateText::new(model, Prompt::text("Hello, Azure!"))
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
use ai_sdk_azure::{AzureOpenAIProvider, AzureOpenAIProviderSettings};
use ai_sdk_core::{GenerateText, Prompt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider with settings
    let provider = AzureOpenAIProvider::new(
        AzureOpenAIProviderSettings::new()
            .with_resource_name("my-azure-resource")
            .with_api_key("your-api-key")
    );
    
    let model = provider.chat_model("gpt-4-deployment");
    
    let result = GenerateText::new(model, Prompt::text("Hello, Azure!"))
        .execute()
        .await?;
    
    println!("{}", result.text);
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
use ai_sdk_azure::AzureClient;

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
use ai_sdk_azure::{AzureOpenAIProvider, AzureOpenAIProviderSettings};

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

## URL Formats

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

## Model Types

### Chat Models

Use `.chat_model()` or `.model()` for conversational AI:

```rust
use ai_sdk_azure::AzureClient;
use ai_sdk_core::{GenerateText, Prompt};

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
use ai_sdk_core::Embed;

let model = provider.text_embedding_model("text-embedding-ada-002");

let result = Embed::new(model, "Hello, world!".to_string())
    .execute()
    .await?;

println!("Embedding dimensions: {}", result.embedding.len());
```

### Image Models

Use `.image_model()` for image generation:

```rust
use ai_sdk_core::GenerateImage;

let model = provider.image_model("dall-e-3");

let result = GenerateImage::new(model, "A serene landscape".to_string())
    .size("1024x1024")
    .execute()
    .await?;
```

## Streaming

Stream responses for real-time output:

```rust
use ai_sdk_azure::AzureClient;
use ai_sdk_core::{StreamText, Prompt};
use futures_util::StreamExt;

let provider = AzureClient::new()
    .resource_name("my-resource")
    .api_key("your-api-key")
    .build();

let model = provider.chat_model("gpt-4-deployment");

let result = StreamText::new(model, Prompt::text("Write a story"))
    .temperature(0.8)
    .execute()
    .await?;

let mut text_stream = result.text_stream();
while let Some(text_delta) = text_stream.next().await {
    print!("{}", text_delta);
}
```

## Common API Versions

- `"v1"` - Default, recommended version
- `"2023-05-15"` - Stable release
- `"2024-02-15-preview"` - Preview features
- `"2024-08-01-preview"` - Latest preview

## Prerequisites

To use this provider, you need:

1. An Azure subscription
2. An Azure OpenAI resource
3. Deployed models in your Azure OpenAI resource
4. API key from Azure portal

## Documentation

- [API Documentation](https://docs.rs/ai-sdk-azure)
- [AI SDK Documentation](https://github.com/saribmah/ai-sdk)
- [Azure OpenAI Documentation](https://learn.microsoft.com/en-us/azure/ai-services/openai/)

## Examples

See the [examples directory](../examples/) for complete examples:

- `azure_basic.rs` - Comprehensive examples of all features

## License

Licensed under Apache License, Version 2.0 ([LICENSE](../LICENSE))

## Contributing

Contributions are welcome! Please see the [Contributing Guide](../CONTRIBUTING.md) for more details.
