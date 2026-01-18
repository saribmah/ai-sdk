# LLM Kit Core

Core functionality for [LLM Kit](https://github.com/saribmah/llm-kit) - unified interface for building AI-powered applications with multiple model providers.

## Features

- **Text Generation**: Generate text using various language models with comprehensive configuration options
- **Streaming**: Stream responses in real-time with callbacks and transforms
- **Tool Calling**: Dynamic and type-safe tool integration for function calling
- **Multi-step Execution**: Automatic tool execution with multiple reasoning steps
- **Agent System**: Reusable AI agents with persistent configuration
- **Embeddings**: Single and batch text embedding generation
- **Image Generation**: Generate images from text prompts
- **Speech Generation**: Convert text to speech with various voices
- **Transcription**: Convert audio to text
- **Reranking**: Rerank documents based on relevance to a query
- **Storage Integration**: Persistent conversation history with automatic loading and saving

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm-kit-core = "0.1"
llm-kit-provider = "0.1"
tokio = { version = "1", features = ["full"] }
```

You'll also need a provider implementation. Choose one based on your needs:

```toml
# For OpenAI-compatible APIs (OpenAI, OpenRouter, etc.)
llm-kit-openai-compatible = "0.1"

# For specific providers
llm-kit-openai = "0.1"
llm-kit-anthropic = "0.1"
llm-kit-azure = "0.1"
llm-kit-deepseek = "0.1"
llm-kit-groq = "0.1"
# ... and more
```

For storage functionality:

```toml
[dependencies]
llm-kit-core = { version = "0.1", features = ["storage"] }
llm-kit-storage = "0.1"
llm-kit-storage-filesystem = "0.1"  # Or another storage provider
```

## Quick Start

### Text Generation

```rust
use llm_kit_core::{GenerateText, prompt::Prompt};
use llm_kit_openai_compatible::OpenAICompatibleClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a provider
    let provider = OpenAICompatibleClient::new()
        .api_key("your-api-key")
        .build();
    
    // Get a language model
    let model = provider.chat_model("gpt-4o-mini");
    
    // Generate text
    let result = GenerateText::new(model, Prompt::text("What is the capital of France?"))
        .temperature(0.7)
        .max_output_tokens(100)
        .execute()
        .await?;
    
    println!("Response: {}", result.text);
    Ok(())
}
```

### Text Streaming

```rust
use llm_kit_core::{StreamText, prompt::Prompt};
use llm_kit_openai_compatible::OpenAICompatibleClient;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = OpenAICompatibleClient::new()
        .api_key("your-api-key")
        .build();
    
    let model = provider.chat_model("gpt-4o-mini");
    
    let result = StreamText::new(model, Prompt::text("Write a poem"))
        .temperature(0.8)
        .execute()
        .await?;
    
    let mut stream = result.text_stream();
    while let Some(text) = stream.next().await {
        print!("{}", text);
    }
    
    Ok(())
}
```

### Tool Calling

```rust
use llm_kit_core::{GenerateText, prompt::Prompt, ToolSet};
use llm_kit_provider_utils::tool::{Tool, ToolExecutionOutput};
use serde_json::json;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... create provider and model ...
    
    // Define a tool
    let weather_tool = Tool::function(json!({
        "type": "object",
        "properties": {
            "city": {"type": "string", "description": "City name"}
        },
        "required": ["city"]
    }))
    .with_description("Get the current weather for a location")
    .with_execute(Arc::new(|input, _opts| {
        ToolExecutionOutput::Single(Box::pin(async move {
            Ok(json!({"temperature": 72, "conditions": "Sunny"}))
        }))
    }));
    
    let mut tools = ToolSet::new();
    tools.insert("get_weather".to_string(), weather_tool);
    
    let result = GenerateText::new(model, Prompt::text("What's the weather in Paris?"))
        .tools(tools)
        .execute()
        .await?;
    
    println!("Response: {}", result.text);
    Ok(())
}
```

### Agent Pattern

```rust
use llm_kit_core::{Agent, AgentSettings, AgentCallParameters};
use llm_kit_core::agent::AgentInterface;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... create provider and model ...
    
    // Configure agent with persistent settings
    let settings = AgentSettings::new(model)
        .with_tools(tools)
        .with_temperature(0.7)
        .with_max_output_tokens(500);
    
    let agent = Agent::new(settings);
    
    // Use agent multiple times with consistent settings
    let result1 = agent.generate(AgentCallParameters::from_text("Hello"))?
        .execute()
        .await?;
    
    let result2 = agent.generate(AgentCallParameters::from_text("Follow-up question"))?
        .execute()
        .await?;
    
    Ok(())
}
```

### Embeddings

```rust
use llm_kit_core::{Embed, EmbedMany};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... create provider ...
    
    let embedding_model = provider.text_embedding_model("text-embedding-3-small")?;
    
    // Single embedding
    let result = Embed::new(embedding_model.clone(), "Hello world".to_string())
        .execute()
        .await?;
    
    println!("Embedding dimension: {}", result.embedding.len());
    
    // Batch embeddings
    let texts = vec!["text1".to_string(), "text2".to_string()];
    let results = EmbedMany::new(embedding_model, texts)
        .max_parallel_calls(5)
        .execute()
        .await?;
    
    println!("Generated {} embeddings", results.embeddings.len());
    Ok(())
}
```

## Core Builder APIs

All builder APIs follow a consistent pattern:

### GenerateText

Generate text with optional tool calling and multi-step execution.

```rust
let result = GenerateText::new(model, prompt)
    .temperature(0.7)
    .max_output_tokens(500)
    .tools(tool_set)
    .top_p(0.9)
    .top_k(50)
    .frequency_penalty(0.5)
    .presence_penalty(0.5)
    .seed(42)
    .max_retries(3)
    .execute()
    .await?;
