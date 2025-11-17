# AI SDK Anthropic

Anthropic provider for [AI SDK Rust](https://github.com/saribmah/ai-sdk) - Complete Claude integration with streaming, tools, extended thinking, and citations.

> **Note**: This provider uses the standardized builder pattern. See the [Quick Start](#quick-start) section for the recommended usage.

## Features

- **Text Generation**: Generate text using Claude models (Opus, Sonnet, Haiku)
- **Streaming**: Stream responses in real-time for immediate feedback
- **Tool Calling**: Support for both custom tools and Anthropic provider-defined tools
- **Multi-modal**: Support for text and image inputs (vision)
- **Extended Thinking**: Enable Claude's reasoning process with thinking blocks for complex problem-solving
- **Citations**: Enable source citations for generated content with web search and fetch tools
- **Prompt Caching**: Reduce costs and latency with automatic prompt caching
- **Provider-Defined Tools**: Bash execution, web search, web fetch, code execution, computer use, text editor, and persistent memory

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ai-sdk-anthropic = "0.1"
ai-sdk-core = "0.1"
ai-sdk-provider = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Using the Client Builder (Recommended)

```rust
use ai_sdk_anthropic::AnthropicClient;
use ai_sdk_core::{GenerateText, Prompt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider using the client builder
    let provider = AnthropicClient::new()
        .api_key("your-api-key")  // Or use ANTHROPIC_API_KEY env var
        .build();
    
    // Create a language model
    let model = provider.language_model("claude-3-5-sonnet-20241022".to_string());
    
    // Generate text
    let result = GenerateText::new(std::sync::Arc::new(model), Prompt::text("Hello, Claude!"))
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
use ai_sdk_anthropic::{AnthropicProvider, AnthropicProviderSettings};
use ai_sdk_core::{GenerateText, Prompt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider with settings
    let provider = AnthropicProvider::new(AnthropicProviderSettings::default());
    
    let model = provider.language_model("claude-3-5-sonnet-20241022".to_string());
    
    let result = GenerateText::new(std::sync::Arc::new(model), Prompt::text("Hello, Claude!"))
        .execute()
        .await?;
    
    println!("{}", result.text);
    Ok(())
}
```

## Configuration

### Environment Variables

Set your Anthropic API key as an environment variable:

```bash
export ANTHROPIC_API_KEY=your-api-key
export ANTHROPIC_BASE_URL=https://api.anthropic.com/v1  # Optional
```

### Using the Client Builder

```rust
use ai_sdk_anthropic::AnthropicClient;

let provider = AnthropicClient::new()
    .api_key("your-api-key")
    .base_url("https://api.anthropic.com/v1")
    .header("Custom-Header", "value")
    .name("my-anthropic-provider")
    .build();
```

### Using Settings Directly

```rust
use ai_sdk_anthropic::{AnthropicProvider, AnthropicProviderSettings};

let settings = AnthropicProviderSettings::new()
    .with_api_key("your-api-key")
    .with_base_url("https://api.anthropic.com/v1")
    .add_header("Custom-Header", "value")
    .with_name("my-anthropic-provider");

let provider = AnthropicProvider::new(settings);
```

### Builder Methods

The `AnthropicClient` builder supports:

- `.api_key(key)` - Set the API key
- `.base_url(url)` - Set custom base URL
- `.name(name)` - Set provider name
- `.header(key, value)` - Add a single custom header
- `.headers(map)` - Add multiple custom headers
- `.build()` - Build the provider

## Provider-Defined Tools

Anthropic provides several powerful provider-defined tools:

```rust
use ai_sdk_anthropic::anthropic_tools;
use ai_sdk_core::ToolSet;

let tools = ToolSet::from_vec(vec![
    // Execute bash commands
    anthropic_tools::bash_20250124(None),
    
    // Search the web with citations
    anthropic_tools::web_search_20250305()
        .max_uses(5)
        .build(),
    
    // Fetch web content
    anthropic_tools::web_fetch_20250910()
        .citations(true)
        .build(),
    
    // Execute Python code
    anthropic_tools::code_execution_20250825(None),
    
    // Computer use (screenshots + mouse/keyboard)
    anthropic_tools::computer_20250124(1920, 1080, None),
    
    // Text editor
    anthropic_tools::text_editor_20250728()
        .max_characters(10000)
        .build(),
    
    // Persistent memory
    anthropic_tools::memory_20250818(None),
]);
```

## Extended Thinking

Enable Claude's extended reasoning process:

```rust
use ai_sdk_anthropic::AnthropicClient;
use ai_sdk_core::{GenerateText, Prompt};

let provider = AnthropicClient::new().build();
let model = provider.language_model("claude-3-7-sonnet-20250219".to_string());

let result = GenerateText::new(std::sync::Arc::new(model), 
    Prompt::text("Solve this complex problem"))
    .thinking_enabled(true)
    .thinking_budget(10000) // Optional token budget
    .execute()
    .await?;

// Access reasoning
for output in result.experimental_output.iter() {
    if let ai_sdk_provider::language_model::Output::Reasoning(reasoning) = output {
        println!("Reasoning: {}", reasoning.text);
    }
}
```

## Streaming

Stream responses for real-time output:

```rust
use ai_sdk_anthropic::AnthropicClient;
use ai_sdk_core::{StreamText, Prompt};
use futures_util::StreamExt;

let provider = AnthropicClient::new().build();
let model = provider.language_model("claude-3-5-sonnet-20241022".to_string());

let result = StreamText::new(std::sync::Arc::new(model), 
    Prompt::text("Write a story"))
    .temperature(0.8)
    .execute()
    .await?;

let mut text_stream = result.text_stream();
while let Some(text_delta) = text_stream.next().await {
    print!("{}", text_delta);
}
```

## Supported Models

All Claude models are supported, including:

- **Claude 3.5 Sonnet**: `claude-3-5-sonnet-20241022` - Most intelligent model with extended thinking
- **Claude 3.7 Sonnet**: `claude-3-7-sonnet-20250219` - Latest model with enhanced extended thinking capabilities
- **Claude 3 Opus**: `claude-3-opus-20240229` - Powerful model for complex tasks
- **Claude 3 Sonnet**: `claude-3-sonnet-20240229` - Balanced performance and speed
- **Claude 3 Haiku**: `claude-3-haiku-20240307` - Fastest model for simple tasks

For a complete list of available models, see the [Anthropic documentation](https://docs.anthropic.com/en/docs/models-overview).

## Provider-Specific Options

Anthropic-specific options can be passed through `provider_options`:

```rust
use ai_sdk_anthropic::language_model::{ProviderChatLanguageModelOptions, provider_chat_options::*};
use ai_sdk_core::{GenerateText, Prompt};

let options = ProviderChatLanguageModelOptions {
    thinking: Some(Thinking {
        type_: ThinkingType::Enabled,
        budget_tokens: Some(10000),
    }),
    citations: Some(Citations {
        type_: CitationsType::Enabled,
    }),
    ..Default::default()
};

let result = GenerateText::new(model, prompt)
    .provider_options(options)
    .execute()
    .await?;
```

### Available Provider Options

- **`thinking`** - Control extended thinking behavior:
  - `ThinkingType::Enabled` - Enable extended thinking
  - `ThinkingType::Disabled` - Disable extended thinking
  - `budget_tokens` - Optional token limit for thinking

- **`citations`** - Control citation generation:
  - `CitationsType::Enabled` - Enable citations
  - `CitationsType::Disabled` - Disable citations

## Examples

See the `examples/` directory for complete examples:

- `chat.rs` - Basic chat completion with Claude
- `stream.rs` - Streaming responses
- `chat_tool_calling.rs` - Tool calling with custom tools
- `stream_tool_calling.rs` - Streaming with tool calls
- `provider_specific_bash_tool.rs` - Using Anthropic's bash tool
- `provider_specific_defined_tools.rs` - Using all provider-defined tools

Run examples with:

```bash
cargo run --example chat
cargo run --example stream
cargo run --example chat_tool_calling
cargo run --example provider_specific_defined_tools
```

## Documentation

- [API Documentation](https://docs.rs/ai-sdk-anthropic)
- [AI SDK Documentation](https://github.com/saribmah/ai-sdk)
- [Anthropic API Reference](https://docs.anthropic.com/en/api)

## License

Licensed under:

- MIT license ([LICENSE-MIT](LICENSE-MIT))

## Contributing

Contributions are welcome! Please see the [Contributing Guide](../CONTRIBUTING.md) for more details.
