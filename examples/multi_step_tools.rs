use ai_sdk_core::prompt::{Prompt, call_settings::CallSettings};
use ai_sdk_core::tool::definition::Tool;
/// Multi-step tool execution example demonstrating iterative tool calling.
///
/// This example shows how to:
/// - Use tools with multi-step generation
/// - Handle multiple tool calls in a conversation
/// - See the iterative process as the model calls tools and processes results
///
/// Run with:
/// ```bash
/// export OPENAI_API_KEY="your-api-key"
/// cargo run --example multi_step_tools
/// ```
use ai_sdk_core::{ToolSet, generate_text, step_count_is};
use ai_sdk_openai_compatible::{OpenAICompatibleProvider, OpenAICompatibleProviderSettings};
use serde_json::{Value, json};
use std::env;

/// Simulates getting the current weather for a city
fn get_weather(city: &str) -> Value {
    println!("    🌤️  Executing: get_weather(city=\"{}\")", city);

    // Mock weather data
    let weather = match city.to_lowercase().as_str() {
        "san francisco" | "sf" => json!({
            "city": "San Francisco",
            "temperature": 68,
            "unit": "fahrenheit",
            "conditions": "Foggy",
            "humidity": 75,
        }),
        "new york" | "nyc" => json!({
            "city": "New York",
            "temperature": 72,
            "unit": "fahrenheit",
            "conditions": "Sunny",
            "humidity": 60,
        }),
        "london" => json!({
            "city": "London",
            "temperature": 55,
            "unit": "fahrenheit",
            "conditions": "Rainy",
            "humidity": 85,
        }),
        _ => json!({
            "city": city,
            "temperature": 70,
            "unit": "fahrenheit",
            "conditions": "Partly cloudy",
            "humidity": 65,
        }),
    };

    println!("    ✓ Weather data retrieved for {}", city);
    weather
}

