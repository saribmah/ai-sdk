# AI SDK Cerebras

Cerebras provider for [LLM Kit](https://github.com/saribmah/llm-kit) - High-speed AI model inference powered by Cerebras Wafer-Scale Engines and CS-3 systems.

> **Note**: This provider uses the standardized builder pattern. See the [Quick Start](#quick-start) section for the recommended usage.

## Features

- **Text Generation**: Full support for chat-based language models including Llama, Qwen, and GPT-OSS
- **Streaming**: Real-time streaming of model responses for interactive applications
- **Tool Calling**: Function calling capabilities for building AI agents
- **Structured Outputs**: JSON schema-based structured output generation with OpenAI-compatible format
- **Reasoning Models**: Support for thinking/reasoning models that expose their thought process

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-kit-cerebras = "0.1"
llm-kit-core = "0.1"
llm-kit-provider = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Using the Client Builder (Recommended)

```rust
use llm_kit_cerebras::CerebrasClient;
use llm_kit_provider::LanguageModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider using the client builder
    let provider = CerebrasClient::new()
        .api_key("your-api-key")  // Or use CEREBRAS_API_KEY env var
        .build();
    
    // Create a language model
    let model = provider.chat_model("llama-3.3-70b");
    
    println!("Model: {}", model.model_id());
    println!("Provider: {}", model.provider());
    Ok(())
}
```

### Using Settings Directly (Alternative)

```rust
use llm_kit_cerebras::{CerebrasProvider, CerebrasProviderSettings};
use llm_kit_provider::LanguageModel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider with settings
    let settings = CerebrasProviderSettings::new("https://api.cerebras.ai/v1")
        .with_api_key("your-api-key");
    
    let provider = CerebrasProvider::new(settings);
    let model = provider.chat_model("llama-3.3-70b");
    
    println!("Model: {}", model.model_id());
    Ok(())
}
```

## Configuration

### Environment Variables

Set your Cerebras API key as an environment variable:

```bash
export CEREBRAS_API_KEY=your-api-key
export CEREBRAS_BASE_URL=https://api.cerebras.ai/v1  # Optional
```

### Using the Client Builder

```rust
use llm_kit_cerebras::CerebrasClient;

let provider = CerebrasClient::new()
    .api_key("your-api-key")
    .base_url("https://api.cerebras.ai/v1")
    .header("Custom-Header", "value")
    .name("my-cerebras-provider")
    .build();
```

### Builder Methods

The `CerebrasClient` builder supports:

- `.api_key(key)` - Set the API key (or use `CEREBRAS_API_KEY` env var)
- `.base_url(url)` - Set custom base URL (default: `https://api.cerebras.ai/v1`)
- `.name(name)` - Set provider name (default: `cerebras`)
- `.header(key, value)` - Add a single custom header
- `.headers(map)` - Add multiple custom headers
- `.build()` - Build the provider

## Supported Models

Cerebras offers high-performance language models powered by Wafer-Scale Engines. For the complete list and latest information, see the [Cerebras Models Overview](https://inference-docs.cerebras.ai/models/overview).

### Production Models

- **Llama 3.1**: `llama3.1-8b` - Llama 3.1 8B parameter model
- **Llama 3.3**: `llama-3.3-70b` - Llama 3.3 70B parameter model
- **GPT-OSS**: `gpt-oss-120b` - GPT-OSS 120B parameter model
- **Qwen 3**: `qwen-3-32b` - Qwen 3 32B parameter model

### Preview Models

- **Qwen 3 Instruct**: `qwen-3-235b-a22b-instruct-2507` - Qwen 3 235B instruct model
- **Qwen 3 Thinking**: `qwen-3-235b-a22b-thinking-2507` - Qwen 3 235B reasoning model with exposed thought process
- **ZAI GLM**: `zai-glm-4.6` - ZAI GLM 4.6 model

You can use predefined model constants:

```rust
use llm_kit_cerebras::chat::models;

let model = provider.chat_model(models::LLAMA_3_3_70B);
let reasoning_model = provider.chat_model(models::QWEN_3_235B_THINKING);
```

## Provider-Specific Features

### Structured Outputs

Cerebras supports OpenAI-compatible structured outputs with JSON schema validation:

```rust
use llm_kit_core::{GenerateText, prompt::Prompt};
use serde_json::json;

let result = GenerateText::new(model, Prompt::text("Extract user info"))
    .response_format(json!({
        "type": "json_schema",
        "json_schema": {
            "name": "user_info",
            "schema": {
                "type": "object",
                "properties": {
                    "name": { "type": "string" },
                    "age": { "type": "number" }
                },
                "required": ["name", "age"]
            }
        }
    }))
    .execute()
    .await?;
```

### Reasoning Models

Cerebras offers thinking models (e.g., `qwen-3-235b-a22b-thinking-2507`) that expose their reasoning process:

```rust
use llm_kit_cerebras::chat::models;
use llm_kit_core::{GenerateText, prompt::Prompt, output::Output};

let model = provider.chat_model(models::QWEN_3_235B_THINKING);
let result = GenerateText::new(model, Prompt::text("Solve this logic puzzle..."))
    .execute()
    .await?;

// Process reasoning and answer separately
for output in &result.content {
    match output {
        Output::Reasoning { text, .. } => {
            println!("💭 Reasoning: {}", text);
        }
        Output::Text { text, .. } => {
            println!("📝 Answer: {}", text);
        }
        _ => {}
    }
}
```

### Performance Notes

- **Ultra-Fast Inference**: Powered by Cerebras Wafer-Scale Engines for high-speed generation
- **Free Tier**: Context windows are temporarily limited to 8192 tokens during early launch phase
- **High Demand**: Some models may have rate limits due to popularity

## Examples

See the `examples/` directory for complete examples:

- `chat.rs` - Basic chat completion using provider traits
- `stream.rs` - Streaming responses using provider traits
- `chat_tool_calling.rs` - Tool calling with provider traits
- `stream_tool_calling.rs` - Streaming with tools using provider traits

Run examples with:

```bash
export CEREBRAS_API_KEY=your-api-key
cargo run --example chat -p llm-kit-cerebras
cargo run --example stream -p llm-kit-cerebras
```

For user-facing examples using `llm-kit-core`, see the root `examples/` directory:

```bash
cargo run --example cerebras_basic_chat
cargo run --example cerebras_streaming_chat
cargo run --example cerebras_tool_calling
cargo run --example cerebras_reasoning
```

## Documentation

- [API Documentation](https://docs.rs/llm-kit-cerebras)
- [AI SDK Documentation](https://github.com/saribmah/llm-kit)
- [Cerebras API Reference](https://inference-docs.cerebras.ai/introduction)
- [Cerebras Models Overview](https://inference-docs.cerebras.ai/models/overview)

## License

Licensed under:

- MIT license ([LICENSE-MIT](LICENSE-MIT))

## Contributing

Contributions are welcome! Please see the [Contributing Guide](../CONTRIBUTING.md) for more details.
