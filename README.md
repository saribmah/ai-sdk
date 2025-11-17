# AI SDK Rust

A unified Rust SDK for building AI-powered applications with multiple model providers. Build with type safety, async/await, and ergonomic APIs designed for the Rust ecosystem.

> **Status**: All 12 providers standardized with comprehensive documentation and examples. Ready for production use.

## Features

- **Unified Interface**: Single API for all providers - write once, switch providers easily
- **Multiple Providers**: 12 standardized providers including OpenAI, Anthropic, Azure, Groq, DeepSeek, and more
- **Builder Pattern APIs**: Ergonomic, fluent APIs for all operations
- **Type Safety**: Leverages Rust's type system for compile-time safety
- **Async/Await**: Built on Tokio for efficient async operations
- **Streaming Support**: Real-time streaming with callbacks and transforms
- **Tool Calling**: Dynamic and type-safe tool integration for function calling
- **Multi-step Execution**: Automatic tool execution with multiple reasoning steps
- **Agent System**: Reusable AI agents with persistent configuration
- **Storage Integration**: Persistent conversation history with automatic loading
- **Multiple Capabilities**: Text generation, embeddings, images, speech, transcription, reranking

## Quick Start

### Installation

Add the core library and a provider to your `Cargo.toml`:

```toml
[dependencies]
ai-sdk-core = "0.1"
ai-sdk-openai = "0.1"  # Or any other provider
tokio = { version = "1", features = ["full"] }
```

### Basic Example

```rust
use ai_sdk_core::{GenerateText, prompt::Prompt};
use ai_sdk_openai::OpenAIClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider
    let provider = OpenAIClient::new()
        .api_key(std::env::var("OPENAI_API_KEY")?)
        .build();
    
    // Get model
    let model = provider.chat_model("gpt-4o-mini");
    
    // Generate text
    let result = GenerateText::new(model, Prompt::text("What is Rust?"))
        .temperature(0.7)
        .max_output_tokens(100)
        .execute()
        .await?;
    
    println!("Response: {}", result.text);
    Ok(())
}
```

## Supported Providers

All providers follow the same standardized builder pattern and API:

| Provider | Chat | Embed | Image | Speech | Transcription | Reranking | Status |
|----------|------|-------|-------|--------|---------------|-----------|--------|
| [OpenAI](ai-sdk-openai/) | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ Standardized |
| [Anthropic](ai-sdk-anthropic/) | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ Standardized |
| [Azure](ai-sdk-azure/) | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ | ✅ Standardized |
| [Groq](ai-sdk-groq/) | ✅ | ❌ | ❌ | ✅ | ✅ | ❌ | ✅ Standardized |
| [DeepSeek](ai-sdk-deepseek/) | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ Standardized |
| [xAI](ai-sdk-xai/) | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | ✅ Standardized |
| [TogetherAI](ai-sdk-togetherai/) | ✅ | ✅ | ✅ | ❌ | ❌ | ✅ | ✅ Standardized |
| [Baseten](ai-sdk-baseten/) | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ Standardized |
| [Hugging Face](ai-sdk-huggingface/) | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ Standardized |
| [ElevenLabs](ai-sdk-elevenlabs/) | ❌ | ❌ | ❌ | ✅ | ✅ | ❌ | ✅ Standardized |
| [AssemblyAI](ai-sdk-assemblyai/) | ❌ | ❌ | ❌ | ❌ | ✅ | ❌ | ✅ Standardized |
| [OpenAI-Compatible](ai-sdk-openai-compatible/) | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ | ✅ Standardized |

**Legend:**
- ✅ = Feature supported and implemented
- ❌ = Feature not supported by provider

### Using Different Providers

All providers use the same builder pattern:

