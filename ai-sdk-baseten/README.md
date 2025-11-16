# AI SDK - Baseten Provider

The **Baseten provider** for the AI SDK contains language model and embedding model support for the [Baseten](https://baseten.co) platform.

## Setup

The Baseten provider is available in the `ai-sdk-baseten` crate. You can add it to your project with:

```toml
[dependencies]
ai-sdk-baseten = "0.1.0"
ai-sdk-core = "0.1.0"
```

## Provider Instance

You can import the provider functions from `ai-sdk-baseten`:

```rust
use ai_sdk_baseten::{BasetenClient, baseten};
```

## Language Model Example (Model APIs)

The simplest way to use Baseten is with the Model APIs, which provide access to hosted models:

```rust
use ai_sdk_baseten::BasetenClient;
use ai_sdk_core::{GenerateText, prompt::Prompt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // API key loaded from BASETEN_API_KEY environment variable
    let provider = BasetenClient::new().build();
    
    let model = provider.chat_model(Some("deepseek-ai/DeepSeek-V3-0324"));
    
    let result = GenerateText::new(model, Prompt::text("What is the meaning of life?"))
        .execute()
        .await?;
    
    println!("{}", result.text);
    Ok(())
}
```

## Custom Model URL

For dedicated model deployments, you can specify a custom model URL:

```rust
use ai_sdk_baseten::BasetenClient;
use ai_sdk_core::{GenerateText, prompt::Prompt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = BasetenClient::new()
        .api_key("your-api-key")
        .model_url("https://model-{id}.api.baseten.co/environments/production/sync/v1")
        .build();
    
    // model_id is optional when using custom URL
    let model = provider.chat_model(None);
    
    let result = GenerateText::new(model, Prompt::text("Hello!"))
        .execute()
        .await?;
    
    println!("{}", result.text);
    Ok(())
}
```

## Text Embeddings

Embeddings require a custom model URL and are not available via Model APIs:

```rust
use ai_sdk_baseten::BasetenClient;
use ai_sdk_core::EmbedMany;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = BasetenClient::new()
        .api_key("your-api-key")
        .model_url("https://model-{id}.api.baseten.co/environments/production/sync")
        .build();
    
    let model = provider.text_embedding_model(None);
    
    let texts = vec![
        "The capital of France is Paris.".to_string(),
        "The capital of Germany is Berlin.".to_string(),
    ];
    
    let result = EmbedMany::new(model, texts).execute().await?;
    
    println!("Embeddings: {} vectors of dimension {}", 
        result.embeddings.len(), 
        result.embeddings[0].len()
    );
    Ok(())
}
```

## Streaming Text Generation

```rust
use ai_sdk_baseten::BasetenClient;
use ai_sdk_core::{StreamText, prompt::Prompt};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = BasetenClient::new().build();
    let model = provider.chat_model(Some("deepseek-ai/DeepSeek-V3-0324"));
    
    let mut result = StreamText::new(model, Prompt::text("Write a haiku."))
        .execute()
        .await?;
    
    while let Some(part) = result.stream.next().await {
        use ai_sdk_core::text_stream::TextStreamPart;
        
        match part {
            TextStreamPart::TextDelta(delta) => print!("{}", delta.text_delta),
            TextStreamPart::Finish(finish) => {
                println!("\nUsage: {:?}", finish.usage);
            }
            _ => {}
        }
    }
    
    Ok(())
}
```

## Supported Models

### Model APIs (Hosted Models)

When using the default Model APIs, the following models are available:

- `deepseek-ai/DeepSeek-R1-0528` - DeepSeek R1 with reasoning
- `deepseek-ai/DeepSeek-V3-0324` - DeepSeek V3
- `deepseek-ai/DeepSeek-V3.1` - DeepSeek V3.1
- `moonshotai/Kimi-K2-Instruct-0905` - Kimi K2
- `Qwen/Qwen3-235B-A22B-Instruct-2507` - Qwen 3
- `Qwen/Qwen3-Coder-480B-A35B-Instruct` - Qwen 3 Coder
- `openai/gpt-oss-120b` - GPT OSS
- `zai-org/GLM-4.6` - GLM 4.6

### Custom Models

For custom model deployments, use the `model_url` option:

- **Chat models**: Must use `/sync/v1` endpoints
- **Embedding models**: Must use `/sync` or `/sync/v1` endpoints

## Environment Variables

- `BASETEN_API_KEY` - Your Baseten API key

## Configuration Options

### `BasetenClient` Builder

- `.api_key(key)` - Set the API key (overrides `BASETEN_API_KEY`)
- `.base_url(url)` - Set the base URL for Model APIs (default: `https://inference.baseten.co/v1`)
- `.model_url(url)` - Set a custom model URL for dedicated deployments
- `.header(key, value)` - Add a custom header
- `.headers(map)` - Add multiple custom headers

## Examples

See the `examples/` directory for complete examples:

- `baseten_basic_chat.rs` - Basic chat with Model APIs
- `baseten_custom_chat.rs` - Chat with custom model URL
- `baseten_embeddings.rs` - Text embeddings
- `baseten_streaming.rs` - Streaming text generation

Run an example with:

```bash
export BASETEN_API_KEY=your-api-key
cargo run --example baseten_basic_chat
```

## Documentation

For more information, visit:

- [Baseten Documentation](https://docs.baseten.co/)
- [Baseten Model APIs](https://docs.baseten.co/development/model-apis/overview)
- [AI SDK Rust Documentation](https://github.com/vercel/ai-sdk-rust)

## License

Apache-2.0
