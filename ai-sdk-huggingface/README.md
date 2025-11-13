# AI SDK Hugging Face Provider

Hugging Face provider for AI SDK Rust, supporting the [Hugging Face Responses API](https://router.huggingface.co/v1).

## Features

- ✅ Text generation (streaming and non-streaming)
- ✅ Tool calling (function calling)
- ✅ Multimodal inputs (text + images)
- ✅ Provider-executed tools (MCP integration)
- ✅ Source annotations
- ✅ Structured output (JSON schema)
- ✅ Reasoning content support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ai-sdk-huggingface = "0.1"
ai-sdk-core = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Basic Chat

```rust
use ai_sdk_core::{GenerateText, prompt::Prompt};
use ai_sdk_huggingface::{create_huggingface, HuggingFaceProviderSettings, LLAMA_3_1_8B_INSTRUCT};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider (API key from HUGGINGFACE_API_KEY env var)
    let provider = create_huggingface(
        HuggingFaceProviderSettings::new()
            .load_api_key_from_env()
    );
    
    // Create model
    let model = provider.responses(LLAMA_3_1_8B_INSTRUCT);
    
    // Generate text
    let result = GenerateText::new(model, Prompt::text("What is Rust?"))
        .temperature(0.7)
        .execute()
        .await?;
    
    println!("{}", result.text);
    Ok(())
}
```

### Streaming

```rust
use ai_sdk_core::{StreamText, prompt::Prompt};
use ai_sdk_huggingface::{create_huggingface, HuggingFaceProviderSettings};
use futures_util::StreamExt;

let mut stream = StreamText::new(model, Prompt::text("Tell me a story"))
    .execute()
    .await?;

while let Some(part) = stream.stream.next().await {
    if let LanguageModelStreamPart::TextDelta(delta) = part {
        print!("{}", delta.delta);
    }
}
```

## Available Models

The provider includes constants for common models:

```rust
use ai_sdk_huggingface::{
    LLAMA_3_1_8B_INSTRUCT,
    LLAMA_3_1_70B_INSTRUCT,
    LLAMA_3_3_70B_INSTRUCT,
    DEEPSEEK_V3_1,
    DEEPSEEK_R1,
    QWEN3_32B,
    GEMMA_2_9B_IT,
    // ... and more
};
```

You can also use any model ID as a string:

```rust
let model = provider.responses("meta-llama/Llama-3.1-8B-Instruct");
```

## Configuration

### API Key

Set your API key via environment variable:

```bash
export HUGGINGFACE_API_KEY=your-api-key
```

Or provide it directly:

```rust
let provider = create_huggingface(
    HuggingFaceProviderSettings::new()
        .with_api_key("your-api-key")
);
```

### Custom Base URL

```rust
let provider = create_huggingface(
    HuggingFaceProviderSettings::new()
        .with_base_url("https://custom-endpoint.example.com/v1")
        .with_api_key("key")
);
```

### Custom Headers

```rust
let provider = create_huggingface(
    HuggingFaceProviderSettings::new()
        .with_header("X-Custom-Header", "value")
        .with_api_key("key")
);
```

## Advanced Features

### Tool Calling

```rust
use ai_sdk_core::tool::{Tool, ToolSet};

let tools = ToolSet::new(vec![
    Tool::new("get_weather", |args| async move {
        // Tool implementation
        Ok("Sunny, 72°F".to_string())
    })
    .with_description("Get current weather")
    .with_parameters(json!({
        "type": "object",
        "properties": {
            "location": { "type": "string" }
        }
    }))
]);

let result = GenerateText::new(model, Prompt::text("What's the weather?"))
    .tools(tools)
    .execute()
    .await?;
```

### Multimodal (Images)

```rust
use ai_sdk_core::prompt::{Prompt, ImagePart};

let prompt = Prompt::new()
    .add_text("What's in this image?")
    .add_image("https://example.com/image.jpg");

let result = GenerateText::new(model, prompt)
    .execute()
    .await?;
```

### Structured Output

```rust
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

## Examples

Run the examples:

```bash
# Basic chat
HUGGINGFACE_API_KEY=your-key cargo run --example basic_chat -p ai-sdk-huggingface

# Streaming
HUGGINGFACE_API_KEY=your-key cargo run --example streaming_chat -p ai-sdk-huggingface
```

## Supported Settings

| Setting | Supported | Notes |
|---------|-----------|-------|
| `temperature` | ✅ | Temperature for sampling |
| `top_p` | ✅ | Nucleus sampling |
| `max_output_tokens` | ✅ | Maximum tokens to generate |
| `tools` | ✅ | Function calling |
| `tool_choice` | ✅ | `auto`, `required`, specific tool |
| `response_format` | ✅ | JSON schema support |
| `top_k` | ❌ | Not supported by API |
| `seed` | ❌ | Not supported by API |
| `presence_penalty` | ❌ | Not supported by API |
| `frequency_penalty` | ❌ | Not supported by API |
| `stop_sequences` | ❌ | Not supported by API |

## Provider-Specific Features

### Provider-Executed Tools (MCP)

Hugging Face Responses API can execute tools on the provider side (Model Context Protocol):

```rust
// Tool calls with `provider_executed: true` are handled by the API
for content in result.content {
    if let LanguageModelContent::ToolCall(call) = content {
        if call.provider_executed == Some(true) {
            println!("Tool executed by provider: {}", call.tool_name);
        }
    }
}
```

### Source Annotations

The API returns source citations as separate content items:

```rust
for content in result.content {
    if let LanguageModelContent::Source(source) = content {
        println!("Source: {} - {}", source.url, source.title);
    }
}
```

## Error Handling

```rust
match GenerateText::new(model, prompt).execute().await {
    Ok(result) => println!("{}", result.text),
    Err(e) => eprintln!("Error: {}", e),
}
```

## API Reference

For detailed API documentation, see:
- [Hugging Face Responses API Docs](https://router.huggingface.co/v1)
- [AI SDK Core Documentation](../ai-sdk-core/README.md)

## License

MIT