```rust
// OpenAI
use ai_sdk_openai::OpenAIClient;
let provider = OpenAIClient::new()
    .api_key("your-key")
    .build();

// Anthropic (Claude)
use ai_sdk_anthropic::AnthropicClient;
let provider = AnthropicClient::new()
    .api_key("your-key")
    .build();

// Azure OpenAI
use ai_sdk_azure::AzureClient;
let provider = AzureClient::new()
    .api_key("your-key")
    .resource_name("your-resource")
    .deployment_id("your-deployment")
    .build();

// Groq (ultra-fast inference)
use ai_sdk_groq::GroqClient;
let provider = GroqClient::new()
    .api_key("your-key")
    .build();

// DeepSeek (reasoning models)
use ai_sdk_deepseek::DeepSeekClient;
let provider = DeepSeekClient::new()
    .api_key("your-key")
    .build();

// And more...
```

Switch providers by changing just 2-3 lines of code. The rest of your application remains the same.

## Core Capabilities

### Text Generation

Generate text with comprehensive configuration options:

```rust
use ai_sdk_core::{GenerateText, prompt::Prompt};

let result = GenerateText::new(model, Prompt::text("Write a poem"))
    .temperature(0.8)
    .max_output_tokens(500)
    .top_p(0.9)
    .frequency_penalty(0.5)
    .presence_penalty(0.5)
    .seed(42)
    .execute()
    .await?;

println!("Response: {}", result.text);
```

### Streaming

Stream responses in real-time:

```rust
use ai_sdk_core::{StreamText, prompt::Prompt};
use futures::StreamExt;

let result = StreamText::new(model, Prompt::text("Tell me a story"))
    .temperature(0.8)
    .on_chunk(|chunk| {
        println!("Chunk: {:?}", chunk);
    })
    .execute()
    .await?;

let mut stream = result.text_stream();
while let Some(text) = stream.next().await {
    print!("{}", text);
}
```

### Tool Calling

Define tools with dynamic or type-safe APIs:

**Dynamic Tools:**

```rust
use ai_sdk_core::{GenerateText, ToolSet};
use ai_sdk_provider_utils::tool::{Tool, ToolExecutionOutput};
use serde_json::json;
use std::sync::Arc;

let tool = Tool::function(json!({
    "type": "object",
    "properties": {
        "city": {"type": "string", "description": "City name"}
    },
    "required": ["city"]
}))
.with_description("Get weather for a city")
.with_execute(Arc::new(|input, _opts| {
    ToolExecutionOutput::Single(Box::pin(async move {
        Ok(json!({"temperature": 72, "conditions": "Sunny"}))
    }))
}));

let mut tools = ToolSet::new();
tools.insert("get_weather".to_string(), tool);

let result = GenerateText::new(model, Prompt::text("What's the weather in Paris?"))
    .tools(tools)
    .execute()
    .await?;
```

**Type-Safe Tools:**

```rust
use ai_sdk_core::tool::TypeSafeTool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, JsonSchema)]
struct WeatherInput {
    city: String,
}

#[derive(Serialize)]
struct WeatherOutput {
    temperature: i32,
    conditions: String,
}

impl TypeSafeTool for WeatherInput {
    type Output = WeatherOutput;
    
    fn description() -> String {
        "Get weather for a city".to_string()
    }
    
    async fn execute(self) -> Result<Self::Output, String> {
        Ok(WeatherOutput {
            temperature: 72,
            conditions: format!("Sunny in {}", self.city),
        })
    }
}
```

### Agent System

Create reusable AI agents with persistent configuration:

```rust
use ai_sdk_core::{Agent, AgentSettings, AgentCallParameters};
use ai_sdk_core::agent::AgentInterface;

// Configure agent once
let settings = AgentSettings::new(model)
    .with_tools(tools)
    .with_temperature(0.7)
    .with_max_output_tokens(500);

let agent = Agent::new(settings);

// Use multiple times
let result1 = agent.generate(AgentCallParameters::from_text("Hello"))?
    .execute()
    .await?;

let result2 = agent.generate(AgentCallParameters::from_text("Follow-up"))?
    .temperature(0.9)  // Override settings per call
    .execute()
    .await?;

// Streaming also supported
let stream_result = agent.stream(AgentCallParameters::from_text("Tell a story"))?
    .execute()
    .await?;
```

