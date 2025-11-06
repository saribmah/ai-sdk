/// Streaming tool calling example demonstrating real-time tool execution with streaming.
///
/// This example shows how to:
/// - Define tools with parameters
/// - Use stream_text with tools for real-time streaming
/// - Stream text deltas while tools are being called and executed
/// - Handle multi-step tool execution with streaming
/// - See tool calls, executions, and results in real-time
///
/// Run with:
/// ```bash
/// export OPENAI_API_KEY="your-api-key"
/// cargo run --example stream_tool_calling
/// ```
use ai_sdk_core::tool::definition::Tool;
use ai_sdk_core::prompt::{Prompt, call_settings::CallSettings};
use ai_sdk_core::{ToolSet, step_count_is, stream_text};
use ai_sdk_openai_compatible::{OpenAICompatibleProviderSettings, create_openai_compatible};
use futures_util::StreamExt;
use serde_json::{Value, json};
use std::env;
use std::sync::Arc;

/// Simulates fetching weather data for a given city
fn get_weather(city: &str) -> Value {
    json!({
        "city": city,
        "temperature": match city.to_lowercase().as_str() {
            "san francisco" | "sf" => 68,
            "new york" | "nyc" => 72,
            "london" => 55,
            "tokyo" => 75,
            _ => 70
        },
        "unit": "fahrenheit",
        "conditions": match city.to_lowercase().as_str() {
            "san francisco" | "sf" => "Foggy",
            "new york" | "nyc" => "Sunny",
            "london" => "Rainy",
            "tokyo" => "Clear",
            _ => "Partly cloudy"
        },
        "humidity": 65,
        "wind_speed": 10
    })
}

