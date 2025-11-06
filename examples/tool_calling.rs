use ai_sdk_core::output::Output;
use ai_sdk_core::prompt::{Prompt, call_settings::CallSettings};
use ai_sdk_core::tool::definition::Tool;
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
use ai_sdk_core::{ToolSet, generate_text};
use ai_sdk_openai_compatible::{OpenAICompatibleProviderSettings, create_openai_compatible};
use serde_json::{Value, json};
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
    let api_key = env::var("OPENAI_API_KEY").map_err(
        |_| "OPENAI_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create OpenAI provider
    let provider = create_openai_compatible(
        OpenAICompatibleProviderSettings::new("https://openrouter.ai/api/v1", "openai")
            .with_api_key(api_key),
    );

    let model = provider.chat_model("gpt-4o-mini");

    println!("✓ Model loaded: {}\n", model.model_id());

    // Define the weather tool
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Defining Tool");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    use ai_sdk_core::tool::definition::ToolExecutionOutput;

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
    .with_execute(Box::new(|input: Value, _options| {
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

    // Check if the model made a tool call by looking at the steps
    let mut found_tool_call = false;

    // Check the last step for tool calls
    if let Some(last_step) = result.steps.last() {
        for content in &last_step.content {
            use ai_sdk_core::tool::TypedToolCall;
            if let Output::ToolCall(tool_call) = content {
                found_tool_call = true;
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                println!("Tool Call Detected!");
                println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

                // Extract tool call details based on the variant
                let (tool_call_id, tool_name, input_str) = match tool_call {
                    TypedToolCall::Static(call) => (
                        &call.tool_call_id,
                        &call.tool_name,
                        serde_json::to_string(&call.input)?,
                    ),
                    TypedToolCall::Dynamic(call) => (
                        &call.tool_call_id,
                        &call.tool_name,
                        serde_json::to_string(&call.input)?,
                    ),
                };

                println!("🔧 Tool Call Details:");
                println!("  • Tool ID: {}", tool_call_id);
                println!("  • Tool Name: {}", tool_name);
                println!("  • Arguments: {}\n", input_str);

                // Parse the arguments
                let args: Value = serde_json::from_str(&input_str)?;
                if let Some(city) = args.get("city").and_then(|v| v.as_str()) {
                    println!("📍 Executing tool: get_weather(city=\"{}\")\n", city);

                    // Execute the tool
                    let weather_data = get_weather(city);

                    println!("☁️  Weather Results:");
                    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
                    println!("  City: {}", weather_data["city"]);
                    println!(
                        "  Temperature: {}°{}",
                        weather_data["temperature"], weather_data["unit"]
                    );
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
    }

    if !found_tool_call {
        println!("ℹ️  No tool calls detected in this response.");
        println!("   The model may have responded with text instead.\n");
    }

    println!("✅ Example completed successfully!\n");

    Ok(())
}
