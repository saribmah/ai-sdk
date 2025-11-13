use ai_sdk_core::{StreamText, prompt::Prompt};
use ai_sdk_huggingface::{HuggingFaceProviderSettings, LLAMA_3_1_8B_INSTRUCT, create_huggingface};
use futures_util::StreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file (if present)
    dotenv::dotenv().ok();

    // Create the Hugging Face provider
    let provider = create_huggingface(HuggingFaceProviderSettings::new().load_api_key_from_env());

    // Create a model
    let model = provider.responses(LLAMA_3_1_8B_INSTRUCT);

    // Stream text generation
    println!("Streaming response...\n");
    print!("Response: ");

    // Use Arc to share metadata between callback and main thread
    let metadata = Arc::new(Mutex::new(None));
    let metadata_clone = metadata.clone();

    let result = StreamText::new(model, Prompt::text("Write a haiku about programming."))
        .temperature(0.8)
        .on_finish(Box::new(move |event| {
            let metadata = metadata_clone.clone();
            Box::pin(async move {
                let mut meta = metadata.lock().await;
                *meta = Some((
                    event.step_result.finish_reason.clone(),
                    event.step_result.usage,
                ));
            })
        }))
        .execute()
        .await?;

    // Stream text deltas
    let mut text_stream = result.text_stream();
    while let Some(text_delta) = text_stream.next().await {
        print!("{}", text_delta);
        use std::io::Write;
        std::io::stdout().flush()?;
    }

    // Access metadata captured from on_finish callback
    if let Some((finish_reason, usage)) = metadata.lock().await.as_ref() {
        println!("\n\nUsage:");
        println!("  Input tokens:  {}", usage.input_tokens);
        println!("  Output tokens: {}", usage.output_tokens);
        println!("  Total tokens:  {}", usage.total_tokens);

        if usage.reasoning_tokens > 0 {
            println!("  Reasoning tokens: {}", usage.reasoning_tokens);
        }

        if usage.cached_input_tokens > 0 {
            println!("  Cached input tokens: {}", usage.cached_input_tokens);
        }

        println!("\nFinish reason: {:?}", finish_reason);
    }

    Ok(())
}
