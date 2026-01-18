# Agent Interface

The Agent interface provides a unified way to create AI agents that can generate or stream outputs from prompts and messages. This document explains the Agent system and how to use it.

## Overview

An Agent is a reusable wrapper around a language model with persistent configuration (tools, temperature, etc.). It receives a prompt (text or messages) and generates or streams output using `GenerateText` or `StreamText` builders.

## Quick Start

```rust
use llm_kit_core::{Agent, AgentSettings, AgentCallParameters};
use llm_kit_core::tool::ToolSet;

// 1. Create and configure tools
let mut tools = ToolSet::new();
tools.insert("weather".to_string(), weather_tool);

// 2. Create agent with persistent settings
let settings = AgentSettings::new(model)
    .with_tools(tools)
    .with_temperature(0.7)
    .with_max_tokens(500);

let agent = Agent::new(settings);

// 3. Use agent multiple times with different prompts
let result = agent.generate(AgentCallParameters::from_text("What's the weather?"))?
    .execute()
    .await?;
```

## Key Components

### 1. AgentSettings

`AgentSettings` stores the persistent configuration for an agent:

```rust
pub struct AgentSettings {
    pub model: Arc<dyn LanguageModel>,
    pub tools: Option<ToolSet>,
    pub temperature: Option<f64>,
    pub max_tokens: Option<u32>,
    pub top_p: Option<f64>,
    pub top_k: Option<u32>,
    pub presence_penalty: Option<f64>,
    pub frequency_penalty: Option<f64>,
    pub seed: Option<u32>,
    pub max_retries: Option<u32>,
}
```

**Builder methods:**

```rust
let settings = AgentSettings::new(model)
    .with_tools(tool_set)
    .with_temperature(0.7)
    .with_max_tokens(500)
    .with_top_p(0.9)
    .with_top_k(50)
    .with_presence_penalty(0.6)
    .with_frequency_penalty(0.5)
    .with_seed(42)
    .with_max_retries(3);
```

### 2. Agent

`Agent` is the main struct that provides `generate()` and `stream()` methods:

```rust
pub struct Agent {
    settings: AgentSettings,
}

impl Agent {
    pub fn new(settings: AgentSettings) -> Self;
    
    pub fn generate(&self, params: AgentCallParameters) 
        -> Result<GenerateText, AISDKError>;
    
    pub fn stream(&self, params: AgentCallParameters) 
        -> Result<StreamText, AISDKError>;
}
```

**Key design decision:** Agent methods return builders (`GenerateText`/`StreamText`), not results. This allows customization before execution:

```rust
// Agent returns a builder
let result = agent.generate(params)?
    .temperature(0.9)  // Override agent's temperature
    .on_finish(callback)  // Add callbacks
    .execute()  // Execute when ready
    .await?;
```

### 3. AgentCallParameters

`AgentCallParameters` specifies the prompt for each agent call:

```rust
pub struct AgentCallParameters {
    pub prompt: PromptContent,
}

impl AgentCallParameters {
    // Create from text
    pub fn from_text(text: impl Into<String>) -> Self;
    
    // Create from messages
    pub fn from_messages(messages: Vec<Message>) -> Self;
}
```

**Usage:**

```rust
// From text
let params = AgentCallParameters::from_text("Hello!");

// From messages
let messages = vec![
    Message::User(UserMessage::new("What's the weather?")),
];
let params = AgentCallParameters::from_messages(messages);
```

## AgentInterface Trait

The `AgentInterface` trait defines the interface that all agents must implement:

```rust
pub trait AgentInterface: Send + Sync {
    fn version(&self) -> &'static str {
        "agent-v1"
    }

    fn id(&self) -> Option<&str>;

    fn tools(&self) -> Option<&ToolSet>;

    fn generate(
        &self,
        params: AgentCallParameters,
    ) -> Result<GenerateText, AISDKError>;

    fn stream(
        &self,
        params: AgentCallParameters,
    ) -> Result<StreamText, AISDKError>;
}
```

## Default Implementation

The default `Agent` implementation is in `llm-kit-core/src/agent/default_impl.rs`:

```rust
impl AgentInterface for Agent {
    fn version(&self) -> &'static str {
        "agent-v1"
    }

    fn id(&self) -> Option<&str> {
        None
    }

    fn tools(&self) -> Option<&ToolSet> {
        self.settings.tools.as_ref()
    }

    fn generate(&self, params: AgentCallParameters) -> Result<GenerateText, AISDKError> {
        let prompt = self.build_prompt(params.prompt);
        let mut builder = GenerateText::new(self.settings.model.clone(), prompt);

        // Apply settings to builder
        if let Some(tools) = &self.settings.tools {
            builder = builder.tools(tools.clone());
        }
        if let Some(temp) = self.settings.temperature {
            builder = builder.temperature(temp);
        }
        // ... (other settings)

        Ok(builder)
    }

    fn stream(&self, params: AgentCallParameters) -> Result<StreamText, AISDKError> {
        // Similar to generate(), but returns StreamText
    }
}
```

## Complete Example

### Non-Streaming Generation

