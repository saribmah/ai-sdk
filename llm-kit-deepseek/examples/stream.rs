// Provider example: Streaming chat using do_stream()
// This example demonstrates using the DeepSeek provider's streaming capabilities directly.
// It does NOT use llm-kit-core (GenerateText, StreamText, etc.)

use futures_util::StreamExt;
use llm_kit_deepseek::DeepSeekClient;
use llm_kit_provider::language_model::call_options::LanguageModelCallOptions;
use llm_kit_provider::language_model::prompt::LanguageModelMessage;
use llm_kit_provider::language_model::stream_part::LanguageModelStreamPart;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load API key from environment
    let api_key =
        std::env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY environment variable not set");

    // Create the DeepSeek provider using the client builder
    let provider = DeepSeekClient::new().api_key(api_key).build();

    // Get a language model
    let model = provider.chat_model("deepseek-chat");

    println!("Streaming response from DeepSeek Chat (using do_stream)...\n");
    println!("Response:");

    // Create a prompt
    let prompt = vec![LanguageModelMessage::user_text(
        "Tell me a short story about a robot learning to code",
    )];

    // Create call options
    let options = LanguageModelCallOptions::new(prompt).with_max_output_tokens(500);

    // Call do_stream() directly (provider trait method)
    let mut result = model.do_stream(options).await?;

    // Process the stream
    while let Some(part) = result.stream.next().await {
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