### Conversation Storage

Persist conversation history with automatic loading:

```rust
use ai_sdk_storage_filesystem::FilesystemStorage;
use ai_sdk_storage::Storage;
use std::sync::Arc;

let storage: Arc<dyn Storage> = Arc::new(FilesystemStorage::new("./storage")?);
storage.initialize().await?;
let session_id = storage.generate_session_id();

// First message - no history
GenerateText::new(model.clone(), Prompt::text("What is Rust?"))
    .with_storage(storage.clone())
    .with_session_id(session_id.clone())
    .without_history()  // Important!
    .execute()
    .await?;

// Follow-up - history loaded automatically
GenerateText::new(model, Prompt::text("Why should I learn it?"))
    .with_storage(storage)
    .with_session_id(session_id)
    .execute()
    .await?;  // Previous messages included
```

**Enable storage feature:**

```toml
ai-sdk-core = { version = "0.1", features = ["storage"] }
ai-sdk-storage = "0.1"
ai-sdk-storage-filesystem = "0.1"
```

### Embeddings

Generate embeddings for single or multiple texts:

```rust
use ai_sdk_core::{Embed, EmbedMany};

// Single embedding
let result = Embed::new(embedding_model.clone(), "Hello world".to_string())
    .execute()
    .await?;

// Batch embeddings
let texts = vec!["text1".to_string(), "text2".to_string()];
let results = EmbedMany::new(embedding_model, texts)
    .max_parallel_calls(5)
    .execute()
    .await?;
```

### Image Generation

Generate images from text prompts:

```rust
use ai_sdk_core::GenerateImage;

let result = GenerateImage::new(image_model, "A serene landscape".to_string())
    .n(2)
    .size("1024x1024")
    .seed(42)
    .execute()
    .await?;
```

### Speech & Transcription

Convert between text and speech:

```rust
use ai_sdk_core::{GenerateSpeech, Transcribe, AudioInput};

// Text to speech
let result = GenerateSpeech::new(speech_model, "Hello world".to_string())
    .voice("alloy")
    .output_format("mp3")
    .speed(1.0)
    .execute()
    .await?;

// Speech to text
let result = Transcribe::new(transcription_model, AudioInput::Data(audio_data))
    .execute()
    .await?;
```

### Reranking

Rerank documents based on relevance:

```rust
use ai_sdk_core::Rerank;

let documents = vec!["doc1".to_string(), "doc2".to_string()];
let result = Rerank::new(reranking_model, documents, "search query".to_string())
    .top_n(5)
    .execute()
    .await?;
```

## Project Structure

This is a Cargo workspace organized into layers:

### Core Layer

- **[ai-sdk-core](ai-sdk-core/)** - Core functionality with builder APIs, agent system, tool integration, and storage
- **[ai-sdk-provider](ai-sdk-provider/)** - Provider interface and traits for implementing new providers
- **[ai-sdk-provider-utils](ai-sdk-provider-utils/)** - Shared utilities for providers

### Storage Layer

- **[ai-sdk-storage](ai-sdk-storage/)** - Storage trait and types for conversation persistence
- **[ai-sdk-storage-filesystem](ai-sdk-storage-filesystem/)** - Filesystem-based storage implementation

### Provider Implementations

**Language Model Providers:**
- **[ai-sdk-openai](ai-sdk-openai/)** - OpenAI (GPT models)
- **[ai-sdk-anthropic](ai-sdk-anthropic/)** - Anthropic (Claude models) with extended thinking and citations
- **[ai-sdk-deepseek](ai-sdk-deepseek/)** - DeepSeek (reasoning models)
- **[ai-sdk-huggingface](ai-sdk-huggingface/)** - Hugging Face Inference API (Llama, Mistral, Qwen, and more)
- **[ai-sdk-xai](ai-sdk-xai/)** - xAI (Grok models)

