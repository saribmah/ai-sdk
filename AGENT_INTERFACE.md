# Agent Interface

The Agent interface provides a unified way to create AI agents that can generate or stream outputs from prompts and messages. This document explains how to implement and use the `AgentInterface` trait.

## Overview

An Agent receives a prompt (text or messages) and generates or streams an output that consists of steps, tool calls, data parts, etc. You can implement your own agent by implementing the `AgentInterface` trait.

## Key Components

### 1. AgentInterface Trait

The `AgentInterface` trait defines the interface for all agents:

```rust
pub trait AgentInterface: Send + Sync {
    /// The type for call options (provider-specific settings).
    type CallOptions: Send + Sync;

    /// The output type for this agent.
    type Output: Send + Sync;

    /// The specification version of the agent interface.
    fn version(&self) -> &'static str {
        "agent-v1"
    }

    /// The id of the agent (optional).
    fn id(&self) -> Option<&str>;

    /// The tools that the agent can use.
    fn tools(&self) -> &ToolSet;

    /// Generates an output from the agent (non-streaming).
    async fn generate(
        &self,
        params: AgentCallParameters<Self::CallOptions>,
    ) -> Result<GenerateTextResult, AISDKError>;

    /// Streams an output from the agent (streaming).
    async fn stream(
        &self,
        params: AgentCallParameters<Self::CallOptions>,
    ) -> Result<StreamTextResult, AISDKError>;
}
```

### 2. AgentCallParameters

The `AgentCallParameters` struct ensures that you provide either a `prompt` or `messages`, but not both:

```rust
pub struct AgentCallParameters<CallOptions = ()> {
    /// A prompt - either text or messages. Mutually exclusive with `messages`.
    pub prompt: Option<AgentPrompt>,

    /// A list of messages. Mutually exclusive with `prompt`.
    pub messages: Option<Vec<Message>>,

    /// Optional provider-specific call options.
    pub options: Option<CallOptions>,
}
```

#### Creating AgentCallParameters

```rust
// Using a text prompt
let params = AgentCallParameters::with_prompt("What is the weather in San Francisco?");

// Using messages
let messages = vec![
    Message::User(UserMessage::new("Hello!")),
    Message::Assistant(AssistantMessage::new("Hi! How can I help?")),
    Message::User(UserMessage::new("What's the weather?")),
];
let params = AgentCallParameters::with_messages(messages);

// With call options
let params = AgentCallParameters::with_prompt("What is the weather?")
    .with_options(MyCallOptions { temperature: 0.7 });
```

### 3. AgentPrompt

The `AgentPrompt` enum represents the content of a prompt:

```rust
pub enum AgentPrompt {
    /// A simple text prompt
    Text(String),
    /// A list of messages
    Messages(Vec<Message>),
}
```

## Implementing an Agent

Here's an example of implementing a custom agent:

```rust
use ai_sdk_core::{
    Agent, AgentCallParameters, GenerateTextResult, StreamTextResult,
    AISDKError, ToolSet, Output,
};

struct MyAgent {
    id: String,
    tools: ToolSet,
    model: Arc<dyn LanguageModel>,
}

impl MyAgent {
    fn new(id: String, model: Arc<dyn LanguageModel>) -> Self {
        Self {
            id,
            tools: ToolSet::new(),
            model,
        }
    }

    fn with_tools(mut self, tools: ToolSet) -> Self {
        self.tools = tools;
        self
    }
}

impl AgentInterface for MyAgent {
    type CallOptions = ();
    type Output = Output;

    fn id(&self) -> Option<&str> {
        Some(&self.id)
    }

    fn tools(&self) -> &ToolSet {
        &self.tools
    }

    async fn generate(
        &self,
        params: AgentCallParameters<Self::CallOptions>,
    ) -> Result<GenerateTextResult, AISDKError> {
        // Validate parameters
        params.validate()
            .map_err(|e| AISDKError::invalid_argument(e))?;

        // Convert params to Prompt
        let prompt = match (params.prompt, params.messages) {
            (Some(AgentPrompt::Text(text)), None) => Prompt::text(text),
            (Some(AgentPrompt::Messages(msgs)), None) => Prompt::messages(msgs),
            (None, Some(msgs)) => Prompt::messages(msgs),
            _ => return Err(AISDKError::invalid_argument(
                "Invalid prompt configuration"
            )),
        };

        // Use GenerateText to generate output
        GenerateText::new(self.model.clone(), prompt)
            .tools(self.tools.clone())
            .execute()
            .await
    }

    async fn stream(
        &self,
        params: AgentCallParameters<Self::CallOptions>,
    ) -> Result<StreamTextResult, AISDKError> {
        // Validate parameters
        params.validate()
            .map_err(|e| AISDKError::invalid_argument(e))?;

        // Convert params to Prompt
        let prompt = match (params.prompt, params.messages) {
            (Some(AgentPrompt::Text(text)), None) => Prompt::text(text),
            (Some(AgentPrompt::Messages(msgs)), None) => Prompt::messages(msgs),
            (None, Some(msgs)) => Prompt::messages(msgs),
            _ => return Err(AISDKError::invalid_argument(
                "Invalid prompt configuration"
            )),
        };

        // Use StreamText to stream output
        StreamText::new(self.model.clone(), prompt)
            .tools(self.tools.clone())
            .execute()
            .await
    }
}
```

