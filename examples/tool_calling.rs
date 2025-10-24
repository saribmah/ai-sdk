/// Tool calling example demonstrating function calling with a weather tool.
///
/// This example shows how to:
/// - Define a tool with parameters
/// - Use generate_text with tools
/// - Handle tool calls in the response
/// - Execute tools and process results
///
/// Run with:
/// ```bash
/// export OPENAI_API_KEY="your-api-key"
/// cargo run --example tool_calling
/// ```

use ai_sdk_core::{generate_text, ToolSet};
use ai_sdk_core::message::tool::definition::Tool;
use ai_sdk_core::prompt::{call_settings::CallSettings, Prompt};
use ai_sdk_openai_compatible::{create_openai_compatible, OpenAICompatibleProviderSettings};
use serde_json::{json, Value};
use std::env;

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
    println!("🤖 AI SDK Rust - Tool Calling Example\n");

    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(|_| {
        "OPENAI_API_KEY environment variable not set. Please set it with your API key."
    })?;

    println!("✓ API key loaded from environment");

    // Create OpenAI provider
    let provider = create_openai_compatible(
        OpenAICompatibleProviderSettings::new("https://api.openai.com/v1", "openai")
            .with_api_key(api_key),
    );

    let model = provider.chat_model("gpt-4o-mini");

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
    .with_description("Get the current weather for a given city");

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

    let settings = CallSettings::default()
        .with_temperature(0.7)
        .with_max_output_tokens(500);

    // Generate text with the tool
    println!("⏳ Generating response...\n");
    let result = generate_text(
        &*model,
        prompt,
        settings,
        Some(tools),
        None,
        None,
        None,
    )
    .await?;

    println!("✅ Response received!\n");

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Response Analysis");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("📊 Metadata:");
    println!("  • Finish reason: {:?}", result.finish_reason);
    println!("  • Input tokens: {}", result.usage.input_tokens);
    println!("  • Output tokens: {}", result.usage.output_tokens);
    println!("  • Total tokens: {}\n", result.usage.total_tokens);

    println!("📝 Content ({} parts):", result.content.len());
    for (i, content) in result.content.iter().enumerate() {
        println!("  [{}] {:?}", i + 1, content);
    }
    println!();

    // Check if the model made a tool call
    use ai_sdk_provider::language_model::content::Content;
    let mut found_tool_call = false;

    for content in &result.content {
        if let Content::ToolCall(tool_call) = content {
            found_tool_call = true;
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("Tool Call Detected!");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

            println!("🔧 Tool Call Details:");
            println!("  • Tool ID: {}", tool_call.tool_call_id);
            println!("  • Tool Name: {}", tool_call.tool_name);
            println!("  • Arguments: {}\n", tool_call.input);

            // Parse the arguments
            let args: Value = serde_json::from_str(&tool_call.input)?;
            if let Some(city) = args.get("city").and_then(|v| v.as_str()) {
                println!("📍 Executing tool: get_weather(city=\"{}\")\n", city);

                // Execute the tool
                let weather_data = get_weather(city);

                println!("☁️  Weather Results:");
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("  City: {}", weather_data["city"]);
                println!("  Temperature: {}°{}", weather_data["temperature"], weather_data["unit"]);
                println!("  Conditions: {}", weather_data["conditions"]);
                println!("  Humidity: {}%", weather_data["humidity"]);
                println!("  Wind Speed: {} mph", weather_data["wind_speed"]);
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

                println!("💡 In a real application, you would:");
                println!("   1. Execute the tool with these arguments");
                println!("   2. Create a tool result message with the weather data");
                println!("   3. Send it back to the model for a final response");
            }
        }
    }

    if !found_tool_call {
        println!("ℹ️  No tool calls detected in this response.");
        println!("   The model may have responded with text instead.\n");
    }

    println!("✅ Example completed successfully!\n");

    Ok(())
}