**Multi-Feature Providers:**
- **[ai-sdk-azure](ai-sdk-azure/)** - Azure OpenAI (chat, embeddings, images)
- **[ai-sdk-groq](ai-sdk-groq/)** - Groq (ultra-fast chat, speech, transcription)
- **[ai-sdk-togetherai](ai-sdk-togetherai/)** - TogetherAI (chat, embeddings, images, reranking)
- **[ai-sdk-baseten](ai-sdk-baseten/)** - Baseten (chat, embeddings)
- **[ai-sdk-openai-compatible](ai-sdk-openai-compatible/)** - Base for OpenAI-compatible APIs

**Specialized Providers:**
- **[ai-sdk-elevenlabs](ai-sdk-elevenlabs/)** - ElevenLabs (speech generation, transcription)
- **[ai-sdk-assemblyai](ai-sdk-assemblyai/)** - AssemblyAI (transcription)

## Examples

The repository includes 29 comprehensive examples demonstrating all features:

### Setup

1. Copy the example environment file:
```bash
cp .env.example .env
```

2. Add your API key to `.env`:
```
OPENAI_API_KEY=your-api-key-here
```

### Running Examples

```bash
# Set API key
export OPENAI_API_KEY="your-api-key"

# Basic Examples
cargo run --example basic_chat              # Simple text generation
cargo run --example basic_stream            # Streaming responses
cargo run --example basic_embedding         # Text embeddings
cargo run --example basic_image             # Image generation
cargo run --example conversation            # Multi-turn conversations

# Agent Examples
cargo run --example agent_generate          # Reusable agents (non-streaming)
cargo run --example agent_stream            # Reusable agents (streaming)
cargo run --example agent_storage_conversation --features storage  # Agents with storage

# Tool Calling Examples
cargo run --example tool_calling            # Basic tool calling
cargo run --example type_safe_tools         # Type-safe tools
cargo run --example multi_step_tools        # Multi-step execution
cargo run --example stream_tool_calling     # Streaming with tools

# Streaming Examples
cargo run --example stream_transforms       # Stream filtering and batching
cargo run --example partial_output          # Partial JSON parsing

# Storage Examples (require --features storage)
cargo run --example storage_basic --features storage
cargo run --example storage_filesystem_basic --features storage
cargo run --example storage_filesystem_conversation --features storage
cargo run --example storage_conversation_full --features storage

# Provider-Specific Examples
cargo run --example azure_basic             # Azure OpenAI
cargo run --example groq_basic_chat         # Groq
cargo run --example groq_text_to_speech     # Groq TTS
cargo run --example groq_transcription      # Groq transcription
cargo run --example xai_basic_chat          # xAI
```

See the [`examples/`](examples/) directory for all available examples.

## Architecture

The SDK follows a three-layer architecture:

### 1. Core Layer (`ai-sdk-core`)

Provides builder pattern APIs and core functionality:
- **Builders**: `GenerateText`, `StreamText`, `Embed`, `EmbedMany`, `GenerateImage`, `GenerateSpeech`, `Transcribe`, `Rerank`
- **Agent System**: Reusable AI agents with persistent configuration
- **Tool System**: Dynamic and type-safe tool integration
- **Prompt Management**: Standardized message types and conversions
- **Storage Integration**: Conversation persistence

### 2. Provider Layer (`ai-sdk-provider`)

Defines traits for implementing providers:
- **`Provider` trait**: Top-level provider interface
- **Model traits**: `LanguageModel`, `EmbeddingModel`, `ImageModel`, `SpeechModel`, `TranscriptionModel`, `RerankingModel`
- **Standardized types**: `CallOptions`, `Content`, `FinishReason`, `Usage`, `ToolCall`

### 3. Implementation Layer (Provider Crates)

Concrete implementations for each provider:
- API-specific request/response handling
- HTTP client management
- Format conversions
- Provider-specific features

## Documentation