```rust
use llm_kit_core::{Agent, AgentSettings, AgentCallParameters};
use llm_kit_core::tool::{Tool, ToolSet};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider and model
    let provider = OpenAICompatibleClient::new()
        .base_url("https://api.openai.com/v1")
        .api_key(std::env::var("OPENAI_API_KEY")?)
        .build();
    
    let model = provider.chat_model("gpt-4");

    // Create tools
    let weather_tool = Tool::new(
        "get_weather",
        "Get weather for a city",
        /* schema */,
        Arc::new(|args| {
            Box::pin(async move {
                // Weather API call
                Ok("Sunny, 72°F".to_string())
            })
        }),
    );

    let mut tools = ToolSet::new();
    tools.insert("get_weather".to_string(), weather_tool);

    // Create agent with settings
    let settings = AgentSettings::new(model)
        .with_tools(tools)
        .with_temperature(0.7)
        .with_max_tokens(500);

    let agent = Agent::new(settings);

    // Use agent multiple times
    let result1 = agent.generate(
        AgentCallParameters::from_text("What's the weather in SF?")
    )?
        .execute()
        .await?;

    println!("Response: {}", result1.text);

    // Use again with different prompt
    let result2 = agent.generate(
        AgentCallParameters::from_text("How about New York?")
    )?
        .temperature(0.9)  // Override agent temperature
        .execute()
        .await?;

    println!("Response: {}", result2.text);

    Ok(())
}
```

### Streaming Example

```rust
use futures_util::StreamExt;

let result = agent.stream(
    AgentCallParameters::from_text("Tell me a story about AI")
)?
    .on_chunk(Box::new(|event| {
        Box::pin(async move {
            // Process chunks in real-time
        })
    }))
    .execute()
    .await?;

let mut text_stream = result.text_stream();
while let Some(delta) = text_stream.next().await {
    print!("{}", delta);
}
```

### Using Messages

```rust
use llm_kit_core::prompt::message::{Message, UserMessage, SystemMessage};

let messages = vec![
    Message::System(SystemMessage::new("You are a helpful assistant")),
    Message::User(UserMessage::new("What's the capital of France?")),
];

let result = agent.generate(
    AgentCallParameters::from_messages(messages)
)?
    .execute()
    .await?;

println!("Response: {}", result.text);
```

## Design Rationale

### Why Tools in AgentSettings (Not Parameters)?

- **Tools are cloneable** - Using `Arc` for function pointers makes tools cheap to clone
- **Reusable configuration** - Agent is configured once, used many times
- **Single source of truth** - Tools defined in one place
- **Just change the prompt** - Each call only needs a different prompt

### Why Agent Returns Builders (Not Results)?

- **Flexibility** - Users can customize per call (temperature, callbacks, etc.)
- **Ergonomic** - Familiar builder pattern
- **Non-breaking** - Future settings can be added without breaking changes
- **Progressive disclosure** - Simple cases are simple, complex cases are possible

**Example:**

```rust
// Simple case - just execute
let result = agent.generate(params)?.execute().await?;

// Complex case - customize before execution
let result = agent.generate(params)?
    .temperature(0.9)
    .on_finish(callback)
    .stop_sequences(vec!["END".to_string()])
    .execute()
    .await?;
```

### Why No Generic CallOptions?

In earlier versions, `Agent` had a `CallOptions` generic parameter. We removed it because:

- **Unused** - The generic was never actually used after removing `prepare_call`
- **Complexity** - Added unnecessary type parameters and `PhantomData`
- **Builders handle it** - Call-specific options can be set on the returned builder

## Implementing Custom Agents

You can implement `AgentInterface` for your own types:

```rust
struct MyCustomAgent {
    model: Arc<dyn LanguageModel>,
    system_prompt: String,
}

impl AgentInterface for MyCustomAgent {
    fn version(&self) -> &'static str {
        "agent-v1"
    }

    fn id(&self) -> Option<&str> {
        Some("my-custom-agent")
    }

    fn tools(&self) -> Option<&ToolSet> {
        None
    }

    fn generate(&self, params: AgentCallParameters) -> Result<GenerateText, AISDKError> {
        // Custom logic - e.g., inject system prompt
        let messages = match params.prompt {
            PromptContent::Text(text) => vec![
                Message::System(SystemMessage::new(&self.system_prompt)),
                Message::User(UserMessage::new(text)),
            ],
            PromptContent::Messages(mut msgs) => {
                msgs.insert(0, Message::System(SystemMessage::new(&self.system_prompt)));
                msgs
            }
        };

        Ok(GenerateText::new(
            self.model.clone(),
            Prompt::messages(messages),
        ))
    }

    fn stream(&self, params: AgentCallParameters) -> Result<StreamText, AISDKError> {
        // Similar to generate()
    }
}
```

## Architecture Summary

```
┌─────────────────────────────────────────┐
│          AgentSettings                  │
│  - model: Arc<LanguageModel>           │
│  - tools: ToolSet                      │
│  - temperature, max_tokens, etc.       │
└────────────┬────────────────────────────┘
             │
             │ used by
             ▼
┌─────────────────────────────────────────┐
│            Agent                        │
│  - settings: AgentSettings             │
│                                        │
│  Methods:                              │
│  + generate(params) -> GenerateText   │
│  + stream(params) -> StreamText       │
└────────────┬────────────────────────────┘
             │
             │ takes
             ▼
┌─────────────────────────────────────────┐
│       AgentCallParameters               │
│  - prompt: PromptContent               │
│                                        │
│  Constructors:                         │
│  + from_text(text)                     │
│  + from_messages(messages)             │
└─────────────────────────────────────────┘
```

## Examples

See the following example files:

- **`examples/agent_generate.rs`** - Non-streaming generation with reusable agent
- **`examples/agent_stream.rs`** - Streaming with reusable agent

## Best Practices

1. **Create agents once, use many times** - Agents are designed to be reusable
2. **Configure tools in settings** - Tools are cloneable and cheap to share
3. **Use builders for customization** - Override agent settings per call when needed
4. **Keep prompts in parameters** - Only the prompt changes between calls
5. **Leverage type safety** - Use `TypeSafeTool` for compile-time checking

## Versioning

The agent interface uses a `version()` method that returns `"agent-v1"`. This allows for future evolution of the interface while maintaining backwards compatibility.
