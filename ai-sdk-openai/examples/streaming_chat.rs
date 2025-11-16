//! Streaming chat completion example using OpenAI provider
//!
//! Run with: cargo run --example streaming_chat

use ai_sdk_openai::OpenAIClient;
use ai_sdk_provider::language_model::LanguageModel;
use ai_sdk_provider::language_model::call_options::LanguageModelCallOptions;
use ai_sdk_provider::language_model::prompt::message::LanguageModelMessage;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create provider (reads OPENAI_API_KEY from environment)
    let provider = OpenAIClient::new().build();

    // Create a chat model
    let model = provider.chat("gpt-4o-mini");

    println!("Using model: {}", model.model_id());
    println!("Streaming response...\n");

    // Create a simple prompt
    let prompt = vec![
        LanguageModelMessage::system("You are a helpful assistant."),
        LanguageModelMessage::user_text("Write a haiku about programming."),
    ];

    // Create call options
    let options = LanguageModelCallOptions {
        prompt,
        max_output_tokens: Some(100),
        temperature: Some(0.7),
        top_p: None,
        top_k: None,
        frequency_penalty: None,
        presence_penalty: None,
        stop_sequences: None,
        seed: None,
        response_format: None,
        tools: None,
        tool_choice: None,
        headers: None,
        provider_options: None,
        abort_signal: None,
        include_raw_chunks: None,
    };

    // Generate streaming response
    let mut response = model.do_stream(options).await?;

    // Process the stream
    while let Some(part) = response.stream.next().await {
        use ai_sdk_provider::language_model::stream_part::LanguageModelStreamPart;

        match part {
            LanguageModelStreamPart::StreamStart(start) => {
                if !start.warnings.is_empty() {
                    println!("Warnings: {:?}", start.warnings);
                }
            }
            LanguageModelStreamPart::TextDelta(delta) => {
                print!("{}", delta.delta);
                std::io::Write::flush(&mut std::io::stdout())?;
            }
            LanguageModelStreamPart::ReasoningDelta(delta) => {
                println!("\n[Reasoning: {}]", delta.delta);
            }
            LanguageModelStreamPart::ToolInputStart(start) => {
                println!("\n\nTool call: {}", start.tool_name);
            }
            LanguageModelStreamPart::ToolInputDelta(delta) => {
                print!("{}", delta.delta);
                std::io::Write::flush(&mut std::io::stdout())?;
            }
            LanguageModelStreamPart::Finish(finish) => {
                println!("\n\nFinish reason: {:?}", finish.finish_reason);
                println!("Usage:");
                println!("  Input tokens: {}", finish.usage.input_tokens);
                println!("  Output tokens: {}", finish.usage.output_tokens);
                println!("  Total tokens: {}", finish.usage.total_tokens);
                if finish.usage.reasoning_tokens > 0 {
                    println!("  Reasoning tokens: {}", finish.usage.reasoning_tokens);
                }
            }
            LanguageModelStreamPart::Error(error) => {
                eprintln!("\nError: {:?}", error);
            }
            _ => {}
        }
    }

    println!("\n");

    Ok(())
}