```

**Key methods:**
- `.temperature(f32)` - Sampling temperature (0.0-2.0)
- `.max_output_tokens(u32)` - Maximum tokens to generate
- `.tools(ToolSet)` - Enable tool calling
- `.top_p(f32)` - Nucleus sampling threshold
- `.top_k(u32)` - Top-k sampling limit
- `.frequency_penalty(f32)` - Reduce repetition
- `.presence_penalty(f32)` - Encourage diversity
- `.seed(u32)` - Deterministic generation
- `.max_retries(u32)` - Retry on transient failures
- `.execute()` - Execute the request

### StreamText

Stream text generation with callbacks and transforms.

```rust
let result = StreamText::new(model, prompt)
    .temperature(0.8)
    .on_chunk(|chunk| {
        println!("Chunk: {:?}", chunk);
    })
    .on_finish(|event| {
        println!("Finished: {} tokens", event.usage.total_tokens);
    })
    .execute()
    .await?;

let mut stream = result.text_stream();
while let Some(text) = stream.next().await {
    print!("{}", text);
}
```

**Key methods:**
- `.on_chunk(callback)` - Called for each stream chunk
- `.on_finish(callback)` - Called when streaming completes
- `.on_step_finish(callback)` - Called after each tool execution step
- `.text_stream()` - Get stream of text chunks
- `.full_stream()` - Get stream of all event types

### Embed & EmbedMany

Generate embeddings for single or multiple texts.

```rust
// Single embedding
let result = Embed::new(embedding_model, "text".to_string())
    .max_retries(3)
    .execute()
    .await?;

// Batch embeddings
let results = EmbedMany::new(embedding_model, vec!["text1".to_string()])
    .max_retries(3)
    .max_parallel_calls(5)
    .execute()
    .await?;
```

### GenerateImage

Generate images from text prompts.

```rust
let result = GenerateImage::new(image_model, "A serene landscape".to_string())
    .n(2)  // Generate 2 images
    .size("1024x1024")
    .seed(42)
    .execute()
    .await?;
```

### GenerateSpeech

Convert text to speech.

```rust
let result = GenerateSpeech::new(speech_model, "Hello, world!".to_string())
    .voice("alloy")
    .output_format("mp3")
    .speed(1.0)
    .execute()
    .await?;
```

### Transcribe

Convert audio to text.

```rust
use llm_kit_core::AudioInput;

let result = Transcribe::new(transcription_model, AudioInput::Data(audio_data))
    .max_retries(3)
    .execute()
    .await?;
```

### Rerank

Rerank documents based on relevance to a query.

```rust
let documents = vec!["doc1".to_string(), "doc2".to_string()];
let result = Rerank::new(reranking_model, documents, "search query".to_string())
    .top_n(5)
    .max_retries(3)
    .execute()
    .await?;
```

## Agent System

The Agent pattern provides reusable AI agents with persistent configuration:

```rust
// Create agent with persistent settings
let settings = AgentSettings::new(model)
    .with_tools(tools)
    .with_temperature(0.7)
    .with_max_output_tokens(500);

let agent = Agent::new(settings);

