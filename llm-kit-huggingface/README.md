# AI SDK Hugging Face

Hugging Face provider for [LLM Kit](https://github.com/saribmah/llm-kit) - Complete integration with the Hugging Face Responses API for chat models with advanced features.

> **Note**: This provider uses the standardized builder pattern. See the [Quick Start](#quick-start) section for the recommended usage.

## Features

- **Text Generation**: Generate text using Hugging Face models via the Responses API
- **Streaming**: Stream responses in real-time with support for tool calls
- **Tool Calling**: Support for function calling with automatic execution
- **Multi-modal**: Support for text and image inputs
- **Provider-Executed Tools**: MCP (Model Context Protocol) integration for server-side tool execution
- **Source Annotations**: Automatic source citations in responses
- **Structured Output**: JSON schema support for constrained generation
- **Reasoning Content**: Support for models with reasoning capabilities

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-kit-huggingface = "0.1"
llm-kit-core = "0.1"
llm-kit-provider = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Using the Client Builder (Recommended)

```rust
use llm_kit_huggingface::HuggingFaceClient;
use llm_kit_provider::language_model::LanguageModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider using the client builder
    let provider = HuggingFaceClient::new()
        .api_key("your-api-key")  // Or use HUGGINGFACE_API_KEY env var
        .build();
    
    // Create a language model
    let model = provider.responses("meta-llama/Llama-3.1-8B-Instruct");
    
    println!("Model: {}", model.model_id());
    println!("Provider: {}", model.provider());
    Ok(())
}
```

### Using Settings Directly (Alternative)

```rust
use llm_kit_huggingface::{HuggingFaceProvider, HuggingFaceProviderSettings};
use llm_kit_provider::language_model::LanguageModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider with settings
    let provider = HuggingFaceProvider::new(
        HuggingFaceProviderSettings::new()
            .with_api_key("your-api-key")
    );
    
    let model = provider.responses("meta-llama/Llama-3.1-8B-Instruct");
    
    println!("Model: {}", model.model_id());
    Ok(())
}
```

## Configuration

### Environment Variables

Set your Hugging Face API key as an environment variable:

```bash
export HUGGINGFACE_API_KEY=your-api-key
```

### Using the Client Builder

```rust
use llm_kit_huggingface::HuggingFaceClient;

let provider = HuggingFaceClient::new()
    .api_key("your-api-key")
    .base_url("https://router.huggingface.co/v1")
    .header("Custom-Header", "value")
    .name("my-huggingface")
    .build();
```

### Builder Methods

The `HuggingFaceClient` builder supports:

- `.api_key(key)` - Set the API key (overrides `HUGGINGFACE_API_KEY` environment variable)
- `.base_url(url)` - Set custom base URL (default: `https://router.huggingface.co/v1`)
- `.name(name)` - Set provider name (optional)
- `.header(key, value)` - Add a single custom header
- `.headers(map)` - Add multiple custom headers
- `.build()` - Build the provider

## Provider-Specific Options

### Provider-Executed Tools (MCP)

Hugging Face Responses API supports Model Context Protocol (MCP), allowing tools to be executed on the provider side:

```rust
use llm_kit_core::{GenerateText, prompt::Prompt, tool::ToolSet};

// When tools are called, check if they were executed by the provider
let result = GenerateText::new(model, Prompt::text("What's the weather?"))
    .tools(tool_set)
    .execute()
    .await?;

for content in result.content {
    if let LanguageModelContent::ToolCall(call) = content {
        if call.provider_executed == Some(true) {
            println!("Tool executed by provider: {}", call.tool_name);
        }
    }
}
```

### Source Annotations

The API automatically returns source citations as separate content items:

```rust
for content in result.content {
    if let LanguageModelContent::Source(source) = content {
        println!("Source: {} - {}", source.url, source.title);
    }
}
```

### Structured Output

Use JSON schema to constrain model output:

```rust
use llm_kit_core::response_format::ResponseFormat;
use serde_json::json;

let result = GenerateText::new(model, Prompt::text("Generate a person"))
    .response_format(ResponseFormat::json_schema(
        json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "age": { "type": "number" }
            }
        })
    ))
    .execute()
    .await?;
```

### Multi-modal Inputs

Include images in your prompts:

```rust
use llm_kit_core::prompt::Prompt;

let prompt = Prompt::new()
    .add_text("What's in this image?")
    .add_image("https://example.com/image.jpg");

let result = GenerateText::new(model, prompt)
    .execute()
    .await?;
```

## Supported Models

The provider includes constants for popular models:

```rust
use llm_kit_huggingface::{
    LLAMA_3_1_8B_INSTRUCT,
    LLAMA_3_1_70B_INSTRUCT,
    LLAMA_3_3_70B_INSTRUCT,
    DEEPSEEK_V3_1,
    DEEPSEEK_R1,
    QWEN3_32B,
    QWEN2_5_7B_INSTRUCT,
    GEMMA_2_9B_IT,
    KIMI_K2_INSTRUCT,
};

let model = provider.responses(LLAMA_3_1_8B_INSTRUCT);
```

All models available via the Hugging Face Responses API are supported. You can also use any model ID as a string:

```rust
let model = provider.responses("meta-llama/Llama-3.1-8B-Instruct");
```

For a complete list of available models, see the [Hugging Face Responses API documentation](https://router.huggingface.co/v1).

## Supported Settings

| Setting | Supported | Notes |
|---------|-----------|-------|
| `temperature` | ✅ | Temperature for sampling |
| `top_p` | ✅ | Nucleus sampling |
| `max_output_tokens` | ✅ | Maximum tokens to generate |
| `tools` | ✅ | Function calling with MCP support |
| `tool_choice` | ✅ | `auto`, `required`, specific tool |
| `response_format` | ✅ | JSON schema support |
| `top_k` | ❌ | Not supported by API |
| `seed` | ❌ | Not supported by API |
| `presence_penalty` | ❌ | Not supported by API |
| `frequency_penalty` | ❌ | Not supported by API |
| `stop_sequences` | ❌ | Not supported by API |

## Examples

See the `examples/` directory for complete examples:

- `chat.rs` - Basic chat completion with usage statistics
- `stream.rs` - Streaming responses with real-time output
- `chat_tool_calling.rs` - Tool calling with function definitions
- `stream_tool_calling.rs` - Streaming with tool calls

Run examples with:

```bash
export HUGGINGFACE_API_KEY=your-api-key
cargo run --example chat
cargo run --example stream
cargo run --example chat_tool_calling
cargo run --example stream_tool_calling
```

## Documentation

- [API Documentation](https://docs.rs/llm-kit-huggingface)
- [AI SDK Documentation](https://github.com/saribmah/llm-kit)
- [Hugging Face Responses API Reference](https://router.huggingface.co/v1)
- [Model Context Protocol (MCP)](https://modelcontextprotocol.io/)

## License

MIT

## Contributing

Contributions are welcome! Please see the [Contributing Guide](../CONTRIBUTING.md) for more details.
