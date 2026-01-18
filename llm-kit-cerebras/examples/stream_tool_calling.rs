use futures_util::StreamExt;
/// Streaming tool calling example using llm-kit-provider traits only.
///
/// This example demonstrates direct usage of LanguageModel::do_stream() with tools
/// without llm-kit-core abstractions.
///
/// Run with:
/// ```bash
/// export CEREBRAS_API_KEY="your-api-key"
/// cargo run --example stream_tool_calling
/// ```
use llm_kit_cerebras::CerebrasClient;
use llm_kit_provider::language_model::call_options::LanguageModelCallOptions;
use llm_kit_provider::language_model::prompt::LanguageModelMessage;
use llm_kit_provider::language_model::stream_part::LanguageModelStreamPart;
use llm_kit_provider::language_model::tool::LanguageModelTool;
use llm_kit_provider::language_model::tool::function_tool::LanguageModelFunctionTool;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 Cerebras Streaming Tool Calling Example (Provider Traits)\n");

    // Load environment variables
    dotenvy::dotenv().ok();

    // Get API key from environment
    let api_key = std::env::var("CEREBRAS_API_KEY")
        .map_err(|_| "CEREBRAS_API_KEY environment variable not set")?;

    println!("✓ API key loaded from environment");

    // Create provider using builder
    let provider = CerebrasClient::new().api_key(api_key).build();

    println!("✓ Provider created: {}", provider.name());

    // Get a language model
    let model = provider.chat_model("llama-3.3-70b");
    println!("✓ Model: {}\n", model.model_id());

    // Define a weather tool using provider types
    let weather_tool = LanguageModelFunctionTool::new(
        "get_weather",
        json!({
            "type": "object",
            "properties": {
                "city": {
                    "type": "string",
                    "description": "The city to get weather for, e.g. San Francisco, CA"
                }
            },
            "required": ["city"]
        }),
    )
    .with_description("Get the current weather for a given city");

    println!("📋 Tool defined: get_weather\n");

    // Create a prompt that will trigger the tool
    let messages = vec![LanguageModelMessage::user_text(
        "What's the weather like in San Francisco?",
    )];

    println!("📤 Sending prompt with streaming...\n");

    // Call do_stream with tools
    let options = LanguageModelCallOptions::new(messages)
        .with_temperature(0.7)
        .with_max_output_tokens(500)
        .with_tools(vec![LanguageModelTool::Function(weather_tool)]);

    let result = model.do_stream(options).await?;

    println!("⏳ Streaming response...\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Process the stream
    let mut stream = result.stream;
    while let Some(part) = stream.next().await {
        match part {
            LanguageModelStreamPart::TextStart(text_start) => {
                println!("\n📝 Text started (ID: {})", text_start.id);
                print!("   ");
                std::io::Write::flush(&mut std::io::stdout())?;
            }
            LanguageModelStreamPart::TextDelta(text_delta) => {
                print!("{}", text_delta.delta);
                std::io::Write::flush(&mut std::io::stdout())?;
            }
            LanguageModelStreamPart::TextEnd(_) => {
                println!();
            }
            LanguageModelStreamPart::ToolInputStart(tool_input_start) => {
                println!("\n🔧 Tool call started:");
                println!("   • ID: {}", tool_input_start.id);
                println!("   • Name: {}", tool_input_start.tool_name);
                print!("   • Arguments: ");
                std::io::Write::flush(&mut std::io::stdout())?;
            }
            LanguageModelStreamPart::ToolInputDelta(tool_input_delta) => {
                print!("{}", tool_input_delta.delta);
                std::io::Write::flush(&mut std::io::stdout())?;
            }
            LanguageModelStreamPart::ToolInputEnd(_) => {
                println!();
            }
            LanguageModelStreamPart::ToolCall(tool_call) => {
                println!("\n✅ Tool call completed:");
                println!("   • ID: {}", tool_call.tool_call_id);
                println!("   • Name: {}", tool_call.tool_name);
                println!("   • Arguments: {}", tool_call.input);
            }
            LanguageModelStreamPart::ReasoningStart(reasoning_start) => {
                println!("\n💭 Reasoning started (ID: {})", reasoning_start.id);
                print!("   ");
                std::io::Write::flush(&mut std::io::stdout())?;
            }
            LanguageModelStreamPart::ReasoningDelta(reasoning_delta) => {
                print!("{}", reasoning_delta.delta);
                std::io::Write::flush(&mut std::io::stdout())?;
            }
            LanguageModelStreamPart::ReasoningEnd(_) => {
                println!();
            }
            LanguageModelStreamPart::Finish(finish) => {
                println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("\n📊 Stream Complete:");
                println!("  • Finish reason: {:?}", finish.finish_reason);
                println!("  • Input tokens: {}", finish.usage.input_tokens);
                println!("  • Output tokens: {}", finish.usage.output_tokens);
                println!("  • Total tokens: {}", finish.usage.total_tokens);
            }
            LanguageModelStreamPart::Error(error) => {
                eprintln!("\n❌ Stream error: {:?}", error);
            }
            _ => {}
        }
    }

    println!("\n✅ Example completed successfully!");

    Ok(())
}