// Generate with agent (tools come from settings)
let result = agent.generate(AgentCallParameters::from_text("Hello"))?
    .temperature(0.9)  // Override settings per call
    .execute()
    .await?;

// Stream with agent
let result = agent.stream(AgentCallParameters::from_text("Hello"))?
    .on_chunk(|chunk| println!("{:?}", chunk))
    .execute()
    .await?;
```

**Key features:**
- Tools configured once in `AgentSettings` (cloneable via `Arc`)
- Returns builders for per-call customization
- Supports both `generate()` and `stream()` operations
- Storage can be configured for persistent conversations

## Storage Integration

Enable persistent conversation history with automatic loading and saving:

```rust
use llm_kit_storage_filesystem::FilesystemStorage;
use llm_kit_storage::Storage;
use std::sync::Arc;

let storage: Arc<dyn Storage> = Arc::new(FilesystemStorage::new("./storage")?);
storage.initialize().await?;
let session_id = storage.generate_session_id();

// First message - no history
let result = GenerateText::new(model, Prompt::text("Hello"))
    .with_storage(storage.clone())
    .with_session_id(session_id.clone())
    .without_history()  // Important for first message!
    .execute()
    .await?;

// Subsequent messages - history loaded automatically
let result = GenerateText::new(model, Prompt::text("Follow-up"))
    .with_storage(storage)
    .with_session_id(session_id)
    .execute()
    .await?;  // Previous messages automatically included
```

**Storage with Agents:**

```rust
// Stateful mode (session configured in settings)
let settings = AgentSettings::new(model)
    .with_storage(storage.clone())
    .with_session_id(session_id);  // Fixed session

let agent = Agent::new(settings);

let result1 = agent.generate(AgentCallParameters::from_text("Hello"))?
    .without_history()  // First message
    .execute().await?;

let result2 = agent.generate(AgentCallParameters::from_text("Follow-up"))?
    .execute().await?;  // History loaded automatically
```

**Storage providers:**
- `llm-kit-storage-filesystem` - Filesystem-based storage with JSON files
- More storage providers coming soon (MongoDB, PostgreSQL, etc.)

**Enable with feature flag:**
```toml
llm-kit-core = { version = "0.1", features = ["storage"] }
```

## Tool System

The SDK supports both dynamic and type-safe tools:

### Dynamic Tools

```rust
use llm_kit_provider_utils::tool::{Tool, ToolExecutionOutput};
use serde_json::json;
use std::sync::Arc;

let tool = Tool::function(json!({
    "type": "object",
    "properties": {
        "city": {"type": "string"}
    },
    "required": ["city"]
}))
.with_description("Get weather for a city")
.with_execute(Arc::new(|input, _opts| {
    ToolExecutionOutput::Single(Box::pin(async move {
        Ok(json!({"temperature": 72}))
    }))
}));
```

### Type-Safe Tools

```rust
use llm_kit_core::tool::TypeSafeTool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, JsonSchema)]
struct WeatherInput {
    city: String,
}

#[derive(Serialize)]
struct WeatherOutput {
    temperature: i32,
}

impl TypeSafeTool for WeatherInput {
    type Output = WeatherOutput;
    
    fn description() -> String {
        "Get weather for a city".to_string()
    }
    
    async fn execute(self) -> Result<Self::Output, String> {
        Ok(WeatherOutput { temperature: 72 })
    }
}
```

### Multi-Step Tool Execution

The SDK automatically executes tools and feeds results back to the model:

```rust
use llm_kit_core::{GenerateText, has_tool_call};

let result = GenerateText::new(model, prompt)
    .tools(tools)
    .execute()
    .await?;

// Check if tools were called
if has_tool_call(&result) {
    println!("Tools were executed!");
}

// Inspect execution steps
for step in &result.steps {
    println!("Step {}: {} tool calls", step.step, step.tool_calls.len());
}
```

## Message Types and Prompts

Build prompts using various message types:

```rust
use llm_kit_core::prompt::{Prompt, UserMessage, SystemMessage};

// Simple text prompt
let prompt = Prompt::text("Hello");

// With system message
let prompt = Prompt::from_messages(vec![
    SystemMessage::text("You are a helpful assistant").into(),
    UserMessage::text("Hello").into(),
]);

// Multi-modal with images
use llm_kit_core::prompt::ImagePart;

