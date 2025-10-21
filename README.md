# AI SDK Rust

A unified Rust SDK for AI model providers.

## Installation
```toml
[dependencies]
ai-sdk-core = "0.1"
ai-sdk-openai = "0.1"
```

## Quick Start
```rust
use ai_sdk_core::generate_text;
use ai_sdk_openai::OpenAI;

#[tokio::main]
async fn main() -> Result<()> {
    let openai = OpenAI::new(env::var("OPENAI_API_KEY")?);
    let result = generate_text(/* options */).await?;
    println!("{}", result.text);
    Ok(())
}
```
