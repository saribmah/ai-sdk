/// Type-safe tools example demonstrating compile-time type checking for tool inputs/outputs.
///
/// This example shows how to:
/// - Define type-safe tools using the TypeSafeTool trait
/// - Get compile-time guarantees about tool input/output types
/// - Automatically generate JSON schemas from Rust types
/// - Mix type-safe and dynamic tools in the same application
///
/// Run with:
/// ```bash
/// export OPENAI_API_KEY="your-api-key"
/// cargo run --example type_safe_tools
/// ```
use ai_sdk_core::output::Output;
use ai_sdk_core::prompt::{Prompt, call_settings::CallSettings};
use ai_sdk_core::tool::TypeSafeTool;
use ai_sdk_core::{ToolSet, generate_text};
use ai_sdk_openai_compatible::OpenAICompatibleClient;
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::env;

// Define typed input/output structures for weather tool
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct WeatherInput {
    /// The city to get weather for (e.g., "San Francisco, CA")
    city: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct WeatherOutput {
    city: String,
    temperature: f64,
    unit: String,
    conditions: String,
    humidity: u32,
}

// Define typed input/output structures for temperature conversion tool
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct TemperatureConversionInput {
    /// The temperature value to convert
    temperature: f64,
    /// The current unit (fahrenheit or celsius)
    from_unit: String,
    /// The target unit (fahrenheit or celsius)
    to_unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TemperatureConversionOutput {
    original_value: f64,
    original_unit: String,
    converted_value: f64,
    converted_unit: String,
}

// Implement a type-safe weather tool
struct WeatherTool;

#[async_trait]
impl TypeSafeTool for WeatherTool {
    type Input = WeatherInput;
    type Output = WeatherOutput;

    fn name(&self) -> &str {
        "get_weather"
    }

    fn description(&self) -> &str {
        "Get the current weather for a given city"
    }

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, String> {
        println!("\n🔧 Executing: get_weather(city=\"{}\")", input.city);

        // Mock weather data based on city
        let (temperature, conditions) = match input.city.to_lowercase().as_str() {
            city if city.contains("san francisco") || city.contains("sf") => (68.0, "Foggy"),
            city if city.contains("new york") || city.contains("nyc") => (72.0, "Sunny"),
            city if city.contains("london") => (55.0, "Rainy"),
            city if city.contains("tokyo") => (75.0, "Clear"),
            _ => (70.0, "Partly cloudy"),
        };

        let output = WeatherOutput {
            city: input.city.clone(),
            temperature,
            unit: "fahrenheit".to_string(),
            conditions: conditions.to_string(),
            humidity: 65,
        };

        println!("   Result: {}°F, {}", output.temperature, output.conditions);

        Ok(output)
    }
}

// Implement a type-safe temperature conversion tool
struct TemperatureConversionTool;

#[async_trait]
impl TypeSafeTool for TemperatureConversionTool {
    type Input = TemperatureConversionInput;
    type Output = TemperatureConversionOutput;

    fn name(&self) -> &str {
        "convert_temperature"
    }

    fn description(&self) -> &str {
        "Convert temperature between fahrenheit and celsius"
    }

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, String> {
        println!(
            "\n🔧 Executing: convert_temperature({} {} -> {})",
            input.temperature, input.from_unit, input.to_unit
        );

        // Convert to celsius first
        let celsius = match input.from_unit.to_lowercase().as_str() {
            "fahrenheit" | "f" => (input.temperature - 32.0) * 5.0 / 9.0,
            "celsius" | "c" => input.temperature,
            _ => {
                return Err(format!("Unsupported from_unit: {}", input.from_unit));
            }
        };

        // Convert from celsius to target unit
        let converted_value = match input.to_unit.to_lowercase().as_str() {
            "fahrenheit" | "f" => celsius * 9.0 / 5.0 + 32.0,
            "celsius" | "c" => celsius,
            _ => {
                return Err(format!("Unsupported to_unit: {}", input.to_unit));
            }
        };

        let output = TemperatureConversionOutput {
            original_value: input.temperature,
            original_unit: input.from_unit.clone(),
            converted_value: converted_value.round(),
            converted_unit: input.to_unit.clone(),
        };

        println!(
            "   Result: {}°{}",
            output.converted_value, output.converted_unit
        );

        Ok(output)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 AI SDK Rust - Type-Safe Tools Example\n");

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

    let model = provider.chat_model("gpt-4o-mini");
    println!("✓ Model loaded: {}\n", model.model_id());

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Type-Safe Tools");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("✨ Benefits of Type-Safe Tools:");
    println!("   • Compile-time type checking for inputs and outputs");
    println!("   • Automatic JSON schema generation from Rust types");
    println!("   • Better IDE autocomplete and refactoring support");
    println!("   • Catch errors at compile time, not runtime\n");

    // Create type-safe tools
    let weather_tool = WeatherTool;
    let conversion_tool = TemperatureConversionTool;

    // Convert type-safe tools into untyped Tool instances
    // The into_tool() method automatically:
    // - Generates JSON schema from the Input type using schemars
    // - Wraps execute() to handle JSON serialization/deserialization
    // - Provides proper error handling
    let weather_tool_untyped = weather_tool.into_tool();
    let conversion_tool_untyped = conversion_tool.into_tool();

    // Create a ToolSet with our type-safe tools
    let mut tools = ToolSet::new();
    tools.insert("get_weather".to_string(), weather_tool_untyped);
    tools.insert("convert_temperature".to_string(), conversion_tool_untyped);

    println!("📋 Type-Safe Tools Registered:");
    println!("   • get_weather: WeatherInput -> WeatherOutput");
    println!(
        "   • convert_temperature: TemperatureConversionInput -> TemperatureConversionOutput\n"
    );

    // Example 1: Simple tool call
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 1: Single Type-Safe Tool Call");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt = Prompt::text("What's the weather like in San Francisco?");
    println!("📤 Prompt: \"What's the weather like in San Francisco?\"\n");

    let settings = CallSettings::default()
        .with_temperature(0.7)
        .with_max_output_tokens(500);

    println!("⏳ Generating response...");

    let result = generate_text(
        &*model,
        prompt,
        settings.clone(),
        Some(tools),
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await?;

    println!("\n✅ Response received!\n");

    // Display the result
    println!("📝 Final Response:");
    println!("   {}\n", result.text);

    println!("📊 Metadata:");
    println!("   • Steps: {}", result.steps.len());
    println!("   • Total tokens: {}\n", result.usage.total_tokens);

    // Show tool calls made
    if let Some(last_step) = result.steps.last() {
        let mut tool_calls_found = false;
        for content in &last_step.content {
            if let Output::ToolCall(tool_call) = content {
                if !tool_calls_found {
                    println!("🔧 Tool Calls Made:");
                    tool_calls_found = true;
                }
                println!("   • {} ({})", tool_call.tool_name, tool_call.tool_call_id);
                println!(
                    "     Input: {}",
                    serde_json::to_string_pretty(&tool_call.input)?
                );
            }
        }
        if tool_calls_found {
            println!();
        }
    }

    // Example 2: Multi-step with temperature conversion
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: Multi-Step Type-Safe Tools");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("💡 Type-safe tools can be used in multi-step scenarios:");
    println!();

    // Create new type-safe tool instances
    let weather_tool2 = WeatherTool.into_tool();
    let conversion_tool2 = TemperatureConversionTool.into_tool();

    let mut tools2 = ToolSet::new();
    tools2.insert("get_weather".to_string(), weather_tool2);
    tools2.insert("convert_temperature".to_string(), conversion_tool2);

    let prompt2 =
        Prompt::text("What's the weather in Tokyo? Then convert that temperature to Celsius.");

    println!(
        "📤 Prompt: \"What's the weather in Tokyo? Then convert that temperature to Celsius.\"\n"
    );
    println!("⏳ Generating response with multi-step tool execution...");

    let result2 = generate_text(
        &*model,
        prompt2,
        settings.clone(),
        Some(tools2),
        None,
        None,
        Some(vec![Box::new(ai_sdk_core::step_count_is(5))]), // Allow multiple steps
        None,
        None,
        None,
    )
    .await?;

    println!("\n✅ Response received!\n");

    // Display the result
    println!("📝 Final Response:");
    println!("   {}\n", result2.text);

    println!("📊 Metadata:");
    println!("   • Steps: {}", result2.steps.len());
    println!("   • Total tokens: {}\n", result2.usage.total_tokens);

    // Show all tool calls made across all steps
    println!("🔧 Tool Execution Trace:");
    for (step_idx, step) in result2.steps.iter().enumerate() {
        let mut step_has_tools = false;

        for content in &step.content {
            if let Output::ToolCall(tool_call) = content {
                if !step_has_tools {
                    println!("   Step {}:", step_idx + 1);
                    step_has_tools = true;
                }
                println!(
                    "   └─ {} (id: {})",
                    tool_call.tool_name, tool_call.tool_call_id
                );
                println!("      Input: {}", serde_json::to_string(&tool_call.input)?);
            }
        }

        for content in &step.content {
            if let Output::ToolResult(tool_result) = content {
                println!(
                    "      Result: {}",
                    serde_json::to_string_pretty(&tool_result.output)?
                );
            }
        }

        if step_has_tools {
            println!();
        }
    }

    // Example 3: Direct execution (bonus - showing flexibility)
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 3: Direct Type-Safe Execution (Bonus)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("💡 Type-safe tools can also be called directly without an LLM:");
    println!("   (Useful for testing, debugging, or mixed AI/non-AI workflows)\n");

    // Direct execution with compile-time type checking
    let weather_tool = WeatherTool;
    let typed_input = WeatherInput {
        city: "London".to_string(),
    };

    println!("   Executing: weather_tool.execute(WeatherInput {{ city: \"London\" }})");
    let typed_output = weather_tool.execute(typed_input).await?;

    println!("   Typed Output (WeatherOutput):");
    println!("   └─ city: {}", typed_output.city);
    println!(
        "   └─ temperature: {}°{}",
        typed_output.temperature, typed_output.unit
    );
    println!("   └─ conditions: {}", typed_output.conditions);
    println!("   └─ humidity: {}%\n", typed_output.humidity);

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Summary");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("✅ Type-Safe Tools provide:");
    println!("   1. Compile-time type checking - catch errors before runtime");
    println!("   2. Automatic schema generation - no manual JSON schema writing");
    println!("   3. IDE support - autocomplete, go-to-definition, refactoring");
    println!("   4. LLM integration - tools work seamlessly with generate_text");
    println!("   5. Direct execution - same tools can be called without LLM");
    println!("   6. Safety - impossible to pass wrong types or forget fields\n");

    println!("💡 Compare with dynamic tools:");
    println!("   Dynamic: Tool::function(json!(...)) - runtime type checking");
    println!("   Type-Safe: impl TypeSafeTool - compile-time type checking\n");

    println!("📋 This example demonstrated:");
    println!("   • Example 1: Single tool call via LLM");
    println!("   • Example 2: Multi-step tool execution via LLM");
    println!("   • Example 3: Direct execution (no LLM)\n");

    println!("✅ Example completed successfully!\n");

    Ok(())
}
