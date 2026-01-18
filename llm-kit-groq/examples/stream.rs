use futures_util::StreamExt;
/// Streaming chat example using Groq provider with only llm-kit-provider.
///
/// This example demonstrates:
/// - Using LanguageModel::do_stream() directly (no llm-kit-core)
/// - Processing stream parts and displaying output
/// - Real-time streaming text generation
///
/// Run with:
/// ```bash
/// export GROQ_API_KEY="your-api-key"
/// cargo run --example stream -p llm-kit-groq
/// ```
use llm_kit_groq::GroqClient;
use llm_kit_provider::language_model::call_options::LanguageModelCallOptions;
use llm_kit_provider::language_model::prompt::LanguageModelMessage;
use llm_kit_provider::language_model::stream_part::LanguageModelStreamPart;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌊 Groq Streaming Chat Example (Provider-Only)\n");

    // Get API key from environment
    let api_key = std::env::var("GROQ_API_KEY").map_err(
        |_| "GROQ_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create Groq provider using client builder
    let provider = GroqClient::new().api_key(api_key).build();

    // Create a language model
    let model = provider.chat_model("llama-3.1-8b-instant");

    println!("✓ Model loaded: {}\n", model.model_id());

    // Example 1: Basic streaming
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 1: Basic Streaming");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt1 = vec![LanguageModelMessage::user_text(
        "Tell me a short story about a robot learning to code.",
    )];

    let options1 = LanguageModelCallOptions::new(prompt1).with_max_output_tokens(300);

    println!("💬 User: Tell me a short story about a robot learning to code.\n");
    println!("🤖 Assistant (streaming): ");

    let result1 = model.do_stream(options1).await?;
    let mut stream1 = result1.stream;

    let mut full_text = String::new();
    let mut final_usage = None;

    while let Some(part) = stream1.next().await {
        match part {
            LanguageModelStreamPart::TextDelta(text_delta) => {
                print!("{}", text_delta.delta);
                full_text.push_str(&text_delta.delta);
                std::io::Write::flush(&mut std::io::stdout()).ok();
            }
            LanguageModelStreamPart::Finish(finish) => {
                final_usage = Some(finish.usage);
            }
            _ => {}
        }
    }

    if let Some(usage) = final_usage {
        println!(
            "\n\n📊 Usage: {} input tokens, {} output tokens",
            usage.input_tokens, usage.output_tokens
        );
    }
    println!("📝 Total characters: {}\n", full_text.len());

    // Example 2: Streaming with temperature
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: Creative Streaming");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt2 = vec![LanguageModelMessage::user_text(
        "Write a creative poem about artificial intelligence.",
    )];

    let options2 = LanguageModelCallOptions::new(prompt2)
        .with_temperature(0.9)
        .with_max_output_tokens(400);

    println!("💬 User: Write a creative poem about artificial intelligence.\n");
    println!("🤖 Assistant (streaming): ");

    let result2 = model.do_stream(options2).await?;
    let mut stream2 = result2.stream;

    let mut chunk_count = 0;
    let mut final_usage2 = None;

    while let Some(part) = stream2.next().await {
        match part {
            LanguageModelStreamPart::TextDelta(text_delta) => {
                print!("{}", text_delta.delta);
                chunk_count += 1;
                std::io::Write::flush(&mut std::io::stdout()).ok();
            }
            LanguageModelStreamPart::Finish(finish) => {
                final_usage2 = Some(finish.usage);
            }
            _ => {}
        }
    }

    if let Some(usage) = final_usage2 {
        println!(
            "\n\n📊 Usage: {} input tokens, {} output tokens",
            usage.input_tokens, usage.output_tokens
        );
    }
    println!("📦 Total chunks received: {}\n", chunk_count);

    // Example 3: Streaming longer content
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 3: Streaming Explanation");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt3 = vec![LanguageModelMessage::user_text(
        "Explain how neural networks work in simple terms (3-4 sentences).",
    )];

    let options3 = LanguageModelCallOptions::new(prompt3).with_max_output_tokens(500);

    println!("💬 User: Explain how neural networks work in simple terms.\n");
    println!("🤖 Assistant (streaming): ");

    let result3 = model.do_stream(options3).await?;
    let mut stream3 = result3.stream;

    let mut text_buffer = String::new();
    let mut first_chunk_time = None::<std::time::Instant>;
    let mut last_chunk_time = std::time::Instant::now();
    let mut final_usage3 = None;

    while let Some(part) = stream3.next().await {
        match part {
            LanguageModelStreamPart::TextDelta(text_delta) => {
                if first_chunk_time.is_none() {
                    first_chunk_time = Some(std::time::Instant::now());
                }
                print!("{}", text_delta.delta);
                text_buffer.push_str(&text_delta.delta);
                last_chunk_time = std::time::Instant::now();
                std::io::Write::flush(&mut std::io::stdout()).ok();
            }
            LanguageModelStreamPart::Finish(finish) => {
                final_usage3 = Some(finish.usage);
            }
            _ => {}
        }
    }

    if let Some(usage) = final_usage3 {
        println!(
            "\n\n📊 Usage: {} input tokens, {} output tokens",
            usage.input_tokens, usage.output_tokens
        );
    }

    if let Some(first_time) = first_chunk_time {
        let total_duration = last_chunk_time.duration_since(first_time);
        println!(
            "⏱️  Streaming duration: {:.2}s",
            total_duration.as_secs_f64()
        );
    }

    println!("\n✅ All examples completed successfully!");
    println!("\n💡 Key Features Demonstrated:");
    println!("   ✓ Using do_stream() directly (provider-only)");
    println!("   ✓ Real-time streaming text generation");
    println!("   ✓ Processing stream chunks");
    println!("   ✓ Temperature control");
    println!("   ✓ Usage tracking");
    println!("   ✓ Streaming performance metrics");

    Ok(())
}