/// Simulates converting temperature between units
fn convert_temperature(temperature: f64, from_unit: &str, to_unit: &str) -> Value {
    let celsius = match from_unit.to_lowercase().as_str() {
        "fahrenheit" | "f" => (temperature - 32.0) * 5.0 / 9.0,
        "celsius" | "c" => temperature,
        "kelvin" | "k" => temperature - 273.15,
        _ => temperature,
    };

    let result = match to_unit.to_lowercase().as_str() {
        "fahrenheit" | "f" => celsius * 9.0 / 5.0 + 32.0,
        "celsius" | "c" => celsius,
        "kelvin" | "k" => celsius + 273.15,
        _ => celsius,
    };

    json!({
        "original_value": temperature,
        "original_unit": from_unit,
        "converted_value": result.round(),
        "converted_unit": to_unit
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AI SDK Rust - Streaming Tool Calling Example\n");

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

    let model: Arc<dyn ai_sdk_provider::language_model::LanguageModel> =
        Arc::from(provider.chat_model("gpt-4o-mini"));
    println!("✓ Model loaded: {}", model.model_id());
    println!("✓ Provider: {}\n", model.provider());

    // Define tools
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📋 Defining Tools");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    use ai_sdk_core::tool::definition::ToolExecutionOutput;

    // Weather tool
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
        let city = input
            .get("city")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();

        println!("\n🔧 Executing: get_weather(city=\"{}\")", city);
        let weather_data = get_weather(&city);
        println!(
            "   Result: {}°F, {}",
            weather_data["temperature"], weather_data["conditions"]
        );

        ToolExecutionOutput::Single(Box::pin(async move { Ok(weather_data) }))
    }));

    // Temperature conversion tool
    let convert_tool = Tool::function(json!({
        "type": "object",
        "properties": {
            "temperature": {
                "type": "number",
                "description": "The temperature value to convert"
            },
            "from_unit": {
                "type": "string",
                "description": "The current unit (fahrenheit, celsius, or kelvin)",
                "enum": ["fahrenheit", "celsius", "kelvin"]
            },
            "to_unit": {
                "type": "string",
                "description": "The target unit (fahrenheit, celsius, or kelvin)",
                "enum": ["fahrenheit", "celsius", "kelvin"]
            }
        },
        "required": ["temperature", "from_unit", "to_unit"]
    }))
    .with_description("Convert temperature between different units")
    .with_execute(Box::new(|input: Value, _options| {
        let temperature = input
            .get("temperature")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let from_unit = input
            .get("from_unit")
            .and_then(|v| v.as_str())
            .unwrap_or("fahrenheit")
            .to_string();
        let to_unit = input
            .get("to_unit")
            .and_then(|v| v.as_str())
            .unwrap_or("celsius")
            .to_string();

        println!(
            "\n🔧 Executing: convert_temperature({} {} -> {})",
            temperature, from_unit, to_unit
        );
        let result = convert_temperature(temperature, &from_unit, &to_unit);
        println!(
            "   Result: {}°{}",
            result["converted_value"], result["converted_unit"]
        );

        ToolExecutionOutput::Single(Box::pin(async move { Ok(result) }))
    }));

    println!("📋 Tools defined:");
    println!("  1. get_weather - Get current weather for a city");
    println!("  2. convert_temperature - Convert temperature units\n");

    // Create ToolSet for first example (weather only)
    let mut tools_example1 = ToolSet::new();
    tools_example1.insert("get_weather".to_string(), weather_tool);

    // Example 1: Single tool call with streaming
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📋 Example 1: Single Tool Call (Weather Query)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt = Prompt::text("What's the weather like in San Francisco?");
    println!("📤 Prompt: \"What's the weather like in San Francisco?\"");
    println!("ℹ️  Note: Using step_count_is(3) to allow tool execution + response\n");

    let settings = CallSettings::default()
        .with_temperature(0.7)
        .with_max_output_tokens(500);

    println!("⏳ Streaming response...\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let result = stream_text::stream_text(
        Arc::clone(&model),
        prompt,
        settings.clone(),
        Some(tools_example1),
        None,                                   // tool_choice - let model decide
        None,                                   // provider_options
        Some(vec![Box::new(step_count_is(3))]), // Allow up to 3 steps for tool execution
        None,                                   // prepare_step
        false,                                  // include_raw_chunks
        None,                                   // transforms
        None,                                   // on_chunk
        None,                                   // on_error
        None,                                   // on_step_finish
        None,                                   // on_finish
    )
    .await?;

    // Stream the full output showing tool calls and results
    let mut full_stream = result.full_stream();
    print!("📝 ");

    while let Some(part) = full_stream.next().await {
        use ai_sdk_core::stream_text::TextStreamPart;
        match part {
            TextStreamPart::TextDelta { text, .. } => {
                print!("{}", text);
                std::io::Write::flush(&mut std::io::stdout())?;
            }
            TextStreamPart::ToolInputStart { tool_name, .. } => {
                println!("\n\n🔧 Tool Call Starting: {}", tool_name);
                print!("   Args: ");
                std::io::Write::flush(&mut std::io::stdout())?;
            }
            TextStreamPart::ToolInputDelta { delta, .. } => {
                print!("{}", delta);
                std::io::Write::flush(&mut std::io::stdout())?;
            }
            TextStreamPart::ToolInputEnd { .. } => {
                println!();
            }
            TextStreamPart::ToolCall { tool_call } => {
                use ai_sdk_core::tool::TypedToolCall;
                let (tool_name, args) = match &tool_call {
                    TypedToolCall::Static(call) => (&call.tool_name, &call.input),
                    TypedToolCall::Dynamic(call) => (&call.tool_name, &call.input),
                };
                println!("\n\n🔧 Tool Call: {}", tool_name);
                println!("   Args: {}", serde_json::to_string_pretty(args)?);
            }
            TextStreamPart::ToolResult { tool_result } => {
                use ai_sdk_core::TypedToolResult;
                let result_value = match &tool_result {
                    TypedToolResult::Static(r) => &r.output,
                    TypedToolResult::Dynamic(r) => &r.output,
                };
                println!("\n✅ Tool Result:");
                println!("   {}", serde_json::to_string_pretty(result_value)?);
                print!("\n📝 ");
            }
            _ => {}
        }
    }

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Example 2: Multi-step tool execution with streaming
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📋 Example 2: Multi-Step Tool Execution");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Create tools for second example - need both tools for multi-step
    let weather_tool2 = Tool::function(json!({
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
        let city = input
            .get("city")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();

        println!("\n🔧 Executing: get_weather(city=\"{}\")", city);
        let weather_data = get_weather(&city);
        println!(
            "   Result: {}°F, {}",
            weather_data["temperature"], weather_data["conditions"]
        );

        ToolExecutionOutput::Single(Box::pin(async move { Ok(weather_data) }))
    }));

    let mut tools_example2 = ToolSet::new();
    tools_example2.insert("get_weather".to_string(), weather_tool2);
    tools_example2.insert("convert_temperature".to_string(), convert_tool);

    let prompt =
        Prompt::text("What's the weather in Tokyo? Then convert the temperature to Celsius.");
    println!(
        "📤 Prompt: \"What's the weather in Tokyo? Then convert the temperature to Celsius.\"\n"
    );
    println!("⏳ Streaming multi-step response...\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let result = stream_text::stream_text(
        Arc::clone(&model),
        prompt,
        settings.clone(),
        Some(tools_example2),
        None,                                   // tool_choice
        None,                                   // provider_options
        Some(vec![Box::new(step_count_is(5))]), // Allow up to 5 steps
        None,                                   // prepare_step
        false,                                  // include_raw_chunks
        None,                                   // transforms
        None,                                   // on_chunk
        None,                                   // on_error
        None,                                   // on_step_finish
        None,                                   // on_finish
    )
    .await?;

    let mut full_stream = result.full_stream();
    print!("📝 ");
    let mut step_count = 0;

    while let Some(part) = full_stream.next().await {
        use ai_sdk_core::stream_text::TextStreamPart;
        match part {
            TextStreamPart::StartStep { .. } => {
                step_count += 1;
                if step_count > 1 {
                    println!("\n\n🔄 Step {} starting...", step_count);
                    print!("📝 ");
                }
            }
            TextStreamPart::TextDelta { text, .. } => {
                print!("{}", text);
                std::io::Write::flush(&mut std::io::stdout())?;
            }
            TextStreamPart::ToolCall { tool_call } => {
                use ai_sdk_core::tool::TypedToolCall;
                let (tool_name, args) = match &tool_call {
                    TypedToolCall::Static(call) => (&call.tool_name, &call.input),
                    TypedToolCall::Dynamic(call) => (&call.tool_name, &call.input),
                };
                println!("\n\n🔧 Tool Call: {}", tool_name);
                println!("   Args: {}", serde_json::to_string_pretty(args)?);
            }
            TextStreamPart::ToolResult { tool_result } => {
                use ai_sdk_core::TypedToolResult;
                let result_value = match &tool_result {
                    TypedToolResult::Static(r) => &r.output,
                    TypedToolResult::Dynamic(r) => &r.output,
                };
                println!("\n✅ Tool Result:");
                println!("   {}", serde_json::to_string_pretty(result_value)?);
                print!("\n📝 ");
            }
            TextStreamPart::FinishStep { usage, .. } => {
                println!("\n\n📊 Step {} completed", step_count);
                println!(
                    "   Tokens: {} input, {} output",
                    usage.input_tokens, usage.output_tokens
                );
            }
            _ => {}
        }
    }

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("\n✅ All examples completed successfully!");
    println!("   Total steps executed: {}", step_count);

    Ok(())
}
