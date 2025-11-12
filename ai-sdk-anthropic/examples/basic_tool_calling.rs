/// Basic tool calling example with Anthropic Claude.
///
/// This example demonstrates:
/// - Creating a simple function tool
/// - Using GenerateText with tools
/// - Handling tool calls in the response
/// - Multi-step tool execution
///
/// Run with:
/// ```bash
/// export ANTHROPIC_API_KEY="your-api-key"
/// cargo run --example basic_tool_calling -p ai-sdk-anthropic
/// ```
use ai_sdk_anthropic::{AnthropicProviderSettings, create_anthropic};
use ai_sdk_core::prompt::Prompt;
use ai_sdk_core::{GenerateText, ToolSet};
use ai_sdk_provider::language_model::LanguageModel;
use ai_sdk_provider_utils::tool::{Tool, ToolExecutionOutput};
use serde_json::{Value, json};
use std::env;
use std::sync::Arc;

/// Simulates fetching weather data for a given city
fn get_weather(city: &str, unit: &str) -> Value {
    // Mock implementation - in a real app, you'd call a weather API
    let temp = match unit {
        "celsius" => 22,
        _ => 72,
    };

    json!({
        "city": city,
        "temperature": temp,
        "unit": unit,
        "conditions": "Partly cloudy",
        "humidity": 65,
        "wind_speed": 10,
        "forecast": "Clear skies expected for the next 3 days"
    })
}