let image = ImagePart::from_base64("data:image/png;base64,...");
let prompt = Prompt::from_user_message(
    UserMessage::from_parts(vec![
        "Describe this image".into(),
        image.into(),
    ])
);
```

## Error Handling

All operations return `Result<T, AISDKError>`:

```rust
use llm_kit_core::error::AISDKError;

match GenerateText::new(model, prompt).execute().await {
    Ok(result) => println!("Success: {}", result.text),
    Err(e) => match e {
        AISDKError::InvalidArgument { message, .. } => {
            eprintln!("Invalid argument: {}", message);
        }
        AISDKError::ModelError { message, .. } => {
            eprintln!("Model error: {}", message);
        }
        AISDKError::RetryableError { message, .. } => {
            eprintln!("Retryable error: {}", message);
        }
        _ => eprintln!("Error: {}", e),
    }
}
```

## Examples

See the `examples/` directory in the main repository for complete examples:

**Basic Examples:**
- `basic_chat.rs` - Simple text generation
- `basic_stream.rs` - Streaming responses
- `basic_embedding.rs` - Text embeddings
- `basic_image.rs` - Image generation
- `conversation.rs` - Multi-turn conversations

**Tool Calling Examples:**
- `tool_calling.rs` - Basic tool calling
- `type_safe_tools.rs` - Type-safe tools
- `multi_step_tools.rs` - Multi-step execution
- `stream_tool_calling.rs` - Streaming with tools

**Agent Examples:**
- `agent_generate.rs` - Agent text generation
- `agent_stream.rs` - Agent streaming
- `agent_storage_conversation.rs` - Agents with persistent conversations

**Storage Examples:**
- `storage_basic.rs` - Basic storage operations
- `storage_filesystem_basic.rs` - Filesystem storage
- `storage_filesystem_conversation.rs` - Multi-turn conversations with storage
- `storage_conversation_full.rs` - Full integration with GenerateText

**Advanced Examples:**
- `partial_output.rs` - Partial output handling
- `stream_transforms.rs` - Stream transformations

Run examples with:

```bash
# Set your API key
export OPENAI_API_KEY="your-api-key"

# Run an example
cargo run --example basic_chat

# Run storage examples (requires storage feature)
cargo run --example storage_basic --features storage
```

## Module Organization

- **`agent`**: Agent system for reusable AI agents with persistent configuration
- **`embed`**: Embedding generation (single and batch operations)
- **`error`**: Error types for the SDK
- **`generate_image`**: Image generation functionality
- **`generate_speech`**: Speech synthesis functionality
- **`generate_text`**: Text generation with tool calling support
- **`output`**: Unified output types (text, reasoning, sources)
- **`prompt`**: Message types and prompt management
- **`rerank`**: Document reranking functionality
- **`storage_conversion`**: Storage conversion utilities (requires `storage` feature)
- **`stream_text`**: Text streaming with callbacks and transforms
- **`tool`**: Tool system for function calling (dynamic and type-safe)
- **`transcribe`**: Audio transcription functionality

## Features

### Default Features

The default installation includes all core functionality except storage.

### Optional Features

**`storage`** - Enable storage functionality for persistent conversations:

```toml
llm-kit-core = { version = "0.1", features = ["storage"] }
```

Enables:
- `storage_conversion` module for converting between core and storage types
- `.with_storage()` and `.with_session_id()` methods on builders
- Automatic conversation history loading and saving

## Architecture

The SDK follows a three-layer architecture:

1. **Builder Layer** (llm-kit-core):
   - Ergonomic builder APIs: `GenerateText`, `StreamText`, `Embed`, etc.
   - Agent pattern for reusable configurations
   - Tool execution and management
   - Prompt standardization and validation

2. **Provider Layer** (llm-kit-provider):
   - `Provider` trait for implementing new providers
   - `LanguageModel`, `EmbeddingModel`, `ImageModel`, etc. traits
   - Standardized types: `CallOptions`, `Content`, `FinishReason`, `Usage`

3. **Implementation Layer** (provider crates):
   - Concrete provider implementations
   - API-specific request/response handling
   - HTTP client management

## Documentation

- [API Documentation](https://docs.rs/llm-kit-core)
- [LLM Kit Repository](https://github.com/saribmah/llm-kit)
- [Provider Implementations](https://github.com/saribmah/llm-kit#providers)
- [Contributing Guide](../CONTRIBUTING.md)

## License

Licensed under:

- MIT license ([LICENSE-MIT](LICENSE-MIT))

## Contributing

Contributions are welcome! Please see the [Contributing Guide](../CONTRIBUTING.md) for more details.
