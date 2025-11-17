/// Tool calling example using OpenAI-compatible provider with only ai-sdk-provider.
///
/// This example demonstrates:
/// - Using LanguageModel::do_generate() directly with tools (no ai-sdk-core)
/// - Defining tools using ai-sdk-provider types
/// - Handling tool calls in the response
/// - NOTE: This example shows tool definitions but does not execute them
///   (tool execution requires ai-sdk-core). It validates that the provider
///   correctly handles tool schemas and returns tool calls.
///
/// Run with:
/// ```bash
/// export OPENAI_API_KEY="your-api-key"
/// cargo run --example chat_tool_calling -p ai-sdk-openai-compatible
/// ```
use ai_sdk_openai_compatible::OpenAICompatibleClient;
use ai_sdk_provider::language_model::call_options::LanguageModelCallOptions;
use ai_sdk_provider::language_model::content::LanguageModelContent;
use ai_sdk_provider::language_model::prompt::LanguageModelMessage;
use ai_sdk_provider::language_model::tool::LanguageModelTool;
use ai_sdk_provider::language_model::tool::function_tool::LanguageModelFunctionTool;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 OpenAI-Compatible Tool Calling Example (Provider-Only)\n");

    // Get API key from environment
    let api_key = std::env::var("OPENAI_API_KEY").map_err(
        |_| "OPENAI_API_KEY environment variable not set. Please set it with your API key.",
    )?;

    println!("✓ API key loaded from environment");

    // Create OpenAI-compatible provider using client builder
    let provider = OpenAICompatibleClient::new()
        .base_url("https://api.openai.com/v1")
        .api_key(api_key)
        .build();

    // Create a chat language model
    let model = provider.chat_model("gpt-4o-mini");

    println!("✓ Model loaded: {}\n", model.model_id());

    // Define tools using ai-sdk-provider types
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Defining Tools");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Tool 1: Get Weather
    let weather_tool = LanguageModelFunctionTool::new(
        "get_weather",
        json!({
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
        }),
    )
    .with_description("Get the current weather for a given city");

    // Tool 2: Temperature Converter
    let converter_tool = LanguageModelFunctionTool::new(
        "convert_temperature",
        json!({
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
        }),
    )
    .with_description("Convert temperature between different units");

    let tools = vec![
        LanguageModelTool::Function(weather_tool),
        LanguageModelTool::Function(converter_tool),
    ];

    println!("📋 Registered Tools:");
    println!("   1. get_weather - Get current weather for a city");
    println!("   2. convert_temperature - Convert temperature units\n");

    // Example 1: Simple tool call
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 1: Simple Weather Query");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt1 = vec![LanguageModelMessage::user_text(
        "What's the weather like in San Francisco?",
    )];

    let options1 = LanguageModelCallOptions::new(prompt1).with_tools(tools.clone());

    println!("💬 User: What's the weather like in San Francisco?\n");

    let result1 = model.do_generate(options1).await?;

    // Extract tool calls and text from content
    let (text1, tool_calls1) = extract_content_parts(&result1.content);

    if !tool_calls1.is_empty() {
        println!("🔧 Tool Calls Requested ({}):", tool_calls1.len());
        for tool_call in &tool_calls1 {
            println!(
                "   → {} (ID: {})",
                tool_call.tool_name, tool_call.tool_call_id
            );
            println!("     Args: {}", tool_call.input);
        }
        println!();
    }

    if !text1.is_empty() {
        println!("🤖 Assistant: {}\n", text1);
    }

    println!(
        "📊 Usage: {} input tokens, {} output tokens",
        result1.usage.input_tokens, result1.usage.output_tokens
    );
    println!("🏁 Finish reason: {:?}\n", result1.finish_reason);

    // Example 2: Query that might use multiple tools
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 2: Multiple Tool Usage");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt2 = vec![LanguageModelMessage::user_text(
        "What's the weather in Tokyo and New York?",
    )];

    let options2 = LanguageModelCallOptions::new(prompt2)
        .with_tools(tools.clone())
        .with_max_output_tokens(1024);

    println!("💬 User: What's the weather in Tokyo and New York?\n");

    let result2 = model.do_generate(options2).await?;

    // Extract tool calls and text from content
    let (text2, tool_calls2) = extract_content_parts(&result2.content);

    if !tool_calls2.is_empty() {
        println!("🔧 Tool Calls Requested ({}):", tool_calls2.len());
        for tool_call in &tool_calls2 {
            println!(
                "   → {} (ID: {})",
                tool_call.tool_name, tool_call.tool_call_id
            );
            println!("     Args: {}", tool_call.input);
        }
        println!();
    }

    if !text2.is_empty() {
        println!("🤖 Assistant: {}\n", text2);
    }

    println!(
        "📊 Usage: {} input tokens, {} output tokens",
        result2.usage.input_tokens, result2.usage.output_tokens
    );
    println!("🏁 Finish reason: {:?}\n", result2.finish_reason);

    // Example 3: Examining response structure
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Example 3: Response Structure Analysis");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let prompt3 = vec![LanguageModelMessage::user_text(
        "I need to convert 25 celsius to fahrenheit.",
    )];

    let options3 = LanguageModelCallOptions::new(prompt3).with_tools(tools);

    println!("💬 User: I need to convert 25 celsius to fahrenheit.\n");

    let result3 = model.do_generate(options3).await?;

    // Extract content parts
    let (text3, tool_calls3) = extract_content_parts(&result3.content);
    let (reasoning_parts, source_parts) = count_other_parts(&result3.content);

    println!("📦 Response Analysis:");
    println!("   Text content: {}", !text3.is_empty());
    println!("   Tool calls: {}", tool_calls3.len());
    println!("   Reasoning parts: {}", reasoning_parts);
    println!("   Source parts: {}", source_parts);
    println!("   Finish reason: {:?}", result3.finish_reason);
    println!("   Input tokens: {}", result3.usage.input_tokens);
    println!("   Output tokens: {}", result3.usage.output_tokens);

    if !tool_calls3.is_empty() {
        println!("\n🔧 Detailed Tool Calls:");
        for (i, tool_call) in tool_calls3.iter().enumerate() {
            println!("   Tool Call #{}:", i + 1);
            println!("      Name: {}", tool_call.tool_name);
            println!("      ID: {}", tool_call.tool_call_id);
            println!("      Input: {}", tool_call.input);
        }
    }

    println!("\n✅ Example completed successfully!");
    println!("\n💡 Key Features Demonstrated:");
    println!("   ✓ Using do_generate() with tools (provider-only)");
    println!("   ✓ Defining tools with LanguageModelFunctionTool");
    println!("   ✓ Tool call schema definition");
    println!("   ✓ Receiving tool calls in response");
    println!("   ✓ Inspecting response structure");
    println!("\n⚠️  Note: This example shows tool definitions only. For full tool execution,");
    println!("   use ai-sdk-core's GenerateText with tools.");

    Ok(())
}

/// Helper function to extract text and tool calls from content parts
fn extract_content_parts(
    content: &[LanguageModelContent],
) -> (
    String,
    Vec<&ai_sdk_provider::language_model::content::tool_call::LanguageModelToolCall>,
) {
    let text = content
        .iter()
        .filter_map(|c| match c {
            LanguageModelContent::Text(text) => Some(text.text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("");

    let tool_calls = content
        .iter()
        .filter_map(|c| match c {
            LanguageModelContent::ToolCall(tc) => Some(tc),
            _ => None,
        })
        .collect::<Vec<_>>();

    (text, tool_calls)
}

/// Helper function to count other content parts
fn count_other_parts(content: &[LanguageModelContent]) -> (usize, usize) {
    let reasoning = content
        .iter()
        .filter(|c| matches!(c, LanguageModelContent::Reasoning(_)))
        .count();

    let sources = content
        .iter()
        .filter(|c| matches!(c, LanguageModelContent::Source(_)))
        .count();

    (reasoning, sources)
}
