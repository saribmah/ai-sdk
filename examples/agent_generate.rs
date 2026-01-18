/// Agent generate example demonstrating agent-based tool calling (non-streaming).
///
/// This example shows how to:
/// - Create an agent with tools and instructions
/// - Use the agent.generate() method for non-streaming responses
/// - Handle multi-step tool execution automatically
/// - Configure agent settings (temperature, max tokens, etc.)
///
/// Run with:
/// ```bash
/// export OPENAI_API_KEY="your-api-key"
/// cargo run --example agent_generate
/// ```
use llm_kit_core::{Agent, AgentCallParameters, AgentInterface, AgentSettings};
use llm_kit_core::{ToolSet, step_count_is};
use llm_kit_openai_compatible::OpenAICompatibleClient;
use llm_kit_provider_utils::tool::Tool;
use serde_json::{Value, json};
use std::env;
use std::sync::Arc;

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
            "wind": "10 mph NW"
        }),
        "new york" | "nyc" => json!({
            "city": "New York",
            "temperature": 72,
            "unit": "fahrenheit",
            "conditions": "Sunny",
            "humidity": 60,
            "wind": "8 mph SE"
        }),
        "london" => json!({
            "city": "London",
            "temperature": 55,
            "unit": "fahrenheit",
            "conditions": "Rainy",
            "humidity": 85,
            "wind": "12 mph W"
        }),
        "tokyo" => json!({
            "city": "Tokyo",
            "temperature": 75,
            "unit": "fahrenheit",
            "conditions": "Clear",
            "humidity": 70,
            "wind": "6 mph E"
        }),
        _ => json!({
            "city": city,
            "temperature": 70,
            "unit": "fahrenheit",
            "conditions": "Partly cloudy",
            "humidity": 65,
            "wind": "5 mph N"
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

/// Simulates getting travel time between cities
fn get_travel_time(from: &str, to: &str, mode: &str) -> Value {
    println!(
        "    ✈️  Executing: get_travel_time(from=\"{}\", to=\"{}\", mode=\"{}\")",
        from, to, mode
    );

    // Mock travel time data
    let duration_hours: f64 = match mode.to_lowercase().as_str() {
        "flight" => 5.5,
        "train" => 12.0,
        "car" => 18.0,
        _ => 8.0,
    };

    let result = json!({
        "from": from,
        "to": to,
        "mode": mode,
        "duration_hours": duration_hours,
        "duration_formatted": format!("{} hours {} minutes",
            duration_hours.floor(),
            ((duration_hours.fract() * 60.0).round() as i32)
        )
    });

    println!("    ✓ Travel time calculated: {} hours", duration_hours);
    result
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AI SDK Rust - Agent Generate Example (Non-Streaming)\n");

    // Get API key from environment
    let api_key = env::var("OPENAI_API_KEY").map_err(
        |_| "OPENAI_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create OpenAI provider using the client builder
    let provider = OpenAICompatibleClient::new()
        .base_url("https://openrouter.ai/api/v1")
        .api_key(api_key)
        .build();

    let model = provider.chat_model("openai/gpt-4o-mini");
    println!("✓ Model loaded: {}\n", model.model_id());

    // Define tools
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Defining Tools");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    use llm_kit_provider_utils::tool::ToolExecutionOutput;

    // Helper function to create tools (since tools can't be cloned, we recreate them for each call)
    let mut tools = ToolSet::new();

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
    .with_execute(Arc::new(|input: Value, _options| {
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
                "description": "The unit to convert from (fahrenheit or celsius)",
                "enum": ["fahrenheit", "celsius"]
            },
            "to_unit": {
                "type": "string",
                "description": "The unit to convert to (fahrenheit or celsius)",
                "enum": ["fahrenheit", "celsius"]
            }
        },
        "required": ["value", "from_unit", "to_unit"]
    }))
    .with_description("Convert temperature between fahrenheit and celsius")
    .with_execute(Arc::new(|input: Value, _options| {
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

    // Travel time tool
    let travel_tool = Tool::function(json!({
        "type": "object",
        "properties": {
            "from": {
                "type": "string",
                "description": "The departure city"
            },
            "to": {
                "type": "string",
                "description": "The destination city"
            },
            "mode": {
                "type": "string",
                "description": "The mode of transportation",
                "enum": ["flight", "train", "car"]
            }
        },
        "required": ["from", "to", "mode"]
    }))
    .with_description("Get estimated travel time between two cities")
    .with_execute(Arc::new(|input: Value, _options| {
        let from = input.get("from").and_then(|v| v.as_str()).unwrap_or("");
        let to = input.get("to").and_then(|v| v.as_str()).unwrap_or("");
        let mode = input
            .get("mode")
            .and_then(|v| v.as_str())
            .unwrap_or("flight");

        let result = get_travel_time(from, to, mode);
        ToolExecutionOutput::Single(Box::pin(async move { Ok(result) }))
    }));

    tools.insert("get_weather".to_string(), weather_tool);
    tools.insert("convert_temperature".to_string(), convert_tool);
    tools.insert("get_travel_time".to_string(), travel_tool);

    println!("📋 Available Tools:");
    println!("   1. get_weather - Get current weather for a city");
    println!("   2. convert_temperature - Convert between °F and °C");
    println!("   3. get_travel_time - Get travel time between cities\n");

    // Create agent settings
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Creating Agent");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let settings = AgentSettings::new(model)
        .with_id("weather-travel-agent")
        .with_instructions(
            "You are a helpful travel and weather assistant. \
             When users ask about weather or travel, use the available tools to get accurate information. \
             Always provide temperature in both Fahrenheit and Celsius. \
             Be conversational and friendly in your responses."
        )
        .with_temperature(0.7)
        .with_max_output_tokens(1000)
        .with_stop_when(vec![Arc::new(step_count_is(10))])
        .with_tools(tools);

    let agent = Agent::new(settings);

    println!("✓ Agent created with ID: {:?}", agent.settings().id);
    println!("✓ Temperature: {:?}", agent.settings().temperature);
    println!(
        "✓ Max output tokens: {:?}",
        agent.settings().max_output_tokens
    );
    println!("✓ Tools: Passed in each call\n");

    // Example 1: Simple weather query
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 1: Simple Weather Query");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let params1 = AgentCallParameters::from_text(
        "What's the weather like in San Francisco? Please tell me the temperature in both Fahrenheit and Celsius.",
    );

    println!(
        "📝 User: What's the weather like in San Francisco? Please tell me the temperature in both Fahrenheit and Celsius.\n"
    );

    let result1 = agent.generate(params1)?.execute().await?;

    println!("\n🤖 Agent Response:");
    println!("{}", result1.text);
    println!("\n📊 Stats:");
    println!("   • Steps taken: {}", result1.steps.len());
    println!("   • Finish reason: {:?}", result1.finish_reason);
    println!("   • Total tokens: {}", result1.usage.total_tokens);

    // Example 2: Multi-city comparison
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: Multi-City Weather Comparison");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let params2 = AgentCallParameters::from_text(
        "Compare the weather in New York and London. Which city is warmer?",
    );

    println!("📝 User: Compare the weather in New York and London. Which city is warmer?\n");

    let result2 = agent.generate(params2)?.execute().await?;

    println!("\n🤖 Agent Response:");
    println!("{}", result2.text);
    println!("\n📊 Stats:");
    println!("   • Steps taken: {}", result2.steps.len());
    println!("   • Finish reason: {:?}", result2.finish_reason);
    println!("   • Total tokens: {}", result2.usage.total_tokens);

    // Example 3: Complex multi-tool query
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 3: Complex Multi-Tool Query");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let params3 = AgentCallParameters::from_text(
        "I'm planning a trip from San Francisco to Tokyo. What's the weather like in both cities, \
         and how long would it take to fly there?",
    );

    println!(
        "📝 User: I'm planning a trip from San Francisco to Tokyo. What's the weather like in both cities, and how long would it take to fly there?\n"
    );

    let result3 = agent.generate(params3)?.execute().await?;

    println!("\n🤖 Agent Response:");
    println!("{}", result3.text);
    println!("\n📊 Stats:");
    println!("   • Steps taken: {}", result3.steps.len());
    println!("   • Finish reason: {:?}", result3.finish_reason);
    println!("   • Total tokens: {}", result3.usage.total_tokens);

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("✅ Agent Generate Example Complete!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok(())
}