## Using an Agent

### Non-Streaming Generation

```rust
use ai_sdk_core::{AgentInterface, AgentCallParameters};

async fn example_generate(agent: &impl AgentInterfaceInterface<CallOptions = ()>) -> Result<(), AISDKError> {
    // Create parameters
    let params = AgentCallParameters::with_prompt("What is the capital of France?");

    // Generate output
    let result = agent.generate(params).await?;

    // Access the generated text
    println!("Generated text: {}", result.text);
    println!("Tokens used: {:?}", result.usage);

    Ok(())
}
```

### Streaming Generation

```rust
use ai_sdk_core::{AgentInterface, AgentCallParameters};
use futures_util::StreamExt;

async fn example_stream(agent: &impl AgentInterfaceInterface<CallOptions = ()>) -> Result<(), AISDKError> {
    // Create parameters
    let params = AgentCallParameters::with_prompt("Tell me a short story");

    // Stream output
    let result = agent.stream(params).await?;

    // Stream text deltas
    let mut stream = result.text_stream();
    while let Some(delta) = stream.next().await {
        print!("{}", delta);
    }

    Ok(())
}
```

### Using Messages

```rust
use ai_sdk_core::{AgentInterface, AgentCallParameters};
use ai_sdk_core::prompt::message::{Message, UserMessage};

async fn example_with_messages(agent: &impl AgentInterfaceInterface<CallOptions = ()>) -> Result<(), AISDKError> {
    // Create conversation messages
    let messages = vec![
        Message::User(UserMessage::new("What's the weather like?")),
    ];

    // Create parameters with messages
    let params = AgentCallParameters::with_messages(messages);

    // Generate output
    let result = agent.generate(params).await?;

    println!("Response: {}", result.text);

    Ok(())
}
```

## Type Parameters

### CallOptions

The `CallOptions` type parameter allows you to specify provider-specific options:

```rust
#[derive(Clone)]
struct MyCallOptions {
    temperature: f64,
    max_tokens: usize,
}

struct MyAgent {
    // ...
}

impl AgentInterface for MyAgent {
    type CallOptions = MyCallOptions;
    type Output = Output;

    // ...
}

// Usage
let params = AgentCallParameters::with_prompt("Hello")
    .with_options(MyCallOptions {
        temperature: 0.7,
        max_tokens: 100,
    });
```

If you don't need call options, use `()`:

```rust
impl AgentInterface for MyAgent {
    type CallOptions = ();
    type Output = Output;

    // ...
}
```

### Output

The `Output` type parameter specifies the output type for the agent. By default, use the `Output` enum from `ai_sdk_core::output`.

## Comparison with TypeScript

The Rust implementation closely follows the TypeScript interface:

| TypeScript | Rust |
|------------|------|
| `version: 'agent-v1'` | `fn version(&self) -> &'static str` |
| `id: string \| undefined` | `fn id(&self) -> Option<&str>` |
| `tools: TOOLS` | `fn tools(&self) -> &ToolSet` |
| `generate(options)` | `async fn generate(params)` |
| `stream(options)` | `async fn stream(params)` |

Key differences:
- Rust uses associated types (`CallOptions`, `Output`) instead of generic type parameters
- Rust uses `async fn` instead of `PromiseLike`
- Rust enforces `Send + Sync` bounds for thread safety
- Rust uses `Result` for error handling instead of throwing exceptions

## Best Practices

1. **Always validate parameters**: Call `params.validate()` at the beginning of your `generate` and `stream` methods.

2. **Use proper error handling**: Return descriptive errors using `AISDKError`.

3. **Implement both methods**: Even if you only need one, implement both `generate` and `stream` for consistency.

4. **Tool management**: Store tools in the agent struct and expose them via the `tools()` method.

5. **Thread safety**: Ensure your agent implementation is `Send + Sync` to work with async Rust.

## Future: ToolLoopAgent

A `ToolLoopAgent` implementation will be provided that handles tool execution loops automatically, similar to the TypeScript version.

## Versioning

The agent interface uses a `version()` method that returns `"agent-v1"`. This allows for future evolution of the interface while maintaining backwards compatibility.