- **[Core Library Documentation](ai-sdk-core/README.md)** - Builder APIs, agent system, tools
- **[Provider Implementation Guide](PROVIDER-IMPLEMENTATION.md)** - How to implement new providers
- **[Provider Standardization](PROVIDER-STANDARDIZATION.md)** - Provider standardization process
- **[Provider Examples Guide](PROVIDER_EXAMPLES.md)** - Required examples for each provider
- **[Contributing Guide](CONTRIBUTING.md)** - How to contribute
- **[Development Guide](DEVELOPMENT.md)** - Development workflow and tools

### Provider Documentation

Each provider has comprehensive README documentation:
- [OpenAI](ai-sdk-openai/README.md)
- [Anthropic](ai-sdk-anthropic/README.md)
- [Azure](ai-sdk-azure/README.md)
- [Groq](ai-sdk-groq/README.md)
- [DeepSeek](ai-sdk-deepseek/README.md)
- [xAI](ai-sdk-xai/README.md)
- [TogetherAI](ai-sdk-togetherai/README.md)
- [Baseten](ai-sdk-baseten/README.md)
- [Hugging Face](ai-sdk-huggingface/README.md)
- [ElevenLabs](ai-sdk-elevenlabs/README.md)
- [AssemblyAI](ai-sdk-assemblyai/README.md)
- [OpenAI-Compatible](ai-sdk-openai-compatible/README.md)

## Development

### Prerequisites

This project uses [just](https://github.com/casey/just) as a command runner:

```bash
cargo install just
```

### Pre-Commit Hooks

Install pre-commit hooks for automatic code quality checks:

```bash
just install-hooks
```

This automatically runs:
- ✅ `rustfmt` - Code formatting (auto-fixes)
- ✅ `clippy` - Linting (blocks commit if issues found)
- ✅ `cargo check` - Compilation verification

### Available Commands

```bash
just                # List all available commands
just install-hooks  # Install git pre-commit hooks
just fmt            # Format code
just clippy         # Run clippy linter
just check          # Quick compile check
just test           # Run all tests
just build          # Build all crates
just doc            # Build documentation
just pre-commit     # Run all pre-commit checks
just ci             # Run all CI checks locally
```

See [DEVELOPMENT.md](DEVELOPMENT.md) for detailed development guidelines.

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p ai-sdk-core
cargo test -p ai-sdk-openai
cargo test -p ai-sdk-anthropic

# Run with output
cargo test -- --nocapture

# Run storage tests
cargo test -p ai-sdk-core --features storage
```

### Building

```bash
# Build all crates
cargo build

# Build in release mode
cargo build --release

# Check without building (faster)
cargo check

# Check examples
cargo check --examples
```

## Project Status

### ✅ Completed

**Core Functionality:**
- ✅ Builder APIs: `GenerateText`, `StreamText`, `Embed`, `EmbedMany`, `GenerateImage`, `GenerateSpeech`, `Transcribe`, `Rerank`
- ✅ Agent system with persistent configuration
- ✅ Tool calling (dynamic and type-safe)
- ✅ Multi-step tool execution
- ✅ Streaming with callbacks and transforms
- ✅ Conversation storage with automatic history loading
- ✅ Message types (text, images, files, tool calls, reasoning)
- ✅ Error handling with retry logic
- ✅ Cancellation support

**Providers:**
- ✅ All 12 providers standardized
- ✅ Comprehensive README documentation for all providers
- ✅ 54/54 provider examples implemented (100%)
- ✅ Consistent builder pattern across all providers
- ✅ Provider-specific features documented

**Testing & Documentation:**
- ✅ 1,500+ unit tests across all crates
- ✅ 29 working examples in main repository
- ✅ 54 provider-specific examples
- ✅ Comprehensive documentation

### 🚧 Future Enhancements

- Additional storage providers (MongoDB, PostgreSQL)
- Performance optimizations
- Additional streaming transforms
- Batch processing utilities
- Rate limiting and token management
- Caching strategies

## Contributing

Contributions are welcome! Please see the [Contributing Guide](CONTRIBUTING.md) for:
- Code style guidelines
- Pull request process
- Testing requirements
- Documentation standards

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

<sub>Inspired by [Vercel's AI SDK](https://github.com/vercel/ai)</sub>
