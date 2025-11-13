# ai-sdk-xai

xAI (Grok) provider implementation for the AI SDK in Rust.

## Features

- 🤖 **Chat Completions** - Full support for xAI's Grok models
- 🎨 **Image Generation** - Create images with grok-2-image
- 🧠 **Reasoning Mode** - Access model reasoning with reasoning_effort parameter
- 🔍 **Integrated Search** - Web, X (Twitter), news, and RSS search capabilities
- 📚 **Citations** - Automatic citation extraction from search results
- 🛠️ **Tool Calling** - Full tool/function calling support
- ⚡ **Async/Await** - Built on Tokio for high performance

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ai-sdk-xai = "0.1.0"
ai-sdk-core = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Basic Chat

```rust
use ai_sdk_xai::XaiClient;
use ai_sdk_core::{GenerateText, prompt::Prompt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider (reads XAI_API_KEY from environment)
    let provider = XaiClient::new().build();
    
    // Get a chat model
    let model = provider.chat_model("grok-3-fast");
    
    // Generate text
    let result = GenerateText::new(
        model,
        Prompt::text("What is the capital of France?")
    ).execute().await?;
    
    println!("{}", result.text);
    Ok(())
}
```

### With Custom Configuration

```rust
use ai_sdk_xai::XaiClient;

let provider = XaiClient::new()
    .api_key("your-api-key")
    .base_url("https://api.x.ai/v1")
    .header("X-Custom-Header", "value")
    .build();

let model = provider.chat_model("grok-4");
```

### Streaming

```rust
use ai_sdk_xai::XaiClient;
use ai_sdk_core::{StreamText, prompt::Prompt};
use futures_util::StreamExt;

let provider = XaiClient::new().build();
let model = provider.chat_model("grok-2-1212");

let result = StreamText::new(model, Prompt::text("Tell me a story"))
    .temperature(0.8)
    .execute()
    .await?;

let mut text_stream = result.text_stream();
while let Some(text_delta) = text_stream.next().await {
    print!("{}", text_delta);
}
```

### Image Generation

```rust
use ai_sdk_xai::XaiClient;
use ai_sdk_core::GenerateImage;

let provider = XaiClient::new().build();
let model = provider.image_model("grok-2-image");

let result = GenerateImage::new(
    model,
    "A futuristic cityscape at sunset".to_string(),
)
.n(2)
.execute()
.await?;

println!("Generated {} images", result.images.len());
```

## Available Models

### Chat Models

- `grok-4-fast-non-reasoning` - Fast model without reasoning
- `grok-4-fast-reasoning` - Fast model with reasoning capabilities
- `grok-code-fast-1` - Optimized for code generation
- `grok-4` - Latest Grok-4 model
- `grok-3` - Grok-3 model
- `grok-3-fast` - Faster Grok-3 variant
- `grok-3-mini` - Smaller, efficient model
- `grok-2-vision-1212` - Vision-capable model
- And more...

### Image Models

- `grok-2-image` - Image generation model

## xAI-Specific Features

### Tool Calling

```rust
use ai_sdk_core::{GenerateText, ToolSet};
use ai_sdk_provider_utils::tool::Tool;
use serde_json::json;
use std::sync::Arc;

// Define a tool
let weather_tool = Tool::function(json!({
    "type": "object",
    "properties": {
        "city": {"type": "string", "description": "The city name"}
    },
    "required": ["city"]
}))
.with_description("Get the current weather for a city")
.with_execute(Arc::new(|input, _| {
    // Tool execution logic
    use ai_sdk_provider_utils::tool::ToolExecutionOutput;
    let result = json!({"temperature": 72, "conditions": "sunny"});
    ToolExecutionOutput::Single(Box::pin(async move { Ok(result) }))
}));

// Create tool set
let mut tools = ToolSet::new();
tools.insert("get_weather".to_string(), weather_tool);

// Use with GenerateText
let result = GenerateText::new(model, prompt)
    .tools(tools)
    .execute()
    .await?;
```

### Reasoning Mode

```rust
use ai_sdk_xai::{XaiClient, XaiProviderOptions};
use serde_json::json;

let provider = XaiClient::new().build();
let model = provider.chat_model("grok-2-1212");

let result = GenerateText::new(model, prompt)
    .provider_options(json!({
        "reasoningEffort": "high"
    }))
    .execute()
    .await?;
```

### Integrated Search

```rust
use ai_sdk_xai::{XaiClient, SearchParameters, SearchSource};
use serde_json::json;

let provider = XaiClient::new().build();
let model = provider.chat_model("grok-2-1212");

let result = GenerateText::new(model, prompt)
    .provider_options(json!({
        "searchParameters": {
            "recencyFilter": "day",
            "sources": [{"type": "web"}]
        }
    }))
    .execute()
    .await?;
```

### Citations

Citations are automatically extracted and included in the response:

```rust
let result = GenerateText::new(model, prompt).execute().await?;

// Citations available in result.content
for content in result.content {
    if let ai_sdk_core::output::Output::Source(source) = content {
        println!("Source: {}", source.url);
    }
}
```

## Environment Variables

- `XAI_API_KEY` - Your xAI API key (automatically used if not provided explicitly)

## Examples

See the `examples/` directory for more:

- `xai_basic_chat.rs` - Simple chat completion
- `xai_streaming_chat.rs` - Streaming response
- `xai_provider_options.rs` - Provider-specific options (reasoning effort, search parameters)
- `xai_tool_calling.rs` - Tool/function calling with xAI models

## Status

### ✅ Implemented
- Chat completions (non-streaming & streaming)
- Streaming with Server-Sent Events (SSE)
- Text, reasoning, and tool call deltas
- Image generation  
- Error handling
- Multiple model support
- Basic configuration
- Citations extraction
- Reasoning content support
- Usage tracking (including reasoning tokens)
- **Provider-specific options** (reasoning_effort, search_parameters, parallel_function_calling)
- **Tool calling** (function tools with full lifecycle support)

### 🚧 Coming Soon
- Response format (JSON mode, structured outputs)
- Responses API (agentic mode)
- Provider-defined tools (webSearch, xSearch, etc.)

## License

MIT

## Links

- [xAI API Documentation](https://docs.x.ai/)
- [xAI Console](https://console.x.ai/)
- [AI SDK Rust Documentation](https://github.com/your-repo)
