use llm_kit_cerebras::CerebrasClient;
use llm_kit_core::output::Output;
use llm_kit_core::prompt::Prompt;
use llm_kit_core::{GenerateText, ToolSet};
use llm_kit_provider_utils::tool::{Tool, ToolExecutionOutput};
use serde_json::{Value, json};
use std::env;
use std::sync::Arc;

/// Tool calling example with Cerebras.
///
/// This example shows how to:
/// - Define a tool with parameters
/// - Use GenerateText with tools
/// - Handle tool calls in the response
/// - Execute tools and process results
///
/// Run with:
/// ```bash
/// export CEREBRAS_API_KEY="your-api-key"
/// cargo run --example cerebras_tool_calling
/// ```
/// Simulates fetching weather data for a given city
fn get_weather(city: &str) -> Value {
    // This is a mock implementation - in a real app, you'd call a weather API
    json!({
        "city": city,
        "temperature": 72,
        "unit": "fahrenheit",
        "conditions": "Partly cloudy",
        "humidity": 65,
        "wind_speed": 10
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AI SDK Rust - Cerebras Tool Calling Example\n");

    // Load environment variables from .env file if present
    dotenvy::dotenv().ok();

    // Get API key from environment
    let api_key = env::var("CEREBRAS_API_KEY").map_err(
        |_| "CEREBRAS_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create Cerebras provider
    let provider = CerebrasClient::new().api_key(api_key).build();

    let model = provider.chat_model("llama-3.3-70b");

    println!("✓ Model loaded: {}\n", model.model_id());

    // Define the weather tool
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Defining Tool");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let weather_tool = Tool::function(json!({
        "type": "object",
        "properties": {
            "city": {
                "type": "string",
                "description": "The city to get weather for, e.g. San Francisco, CA"
            }
        },
        "required": ["city"]
    }))
    .with_description("Get the current weather for a given city")
    .with_execute(Arc::new(|input: Value, _options| {
        println!("\n🔧 TOOL EXECUTION TRIGGERED!");
        println!("🔧 Tool: get_weather");
        println!("🔧 Input: {}", input);

        let city = input
            .get("city")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");

        let weather_data = get_weather(city);
        println!("🔧 Output: {}\n", weather_data);

        ToolExecutionOutput::Single(Box::pin(async move { Ok(weather_data) }))
    }));

    // Create a ToolSet (HashMap of tool names to tools)
    let mut tools = ToolSet::new();
    tools.insert("get_weather".to_string(), weather_tool);

    println!("📋 Tool Name: get_weather");
    println!("📋 Description: Get the current weather for a given city");
    println!("📋 Parameters: city (string, required)\n");

    // Create a prompt that will trigger the tool
    let prompt = Prompt::text("What's the weather like in San Francisco?");

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Sending Request with Tool");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("📤 Prompt: \"What's the weather like in San Francisco?\"");
    println!("🔧 Available tools: [get_weather]\n");

    // Generate text with the tool
    println!("⏳ Generating response...\n");
    let result = GenerateText::new(model, prompt)
        .temperature(0.7)
        .max_output_tokens(500)
        .tools(tools)
        .execute()
        .await?;

    println!("✅ Response received!\n");

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Response Content");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Process the response
    for (i, output) in result.content.iter().enumerate() {
        match output {
            Output::Text(text_output) => {
                println!("📝 Text Response [{}]:", i + 1);
                println!("{}\n", text_output.text);
            }
            Output::ToolCall(tool_call) => {
                println!("🔧 Tool Call [{}]:", i + 1);
                println!("  • ID: {}", tool_call.tool_call_id);
                println!("  • Name: {}", tool_call.tool_name);
                println!("  • Input: {}\n", tool_call.input);
            }
            Output::ToolResult(tool_result) => {
                println!("✅ Tool Result [{}]:", i + 1);
                println!("  • ID: {}", tool_result.tool_call_id);
                println!("  • Result: {}\n", tool_result.output);
            }
            Output::Source(_) => {
                println!("📚 Source [{}]", i + 1);
            }
            Output::Reasoning(reasoning) => {
                println!("🧠 Reasoning [{}]:", i + 1);
                println!("{}\n", reasoning.text);
            }
            _ => {}
        }
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Metadata");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("📊 Finish Reason: {:?}", result.finish_reason);
    println!("📊 Input Tokens: {}", result.usage.input_tokens);
    println!("📊 Output Tokens: {}", result.usage.output_tokens);
    println!("📊 Total Tokens: {}", result.usage.total_tokens);

    println!("\n✅ Example completed successfully!");

    Ok(())
}
