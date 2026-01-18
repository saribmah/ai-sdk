/// Tool calling example using llm-kit-provider traits only.
///
/// This example demonstrates direct usage of LanguageModel::do_generate() with tools
/// without llm-kit-core abstractions.
///
/// Run with:
/// ```bash
/// export CEREBRAS_API_KEY="your-api-key"
/// cargo run --example chat_tool_calling
/// ```
use llm_kit_cerebras::CerebrasClient;
use llm_kit_provider::language_model::call_options::LanguageModelCallOptions;
use llm_kit_provider::language_model::content::LanguageModelContent;
use llm_kit_provider::language_model::prompt::LanguageModelMessage;
use llm_kit_provider::language_model::tool::LanguageModelTool;
use llm_kit_provider::language_model::tool::function_tool::LanguageModelFunctionTool;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 Cerebras Tool Calling Example (Provider Traits)\n");

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

    println!("📋 Tool defined: get_weather");
    println!("📋 Description: Get the current weather for a given city\n");

    // Create a prompt that will trigger the tool
    let messages = vec![LanguageModelMessage::user_text(
        "What's the weather like in San Francisco?",
    )];

    println!("📤 Sending prompt: \"What's the weather like in San Francisco?\"\n");

    // Call do_generate with tools
    let options = LanguageModelCallOptions::new(messages)
        .with_temperature(0.7)
        .with_max_output_tokens(500)
        .with_tools(vec![LanguageModelTool::Function(weather_tool)]);

    let result = model.do_generate(options).await?;

    println!("✅ Response received!\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📝 Content:");

    for (i, content) in result.content.iter().enumerate() {
        match content {
            LanguageModelContent::Text(text) => {
                println!("  [{}] Text: {}", i + 1, text.text);
            }
            LanguageModelContent::ToolCall(tool_call) => {
                println!("  [{}] Tool Call:", i + 1);
                println!("      • ID: {}", tool_call.tool_call_id);
                println!("      • Name: {}", tool_call.tool_name);
                println!("      • Arguments: {}", tool_call.input);
            }
            LanguageModelContent::Reasoning(reasoning) => {
                println!("  [{}] Reasoning: {}", i + 1, reasoning.text);
            }
            _ => {
                println!("  [{}] Other content type", i + 1);
            }
        }
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    println!("\n📊 Metadata:");
    println!("  • Finish reason: {:?}", result.finish_reason);
    println!("  • Input tokens: {}", result.usage.input_tokens);
    println!("  • Output tokens: {}", result.usage.output_tokens);
    println!("  • Total tokens: {}", result.usage.total_tokens);

    println!("\n✅ Example completed successfully!");

    Ok(())
}
