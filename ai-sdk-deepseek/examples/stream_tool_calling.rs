// Provider example: Streaming tool calling with do_stream()
// This example demonstrates using the DeepSeek provider's streaming with tool calling.
// It does NOT use llm-kit-core (GenerateText, StreamText, etc.)

use futures_util::StreamExt;
use llm_kit_deepseek::DeepSeekClient;
use llm_kit_provider::language_model::call_options::LanguageModelCallOptions;
use llm_kit_provider::language_model::prompt::LanguageModelMessage;
use llm_kit_provider::language_model::stream_part::LanguageModelStreamPart;
use llm_kit_provider::language_model::tool::LanguageModelTool;
use llm_kit_provider::language_model::tool::function_tool::LanguageModelFunctionTool;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load API key from environment
    let api_key =
        std::env::var("DEEPSEEK_API_KEY").expect("DEEPSEEK_API_KEY environment variable not set");

    // Create the DeepSeek provider using the client builder
    let provider = DeepSeekClient::new().api_key(api_key).build();

    // Get a language model
    let model = provider.chat_model("deepseek-chat");

    println!("Testing streaming tool calling with DeepSeek Chat (using do_stream)...\n");

    // Define a simple calculator tool
    let calculator_tool = LanguageModelFunctionTool::new(
        "calculate",
        json!({
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "The mathematical expression to evaluate, e.g. '2 + 2'"
                }
            },
            "required": ["expression"]
        }),
    )
    .with_description("Perform a mathematical calculation");

    let tools = vec![LanguageModelTool::Function(calculator_tool)];

    // Create a prompt that should trigger tool calling
    let prompt = vec![LanguageModelMessage::user_text(
        "What is 156 multiplied by 47?",
    )];

    // Create call options with tools
    let options = LanguageModelCallOptions::new(prompt)
        .with_tools(tools)
        .with_max_output_tokens(500);

    // Call do_stream() with tools
    let mut result = model.do_stream(options).await?;

    println!("Streaming response:\n");

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
            LanguageModelStreamPart::ToolInputStart(start) => {
                println!("\n\nTool Call: {}", start.tool_name);
                println!("  ID: {}", start.id);
                print!("  Arguments: ");
                std::io::Write::flush(&mut std::io::stdout())?;
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
