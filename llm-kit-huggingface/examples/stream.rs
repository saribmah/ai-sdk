use futures_util::StreamExt;
/// Streaming chat example using Hugging Face provider with only llm-kit-provider.
///
/// This example demonstrates:
/// - Using LanguageModel::do_stream() directly (no llm-kit-core)
/// - Processing streaming responses in real-time
/// - Handling different stream part types
///
/// Run with:
/// ```bash
/// export HUGGINGFACE_API_KEY="your-api-key"
/// cargo run --example stream -p llm-kit-huggingface
/// ```
use llm_kit_huggingface::{HuggingFaceClient, LLAMA_3_1_8B_INSTRUCT};
use llm_kit_provider::language_model::call_options::LanguageModelCallOptions;
use llm_kit_provider::language_model::prompt::LanguageModelMessage;
use llm_kit_provider::language_model::stream_part::LanguageModelStreamPart;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 Hugging Face Streaming Chat Example (Provider-Only)\n");

    // Get API key from environment
    let api_key = std::env::var("HUGGINGFACE_API_KEY").map_err(
        |_| "HUGGINGFACE_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create Hugging Face provider using client builder
    let provider = HuggingFaceClient::new().api_key(api_key).build();

    // Create a language model
    let model = provider.responses(LLAMA_3_1_8B_INSTRUCT);

    println!("✓ Model loaded: {}\n", model.model_id());

    // Example 1: Simple streaming
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 1: Simple Streaming");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt1 = vec![LanguageModelMessage::user_text(
        "Write a haiku about programming.",
    )];

    let options1 = LanguageModelCallOptions::new(prompt1)
        .with_temperature(0.8)
        .with_max_output_tokens(200);

    println!("💬 User: Write a haiku about programming.\n");
    print!("🤖 Assistant: ");

    let result1 = model.do_stream(options1).await?;
    let mut stream1 = result1.stream;

    let mut usage_info = None;
    let mut finish_reason = None;

    while let Some(part) = stream1.next().await {
        match part {
            LanguageModelStreamPart::TextDelta(delta) => {
                print!("{}", delta.delta);
                use std::io::Write;
                std::io::stdout().flush()?;
            }
            LanguageModelStreamPart::Finish(finish) => {
                usage_info = Some(finish.usage);
                finish_reason = Some(finish.finish_reason);
            }
            _ => {}
        }
    }

    if let Some(usage) = usage_info {
        println!(
            "\n\n📊 Usage: {} input tokens, {} output tokens",
            usage.input_tokens, usage.output_tokens
        );
    }
    if let Some(reason) = finish_reason {
        println!("🏁 Finish reason: {:?}\n", reason);
    }

    // Example 2: Streaming with analysis
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: Streaming with Part Analysis");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt2 = vec![LanguageModelMessage::user_text(
        "Explain Rust's ownership system in one paragraph.",
    )];

    let options2 = LanguageModelCallOptions::new(prompt2)
        .with_temperature(0.7)
        .with_max_output_tokens(500);

    println!("💬 User: Explain Rust's ownership system in one paragraph.\n");
    print!("🤖 Assistant: ");

    let result2 = model.do_stream(options2).await?;
    let mut stream2 = result2.stream;

    let mut text_parts = 0;
    let mut metadata_parts = 0;
    let mut other_parts = 0;
    let mut final_usage = None;
    let mut final_reason = None;

    while let Some(part) = stream2.next().await {
        match part {
            LanguageModelStreamPart::TextDelta(delta) => {
                print!("{}", delta.delta);
                use std::io::Write;
                std::io::stdout().flush()?;
                text_parts += 1;
            }
            LanguageModelStreamPart::ResponseMetadata(_) => {
                metadata_parts += 1;
            }
            LanguageModelStreamPart::Finish(finish) => {
                final_usage = Some(finish.usage);
                final_reason = Some(finish.finish_reason);
            }
            _ => {
                other_parts += 1;
            }
        }
    }

    println!("\n\n📦 Stream Analysis:");
    println!("   Text delta parts: {}", text_parts);
    println!("   Metadata parts: {}", metadata_parts);
    println!("   Other parts: {}", other_parts);

    if let Some(usage) = final_usage {
        println!("\n📊 Usage:");
        println!("   Input tokens: {}", usage.input_tokens);
        println!("   Output tokens: {}", usage.output_tokens);
        println!("   Total tokens: {}", usage.total_tokens);
    }

    if let Some(reason) = final_reason {
        println!("🏁 Finish reason: {:?}\n", reason);
    }

    // Example 3: Collecting all stream parts
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 3: Complete Part Type Breakdown");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt3 = vec![LanguageModelMessage::user_text(
        "What are the benefits of using Rust? Be brief.",
    )];

    let options3 = LanguageModelCallOptions::new(prompt3).with_max_output_tokens(300);

    println!("💬 User: What are the benefits of using Rust?\n");

    let result3 = model.do_stream(options3).await?;
    let mut stream3 = result3.stream;

    let mut full_text = String::new();
    let mut part_types: Vec<String> = Vec::new();

    while let Some(part) = stream3.next().await {
        let part_type = match &part {
            LanguageModelStreamPart::StreamStart(_) => "StreamStart",
            LanguageModelStreamPart::ResponseMetadata(_) => "ResponseMetadata",
            LanguageModelStreamPart::TextStart(_) => "TextStart",
            LanguageModelStreamPart::TextDelta(delta) => {
                full_text.push_str(&delta.delta);
                "TextDelta"
            }
            LanguageModelStreamPart::TextEnd(_) => "TextEnd",
            LanguageModelStreamPart::ReasoningStart(_) => "ReasoningStart",
            LanguageModelStreamPart::ReasoningDelta(_) => "ReasoningDelta",
            LanguageModelStreamPart::ReasoningEnd(_) => "ReasoningEnd",
            LanguageModelStreamPart::ToolInputStart(_) => "ToolInputStart",
            LanguageModelStreamPart::ToolInputDelta(_) => "ToolInputDelta",
            LanguageModelStreamPart::ToolInputEnd(_) => "ToolInputEnd",
            LanguageModelStreamPart::ToolCall(_) => "ToolCall",
            LanguageModelStreamPart::ToolResult(_) => "ToolResult",
            LanguageModelStreamPart::File(_) => "File",
            LanguageModelStreamPart::Source(_) => "Source",
            LanguageModelStreamPart::Finish(_) => "Finish",
            LanguageModelStreamPart::Error(_) => "Error",
            LanguageModelStreamPart::Raw(_) => "Raw",
        };
        part_types.push(part_type.to_string());
    }

    println!("🤖 Complete Response: {}\n", full_text);

    println!("📊 Part Type Sequence:");
    for (i, part_type) in part_types.iter().enumerate() {
        println!("   {}. {}", i + 1, part_type);
    }

    println!("\n✅ All examples completed successfully!");
    println!("\n💡 Key Features Demonstrated:");
    println!("   ✓ Using do_stream() directly (provider-only)");
    println!("   ✓ Real-time text streaming");
    println!("   ✓ Processing different stream part types");
    println!("   ✓ Collecting usage information from stream");
    println!("   ✓ Stream part type analysis");
    println!("   ✓ Finish reason handling");

    Ok(())
}
