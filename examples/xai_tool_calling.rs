///
/// This example demonstrates tool calling with the xAI provider.
///
/// This example shows how to:
/// - Define tools with parameters for xAI models
/// - Use GenerateText with tools
/// - Handle tool calls in the response
/// - Execute tools and process results
///
/// Run with:
/// ```bash
/// export XAI_API_KEY="your-xai-api-key"
/// cargo run --example xai_tool_calling
/// ```
use llm_kit_core::output::Output;
use llm_kit_core::prompt::Prompt;
use llm_kit_core::{GenerateText, ToolSet};
use llm_kit_provider_utils::tool::Tool;
use llm_kit_xai::XaiClient;
use serde_json::{Value, json};
use std::env;
use std::sync::Arc;

/// Simulates fetching weather data for a given city
fn get_weather(city: &str) -> Value {
    // This is a mock implementation - in a real app, you'd call a weather API
    json!({
        "city": city,
        "temperature": 22,
        "unit": "celsius",
        "conditions": "Sunny",
        "humidity": 45,
        "wind_speed": 8
    })
}

/// Simulates a calculation tool
fn calculate(operation: &str, a: f64, b: f64) -> Value {
    let result = match operation {
        "add" => a + b,
        "subtract" => a - b,
        "multiply" => a * b,
        "divide" => {
            if b != 0.0 {
                a / b
            } else {
                return json!({
                    "error": "Division by zero",
                });
            }
        }
        _ => {
            return json!({
                "error": format!("Unknown operation: {}", operation),
            });
        }
    };

    json!({
        "operation": operation,
        "operand_a": a,
        "operand_b": b,
        "result": result
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 xAI Tool Calling Example\n");

    // Get API key from environment
    let api_key = env::var("XAI_API_KEY").map_err(
        |_| "XAI_API_KEY environment variable not set. Please set it with your xAI API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create xAI provider
    let provider = XaiClient::new().api_key(api_key).build();

    let model = provider.chat_model("grok-2-latest");

    println!("✓ Model loaded: {}\n", model.model_id());

    // Define tools
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Defining Tools");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    use llm_kit_provider_utils::tool::ToolExecutionOutput;

    // Weather tool
    let weather_tool = Tool::function(json!({
        "type": "object",
        "properties": {
            "city": {
                "type": "string",
                "description": "The city to get weather for, e.g. London, Paris, Tokyo"
            }
        },
        "required": ["city"]
    }))
    .with_description("Get the current weather for a given city")
    .with_execute(Arc::new(|input: Value, _options| {
        println!("\n🔧 TOOL EXECUTION: get_weather");
        println!("🔧 Input: {}", input);

        let city = input
            .get("city")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");

        let weather_data = get_weather(city);
        println!("🔧 Output: {}\n", weather_data);

        ToolExecutionOutput::Single(Box::pin(async move { Ok(weather_data) }))
    }));

    // Calculator tool
    let calculator_tool = Tool::function(json!({
        "type": "object",
        "properties": {
            "operation": {
                "type": "string",
                "enum": ["add", "subtract", "multiply", "divide"],
                "description": "The mathematical operation to perform"
            },
            "a": {
                "type": "number",
                "description": "The first operand"
            },
            "b": {
                "type": "number",
                "description": "The second operand"
            }
        },
        "required": ["operation", "a", "b"]
    }))
    .with_description("Perform a mathematical calculation")
    .with_execute(Arc::new(|input: Value, _options| {
        println!("\n🔧 TOOL EXECUTION: calculate");
        println!("🔧 Input: {}", input);

        let operation = input
            .get("operation")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let a = input.get("a").and_then(|v| v.as_f64()).unwrap_or(0.0);
        let b = input.get("b").and_then(|v| v.as_f64()).unwrap_or(0.0);

        let result = calculate(operation, a, b);
        println!("🔧 Output: {}\n", result);

        ToolExecutionOutput::Single(Box::pin(async move { Ok(result) }))
    }));

    // Create ToolSet
    let mut tools = ToolSet::new();
    tools.insert("get_weather".to_string(), weather_tool);
    tools.insert("calculate".to_string(), calculator_tool);

    println!("📋 Tool 1: get_weather");
    println!("   Description: Get the current weather for a given city");
    println!("   Parameters: city (string, required)\n");

    println!("📋 Tool 2: calculate");
    println!("   Description: Perform a mathematical calculation");
    println!("   Parameters: operation (enum), a (number), b (number)\n");

    // Example 1: Single tool call
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 1: Single Tool Call");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt1 = Prompt::text("What's the weather like in Tokyo?");

    println!("📤 Prompt: \"What's the weather like in Tokyo?\"");
    println!("🔧 Available tools: [get_weather, calculate]\n");

    println!("⏳ Generating response...\n");
    let result1 = GenerateText::new(model.clone(), prompt1)
        .temperature(0.7)
        .max_output_tokens(500)
        .tools(tools.clone())
        .execute()
        .await?;

    println!("✅ Response received!\n");

    println!("📊 Metadata:");
    println!("  • Finish reason: {:?}", result1.finish_reason);
    println!("  • Input tokens: {}", result1.usage.input_tokens);
    println!("  • Output tokens: {}", result1.usage.output_tokens);
    println!("  • Total tokens: {}\n", result1.usage.total_tokens);

    // Process tool calls
    if let Some(last_step) = result1.steps.last() {
        for content in &last_step.content {
            if let Output::ToolCall(tool_call) = content {
                println!("🔧 Tool Call Detected:");
                println!("  • Tool ID: {}", tool_call.tool_call_id);
                println!("  • Tool Name: {}", tool_call.tool_name);
                println!(
                    "  • Arguments: {}\n",
                    serde_json::to_string(&tool_call.input)?
                );
            }
        }
    }

    // Example 2: Multiple tool calls
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: Multiple Tool Calls");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt2 = Prompt::text("What's the weather in Paris? Also, what's 42 multiplied by 17?");

    println!("📤 Prompt: \"What's the weather in Paris? Also, what's 42 multiplied by 17?\"");
    println!("🔧 Available tools: [get_weather, calculate]\n");

    println!("⏳ Generating response...\n");
    let result2 = GenerateText::new(model.clone(), prompt2)
        .temperature(0.7)
        .max_output_tokens(500)
        .tools(tools.clone())
        .execute()
        .await?;

    println!("✅ Response received!\n");

    println!("📊 Metadata:");
    println!("  • Finish reason: {:?}", result2.finish_reason);
    println!("  • Input tokens: {}", result2.usage.input_tokens);
    println!("  • Output tokens: {}", result2.usage.output_tokens);
    println!("  • Total tokens: {}\n", result2.usage.total_tokens);

    let mut tool_call_count = 0;
    if let Some(last_step) = result2.steps.last() {
        for content in &last_step.content {
            if let Output::ToolCall(tool_call) = content {
                tool_call_count += 1;
                println!("🔧 Tool Call {} Detected:", tool_call_count);
                println!("  • Tool ID: {}", tool_call.tool_call_id);
                println!("  • Tool Name: {}", tool_call.tool_name);
                println!(
                    "  • Arguments: {}\n",
                    serde_json::to_string(&tool_call.input)?
                );
            }
        }
    }

    println!("💡 Total tool calls detected: {}\n", tool_call_count);

    // Example 3: Tool choice control
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 3: Specific Tool Choice");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    use llm_kit_provider::language_model::tool_choice::LanguageModelToolChoice;

    let prompt3 = Prompt::text("Tell me about London");

    println!("📤 Prompt: \"Tell me about London\"");
    println!("🔧 Available tools: [get_weather, calculate]");
    println!("⚙️  Tool choice: Required (must use a tool)\n");

    println!("⏳ Generating response...\n");
    let result3 = GenerateText::new(model, prompt3)
        .temperature(0.7)
        .max_output_tokens(500)
        .tools(tools)
        .tool_choice(LanguageModelToolChoice::Required)
        .execute()
        .await?;

    println!("✅ Response received!\n");

    println!("📊 Metadata:");
    println!("  • Finish reason: {:?}", result3.finish_reason);
    println!("  • Input tokens: {}", result3.usage.input_tokens);
    println!("  • Output tokens: {}", result3.usage.output_tokens);
    println!("  • Total tokens: {}\n", result3.usage.total_tokens);

    if let Some(last_step) = result3.steps.last() {
        for content in &last_step.content {
            if let Output::ToolCall(tool_call) = content {
                println!("🔧 Tool Call Detected (forced by tool_choice):");
                println!("  • Tool ID: {}", tool_call.tool_call_id);
                println!("  • Tool Name: {}", tool_call.tool_name);
                println!(
                    "  • Arguments: {}\n",
                    serde_json::to_string(&tool_call.input)?
                );
            }
        }
    }

    println!("✅ Example completed successfully!\n");

    Ok(())
}