/// Simulates converting between temperature units
fn convert_temperature(value: f64, from_unit: &str, to_unit: &str) -> Value {
    let celsius = match from_unit {
        "fahrenheit" => (value - 32.0) * 5.0 / 9.0,
        "kelvin" => value - 273.15,
        _ => value,
    };

    let result = match to_unit {
        "fahrenheit" => celsius * 9.0 / 5.0 + 32.0,
        "kelvin" => celsius + 273.15,
        _ => celsius,
    };

    json!({
        "original_value": value,
        "original_unit": from_unit,
        "converted_value": result,
        "converted_unit": to_unit
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 Anthropic Tool Calling Example\n");

    // Get API key from environment
    let api_key = env::var("ANTHROPIC_API_KEY").map_err(
        |_| "ANTHROPIC_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create Anthropic provider
    let settings = AnthropicProviderSettings::new().with_api_key(api_key);
    let provider = create_anthropic(settings);
    let model = Arc::new(provider.language_model("claude-3-haiku-20240307".to_string()));

    println!("✓ Model loaded: {}\n", model.model_id());

    // Define tools
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Defining Tools");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Tool 1: Get Weather
    let weather_tool = Tool::function(json!({
        "type": "object",
        "properties": {
            "city": {
                "type": "string",
                "description": "The city to get weather for, e.g. San Francisco, CA"
            },
            "unit": {
                "type": "string",
                "enum": ["celsius", "fahrenheit"],
                "description": "Temperature unit"
            }
        },
        "required": ["city"]
    }))
    .with_description("Get the current weather for a given city")
    .with_execute(Arc::new(|input: Value, _options| {
        println!("\n🔧 TOOL EXECUTION: get_weather");
        println!(
            "   Input: {}",
            serde_json::to_string_pretty(&input).unwrap()
        );

        let city = input
            .get("city")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");
        let unit = input
            .get("unit")
            .and_then(|v| v.as_str())
            .unwrap_or("fahrenheit");

        let weather_data = get_weather(city, unit);
        println!(
            "   Output: {}\n",
            serde_json::to_string_pretty(&weather_data).unwrap()
        );

        ToolExecutionOutput::Single(Box::pin(async move { Ok(weather_data) }))
    }));

    // Tool 2: Temperature Converter
    let converter_tool = Tool::function(json!({
        "type": "object",
        "properties": {
            "value": {
                "type": "number",
                "description": "The temperature value to convert"
            },
            "from_unit": {
                "type": "string",
                "enum": ["celsius", "fahrenheit", "kelvin"],
                "description": "The unit to convert from"
            },
            "to_unit": {
                "type": "string",
                "enum": ["celsius", "fahrenheit", "kelvin"],
                "description": "The unit to convert to"
            }
        },
        "required": ["value", "from_unit", "to_unit"]
    }))
    .with_description("Convert temperature between different units")
    .with_execute(Arc::new(|input: Value, _options| {
        println!("\n🔧 TOOL EXECUTION: convert_temperature");
        println!(
            "   Input: {}",
            serde_json::to_string_pretty(&input).unwrap()
        );

        let value = input.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let from_unit = input
            .get("from_unit")
            .and_then(|v| v.as_str())
            .unwrap_or("celsius");
        let to_unit = input
            .get("to_unit")
            .and_then(|v| v.as_str())
            .unwrap_or("celsius");

        let result = convert_temperature(value, from_unit, to_unit);
        println!(
            "   Output: {}\n",
            serde_json::to_string_pretty(&result).unwrap()
        );

        ToolExecutionOutput::Single(Box::pin(async move { Ok(result) }))
    }));

    // Create ToolSet
    let mut tools = ToolSet::new();
    tools.insert("get_weather".to_string(), weather_tool);
    tools.insert("convert_temperature".to_string(), converter_tool);

    println!("📋 Registered Tools:");
    println!("   1. get_weather - Get current weather for a city");
    println!("   2. convert_temperature - Convert temperature units\n");

    // Example 1: Simple tool call
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 1: Simple Weather Query");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt1 = Prompt::text("What's the weather like in San Francisco?");

    println!("💬 User: What's the weather like in San Francisco?\n");

    let result1 = GenerateText::new(model.clone(), prompt1)
        .tools(tools.clone())
        .execute()
        .await?;

    println!("🤖 Assistant: {}\n", result1.text);
    println!(
        "📊 Usage: {} input tokens, {} output tokens",
        result1.usage.input_tokens, result1.usage.output_tokens
    );
    println!("🏁 Finish reason: {:?}\n", result1.finish_reason);

    // Example 2: Multi-step tool usage
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: Multi-step Tool Usage");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt2 = Prompt::text(
        "Get the weather in Tokyo in celsius, then convert that temperature to fahrenheit.",
    );

    println!(
        "💬 User: Get the weather in Tokyo in celsius, then convert that temperature to fahrenheit.\n"
    );

    let result2 = GenerateText::new(model.clone(), prompt2)
        .tools(tools.clone())
        .max_output_tokens(1024)
        .execute()
        .await?;

    println!("🤖 Assistant: {}\n", result2.text);
    println!(
        "📊 Usage: {} input tokens, {} output tokens",
        result2.usage.input_tokens, result2.usage.output_tokens
    );
    println!("🔄 Total steps: {}\n", result2.steps.len());

    // Example 3: Examining tool calls in detail
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 3: Examining Tool Call Details");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt3 = Prompt::text("What's the weather in New York and London?");

    println!("💬 User: What's the weather in New York and London?\n");

    let result3 = GenerateText::new(model, prompt3)
        .tools(tools)
        .execute()
        .await?;

    // Examine the steps to see tool calls and results
    println!("📊 Execution steps: {}\n", result3.steps.len());

    for (step_idx, step) in result3.steps.iter().enumerate() {
        println!("Step {}:", step_idx + 1);
        println!("  Finish reason: {:?}", step.finish_reason);
        println!(
            "  Tokens: {} in, {} out",
            step.usage.input_tokens, step.usage.output_tokens
        );

        // Show text if present
        let text = step.text();
        if !text.is_empty() {
            println!("  Text: {}", text);
        }

        // Show tool calls in this step
        let tool_calls = step.tool_calls();
        if !tool_calls.is_empty() {
            println!("  🔧 Tool calls made: {}", tool_calls.len());
            for tc in tool_calls {
                println!("     → {} (ID: {})", tc.tool_name, tc.tool_call_id);
                println!(
                    "       Args: {}",
                    serde_json::to_string_pretty(&tc.input).unwrap()
                );
            }
        }

        // Show tool results
        let tool_results = step.tool_results();
        if !tool_results.is_empty() {
            println!("  📥 Tool results: {}", tool_results.len());
            for tr in tool_results {
                println!("     → {} (ID: {})", tr.tool_name, tr.tool_call_id);
                println!(
                    "       Result: {}",
                    serde_json::to_string_pretty(&tr.output).unwrap()
                );
            }
        }

        println!();
    }

    println!("🤖 Final Response: {}\n", result3.text);

    println!("✅ Example completed successfully!");
    println!("\n💡 Key Features Demonstrated:");
    println!("   ✓ Function tool definition with JSON schema");
    println!("   ✓ Tool execution with custom logic");
    println!("   ✓ Single and multi-step tool calls");
    println!("   ✓ Parallel tool execution");
    println!("   ✓ Tool call and result inspection");

    Ok(())
}
