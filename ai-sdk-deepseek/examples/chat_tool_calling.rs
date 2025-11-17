// Provider example: Tool calling with do_generate()
// This example demonstrates using the DeepSeek provider with tool calling.
// It does NOT use ai-sdk-core (GenerateText, StreamText, etc.)

use ai_sdk_deepseek::DeepSeekClient;
use ai_sdk_provider::language_model::call_options::LanguageModelCallOptions;
use ai_sdk_provider::language_model::content::LanguageModelContent;
use ai_sdk_provider::language_model::prompt::LanguageModelMessage;
use ai_sdk_provider::language_model::tool::LanguageModelTool;
use ai_sdk_provider::language_model::tool::function_tool::LanguageModelFunctionTool;
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

    println!("Testing tool calling with DeepSeek Chat (using do_generate)...\n");

    // Define a simple weather tool
    let weather_tool = LanguageModelFunctionTool::new(
        "get_weather",
        json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The city and state, e.g. San Francisco, CA"
                },
                "unit": {
                    "type": "string",
                    "enum": ["celsius", "fahrenheit"],
                    "description": "The temperature unit"
                }
            },
            "required": ["location"]
        }),
    )
    .with_description("Get the current weather for a location");

    let tools = vec![LanguageModelTool::Function(weather_tool)];

    // Create a prompt that should trigger tool calling
    let prompt = vec![LanguageModelMessage::user_text(
        "What's the weather like in San Francisco?",
    )];

    // Create call options with tools
    let options = LanguageModelCallOptions::new(prompt)
        .with_tools(tools)
        .with_max_output_tokens(500);

    // Call do_generate() with tools
    let result = model.do_generate(options).await?;

    // Print the response
    println!("Response:");
    for content in &result.content {
        match content {
            LanguageModelContent::Text(text) => {
                println!("{}", text.text);
            }
            LanguageModelContent::ToolCall(tool_call) => {
                println!("\nTool Call Detected:");
                println!("  Tool: {}", tool_call.tool_name);
                println!("  ID: {}", tool_call.tool_call_id);
                println!("  Arguments: {}", tool_call.input);

                // Simulate executing the tool call
                if tool_call.tool_name == "get_weather" {
                    println!(
                        "  Result: {{\"temperature\": 72, \"unit\": \"fahrenheit\", \"condition\": \"sunny\"}}"
                    );
                }
            }
            _ => {}
        }
    }
    println!();

    // Print usage information
    println!("Token Usage:");
    println!("  Input tokens: {}", result.usage.input_tokens);
    println!("  Output tokens: {}", result.usage.output_tokens);
    println!("  Total tokens: {}", result.usage.total_tokens);

    // Print finish reason
    println!("\nFinish Reason: {:?}", result.finish_reason);

    Ok(())
}
