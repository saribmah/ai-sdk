/// Stream transformations example demonstrating various stream processing capabilities.
///
/// This example shows how to:
/// - Apply filter transforms to select specific stream parts
/// - Use map transforms to modify stream content
/// - Apply throttling for rate limiting
/// - Batch text deltas for reduced overhead
/// - Chain multiple transforms together
///
/// Run with:
/// ```bash
/// export OPENAI_API_KEY="your-api-key"
/// cargo run --example stream_transforms
/// ```
use ai_sdk_core::prompt::{Prompt, call_settings::CallSettings};
use ai_sdk_core::stream_text::{
    self, TextStreamPart, batch_text_transform, filter_transform, map_transform, throttle_transform,
};
use ai_sdk_openai_compatible::OpenAICompatibleClient;
use futures_util::StreamExt;
use std::env;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AI SDK Rust - Stream Transformations Example\n");

    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(
        |_| "OPENAI_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create OpenAI provider using the client builder
    let provider = OpenAICompatibleClient::new()
        .base_url("https://openrouter.ai/api/v1")
        .api_key(api_key)
        .build();

    println!("✓ Provider created: {}", provider.name());
    println!("✓ Base URL: {}\n", provider.base_url());

    // Get a language model and wrap in Arc
    let model: Arc<dyn ai_sdk_provider::language_model::LanguageModel> =
        Arc::from(provider.chat_model("gpt-4o-mini"));
    println!("✓ Model loaded: {}", model.model_id());
    println!("✓ Provider: {}\n", model.provider());

    // Example 1: Filter transform - only show text deltas
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📋 Example 1: Filter Transform (text deltas only)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt = Prompt::text("Say 'Hello from Rust!'");
    let settings = CallSettings::default().with_temperature(0.7);

    // Create a filter transform that only passes text deltas
    let text_filter = filter_transform(|part| matches!(part, TextStreamPart::TextDelta { .. }));

    let result = stream_text::stream_text(
        Arc::clone(&model),
        prompt,
        settings.clone(),
        None,                              // tools
        None,                              // tool_choice
        None,                              // stop_when
        None,                              // provider_options
        None,                              // prepare_step
        false,                             // include_raw_chunks
        Some(vec![Box::new(text_filter)]), // transforms
        None,                              // on_chunk
        None,                              // on_error
        None,                              // on_step_finish
        None,                              // on_finish
    )
    .await?;

    print!("📝 Output: ");
    let mut text_stream = result.text_stream();
    while let Some(text_delta) = text_stream.next().await {
        print!("{}", text_delta);
        std::io::Write::flush(&mut std::io::stdout())?;
    }
    println!("\n");

    // Example 2: Map transform - convert text to uppercase
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📋 Example 2: Map Transform (uppercase text)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt = Prompt::text("Say 'hello world' in lowercase");

    // Create a map transform that uppercases text deltas
    let uppercase_mapper = map_transform(|part| match part {
        TextStreamPart::TextDelta {
            id,
            text,
            provider_metadata,
        } => TextStreamPart::TextDelta {
            id,
            text: text.to_uppercase(),
            provider_metadata,
        },
        other => other,
    });

    let result = stream_text::stream_text(
        Arc::clone(&model),
        prompt,
        settings.clone(),
        None,
        None,
        None,
        None,
        None,
        false,
        Some(vec![Box::new(uppercase_mapper)]),
        None,
        None,
        None,
        None,
    )
    .await?;

    print!("📝 Output: ");
    let mut text_stream = result.text_stream();
    while let Some(text_delta) = text_stream.next().await {
        print!("{}", text_delta);
        std::io::Write::flush(&mut std::io::stdout())?;
    }
    println!("\n");

    // Example 3: Batch transform - reduce number of updates
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📋 Example 3: Batch Transform (combine deltas)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt = Prompt::text("Count from 1 to 10");

    // Create a batch transform that combines up to 50 characters or 100ms delay
    let batcher = batch_text_transform(50, Duration::from_millis(100));

    let result = stream_text::stream_text(
        Arc::clone(&model),
        prompt,
        settings.clone(),
        None,
        None,
        None,
        None,
        None,
        false,
        Some(vec![Box::new(batcher)]),
        None,
        None,
        None,
        None,
    )
    .await?;

    print!("📝 Output (batched): ");
    let mut text_stream = result.text_stream();
    let mut chunk_count = 0;
    while let Some(text_delta) = text_stream.next().await {
        print!("{}", text_delta);
        chunk_count += 1;
        std::io::Write::flush(&mut std::io::stdout())?;
    }
    println!(
        "\n  ℹ️  Received {} chunks (would be more without batching)\n",
        chunk_count
    );

    // Example 4: Throttle transform - slow down output
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📋 Example 4: Throttle Transform (rate limited)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt = Prompt::text("Say 'Hello, World!'");

    // Create a throttle transform with 50ms delay between chunks
    let throttler = throttle_transform(Duration::from_millis(50));

    let result = stream_text::stream_text(
        Arc::clone(&model),
        prompt,
        settings.clone(),
        None,
        None,
        None,
        None,
        None,
        false,
        Some(vec![Box::new(throttler)]),
        None,
        None,
        None,
        None,
    )
    .await?;

    print!("📝 Output (throttled): ");
    let start = std::time::Instant::now();
    let mut text_stream = result.text_stream();
    while let Some(text_delta) = text_stream.next().await {
        print!("{}", text_delta);
        std::io::Write::flush(&mut std::io::stdout())?;
    }
    let elapsed = start.elapsed();
    println!(
        "\n  ℹ️  Streaming took {:?} (delayed by throttling)\n",
        elapsed
    );

    // Example 5: Chained transforms
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📋 Example 5: Chained Transforms (filter + map + batch)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt = Prompt::text("Write a haiku about programming");

    // Chain multiple transforms together
    let text_filter = filter_transform(|part| matches!(part, TextStreamPart::TextDelta { .. }));
    let replacer = map_transform(|part| match part {
        TextStreamPart::TextDelta {
            id,
            text,
            provider_metadata,
        } => TextStreamPart::TextDelta {
            id,
            text: text.replace("code", "🔧").replace("program", "💻"),
            provider_metadata,
        },
        other => other,
    });
    let batcher = batch_text_transform(30, Duration::from_millis(50));

    let result = stream_text::stream_text(
        Arc::from(model),
        prompt,
        settings,
        None,
        None,
        None,
        None,
        None,
        false,
        Some(vec![
            Box::new(text_filter),
            Box::new(replacer),
            Box::new(batcher),
        ]),
        None,
        None,
        None,
        None,
    )
    .await?;

    print!("📝 Output (filtered + transformed + batched): ");
    let mut text_stream = result.text_stream();
    while let Some(text_delta) = text_stream.next().await {
        print!("{}", text_delta);
        std::io::Write::flush(&mut std::io::stdout())?;
    }
    println!("\n");

    println!("✅ All examples completed successfully!");

    Ok(())
}