/// Simulates converting temperature between units
fn convert_temperature(value: f64, from_unit: &str, to_unit: &str) -> Value {
    println!(
        "    🌡️  Executing: convert_temperature(value={}, from=\"{}\", to=\"{}\")",
        value, from_unit, to_unit
    );

    let celsius = if from_unit.to_lowercase() == "fahrenheit" {
        (value - 32.0) * 5.0 / 9.0
    } else {
        value
    };

    let result = if to_unit.to_lowercase() == "fahrenheit" {
        celsius * 9.0 / 5.0 + 32.0
    } else {
        celsius
    };

    println!(
        "    ✓ Converted {} {}° to {:.1} {}°",
        value, from_unit, result, to_unit
    );

    json!({
        "value": result,
        "unit": to_unit,
        "original_value": value,
        "original_unit": from_unit,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AI SDK Rust - Multi-Step Tool Execution Example\n");

    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(
        |_| "OPENAI_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create OpenAI provider
    let provider = OpenAICompatibleProvider::new(
        OpenAICompatibleProviderSettings::new("https://openrouter.ai/api/v1", "openai")
            .with_api_key(api_key),
    );

    let model = provider.model("openai/gpt-4o");
    println!("✓ Model loaded: {}\n", model.model_id());

    // Define tools
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Defining Tools");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    use ai_sdk_core::tool::definition::ToolExecutionOutput;

    // Weather tool
    let weather_tool = Tool::function(json!({
        "type": "object",
        "properties": {
            "city": {
                "type": "string",
                "description": "The city to get weather for"
            }
        },
        "required": ["city"]
    }))
    .with_description("Get the current weather for a given city")
    .with_execute(Box::new(|input: Value, _options| {
        let city = input
            .get("city")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");

        let weather_data = get_weather(city);

        ToolExecutionOutput::Single(Box::pin(async move { Ok(weather_data) }))
    }));

    // Temperature conversion tool
    let convert_tool = Tool::function(json!({
        "type": "object",
        "properties": {
            "value": {
                "type": "number",
                "description": "The temperature value to convert"
            },
            "from_unit": {
                "type": "string",
                "description": "The unit to convert from (fahrenheit or celsius)"
            },
            "to_unit": {
                "type": "string",
                "description": "The unit to convert to (fahrenheit or celsius)"
            }
        },
        "required": ["value", "from_unit", "to_unit"]
    }))
    .with_description("Convert temperature between fahrenheit and celsius")
    .with_execute(Box::new(|input: Value, _options| {
        let value = input.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let from_unit = input
            .get("from_unit")
            .and_then(|v| v.as_str())
            .unwrap_or("fahrenheit");
        let to_unit = input
            .get("to_unit")
            .and_then(|v| v.as_str())
            .unwrap_or("celsius");

        let result = convert_temperature(value, from_unit, to_unit);

        ToolExecutionOutput::Single(Box::pin(async move { Ok(result) }))
    }));

    // Create tool set
    let mut tools = ToolSet::new();
    tools.insert("get_weather".to_string(), weather_tool);
    tools.insert("convert_temperature".to_string(), convert_tool);

    println!("📋 Available Tools:");
    println!("  • get_weather - Get current weather for a city");
    println!("  • convert_temperature - Convert temperature between units\n");

    // Create a prompt that will require multiple tool calls
    let prompt = Prompt::text(
        "What's the weather in San Francisco and New York? \
         Also, convert the San Francisco temperature to Celsius.",
    );

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Multi-Step Generation with Tools");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("📤 User Prompt:");
    println!("   \"What's the weather in San Francisco and New York?");
    println!("    Also, convert the San Francisco temperature to Celsius.\"\n");

    let settings = CallSettings::default()
        .with_temperature(0.7)
        .with_max_output_tokens(1000);

    // Generate with tools - this will execute multiple steps
    println!("⏳ Starting multi-step generation (max 10 steps)...\n");

    let result = generate_text(
        &*model,
        prompt,
        settings,
        Some(tools),
        None,
        None,
        Some(vec![Box::new(step_count_is(10))]), // Allow up to 10 steps for multi-step tool execution
        None,
        None,
        None,
    )
    .await?;

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Generation Complete");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("📊 Summary:");
    println!("  • Total steps: {}", result.steps.len());
    println!("  • Finish reason: {:?}", result.finish_reason);
    println!(
        "  • Total input tokens: {}",
        result.total_usage.input_tokens
    );
    println!(
        "  • Total output tokens: {}",
        result.total_usage.output_tokens
    );
    println!("  • Total tokens: {}", result.total_usage.total_tokens);

    println!("\n📝 Step-by-Step Breakdown:\n");

    for (i, step) in result.steps.iter().enumerate() {
        println!("  Step {} ({:?}):", i + 1, step.finish_reason);
        println!(
            "    Tokens: {} in, {} out",
            step.usage.input_tokens, step.usage.output_tokens
        );

        // Check for text content
        let text = step.text();
        if !text.is_empty() {
            println!("    Text: {}", text);
        }

        // Check for tool calls
        let tool_calls = step.tool_calls();
        if !tool_calls.is_empty() {
            println!("    Tool calls made: {}", tool_calls.len());
            for tc in tool_calls {
                use ai_sdk_core::tool::TypedToolCall;
                match tc {
                    TypedToolCall::Static(call) => {
                        println!("      → {} ({})", call.tool_name, call.tool_call_id);
                    }
                    TypedToolCall::Dynamic(call) => {
                        println!("      → {} ({})", call.tool_name, call.tool_call_id);
                    }
                }
            }
        }

        // Check for tool results
        let tool_results = step.tool_results();
        if !tool_results.is_empty() {
            println!("    Tool results: {}", tool_results.len());
        }

        println!();
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Final Response");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("📝 Generated Text:\n");
    println!("{}\n", result.text);

    println!("✅ Example completed successfully!");
    println!("\nNote: This example demonstrates how generate_text can:");
    println!("  • Make multiple tool calls in a single step");
    println!("  • Execute tools and incorporate results");
    println!("  • Continue generation across multiple steps (with step_count_is(10))");
    println!("  • Accumulate token usage across all steps\n");

    Ok(())
}
